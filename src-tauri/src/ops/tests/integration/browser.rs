//! Manual Playwright CLI fixture. Not compiled into a shipped application.
//! Actual protected router/store/gate/client; only provider and keyring replaced.
use super::*;

pub(super) fn provider_reply(uri: &str, input: &Value) -> (StatusCode, Value, Duration) {
    let received = json!({"object":"email","id":RECEIPT,"from":"Reader <reader@example.com>",
        "to":["support@example.com"],"cc":[],"bcc":[],"reply_to":[],"received_for":["support@example.com"],
        "subject":"An imported support question","message_id":"browser-pull@example.com",
        "created_at":"2026-09-08T07:00:00Z","text":"Can you send a copy of my receipt? <script>window.fixtureUnsafe=true</script>","html":null,
        "headers":{"Message-ID":"<browser-pull@example.com>"},"attachments":[]});
    let (status, body) = if uri.starts_with("/emails/receiving?") {
        (
            StatusCode::OK,
            json!({"object":"list","has_more":false,"data":[received]}),
        )
    } else if uri.starts_with("/emails/receiving/") {
        (StatusCode::OK, received)
    } else {
        assert_eq!(uri, "/emails");
        match input["text"].as_str().unwrap_or_default() {
            "simulate unknown" => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"message":"synthetic unknown outcome"}),
            ),
            "simulate rejection" => (
                StatusCode::UNPROCESSABLE_ENTITY,
                json!({"message":"synthetic provider rejection"}),
            ),
            _ => (
                StatusCode::OK,
                json!({"id":uuid::Uuid::new_v4().to_string()}),
            ),
        }
    };
    (status, body, Duration::from_millis(100))
}

#[tokio::test]
#[ignore = "manual isolated browser fixture on 127.0.0.1:4320; stop after Playwright CLI"]
async fn ops_ui_browser_fixture() {
    let dir = tempfile::tempdir().unwrap();
    let db = db::init_database(dir.path(), "ops-ui-browser")
        .await
        .unwrap();
    let provider = Provider::start(true).await;
    if std::env::var("OPS_UI_FIXTURE_EMPTY").as_deref() != Ok("1") {
        let (op, key) = seed(&db, 1).await;
        let draft = saved(&db, &op, key).await;
        pending(&db, &op, &draft).await;
        store::add_note(
            &db.conn,
            &op,
            NoteInput {
                inbox_id: key.inbox_id,
                conversation_id: key.conversation_id,
                content: "Fixture private note: verify the receipt with finance.".into(),
            },
        )
        .await
        .unwrap();
        for (id, subject, text, stale) in [
            (
                "unknown",
                "An uncertain delivery",
                "simulate unknown",
                false,
            ),
            ("reject", "A rejected delivery", "simulate rejection", false),
            ("stale", "An outdated review", "An old prepared reply", true),
        ] {
            let mail = tickets::ingest_email(
                &db.conn,
                op.scope(key.inbox_id),
                tickets::IncomingEmail {
                    headers: tickets::threading::ThreadHeaders::default(),
                    message_id: format!("{id}@example.com"),
                    sender_email: format!("{id}@example.com"),
                    sender_name: Some("Fixture reader".into()),
                    subject: subject.into(),
                    content: "Synthetic browser fixture correspondence.".into(),
                    auto_reply: false,
                },
            )
            .await
            .unwrap();
            let thread = ThreadInput {
                inbox_id: key.inbox_id,
                conversation_id: mail.conversation.id,
            };
            let mut reply = store::thread(&db.conn, &op, thread)
                .await
                .unwrap()
                .suggested_reply;
            reply.text = text.into();
            let draft = store::save_draft(
                &db.conn,
                &op,
                SaveDraftInput {
                    inbox_id: key.inbox_id,
                    conversation_id: thread.conversation_id,
                    expected_revision: 0,
                    reply: reply.clone(),
                },
            )
            .await
            .unwrap();
            pending(&db, &op, &draft).await;
            if stale {
                reply.text = "A newer saved draft".into();
                store::save_draft(
                    &db.conn,
                    &op,
                    SaveDraftInput {
                        inbox_id: key.inbox_id,
                        conversation_id: thread.conversation_id,
                        expected_revision: 1,
                        reply,
                    },
                )
                .await
                .unwrap();
            }
        }
    }
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let static_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("out");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4320")
        .await
        .unwrap();
    println!("Ops UI synthetic fixture ready on 127.0.0.1:4320; no live provider credentials");
    axum::serve(
        listener,
        router(state, provider.runtime.clone(), static_dir),
    )
    .await
    .unwrap();
}
