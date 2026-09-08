//! Reviewer-only probes at 29774b50; no product correction or real credentials.
//! Apache Codeg f3813e3f1edb521f1d1b20d0b372643acc4123a5:
//! business_identity/tests/transactions.rs and db/test_helpers.rs fixtures.
//! Review febbdc7f retained-row/receipt probe adapted for the new epoch sidecar.
//! Installed SeaORM1.1.19, SQLx0.8.6 and axum-test17.3.0 are API references.
use super::*;
use crate::business_identity::platform::{self, PlatformContext};
use crate::db::migration::Migrator;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

const TENANCY: &str = "m20260908_000012_business_tenancy";

fn platform() -> PlatformContext {
    PlatformContext::from_operator(&crate::web::auth::AuthenticatedOperator)
}
async fn scalar(conn: &DatabaseConnection, sql: &str) -> i64 {
    conn.query_one(store::statement(sql, vec![]))
        .await
        .unwrap()
        .unwrap()
        .try_get_by_index(0)
        .unwrap()
}
async fn snapshot(conn: &DatabaseConnection, table: &str) -> String {
    // Fixed fixture tables only. Preserve every column of retained authority/grant.
    let columns = conn
        .query_all(store::statement(
            &format!("PRAGMA table_info({table})"),
            vec![],
        ))
        .await
        .unwrap()
        .iter()
        .map(|r| {
            format!(
                "\"{}\"",
                r.try_get::<String>("", "name")
                    .unwrap()
                    .replace('"', "\"\"")
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    conn.query_one(store::statement(&format!("SELECT json_group_array(json_array({columns})) FROM (SELECT * FROM {table} ORDER BY id)"),vec![]))
        .await.unwrap().unwrap().try_get_by_index(0).unwrap()
}
async fn create(conn: &DatabaseConnection, name: &str) -> platform::ProvisionResult {
    platform::create(
        conn,
        &platform(),
        platform::CreateTenantInput {
            operation_id: uuid::Uuid::new_v4().to_string(),
            organization_name: name.into(),
            owner_name: format!("{name} owner"),
        },
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn review_297_sidecar_retains_authority_and_grant_across_receipt_retry_and_owner_fks() {
    let conn = Database::connect("sqlite::memory:").await.unwrap();
    let count = Migrator::migrations()
        .iter()
        .position(|m| m.name() == TENANCY)
        .unwrap();
    Migrator::up(&conn, Some(count.try_into().unwrap()))
        .await
        .unwrap();
    conn.execute_unprepared(r#"
INSERT INTO business_organization VALUES ('original',1,'Retained team','before');
INSERT INTO business_member (id,organization_id,display_name,kind,role,domains_json,operator_owner,created_at,updated_at)
 VALUES ('owner','original','Owner','human','owner','["feedback"]',1,'before','before'),
        ('agent','original','Agent','agent','member','["feedback"]',0,'before','before');
INSERT INTO business_credential VALUES ('credential','original','owner','aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa','synthetic','before',NULL);
INSERT INTO business_task (id,organization_id,title,domain,owner_id,assignee_id,creator_id,created_at,updated_at)
 VALUES ('task','original','Retained public work','feedback','owner','agent','owner','before','before');
INSERT INTO business_task_execution_authority VALUES ('authority','original','task','feedback',1,37,2,'synthetic-root','agent','pi','owner','before');
INSERT INTO business_task_execution VALUES ('execution','original','task','authority',37,2,'synthetic-root','agent','pi','{"organization_id":"original","delegator_id":"owner","credential_id":"credential","operator":false}','owner','before',NULL);
CREATE TRIGGER review_receipt_failure BEFORE INSERT ON seaql_migrations
 WHEN NEW.version='m20260908_000012_business_tenancy' BEGIN SELECT RAISE(ABORT,'synthetic receipt failure'); END;
"#).await.unwrap();
    let original_authority = snapshot(&conn, "business_task_execution_authority").await;
    let original_grant = snapshot(&conn, "business_task_execution").await;
    assert!(Migrator::up(&conn, None).await.is_err());
    assert_eq!(
        snapshot(&conn, "business_task_execution_authority").await,
        original_authority
    );
    assert_eq!(
        snapshot(&conn, "business_task_execution").await,
        original_grant
    );
    assert_eq!(scalar(&conn,"SELECT authorization_epoch FROM business_execution_authority_epoch WHERE authority_id='authority'").await,1);
    assert_eq!(scalar(&conn,"SELECT count(*) FROM seaql_migrations WHERE version='m20260908_000012_business_tenancy'").await,0);
    for (revision, status) in [
        (1, OrganizationStatus::Suspended),
        (2, OrganizationStatus::Active),
    ] {
        platform::status(
            &conn,
            &platform(),
            platform::StatusInput {
                operation_id: uuid::Uuid::new_v4().to_string(),
                organization_id: "original".into(),
                expected_revision: revision,
                expected_authorization_epoch: revision,
                status,
            },
        )
        .await
        .unwrap();
    }
    // A post-schema-commit authority captures3; migration receipt retry must not
    // backfill it to1 or rewrite the old source/grant. No actual run is claimed.
    conn.execute_unprepared("INSERT INTO business_task_execution_authority VALUES ('later-authority','original','task','feedback',1,37,3,'later-root','agent','pi','owner','after'); DROP TRIGGER review_receipt_failure;").await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    assert_eq!(scalar(&conn,"SELECT authorization_epoch FROM business_execution_authority_epoch WHERE authority_id='authority'").await,1);
    assert_eq!(scalar(&conn,"SELECT authorization_epoch FROM business_execution_authority_epoch WHERE authority_id='later-authority'").await,3);
    assert_eq!(
        snapshot(&conn, "business_task_execution").await,
        original_grant
    );
    assert_eq!(
        scalar(
            &conn,
            "SELECT count(*) FROM business_task_execution_authority"
        )
        .await,
        2
    );
    assert_eq!(scalar(&conn,"SELECT count(*) FROM seaql_migrations WHERE version='m20260908_000012_business_tenancy'").await,1);
    for sql in [
        "INSERT INTO business_execution_authority_epoch VALUES ('missing-authority',3)",
        "UPDATE business_execution_authority_epoch SET authorization_epoch=3 WHERE authority_id='authority'",
        "DELETE FROM business_execution_authority_epoch WHERE authority_id='authority'",
    ] { assert!(conn.execute_unprepared(sql).await.is_err()); }

    // Both foreign identities exist; this proves composite owner/credential
    // lineage enforcement, not merely rejection of a nonexistent UUID.
    let a = create(&conn, "A").await;
    let b = create(&conn, "B").await;
    let a_owner = store::resolve_credential(&conn, a.token.as_ref().unwrap())
        .await
        .unwrap();
    let (_, other_credential, _) =
        human(&conn, &a_owner, Role::Member, vec![Domain::Feedback]).await;
    for credential in [&b.metadata.credential.id, &other_credential.credential.id] {
        assert!(conn
            .execute(store::statement(
                "UPDATE business_provisioning_owner SET credential_id=? WHERE organization_id=?",
                vec![
                    credential.clone().into(),
                    a.metadata.organization.id.clone().into()
                ]
            ))
            .await
            .is_err());
    }
    let kept: String = conn
        .query_one(store::statement(
            "SELECT credential_id FROM business_provisioning_owner WHERE organization_id=?",
            vec![a.metadata.organization.id.clone().into()],
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get_by_index(0)
        .unwrap();
    assert_eq!(kept, a.metadata.credential.id);
    assert_eq!(
        store::organization(&conn).await.unwrap().unwrap().id,
        "original"
    );
    assert_eq!(scalar(&conn, "PRAGMA foreign_keys").await, 1);
    assert!(conn
        .query_all(store::statement("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn review_297_protected_recovery_receipt_rollback_and_lifecycle_replay() {
    let db = fresh_in_memory_db().await;
    let dir = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let server = axum_test::TestServer::new(crate::web::router::build_router(
        state.clone(),
        "review-only-operator".into(),
        dir.path().into(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    ))
    .unwrap();
    let op = "Bearer review-only-operator";
    let response = server.post("/api/platform/business/tenants/create").add_header("authorization",op)
        .json(&json!({"input":{"operationId":uuid::Uuid::new_v4().to_string(),"organizationName":"Recovered tenant","ownerName":"Owner"}})).await;
    response.assert_status_ok();
    assert_eq!(response.header("cache-control"), "no-store");
    let created = response.json::<Value>();
    let old_token = created["token"].as_str().unwrap().to_owned();
    let owner = store::resolve_credential(&state.db.conn, &old_token)
        .await
        .unwrap();
    assert!(!owner.is_operator());
    let recovery = json!({"operationId":uuid::Uuid::new_v4().to_string(),"organizationId":created["organization"]["id"],"expectedRevision":1,"expectedAuthorizationEpoch":1,"ownerMemberId":created["ownerMemberId"],"expectedOwnerRevision":1,"expectedCredentialId":created["credential"]["id"]});
    let before_credentials = snapshot(&state.db.conn, "business_credential").await;
    state.db.conn.execute_unprepared("CREATE TRIGGER review_recovery_receipt_failure BEFORE INSERT ON business_platform_receipt WHEN NEW.action='owner_credential_reissued' BEGIN SELECT RAISE(ABORT,'synthetic receipt failure'); END;").await.unwrap();
    let failure = server
        .post("/api/platform/business/tenants/reissue-owner-credential")
        .add_header("authorization", op)
        .json(&json!({"input":recovery}))
        .await;
    failure.assert_status_internal_server_error();
    assert_eq!(failure.header("cache-control"), "no-store");
    assert_eq!(
        snapshot(&state.db.conn, "business_credential").await,
        before_credentials
    );
    assert_eq!(
        scalar(
            &state.db.conn,
            "SELECT count(*) FROM business_platform_receipt"
        )
        .await,
        1
    );
    let listing = platform::list(&state.db.conn, &platform()).await.unwrap();
    assert_eq!(
        listing[0].credential.as_ref().unwrap().id,
        created["credential"]["id"]
    );
    assert!(store::resolve_credential(&state.db.conn, &old_token)
        .await
        .is_ok());
    state
        .db
        .conn
        .execute_unprepared("DROP TRIGGER review_recovery_receipt_failure")
        .await
        .unwrap();
    let issued = server
        .post("/api/platform/business/tenants/reissue-owner-credential")
        .add_header("authorization", op)
        .json(&json!({"input":recovery}))
        .await;
    issued.assert_status_ok();
    let issued = issued.json::<Value>();
    let new_token = issued["token"].as_str().unwrap();
    let new_bearer = format!("Bearer {new_token}");
    assert!(store::resolve_credential(&state.db.conn, &old_token)
        .await
        .is_err());
    let replay = server
        .post("/api/platform/business/tenants/reissue-owner-credential")
        .add_header("authorization", op)
        .json(&json!({"input":recovery}))
        .await;
    replay.assert_status_ok();
    let replay = replay.json::<Value>();
    assert!(replay.get("token").is_none());
    assert_eq!(replay["credential"], issued["credential"]);
    assert_eq!(replay["delivery"], "already_provisioned");
    let mut stale = recovery.clone();
    stale["operationId"] = json!(uuid::Uuid::new_v4().to_string());
    server
        .post("/api/platform/business/tenants/reissue-owner-credential")
        .add_header("authorization", op)
        .json(&json!({"input":stale}))
        .await
        .assert_status_conflict();

    let suspend = json!({"operationId":uuid::Uuid::new_v4().to_string(),"organizationId":created["organization"]["id"],"expectedRevision":1,"expectedAuthorizationEpoch":1,"status":"suspended"});
    server
        .post("/api/platform/business/tenants/status")
        .add_header("authorization", op)
        .json(&json!({"input":suspend}))
        .await
        .assert_status_ok();
    server
        .post("/api/business/context")
        .add_header("authorization", new_bearer.as_str())
        .json(&json!({"input":{}}))
        .await
        .assert_status_unauthorized();
    let mut while_suspended = recovery.clone();
    while_suspended["operationId"] = json!(uuid::Uuid::new_v4().to_string());
    while_suspended["expectedRevision"] = json!(2);
    while_suspended["expectedAuthorizationEpoch"] = json!(2);
    while_suspended["expectedCredentialId"] = issued["credential"]["id"].clone();
    server
        .post("/api/platform/business/tenants/reissue-owner-credential")
        .add_header("authorization", op)
        .json(&json!({"input":while_suspended}))
        .await
        .assert_status_forbidden();
    let resume = json!({"operationId":uuid::Uuid::new_v4().to_string(),"organizationId":created["organization"]["id"],"expectedRevision":2,"expectedAuthorizationEpoch":2,"status":"active"});
    server
        .post("/api/platform/business/tenants/status")
        .add_header("authorization", op)
        .json(&json!({"input":resume}))
        .await
        .assert_status_ok();
    let fresh = server
        .post("/api/business/context")
        .add_header("authorization", new_bearer.as_str())
        .json(&json!({"input":{}}))
        .await;
    fresh.assert_status_ok();
    assert_eq!(
        fresh.json::<Value>()["organization"]["authorizationEpoch"],
        3
    );
    assert!(authorize(
        &state.db.conn,
        &owner,
        owner.organization_id(),
        Permission::Read,
        None
    )
    .await
    .is_err());
    // A historical operation replay is its original metadata, not a new status
    // transition: the active tenant/epoch and receipt count must stay unchanged.
    let replay = server
        .post("/api/platform/business/tenants/status")
        .add_header("authorization", op)
        .json(&json!({"input":suspend}))
        .await;
    replay.assert_status_ok();
    assert_eq!(replay.json::<Value>()["authorizationEpoch"], 2);
    assert_eq!(
        store::organization_by_id(&state.db.conn, owner.organization_id())
            .await
            .unwrap()
            .unwrap()
            .authorization_epoch,
        3
    );
    assert_eq!(
        scalar(
            &state.db.conn,
            "SELECT count(*) FROM business_platform_receipt"
        )
        .await,
        4
    );
    let list = server
        .post("/api/platform/business/tenants/list")
        .add_header("authorization", op)
        .json(&json!({"input":{}}))
        .await;
    list.assert_status_ok();
    let text = list.json::<Value>().to_string();
    for private in [old_token.as_str(), new_token, "token_hash"] {
        assert!(!text.contains(private));
    }
    assert_eq!(
        scalar(
            &state.db.conn,
            "SELECT count(*) FROM business_platform_receipt WHERE authority='legacy_operator'"
        )
        .await,
        4
    );
    for sql in [
        "UPDATE business_platform_receipt SET result_json='{}'",
        "DELETE FROM business_platform_receipt",
    ] {
        assert!(state.db.conn.execute_unprepared(sql).await.is_err());
    }
}
