use super::{super::*, support::*};
use crate::business_identity::{self as identity, store, types::*};
use sea_orm::{ConnectionTrait, TransactionTrait};
use serde_json::{json, Value};
use std::sync::{atomic::Ordering, Arc};

#[tokio::test]
async fn intake_setup_zero_grants_actual_operator_and_exact_replay() {
    let f = Fixture::new().await;
    let operation = common::id();
    let (_, _, owner_credential) =
        human(&f.db.conn, &f.op, Role::Owner, vec![Domain::Feedback]).await;
    assert!(bindings_create(
        &f.db.conn,
        &owner_credential,
        &f.services,
        None,
        f.create_input(&operation)
    )
    .await
    .is_err());
    assert_eq!(f.mock.count(), 0);
    assert!(f.secrets.values.lock().unwrap().is_empty());
    f.mock
        .json(json!({"data":{"user":{"user_id":"source-user"}}}));
    let b = bindings_create(
        &f.db.conn,
        &f.op,
        &f.services,
        None,
        f.create_input(&operation),
    )
    .await
    .unwrap();
    assert!(!b.enabled);
    assert_eq!(b.revision, 1);
    assert_eq!(count(&f.db.conn, "business_intake_grant").await, 0);
    let again = bindings_create(
        &f.db.conn,
        &f.op,
        &f.services,
        None,
        f.create_input(&operation),
    )
    .await
    .unwrap();
    assert_eq!(again.id, b.id);
    assert_eq!(f.mock.count(), 1);
    let mut changed = f.create_json(&operation);
    changed["source"]["apiKey"] = json!("synthetic-different");
    assert!(bindings_create(
        &f.db.conn,
        &f.op,
        &f.services,
        None,
        serde_json::from_value(changed).unwrap()
    )
    .await
    .is_err());
    let listed = bindings_list(&f.db.conn, &f.op, &f.services, PageInput { page: 0 })
        .await
        .unwrap();
    assert!(listed.can_manage_setup);
    assert!(!listed.items[0].binding.capabilities.read);
    let enabled = bindings_update(&f.db.conn, &f.op, &f.services, enable(&b))
        .await
        .unwrap();
    let tx = f.db.conn.begin().await.unwrap();
    assert!(
        access::check_binding(&tx, &f.op, &f.services, &enabled.id, access::Use::Read)
            .await
            .is_err()
    );
    tx.rollback().await.unwrap();
    assert_eq!(count(&f.db.conn, "business_intake_binding").await, 1);
}

#[tokio::test]
async fn intake_staged_failure_and_wrong_provider_preserve_active_and_unrelated_keys() {
    let f = Fixture::new().await;
    f.secrets
        .values
        .lock()
        .unwrap()
        .insert("unrelated".into(), "synthetic-preserved".into());
    f.secrets.fail_set.store(true, Ordering::SeqCst);
    assert!(bindings_create(
        &f.db.conn,
        &f.op,
        &f.services,
        None,
        f.create_input(&common::id())
    )
    .await
    .is_err());
    assert_eq!(count(&f.db.conn, "business_intake_binding").await, 0);
    assert_eq!(f.mock.count(), 0);
    f.secrets.fail_set.store(false, Ordering::SeqCst);
    let b = f.binding().await;
    let row =
        f.db.conn
            .query_one(common::sql(
                "SELECT credential_ref FROM business_intake_binding WHERE id=?",
                vec![b.id.clone().into()],
            ))
            .await
            .unwrap()
            .unwrap();
    let reference: String = row.try_get("", "credential_ref").unwrap();
    let mut update = enable(&b);
    update.credential = Some(CredentialReplacement::Fireflies {
        api_key: Secret("synthetic-wrong-principal".into()),
    });
    f.mock
        .json(json!({"data":{"user":{"user_id":"other-provider-user"}}}));
    assert!(bindings_update(&f.db.conn, &f.op, &f.services, update)
        .await
        .is_err());
    let current = bindings_status(
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
    assert_eq!(current.revision, b.revision);
    assert_eq!(
        f.secrets.values.lock().unwrap()[&reference],
        "synthetic-fireflies"
    );
    assert_eq!(
        f.secrets.values.lock().unwrap()["unrelated"],
        "synthetic-preserved"
    );
    let mut rotation = enable(&b);
    rotation.credential = Some(CredentialReplacement::Fireflies {
        api_key: Secret("synthetic-rotated".into()),
    });
    f.mock
        .json(json!({"data":{"user":{"user_id":"source-user"}}}));
    let rotated = bindings_update(&f.db.conn, &f.op, &f.services, rotation)
        .await
        .unwrap();
    assert_eq!(rotated.revision, b.revision + 1);
    // Next protected setup performs bounded orphan/retired cleanup, never active deletion.
    bindings_update(&f.db.conn, &f.op, &f.services, enable(&rotated))
        .await
        .unwrap();
    assert!(!f.secrets.values.lock().unwrap().contains_key(&reference));
    assert_eq!(
        f.secrets.values.lock().unwrap()["unrelated"],
        "synthetic-preserved"
    );
    assert!(f
        .secrets
        .values
        .lock()
        .unwrap()
        .values()
        .any(|v| v == "synthetic-rotated"));
}

#[tokio::test]
async fn intake_setup_await_rechecks_owner_revision_and_binding_disable() {
    let f = Fixture::new().await;
    let b = f.binding().await;
    let gate = Arc::new(tokio::sync::Barrier::new(2));
    let mut reply = Reply::json(json!({"data":{"user":{"user_id":"source-user"}}}));
    reply.gate = Some(gate.clone());
    f.mock.reply(reply);
    let mut input = enable(&b);
    input.credential = Some(CredentialReplacement::Fireflies {
        api_key: Secret("synthetic-racing".into()),
    });
    let future = bindings_update(&f.db.conn, &f.op, &f.services, input);
    let race = async {
        f.mock.seen(2).await;
        let disabled = bindings_disable(
            &f.db.conn,
            &f.op,
            &f.services,
            DisableBindingInput {
                operation_id: common::id(),
                binding_id: b.id.clone(),
                expected_revision: b.revision,
            },
        )
        .await
        .unwrap();
        gate.wait().await;
        disabled
    };
    let (result, disabled) = tokio::join!(future, race);
    assert!(result.is_err());
    assert!(!disabled.enabled);
    let before = count(&f.db.conn, "business_intake_binding").await;
    let gate = Arc::new(tokio::sync::Barrier::new(2));
    let mut reply = Reply::json(json!({"data":{"user":{"user_id":"source-user"}}}));
    reply.gate = Some(gate.clone());
    f.mock.reply(reply);
    let future = bindings_create(
        &f.db.conn,
        &f.op,
        &f.services,
        None,
        f.create_input(&common::id()),
    );
    let race = async {
        f.mock.seen(3).await;
        f.db.conn
            .execute(common::sql(
                "UPDATE business_member SET revision=revision+1 WHERE id=?",
                vec![f.op.member_id().into()],
            ))
            .await
            .unwrap();
        gate.wait().await;
    };
    let (result, _) = tokio::join!(future, race);
    assert!(result.is_err());
    assert_eq!(count(&f.db.conn, "business_intake_binding").await, before);
}

#[tokio::test]
async fn intake_grants_ceiling_current_membership_and_owner_drift_are_rechecked() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let (m, issued, p) = human(&f.db.conn, &f.op, Role::Member, vec![Domain::Feedback]).await;
    let g = grants_upsert(&f.db.conn, &f.op, &f.services, grant(&b, &m.id, None))
        .await
        .unwrap();
    let member = bindings_status(
        &f.db.conn,
        &p,
        &f.services,
        BindingInput {
            binding_id: b.id.clone(),
        },
    )
    .await
    .unwrap();
    assert!(member.admin.is_none());
    assert!(member.binding.capabilities.triage);
    let payload = serde_json::to_value(member).unwrap();
    for field in [
        "resource",
        "sourceOwnerId",
        "credentialRef",
        "providerUserId",
    ] {
        assert!(!payload.to_string().contains(field));
    }
    let (viewer, _, _) = human(&f.db.conn, &f.op, Role::Viewer, vec![Domain::Feedback]).await;
    assert!(grants_upsert(
        &f.db.conn,
        &f.op,
        &f.services,
        grant(&g.binding, &viewer.id, None)
    )
    .await
    .is_err());
    let agent = store::create_member(
        &f.db.conn,
        &f.op,
        CreateMemberInput {
            organization_id: f.op.organization_id().into(),
            display_name: "Scoped agent".into(),
            kind: MemberKind::Agent,
            role: Role::Member,
            domains: vec![Domain::Feedback],
        },
    )
    .await
    .unwrap();
    assert!(grants_upsert(
        &f.db.conn,
        &f.op,
        &f.services,
        grant(&g.binding, &agent.id, None)
    )
    .await
    .is_err());
    let agent_p = identity::agent_principal(&f.db.conn, &f.op, &agent.id)
        .await
        .unwrap();
    assert!(
        bindings_list(&f.db.conn, &agent_p, &f.services, PageInput { page: 0 })
            .await
            .is_err()
    );
    f.db.conn
        .execute(common::sql(
            "UPDATE business_member SET revision=revision+1 WHERE id=?",
            vec![f.op.member_id().into()],
        ))
        .await
        .unwrap();
    let paused = bindings_status(
        &f.db.conn,
        &p,
        &f.services,
        BindingInput {
            binding_id: b.id.clone(),
        },
    )
    .await
    .unwrap();
    assert!(!paused.binding.capabilities.read);
    let repaired = bindings_update(&f.db.conn, &f.op, &f.services, enable(&g.binding))
        .await
        .unwrap();
    assert!(repaired.access_epoch > g.binding.access_epoch);
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
    assert!(bindings_status(
        &f.db.conn,
        &p,
        &f.services,
        BindingInput { binding_id: b.id }
    )
    .await
    .is_err());
}

#[tokio::test]
async fn intake_protected_setup_router_rejects_member_owner_spoofs_and_redacts_keys() {
    let f = Fixture::new().await;
    let (_, issued, _) = human(&f.db.conn, &f.op, Role::Owner, vec![Domain::Feedback]).await;
    let input = f.create_json(&common::id());
    let dir = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        f.db,
        dir.path().into(),
    ));
    let router = crate::web::router::build_router(
        state,
        "synthetic-intake-operator".into(),
        dir.path().into(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    )
    .layer(axum::Extension(f.services));
    let server = axum_test::TestServer::new(router).unwrap();
    let path = "/api/business/intake/bindings/create";
    server
        .post(path)
        .json(&json!({"input":input}))
        .await
        .assert_status_unauthorized();
    server
        .post(path)
        .add_header("authorization", format!("Bearer {}", issued.token))
        .json(&json!({"input":input}))
        .await
        .assert_status_forbidden();
    let mut spoof = input.clone();
    spoof["actorId"] = json!("caller");
    server
        .post(path)
        .add_header("authorization", "Bearer synthetic-intake-operator")
        .json(&json!({"input":spoof}))
        .await
        .assert_status_bad_request();
    f.mock
        .json(json!({"data":{"user":{"user_id":"source-user"}}}));
    let response = server
        .post(path)
        .add_header("authorization", "Bearer synthetic-intake-operator")
        .json(&json!({"input":input}))
        .await;
    response.assert_status_ok();
    assert_eq!(response.header("cache-control"), "no-store");
    let body = response.json::<Value>().to_string();
    assert!(!body.contains("synthetic-fireflies"));
    assert!(!body.contains("credentialRef"));
    assert_eq!(f.mock.count(), 1);
}
