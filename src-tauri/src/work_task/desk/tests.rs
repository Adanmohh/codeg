//! Production token listener -> TaskEngine -> accepted Ops helpers/gate.
//! Reuses Codeg's task transitions/framing fixtures and Ops public/CAS cases.
use super::super::tests::{relaunch_on, running_task, running_task_in, PARENT_CONN};
use super::*;
use crate::acp::delegation::{
    listener::{tests::desk_listener, DelegationListener, TokenEntry, TokenRegistry},
    transport::{read_frame, write_frame, BrokerDeskRequest, BrokerMessage, BrokerResponse},
};
use crate::acp::work_task_tools::{TaskReportAck, WorkTaskToolAccess};
use crate::db::{
    entities::{
        ops_agent_rule as rule, ops_agent_scope as scope, ops_audit_log as audit, ops_proposal,
    },
    service::ticket_service as tickets,
};
use crate::models::AgentType;
use crate::web::event_bridge::EventEmitter;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, Statement,
    TransactionTrait,
};
use std::{path::Path, sync::Arc, time::Duration};

struct EngineAccess {
    engine: Arc<TaskEngine>,
    entered: tokio::sync::Notify,
}

#[async_trait::async_trait]
impl WorkTaskToolAccess for EngineAccess {
    async fn desk_call(&self, parent: &str, request: DeskCall) -> DeskResponse {
        self.entered.notify_one();
        self.engine.desk_call(parent, request).await
    }
    async fn report_progress(&self, _: &str, _: &str) -> TaskReportAck {
        panic!("Desk must not dispatch a generic task tool")
    }
    async fn complete(&self, _: &str, _: &str, _: Option<&str>) -> TaskReportAck {
        panic!("Desk must not dispatch a generic task tool")
    }
}

async fn bridge(
    engine: &Arc<TaskEngine>,
    connection: &str,
) -> (Arc<DelegationListener>, Arc<EngineAccess>, String) {
    engine
        .manager
        .insert_test_connection(connection, AgentType::Pi, None, EventEmitter::Noop)
        .await;
    let tokens = Arc::new(TokenRegistry::default());
    let token = uuid::Uuid::new_v4().to_string();
    tokens
        .register(
            token.clone(),
            TokenEntry {
                parent_connection_id: connection.into(),
                working_dir: "/fixture/unused".into(),
            },
        )
        .await;
    let access = Arc::new(EngineAccess {
        engine: engine.clone(),
        entered: Default::default(),
    });
    (desk_listener(tokens, access.clone()), access, token)
}

async fn start_call(
    listener: Arc<DelegationListener>,
    token: &str,
    tool: DeskTool,
    input: Value,
) -> (tokio::io::DuplexStream, tokio::task::JoinHandle<()>) {
    let (mut client, mut server) = tokio::io::duplex(8192);
    let served = tokio::spawn(async move {
        listener.serve_one(&mut server).await.unwrap();
    });
    write_frame(
        &mut client,
        &BrokerMessage::Desk(BrokerDeskRequest {
            token: token.into(),
            request: DeskCall { tool, input },
        }),
    )
    .await
    .unwrap();
    (client, served)
}

async fn call(
    listener: &Arc<DelegationListener>,
    token: &str,
    tool: DeskTool,
    input: Value,
) -> DeskResponse {
    let (mut client, served) = start_call(listener.clone(), token, tool, input).await;
    let reply: BrokerResponse =
        tokio::time::timeout(Duration::from_secs(5), read_frame(&mut client))
            .await
            .unwrap()
            .unwrap();
    served.await.unwrap();
    serde_json::from_value(reply.outcome).unwrap()
}

fn succeeded(response: DeskResponse) -> Value {
    assert!(response.ok, "expected success, got {:?}", response.code);
    response.value.unwrap()
}

async fn seed(engine: &TaskEngine, account: i32) -> (tickets::Scope, i32) {
    let inbox = tickets::create_inbox(&engine.db.conn, account, "Support", "support@example.com")
        .await
        .unwrap();
    let scope = tickets::Scope {
        account_id: account,
        inbox_id: inbox.id,
    };
    let email = tickets::ingest_email(
        &engine.db.conn,
        scope,
        tickets::IncomingEmail {
            headers: Default::default(),
            message_id: "fixture@example.com".into(),
            sender_email: "reporter@example.com".into(),
            sender_name: Some("Reporter".into()),
            subject: "Synthetic bridge fixture".into(),
            content: "Public message".into(),
            auto_reply: false,
        },
    )
    .await
    .unwrap();
    (scope, email.conversation.id)
}

fn key(scope: tickets::Scope, conversation: i32) -> Value {
    json!({"inboxId":scope.inbox_id, "conversationId":conversation})
}

async fn save(
    listener: &Arc<DelegationListener>,
    token: &str,
    key: &Value,
    revision: i32,
    reply: Value,
) -> DeskResponse {
    call(
        listener,
        token,
        DeskTool::DeskSaveReply,
        save_input(key, revision, reply),
    )
    .await
}
fn save_input(key: &Value, revision: i32, reply: Value) -> Value {
    json!({"inboxId":key["inboxId"], "conversationId":key["conversationId"], "expectedRevision":revision, "reply":reply})
}

#[tokio::test]
async fn desk_bridge_stable_pi_deny_applies_across_two_launch_uuids() {
    let (engine, task_id) = running_task().await;
    let account = agent::account_id().unwrap();
    let (inbox, conversation) = seed(&engine, account).await;
    let resource = Some(format!("account/{account}/inbox/{}", inbox.inbox_id));
    scope::ActiveModel {
        agent_id: Set("pi".into()),
        domain: Set("email".into()),
        resource: Set(resource.clone()),
        mode: Set("act_low_risk".into()),
        ..Default::default()
    }
    .insert(&engine.db.conn)
    .await
    .unwrap();
    rule::ActiveModel {
        agent_id: Set("pi".into()),
        domain: Set("email".into()),
        resource: Set(resource.clone()),
        action_name: Set(Some("ops.email.reply".into())),
        behavior: Set("allow".into()),
        ..Default::default()
    }
    .insert(&engine.db.conn)
    .await
    .unwrap();
    let deny = rule::ActiveModel {
        agent_id: Set("pi".into()),
        domain: Set("email".into()),
        resource: Set(resource),
        action_name: Set(Some("ops.email.reply".into())),
        behavior: Set("deny".into()),
        ..Default::default()
    }
    .insert(&engine.db.conn)
    .await
    .unwrap();
    let mut previous: Option<(Arc<DelegationListener>, String)> = None;
    let mut draft = None;
    for _ in 0..2 {
        let connection = uuid::Uuid::new_v4().to_string();
        let run = relaunch_on(&engine, task_id, &connection).await;
        let (listener, _, token) = bridge(&engine, &connection).await;
        let context = succeeded(call(&listener, &token, DeskTool::DeskContext, json!({})).await);
        assert_eq!(context["taskId"], task_id);
        assert_eq!(context["runSeq"], run);
        if let Some((old, old_token)) = previous.take() {
            assert_eq!(
                call(&old, &old_token, DeskTool::DeskContext, json!({}))
                    .await
                    .code,
                Some(DeskError::Stale)
            );
        }
        if draft.is_none() {
            let k = key(inbox, conversation);
            let thread = succeeded(call(&listener, &token, DeskTool::DeskThread, k.clone()).await);
            let mut reply = thread["suggestedReply"].clone();
            reply["text"] = "Review this exact synthetic reply".into();
            draft = Some(succeeded(save(&listener, &token, &k, 0, reply).await));
        }
        let d = draft.as_ref().unwrap();
        assert_eq!(
            call(
                &listener,
                &token,
                DeskTool::DeskProposeReply,
                json!({"draftId": d["id"], "expectedRevision":d["revision"]})
            )
            .await
            .code,
            Some(DeskError::Denied)
        );
        previous = Some((listener, token));
    }
    assert!(ops_proposal::Entity::find()
        .all(&engine.db.conn)
        .await
        .unwrap()
        .is_empty());
    let gates = audit::Entity::find()
        .filter(audit::Column::Action.eq("agent.gate"))
        .all(&engine.db.conn)
        .await
        .unwrap();
    assert_eq!(gates.len(), 2);
    assert_ne!(gates[0].run_seq, gates[1].run_seq);
    for row in gates {
        assert_eq!(row.actor, "pi");
        assert_eq!(row.task_id, task_id);
        let detail: Value = serde_json::from_str(&row.detail).unwrap();
        assert_eq!(detail["gate"]["mode"], "act_low_risk");
        assert_eq!(detail["gate"]["reason"], "deny_rule");
        assert_eq!(detail["gate"]["matched_rule_id"], deny.id);
    }
    // Removing only the deny restores review. An allow rule/act_low_risk scope
    // still cannot bypass the accepted email action's destructive floor.
    rule::Entity::delete_by_id(deny.id)
        .exec(&engine.db.conn)
        .await
        .unwrap();
    let (listener, token) = previous.unwrap();
    let d = draft.unwrap();
    let pending = succeeded(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeReply,
            json!({"draftId":d["id"], "expectedRevision":d["revision"]}),
        )
        .await,
    );
    assert_eq!(pending["status"], "pending");
    let row = ops_proposal::Entity::find_by_id(pending["proposalId"].as_i64().unwrap() as i32)
        .one(&engine.db.conn)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.agent_id, "pi");
    assert_eq!(row.status, "pending");
    assert_eq!(
        serde_json::from_str::<Value>(&row.payload_json).unwrap()["reply"],
        d["reply"]
    );
}

#[tokio::test]
async fn desk_bridge_public_scope_closed_inputs_and_exact_draft_persistence() {
    let (engine, _) = running_task().await;
    let account = agent::account_id().unwrap();
    let (scope, conversation) = seed(&engine, account).await;
    let foreign_account = if account == 1 { 2 } else { 1 };
    let (foreign, other) = seed(&engine, foreign_account).await;
    tickets::add_private_note(
        &engine.db.conn,
        scope,
        conversation,
        "operator:http",
        "private sentinel",
    )
    .await
    .unwrap();
    let (listener, _, token) = bridge(&engine, PARENT_CONN).await;
    let k = key(scope, conversation);
    let thread = succeeded(call(&listener, &token, DeskTool::DeskThread, k.clone()).await);
    assert_eq!(thread["messages"].as_array().unwrap().len(), 1);
    assert!(!thread.to_string().contains("private sentinel"));
    let list = succeeded(
        call(
            &listener,
            &token,
            DeskTool::DeskTickets,
            json!({"inboxId":scope.inbox_id}),
        )
        .await,
    );
    assert_eq!(list["items"].as_array().unwrap().len(), 1);
    for (tool, input) in [
        (DeskTool::DeskThread, key(foreign, other)),
        (DeskTool::DeskTickets, json!({"inboxId":foreign.inbox_id})),
    ] {
        assert_eq!(
            call(&listener, &token, tool, input).await.code,
            Some(DeskError::Denied)
        );
    }
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskContext,
            json!({"accountId":foreign_account})
        )
        .await
        .code,
        Some(DeskError::InvalidInput)
    );
    let mut injected = k.clone();
    injected["agentId"] = "operator:http".into();
    assert_eq!(
        call(&listener, &token, DeskTool::DeskThread, injected)
            .await
            .code,
        Some(DeskError::InvalidInput)
    );
    let mut reply = thread["suggestedReply"].clone();
    reply["text"] = "Original draft".into();
    let first = succeeded(save(&listener, &token, &k, 0, reply.clone()).await);
    assert_eq!(first["revision"], 1);
    let mut bad = reply.clone();
    bad["from"] = "support@example.com\r\nBcc: injected@example.com".into();
    assert_eq!(
        save(&listener, &token, &k, 1, bad).await.code,
        Some(DeskError::InvalidInput)
    );
    reply["text"] = "Edited exact payload".into();
    assert_eq!(
        save(&listener, &token, &k, 0, reply.clone()).await.code,
        Some(DeskError::Stale)
    );
    let edited = succeeded(save(&listener, &token, &k, 1, reply.clone()).await);
    let reloaded = succeeded(call(&listener, &token, DeskTool::DeskThread, k).await);
    assert_eq!(reloaded["draft"], edited);
    assert_eq!(edited["reply"], reply);
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeReply,
            json!({"draftId": first["id"], "expectedRevision":1})
        )
        .await
        .code,
        Some(DeskError::Stale)
    );
    let pending = succeeded(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeReply,
            json!({"draftId":edited["id"], "expectedRevision":2}),
        )
        .await,
    );
    let row = ops_proposal::Entity::find_by_id(pending["proposalId"].as_i64().unwrap() as i32)
        .one(&engine.db.conn)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&row.payload_json).unwrap()["reply"],
        reply
    );
}

#[tokio::test]
async fn desk_bridge_cancellation_writer_wins_before_draft_save() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/pi-desk-fixtures");
    std::fs::create_dir_all(&root).unwrap();
    let dir = tempfile::tempdir_in(root).unwrap();
    let db = crate::db::init_database(dir.path(), "desk-cancel")
        .await
        .unwrap();
    let (engine, task) = running_task_in(db).await;
    let (scope, conversation) = seed(&engine, agent::account_id().unwrap()).await;
    let (listener, _, token) = bridge(&engine, PARENT_CONN).await;
    let k = key(scope, conversation);
    let thread = succeeded(call(&listener, &token, DeskTool::DeskThread, k.clone()).await);
    let original =
        succeeded(save(&listener, &token, &k, 0, thread["suggestedReply"].clone()).await);
    let writer = engine.db.conn.begin().await.unwrap();
    writer
        .execute(Statement::from_sql_and_values(
            writer.get_database_backend(),
            "UPDATE work_task SET status = 'canceled', connection_id = NULL WHERE id = ?",
            [task.into()],
        ))
        .await
        .unwrap();
    let mut reply = original["reply"].clone();
    reply["text"] = "Canceled overwrite".into();
    let (result, ()) = tokio::join!(save(&listener, &token, &k, 1, reply), async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        writer.commit().await.unwrap();
    });
    assert_eq!(result.code, Some(DeskError::Stale));
    assert_persisted(&engine, &original).await;
    assert_eq!(
        call(&listener, &token, DeskTool::DeskContext, json!({}))
            .await
            .code,
        Some(DeskError::Stale)
    );
}

#[tokio::test]
async fn desk_bridge_peer_abort_releases_token_and_drops_queued_draft_transaction() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/pi-desk-fixtures");
    std::fs::create_dir_all(&root).unwrap();
    let dir = tempfile::tempdir_in(root).unwrap();
    let db = crate::db::init_database(dir.path(), "desk-abort")
        .await
        .unwrap();
    let (engine, _) = running_task_in(db).await;
    let (scope, conversation) = seed(&engine, agent::account_id().unwrap()).await;
    let (listener, access, token) = bridge(&engine, PARENT_CONN).await;
    let k = key(scope, conversation);
    let thread = succeeded(call(&listener, &token, DeskTool::DeskThread, k.clone()).await);
    let original =
        succeeded(save(&listener, &token, &k, 0, thread["suggestedReply"].clone()).await);
    access.entered.notified().await; // Consume the prior completed calls' permit.
    let writer = engine.db.conn.begin().await.unwrap();
    writer
        .execute(Statement::from_sql_and_values(
            writer.get_database_backend(),
            "UPDATE ops_ticket_inbox SET updated_at = updated_at WHERE id = ?",
            [scope.inbox_id.into()],
        ))
        .await
        .unwrap();
    let mut reply = original["reply"].clone();
    reply["text"] = "Aborted overwrite".into();
    let (client, served) = start_call(
        listener.clone(),
        &token,
        DeskTool::DeskSaveReply,
        save_input(&k, 1, reply),
    )
    .await;
    tokio::time::timeout(Duration::from_secs(2), access.entered.notified())
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(
        !served.is_finished(),
        "writer holds the real draft transaction"
    );
    drop(client);
    tokio::time::timeout(Duration::from_secs(2), served)
        .await
        .unwrap()
        .unwrap();
    tokio::time::timeout(Duration::from_secs(2), listener.tokens.revoke(&token))
        .await
        .unwrap();
    writer.commit().await.unwrap();
    assert_persisted(&engine, &original).await;
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskSaveReply,
            save_input(&k, 1, original["reply"].clone())
        )
        .await
        .code,
        Some(DeskError::Denied)
    );
}

async fn assert_persisted(engine: &TaskEngine, original: &Value) {
    let row = engine
        .db
        .conn
        .query_one(Statement::from_sql_and_values(
            engine.db.conn.get_database_backend(),
            "SELECT revision, reply_json FROM ops_reply_draft WHERE id = ?",
            [original["id"].as_i64().unwrap().into()],
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        row.try_get::<i32>("", "revision").unwrap(),
        original["revision"].as_i64().unwrap() as i32
    );
    assert_eq!(
        serde_json::from_str::<Value>(&row.try_get::<String>("", "reply_json").unwrap()).unwrap(),
        original["reply"]
    );
}

/// Reuses the accepted Ops browser fixture's AppState/protected-router setup.
/// No normal server startup, global skill installation, task or model prompt.
#[tokio::test]
#[ignore = "manual isolated Playwright CLI fixture on 127.0.0.1:4324"]
async fn pi_desk_browser_fixture() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/pi-desk-browser");
    std::fs::create_dir_all(&root).unwrap();
    let stop = root.join("stop");
    assert!(!stop.exists(), "remove the previous fixture stop file first");
    let catalogue = std::path::PathBuf::from(
        std::env::var("PI_CODING_AGENT_DIR").expect("an isolated empty Pi catalogue is required"),
    );
    assert!(catalogue.starts_with(&root));
    assert!(!catalogue.join("models.json").exists());
    assert!(!catalogue.join("auth.json").exists());
    let dir = tempfile::tempdir_in(&root).unwrap();
    let workspace = dir.path().join("fresh-desk-workspace");
    std::fs::create_dir_all(&workspace).unwrap();
    let db = crate::db::init_database(dir.path(), "pi-desk-browser")
        .await
        .unwrap();
    let folder = crate::db::test_helpers::seed_folder(&db, workspace.to_str().unwrap()).await;
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let router = crate::web::router::build_router(
        state,
        "pi-desk-browser-fixture".into(), // Public, synthetic test-only token.
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("out"),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4324")
        .await
        .unwrap();
    println!("Pi Desk browser fixture ready on 127.0.0.1:4324; fresh folder {folder}");
    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            while !stop.exists() {
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        })
        .await
        .unwrap();
}
