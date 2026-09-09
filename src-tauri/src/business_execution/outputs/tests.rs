//! Real file scans and DB fences; session engine links remain explicit synthetic
//! records. No client, engine, model, fixture server or provider is launched.
use super::*;
use crate::business_execution::{session_store, session_tests};
use std::path::PathBuf;

struct Case {
    _directory: tempfile::TempDir,
    db: DatabaseConnection,
    principal: Principal,
    files: files::Files,
    session_id: String,
    workspace: PathBuf,
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
    let tx = db.begin().await.unwrap();
    let current = source(&tx, &principal, &session_id).await.unwrap();
    tx.rollback().await.unwrap();
    let directory = tempfile::tempdir().unwrap();
    let files = files::Files::new(directory.path().into());
    let workspace = files.workspace(&current.admission_id).unwrap();
    Case {
        _directory: directory,
        db,
        principal,
        files,
        session_id,
        workspace,
    }
}
fn input(case: &Case) -> OutputListInput {
    OutputListInput {
        session_id: case.session_id.clone(),
        cursor: None,
        limit: 50,
    }
}
async fn snapshot(db: &DatabaseConnection) -> Vec<String> {
    db.query_all(statement(
        "SELECT json_array(id,organization_id,session_id,generation,revision,relative_path,metadata_json) AS value FROM business_execution_output ORDER BY id", vec![],
    )).await.unwrap().into_iter().map(|r| r.try_get("", "value").unwrap()).collect()
}

#[tokio::test]
async fn execution_outputs_revisioned_discovery_and_pagination_do_not_publish() {
    let case = setup().await;
    for name in ["brief.md", "notes.txt", "summary.json"] {
        std::fs::write(case.workspace.join(name), b"Exact source observation").unwrap();
    }
    let initial = list(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    assert_eq!(initial.items.len(), 3);
    assert!(initial
        .items
        .iter()
        .all(|r| r.revision == 1 && r.status == OutputStatus::Available));
    let before = snapshot(&case.db).await;
    let mut request = input(&case);
    request.limit = 1;
    let mut ids = vec![];
    loop {
        let page = list(&case.db, &case.files, &case.principal, request.clone())
            .await
            .unwrap();
        assert_eq!(page.items.len(), 1);
        ids.push(page.items[0].id.clone());
        request.cursor = page.next_cursor;
        if request.cursor.is_none() {
            break;
        }
    }
    assert_eq!(
        ids,
        initial
            .items
            .iter()
            .map(|r| r.id.clone())
            .collect::<Vec<_>>()
    );
    assert_eq!(snapshot(&case.db).await, before);
    let original = initial.items.iter().find(|r| r.name == "brief.md").unwrap();
    std::fs::write(
        case.workspace.join("brief.md"),
        b"Revised bytes and changed length",
    )
    .unwrap();
    let changed = list(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    let changed = changed.items.iter().find(|r| r.id == original.id).unwrap();
    assert_eq!(changed.revision, 2);
    assert_eq!(changed.status, OutputStatus::Available);
    assert_ne!(changed.byte_size, original.byte_size);
    std::fs::remove_file(case.workspace.join("brief.md")).unwrap();
    let removed = list(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    let removed = removed.items.iter().find(|r| r.id == original.id).unwrap();
    assert_eq!(removed.revision, 3);
    assert_eq!(removed.status, OutputStatus::Changed);
    let json = serde_json::to_string(&removed).unwrap();
    for private in [
        "admission",
        "relative_path",
        "stamp",
        "sha256",
        case.workspace.to_str().unwrap(),
    ] {
        assert!(!json.contains(private));
    }
    let count: i64 = case
        .db
        .query_one(statement(
            "SELECT count(*) AS n FROM business_execution_asset_version",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "n")
        .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
async fn execution_outputs_foreign_cursor_and_member_transport_fail_before_scan() {
    let case = setup().await;
    std::fs::write(case.workspace.join("brief.md"), b"Private source").unwrap();
    let page = list(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    let before = snapshot(&case.db).await;
    let mut wrong = input(&case);
    wrong.cursor = Some(id());
    assert!(matches!(
        list(&case.db, &case.files, &case.principal, wrong).await,
        Err(Error(OperationReason::Missing))
    ));
    let credential = identity::store::issue_credential(
        &case.db,
        &case.principal,
        identity::types::IssueCredentialInput {
            organization_id: case.principal.organization_id().into(),
            member_id: case.principal.member_id().into(),
            label: "Synthetic owner member transport".into(),
        },
    )
    .await
    .unwrap();
    let owner = identity::store::resolve_credential(&case.db, &credential.token)
        .await
        .unwrap();
    assert!(matches!(
        list(&case.db, &case.files, &owner, input(&case)).await,
        Err(Error(OperationReason::Forbidden))
    ));
    // A second real admission, same operator/task/profile, has a distinct source
    // namespace. An otherwise well-formed cursor cannot cross it.
    let tx = case.db.begin().await.unwrap();
    let row = scope::session(&tx, &case.principal, &case.session_id)
        .await
        .unwrap();
    let task = scope::task(&tx, &case.principal, &row.task_id, false)
        .await
        .unwrap();
    tx.rollback().await.unwrap();
    let second = session_store::reserve_start(
        &case.db,
        &case.principal,
        &StartInput {
            operation_id: id(),
            task_id: row.task_id,
            expected_task_revision: task.task.revision,
            profile_id: row.profile_id,
            expected_profile_revision: row.profile_revision,
            mode: Mode::Chat,
        },
    )
    .await
    .unwrap();
    session_store::complete_launch(&case.db, &second.admission.unwrap(), &session_tests::link())
        .await
        .unwrap();
    let foreign = OutputListInput {
        session_id: second.result.session.id,
        cursor: Some(page.items[0].id.clone()),
        limit: 1,
    };
    assert!(matches!(
        list(&case.db, &case.files, &case.principal, foreign).await,
        Err(Error(OperationReason::Missing))
    ));
    assert_eq!(snapshot(&case.db).await, before);
}

#[tokio::test]
async fn execution_outputs_late_profile_revocation_retains_no_observation() {
    let case = setup().await;
    std::fs::write(
        case.workspace.join("brief.md"),
        b"Bytes scanned before revoke",
    )
    .unwrap();
    let result = list_with(
        &case.db,
        &case.files,
        &case.principal,
        input(&case),
        async {
            session_store::sync_profiles(
                &case.db,
                &case.principal,
                vec![session_tests::profile("b")],
            )
            .await
            .unwrap();
        },
    )
    .await;
    assert!(matches!(
        result,
        Err(Error(OperationReason::AuthorityChanged))
    ));
    assert!(snapshot(&case.db).await.is_empty());
}

#[tokio::test]
async fn execution_outputs_reordered_scan_cannot_replace_newer_observation() {
    let case = setup().await;
    std::fs::write(case.workspace.join("brief.md"), b"First observation").unwrap();
    let initial = list(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    let mut committed = None;
    let result = list_with(
        &case.db,
        &case.files,
        &case.principal,
        input(&case),
        async {
            std::fs::write(
                case.workspace.join("brief.md"),
                b"A newer accepted source observation",
            )
            .unwrap();
            let newer = list(&case.db, &case.files, &case.principal, input(&case))
                .await
                .unwrap();
            assert_eq!(newer.items[0].id, initial.items[0].id);
            assert_eq!(newer.items[0].revision, 2);
            committed = Some(snapshot(&case.db).await);
        },
    )
    .await;
    assert!(matches!(result, Err(Error(OperationReason::Conflict))));
    assert_eq!(snapshot(&case.db).await, committed.unwrap());
}

#[tokio::test]
async fn execution_outputs_stop_fence_and_new_generation_cannot_rebind_old_scan() {
    let case = setup().await;
    std::fs::write(
        case.workspace.join("brief.md"),
        b"Stopped generation output",
    )
    .unwrap();
    let original = list(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    let before = snapshot(&case.db).await;
    std::fs::write(
        case.workspace.join("brief.md"),
        b"Late bytes must not replace the observed candidate",
    )
    .unwrap();
    let result = list_with(&case.db, &case.files, &case.principal, input(&case), async {
        // Synthetic owned-run stop boundary only; not a process-teardown proof.
        case.db.execute(statement("UPDATE business_execution_session SET generation=generation+1,revision=revision+1,status='stopped' WHERE id=?", vec![case.session_id.clone().into()])).await.unwrap();
    }).await;
    assert!(matches!(result, Err(Error(OperationReason::Conflict))));
    assert_eq!(snapshot(&case.db).await, before);
    let stopped = list(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    assert_eq!(stopped.items.len(), 1);
    assert_eq!(stopped.items[0].revision, original.items[0].revision);
    assert_eq!(stopped.items[0].byte_size, original.items[0].byte_size);
    assert_eq!(snapshot(&case.db).await, before);
    let before = snapshot(&case.db).await;
    let admission = id();
    let tx = identity::begin_write(&case.db, case.principal.organization_id())
        .await
        .unwrap();
    tx.execute(statement("INSERT INTO business_execution_generation(organization_id,session_id,generation,admission_id,initial_cursor,engine_json,created_at) VALUES(?,?,3,?,?,?,?)",
        vec![case.principal.organization_id().into(), case.session_id.clone().into(), admission.clone().into(), id().into(), encode(&session_tests::link()).unwrap().into(), now().into()],
    )).await.unwrap();
    tx.execute(statement("UPDATE business_execution_session SET generation=3,revision=revision+1,status='idle' WHERE id=?", vec![case.session_id.clone().into()])).await.unwrap();
    tx.commit().await.unwrap();
    let workspace = case.files.workspace(&admission).unwrap();
    std::fs::write(workspace.join("brief.md"), b"New generation output").unwrap();
    let mut old_cursor = input(&case);
    old_cursor.cursor = Some(stopped.items[0].id.clone());
    assert!(matches!(
        list(&case.db, &case.files, &case.principal, old_cursor).await,
        Err(Error(OperationReason::Missing))
    ));
    assert_eq!(snapshot(&case.db).await, before);
    let fresh = list(&case.db, &case.files, &case.principal, input(&case))
        .await
        .unwrap();
    assert_ne!(fresh.items[0].id, stopped.items[0].id);
}
