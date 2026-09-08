//! Remaining B recovery probes. All keys, provider replies and databases are synthetic.
use super::{super::*, support::*};
use crate::business_identity::{self as identity, store, types::*};
use sea_orm::{ConnectionTrait, TransactionTrait};
use serde_json::json;
use std::{sync::atomic::Ordering, time::Duration};

fn extra(source: &SourceDetail, operation: &str) -> CreateCandidateInput {
    CreateCandidateInput {
        operation_id: operation.into(),
        source_id: source.source.id.clone(),
        expected_source_revision: source.source.revision.unwrap(),
        passage_ids: vec![source.passages[0].id.clone()],
    }
}
fn discard(row: &Candidate, operation: &str) -> DiscardInput {
    DiscardInput {
        operation_id: operation.into(),
        candidate_id: row.id.clone(),
        expected_revision: row.revision,
    }
}
async fn candidate(f: &Fixture, p: &identity::Principal, id: &str) -> CandidateDetail {
    candidates_get(
        &f.db.conn,
        p,
        &f.services,
        CandidateInput {
            candidate_id: id.into(),
        },
    )
    .await
    .unwrap()
}
async fn active_ref(f: &Fixture, binding: &str) -> String {
    f.db.conn
        .query_one(common::sql(
            "SELECT credential_ref FROM business_intake_binding WHERE organization_id=? AND id=?",
            vec![f.op.organization_id().into(), binding.into()],
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "credential_ref")
        .unwrap()
}
async fn stage_ref(f: &Fixture, operation: &str) -> (String, String) {
    let row = f.db.conn.query_one(common::sql("SELECT credential_ref,state FROM business_intake_setup WHERE organization_id=? AND actor_id=? AND operation_id=?",
        vec![f.op.organization_id().into(), f.op.member_id().into(), operation.into()])).await.unwrap().unwrap();
    (
        row.try_get("", "credential_ref").unwrap(),
        row.try_get("", "state").unwrap(),
    )
}
fn rotation(b: &BindingAdmin, operation: &str) -> UpdateBindingInput {
    let mut input = enable(b);
    input.operation_id = operation.into();
    input.credential = Some(CredentialReplacement::Fireflies {
        api_key: Secret("synthetic-recovery-rotation".into()),
    });
    input
}

#[tokio::test]
async fn intake_recovery_extra_candidate_discard_history_and_explicit_new_work() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let source = f.source(&b).await;
    let operation = common::id();
    let created = candidates_create(&f.db.conn, &f.op, &f.services, extra(&source, &operation))
        .await
        .unwrap();
    assert_eq!(created.origin, CandidateOrigin::HumanSelection);
    assert!(!created.has_prepared_draft && created.draft.is_none());
    assert_eq!(count(&f.db.conn, "business_intake_candidate").await, 2);
    let replay = candidates_create(&f.db.conn, &f.op, &f.services, extra(&source, &operation))
        .await
        .unwrap();
    assert_eq!(replay.id, created.id);
    let mut different = extra(&source, &operation);
    different.passage_ids = vec![source.passages[1].id.clone()];
    assert!(matches!(
        candidates_create(&f.db.conn, &f.op, &f.services, different).await,
        Err(error::Error::Identity(identity::IdentityError::Conflict))
    ));
    assert_eq!(count(&f.db.conn, "business_intake_candidate").await, 2);
    f.db.conn
        .execute(common::sql(
            "UPDATE business_intake_source SET access_until='2020-01-01T00:00:00Z' WHERE id=?",
            vec![source.source.id.clone().into()],
        ))
        .await
        .unwrap();
    assert!(matches!(
        candidates_create(
            &f.db.conn,
            &f.op,
            &f.services,
            extra(&source, &common::id())
        )
        .await,
        Err(error::Error::Intake(error::Reason::SourceExpired))
    ));
    let expired = candidate(&f, &f.op, &created.id).await;
    assert_eq!(expired.candidate.disclosure, Disclosure::MetadataOnly);
    assert!(expired.candidate.capabilities.discard && !expired.candidate.capabilities.select);
    let discard_operation = common::id();
    let decision = candidates_discard(
        &f.db.conn,
        &f.op,
        &f.services,
        discard(&created, &discard_operation),
    )
    .await
    .unwrap();
    assert_eq!(decision.kind, CandidateState::Discarded);
    let replay = candidates_discard(
        &f.db.conn,
        &f.op,
        &f.services,
        discard(&created, &discard_operation),
    )
    .await
    .unwrap();
    assert_eq!(replay.id, decision.id);
    let retained = candidate(&f, &f.op, &created.id).await;
    assert_eq!(retained.candidate.revision, created.revision + 1);
    assert!(retained.passages.is_empty());
    let changed = f
        .refresh(
            &b,
            &source.source.id,
            "Changed source after a terminal human decision",
        )
        .await;
    let retained = candidate(&f, &f.op, &created.id).await;
    assert_eq!(retained.candidate.source_revision, 1);
    assert_eq!(retained.candidate.revision, created.revision + 1);
    assert_eq!(retained.decision.unwrap().id, decision.id);
    assert_eq!(count(&f.db.conn, "business_intake_candidate").await, 2);
    let fresh_work = candidates_create(
        &f.db.conn,
        &f.op,
        &f.services,
        extra(&changed, &common::id()),
    )
    .await
    .unwrap();
    assert_ne!(fresh_work.id, created.id);
    assert_eq!(fresh_work.source_revision, 2);
    assert_eq!(count(&f.db.conn, "business_intake_candidate").await, 3);
    let history = candidates_list(
        &f.db.conn,
        &f.op,
        &f.services,
        CandidatesInput {
            source_id: source.source.id,
            state: CandidateState::Discarded,
            page: 0,
        },
    )
    .await
    .unwrap();
    assert_eq!(history.items.len(), 1);
    assert_eq!(history.items[0].id, created.id);
    for sql in [
        "DELETE FROM business_intake_candidate",
        "UPDATE business_intake_decision SET kind='linked'",
        "DELETE FROM business_intake_decision",
    ] {
        assert!(f.db.conn.execute_unprepared(sql).await.is_err());
    }
    assert_eq!(count(&f.db.conn, "business_task").await, 0);
}

#[tokio::test]
async fn intake_recovery_uses_current_credential_grant_and_real_passage_scope() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let (m, issued, p) = human(&f.db.conn, &f.op, Role::Member, vec![Domain::Feedback]).await;
    let granted = grants_upsert(&f.db.conn, &f.op, &f.services, grant(&b, &m.id, None))
        .await
        .unwrap();
    let b = granted.binding;
    let source = f.source(&b).await;
    let operation = common::id();
    let row = candidates_create(&f.db.conn, &p, &f.services, extra(&source, &operation))
        .await
        .unwrap();
    let discard_operation = common::id();
    let decided = candidates_discard(
        &f.db.conn,
        &p,
        &f.services,
        discard(&row, &discard_operation),
    )
    .await
    .unwrap();
    let other_binding = f.allowed().await;
    let other_source = f.source(&other_binding).await;
    assert_ne!(other_source.source.id, source.source.id);
    let mut wrong_passage = extra(&source, &common::id());
    wrong_passage.passage_ids = vec![other_source.passages[0].id.clone()];
    assert!(
        candidates_create(&f.db.conn, &f.op, &f.services, wrong_passage)
            .await
            .is_err()
    );
    let before = count(&f.db.conn, "business_intake_operation").await;
    store::revoke_credential(
        &f.db.conn,
        &f.op,
        RevokeCredentialInput {
            organization_id: f.op.organization_id().into(),
            credential_id: issued.credential.id,
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        candidates_create(&f.db.conn, &p, &f.services, extra(&source, &operation)).await,
        Err(error::Error::Identity(
            identity::IdentityError::Unauthorized
        ))
    ));
    assert!(matches!(
        candidates_discard(
            &f.db.conn,
            &p,
            &f.services,
            discard(&row, &discard_operation)
        )
        .await,
        Err(error::Error::Identity(
            identity::IdentityError::Unauthorized
        ))
    ));
    let newly_issued = store::issue_credential(
        &f.db.conn,
        &f.op,
        IssueCredentialInput {
            organization_id: f.op.organization_id().into(),
            member_id: m.id,
            label: "Explicit synthetic replacement session".into(),
        },
    )
    .await
    .unwrap();
    let current = store::resolve_credential(&f.db.conn, &newly_issued.token)
        .await
        .unwrap();
    assert_eq!(
        candidates_discard(
            &f.db.conn,
            &current,
            &f.services,
            discard(&row, &discard_operation)
        )
        .await
        .unwrap()
        .id,
        decided.id
    );
    let revoked = grants_revoke(
        &f.db.conn,
        &f.op,
        &f.services,
        RevokeGrantInput {
            operation_id: common::id(),
            binding_id: b.id.clone(),
            expected_binding_revision: b.revision,
            grant_id: granted.grant.id,
            expected_grant_revision: granted.grant.revision,
        },
    )
    .await
    .unwrap();
    assert!(candidates_get(
        &f.db.conn,
        &current,
        &f.services,
        CandidateInput {
            candidate_id: row.id.clone()
        }
    )
    .await
    .is_err());
    assert!(candidates_discard(
        &f.db.conn,
        &current,
        &f.services,
        discard(&row, &discard_operation)
    )
    .await
    .is_err());
    grants_upsert(
        &f.db.conn,
        &f.op,
        &f.services,
        grant(
            &revoked.binding,
            current.member_id(),
            Some(revoked.grant.revision),
        ),
    )
    .await
    .unwrap();
    let recovered = candidate(&f, &current, &row.id).await;
    assert_eq!(recovered.candidate.disclosure, Disclosure::MetadataOnly);
    assert!(recovered.passages.is_empty());
    assert_eq!(recovered.decision.unwrap().id, decided.id);
    assert_eq!(
        count(&f.db.conn, "business_intake_operation").await,
        before + 2
    ); // only explicit revoke/regrant
    assert_eq!(count(&f.db.conn, "business_intake_decision").await, 1);
    assert_eq!(count(&f.db.conn, "business_task").await, 0);
}

#[tokio::test]
async fn intake_recovery_late_blocking_store_after_abort_and_cleanup_retries_only_orphan() {
    let f = Fixture::new().await;
    let b = f.binding().await;
    let active = active_ref(&f, &b.id).await;
    f.secrets
        .values
        .lock()
        .unwrap()
        .insert("synthetic-unrelated".into(), "synthetic-preserved".into());
    let (release, receiver) = tokio::sync::oneshot::channel();
    *f.secrets.set_gate.lock().unwrap() = Some(receiver);
    let operation = common::id();
    {
        let request = bindings_update(&f.db.conn, &f.op, &f.services, rotation(&b, &operation));
        tokio::pin!(request);
        tokio::select! {
            _=&mut request=>panic!("gated adapter completed before release"),
            seen=tokio::time::timeout(Duration::from_secs(3), f.secrets.set_started.notified())=>seen.unwrap(),
        }
    } // Drop the core future while its actual spawn_blocking set is still running.
    let (orphan, state) = stage_ref(&f, &operation).await;
    assert_eq!(state, "staged");
    assert_ne!(orphan, active);
    assert!(!f.secrets.values.lock().unwrap().contains_key(&orphan));
    f.db.conn.execute(common::sql("UPDATE business_intake_setup SET expires_at='2020-01-01T00:00:00Z' WHERE operation_id=?", vec![operation.clone().into()])).await.unwrap();
    let current = bindings_update(&f.db.conn, &f.op, &f.services, enable(&b))
        .await
        .unwrap();
    assert_eq!(stage_ref(&f, &operation).await.1, "retired");
    assert!(!f.secrets.values.lock().unwrap().contains_key(&orphan));
    release.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(3), f.secrets.set_finished.notified())
        .await
        .unwrap();
    assert!(f.secrets.values.lock().unwrap().contains_key(&orphan));
    assert_eq!(active_ref(&f, &b.id).await, active);
    assert_eq!(f.mock.count(), 1); // Aborted caller never reaches provider verification.
    f.secrets.fail_delete.store(true, Ordering::SeqCst);
    let current = bindings_update(&f.db.conn, &f.op, &f.services, enable(&current))
        .await
        .unwrap();
    assert!(f.secrets.values.lock().unwrap().contains_key(&orphan));
    f.secrets.fail_delete.store(false, Ordering::SeqCst);
    bindings_update(&f.db.conn, &f.op, &f.services, enable(&current))
        .await
        .unwrap();
    assert!(!f.secrets.values.lock().unwrap().contains_key(&orphan));
    let keys = f.secrets.values.lock().unwrap();
    assert_eq!(keys.get(&active).unwrap(), "synthetic-fireflies");
    assert_eq!(
        keys.get("synthetic-unrelated").unwrap(),
        "synthetic-preserved"
    );
    assert_eq!(keys.len(), 2);
    drop(keys);
    assert_eq!(count(&f.db.conn, "business_intake_binding").await, 1);
    assert_eq!(count(&f.db.conn, "business_intake_grant").await, 0);
}

#[tokio::test]
async fn intake_recovery_activation_commit_failure_and_lost_response_preserve_active_reference() {
    let f = Fixture::new().await;
    let b = f.binding().await;
    let active = active_ref(&f, &b.id).await;
    f.secrets
        .values
        .lock()
        .unwrap()
        .insert("synthetic-unrelated".into(), "synthetic-preserved".into());
    f.db.conn.execute_unprepared("CREATE TABLE fixture_activation_parent(id INTEGER PRIMARY KEY); CREATE TABLE fixture_activation_child(id INTEGER REFERENCES fixture_activation_parent(id) DEFERRABLE INITIALLY DEFERRED);").await.unwrap();
    // Prove the injection is deferred until COMMIT, not a failed INSERT probe.
    let control = f.db.conn.begin().await.unwrap();
    control
        .execute_unprepared("INSERT INTO fixture_activation_child VALUES(1)")
        .await
        .unwrap();
    assert!(control.commit().await.is_err());
    f.db.conn.execute_unprepared("CREATE TRIGGER fixture_activation_commit_failure AFTER INSERT ON business_intake_operation WHEN NEW.operation='bindings/update' BEGIN INSERT INTO fixture_activation_child VALUES(1); END;").await.unwrap();
    let failed_operation = common::id();
    f.mock
        .json(json!({"data":{"user":{"user_id":"source-user"}}}));
    let failed = bindings_update(
        &f.db.conn,
        &f.op,
        &f.services,
        rotation(&b, &failed_operation),
    )
    .await;
    assert!(matches!(
        failed,
        Err(error::Error::Identity(identity::IdentityError::Database(_)))
    ));
    assert_eq!(active_ref(&f, &b.id).await, active);
    assert_eq!(count(&f.db.conn, "fixture_activation_child").await, 0);
    assert_eq!(count(&f.db.conn, "business_intake_operation").await, 1);
    assert_eq!(count(&f.db.conn, "business_intake_audit").await, 1);
    let unchanged = bindings_status(
        &f.db.conn,
        &f.op,
        &f.services,
        BindingInput {
            binding_id: b.id.clone(),
        },
    )
    .await
    .unwrap()
    .admin
    .unwrap();
    assert_eq!(unchanged.revision, b.revision);
    let (retired, state) = stage_ref(&f, &failed_operation).await;
    assert_eq!(state, "retired");
    assert!(f.secrets.values.lock().unwrap().contains_key(&retired));
    f.db.conn
        .execute_unprepared("DROP TRIGGER fixture_activation_commit_failure")
        .await
        .unwrap();
    let b = bindings_update(&f.db.conn, &f.op, &f.services, enable(&b))
        .await
        .unwrap();
    assert!(!f.secrets.values.lock().unwrap().contains_key(&retired));
    let successful_operation = common::id();
    f.mock
        .json(json!({"data":{"user":{"user_id":"source-user"}}}));
    // Discard the successful response; recovery must use the durable receipt.
    bindings_update(
        &f.db.conn,
        &f.op,
        &f.services,
        rotation(&b, &successful_operation),
    )
    .await
    .unwrap();
    let new_active = active_ref(&f, &b.id).await;
    assert_ne!(new_active, active);
    let observed_reads = f.mock.count();
    let recovered = bindings_update(
        &f.db.conn,
        &f.op,
        &f.services,
        rotation(&b, &successful_operation),
    )
    .await
    .unwrap();
    assert_eq!(recovered.revision, b.revision + 1);
    assert_eq!(active_ref(&f, &b.id).await, new_active);
    assert_eq!(f.mock.count(), observed_reads);
    assert_eq!(stage_ref(&f, &successful_operation).await.1, "active");
    let keys = f.secrets.values.lock().unwrap();
    assert!(!keys.contains_key(&active)); // Replay cleanup removed only the retired old key.
    assert_eq!(
        keys.get(&new_active).unwrap(),
        "synthetic-recovery-rotation"
    );
    assert_eq!(
        keys.get("synthetic-unrelated").unwrap(),
        "synthetic-preserved"
    );
    assert_eq!(keys.len(), 2);
    assert!(!serde_json::to_string(&recovered)
        .unwrap()
        .contains("synthetic-recovery-rotation"));
}
