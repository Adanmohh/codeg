use super::*;
use crate::{
    db::{self, test_helpers::fresh_in_memory_db, AppDatabase},
    ops::{
        store,
        tests::{operator, pending, saved, seed},
        types::{DenyInput, SaveDraftInput},
    },
};
use axum::http::StatusCode;
use sea_orm_migration::{MigratorTrait, SchemaManager};
use serde_json::json;
mod migration;
mod issues;
mod issues_browser;
pub(crate) mod provider;
use provider::Provider;

async fn settings(
    db: &AppDatabase,
    op: &Operator,
    runtime: &TelegramRuntime,
    enabled: bool,
) -> Status {
    let ch = crate::db::service::chat_channel_service::create(
        &db.conn,
        "Private operator".into(),
        "telegram".into(),
        json!({"chat_id":"123","topic_mode":false}).to_string(),
        false,
        false,
        None,
    )
    .await
    .unwrap();
    configure(
        &db.conn,
        op,
        runtime,
        ConfigureInput {
            channel_id: ch.id,
            private_user_id: "123".into(),
            review_origin: "http://127.0.0.1:4323".into(),
            enabled,
            github_issues_enabled: false,
            expected_revision: None,
        },
    )
    .await
    .unwrap()
}
async fn first_notice(db: &AppDatabase) -> notice::Model {
    notice::Entity::find().one(&db.conn).await.unwrap().unwrap()
}

#[test]
fn review_origin_is_a_closed_origin_without_credentials_or_redirect_payload() {
    for value in [
        "http://public.example",
        "https://operator:secret@example.com",
        "https://example.com/path",
        "https://example.com/?token=secret",
        "https://example.com/#token",
        "javascript:alert(1)",
    ] {
        assert!(origin(value).is_err(), "{value}");
    }
    assert_eq!(
        origin("https://desk.example/").unwrap(),
        "https://desk.example"
    );
    assert_eq!(
        origin("http://127.0.0.1:4323").unwrap(),
        "http://127.0.0.1:4323"
    );
}

#[test]
fn private_identity_rejects_group_username_and_noncanonical_ids() {
    for value in ["-100123", "@operator", "0", "00123", "+123", " 123"] {
        assert!(private_id(value).is_err());
    }
    assert_eq!(private_id("123").unwrap(), 123);
}

#[tokio::test]
async fn default_off_and_missing_key_never_activate_or_consume_a_proposal() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    let reads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let reads_in_callback = reads.clone();
    let never = TelegramRuntime::fixture(
        move |_| {
            reads_in_callback.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            None
        },
        provider.runtime.mock.unwrap(),
    );
    tick(&db.conn, &never).await.unwrap();
    settings(&db, &op, &never, false).await;
    // Even a legacy channel enabled for ACP is not Ops opt-in.
    chat_channel::Entity::update_many()
        .col_expr(chat_channel::Column::Enabled, Expr::value(true))
        .exec(&db.conn)
        .await
        .unwrap();
    tick(&db.conn, &never).await.unwrap();
    assert_eq!(provider.sends(), 0);
    assert_eq!(reads.load(std::sync::atomic::Ordering::SeqCst), 0);
    config::Entity::update_many()
        .col_expr(config::Column::Enabled, Expr::value(true))
        .exec(&db.conn)
        .await
        .unwrap();
    let missing = TelegramRuntime::fixture(|_| None, provider.runtime.mock.unwrap());
    assert_eq!(
        notify_now(&db.conn, &op, &missing).await.unwrap().state,
        "missing_token"
    );
    assert!(notice::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
    assert_eq!(
        ops_proposal::Entity::find_by_id(p.id)
            .one(&db.conn)
            .await
            .unwrap()
            .unwrap()
            .status,
        "pending"
    );
    notify_now(&db.conn, &op, &provider.runtime).await.unwrap();
    assert_eq!(provider.sends(), 1);
}

#[tokio::test]
async fn concurrent_scans_and_restart_send_one_sanitized_notice_with_no_approval() {
    let dir = tempfile::tempdir().unwrap();
    let db = db::init_database(dir.path(), "telegram-dedupe")
        .await
        .unwrap();
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    settings(&db, &op, &provider.runtime, true).await;
    let (a, b) = tokio::join!(
        notify_now(&db.conn, &op, &provider.runtime),
        tick(&db.conn, &provider.runtime)
    );
    a.unwrap();
    b.unwrap();
    let row = first_notice(&db).await;
    assert_eq!(row.status, "sent");
    assert_eq!(row.provider_message_id.as_deref(), Some("41"));
    assert_eq!((row.task_id, row.run_seq), (p.task_id, p.run_seq));
    assert_eq!(provider.sends(), 1);
    let seen = provider.state.seen.lock().unwrap().clone();
    let body = &seen.iter().find(|(m, _)| m == "/sendMessage").unwrap().1;
    assert_eq!(body["chat_id"], "123");
    assert_eq!(body["link_preview_options"]["is_disabled"], true);
    for private in [
        "Original approved body",
        "reader@example.com",
        "synthetic-only",
        "A real local thread",
        "approve",
        "payload",
    ] {
        // Fixed wording may mention a payload, but never include its content.
        if private != "payload" {
            assert!(!body.to_string().contains(private));
        }
    }
    assert!(body["parse_mode"].is_null() && body["reply_markup"].is_null());
    let resolved = resolve(
        &db.conn,
        &op,
        ResolveInput {
            notice: row.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        resolved.proposal.unwrap().payload.unwrap().reply,
        draft.reply
    );
    assert!(
        resolve(&db.conn, &operator(2), ResolveInput { notice: row.id })
            .await
            .unwrap()
            .proposal
            .is_none()
    );
    drop(db);
    let reopened = db::init_database(dir.path(), "telegram-dedupe")
        .await
        .unwrap();
    tick(&reopened.conn, &provider.runtime).await.unwrap();
    assert_eq!(provider.sends(), 1);
    assert_eq!(
        ops_proposal::Entity::find_by_id(p.id)
            .one(&reopened.conn)
            .await
            .unwrap()
            .unwrap()
            .status,
        "pending"
    );
}

#[tokio::test]
async fn transient_recipient_failure_retries_without_duplicate_send_or_new_notice() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    settings(&db, &op, &provider.runtime, true).await;
    provider.reply(
        StatusCode::SERVICE_UNAVAILABLE,
        json!({"ok":false,"description":"sensitive provider diagnostic"}),
        Duration::ZERO,
    );
    notify_now(&db.conn, &op, &provider.runtime).await.unwrap();
    let before = first_notice(&db).await;
    assert_eq!(before.status, "preflight_failed");
    assert_eq!(provider.sends(), 0);
    let (a, b) = tokio::join!(
        notify_now(&db.conn, &op, &provider.runtime),
        notify_now(&db.conn, &op, &provider.runtime)
    );
    a.unwrap();
    b.unwrap();
    let after = first_notice(&db).await;
    assert_eq!(after.id, before.id);
    assert_ne!(after.claim_id, before.claim_id);
    assert_eq!(after.status, "sent");
    assert_eq!(provider.sends(), 1);
    assert!(
        !serde_json::to_string(&status(&db.conn, &op, &provider.runtime).await.unwrap())
            .unwrap()
            .contains("sensitive")
    );
}

#[tokio::test]
async fn cancellation_is_bounded_and_sending_is_durably_unknown_without_retry() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    settings(&db, &op, &provider.runtime, true).await;
    provider.reply(
        StatusCode::OK,
        json!({"ok":true,"result":{"id":123,"type":"private"}}),
        Duration::ZERO,
    );
    provider.reply(StatusCode::OK, json!({"ok":true}), Duration::from_secs(5));
    let start = std::time::Instant::now();
    bounded_scan(
        &db.conn,
        &provider.runtime,
        None,
        Duration::from_millis(250),
    )
    .await
    .unwrap();
    assert!(start.elapsed() < Duration::from_secs(2));
    assert_eq!(first_notice(&db).await.status, "unknown");
    assert_eq!(provider.sends(), 1);
    tick(&db.conn, &provider.runtime).await.unwrap();
    assert_eq!(provider.sends(), 1);
}

#[tokio::test]
async fn recipient_check_timeout_is_recoverable_and_an_old_claim_cannot_send() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    settings(&db, &op, &provider.runtime, true).await;
    provider.reply(StatusCode::OK, json!({"ok":true}), Duration::from_secs(5));
    bounded_scan(
        &db.conn,
        &provider.runtime,
        None,
        Duration::from_millis(100),
    )
    .await
    .unwrap();
    let old = first_notice(&db).await;
    assert_eq!(old.status, "preflight_failed");
    assert_eq!(provider.sends(), 0);
    let cfg = config::Entity::find_by_id(1)
        .one(&db.conn)
        .await
        .unwrap()
        .unwrap();
    let new = claim(&db.conn, &cfg, p.id, "replacement-claim")
        .await
        .unwrap()
        .unwrap();
    dispatch(
        &db.conn,
        &cfg,
        &provider.runtime.backend(&cfg).await.unwrap(),
        &old,
    )
    .await
    .unwrap();
    assert_eq!(
        provider.sends(),
        0,
        "old lease cannot send after replacement"
    );
    dispatch(
        &db.conn,
        &cfg,
        &provider.runtime.backend(&cfg).await.unwrap(),
        &new,
    )
    .await
    .unwrap();
    assert_eq!(provider.sends(), 1);
}

#[tokio::test]
async fn wrong_private_recipient_topic_or_sender_receipts_never_count_as_delivery() {
    for receipt in [
        json!({"message_id":41,"chat":{"id":456,"type":"private"},"from":{"is_bot":true}}),
        json!({"message_id":41,"chat":{"id":123,"type":"group"},"from":{"is_bot":true}}),
        json!({"message_id":41,"chat":{"id":123,"type":"private"},"message_thread_id":17,"from":{"is_bot":true}}),
        json!({"message_id":41,"chat":{"id":123,"type":"private"},"from":{"is_bot":false}}),
        json!({"message_id":0,"chat":{"id":123,"type":"private"},"from":{"is_bot":true}}),
    ] {
        let db = fresh_in_memory_db().await;
        let (op, key) = seed(&db, 1).await;
        let draft = saved(&db, &op, key).await;
        pending(&db, &op, &draft).await;
        let provider = Provider::new().await;
        settings(&db, &op, &provider.runtime, true).await;
        provider.reply(
            StatusCode::OK,
            json!({"ok":true,"result":{"id":123,"type":"private"}}),
            Duration::ZERO,
        );
        provider.reply(
            StatusCode::OK,
            json!({"ok":true,"result":receipt}),
            Duration::ZERO,
        );
        notify_now(&db.conn, &op, &provider.runtime).await.unwrap();
        assert_eq!(first_notice(&db).await.status, "unknown");
        tick(&db.conn, &provider.runtime).await.unwrap();
        assert_eq!(provider.sends(), 1);
    }
}

#[tokio::test]
async fn private_preflight_and_channel_configuration_fail_closed_without_any_send() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    let configured = settings(&db, &op, &provider.runtime, true)
        .await
        .configuration
        .unwrap();
    for chat in [
        json!({"id":456,"type":"private"}),
        json!({"id":123,"type":"supergroup"}),
    ] {
        provider.reply(
            StatusCode::OK,
            json!({"ok":true,"result":chat}),
            Duration::ZERO,
        );
        notify_now(&db.conn, &op, &provider.runtime).await.unwrap();
        assert_eq!(first_notice(&db).await.status, "preflight_failed");
        assert_eq!(provider.sends(), 0);
    }
    for value in [
        json!({"chat_id":"123","topic_mode":true}),
        json!({"chat_id":"456","topic_mode":false}),
    ] {
        chat_channel::Entity::update_many()
            .col_expr(
                chat_channel::Column::ConfigJson,
                Expr::value(value.to_string()),
            )
            .exec(&db.conn)
            .await
            .unwrap();
        assert_eq!(
            notify_now(&db.conn, &op, &provider.runtime)
                .await
                .unwrap()
                .state,
            "recipient_changed"
        );
        assert!(configure(
            &db.conn,
            &op,
            &provider.runtime,
            ConfigureInput {
                channel_id: configured.channel_id,
                private_user_id: "123".into(),
                review_origin: configured.review_origin.clone(),
                enabled: true,
                github_issues_enabled: false,
                expected_revision: Some(configured.revision.clone())
            }
        )
        .await
        .is_err());
        assert_eq!(provider.sends(), 0);
    }
    disable(&db.conn, &op, &provider.runtime).await.unwrap();
}

#[tokio::test]
async fn edited_denied_cancelled_and_old_run_links_never_resolve_an_action() {
    for change in ["edit", "deny", "cancel", "run", "recipient", "disable"] {
        let db = fresh_in_memory_db().await;
        let (op, key) = seed(&db, 1).await;
        let draft = saved(&db, &op, key).await;
        let p = pending(&db, &op, &draft).await;
        let provider = Provider::new().await;
        settings(&db, &op, &provider.runtime, true).await;
        notify_now(&db.conn, &op, &provider.runtime).await.unwrap();
        let row = first_notice(&db).await;
        match change {
            "edit" => {
                let mut reply = draft.reply.clone();
                reply.text = "New private snapshot".into();
                store::save_draft(
                    &db.conn,
                    &op,
                    SaveDraftInput {
                        inbox_id: key.inbox_id,
                        conversation_id: key.conversation_id,
                        expected_revision: draft.revision,
                        reply,
                    },
                )
                .await
                .unwrap();
            }
            "deny" => {
                review::deny(
                    &db.conn,
                    &op,
                    DenyInput {
                        id: p.id,
                        expected_payload: serde_json::from_str(&p.payload_json).unwrap(),
                    },
                )
                .await
                .unwrap();
            }
            "cancel" => {
                assert!(
                    crate::db::service::work_task_service::cancel(&db.conn, p.task_id, None)
                        .await
                        .unwrap()
                );
            }
            "run" => {
                db.conn
                    .execute_unprepared("UPDATE work_task SET run_seq = run_seq + 1")
                    .await
                    .unwrap();
            }
            "recipient" => {
                db.conn.execute_unprepared("UPDATE chat_channel SET config_json = '{\"chat_id\":\"456\",\"topic_mode\":false}'").await.unwrap();
            }
            _ => {
                disable(&db.conn, &op, &provider.runtime).await.unwrap();
            }
        }
        let result = resolve(&db.conn, &op, ResolveInput { notice: row.id })
            .await
            .unwrap();
        assert_eq!(result.state, "unavailable", "{change}");
        assert!(result.proposal.is_none());
        tick(&db.conn, &provider.runtime).await.unwrap();
        assert_eq!(provider.sends(), 1);
    }
}

#[tokio::test]
async fn concurrent_edit_during_recipient_lookup_is_rechecked_before_send() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    settings(&db, &op, &provider.runtime, true).await;
    provider
        .state
        .pause_check
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let scan = notify_now(&db.conn, &op, &provider.runtime);
    let edit = async {
        provider.state.started.notified().await;
        let mut reply = draft.reply.clone();
        reply.text = "Race winner".into();
        store::save_draft(
            &db.conn,
            &op,
            SaveDraftInput {
                inbox_id: key.inbox_id,
                conversation_id: key.conversation_id,
                expected_revision: draft.revision,
                reply,
            },
        )
        .await
        .unwrap();
        provider.state.release.notify_one();
    };
    let (result, ()) = tokio::join!(scan, edit);
    result.unwrap();
    assert_eq!(provider.sends(), 0);
    assert_eq!(first_notice(&db).await.status, "obsolete");
}

#[tokio::test]
async fn named_migration_rolls_back_partial_ddl_and_preserves_accepted_tables() {
    let db = fresh_in_memory_db().await;
    let migration = db::migration::Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260908_000007_ops_telegram")
        .unwrap();
    let manager = SchemaManager::new(&db.conn);
    migration.down(&manager).await.unwrap();
    db.conn
        .execute_unprepared("CREATE TABLE ops_telegram_notice (id INTEGER)")
        .await
        .unwrap();
    assert!(migration.up(&manager).await.is_err());
    assert!(!manager.has_table("ops_telegram_config").await.unwrap());
    for table in [
        "ops_proposal",
        "ops_reply_draft",
        "ops_email_attempt",
        "ops_ticket_message",
    ] {
        assert!(manager.has_table(table).await.unwrap());
    }
    db.conn
        .execute_unprepared("DROP TABLE ops_telegram_notice")
        .await
        .unwrap();
    migration.up(&manager).await.unwrap();
    assert!(manager.has_table("ops_telegram_notice").await.unwrap());
}

#[tokio::test]
async fn review_locator_and_sender_strings_are_not_operator_authority() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    settings(&db, &op, &provider.runtime, true).await;
    notify_now(&db.conn, &op, &provider.runtime).await.unwrap();
    let n = first_notice(&db).await;
    let dir = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let router = crate::web::router::build_router(
        state.clone(),
        "synthetic-telegram-operator".into(),
        dir.path().into(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    )
    .layer(axum::Extension(provider.runtime.clone()));
    let server = axum_test::TestServer::new(router).unwrap();
    for route in ["status", "configure", "disable", "notify", "resolve"] {
        server
            .post(&format!("/api/ops_telegram_{route}"))
            .json(&json!({"input":{"notice":n.id,"senderId":"123","actor":"operator:http"}}))
            .await
            .assert_status_unauthorized();
    }
    let auth = "Bearer synthetic-telegram-operator";
    server
        .post("/api/ops_telegram_resolve")
        .add_header("authorization", auth)
        .json(&json!({"input":{"notice":n.id,"senderId":"123"}}))
        .await
        .assert_status_unprocessable_entity();
    let resolved = server
        .post("/api/ops_telegram_resolve")
        .add_header("authorization", auth)
        .json(&json!({"input":{"notice":n.id}}))
        .await;
    resolved.assert_status_ok();
    assert_eq!(resolved.json::<serde_json::Value>()["proposal"]["id"], p.id);
    // No ACP/phone decision endpoint exists. Even an alleged approve-always
    // callback with the correct locator cannot resolve the Ops proposal.
    server
        .post("/api/ops_telegram_approve")
        .add_header("authorization", auth)
        .json(&json!({"notice":n.id,"senderId":"123","always":true}))
        .await
        .assert_status(StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        ops_proposal::Entity::find_by_id(p.id)
            .one(&state.db.conn)
            .await
            .unwrap()
            .unwrap()
            .status,
        "pending"
    );
}
