//! Real identity/writer admissions, with a synthetic linkage only: no spawn.
use super::{common::*, session_store as store, types::*};
use crate::{business_identity as identity, business_tasks as tasks, db::migration::Migrator};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

pub(super) async fn setup() -> (
    DatabaseConnection,
    identity::Principal,
    tasks::types::Detail,
    ProfileSummary,
) {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    let (op, task) = super::tests::task(&db).await;
    let p = store::sync_profiles(&db, &op, vec![profile("a")])
        .await
        .unwrap()
        .profiles
        .remove(0);
    (db, op, task, p)
}
pub(super) fn profile(hash: &str) -> store::DiscoveredProfile {
    store::DiscoveredProfile {
        config_key: "synthetic-existing-client".into(),
        config_hash: hash.repeat(64),
        summary: ProfileSummary {
            id: String::new(),
            revision: 1,
            label: "Synthetic ACP".into(),
            client_id: "pi".into(),
            modes: vec![Mode::Chat],
            custody: Custody::OriginalOperator,
            model: Some(ConfiguredModel {
                id: "gpt-6-astra".into(),
                reasoning: "max".into(),
            }),
            readiness: Readiness::Ready,
            reason: None,
            capabilities: ProfileCapabilities {
                start: true,
                r#continue: true,
                managed_output: true,
                office_preview: false,
            },
        },
    }
}
pub(super) fn start(task: &tasks::types::Detail, p: &ProfileSummary) -> StartInput {
    StartInput {
        operation_id: id(),
        task_id: task.task.id.clone(),
        expected_task_revision: task.task.revision,
        profile_id: p.id.clone(),
        expected_profile_revision: p.revision,
        mode: Mode::Chat,
    }
}
pub(super) fn link() -> store::EngineLink {
    store::EngineLink {
        connection_id: id(),
        conversation_id: None,
        folder_id: None,
    }
}
async fn count(db: &DatabaseConnection, table: &str) -> i64 {
    db.query_one(statement(
        &format!("SELECT count(*) AS n FROM {table}"),
        vec![],
    ))
    .await
    .unwrap()
    .unwrap()
    .try_get("", "n")
    .unwrap()
}

#[tokio::test]
async fn execution_admission_replay_and_changed_input_never_mint_second_launch() {
    let (db, op, task, p) = setup().await;
    assert!(task.task.assignee_id.is_none());
    let input = start(&task, &p);
    let first = store::reserve_start(&db, &op, &input).await.unwrap();
    assert_eq!(first.result.operation.status, OperationStatus::Pending);
    let same = store::reserve_start(&db, &op, &input).await.unwrap();
    assert!(same.admission.is_none());
    assert_eq!(same.result.session.id, first.result.session.id);
    let mut different = input.clone();
    different.mode = Mode::Terminal;
    assert!(matches!(
        store::reserve_start(&db, &op, &different).await,
        Err(Error(OperationReason::Conflict))
    ));
    let admitted = first.admission.unwrap();
    let confirmed = store::complete_launch(&db, &admitted, &link())
        .await
        .unwrap();
    assert_eq!(confirmed.operation.status, OperationStatus::Confirmed);
    assert_eq!(confirmed.session.revision, 2);
    assert!(store::complete_launch(&db, &admitted, &link())
        .await
        .is_err());
    let replay = store::reserve_start(&db, &op, &input).await.unwrap();
    assert!(replay.admission.is_none());
    assert_eq!(replay.result.operation.status, OperationStatus::Confirmed);
    assert_eq!(count(&db, "business_execution_session").await, 1);
    assert_eq!(count(&db, "business_execution_generation").await, 1);
    assert_eq!(count(&db, "business_execution_operation").await, 1);
    assert_eq!(count(&db, "work_task").await, 0);
}

#[tokio::test]
async fn execution_authority_owner_credentials_and_other_tenant_cannot_use_private_operations() {
    let (db, op, task, p) = setup().await;
    let input = start(&task, &p);
    let reserved = store::reserve_start(&db, &op, &input).await.unwrap();
    let own_credential = identity::store::issue_credential(
        &db,
        &op,
        identity::types::IssueCredentialInput {
            organization_id: op.organization_id().into(),
            member_id: op.member_id().into(),
            label: "Synthetic member client".into(),
        },
    )
    .await
    .unwrap();
    let owner_member = identity::store::resolve_credential(&db, &own_credential.token)
        .await
        .unwrap();
    assert!(!owner_member.is_operator());
    let provisioned = identity::platform::create(
        &db,
        &identity::platform::PlatformContext::from_operator(
            &crate::web::auth::AuthenticatedOperator,
        ),
        identity::platform::CreateTenantInput {
            operation_id: id(),
            organization_name: "Other tenant".into(),
            owner_name: "Other human".into(),
        },
    )
    .await
    .unwrap();
    let tenant = identity::store::resolve_credential(&db, provisioned.token.as_ref().unwrap())
        .await
        .unwrap();
    for principal in [&owner_member, &tenant] {
        assert!(matches!(
            store::sync_profiles(&db, principal, vec![profile("b")]).await,
            Err(Error(OperationReason::Forbidden))
        ));
        assert!(matches!(
            store::reserve_start(&db, principal, &input).await,
            Err(Error(OperationReason::Forbidden))
        ));
        assert!(matches!(
            store::get(
                &db,
                principal,
                SessionInput {
                    session_id: reserved.result.session.id.clone()
                }
            )
            .await,
            Err(Error(OperationReason::Forbidden))
        ));
        assert!(matches!(
            store::operation(
                &db,
                principal,
                OperationInput {
                    operation_id: input.operation_id.clone(),
                    kind: OperationKind::Start
                }
            )
            .await,
            Err(Error(OperationReason::Forbidden))
        ));
    }
    assert_eq!(count(&db, "business_execution_session").await, 1);
    assert_eq!(count(&db, "business_execution_operation").await, 1);
    let grant: serde_json::Value = db
        .query_one(statement(
            "SELECT authority_json FROM business_execution_session",
            vec![],
        ))
        .await
        .unwrap()
        .map(|r| r.try_get::<String>("", "authority_json").unwrap())
        .map(|s| serde_json::from_str(&s).unwrap())
        .unwrap();
    assert_eq!(grant["operator"], true);
    assert!(grant["credential_id"].is_null());
    assert_eq!(grant["delegator_id"], op.member_id());
}

#[tokio::test]
async fn execution_authority_profile_away_back_and_late_launch_stay_revoked() {
    let (db, op, task, p) = setup().await;
    let input = start(&task, &p);
    let admission = store::reserve_start(&db, &op, &input)
        .await
        .unwrap()
        .admission
        .unwrap();
    store::sync_profiles(&db, &op, vec![profile("b")])
        .await
        .unwrap();
    let restored = store::sync_profiles(&db, &op, vec![profile("a")])
        .await
        .unwrap()
        .profiles
        .remove(0);
    assert_eq!(restored.revision, 3);
    assert!(matches!(
        store::complete_launch(&db, &admission, &link()).await,
        Err(Error(OperationReason::AuthorityChanged))
    ));
    assert!(matches!(
        store::reserve_start(&db, &op, &input).await,
        Err(Error(OperationReason::AuthorityChanged))
    ));
    let row = db
        .query_one(statement(
            "SELECT status,generation FROM business_execution_session",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<String>("", "status").unwrap(), "revoked");
    assert_eq!(row.try_get::<i64>("", "generation").unwrap(), 2);
    assert_eq!(count(&db, "business_execution_operation").await, 1);
}

#[tokio::test]
async fn execution_authority_task_cancel_reopen_and_fresh_tenant_login_do_not_rebind_history() {
    let (db, op, task, p) = setup().await;
    let input = start(&task, &p);
    let admission = store::reserve_start(&db, &op, &input)
        .await
        .unwrap()
        .admission
        .unwrap();
    let ctx = tasks::ActorContext::authenticated(op.clone());
    let cancelled = tasks::store::cancel(
        &db,
        &ctx,
        tasks::types::RevisionInput {
            task_id: task.task.id.clone(),
            expected_revision: 1,
        },
    )
    .await
    .unwrap();
    tasks::store::progress(
        &db,
        &ctx,
        tasks::types::ProgressInput {
            task_id: task.task.id.clone(),
            expected_revision: cancelled.task.revision,
            status: tasks::vocabulary::ProgressStatus::Todo,
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        store::complete_launch(&db, &admission, &link()).await,
        Err(Error(OperationReason::AuthorityChanged))
    ));
    // Original operator is still a captured epoch; freshly authenticated original
    // transport must not resurrect a session from before suspend/resume.
    for state in ["suspended", "active"] {
        db.execute(statement("UPDATE business_organization SET status=?,revision=revision+1,authorization_epoch=authorization_epoch+1 WHERE id=?",
            vec![state.into(), op.organization_id().into()])).await.unwrap();
    }
    let fresh = identity::operator_principal(&db).await.unwrap();
    assert_eq!(fresh.authorization_epoch(), 3);
    assert!(matches!(
        store::get(
            &db,
            &fresh,
            SessionInput {
                session_id: admission.session_id().into()
            }
        )
        .await,
        Err(Error(OperationReason::AuthorityChanged))
    ));
    assert!(matches!(
        store::reserve_start(&db, &fresh, &input).await,
        Err(Error(OperationReason::AuthorityChanged))
    ));
}
