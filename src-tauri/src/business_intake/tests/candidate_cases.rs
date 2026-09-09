use super::{super::*, support::*};
use crate::{
    business_identity::{self as identity, types::*},
    business_tasks::{store as tasks, types as task, vocabulary::*, ActorContext},
};
use sea_orm::{ConnectionTrait, Database};
use serde_json::{json, Value};

fn task_input(domain: Domain) -> task::CreateInput {
    task::CreateInput {
        title: "Discuss the reviewed follow-up".into(),
        notes: "Exact human-approved public notes.\nSecond line.".into(),
        domain,
        priority: TaskPriority::High,
        due_date: Some("2026-11-01".into()),
        owner_id: None,
        assignee_id: None,
        reviewer_id: None,
    }
}
async fn seed(f: &Fixture, p: &identity::Principal, source: &SourceDetail) -> Candidate {
    candidates_list(
        &f.db.conn,
        p,
        &f.services,
        CandidatesInput {
            source_id: source.source.id.clone(),
            state: CandidateState::Pending,
            page: 0,
        },
    )
    .await
    .unwrap()
    .items
    .remove(0)
}
async fn get(f: &Fixture, p: &identity::Principal, id: &str) -> CandidateDetail {
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
fn edit_input(row: &Candidate, source: &SourceDetail, domain: Domain) -> EditCandidateInput {
    EditCandidateInput {
        operation_id: common::id(),
        candidate_id: row.id.clone(),
        expected_revision: row.revision,
        expected_source_revision: source.source.revision.unwrap(),
        passage_ids: vec![source.passages[0].id.clone()],
        task: task_input(domain),
        owner_suggestion: Some("Alex? Unresolved human annotation".into()),
        due_suggestion: Some("Next Friday? Not a deadline".into()),
    }
}
fn accept_input(row: &Candidate, operation_id: &str, domain: Domain) -> AcceptCandidateInput {
    AcceptCandidateInput {
        operation_id: operation_id.into(),
        candidate_id: row.id.clone(),
        expected_revision: row.revision,
        expected_source_revision: row.source_revision,
        publish_to_domain: domain,
    }
}
async fn prepared(f: &Fixture, source: &SourceDetail) -> CandidateDetail {
    let row = seed(f, &f.op, source).await;
    candidates_edit(
        &f.db.conn,
        &f.op,
        &f.services,
        edit_input(&row, source, Domain::Feedback),
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn intake_candidate_exact_human_draft_atomic_accept_replay_and_restricted_source_link() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let (editor, _, p) = human(&f.db.conn, &f.op, Role::Member, vec![Domain::Feedback]).await;
    let b = grants_upsert(&f.db.conn, &f.op, &f.services, grant(&b, &editor.id, None))
        .await
        .unwrap()
        .binding;
    let source = f.source(&b).await;
    let row = seed(&f, &p, &source).await;
    assert!(!row.has_prepared_draft);
    assert!(row.capabilities.edit && row.capabilities.link && !row.capabilities.accept);
    let edited = candidates_edit(
        &f.db.conn,
        &p,
        &f.services,
        edit_input(&row, &source, Domain::Feedback),
    )
    .await
    .unwrap();
    let draft = edited.candidate.draft.as_ref().unwrap();
    assert_eq!(draft.owner_id, editor.id);
    assert_eq!(draft.assignee_id, None);
    assert_eq!(draft.reviewer_id, None);
    assert_eq!(draft.due_date.as_deref(), Some("2026-11-01"));
    let operation = common::id();
    let result = candidates_accept(
        &f.db.conn,
        &f.op,
        &f.services,
        accept_input(&edited.candidate, &operation, Domain::Feedback),
    )
    .await
    .unwrap();
    assert!(!result.replayed);
    assert_eq!(result.task.task.status, TaskStatus::Todo);
    assert_eq!(result.task.task.revision, 1);
    assert_eq!(result.task.task.owner_id, editor.id);
    assert_eq!(result.task.task.creator_id, f.op.member_id());
    assert_eq!(result.task.task.notes, draft.notes);
    assert_eq!(result.task.activity.len(), 1);
    assert_eq!(result.task.activity[0].kind, "created");
    assert!(result.task.execution.is_none());
    let public = serde_json::to_string(&result.task).unwrap();
    for private in [
        "Private transcript marker",
        "Unresolved human annotation",
        "Not a deadline",
        &source.source.id,
    ] {
        assert!(!public.contains(private));
    }
    let replay = candidates_accept(
        &f.db.conn,
        &f.op,
        &f.services,
        accept_input(&edited.candidate, &operation, Domain::Feedback),
    )
    .await
    .unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.decision.id, result.decision.id);
    assert_eq!(replay.task.task.id, result.task.task.id);
    assert_eq!(count(&f.db.conn, "business_task").await, 1);
    assert_eq!(count(&f.db.conn, "business_task_activity").await, 1);
    assert_eq!(count(&f.db.conn, "business_intake_decision").await, 1);
    assert_eq!(count(&f.db.conn, "business_intake_link").await, 1);
    assert!(candidates_accept(
        &f.db.conn,
        &f.op,
        &f.services,
        accept_input(&edited.candidate, &operation, Domain::Marketing)
    )
    .await
    .is_err());
    assert!(candidates_accept(
        &f.db.conn,
        &p,
        &f.services,
        accept_input(&edited.candidate, &operation, Domain::Feedback)
    )
    .await
    .is_err());
    let recovered = get(&f, &p, &row.id).await;
    assert_eq!(recovered.candidate.state, CandidateState::Accepted);
    assert_eq!(recovered.decision.unwrap().id, result.decision.id);
    let (_, _, task_reader) = human(&f.db.conn, &f.op, Role::Viewer, vec![Domain::Feedback]).await;
    let links = tasks_sources(
        &f.db.conn,
        &task_reader,
        &f.services,
        TaskInput {
            task_id: result.task.task.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(links.links.len(), 1);
    assert!(!links.links[0].accessible);
    assert!(links.links[0].source.is_none());
    assert!(!serde_json::to_string(&links)
        .unwrap()
        .contains(&source.source.id));
    assert!(candidates_get(
        &f.db.conn,
        &task_reader,
        &f.services,
        CandidateInput {
            candidate_id: row.id
        }
    )
    .await
    .is_err());
}

#[tokio::test]
async fn intake_candidate_null_draft_rebase_links_exact_target_and_invalidates_review() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let source = f.source(&b).await;
    let row = seed(&f, &f.op, &source).await;
    let original = get(&f, &f.op, &row.id).await;
    let ctx = ActorContext::authenticated(f.op.clone());
    let target = tasks::create(&f.db.conn, &ctx, task_input(Domain::Feedback))
        .await
        .unwrap();
    let target = tasks::progress(
        &f.db.conn,
        &ctx,
        task::ProgressInput {
            task_id: target.task.id,
            expected_revision: 1,
            status: ProgressStatus::Review,
        },
    )
    .await
    .unwrap();
    let changed = f
        .refresh(
            &b,
            &source.source.id,
            "Changed transcript requiring exact review",
        )
        .await;
    let stale = get(&f, &f.op, &row.id).await;
    assert!(stale.candidate.requires_rebase && !stale.candidate.capabilities.link);
    assert_eq!(stale.candidate.source_revision, 1);
    assert_eq!(stale.source.revision, Some(2));
    assert_eq!(stale.passages[0].id, original.passages[0].id);
    let link_input = |c: &Candidate, revision: i64| LinkCandidateInput {
        operation_id: common::id(),
        candidate_id: c.id.clone(),
        expected_revision: c.revision,
        expected_source_revision: 2,
        task_id: target.task.id.clone(),
        expected_task_revision: revision,
        publish_to_domain: Domain::Feedback,
    };
    assert!(candidates_link(
        &f.db.conn,
        &f.op,
        &f.services,
        link_input(&stale.candidate, target.task.revision)
    )
    .await
    .is_err());
    let rebased = candidates_select(
        &f.db.conn,
        &f.op,
        &f.services,
        SelectCandidateInput {
            operation_id: common::id(),
            candidate_id: row.id.clone(),
            expected_revision: stale.candidate.revision,
            expected_source_revision: 2,
            passage_ids: vec![changed.passages[0].id.clone()],
        },
    )
    .await
    .unwrap();
    assert!(!rebased.candidate.has_prepared_draft && rebased.candidate.draft.is_none());
    assert!(!rebased.candidate.requires_rebase && rebased.candidate.capabilities.link);
    assert!(candidates_link(
        &f.db.conn,
        &f.op,
        &f.services,
        link_input(&rebased.candidate, 1)
    )
    .await
    .is_err());
    assert_eq!(count(&f.db.conn, "business_intake_link").await, 0);
    let linked = candidates_link(
        &f.db.conn,
        &f.op,
        &f.services,
        link_input(&rebased.candidate, target.task.revision),
    )
    .await
    .unwrap();
    assert_eq!(linked.task.task.title, target.task.title);
    assert_eq!(linked.task.task.notes, target.task.notes);
    assert_eq!(linked.task.task.status, TaskStatus::InProgress);
    assert!(linked.task.task.current_deliverable_id.is_none());
    let links = tasks_sources(
        &f.db.conn,
        &f.op,
        &f.services,
        TaskInput {
            task_id: target.task.id,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        linked.task.activity.last().unwrap().payload,
        json!({"linkId":links.links[0].link_id})
    );
    assert!(links.links[0].accessible);
    let stored =
        f.db.conn
            .query_one(common::sql(
                "SELECT reviewed_draft FROM business_intake_decision WHERE id=?",
                vec![linked.decision.id.into()],
            ))
            .await
            .unwrap()
            .unwrap();
    assert_eq!(
        stored
            .try_get::<Option<String>>("", "reviewed_draft")
            .unwrap(),
        None
    );
    assert_eq!(count(&f.db.conn, "business_task").await, 1);
}

#[tokio::test]
async fn intake_candidate_destination_read_and_expired_metadata_withhold_prepared_text() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let mut update = enable(&b);
    update.publication_domains.push(Domain::Marketing);
    let b = bindings_update(&f.db.conn, &f.op, &f.services, update)
        .await
        .unwrap();
    let current = grants_list(
        &f.db.conn,
        &f.op,
        &f.services,
        BindingPageInput {
            binding_id: b.id.clone(),
            page: 0,
        },
    )
    .await
    .unwrap()
    .items
    .remove(0);
    let mut g = grant(&b, f.op.member_id(), Some(current.revision));
    g.publication_domains.push(Domain::Marketing);
    let b = grants_upsert(&f.db.conn, &f.op, &f.services, g)
        .await
        .unwrap()
        .binding;
    let (m, _, reader) = human(&f.db.conn, &f.op, Role::Viewer, vec![Domain::Feedback]).await;
    let mut g = grant(&b, &m.id, None);
    g.import = false;
    g.triage = false;
    g.publication_domains.clear();
    let b = grants_upsert(&f.db.conn, &f.op, &f.services, g)
        .await
        .unwrap()
        .binding;
    let source = f.source(&b).await;
    let row = seed(&f, &f.op, &source).await;
    let edited = candidates_edit(
        &f.db.conn,
        &f.op,
        &f.services,
        edit_input(&row, &source, Domain::Marketing),
    )
    .await
    .unwrap();
    let restricted = get(&f, &reader, &row.id).await;
    assert_eq!(restricted.candidate.disclosure, Disclosure::Fresh);
    assert!(restricted.candidate.has_prepared_draft && restricted.candidate.draft.is_none());
    assert!(!restricted.passages.is_empty());
    assert!(
        !restricted.candidate.capabilities.select
            && !restricted.candidate.capabilities.edit
            && !restricted.candidate.capabilities.link
            && !restricted.candidate.capabilities.discard
    );
    let accepted = candidates_accept(
        &f.db.conn,
        &f.op,
        &f.services,
        accept_input(&edited.candidate, &common::id(), Domain::Marketing),
    )
    .await
    .unwrap();
    let restricted = get(&f, &reader, &row.id).await;
    assert!(matches!(
        restricted.decision.as_ref().unwrap().task,
        DecisionTask::Restricted
    ));
    assert!(!serde_json::to_string(&restricted)
        .unwrap()
        .contains(&accepted.task.task.id));
    f.db.conn
        .execute(common::sql(
            "UPDATE business_intake_source SET access_until='2020-01-01T00:00:00Z' WHERE id=?",
            vec![source.source.id.into()],
        ))
        .await
        .unwrap();
    let expired = get(&f, &f.op, &row.id).await;
    assert_eq!(expired.candidate.disclosure, Disclosure::MetadataOnly);
    assert!(expired.candidate.has_prepared_draft && expired.candidate.draft.is_none());
    assert!(
        expired.candidate.owner_suggestion.is_none() && expired.candidate.due_suggestion.is_none()
    );
    assert!(expired.passages.is_empty());
    assert!(
        !expired.candidate.capabilities.select
            && !expired.candidate.capabilities.edit
            && !expired.candidate.capabilities.accept
            && !expired.candidate.capabilities.link
            && !expired.candidate.capabilities.discard
    );
}

#[tokio::test]
async fn intake_candidate_atomic_failure_and_two_connections_cas_leave_no_orphan() {
    let dir = tempfile::tempdir().unwrap();
    let f = Fixture::with_db(crate::db::test_helpers::fresh_disk_db(dir.path()).await).await;
    let other = Database::connect(format!(
        "sqlite:{}?mode=rw",
        dir.path().join("source.db").display()
    ))
    .await
    .unwrap();
    let b = f.allowed().await;
    let source = f.source(&b).await;
    let edited = prepared(&f, &source).await;
    let audit_count = count(&f.db.conn, "business_intake_audit").await;
    let receipt_count = count(&f.db.conn, "business_intake_operation").await;
    f.db.conn.execute_unprepared("CREATE TRIGGER fixture_decision_failure BEFORE INSERT ON business_intake_decision BEGIN SELECT RAISE(ABORT,'synthetic decision unavailable'); END;").await.unwrap();
    assert!(candidates_accept(
        &f.db.conn,
        &f.op,
        &f.services,
        accept_input(&edited.candidate, &common::id(), Domain::Feedback)
    )
    .await
    .is_err());
    for table in [
        "business_task",
        "business_task_activity",
        "business_intake_decision",
        "business_intake_link",
    ] {
        assert_eq!(count(&f.db.conn, table).await, 0);
    }
    assert_eq!(
        count(&f.db.conn, "business_intake_audit").await,
        audit_count
    );
    assert_eq!(
        count(&f.db.conn, "business_intake_operation").await,
        receipt_count
    );
    assert_eq!(
        get(&f, &f.op, &edited.candidate.id)
            .await
            .candidate
            .revision,
        edited.candidate.revision
    );
    f.db.conn
        .execute_unprepared("DROP TRIGGER fixture_decision_failure")
        .await
        .unwrap();
    let (a, b) = tokio::join!(
        candidates_accept(
            &f.db.conn,
            &f.op,
            &f.services,
            accept_input(&edited.candidate, &common::id(), Domain::Feedback)
        ),
        candidates_accept(
            &other,
            &f.op,
            &f.services,
            accept_input(&edited.candidate, &common::id(), Domain::Feedback)
        )
    );
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    assert!(
        matches!(
            a,
            Err(error::Error::Identity(identity::IdentityError::Conflict))
        ) || matches!(
            b,
            Err(error::Error::Identity(identity::IdentityError::Conflict))
        )
    );
    for table in [
        "business_task",
        "business_task_activity",
        "business_intake_decision",
        "business_intake_link",
    ] {
        assert_eq!(count(&f.db.conn, table).await, 1);
    }
    assert_eq!(
        count(&f.db.conn, "business_intake_audit").await,
        audit_count + 1
    );
    assert_eq!(
        count(&f.db.conn, "business_intake_operation").await,
        receipt_count + 1
    );
}

#[tokio::test]
async fn intake_candidate_source_change_grant_epoch_and_invalid_selection_cannot_publish() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let source = f.source(&b).await;
    let edited = prepared(&f, &source).await;
    let draft = serde_json::to_value(edited.candidate.draft.as_ref().unwrap()).unwrap();
    let changed = f
        .refresh(&b, &source.source.id, "New evidence exact version")
        .await;
    let stale = get(&f, &f.op, &edited.candidate.id).await;
    assert_eq!(
        serde_json::to_value(stale.candidate.draft.as_ref().unwrap()).unwrap(),
        draft
    );
    assert!(candidates_accept(
        &f.db.conn,
        &f.op,
        &f.services,
        accept_input(&edited.candidate, &common::id(), Domain::Feedback)
    )
    .await
    .is_err());
    for ids in [
        vec![source.passages[0].id.clone()],
        vec![changed.passages[0].id.clone(); 2],
        vec![common::id()],
    ] {
        assert!(candidates_select(
            &f.db.conn,
            &f.op,
            &f.services,
            SelectCandidateInput {
                operation_id: common::id(),
                candidate_id: stale.candidate.id.clone(),
                expected_revision: stale.candidate.revision,
                expected_source_revision: 2,
                passage_ids: ids
            }
        )
        .await
        .is_err());
    }
    let selected = candidates_select(
        &f.db.conn,
        &f.op,
        &f.services,
        SelectCandidateInput {
            operation_id: common::id(),
            candidate_id: stale.candidate.id.clone(),
            expected_revision: stale.candidate.revision,
            expected_source_revision: 2,
            passage_ids: vec![changed.passages[0].id.clone()],
        },
    )
    .await
    .unwrap();
    assert_eq!(
        serde_json::to_value(selected.candidate.draft.as_ref().unwrap()).unwrap(),
        draft
    );
    let g = grants_list(
        &f.db.conn,
        &f.op,
        &f.services,
        BindingPageInput {
            binding_id: b.id.clone(),
            page: 0,
        },
    )
    .await
    .unwrap()
    .items
    .remove(0);
    let b = grants_upsert(
        &f.db.conn,
        &f.op,
        &f.services,
        grant(&b, f.op.member_id(), Some(g.revision)),
    )
    .await
    .unwrap()
    .binding;
    let epoch_stale = get(&f, &f.op, &selected.candidate.id).await;
    assert!(epoch_stale.candidate.requires_rebase);
    assert_eq!(epoch_stale.candidate.disclosure, Disclosure::MetadataOnly);
    assert!(candidates_accept(
        &f.db.conn,
        &f.op,
        &f.services,
        accept_input(&selected.candidate, &common::id(), Domain::Feedback)
    )
    .await
    .is_err());
    let fresh = f
        .refresh(&b, &source.source.id, "New evidence exact version")
        .await;
    assert_eq!(fresh.source.revision, Some(2));
    let discard = candidates_discard(
        &f.db.conn,
        &f.op,
        &f.services,
        DiscardInput {
            operation_id: common::id(),
            candidate_id: selected.candidate.id,
            expected_revision: selected.candidate.revision,
        },
    )
    .await
    .unwrap();
    assert_eq!(discard.kind, CandidateState::Discarded);
    assert!(matches!(discard.task, DecisionTask::None));
    assert_eq!(count(&f.db.conn, "business_task").await, 0);
}

#[tokio::test]
async fn intake_candidate_waiting_writer_rechecks_original_credential_before_any_publication() {
    let dir = tempfile::tempdir().unwrap();
    let f = Fixture::with_db(crate::db::test_helpers::fresh_disk_db(dir.path()).await).await;
    let other = Database::connect(format!(
        "sqlite:{}?mode=rw",
        dir.path().join("source.db").display()
    ))
    .await
    .unwrap();
    let b = f.allowed().await;
    let (m, issued, p) = human(&f.db.conn, &f.op, Role::Member, vec![Domain::Feedback]).await;
    let b = grants_upsert(&f.db.conn, &f.op, &f.services, grant(&b, &m.id, None))
        .await
        .unwrap()
        .binding;
    let source = f.source(&b).await;
    let row = seed(&f, &p, &source).await;
    let edited = candidates_edit(
        &f.db.conn,
        &p,
        &f.services,
        edit_input(&row, &source, Domain::Feedback),
    )
    .await
    .unwrap();
    let revoke = identity::begin_write(&f.db.conn, f.op.organization_id())
        .await
        .unwrap();
    revoke
        .execute(common::sql(
            "UPDATE business_credential SET revoked_at=? WHERE organization_id=? AND id=?",
            vec![
                common::now().into(),
                f.op.organization_id().into(),
                issued.credential.id.into(),
            ],
        ))
        .await
        .unwrap();
    let input = accept_input(&edited.candidate, &common::id(), Domain::Feedback);
    let services = f.services.clone();
    let mut waiting =
        tokio::spawn(async move { candidates_accept(&other, &p, &services, input).await });
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(50), &mut waiting)
            .await
            .is_err()
    );
    revoke.commit().await.unwrap();
    assert!(matches!(
        waiting.await.unwrap(),
        Err(error::Error::Identity(
            identity::IdentityError::Unauthorized
        ))
    ));
    for table in [
        "business_task",
        "business_task_activity",
        "business_intake_decision",
        "business_intake_link",
    ] {
        assert_eq!(count(&f.db.conn, table).await, 0);
    }
}

#[test]
fn intake_publication_inputs_reject_embedded_task_identity_and_provider_payload() {
    let base = json!({"operationId":common::id(),"candidateId":common::id(),"expectedRevision":1,"expectedSourceRevision":1,"publishToDomain":"feedback"});
    assert!(serde_json::from_value::<AcceptCandidateInput>(base.clone()).is_ok());
    for field in [
        "task",
        "organizationId",
        "actorId",
        "sourceText",
        "credentialRef",
        "approve",
        "endpoint",
    ] {
        let mut value: Value = base.clone();
        value[field] = json!("spoofed");
        assert!(serde_json::from_value::<AcceptCandidateInput>(value).is_err());
    }
}
