//! Owned 4326 / out-design-ops fixture. No engine, live provider or keyring.
//! Reuses the accepted Ops/Telegram browser fixtures at 61738519 (Apache-2.0).
use super::*;
use axum::routing::get;

const DESIGN_TOKEN: &str = "ops-design-synthetic-operator";

async fn example(
    db: &AppDatabase,
    op: &Operator,
    inbox: ThreadInput,
    name: &str,
) -> (ThreadInput, ops_proposal::Model) {
    let mail = tickets::ingest_email(
        &db.conn,
        op.scope(inbox.inbox_id),
        tickets::IncomingEmail {
            headers: tickets::threading::ThreadHeaders::default(),
            message_id: format!("design-{name}@example.com"),
            sender_email: "reader@example.com".into(),
            sender_name: Some("Synthetic reader · قارئ تجريبي".into()),
            subject: format!("Synthetic {name} review"),
            content: "Hello (build 42). Reply to reader@example.com, please.\nمرحباً، الصوت يتوقف في الشاشة الرئيسية. Build 42.".into(),
            auto_reply: false,
        },
    )
    .await
    .unwrap();
    let key = ThreadInput {
        inbox_id: inbox.inbox_id,
        conversation_id: mail.conversation.id,
    };
    let draft = saved(db, op, key).await;
    (key, pending(db, op, &draft).await)
}

#[tokio::test]
#[ignore = "manual synthetic Ops design fixture on 4326; own out-design-ops export"]
async fn ops_design_browser_fixture() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let dir = root
        .join(".build/design-ops")
        .join(uuid::Uuid::new_v4().to_string());
    std::fs::create_dir_all(&dir).unwrap();
    let db = db::init_database(&dir, "ops-design-browser").await.unwrap();
    let provider = Provider::start(true).await;
    let (op, inbox) = seed(&db, 1).await;
    provider.configured(&db, &op, inbox).await;
    let (pending_key, pending) = example(&db, &op, inbox, "locale").await;
    store::add_note(
        &db.conn,
        &op,
        NoteInput {
            inbox_id: pending_key.inbox_id,
            conversation_id: pending_key.conversation_id,
            content: "ملاحظة تجريبية داخلية: تحقق من Build 42 (reader@example.com).".into(),
        },
    )
    .await
    .unwrap();

    let (_, receipt) = example(&db, &op, inbox, "recording").await;
    db.conn.execute_unprepared("CREATE TRIGGER block_public_reply BEFORE INSERT ON ops_ticket_message WHEN NEW.message_type = 1 BEGIN SELECT RAISE(ABORT, 'synthetic local recording failure'); END").await.unwrap();
    let result = review::approve_using(&db.conn, &op, &provider.runtime, review_input(&receipt))
        .await
        .unwrap();
    assert_eq!(result.delivery.unwrap().status, "receipt_recorded");
    db.conn
        .execute_unprepared("DROP TRIGGER block_public_reply")
        .await
        .unwrap();

    let (_, sent) = example(&db, &op, inbox, "accepted").await;
    let result = review::approve_using(&db.conn, &op, &provider.runtime, review_input(&sent))
        .await
        .unwrap();
    assert_eq!(result.delivery.unwrap().status, "sent");

    let (_, unknown) = example(&db, &op, inbox, "unknown").await;
    provider.reply(
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"message":"synthetic response lost"}),
        Duration::ZERO,
    );
    let result = review::approve_using(&db.conn, &op, &provider.runtime, review_input(&unknown))
        .await
        .unwrap();
    assert_eq!(result.delivery.unwrap().status, "unknown");

    let (_, failed) = example(&db, &op, inbox, "rejected").await;
    provider.reply(
        StatusCode::UNPROCESSABLE_ENTITY,
        json!({"message":"synthetic rejection"}),
        Duration::ZERO,
    );
    let result = review::approve_using(&db.conn, &op, &provider.runtime, review_input(&failed))
        .await
        .unwrap();
    assert_eq!(result.delivery.unwrap().status, "failed");

    let (_, denied) = example(&db, &op, inbox, "denied").await;
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
    assert_eq!(provider.count(), 4);

    let seen = provider.state.seen.clone();
    let stats = Router::new()
        .route(
            "/api/ops_design_fixture_stats",
            get(move || {
                let seen = seen.clone();
                async move {
                    axum::Json(json!({
                        "syntheticOnly":true,
                        "providerRequests":seen.lock().unwrap().len(),
                        "pendingProposalId":pending.id,
                        "pendingConversationId":pending_key.conversation_id,
                        "inboxId":pending_key.inbox_id,
                        "recordingProposalId":receipt.id,
                        "acceptedProposalId":sent.id,
                        "unknownProposalId":unknown.id,
                        "rejectedProposalId":failed.id,
                        "deniedProposalId":denied.id,
                    }))
                }
            }),
        )
        .layer(axum::middleware::from_fn(|req, next| {
            crate::web::auth::require_token(req, next, DESIGN_TOKEN.into())
        }));
    let state = Arc::new(crate::app_state::AppState::new_for_test(db, dir.clone()));
    let router = crate::web::router::build_router(
        state,
        DESIGN_TOKEN.into(),
        root.join("out-design-ops"),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    )
    .layer(Extension(provider.runtime.clone()))
    .merge(stats);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4326")
        .await
        .unwrap();
    println!(
        "Synthetic Ops design fixture at http://127.0.0.1:4326/workspace; PID {}; state {}",
        std::process::id(),
        dir.display()
    );
    axum::serve(listener, router).await.unwrap();
}
