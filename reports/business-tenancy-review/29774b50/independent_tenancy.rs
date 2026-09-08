//! Reviewer-only full bridge epoch probe at 29774b50, no live model/engine process.
//! Apache Codeg f3813e3f1edb521f1d1b20d0b372643acc4123a5:
//! work_task/engine.rs and desk/{tests,business,business_ownership}.rs fixtures.
//! Actual backend CAS, private index, token listener and SQLite transactions.
use super::*;
use identity::platform::{self, PlatformContext};

async fn cycle(f: &Fixture, epoch: i64) {
    let p = PlatformContext::from_operator(&crate::web::auth::AuthenticatedOperator);
    for (expected, status) in [
        (epoch, OrganizationStatus::Suspended),
        (epoch + 1, OrganizationStatus::Active),
    ] {
        platform::status(
            &f.engine.db.conn,
            &p,
            platform::StatusInput {
                operation_id: uuid::Uuid::new_v4().to_string(),
                organization_id: f.operator.organization_id().into(),
                expected_revision: expected,
                expected_authorization_epoch: expected,
                status,
            },
        )
        .await
        .unwrap();
    }
}
async fn rows(f: &Fixture) -> Value {
    let detail = business::store::get(
        &f.engine.db.conn,
        &ActorContext::authenticated(f.operator.clone()),
        dto::TaskInput {
            task_id: f.task.task.id.clone(),
        },
    )
    .await
    .unwrap();
    let mut counts = Vec::new();
    for table in [
        "business_task_execution_authority",
        "business_execution_authority_epoch",
        "business_task_execution",
        "business_task_activity",
        "business_task_deliverable",
    ] {
        let count: i64 = f
            .engine
            .db
            .conn
            .query_one(Statement::from_string(
                sea_orm::DbBackend::Sqlite,
                format!("SELECT count(*) FROM {table}"),
            ))
            .await
            .unwrap()
            .unwrap()
            .try_get_by_index(0)
            .unwrap();
        counts.push(count);
    }
    json!({"detail":detail,"counts":counts})
}

#[tokio::test]
async fn review_297_indexed_run_rejects_old_entrustment_and_preserves_exact_human_grant() {
    let (engine, id) = running_task().await;
    let mut f = unlinked_fixture_engine(engine, id).await;
    let issued = identities::issue_credential(
        &f.engine.db.conn,
        &f.operator,
        IssueCredentialInput {
            organization_id: f.operator.organization_id().into(),
            member_id: f.delegator.member_id().into(),
            label: "Synthetic original delegation lineage".into(),
        },
    )
    .await
    .unwrap();
    f.delegator = identities::resolve_credential(&f.engine.db.conn, &issued.token)
        .await
        .unwrap();
    f.credential_id = issued.credential.id.clone();
    f.task = f
        .engine
        .entrust_business_execution(
            ActorContext::authenticated(f.operator.clone()),
            link_input(&f, 1),
        )
        .await
        .unwrap();
    assert_eq!(f.task.task.revision, 2);
    let (indexed, agent) = f.engine.desk_scope(PARENT_CONN).await.unwrap();
    assert_eq!(indexed.id, id);
    assert_eq!(agent, "pi");
    let old_run = indexed.run_seq;
    assert!(!context(&f).await.ok, "entrustment is not a delegation");
    cycle(&f, 1).await;
    // Both credentials are deliberately authenticated afresh. The old source
    // itself, not merely a stale Principal, must deny the initial human link.
    f.operator = identity::operator_principal(&f.engine.db.conn)
        .await
        .unwrap();
    f.delegator = identities::resolve_credential(&f.engine.db.conn, &issued.token)
        .await
        .unwrap();
    assert_eq!(f.delegator.authorization_epoch(), 3);
    assert_eq!(
        f.engine.desk_scope(PARENT_CONN).await.unwrap().0.run_seq,
        old_run
    );
    let before = rows(&f).await;
    assert!(matches!(
        f.engine
            .link_business_execution(
                ActorContext::authenticated(f.delegator.clone()),
                link_input(&f, 2)
            )
            .await,
        Err(identity::IdentityError::Forbidden)
    ));
    assert_eq!(rows(&f).await, before);
    assert!(!context(&f).await.ok);

    // A real newly CAS-minted generation can receive explicit new protected
    // entrustment and a separate Assign-authorized human credential grant.
    let new_run = relaunch_on(&f.engine, id, PARENT_CONN).await;
    assert!(new_run > old_run);
    f.task = f
        .engine
        .entrust_business_execution(
            ActorContext::authenticated(f.operator.clone()),
            link_input(&f, 2),
        )
        .await
        .unwrap();
    f.task = f
        .engine
        .link_business_execution(
            ActorContext::authenticated(f.delegator.clone()),
            link_input(&f, 3),
        )
        .await
        .unwrap();
    assert_eq!(f.task.task.revision, 4);
    let grant: String = f
        .engine
        .db
        .conn
        .query_one(Statement::from_string(
            sea_orm::DbBackend::Sqlite,
            "SELECT delegation_json FROM business_task_execution".to_owned(),
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get_by_index(0)
        .unwrap();
    let value: Value = serde_json::from_str(&grant).unwrap();
    assert_eq!(value["credential_id"], issued.credential.id);
    assert_eq!(value["delegator_id"], f.delegator.member_id());
    assert_eq!(value["operator"], false);
    assert_eq!(value["authorization_epoch"], 3);
    assert!(!grant.contains(&issued.token));
    let response = call(
        &f.listener,
        &f.token,
        DeskTool::DeskBusinessNote,
        json!({"expectedRevision":4,"body":"Allowed scoped contribution"}),
    )
    .await;
    assert_eq!(public(response)["task"]["revision"], 5);
    cycle(&f, 3).await;
    f.operator = identity::operator_principal(&f.engine.db.conn)
        .await
        .unwrap();
    let before = rows(&f).await;
    let denied = call(
        &f.listener,
        &f.token,
        DeskTool::DeskBusinessNote,
        json!({"expectedRevision":5,"body":"Old grant must not refresh"}),
    )
    .await;
    assert_eq!(denied.code, Some(DeskError::Denied));
    assert_eq!(rows(&f).await, before);
    let retained: String = f
        .engine
        .db
        .conn
        .query_one(Statement::from_string(
            sea_orm::DbBackend::Sqlite,
            "SELECT delegation_json FROM business_task_execution".to_owned(),
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get_by_index(0)
        .unwrap();
    assert_eq!(retained, grant);
    assert!(
        identities::resolve_credential(&f.engine.db.conn, &issued.token)
            .await
            .is_ok()
    );
}
