use super::*;

#[tokio::test]
async fn tenancy_native_sessions_bind_window_and_never_refresh_or_fallback_to_operator() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let (_, issued, _) = human(&db.conn, &op, Role::Owner, Domain::ALL.to_vec()).await;
    let sessions = native::NativeSessions::default();
    assert!(sessions
        .open(&db.conn, "tenant-a", "synthetic-original-operator")
        .await
        .is_err());
    assert!(sessions
        .open(&db.conn, "main", &issued.token)
        .await
        .is_err());
    assert!(sessions
        .principal(&db.conn, "tenant-a", None)
        .await
        .is_err());
    assert!(sessions.principal(&db.conn, "unknown", None).await.is_err());
    let first = sessions
        .open(&db.conn, "tenant-a", &issued.token)
        .await
        .unwrap();
    let captured = sessions
        .principal(&db.conn, "tenant-a", Some(&first.session))
        .await
        .unwrap();
    assert!(!captured.is_operator());
    assert!(sessions
        .principal(&db.conn, "tenant-b", Some(&first.session))
        .await
        .is_err());
    assert!(sessions
        .principal(&db.conn, "main", Some(&first.session))
        .await
        .is_err());
    let second = sessions
        .open(&db.conn, "tenant-a", &issued.token)
        .await
        .unwrap();
    assert_ne!(first.session, second.session);
    assert!(authorize(
        &db.conn,
        &captured,
        captured.organization_id(),
        Permission::Read,
        None
    )
    .await
    .is_err());
    sessions.close("tenant-a", &first.session).unwrap();
    let current = sessions
        .principal(&db.conn, "tenant-a", Some(&second.session))
        .await
        .unwrap();
    sessions.clear_window("tenant-a");
    assert!(sessions
        .principal(&db.conn, "tenant-a", Some(&second.session))
        .await
        .is_err());
    assert!(settings::get(&db.conn, &current).await.is_err());
    assert!(sessions
        .principal(&db.conn, "main", None)
        .await
        .unwrap()
        .is_operator());
}

#[cfg(all(feature = "tauri-runtime", feature = "test-utils"))]
mod acl {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn tenancy_actual_native_commands_reject_ambient_and_cross_window_session() {
        let (db, token, org) = tauri::async_runtime::block_on(async {
            let db = fresh_in_memory_db().await;
            let op = initialize(&db.conn).await;
            let (_, issued, _) = human(&db.conn, &op, Role::Owner, Domain::ALL.to_vec()).await;
            (db, issued.token, op.organization_id().to_owned())
        });
        let manifests: BTreeMap<String, Manifest> = serde_json::from_str(include_str!(concat!(
            env!("OUT_DIR"),
            "/acl-manifests.json"
        )))
        .unwrap();
        let capabilities: BTreeMap<String, Capability> =
            serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/capabilities.json")))
                .unwrap();
        let resolved = Resolved::resolve(&manifests, capabilities, Target::current()).unwrap();
        let mut context = mock_context(noop_assets());
        *context.runtime_authority_mut() = tauri::runtime_authority!(manifests, resolved);
        let app = mock_builder()
            .manage(db)
            .manage(native::NativeSessions::default())
            .invoke_handler(tauri::generate_handler![
                crate::commands::business_identity::business_context,
                crate::commands::business_tenancy::business_session_open,
                crate::commands::business_tenancy::business_session_close,
                crate::commands::business_tenancy::business_settings_get
            ])
            .build(context)
            .unwrap();
        let host = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .unwrap();
        let a = tauri::WebviewWindowBuilder::new(&app, "tenant-a", Default::default())
            .build()
            .unwrap();
        let b = tauri::WebviewWindowBuilder::new(&app, "tenant-b", Default::default())
            .build()
            .unwrap();
        let request = |name: &str, body: Value| tauri::webview::InvokeRequest {
            cmd: name.into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: if cfg!(target_os = "macos") {
                "tauri://localhost"
            } else {
                "http://tauri.localhost"
            }
            .parse()
            .unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.into(),
        };
        let original = get_ipc_response(&host, request("business_context", json!({})))
            .unwrap()
            .deserialize::<Value>()
            .unwrap();
        assert_eq!(original["operator"], true);
        assert_eq!(original["organization"]["id"], org);
        assert!(get_ipc_response(&a, request("business_context", json!({}))).is_err());
        let opened = get_ipc_response(
            &a,
            request("business_session_open", json!({"input":{"token":token}})),
        )
        .unwrap()
        .deserialize::<Value>()
        .unwrap();
        let session = opened["session"].as_str().unwrap();
        assert_eq!(opened["context"]["operator"], false);
        assert!(get_ipc_response(
            &b,
            request("business_settings_get", json!({"session":session}))
        )
        .is_err());
        let settings = get_ipc_response(
            &a,
            request("business_settings_get", json!({"session":session})),
        )
        .unwrap()
        .deserialize::<Value>()
        .unwrap();
        assert_eq!(settings["organizationId"], org);
        get_ipc_response(
            &a,
            request(
                "business_session_close",
                json!({"input":{"session":session}}),
            ),
        )
        .unwrap();
        assert!(get_ipc_response(
            &a,
            request("business_settings_get", json!({"session":session}))
        )
        .is_err());
    }
    use tauri::{
        test::{get_ipc_response, mock_builder, mock_context, noop_assets},
        utils::{
            acl::{capability::Capability, manifest::Manifest, resolved::Resolved},
            platform::Target,
        },
    };

    #[test]
    fn tenancy_actual_generated_acl_covers_registry_and_denies_direct_host_invokes() {
        let manifests: BTreeMap<String, Manifest> = serde_json::from_str(include_str!(concat!(
            env!("OUT_DIR"),
            "/acl-manifests.json"
        )))
        .unwrap();
        let capabilities: BTreeMap<String, Capability> =
            serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/capabilities.json")))
                .unwrap();
        let resolved = Resolved::resolve(&manifests, capabilities, Target::current()).unwrap();
        assert!(resolved.has_app_acl);
        let registry = native_acl::command_names(include_str!("../../lib.rs"));
        let app_allowed: BTreeSet<_> = resolved
            .allowed_commands
            .keys()
            .filter(|k| !k.starts_with("plugin:"))
            .map(String::as_str)
            .collect();
        assert_eq!(app_allowed, registry.iter().copied().collect());
        let mut context = mock_context(noop_assets());
        *context.runtime_authority_mut() = tauri::runtime_authority!(manifests, resolved);
        let invoked = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let observed = invoked.clone();
        let app = mock_builder()
            .invoke_handler(move |invoke| {
                observed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                invoke.resolver.resolve("test handler reached");
                true
            })
            .build(context)
            .unwrap();
        let host = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .unwrap();
        let tenant = tauri::WebviewWindowBuilder::new(&app, "tenant-synthetic", Default::default())
            .build()
            .unwrap();
        let request = |name: &str| tauri::webview::InvokeRequest {
            cmd: name.into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: if cfg!(target_os = "macos") {
                "tauri://localhost"
            } else {
                "http://tauri.localhost"
            }
            .parse()
            .unwrap(),
            body: tauri::ipc::InvokeBody::default(),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.into(),
        };
        for command in &registry {
            let response = get_ipc_response(&tenant, request(command));
            if native_acl::TENANT_COMMANDS.contains(command) {
                assert!(response.is_ok(), "allowed tenant command {command}");
            } else {
                assert!(response.is_err(), "host command escaped ACL {command}");
            }
        }
        assert_eq!(
            invoked.load(std::sync::atomic::Ordering::SeqCst),
            native_acl::TENANT_COMMANDS.len()
        );
        // Plugin controls/events are also denied; a crafted label cannot target
        // the host window. Only our injected caller-window controls are allowed.
        for command in [
            "plugin:window|close",
            "plugin:window|create",
            "plugin:event|listen",
            "plugin:opener|open_path",
        ] {
            assert!(get_ipc_response(&tenant, request(command)).is_err());
        }
        assert!(get_ipc_response(&host, request("terminal_spawn")).is_ok());
        assert!(get_ipc_response(&host, request("business_platform_tenants_create")).is_ok());
        assert_eq!(
            invoked.load(std::sync::atomic::Ordering::SeqCst),
            native_acl::TENANT_COMMANDS.len() + 2
        );
        // This verifies actual Tauri pre-dispatch ACL with a handler spy; no
        // shell/provider is launched. It does NOT close the channel fetch
        // exception. Production restricted window creation stays unavailable.
    }
}
