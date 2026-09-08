use super::*;
use crate::business_identity::platform::*;
use crate::db::test_helpers::fresh_disk_db;
use sea_orm::Database;

fn platform() -> PlatformContext {
    PlatformContext::from_operator(&crate::web::auth::AuthenticatedOperator)
}
fn create_input(name: &str) -> CreateTenantInput {
    CreateTenantInput {
        operation_id: uuid::Uuid::new_v4().to_string(),
        organization_name: name.into(),
        owner_name: format!("{name} owner"),
    }
}
async fn count(conn: &DatabaseConnection, table: &str) -> i64 {
    conn.query_one(store::statement(
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
async fn tenancy_provision_replay_loss_reissue_preserves_one_tenant_and_unrelated_credentials() {
    let db = fresh_in_memory_db().await;
    let p = platform();
    let input = create_input("Tenant A");
    let original_input = serde_json::to_value(&input).unwrap();
    let created = create(&db.conn, &p, input).await.unwrap();
    let old_token = created.token.as_ref().unwrap();
    let principal = store::resolve_credential(&db.conn, old_token)
        .await
        .unwrap();
    assert!(!principal.is_operator());
    // Provisioning before original bootstrap never implicitly becomes original.
    assert!(matches!(
        operator_principal(&db.conn).await,
        Err(IdentityError::BootstrapRequired)
    ));
    let replayed = create(
        &db.conn,
        &p,
        serde_json::from_value(original_input.clone()).unwrap(),
    )
    .await
    .unwrap();
    assert!(replayed.token.is_none());
    assert_eq!(
        replayed.metadata.organization.id,
        created.metadata.organization.id
    );
    let mut changed = original_input;
    changed["ownerName"] = json!("Different");
    assert!(matches!(
        create(&db.conn, &p, serde_json::from_value(changed).unwrap()).await,
        Err(IdentityError::Conflict)
    ));
    assert_eq!(count(&db.conn, "business_organization").await, 1);
    assert_eq!(count(&db.conn, "business_credential").await, 1);
    let unrelated = store::issue_credential(
        &db.conn,
        &principal,
        IssueCredentialInput {
            organization_id: principal.organization_id().into(),
            member_id: principal.member_id().into(),
            label: "Other device".into(),
        },
    )
    .await
    .unwrap();
    let recover = ReissueInput {
        operation_id: uuid::Uuid::new_v4().to_string(),
        organization_id: principal.organization_id().into(),
        expected_revision: 1,
        expected_authorization_epoch: 1,
        owner_member_id: principal.member_id().into(),
        expected_owner_revision: 1,
        expected_credential_id: created.metadata.credential.id.clone(),
    };
    let recovery_input = serde_json::to_value(&recover).unwrap();
    let reissued = reissue(&db.conn, &p, recover).await.unwrap();
    assert!(store::resolve_credential(&db.conn, old_token)
        .await
        .is_err());
    assert!(store::resolve_credential(&db.conn, &unrelated.token)
        .await
        .is_ok());
    assert!(
        store::resolve_credential(&db.conn, reissued.token.as_ref().unwrap())
            .await
            .is_ok()
    );
    let replayed = reissue(
        &db.conn,
        &p,
        serde_json::from_value(recovery_input.clone()).unwrap(),
    )
    .await
    .unwrap();
    assert!(replayed.token.is_none());
    let mut stale = recovery_input;
    stale["operationId"] = json!(uuid::Uuid::new_v4().to_string());
    assert!(matches!(
        reissue(&db.conn, &p, serde_json::from_value(stale).unwrap()).await,
        Err(IdentityError::Conflict)
    ));
    let registry = list(&db.conn, &p).await.unwrap();
    assert_eq!(
        registry[0].credential.as_ref().unwrap().id,
        reissued.metadata.credential.id
    );
    assert_eq!(count(&db.conn, "business_credential").await, 3);
    let audit = db
        .conn
        .query_all(store::statement(
            "SELECT result_json FROM business_platform_receipt",
            vec![],
        ))
        .await
        .unwrap();
    for row in audit {
        let json: String = row.try_get("", "result_json").unwrap();
        assert!(!json.contains(old_token));
        assert!(!json.contains(reissued.token.as_ref().unwrap()));
        assert!(!json.contains("token_hash"));
    }
    let original = initialize(&db.conn).await;
    assert_ne!(original.organization_id(), principal.organization_id());
    assert_eq!(
        store::context(
            &db.conn,
            &store::resolve_credential(&db.conn, reissued.token.as_ref().unwrap())
                .await
                .unwrap()
        )
        .await
        .unwrap()
        .organization
        .unwrap()
        .id,
        principal.organization_id()
    );
}

#[tokio::test]
async fn tenancy_last_owner_and_provision_receipt_failure_are_atomic() {
    let db = fresh_in_memory_db().await;
    let p = platform();
    db.conn.execute_unprepared("CREATE TRIGGER fixture_platform_audit_failure BEFORE INSERT ON business_platform_receipt BEGIN SELECT RAISE(ABORT,'synthetic failure'); END").await.unwrap();
    assert!(create(&db.conn, &p, create_input("No orphan"))
        .await
        .is_err());
    for table in [
        "business_organization",
        "business_member",
        "business_credential",
        "business_tenant_settings",
    ] {
        assert_eq!(count(&db.conn, table).await, 0);
    }
    db.conn
        .execute_unprepared("DROP TRIGGER fixture_platform_audit_failure")
        .await
        .unwrap();
    let created = create(&db.conn, &p, create_input("Tenant")).await.unwrap();
    let owner = store::resolve_credential(&db.conn, created.token.as_ref().unwrap())
        .await
        .unwrap();
    let demote = || UpdateMemberInput {
        organization_id: owner.organization_id().into(),
        member_id: owner.member_id().into(),
        expected_revision: 1,
        display_name: "Owner".into(),
        role: Role::Admin,
        domains: Domain::ALL.to_vec(),
    };
    assert!(matches!(
        store::update_member(&db.conn, &owner, demote()).await,
        Err(IdentityError::Forbidden)
    ));
    assert!(matches!(
        store::revoke_member(
            &db.conn,
            &owner,
            RevokeMemberInput {
                organization_id: owner.organization_id().into(),
                member_id: owner.member_id().into(),
                expected_revision: 1
            }
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
    assert_eq!(
        store::member(&db.conn, owner.organization_id(), owner.member_id())
            .await
            .unwrap()
            .unwrap()
            .revision,
        1
    );
    let (other, _, _) = human(&db.conn, &owner, Role::Owner, Domain::ALL.to_vec()).await;
    store::update_member(&db.conn, &owner, demote())
        .await
        .unwrap();
    assert_eq!(
        store::member(&db.conn, owner.organization_id(), &other.id)
            .await
            .unwrap()
            .unwrap()
            .role,
        Role::Owner
    );
    // Recovery cannot silently promote a demoted provisioning owner.
    assert!(matches!(
        reissue(
            &db.conn,
            &p,
            ReissueInput {
                operation_id: uuid::Uuid::new_v4().to_string(),
                organization_id: owner.organization_id().into(),
                expected_revision: 1,
                expected_authorization_epoch: 1,
                owner_member_id: owner.member_id().into(),
                expected_owner_revision: 2,
                expected_credential_id: created.metadata.credential.id
            }
        )
        .await,
        Err(IdentityError::Forbidden)
    ));
}

#[tokio::test]
async fn tenancy_two_real_tenants_protected_router_positive_and_negative_resource_controls() {
    let db = fresh_in_memory_db().await;
    let original = initialize(&db.conn).await;
    let dir = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let server = axum_test::TestServer::new(crate::web::router::build_router(
        state.clone(),
        "tenancy-synthetic-operator".into(),
        dir.path().into(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    ))
    .unwrap();
    let op = "Bearer tenancy-synthetic-operator";
    let mut tenants = Vec::new();
    for name in ["A", "B"] {
        let response = server
            .post("/api/platform/business/tenants/create")
            .add_header("authorization", op)
            .json(&json!({"input":create_input(name)}))
            .await;
        response.assert_status_ok();
        assert_eq!(response.header("cache-control"), "no-store");
        let v = response.json::<Value>();
        let bearer = format!("Bearer {}", v["token"].as_str().unwrap());
        let context = server
            .post("/api/business/context")
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":{}}))
            .await;
        context.assert_status_ok();
        assert_eq!(context.json::<Value>()["organization"]["name"], name);
        let task=server.post("/api/business/tasks/create").add_header("authorization",bearer.as_str()).json(&json!({"input":{"title":format!("{name} private task"),"domain":"feedback","notes":"Private briefing"}})).await;
        task.assert_status_ok();
        tenants.push((
            bearer,
            v,
            task.json::<Value>()["task"]["id"]
                .as_str()
                .unwrap()
                .to_owned(),
        ));
    }
    assert_ne!(
        tenants[0].1["organization"]["id"],
        tenants[1].1["organization"]["id"]
    );
    for (index, (bearer, own, own_task)) in tenants.iter().enumerate() {
        let foreign = &tenants[1 - index];
        for path in [
            "context",
            "tenants/list",
            "tenants/create",
            "tenants/status",
            "tenants/reissue-owner-credential",
        ] {
            server
                .post(&format!("/api/platform/business/{path}"))
                .add_header("authorization", bearer.as_str())
                .json(&json!({"input":{}}))
                .await
                .assert_status_unauthorized();
        }
        for path in [
            "/api/terminal_spawn",
            "/api/acp_connect",
            "/api/acp_load_pi_config",
        ] {
            server
                .post(path)
                .add_header("authorization", bearer.as_str())
                .json(&json!({}))
                .await
                .assert_status_unauthorized();
        }
        server
            .post("/api/business/tasks/get")
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":{"taskId":foreign.2}}))
            .await
            .assert_status_not_found();
        server
            .post("/api/business/members/list")
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":{"organizationId":foreign.1["organization"]["id"]}}))
            .await
            .assert_status_not_found();
        server.post("/api/business/tasks/create").add_header("authorization",bearer.as_str()).json(&json!({"input":{"title":"Spoof owner","domain":"feedback","ownerId":foreign.1["ownerMemberId"]}})).await.assert_status_not_found();
        let items = server
            .post("/api/business/tasks/list")
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":{}}))
            .await;
        items.assert_status_ok();
        let text = items.json::<Value>().to_string();
        assert!(text.contains(own_task));
        assert!(!text.contains(&foreign.2));
        let settings = server
            .post("/api/business/settings/get")
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":{}}))
            .await;
        settings.assert_status_ok();
        assert_eq!(
            settings.json::<Value>()["organizationId"],
            own["organization"]["id"]
        );
        server.post("/api/business/settings/update").add_header("authorization",bearer.as_str()).json(&json!({"input":{"expectedRevision":1,"organizationId":foreign.1["organization"]["id"],"settings":settings.json::<Value>()["settings"]}})).await.assert_status_unprocessable_entity();
    }
    assert_eq!(
        operator_principal(&state.db.conn)
            .await
            .unwrap()
            .organization_id(),
        original.organization_id()
    );
    server
        .post("/api/platform/business/context")
        .add_header("authorization", op)
        .json(&json!({"input":{}}))
        .await
        .assert_status_ok();
}

#[tokio::test]
async fn tenancy_two_writers_settings_cas_and_suspend_fence_captured_authority() {
    let dir = tempfile::tempdir().unwrap();
    let db = fresh_disk_db(dir.path()).await;
    let other = Database::connect(format!(
        "sqlite:{}?mode=rw",
        dir.path().join("source.db").display()
    ))
    .await
    .unwrap();
    let p = platform();
    let created = create(&db.conn, &p, create_input("Concurrent"))
        .await
        .unwrap();
    let owner = store::resolve_credential(&db.conn, created.token.as_ref().unwrap())
        .await
        .unwrap();
    let input = |name: &str| settings::UpdateSettingsInput {
        expected_revision: 1,
        settings: settings::Settings::defaults(name.into()),
    };
    let (a, b) = tokio::join!(
        settings::update(&db.conn, &owner, input("A")),
        settings::update(&other, &owner, input("B"))
    );
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    assert!(matches!(a, Err(IdentityError::Conflict)) || matches!(b, Err(IdentityError::Conflict)));
    let tx = begin_write(&db.conn, owner.organization_id())
        .await
        .unwrap();
    tx.execute(store::statement("UPDATE business_organization SET status='suspended',revision=revision+1,authorization_epoch=authorization_epoch+1 WHERE id=?",vec![owner.organization_id().into()])).await.unwrap();
    let captured = owner.clone();
    let start = Arc::new(tokio::sync::Notify::new());
    let ready = start.clone();
    let mut waiting = tokio::spawn(async move {
        ready.notify_one();
        settings::update(
            &other,
            &captured,
            settings::UpdateSettingsInput {
                expected_revision: 2,
                settings: settings::Settings::defaults("Must not write".into()),
            },
        )
        .await
    });
    start.notified().await;
    assert!(
        tokio::time::timeout(std::time::Duration::from_millis(50), &mut waiting)
            .await
            .is_err()
    );
    tx.commit().await.unwrap();
    assert!(matches!(
        waiting.await.unwrap(),
        Err(IdentityError::Unauthorized)
    ));
    status(
        &db.conn,
        &p,
        StatusInput {
            operation_id: uuid::Uuid::new_v4().to_string(),
            organization_id: owner.organization_id().into(),
            expected_revision: 2,
            expected_authorization_epoch: 2,
            status: OrganizationStatus::Active,
        },
    )
    .await
    .unwrap();
    assert!(settings::get(&db.conn, &owner).await.is_err());
    let fresh = store::resolve_credential(&db.conn, created.token.as_ref().unwrap())
        .await
        .unwrap();
    assert_eq!(settings::get(&db.conn, &fresh).await.unwrap().revision, 2);
}

#[tokio::test]
async fn tenancy_two_writers_provision_one_secret_and_cannot_demote_both_owners() {
    let dir = tempfile::tempdir().unwrap();
    let db = fresh_disk_db(dir.path()).await;
    let other = Database::connect(format!(
        "sqlite:{}?mode=rw",
        dir.path().join("source.db").display()
    ))
    .await
    .unwrap();
    let input = serde_json::to_value(create_input("Idempotent tenant")).unwrap();
    let p = platform();
    let (a, b) = tokio::join!(
        create(&db.conn, &p, serde_json::from_value(input.clone()).unwrap()),
        create(&other, &p, serde_json::from_value(input).unwrap())
    );
    let a = a.unwrap();
    let b = b.unwrap();
    assert_eq!(a.metadata.organization.id, b.metadata.organization.id);
    assert_eq!(
        usize::from(a.token.is_some()) + usize::from(b.token.is_some()),
        1
    );
    assert_eq!(count(&db.conn, "business_organization").await, 1);
    assert_eq!(count(&db.conn, "business_platform_receipt").await, 1);
    let owner_a =
        store::resolve_credential(&db.conn, a.token.as_ref().or(b.token.as_ref()).unwrap())
            .await
            .unwrap();
    let (_, _, owner_b) = human(&db.conn, &owner_a, Role::Owner, Domain::ALL.to_vec()).await;
    let demote = |p: &Principal| UpdateMemberInput {
        organization_id: p.organization_id().into(),
        member_id: p.member_id().into(),
        expected_revision: 1,
        display_name: "Demoted owner".into(),
        role: Role::Admin,
        domains: Domain::ALL.to_vec(),
    };
    let (a, b) = tokio::join!(
        store::update_member(&db.conn, &owner_a, demote(&owner_a)),
        store::update_member(&other, &owner_b, demote(&owner_b))
    );
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    assert!(
        matches!(a, Err(IdentityError::Forbidden)) || matches!(b, Err(IdentityError::Forbidden))
    );
    let row=db.conn.query_one(store::statement("SELECT count(*) AS n FROM business_member WHERE organization_id=? AND kind='human' AND role='owner' AND status='active'",vec![owner_a.organization_id().into()])).await.unwrap().unwrap();
    assert_eq!(row.try_get::<i64>("", "n").unwrap(), 1);
}
