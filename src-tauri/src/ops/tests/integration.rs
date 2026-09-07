//! Real protected router + accepted Resend client, synthetic loopback provider.
//! Fixture HTTP pattern borrowed from email_transport/tests.rs at 2fecb1cf.
use super::*;
use crate::ops::{
    delivery,
    email::{self, EmailRuntime, SecretStore},
    email_entity::attempt,
};
use axum::{
    body::to_bytes,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Router,
};
use sea_orm::{ColumnTrait, QueryFilter};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
    time::Duration,
};

const RECEIPT: &str = "67d9bcdb-5a02-42d7-8da9-0d6feea18cff";
const TOKEN: &str = "ops-ui-synthetic-operator";
mod browser;
mod telegram_browser;
#[derive(Default)]
struct MemorySecrets(Mutex<HashMap<String, String>>);
impl SecretStore for MemorySecrets {
    fn get(&self, key: &str) -> Option<String> {
        self.0.lock().unwrap().get(key).cloned()
    }
    fn set(&self, key: &str, value: &str) -> Result<(), ()> {
        self.0.lock().unwrap().insert(key.into(), value.into());
        Ok(())
    }
    fn delete(&self, key: &str) -> Result<(), ()> {
        self.0.lock().unwrap().remove(key);
        Ok(())
    }
}
#[derive(Clone, Default)]
struct ProviderState {
    replies: Arc<Mutex<VecDeque<(StatusCode, Value, Duration)>>>,
    seen: Arc<Mutex<Vec<(String, HeaderMap, Value)>>>,
    browser: bool,
}
struct Provider {
    state: ProviderState,
    runtime: Arc<EmailRuntime>,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Provider {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl Provider {
    async fn new() -> Self {
        Self::start(false).await
    }
    async fn start(browser: bool) -> Self {
        async fn handle(State(state): State<ProviderState>, req: Request) -> impl IntoResponse {
            let (head, body) = req.into_parts();
            let bytes = to_bytes(body, 2 * 1024 * 1024).await.unwrap();
            let input: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
            let uri = head.uri.to_string();
            state
                .seen
                .lock()
                .unwrap()
                .push((head.uri.to_string(), head.headers, input.clone()));
            let reply = state.replies.lock().unwrap().pop_front();
            let (status, body, delay) = reply.unwrap_or_else(|| {
                assert!(state.browser, "unexpected provider call");
                browser::provider_reply(&uri, &input)
            });
            tokio::time::sleep(delay).await;
            (status, axum::Json(body))
        }
        let state = ProviderState {
            browser,
            ..Default::default()
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let runtime = EmailRuntime::fixture(
            Box::<MemorySecrets>::default(),
            listener.local_addr().unwrap(),
        );
        let router = Router::new().fallback(handle).with_state(state.clone());
        let task = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        Self {
            state,
            runtime,
            task,
        }
    }
    fn reply(&self, status: StatusCode, body: Value, delay: Duration) {
        self.state
            .replies
            .lock()
            .unwrap()
            .push_back((status, body, delay));
    }
    fn count(&self) -> usize {
        self.state.seen.lock().unwrap().len()
    }
    async fn configured(&self, db: &AppDatabase, op: &Operator, key: ThreadInput) {
        let status = email::configure(
            &db.conn,
            op,
            &self.runtime,
            EmailConfigureInput {
                inbox_id: key.inbox_id,
                api_key: "synthetic-provider-key".into(),
            },
        )
        .await
        .unwrap();
        assert!(status.configured);
        assert!(!serde_json::to_string(&status)
            .unwrap()
            .contains("synthetic-provider-key"));
    }
}
fn router(
    state: Arc<crate::app_state::AppState>,
    runtime: Arc<EmailRuntime>,
    static_dir: std::path::PathBuf,
) -> Router {
    crate::web::router::build_router(
        state,
        TOKEN.into(),
        static_dir,
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    )
    .layer(Extension(runtime))
}

#[tokio::test]
async fn protected_approve_sends_exact_edit_once_and_records_only_receipt() {
    let dir = tempfile::tempdir().unwrap();
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let server = axum_test::TestServer::new(router(
        state.clone(),
        provider.runtime.clone(),
        dir.path().into(),
    ))
    .unwrap();
    for route in [
        "ops_email_configure",
        "ops_email_status",
        "ops_email_pull",
        "ops_email_disconnect",
        "ops_email_reconcile_receipt",
    ] {
        server
            .post(&format!("/api/{route}"))
            .json(&json!({}))
            .await
            .assert_status_unauthorized();
    }
    let auth = format!("Bearer {TOKEN}");
    let config = server
        .post("/api/ops_email_configure")
        .add_header("authorization", &auth)
        .json(&json!({"input":{"inboxId":key.inbox_id,"apiKey":"synthetic-provider-key"}}))
        .await;
    config.assert_status_ok();
    assert!(!config.text().contains("synthetic-provider-key"));
    let mut input = review_input(&p);
    input.approved_payload.reply.text = "Exact human replacement".into();
    input.approved_payload.reply.to = vec!["changed@example.com".into()];
    input.approved_payload.reply.cc = vec!["cc@example.com".into()];
    input.approved_payload.reply.bcc = vec!["bcc@example.com".into()];
    input.approved_payload.reply.subject = "Reviewed subject".into();
    let input = json!({"input":{"id":p.id,"expectedPayload":input.expected_payload,"approvedPayload":input.approved_payload}});
    provider.reply(StatusCode::OK, json!({"id":RECEIPT}), Duration::ZERO);
    let response = server
        .post("/api/ops_proposal_approve")
        .add_header("authorization", &auth)
        .json(&input)
        .await;
    response.assert_status_ok();
    assert_eq!(response.json::<Value>()["delivery"]["status"], "sent");
    assert_eq!(response.json::<Value>()["payload"], Value::Null);
    server
        .post("/api/ops_proposal_approve")
        .add_header("authorization", &auth)
        .json(&input)
        .await
        .assert_status_conflict();
    let attempt = attempt::Entity::find()
        .one(&state.db.conn)
        .await
        .unwrap()
        .unwrap();
    {
        let seen = provider.state.seen.lock().unwrap();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, "/emails");
        assert_eq!(seen[0].1["idempotency-key"], attempt.idempotency_key);
        assert_eq!(
            seen[0].2,
            json!({"from":"support@example.com","to":["changed@example.com"],"cc":["cc@example.com"],"bcc":["bcc@example.com"],"subject":"Reviewed subject","text":"Exact human replacement","headers":{"Message-ID":format!("<{}>",attempt.message_id),"In-Reply-To":"<fixture@example.com>","References":"<fixture@example.com>"}})
        );
    }
    let thread = store::thread(&state.db.conn, &op, key).await.unwrap();
    assert_eq!(thread.draft.unwrap().reply.text, "Original approved body");
    assert_eq!(thread.messages.len(), 2);
    assert_eq!(thread.messages[1].content, "Exact human replacement");
    assert_eq!(thread.messages[1].author, "operator:http");
    assert_eq!(
        thread.messages[1].source_id.as_deref(),
        Some(attempt.message_id.as_str())
    );
    assert!(
        delivery::reconcile_receipt(&state.db.conn, &operator(2), ProposalInput { id: p.id })
            .await
            .is_err()
    );
    assert_eq!(provider.count(), 1);
}

#[tokio::test]
async fn ambiguous_outcome_survives_restart_and_blocks_new_keys_even_after_draft_edit() {
    let dir = tempfile::tempdir().unwrap();
    let db = db::init_database(dir.path(), "ops-unknown").await.unwrap();
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    provider.configured(&db, &op, key).await;
    provider.reply(
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"message":"secret provider body"}),
        Duration::ZERO,
    );
    let result = review::approve_using(&db.conn, &op, &provider.runtime, review_input(&p))
        .await
        .unwrap();
    assert_eq!(result.delivery.as_ref().unwrap().status, "unknown");
    assert!(!serde_json::to_string(&result)
        .unwrap()
        .contains("secret provider body"));
    let mut reply = draft.reply.clone();
    reply.text = "New draft cannot bypass unknown outcome".into();
    let changed = store::save_draft(
        &db.conn,
        &op,
        SaveDraftInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            expected_revision: 1,
            reply,
        },
    )
    .await
    .unwrap();
    let next = pending(&db, &op, &changed).await;
    db.conn.close().await.unwrap();
    let reopened = db::init_database(dir.path(), "ops-unknown").await.unwrap();
    assert!(
        review::approve_using(&reopened.conn, &op, &provider.runtime, review_input(&next))
            .await
            .is_err()
    );
    assert!(
        delivery::reconcile_receipt(&reopened.conn, &op, ProposalInput { id: p.id })
            .await
            .is_err()
    );
    assert_eq!(
        attempt::Entity::find()
            .all(&reopened.conn)
            .await
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        store::thread(&reopened.conn, &op, key)
            .await
            .unwrap()
            .messages
            .len(),
        1
    );
    assert_eq!(provider.count(), 1);
}

#[tokio::test]
async fn receipt_recording_failure_is_recoverable_without_network_or_draft_reread() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    provider.configured(&db, &op, key).await;
    db.conn.execute_unprepared("CREATE TRIGGER block_public_reply BEFORE INSERT ON ops_ticket_message WHEN NEW.message_type = 1 BEGIN SELECT RAISE(ABORT, 'synthetic local storage failure'); END").await.unwrap();
    provider.reply(StatusCode::OK, json!({"id":RECEIPT}), Duration::ZERO);
    let result = review::approve_using(&db.conn, &op, &provider.runtime, review_input(&p))
        .await
        .unwrap();
    assert_eq!(result.delivery.unwrap().status, "receipt_recorded");
    assert_eq!(
        store::thread(&db.conn, &op, key)
            .await
            .unwrap()
            .messages
            .len(),
        1
    );
    let mut reply = draft.reply.clone();
    reply.text = "Do not use this newer draft".into();
    store::save_draft(
        &db.conn,
        &op,
        SaveDraftInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            expected_revision: 1,
            reply,
        },
    )
    .await
    .unwrap();
    db.conn
        .execute_unprepared("DROP TRIGGER block_public_reply")
        .await
        .unwrap();
    for _ in 0..2 {
        assert_eq!(
            delivery::reconcile_receipt(&db.conn, &op, ProposalInput { id: p.id })
                .await
                .unwrap()
                .status,
            "sent"
        );
    }
    let thread = store::thread(&db.conn, &op, key).await.unwrap();
    assert_eq!(thread.messages.len(), 2);
    assert_eq!(thread.messages[1].content, draft.reply.text);
    assert_eq!(provider.count(), 1);
}

#[tokio::test]
async fn concurrent_approve_and_pull_are_serialized_and_failure_is_visible() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let provider = Provider::new().await;
    provider.configured(&db, &op, key).await;
    provider.reply(
        StatusCode::UNPROCESSABLE_ENTITY,
        json!({"message":"private detail"}),
        Duration::from_millis(100),
    );
    let (a, b) = tokio::join!(
        review::approve_using(&db.conn, &op, &provider.runtime, review_input(&p)),
        review::approve_using(&db.conn, &op, &provider.runtime, review_input(&p))
    );
    assert!(a.is_ok() ^ b.is_ok());
    assert_eq!(a.or(b).unwrap().delivery.unwrap().status, "failed");
    assert_eq!(provider.count(), 1);
    provider.reply(
        StatusCode::OK,
        json!({"object":"list","has_more":false,"data":[]}),
        Duration::from_millis(100),
    );
    let (a, b) = tokio::join!(
        email::pull(
            &db.conn,
            &op,
            &provider.runtime,
            EmailInboxInput {
                inbox_id: key.inbox_id
            }
        ),
        email::pull(
            &db.conn,
            &op,
            &provider.runtime,
            EmailInboxInput {
                inbox_id: key.inbox_id
            }
        )
    );
    assert!(a.is_ok() ^ b.is_ok());
    assert_eq!(provider.count(), 2);
    let status = email::status(
        &db.conn,
        &op,
        &provider.runtime,
        EmailInboxInput {
            inbox_id: key.inbox_id,
        },
    )
    .await
    .unwrap();
    assert_eq!(status.last_pull_status.as_deref(), Some("complete"));
    assert_eq!(
        store::thread(&db.conn, &op, key)
            .await
            .unwrap()
            .messages
            .len(),
        1
    );
}

#[tokio::test]
async fn reserved_payload_mismatch_never_sends() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let input = review_input(&p);
    let provider = Provider::new().await;
    provider.configured(&db, &op, key).await;
    let mut wrong = input.approved_payload.clone();
    wrong.reply.text = "Reservation differs".into();
    delivery::reserve(&db.conn, &op, &p, &wrong).await.unwrap();
    let authorized = review::authorize_for_test(&db.conn, &op, input)
        .await
        .unwrap();
    let client = provider
        .runtime
        .client(&db.conn, &op, key.inbox_id)
        .await
        .unwrap();
    assert!(delivery::dispatch(&db.conn, client, authorized)
        .await
        .is_err());
    assert_eq!(provider.count(), 0);
    assert_eq!(
        attempt::Entity::find()
            .filter(attempt::Column::ProposalId.eq(p.id))
            .one(&db.conn)
            .await
            .unwrap()
            .unwrap()
            .status,
        "reserved"
    );
}

#[tokio::test]
async fn cancellation_after_authorization_but_before_dispatch_never_sends() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let input = review_input(&p);
    let provider = Provider::new().await;
    provider.configured(&db, &op, key).await;
    delivery::reserve(&db.conn, &op, &p, &input.approved_payload)
        .await
        .unwrap();
    let authorized = review::authorize_for_test(&db.conn, &op, input)
        .await
        .unwrap();
    tasks::cancel(&db.conn, p.task_id, None).await.unwrap();
    let client = provider
        .runtime
        .client(&db.conn, &op, key.inbox_id)
        .await
        .unwrap();
    assert_eq!(
        delivery::dispatch(&db.conn, client, authorized)
            .await
            .unwrap()
            .status,
        "not_sent"
    );
    assert_eq!(provider.count(), 0);
}

#[tokio::test]
async fn email_migration_is_named_atomic_and_preserves_other_migrations() {
    let db = fresh_in_memory_db().await;
    let migration = db::migration::Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260907_000004_ops_email")
        .unwrap();
    let manager = SchemaManager::new(&db.conn);
    migration.down(&manager).await.unwrap();
    db.conn.execute_unprepared("CREATE TABLE email_migration_follower(id INTEGER); CREATE INDEX idx_ops_email_attempt_scope ON email_migration_follower(id)").await.unwrap();
    assert!(migration.up(&manager).await.is_err());
    assert!(!manager.has_table("ops_email_config").await.unwrap());
    assert!(!manager.has_table("ops_email_attempt").await.unwrap());
    for table in [
        "ops_reply_draft",
        "ops_ticket_message",
        "ops_proposal",
        "email_migration_follower",
    ] {
        assert!(manager.has_table(table).await.unwrap());
    }
    db.conn
        .execute_unprepared("DROP INDEX idx_ops_email_attempt_scope")
        .await
        .unwrap();
    migration.up(&manager).await.unwrap();
}
