//! Separate synthetic fixture: 4323 / out-telegram. Never touches fixture 4320.
use super::*;
use crate::ops_telegram::{
    self as telegram, tests::provider::Provider as TelegramProvider, types::ConfigureInput,
};
use axum::{response::Html, routing::get};

const TELEGRAM_TOKEN: &str = "ops-telegram-synthetic-operator";

async fn example(
    db: &AppDatabase,
    op: &Operator,
    key: ThreadInput,
    id: &str,
    subject: &str,
) -> (ThreadInput, Draft, ops_proposal::Model) {
    let mail = tickets::ingest_email(
        &db.conn,
        op.scope(key.inbox_id),
        tickets::IncomingEmail {
            headers: tickets::threading::ThreadHeaders::default(),
            message_id: format!("telegram-{id}@example.com"),
            sender_email: "reader@example.com".into(),
            sender_name: Some("Fixture reader".into()),
            subject: subject.into(),
            content: "Synthetic correspondence for protected review.".into(),
            auto_reply: false,
        },
    )
    .await
    .unwrap();
    let key = ThreadInput {
        inbox_id: key.inbox_id,
        conversation_id: mail.conversation.id,
    };
    let draft = saved(db, op, key).await;
    let proposal = pending(db, op, &draft).await;
    (key, draft, proposal)
}

#[tokio::test]
#[ignore = "manual synthetic Telegram/BC14 fixture on 4323; requires own out-telegram export"]
async fn ops_telegram_browser_fixture() {
    let dir = tempfile::tempdir().unwrap();
    let db = db::init_database(dir.path(), "ops-telegram-browser")
        .await
        .unwrap();
    let email = Provider::start(true).await;
    let telegram_provider = TelegramProvider::new().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let ready = pending(&db, &op, &draft).await;
    store::add_note(
        &db.conn,
        &op,
        NoteInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            content: "Synthetic private finance note; must never appear in a Telegram notice."
                .into(),
        },
    )
    .await
    .unwrap();

    // BC14: real synthetic provider receipt exists, local recording alone fails.
    email.configured(&db, &op, key).await;
    let (_, _, receipt) = example(
        &db,
        &op,
        key,
        "receipt",
        "Provider accepted · local recording pending",
    )
    .await;
    db.conn.execute_unprepared("CREATE TRIGGER block_public_reply BEFORE INSERT ON ops_ticket_message WHEN NEW.message_type = 1 BEGIN SELECT RAISE(ABORT, 'controlled BC14 local storage failure'); END").await.unwrap();
    let result = review::approve_using(&db.conn, &op, &email.runtime, review_input(&receipt))
        .await
        .unwrap();
    assert_eq!(result.delivery.unwrap().status, "receipt_recorded");
    assert_eq!(email.count(), 1);
    db.conn
        .execute_unprepared("DROP TRIGGER block_public_reply")
        .await
        .unwrap();

    let (stale_key, stale_draft, stale) = example(&db, &op, key, "stale", "A changed draft").await;
    let (_, _, denied) = example(&db, &op, key, "denied", "A denied proposal").await;
    let (_, _, cancelled) = example(&db, &op, key, "cancelled", "A canceled task").await;
    let channel = crate::db::service::chat_channel_service::create(
        &db.conn,
        "Synthetic private operator".into(),
        "telegram".into(),
        json!({"chat_id":"123","topic_mode":false}).to_string(),
        false,
        false,
        None,
    )
    .await
    .unwrap();
    telegram::configure(
        &db.conn,
        &op,
        &telegram_provider.runtime,
        ConfigureInput {
            channel_id: channel.id,
            private_user_id: "123".into(),
            review_origin: "http://127.0.0.1:4323".into(),
            enabled: true,
            expected_revision: None,
        },
    )
    .await
    .unwrap();
    telegram::notify_now(&db.conn, &op, &telegram_provider.runtime)
        .await
        .unwrap();
    assert_eq!(telegram_provider.sends(), 4);
    let mut changed = stale_draft.reply;
    changed.text = "A newer saved reply after the notice".into();
    store::save_draft(
        &db.conn,
        &op,
        SaveDraftInput {
            inbox_id: key.inbox_id,
            conversation_id: stale_key.conversation_id,
            expected_revision: stale_draft.revision,
            reply: changed,
        },
    )
    .await
    .unwrap();
    review::deny(
        &db.conn,
        &op,
        DenyInput {
            id: denied.id,
            expected_payload: serde_json::from_str(&denied.payload_json).unwrap(),
        },
    )
    .await
    .unwrap();
    assert!(tasks::cancel(&db.conn, cancelled.task_id, None)
        .await
        .unwrap());

    // This page contains locator-only synthetic notifications, never credentials,
    // private mail, logs or an executable callback. Production has no such route.
    let mut links = String::new();
    for (proposal, label) in [
        (ready.id, "Open pending review"),
        (stale.id, "Open stale link"),
        (denied.id, "Open denied link"),
        (cancelled.id, "Open canceled link"),
    ] {
        let row = db
            .conn
            .query_one(sea_orm::Statement::from_sql_and_values(
                db.conn.get_database_backend(),
                "SELECT id FROM ops_telegram_notice WHERE proposal_id = ?",
                [proposal.into()],
            ))
            .await
            .unwrap()
            .unwrap();
        let notice: String = row.try_get("", "id").unwrap();
        links.push_str(&format!(
            "<li><a href='/ops-review?notice={notice}'>{label}</a></li>"
        ));
    }
    let page = Arc::new(format!("<!doctype html><html lang='en'><meta name='viewport' content='width=device-width,initial-scale=1'><title>Local Telegram fixture</title><style>body{{font:16px system-ui;max-width:40rem;margin:2rem auto;padding:1rem}}li{{margin:1rem 0}}a{{display:inline-block;padding:.75rem}}</style><h1>Local Telegram notification fixture</h1><p>Synthetic loopback only. A remote phone needs a configured reachable protected origin.</p><ul>{links}</ul><p>BC14: open Ops → Approvals → Reply review #{} and choose Finish recording receipt. Email provider requests must remain unchanged.</p><a href='/workspace'>Open Ops workspace</a></html>",receipt.id));
    let email_seen = email.state.seen.clone();
    let telegram_seen = telegram_provider.state.seen.clone();
    let stats = Router::new().route("/api/ops_telegram_fixture_stats",get(move || {
        let email_seen = email_seen.clone(); let telegram_seen = telegram_seen.clone();
        async move {
            axum::Json(json!({
                "syntheticOnly":true,
                "emailProviderRequests":email_seen.lock().unwrap().len(),
                "telegramSendRequests":telegram_seen.lock().unwrap().iter().filter(|(m,_)| m=="/sendMessage").count(),
                "receiptProposalId":receipt.id,
            }))
        }
    })).layer(axum::middleware::from_fn(|req,next| crate::web::auth::require_token(req,next,TELEGRAM_TOKEN.into())));
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let static_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("out-telegram");
    assert!(
        static_dir.join("ops-review.html").exists(),
        "build isolated out-telegram first"
    );
    let router = crate::web::router::build_router(
        state,
        TELEGRAM_TOKEN.into(),
        static_dir,
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    )
    .layer(Extension(email.runtime.clone()))
    .layer(Extension(telegram_provider.runtime.clone()))
    .route(
        "/__telegram_fixture",
        get(move || {
            let page = page.clone();
            async move { Html((*page).clone()) }
        }),
    )
    .merge(stats);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4323")
        .await
        .unwrap();
    println!("Synthetic Telegram/BC14 fixture ready on http://127.0.0.1:4323/__telegram_fixture; PID {}. Existing 4320 unchanged.", std::process::id());
    axum::serve(listener, router).await.unwrap();
}
