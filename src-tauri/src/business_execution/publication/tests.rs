//! Synthetic retained bytes + real identity/task writer. Version rows are seeded
//! explicitly: these tests do not certify the still-pending import/runner path.
use super::*;
use crate::business_execution::{session_store, session_tests};
use sea_orm::DatabaseConnection;
use serde_json::Value;

struct Case {
    directory: tempfile::TempDir,
    db: DatabaseConnection,
    principal: Principal,
    task: tasks::types::Detail,
    files: Files,
    session_id: String,
    admission_id: String,
    asset_id: String,
    first: VersionSelection,
}
async fn setup() -> Case {
    let (db, principal, task, profile) = session_tests::setup().await;
    let reserved =
        session_store::reserve_start(&db, &principal, &session_tests::start(&task, &profile))
            .await
            .unwrap();
    let session_id = reserved.result.session.id;
    session_store::complete_launch(&db, &reserved.admission.unwrap(), &session_tests::link())
        .await
        .unwrap();
    let admission_id = db.query_one(statement(
        "SELECT admission_id FROM business_execution_generation WHERE session_id=? AND generation=1",
        vec![session_id.clone().into()],
    )).await.unwrap().unwrap().try_get::<String>("", "admission_id").unwrap();
    let directory = tempfile::tempdir().unwrap();
    let files = Files::new(directory.path().into());
    let asset_id = id();
    let mut case = Case {
        directory,
        db,
        principal,
        task,
        files,
        session_id,
        admission_id,
        asset_id: asset_id.clone(),
        first: VersionSelection {
            asset_id,
            version_id: String::new(),
        },
    };
    case.first = version(
        &case,
        1,
        b"Exact first managed document",
        "Customer proposal",
    )
    .await;
    case
}
async fn version(case: &Case, number: i64, bytes: &[u8], title: &str) -> VersionSelection {
    let workspace = case.files.workspace(&case.admission_id).unwrap();
    std::fs::write(workspace.join("output.md"), bytes).unwrap();
    let found = case.files.scan(&case.admission_id).unwrap().remove(0);
    let retained = case
        .files
        .stage(
            &case.admission_id,
            &found.relative,
            &found.observation,
            &id(),
        )
        .unwrap();
    let version_id = id();
    let tx = identity::begin_write(&case.db, case.principal.organization_id())
        .await
        .unwrap();
    if number == 1 {
        tx.execute(statement(
            "INSERT INTO business_execution_asset(id,organization_id,task_id,member_id,revision,title,media_type,latest_version_id,created_at,updated_at) VALUES(?,?,?,?,1,?,'text/plain',?,?,?)",
            vec![case.asset_id.clone().into(), case.principal.organization_id().into(), case.task.task.id.clone().into(),
                case.principal.member_id().into(), title.into(), version_id.clone().into(), now().into(), now().into()],
        )).await.unwrap();
    }
    tx.execute(statement(
        "INSERT INTO business_execution_asset_version(id,organization_id,task_id,asset_id,version,title,sha256,byte_size,media_type,object_id,created_at,created_by,producer_session_id,producer_turn_id,producer_client_id,producer_model) VALUES(?,?,?,?,?,?,?,?,'text/plain',?,?,?,?,?,'pi','gpt-6-astra')",
        vec![version_id.clone().into(), case.principal.organization_id().into(), case.task.task.id.clone().into(),
            case.asset_id.clone().into(), number.into(), title.into(), retained.sha256.into(), retained.byte_size.into(),
            retained.object_id.into(), now().into(), case.principal.member_id().into(), case.session_id.clone().into(),
            "private-turn-must-not-be-public".into()],
    )).await.unwrap();
    if number > 1 {
        tx.execute(statement(
            "UPDATE business_execution_asset SET revision=revision+1,title=?,latest_version_id=?,updated_at=? WHERE id=?",
            vec![title.into(), version_id.clone().into(), now().into(), case.asset_id.clone().into()],
        )).await.unwrap();
    }
    tx.commit().await.unwrap();
    VersionSelection {
        asset_id: case.asset_id.clone(),
        version_id,
    }
}
fn input(case: &Case) -> SubmitInput {
    SubmitInput {
        operation_id: id(),
        task_id: case.task.task.id.clone(),
        expected_task_revision: 1,
        versions: vec![case.first.clone()],
        body: String::new(),
    }
}
async fn snapshot(db: &DatabaseConnection) -> Vec<Vec<String>> {
    let mut result = vec![];
    for table in [
        "business_task",
        "business_task_activity",
        "business_task_deliverable",
        "business_execution_publication",
        "business_execution_operation",
        "business_execution_asset",
        "business_execution_asset_version",
    ] {
        let names = db
            .query_all(statement(&format!("PRAGMA table_info({table})"), vec![]))
            .await
            .unwrap()
            .into_iter()
            .map(|r| format!("\"{}\"", r.try_get::<String>("", "name").unwrap()))
            .collect::<Vec<_>>()
            .join(",");
        result.push(
            db.query_all(statement(
                &format!("SELECT json_array({names}) AS value FROM {table} ORDER BY rowid"),
                vec![],
            ))
            .await
            .unwrap()
            .into_iter()
            .map(|r| r.try_get::<String>("", "value").unwrap())
            .collect(),
        );
    }
    result
}
async fn member(case: &Case, kind: identity::MemberKind, role: identity::Role) -> Principal {
    let member = identity::store::create_member(
        &case.db,
        &case.principal,
        identity::types::CreateMemberInput {
            organization_id: case.principal.organization_id().into(),
            display_name: "Named reviewer".into(),
            kind,
            role,
            domains: vec![identity::Domain::Feedback],
        },
    )
    .await
    .unwrap();
    if kind == identity::MemberKind::Agent {
        return identity::agent_principal(&case.db, &case.principal, &member.id)
            .await
            .unwrap();
    }
    let issued = identity::store::issue_credential(
        &case.db,
        &case.principal,
        identity::types::IssueCredentialInput {
            organization_id: case.principal.organization_id().into(),
            member_id: member.id,
            label: "Synthetic review".into(),
        },
    )
    .await
    .unwrap();
    identity::store::resolve_credential(&case.db, &issued.token)
        .await
        .unwrap()
}

#[tokio::test]
async fn execution_publication_file_only_is_atomic_replayable_and_rediscoverable() {
    let case = setup().await;
    let request = input(&case);
    let result = submit(&case.db, &case.files, &case.principal, request.clone())
        .await
        .unwrap();
    assert_eq!(result.operation.status, OperationStatus::Confirmed);
    assert_eq!(
        result.detail.task.status,
        tasks::vocabulary::TaskStatus::Review
    );
    assert_eq!(result.detail.task.revision, 2);
    let delivered = &result.detail.deliverables[0];
    assert!(delivered.body.is_empty());
    assert_eq!(delivered.author.id, case.principal.member_id());
    assert_eq!(delivered.author.kind, "human");
    assert_eq!(delivered.assets.len(), 1);
    assert_eq!(delivered.assets[0].version_id, case.first.version_id);
    assert_eq!(
        delivered.assets[0].sha256,
        digest(b"Exact first managed document")
    );
    let saved = snapshot(&case.db).await;
    let replayed = submit(&case.db, &case.files, &case.principal, request.clone())
        .await
        .unwrap();
    assert_eq!(replayed.detail.task.revision, 2);
    assert_eq!(snapshot(&case.db).await, saved);
    let mut changed = request;
    changed.body = "Different human disclosure".into();
    assert!(matches!(
        submit(&case.db, &case.files, &case.principal, changed).await,
        Err(Error(OperationReason::Conflict))
    ));
    assert_eq!(snapshot(&case.db).await, saved);

    let _private_next = version(
        &case,
        2,
        b"Private unselected next draft",
        "Private next title",
    )
    .await;
    let reader = member(&case, identity::MemberKind::Human, identity::Role::Viewer).await;
    let detail = tasks::store::get(
        &case.db,
        &tasks::ActorContext::authenticated(reader.clone()),
        tasks::types::TaskInput {
            task_id: case.task.task.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(detail.deliverables[0].assets[0].title, "Customer proposal");
    assert_eq!(
        detail.deliverables[0].assets[0].version_id,
        case.first.version_id
    );
    let view = published(
        &case.db,
        &reader,
        PublishedInput {
            task_id: case.task.task.id.clone(),
            deliverable_id: delivered.id.clone(),
            asset_id: case.asset_id.clone(),
            version_id: case.first.version_id.clone(),
        },
    )
    .await
    .unwrap();
    let public = serde_json::to_value(&view).unwrap();
    assert_eq!(
        public["publication"]["submittedBy"]["authorityKind"],
        "operator"
    );
    assert_eq!(
        public["producer"],
        serde_json::json!({"clientId":"pi","model":"gpt-6-astra"})
    );
    let encoded = public.to_string();
    for private in [
        "sessionId",
        "turnId",
        "profileId",
        "objectId",
        "latestVersionId",
        "reviewReferences",
        "Private next title",
        "private-turn-must-not-be-public",
    ] {
        assert!(!encoded.contains(private), "{private}");
    }
    assert!(published(
        &case.db,
        &reader,
        PublishedInput {
            task_id: case.task.task.id.clone(),
            deliverable_id: delivered.id.clone(),
            asset_id: case.asset_id.clone(),
            version_id: _private_next.version_id,
        }
    )
    .await
    .is_err());
    let tenant = identity::platform::create(
        &case.db,
        &identity::platform::PlatformContext::from_operator(
            &crate::web::auth::AuthenticatedOperator,
        ),
        identity::platform::CreateTenantInput {
            operation_id: id(),
            organization_name: "Other isolated business".into(),
            owner_name: "Other owner".into(),
        },
    )
    .await
    .unwrap();
    let tenant_principal =
        identity::store::resolve_credential(&case.db, tenant.token.as_ref().unwrap())
            .await
            .unwrap();
    assert!(published(
        &case.db,
        &tenant_principal,
        PublishedInput {
            task_id: case.task.task.id.clone(),
            deliverable_id: delivered.id.clone(),
            asset_id: case.asset_id.clone(),
            version_id: case.first.version_id.clone(),
        }
    )
    .await
    .is_err());
}

#[tokio::test]
async fn execution_publication_agents_member_credentials_and_foreign_refs_cannot_submit() {
    let case = setup().await;
    let agent = member(&case, identity::MemberKind::Agent, identity::Role::Member).await;
    let human = member(&case, identity::MemberKind::Human, identity::Role::Manager).await;
    let before = snapshot(&case.db).await;
    for principal in [&agent, &human] {
        assert!(matches!(
            submit(&case.db, &case.files, principal, input(&case)).await,
            Err(Error(OperationReason::Forbidden))
        ));
        let tx = identity::begin_write(&case.db, principal.organization_id())
            .await
            .unwrap();
        assert!(managed::submit_in_transaction(
            &tx,
            &tasks::ActorContext::authenticated(principal.clone()),
            &input(&case),
            &id()
        )
        .await
        .is_err());
        tx.rollback().await.unwrap();
        assert_eq!(snapshot(&case.db).await, before);
    }
    let other = tasks::store::create(
        &case.db,
        &tasks::ActorContext::authenticated(case.principal.clone()),
        tasks::types::CreateInput {
            title: "Different same-tenant task".into(),
            notes: String::new(),
            domain: identity::Domain::Feedback,
            priority: tasks::vocabulary::TaskPriority::Normal,
            due_date: None,
            owner_id: None,
            assignee_id: None,
            reviewer_id: None,
        },
    )
    .await
    .unwrap();
    let before = snapshot(&case.db).await;
    let mut request = input(&case);
    request.task_id = other.task.id;
    assert!(matches!(
        submit(&case.db, &case.files, &case.principal, request).await,
        Err(Error(OperationReason::Missing))
    ));
    assert_eq!(snapshot(&case.db).await, before);
    let mut empty = input(&case);
    empty.versions.clear();
    assert!(matches!(
        submit(&case.db, &case.files, &case.principal, empty).await,
        Err(Error(OperationReason::Invalid))
    ));
    assert_eq!(snapshot(&case.db).await, before);
}

#[tokio::test]
async fn execution_publication_late_authority_loss_cannot_publish_private_verified_bytes() {
    let case = setup().await;
    let request = input(&case);
    let ctx = tasks::ActorContext::authenticated(case.principal.clone());
    let mut before = None;
    let result = submit_with(&case.db, &case.files, &case.principal, request, async {
        tasks::store::cancel(
            &case.db,
            &ctx,
            tasks::types::RevisionInput {
                task_id: case.task.task.id.clone(),
                expected_revision: 1,
            },
        )
        .await
        .unwrap();
        tasks::store::progress(
            &case.db,
            &ctx,
            tasks::types::ProgressInput {
                task_id: case.task.task.id.clone(),
                expected_revision: 2,
                status: tasks::vocabulary::ProgressStatus::Todo,
            },
        )
        .await
        .unwrap();
        before = Some(snapshot(&case.db).await);
    })
    .await;
    assert!(matches!(
        result,
        Err(Error(OperationReason::AuthorityChanged))
    ));
    assert_eq!(snapshot(&case.db).await, before.unwrap());
}

#[tokio::test]
async fn execution_publication_cas_and_receipt_failure_leave_no_partial_disclosure() {
    let case = setup().await;
    let mut request = input(&case);
    request
        .versions
        .push(version(&case, 2, b"Second selected file version", "Revision two").await);
    // Failure after the first reference, after all references during task CAS,
    // and after task/audit during receipt completion must each roll back fully.
    for sql in [
        "CREATE TRIGGER reject_publication BEFORE INSERT ON business_execution_publication WHEN NEW.ordinal=1 BEGIN SELECT RAISE(ABORT,'synthetic reference failure'); END",
        "CREATE TRIGGER reject_publication BEFORE UPDATE ON business_task WHEN NEW.status='review' BEGIN SELECT RAISE(ABORT,'synthetic task CAS failure'); END",
        "CREATE TRIGGER reject_publication BEFORE UPDATE ON business_execution_operation WHEN NEW.kind='submit' AND NEW.status='confirmed' BEGIN SELECT RAISE(ABORT,'synthetic receipt failure'); END",
    ] {
        case.db.execute_unprepared(sql).await.unwrap();
        let before = snapshot(&case.db).await;
        assert!(submit(&case.db, &case.files, &case.principal, request.clone()).await.is_err());
        assert_eq!(snapshot(&case.db).await, before);
        case.db.execute_unprepared("DROP TRIGGER reject_publication").await.unwrap();
    }
    let result = submit(&case.db, &case.files, &case.principal, request)
        .await
        .unwrap();
    assert_eq!(result.detail.deliverables[0].assets.len(), 2);
}

#[tokio::test]
async fn execution_publication_missing_bytes_and_competing_revision_fail_closed() {
    let case = setup().await;
    let request = input(&case);
    let tx = case.db.begin().await.unwrap();
    let retained = manifest(&tx, &case.principal, &request).await.unwrap();
    tx.commit().await.unwrap();
    let mut other = request.clone();
    other.operation_id = id();
    let (a, b) = tokio::join!(
        commit_verified(&case.db, &case.principal, request, &retained),
        commit_verified(&case.db, &case.principal, other, &retained),
    );
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    assert!(matches!(
        a.err().or(b.err()),
        Some(Error(OperationReason::Conflict))
    ));
    let before = snapshot(&case.db).await;
    let mut next = input(&case);
    next.expected_task_revision = 2;
    // Remove only this test's private object to model failed storage restoration.
    let object = case
        .directory
        .path()
        .join("business-execution")
        .join("objects")
        .join(&retained[0].object_id);
    assert!(object.is_file());
    std::fs::remove_file(&object).unwrap();
    assert!(matches!(
        submit(&case.db, &case.files, &case.principal, next).await,
        Err(Error(OperationReason::ContentUnavailable))
    ));
    assert_eq!(snapshot(&case.db).await, before);
}

#[tokio::test]
async fn execution_publication_human_review_keeps_selected_version_after_session_revocation() {
    let case = setup().await;
    let result = submit(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    let reader = member(&case, identity::MemberKind::Human, identity::Role::Manager).await;
    let public_input = PublishedInput {
        task_id: case.task.task.id.clone(),
        deliverable_id: result.detail.deliverables[0].id.clone(),
        asset_id: case.asset_id.clone(),
        version_id: case.first.version_id.clone(),
    };
    tasks::store::review(
        &case.db,
        &tasks::ActorContext::authenticated(reader.clone()),
        tasks::types::ReviewInput {
            task_id: case.task.task.id.clone(),
            expected_revision: 2,
            decision: tasks::types::ReviewDecision::Accept,
            comment: "Reviewed exact file".into(),
        },
    )
    .await
    .unwrap();
    assert!(scope::session(
        &case.db.begin().await.unwrap(),
        &case.principal,
        &case.session_id
    )
    .await
    .is_err());
    let public: Value =
        serde_json::to_value(published(&case.db, &reader, public_input).await.unwrap()).unwrap();
    assert_eq!(public["version"]["versionId"], case.first.version_id);
    assert_eq!(
        public["publication"]["submittedBy"]["memberId"],
        case.principal.member_id()
    );
    let detail = tasks::store::get(
        &case.db,
        &tasks::ActorContext::authenticated(reader),
        tasks::types::TaskInput {
            task_id: case.task.task.id.clone(),
        },
    )
    .await
    .unwrap();
    let reviewed = detail
        .activity
        .iter()
        .find(|a| a.kind == "reviewed")
        .unwrap();
    assert_ne!(reviewed.actor.id, case.principal.member_id());
    assert_eq!(detail.deliverables[0].assets.len(), 1);
}

#[tokio::test]
async fn execution_publication_content_survives_scratch_and_rechecks_reader_after_io() {
    let case = setup().await;
    let result = submit(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    let reader = member(&case, identity::MemberKind::Human, identity::Role::Viewer).await;
    let request = PublishedContentInput {
        task_id: case.task.task.id.clone(),
        deliverable_id: result.detail.deliverables[0].id.clone(),
        asset_id: case.asset_id.clone(),
        version_id: case.first.version_id.clone(),
        disposition: Disposition::Preview,
    };
    std::fs::remove_dir_all(case.files.workspace(&case.admission_id).unwrap()).unwrap();
    let read = published_content(&case.db, &case.files, &reader, request.clone())
        .await
        .unwrap();
    assert_eq!(read.bytes, b"Exact first managed document");
    assert_eq!(read.metadata.sha256, digest(&read.bytes));
    assert_eq!(read.metadata.file_name, "Customer proposal.txt");
    let before = snapshot(&case.db).await;
    // This action runs after the actual blocking byte read but before its final
    // current identity check. No byte result may escape the revoked reader.
    let denied = published_content_with(&case.db, &case.files, &reader, request.clone(), async {
        case.db.execute(statement("UPDATE business_member SET status='revoked',revision=revision+1 WHERE organization_id=? AND id=?",
            vec![reader.organization_id().into(), reader.member_id().into()])).await.unwrap();
    }).await;
    assert!(denied.is_err());
    assert!(published_content(&case.db, &case.files, &reader, request)
        .await
        .is_err());
    assert_eq!(snapshot(&case.db).await, before);
}

#[tokio::test]
async fn execution_publication_aborted_after_byte_verification_never_commits() {
    let case = setup().await;
    let before = snapshot(&case.db).await;
    let db = case.db.clone();
    let principal = case.principal.clone();
    let files = case.files.clone();
    let request = input(&case);
    let (arrived, ready) = tokio::sync::oneshot::channel();
    let (_release, hold) = tokio::sync::oneshot::channel::<()>();
    let task = tokio::spawn(async move {
        submit_with(&db, &files, &principal, request, async {
            arrived.send(()).unwrap();
            hold.await.unwrap();
        })
        .await
    });
    ready.await.unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert_eq!(snapshot(&case.db).await, before);
}
