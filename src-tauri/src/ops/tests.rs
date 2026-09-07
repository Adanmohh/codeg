use super::{review, store, types::*, Operator};
use crate::db::{
    self,
    entities::{ops_proposal, work_task},
    error::DbError,
    service::{
        ops_approvals::ProposalOutcome, ticket_service as tickets, work_task_service as tasks,
    },
    test_helpers::{fresh_in_memory_db, seed_conversation, seed_folder},
    AppDatabase,
};
use crate::models::{agent::AgentType, WorkTaskDraft, WorkTaskStatus};
use sea_orm::{ConnectionTrait, EntityTrait};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use serde_json::{json, Value};

fn operator(account_id: i32) -> Operator {
    Operator {
        account_id,
        actor: "operator:http",
    }
}

async fn seed(db: &AppDatabase, account: i32) -> (Operator, ThreadKeyFixture) {
    let op = operator(account);
    let inbox = tickets::create_inbox(&db.conn, account, "Support", "support@example.com")
        .await
        .unwrap();
    let mail = tickets::ingest_email(
        &db.conn,
        op.scope(inbox.id),
        tickets::IncomingEmail {
            headers: tickets::threading::ThreadHeaders::default(),
            message_id: "fixture@example.com".into(),
            sender_email: "reader@example.com".into(),
            sender_name: Some("Reader".into()),
            subject: "A real local thread".into(),
            content: "Please help".into(),
            auto_reply: false,
        },
    )
    .await
    .unwrap();
    (
        op,
        ThreadKeyFixture {
            inbox_id: inbox.id,
            conversation_id: mail.conversation.id,
        },
    )
}
type ThreadKeyFixture = ThreadInput;
async fn saved(db: &AppDatabase, op: &Operator, key: ThreadInput) -> Draft {
    let mut reply = store::thread(&db.conn, op, key)
        .await
        .unwrap()
        .suggested_reply;
    reply.text = "Original approved body".into();
    store::save_draft(
        &db.conn,
        op,
        SaveDraftInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            expected_revision: 0,
            reply,
        },
    )
    .await
    .unwrap()
}
async fn start(db: &AppDatabase) -> (i32, i32) {
    let folder = seed_folder(db, "/tmp/ops-ui-task-fixture").await;
    let conversation = seed_conversation(db, folder, AgentType::Codex).await;
    let task = tasks::create(&db.conn, WorkTaskDraft { folder_id: folder,
        title: "Review email".into(), config: json!({"display_text":"Review", "prompt_blocks":[{"type":"text","text":"Review"}]}) }).await.unwrap();
    let seq = tasks::claim_for_run(&db.conn, task.id, WorkTaskStatus::Todo, "user")
        .await
        .unwrap()
        .unwrap();
    assert!(tasks::begin_setup(&db.conn, task.id, seq).await.unwrap());
    assert!(
        tasks::mark_running(&db.conn, task.id, seq, conversation, "ops-ui-fixture")
            .await
            .unwrap()
    );
    (task.id, seq)
}
async fn pending(db: &AppDatabase, op: &Operator, draft: &Draft) -> ops_proposal::Model {
    let (task, seq) = start(db).await;
    match review::propose_reply(
        &db.conn,
        task,
        seq,
        "fixture-agent",
        op.account_id,
        draft.id,
        draft.revision,
    )
    .await
    .unwrap()
    {
        ProposalOutcome::Pending(p) => *p,
        _ => panic!("email must always require human review"),
    }
}
fn review_input(p: &ops_proposal::Model) -> ReviewInput {
    let expected_payload: Value = serde_json::from_str(&p.payload_json).unwrap();
    ReviewInput {
        id: p.id,
        approved_payload: serde_json::from_value(expected_payload.clone()).unwrap(),
        expected_payload,
    }
}

#[tokio::test]
async fn thread_and_note_scope_cannot_cross_accounts_or_inboxes() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let (other, foreign) = seed(&db, 2).await;
    assert!(store::thread(&db.conn, &op, foreign).await.is_err());
    assert!(store::thread(
        &db.conn,
        &op,
        ThreadInput {
            inbox_id: key.inbox_id,
            conversation_id: foreign.conversation_id
        }
    )
    .await
    .is_err());
    assert!(store::add_note(
        &db.conn,
        &op,
        NoteInput {
            inbox_id: foreign.inbox_id,
            conversation_id: foreign.conversation_id,
            content: "private".into()
        }
    )
    .await
    .is_err());
    let note = store::add_note(
        &db.conn,
        &other,
        NoteInput {
            inbox_id: foreign.inbox_id,
            conversation_id: foreign.conversation_id,
            content: "Internal only".into(),
        },
    )
    .await
    .unwrap();
    assert!(note.private);
    assert_eq!(note.author, "operator:http");
    assert!(note.source_id.is_none());
    assert_eq!(
        tickets::list_messages(
            &db.conn,
            other.scope(foreign.inbox_id),
            foreign.conversation_id,
            tickets::MessageView::Public
        )
        .await
        .unwrap()
        .len(),
        1
    );
    assert_eq!(
        store::context(&db.conn, &op).await.unwrap().inboxes.len(),
        1
    );
    assert_eq!(
        store::morning(&db.conn, &op).await.unwrap().tickets[0].id,
        key.conversation_id
    );
}

#[tokio::test]
async fn draft_cas_rejects_stale_and_forged_scope_without_overwriting() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let mut reply = draft.reply.clone();
    reply.text = "older tab".into();
    let result = store::save_draft(
        &db.conn,
        &op,
        SaveDraftInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            expected_revision: 0,
            reply: reply.clone(),
        },
    )
    .await;
    assert!(matches!(result, Err(DbError::Conflict(_))));
    reply.inbox_id += 1;
    assert!(store::save_draft(
        &db.conn,
        &op,
        SaveDraftInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            expected_revision: 1,
            reply
        }
    )
    .await
    .is_err());
    let found = store::thread(&db.conn, &op, key)
        .await
        .unwrap()
        .draft
        .unwrap();
    assert_eq!(found.reply, draft.reply);
    assert_eq!(found.revision, 1);
}

#[tokio::test]
async fn unconfigured_approval_preserves_pending_payload_wait_and_no_public_reply() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let err = review::approve(&db.conn, &op, review_input(&p))
        .await
        .err()
        .unwrap();
    assert!(matches!(
        err.code,
        crate::app_error::AppErrorCode::ConfigurationMissing
    ));
    let row = ops_proposal::Entity::find_by_id(p.id)
        .one(&db.conn)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.status, "pending");
    assert_eq!(row.payload_json, p.payload_json);
    let task = work_task::Entity::find_by_id(p.task_id)
        .one(&db.conn)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(task.status, WorkTaskStatus::AwaitingInput);
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
async fn scoped_proposals_do_not_leak_payload_to_another_account() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    assert!(review::list(&db.conn, &operator(2))
        .await
        .unwrap()
        .is_empty());
    assert!(
        review::get(&db.conn, &operator(2), ProposalInput { id: p.id })
            .await
            .is_err()
    );
    assert!(review::deny(
        &db.conn,
        &operator(2),
        DenyInput {
            id: p.id,
            expected_payload: review_input(&p).expected_payload
        }
    )
    .await
    .is_err());
}

#[tokio::test]
async fn exact_edited_payload_handoff_is_owned_redacted_and_not_replayable() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let mut input = review_input(&p);
    input.approved_payload.reply.to = vec!["new-reader@example.com".into()];
    input.approved_payload.reply.cc = vec!["visible@example.com".into()];
    input.approved_payload.reply.bcc = vec!["hidden@example.com".into()];
    input.approved_payload.reply.text = "The human's exact replacement".into();
    let exact = input.approved_payload.clone();
    let authorized = review::authorize_for_test(&db.conn, &op, input)
        .await
        .unwrap();
    assert_eq!(authorized.proposal_id(), p.id);
    assert_eq!(authorized.payload(), &exact);
    let (_, payload) = authorized.into_parts();
    assert_eq!(payload, exact);
    assert!(review::authorize_for_test(&db.conn, &op, review_input(&p))
        .await
        .is_err());
    let row = ops_proposal::Entity::find_by_id(p.id)
        .one(&db.conn)
        .await
        .unwrap()
        .unwrap();
    assert!(!row.edited_payload_json.unwrap().contains("replacement"));
    assert_eq!(
        store::thread(&db.conn, &op, key)
            .await
            .unwrap()
            .draft
            .unwrap()
            .reply
            .text,
        "Original approved body"
    );
    assert_eq!(
        store::thread(&db.conn, &op, key)
            .await
            .unwrap()
            .messages
            .len(),
        1,
        "authorization alone records no delivery"
    );
}

#[tokio::test]
async fn changed_draft_is_stale_and_deny_remains_possible() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let mut reply = draft.reply;
    reply.text = "newer draft".into();
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
    assert!(
        review::get(&db.conn, &op, ProposalInput { id: p.id })
            .await
            .unwrap()
            .stale
    );
    assert!(review::authorize_for_test(&db.conn, &op, review_input(&p))
        .await
        .is_err());
    let denied = review::deny(
        &db.conn,
        &op,
        DenyInput {
            id: p.id,
            expected_payload: review_input(&p).expected_payload,
        },
    )
    .await
    .unwrap();
    assert_eq!(denied.status, "denied");
    assert!(denied.payload.is_none());
}

#[tokio::test]
async fn edited_binding_and_original_snapshot_are_both_checked() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let mut input = review_input(&p);
    input.approved_payload.draft_revision += 1;
    assert!(review::authorize_for_test(&db.conn, &op, input)
        .await
        .is_err());
    let mut input = review_input(&p);
    input.expected_payload["reply"]["text"] = json!("another card");
    assert!(review::authorize_for_test(&db.conn, &op, input)
        .await
        .is_err());
    let mut input = review_input(&p);
    input.approved_payload.reply.from = "foreign@example.com".into();
    assert!(review::authorize_for_test(&db.conn, &op, input)
        .await
        .is_err());
}

#[tokio::test]
async fn migration_targets_its_name_and_rolls_back_ddl_failure() {
    let db = fresh_in_memory_db().await;
    let migration = db::migration::Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260907_000003_ops_ui")
        .unwrap();
    let manager = SchemaManager::new(&db.conn);
    db.conn.execute_unprepared("CREATE TABLE ops_ui_test_follower (id INTEGER); CREATE INDEX ops_ui_test_follower_index ON ops_ui_test_follower(id)").await.unwrap();
    migration.down(&manager).await.unwrap();
    db.conn
        .execute_unprepared("CREATE INDEX idx_ops_reply_draft_scope ON ops_ui_test_follower(id)")
        .await
        .unwrap();
    assert!(migration.up(&manager).await.is_err());
    assert!(!manager.has_table("ops_reply_draft").await.unwrap());
    assert!(manager.has_table("ops_ticket_conversation").await.unwrap());
    assert!(manager.has_table("ops_ui_test_follower").await.unwrap());
    db.conn
        .execute_unprepared("DROP INDEX idx_ops_reply_draft_scope")
        .await
        .unwrap();
    migration.up(&manager).await.unwrap();
    assert!(manager.has_table("ops_reply_draft").await.unwrap());
}

#[tokio::test]
async fn concurrent_save_and_approval_use_a_single_sqlite_write_order() {
    let dir = tempfile::tempdir().unwrap();
    let db = db::init_database(dir.path(), "ops-ui-test").await.unwrap();
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let p = pending(&db, &op, &draft).await;
    let original = draft.reply.clone();
    let mut newer = original.clone();
    newer.text = "Racing newer draft".into();
    let (save, approve) = tokio::join!(
        store::save_draft(
            &db.conn,
            &op,
            SaveDraftInput {
                inbox_id: key.inbox_id,
                conversation_id: key.conversation_id,
                expected_revision: 1,
                reply: newer
            }
        ),
        review::authorize_for_test(&db.conn, &op, review_input(&p))
    );
    assert_eq!(save.unwrap().revision, 2);
    if let Ok(authorized) = approve {
        assert_eq!(authorized.payload().reply, original);
    } else {
        assert_eq!(
            ops_proposal::Entity::find_by_id(p.id)
                .one(&db.conn)
                .await
                .unwrap()
                .unwrap()
                .status,
            "pending"
        );
    }
    db.conn.close().await.unwrap();
    let reopened = db::init_database(dir.path(), "ops-ui-test").await.unwrap();
    assert_eq!(
        store::thread(&reopened.conn, &op, key)
            .await
            .unwrap()
            .draft
            .unwrap()
            .reply
            .text,
        "Racing newer draft"
    );
}

#[tokio::test]
async fn real_router_auth_actor_injection_and_scope_matrix() {
    use std::sync::Arc;
    let dir = tempfile::tempdir().unwrap();
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let (_, foreign) = seed(&db, 2).await;
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().to_path_buf(),
    ));
    let router = crate::web::router::build_router(
        state.clone(),
        "ops-fixture-token".into(),
        dir.path().to_path_buf(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    );
    let server = axum_test::TestServer::new(router).unwrap();
    for path in [
        "ops_context",
        "ops_thread",
        "ops_note_add",
        "ops_draft_save",
        "ops_proposal_approve",
        "ops_proposal_deny",
        "ops_morning",
    ] {
        server
            .post(&format!("/api/{path}"))
            .json(&json!({}))
            .await
            .assert_status_unauthorized();
        server
            .post(&format!("/api/{path}"))
            .add_header("authorization", "Bearer wrong")
            .json(&json!({}))
            .await
            .assert_status_unauthorized();
    }
    server
        .post("/api/ops_context")
        .add_header("authorization", "Bearer ops-fixture-token")
        .json(&json!({"actor":"forged"}))
        .await
        .assert_status_unprocessable_entity();
    for field in ["actor", "accountId", "actionName", "private"] {
        let mut input =
            json!({"inboxId":key.inbox_id, "conversationId":key.conversation_id, "content":"note"});
        input[field] = json!("forged");
        server
            .post("/api/ops_note_add")
            .add_header("authorization", "Bearer ops-fixture-token")
            .json(&json!({"input":input}))
            .await
            .assert_status_unprocessable_entity();
    }
    server
        .post("/api/ops_thread")
        .add_header("authorization", "Bearer ops-fixture-token")
        .json(&json!({"input":foreign}))
        .await
        .assert_status_not_found();
    let note = server.post("/api/ops_note_add").add_header("authorization", "Bearer ops-fixture-token").json(&json!({"input":{"inboxId":key.inbox_id,"conversationId":key.conversation_id,"content":"Operator note"}})).await;
    note.assert_status_ok();
    assert_eq!(note.json::<Value>()["author"], op.actor);
    assert_eq!(note.json::<Value>()["private"], true);
    server
        .post("/api/ops_execute")
        .add_header("authorization", "Bearer ops-fixture-token")
        .json(&json!({}))
        .await
        .assert_status_not_found();
}
