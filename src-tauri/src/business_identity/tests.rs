use super::{store, types::*, *};
use crate::db::test_helpers::fresh_in_memory_db;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::{json, Value};
use std::sync::Arc;
mod fixture;
mod policy;
mod transactions;

async fn initialize(conn: &DatabaseConnection) -> Principal {
    store::bootstrap(
        conn,
        BootstrapInput {
            organization_name: "Synthetic team".into(),
            owner_name: "Named operator".into(),
        },
    )
    .await
    .unwrap();
    operator_principal(conn).await.unwrap()
}
async fn human(
    conn: &DatabaseConnection,
    op: &Principal,
    role: Role,
    domains: Vec<Domain>,
) -> (Member, IssuedCredential, Principal) {
    let member = store::create_member(
        conn,
        op,
        CreateMemberInput {
            organization_id: op.organization_id().into(),
            display_name: format!("Synthetic {role:?}"),
            kind: MemberKind::Human,
            role,
            domains,
        },
    )
    .await
    .unwrap();
    let credential = store::issue_credential(
        conn,
        op,
        IssueCredentialInput {
            organization_id: op.organization_id().into(),
            member_id: member.id.clone(),
            label: "Synthetic browser".into(),
        },
    )
    .await
    .unwrap();
    let principal = store::resolve_credential(conn, &credential.token)
        .await
        .unwrap();
    (member, credential, principal)
}

#[tokio::test]
async fn full_router_requires_original_operator_bootstrap_and_keeps_member_bearers_out_of_legacy() {
    let db = fresh_in_memory_db().await;
    let dir = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let router = crate::web::router::build_router(
        state.clone(),
        "business-synthetic-operator".into(),
        dir.path().into(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    );
    let server = axum_test::TestServer::new(router).unwrap();
    let op = "Bearer business-synthetic-operator";
    for path in [
        "/api/business/context",
        "/api/business/bootstrap",
        "/api/business/members/list",
    ] {
        server
            .post(path)
            .json(&json!({"input": {}}))
            .await
            .assert_status_unauthorized();
    }
    let initial = server
        .post("/api/business/context")
        .add_header("authorization", op)
        .json(&json!({"input": {}}))
        .await;
    initial.assert_status_ok();
    assert_eq!(initial.json::<Value>()["needsBootstrap"], true);
    let initialized = server
        .post("/api/business/bootstrap")
        .add_header("authorization", op)
        .json(&json!({"input":{"organizationName":"Original team", "ownerName":"Original owner"}}))
        .await;
    initialized.assert_status_ok();
    let org = initialized.json::<Value>()["organization"]["id"]
        .as_str()
        .unwrap()
        .to_owned();
    let original = operator_principal(&state.db.conn).await.unwrap();
    for role in [Role::Member, Role::Viewer, Role::Owner] {
        let (_, issued, _) = human(&state.db.conn, &original, role, vec![Domain::Feedback]).await;
        let bearer = format!("Bearer {}", issued.token);
        let context = server
            .post("/api/business/context")
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":{}}))
            .await;
        context.assert_status_ok();
        assert_eq!(
            context.json::<Value>()["capabilities"]["legacyOperator"],
            false
        );
        assert_eq!(context.json::<Value>()["operator"], false);
        assert_eq!(context.header("cache-control"), "no-store");
        server
            .post("/api/business/bootstrap")
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":{"organizationName":"Takeover", "ownerName":"Attacker"}}))
            .await
            .assert_status_forbidden();
        // All requests stop at legacy auth, before invoking a provider/engine.
        for path in [
            "/api/health",
            "/api/acp_connect",
            "/api/acp_prompt",
            "/api/acp_load_pi_config",
            "/api/ops_context",
            "/api/terminal_spawn",
            "/api/terminal_list",
        ] {
            server
                .post(path)
                .add_header("authorization", bearer.as_str())
                .json(&json!({}))
                .await
                .assert_status_unauthorized();
        }
        server
            .get("/ws/events")
            .add_header("authorization", bearer.as_str())
            .await
            .assert_status_unauthorized();
        let protocol = format!(
            "codeg-events, codeg-token.{}",
            URL_SAFE_NO_PAD.encode(issued.token.as_bytes())
        );
        server
            .get("/ws/events")
            .add_header("sec-websocket-protocol", protocol.as_str())
            .await
            .assert_status_unauthorized();
        server
            .post("/api/health")
            .add_header("sec-websocket-protocol", protocol.as_str())
            .json(&json!({}))
            .await
            .assert_status_unauthorized();
    }
    server
        .post("/api/health")
        .add_header("authorization", op)
        .json(&json!({}))
        .await
        .assert_status_ok();
    let repeated = server.post("/api/business/bootstrap").add_header("authorization", op)
        .json(&json!({"input":{"organizationName":"Must not overwrite", "ownerName":"Must not replace"}})).await;
    repeated.assert_status_ok();
    assert_eq!(repeated.json::<Value>()["organization"]["id"], org);
    assert_eq!(
        repeated.json::<Value>()["organization"]["name"],
        "Original team"
    );
    assert_eq!(
        repeated.json::<Value>()["member"]["displayName"],
        "Original owner"
    );
}

#[tokio::test]
async fn wire_rejects_actor_role_org_spoofing_and_viewer_writes() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let (viewer, issued, _) = human(&db.conn, &op, Role::Viewer, vec![Domain::Feedback]).await;
    let dir = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let router = crate::web::router::build_router(
        state,
        "business-synthetic-operator".into(),
        dir.path().into(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    );
    let server = axum_test::TestServer::new(router).unwrap();
    let bearer = format!("Bearer {}", issued.token);
    for input in [
        json!({"actor": op.member_id()}),
        json!({"role":"owner"}),
        json!({"organizationId":op.organization_id()}),
        json!({"principal":{"id":op.member_id()}}),
    ] {
        server
            .post("/api/business/context")
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":input}))
            .await
            .assert_status_unprocessable_entity();
    }
    server
        .post("/api/business/members/list")
        .add_header("authorization", bearer.as_str())
        .json(&json!({"input":{"organizationId":uuid::Uuid::new_v4().to_string()}}))
        .await
        .assert_status_not_found();
    server
        .post("/api/business/members/list")
        .add_header("authorization", bearer.as_str())
        .json(&json!({"input":{"organizationId":op.organization_id(), "domain":"engineering"}}))
        .await
        .assert_status_forbidden();
    for (path, input) in [
        (
            "members/create",
            json!({"organizationId":op.organization_id(),"displayName":"Spoof","kind":"human","role":"owner","domains":["feedback"]}),
        ),
        (
            "members/update",
            json!({"organizationId":op.organization_id(),"memberId":viewer.id,"expectedRevision":1,"displayName":"Spoof","role":"owner","domains":["feedback"]}),
        ),
        (
            "members/revoke",
            json!({"organizationId":op.organization_id(),"memberId":op.member_id(),"expectedRevision":1}),
        ),
        (
            "credentials/issue",
            json!({"organizationId":op.organization_id(),"memberId":viewer.id,"label":"Spoof"}),
        ),
    ] {
        server
            .post(&format!("/api/business/{path}"))
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":input}))
            .await
            .assert_status_forbidden();
    }
}

#[tokio::test]
async fn member_tokens_are_hashed_individual_and_revocation_rechecks_cached_principal() {
    let db = fresh_in_memory_db().await;
    let ctx = store::bootstrap(
        &db.conn,
        BootstrapInput {
            organization_name: "Synthetic organization".into(),
            owner_name: "Owner".into(),
        },
    )
    .await
    .unwrap();
    let org = ctx.organization.unwrap().id;
    let operator = operator_principal(&db.conn).await.unwrap();
    let member = store::create_member(
        &db.conn,
        &operator,
        CreateMemberInput {
            organization_id: org.clone(),
            display_name: "Reader".into(),
            kind: MemberKind::Human,
            role: Role::Member,
            domains: vec![Domain::Feedback],
        },
    )
    .await
    .unwrap();
    let issued = store::issue_credential(
        &db.conn,
        &operator,
        IssueCredentialInput {
            organization_id: org.clone(),
            member_id: member.id.clone(),
            label: "Synthetic session".into(),
        },
    )
    .await
    .unwrap();
    let principal = store::resolve_credential(&db.conn, &issued.token)
        .await
        .unwrap();
    assert_eq!(principal.member_id(), member.id);
    assert!(!principal.is_operator());
    let stored = db
        .conn
        .query_one(store::statement(
            "SELECT token_hash FROM business_credential WHERE id = ?",
            vec![issued.credential.id.clone().into()],
        ))
        .await
        .unwrap()
        .unwrap();
    let hash: String = stored.try_get("", "token_hash").unwrap();
    assert_eq!(hash.len(), 64);
    assert!(!hash.contains(&issued.token));
    let listed = store::list_credentials(
        &db.conn,
        &operator,
        CredentialsInput {
            organization_id: org.clone(),
            member_id: member.id.clone(),
        },
    )
    .await
    .unwrap();
    let public = serde_json::to_string(&listed).unwrap();
    assert!(!public.contains(&issued.token));
    assert!(!public.contains(&hash));
    assert!(authorize(
        &db.conn,
        &principal,
        &org,
        Permission::Contribute,
        Some(Domain::Feedback)
    )
    .await
    .is_ok());
    assert!(matches!(
        authorize(
            &db.conn,
            &principal,
            &org,
            Permission::Read,
            Some(Domain::Engineering)
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
    store::revoke_credential(
        &db.conn,
        &operator,
        RevokeCredentialInput {
            organization_id: org.clone(),
            credential_id: issued.credential.id,
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        store::resolve_credential(&db.conn, &issued.token).await,
        Err(IdentityError::Unauthorized)
    ));
    let tx = begin_write(&db.conn, &org).await.unwrap();
    assert!(matches!(
        authorize(
            &tx,
            &principal,
            &org,
            Permission::Contribute,
            Some(Domain::Feedback)
        )
        .await,
        Err(IdentityError::Unauthorized)
    ));
}
