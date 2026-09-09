use super::{super::*, support::*};
use crate::business_identity::{platform, store, types::*, Principal};
use sea_orm::ConnectionTrait;
use serde_json::json;
use std::sync::Arc;

async fn tenant(f: &Fixture, name: &str) -> (Principal, String) {
    let platform =
        platform::PlatformContext::from_operator(&crate::web::auth::AuthenticatedOperator);
    let created = platform::create(
        &f.db.conn,
        &platform,
        platform::CreateTenantInput {
            operation_id: common::id(),
            organization_name: name.into(),
            owner_name: format!("{name} owner"),
        },
    )
    .await
    .unwrap();
    let token = created.token.unwrap();
    let p = store::resolve_credential(&f.db.conn, &token).await.unwrap();
    assert!(!p.is_operator());
    (p, token)
}
async fn cycle(f: &Fixture, p: &Principal) {
    let platform =
        platform::PlatformContext::from_operator(&crate::web::auth::AuthenticatedOperator);
    for (delta, status) in [
        (0, OrganizationStatus::Suspended),
        (1, OrganizationStatus::Active),
    ] {
        let result = platform::status(
            &f.db.conn,
            &platform,
            platform::StatusInput {
                operation_id: common::id(),
                organization_id: p.organization_id().into(),
                expected_revision: p.authorization_epoch() + delta,
                expected_authorization_epoch: p.authorization_epoch() + delta,
                status,
            },
        )
        .await
        .unwrap();
        assert_eq!(
            result.authorization_epoch,
            p.authorization_epoch() + delta + 1
        );
    }
}
async fn candidate(f: &Fixture, source: &SourceDetail) -> CandidateDetail {
    let row = candidates_list(
        &f.db.conn,
        &f.op,
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
    .remove(0);
    candidates_edit(&f.db.conn, &f.op, &f.services, EditCandidateInput {
        operation_id: common::id(), candidate_id: row.id, expected_revision: row.revision,
        expected_source_revision: source.source.revision.unwrap(), passage_ids: vec![source.passages[0].id.clone()],
        task: serde_json::from_value(json!({"title":"Human-reviewed follow-up","notes":"Only this public business text","domain":"feedback","dueDate":"2026-11-01"})).unwrap(),
        owner_suggestion: None, due_suggestion: None,
    }).await.unwrap()
}
async fn get(f: &Fixture, id: &str) -> CandidateDetail {
    candidates_get(
        &f.db.conn,
        &f.op,
        &f.services,
        CandidateInput {
            candidate_id: id.into(),
        },
    )
    .await
    .unwrap()
}
async fn publication_counts(f: &Fixture) -> Vec<i64> {
    let mut result = vec![];
    for table in [
        "business_task",
        "business_task_activity",
        "business_intake_candidate",
        "business_intake_decision",
        "business_intake_link",
        "business_intake_operation",
        "business_intake_audit",
    ] {
        result.push(count(&f.db.conn, table).await);
    }
    result
}

#[tokio::test]
async fn intake_tenancy_two_real_tenants_setup_domain_ceiling_zero_grants_and_reference_privacy() {
    let mut f = Fixture::new().await;
    let (a, _) = tenant(&f, "Tenant A").await;
    let (b, _) = tenant(&f, "Tenant B").await;
    f.op = a.clone();
    let own = f.binding().await;
    let list = bindings_list(&f.db.conn, &a, &f.services, PageInput { page: 0 })
        .await
        .unwrap();
    assert_eq!(list.setup_kinds, vec![SourceKind::Fireflies]);
    assert!(list.setup_domains.contains(&Domain::Feedback));
    assert!(list.items[0].admin.is_some());
    assert!(!list.items[0].binding.capabilities.read);
    assert_eq!(count(&f.db.conn, "business_intake_grant").await, 0);
    let (_, _, admin) = human(&f.db.conn, &a, Role::Admin, vec![Domain::Feedback]).await;
    let (_, _, manager) = human(&f.db.conn, &a, Role::Manager, vec![Domain::Feedback]).await;
    let listed = bindings_list(&f.db.conn, &admin, &f.services, PageInput { page: 0 })
        .await
        .unwrap();
    assert_eq!(listed.setup_domains, vec![Domain::Feedback]);
    assert!(listed.items[0].admin.is_some());
    let before = count(&f.db.conn, "business_intake_setup").await;
    let requests = f.mock.count();
    for caller in [&manager, &b] {
        assert!(bindings_status(
            &f.db.conn,
            caller,
            &f.services,
            BindingInput {
                binding_id: own.id.clone()
            }
        )
        .await
        .is_err());
        assert!(grants_list(
            &f.db.conn,
            caller,
            &f.services,
            BindingPageInput {
                binding_id: own.id.clone(),
                page: 0
            }
        )
        .await
        .is_err());
    }
    for change in [
        json!({"sourceOwnerId":b.member_id()}),
        json!({"domain":"marketing"}),
        json!({"publicationDomains":["marketing"]}),
        json!({"source":{"kind":"email","inboxId":1}}),
    ] {
        let mut value = f.create_json(&common::id());
        for (key, value2) in change.as_object().unwrap() {
            value[key] = value2.clone();
        }
        assert!(bindings_create(
            &f.db.conn,
            &admin,
            &f.services,
            None,
            serde_json::from_value(value).unwrap()
        )
        .await
        .is_err());
    }
    assert!(grants_upsert(
        &f.db.conn,
        &a,
        &f.services,
        grant(&own, b.member_id(), None)
    )
    .await
    .is_err());
    assert_eq!(count(&f.db.conn, "business_intake_setup").await, before);
    assert_eq!(f.mock.count(), requests);
    let allowed = grants_upsert(
        &f.db.conn,
        &admin,
        &f.services,
        grant(&own, a.member_id(), None),
    )
    .await
    .unwrap()
    .binding;
    let allowed = bindings_update(&f.db.conn, &admin, &f.services, enable(&allowed))
        .await
        .unwrap();
    let source_a = f.source(&allowed).await;
    f.op = b.clone();
    let own_b = f.allowed().await;
    let source_b = f.source(&own_b).await;
    assert_ne!(source_a.source.id, source_b.source.id);
    assert_eq!(source_a.source.revision, source_b.source.revision);
    for (caller, foreign) in [(&a, &source_b), (&b, &source_a)] {
        assert!(sources_get(
            &f.db.conn,
            caller,
            &f.services,
            SourceInput {
                source_id: foreign.source.id.clone()
            }
        )
        .await
        .is_err());
        assert!(candidates_list(
            &f.db.conn,
            caller,
            &f.services,
            CandidatesInput {
                source_id: foreign.source.id.clone(),
                state: CandidateState::Pending,
                page: 0
            }
        )
        .await
        .is_err());
    }
    assert_eq!(count(&f.db.conn, "business_intake_source").await, 2);
    // Actual same-database foreign member, not a missing UUID from another DB.
    assert!(f.db.conn.execute(common::sql("INSERT INTO business_intake_grant(id,organization_id,binding_id,member_id,revision,state,scope,can_read,can_import,can_triage,publication_domains) VALUES(?,?,?,?,1,'active','binding_current_and_future_sources',1,0,0,'[]')",vec![common::id().into(),a.organization_id().into(),own.id.into(),b.member_id().into()])).await.is_err());
}

#[tokio::test]
async fn intake_tenancy_fresh_login_and_identical_refresh_cannot_resurrect_preview() {
    let mut f = Fixture::new().await;
    let (p, token) = tenant(&f, "Epoch tenant").await;
    f.op = p.clone();
    let binding = f.allowed().await;
    let source = f.source(&binding).await;
    let prepared = candidate(&f, &source).await;
    assert!(prepared.candidate.capabilities.accept);
    cycle(&f, &p).await;
    assert!(sources_get(
        &f.db.conn,
        &p,
        &f.services,
        SourceInput {
            source_id: source.source.id.clone()
        }
    )
    .await
    .is_err());
    f.op = store::resolve_credential(&f.db.conn, &token).await.unwrap();
    let hidden = get(&f, &prepared.candidate.id).await;
    assert_eq!(hidden.candidate.disclosure, Disclosure::MetadataOnly);
    assert!(hidden.candidate.draft.is_none() && hidden.passages.is_empty());
    assert!(hidden.candidate.requires_rebase);
    let refreshed = f
        .refresh(
            &binding,
            &source.source.id,
            "Private transcript marker; never automatically publish.",
        )
        .await;
    assert_eq!(refreshed.source.revision, source.source.revision);
    let still_stale = get(&f, &prepared.candidate.id).await;
    assert_eq!(still_stale.candidate.disclosure, Disclosure::Fresh);
    assert!(still_stale.candidate.requires_rebase && !still_stale.candidate.capabilities.accept);
    let before = publication_counts(&f).await;
    assert!(candidates_accept(
        &f.db.conn,
        &f.op,
        &f.services,
        AcceptCandidateInput {
            operation_id: common::id(),
            candidate_id: prepared.candidate.id.clone(),
            expected_revision: prepared.candidate.revision,
            expected_source_revision: prepared.candidate.source_revision,
            publish_to_domain: Domain::Feedback,
        }
    )
    .await
    .is_err());
    assert_eq!(publication_counts(&f).await, before);
    let rebased = candidates_select(
        &f.db.conn,
        &f.op,
        &f.services,
        SelectCandidateInput {
            operation_id: common::id(),
            candidate_id: prepared.candidate.id.clone(),
            expected_revision: prepared.candidate.revision,
            expected_source_revision: prepared.candidate.source_revision,
            passage_ids: vec![source.passages[0].id.clone()],
        },
    )
    .await
    .unwrap();
    assert_eq!(
        serde_json::to_value(&rebased.candidate.draft).unwrap(),
        serde_json::to_value(&prepared.candidate.draft).unwrap()
    );
    assert!(!rebased.candidate.requires_rebase);
    let result = candidates_accept(
        &f.db.conn,
        &f.op,
        &f.services,
        AcceptCandidateInput {
            operation_id: common::id(),
            candidate_id: rebased.candidate.id,
            expected_revision: rebased.candidate.revision,
            expected_source_revision: rebased.candidate.source_revision,
            publish_to_domain: Domain::Feedback,
        },
    )
    .await
    .unwrap();
    assert_eq!(result.task.task.notes, "Only this public business text");
    assert_eq!(result.task.task.due_date.as_deref(), Some("2026-11-01"));
    assert!(result.task.execution.is_none());
}

#[tokio::test]
async fn intake_tenancy_suspension_fences_blocked_setup_and_source_attempt_without_reconstructing_actor(
) {
    let mut f = Fixture::new().await;
    let (p, token) = tenant(&f, "Racing tenant").await;
    f.op = p.clone();
    let gate = Arc::new(tokio::sync::Barrier::new(2));
    let mut response = Reply::json(json!({"data":{"user":{"user_id":"source-user"}}}));
    response.gate = Some(gate.clone());
    f.mock.reply(response);
    let input = f.create_input(&common::id());
    let old_create = bindings_create(&f.db.conn, &p, &f.services, None, input);
    let lifecycle = async {
        f.mock.seen(1).await;
        cycle(&f, &p).await;
        gate.wait().await;
    };
    let (result, ()) = tokio::join!(old_create, lifecycle);
    assert!(result.is_err());
    assert_eq!(count(&f.db.conn, "business_intake_binding").await, 0);
    f.op = store::resolve_credential(&f.db.conn, &token).await.unwrap();
    let binding = f.allowed().await;
    assert_eq!(f.secrets.values.lock().unwrap().len(), 1); // new setup cleaned stale staged key only
    let source = f.source(&binding).await;
    let old = f.op.clone();
    let job = imports_start(
        &f.db.conn,
        &old,
        &f.services,
        StartImportInput {
            operation_id: common::id(),
            binding_id: binding.id.clone(),
            selection: Selection::Record {
                source_id: source.source.id.clone(),
            },
        },
    )
    .await
    .unwrap();
    let gate = Arc::new(tokio::sync::Barrier::new(2));
    let mut response = Reply::json(transcript("review-record", "Late obsolete observation"));
    response.gate = Some(gate.clone());
    f.mock.reply(response);
    let expected_count = f.mock.count() + 1;
    let old_read = imports_advance(
        &f.db.conn,
        &old,
        &f.services,
        ImportRevisionInput {
            operation_id: common::id(),
            import_id: job.id.clone(),
            expected_revision: job.revision,
        },
    );
    let lifecycle = async {
        f.mock.seen(expected_count).await;
        cycle(&f, &old).await;
        gate.wait().await;
    };
    let (result, ()) = tokio::join!(old_read, lifecycle);
    assert!(result.is_err());
    assert_eq!(count(&f.db.conn, "business_intake_version").await, 1);
    f.op = store::resolve_credential(&f.db.conn, &token).await.unwrap();
    let pending = imports_get(
        &f.db.conn,
        &f.op,
        &f.services,
        ImportInput {
            import_id: job.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(pending.state, ImportState::Waiting);
    assert!(pending.capabilities.advance);
    f.mock.json(transcript(
        "review-record",
        "Current authorized observation",
    ));
    let done = imports_advance(
        &f.db.conn,
        &f.op,
        &f.services,
        ImportRevisionInput {
            operation_id: common::id(),
            import_id: pending.id,
            expected_revision: pending.revision,
        },
    )
    .await
    .unwrap();
    assert_eq!(done.state, ImportState::Complete);
    assert_eq!(count(&f.db.conn, "business_intake_version").await, 2);
    let current = sources_get(
        &f.db.conn,
        &f.op,
        &f.services,
        SourceInput {
            source_id: source.source.id,
        },
    )
    .await
    .unwrap();
    assert!(current
        .passages
        .iter()
        .any(|p| p.text == "Current authorized observation"));
    assert!(!current
        .passages
        .iter()
        .any(|p| p.text == "Late obsolete observation"));
}
