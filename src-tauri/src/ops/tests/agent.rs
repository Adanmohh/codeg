use super::*;
use crate::ops::agent::{self, RunContext};
use sea_orm::{Statement, TransactionTrait};
use std::time::Duration;

fn context(task_id: i32, run_seq: i32) -> RunContext {
    RunContext {
        account_id: 1,
        task_id,
        run_seq,
        connection_id: "ops-ui-fixture".into(),
        agent_id: "fixture-pi".into(),
    }
}
#[tokio::test]
async fn trusted_agent_projection_omits_notes_and_checks_scope_run_and_connection() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let (_, foreign) = seed(&db, 2).await;
    store::add_note(
        &db.conn,
        &op,
        NoteInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            content: "private sentinel".into(),
        },
    )
    .await
    .unwrap();
    let (task, seq) = start(&db).await;
    let mut ctx = context(task, seq);
    let info = serde_json::to_value(agent::context(&db.conn, &ctx).await.unwrap()).unwrap();
    assert_eq!(
        info,
        json!({"accountId":1,"inboxes":[{"id":key.inbox_id,"name":"Support","email":"support@example.com"}]})
    );
    let public = agent::thread(&db.conn, &ctx, key).await.unwrap();
    assert_eq!(public.messages.len(), 1);
    assert!(!serde_json::to_string(&public)
        .unwrap()
        .contains("private sentinel"));
    assert!(public.messages.iter().all(|m| !m.private));
    assert_eq!(
        store::thread(&db.conn, &op, key)
            .await
            .unwrap()
            .messages
            .len(),
        2
    );
    assert!(agent::thread(&db.conn, &ctx, foreign).await.is_err());
    let draft = agent::save_draft(
        &db.conn,
        &ctx,
        SaveDraftInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            expected_revision: 0,
            reply: public.suggested_reply,
        },
    )
    .await
    .unwrap();
    let stored = crate::ops::draft_entity::Entity::find_by_id(draft.id)
        .one(&db.conn)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(stored.updated_by, "agent:fixture-pi");
    ctx.connection_id = "another-parent".into();
    assert!(agent::context(&db.conn, &ctx).await.is_err());
    assert!(agent::thread(&db.conn, &ctx, key).await.is_err());
    ctx.connection_id = "ops-ui-fixture".into();
    ctx.run_seq += 1;
    assert!(agent::thread(&db.conn, &ctx, key).await.is_err());
    ctx.run_seq = seq;
    assert!(tasks::cancel_running_generation(&db.conn, task, seq)
        .await
        .unwrap());
    assert!(agent::context(&db.conn, &ctx).await.is_err());
    assert!(agent::tickets(
        &db.conn,
        &ctx,
        TicketsInput {
            inbox_id: key.inbox_id,
            status: None,
            page: 0
        }
    )
    .await
    .is_err());
}

#[test]
fn shared_host_account_configuration_and_bridge_errors_fail_closed() {
    use std::ffi::OsStr;
    assert_eq!(crate::ops::account_id_from(None).unwrap(), 1);
    assert_eq!(
        crate::ops::account_id_from(Some(OsStr::new("27"))).unwrap(),
        27
    );
    for invalid in ["", "0", "-1", "1.5", "foreign"] {
        assert!(crate::ops::account_id_from(Some(OsStr::new(invalid))).is_err());
    }
    let error = agent::command_error(DbError::Conflict("private sql payload".into()));
    assert!(!serde_json::to_string(&error)
        .unwrap()
        .contains("private sql payload"));
}

#[tokio::test]
async fn agent_save_waits_for_writer_then_rechecks_run_in_its_draft_transaction() {
    // A separate precheck would see the old committed running task while the
    // competing writer is held, then incorrectly save after that writer commits.
    let dir = tempfile::tempdir().unwrap();
    let db = db::init_database(dir.path(), "ops-agent-race")
        .await
        .unwrap();
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let (task, seq) = start(&db).await;
    let ctx = context(task, seq);
    let writer = db.conn.begin().await.unwrap();
    writer.execute(Statement::from_sql_and_values(writer.get_database_backend(),"UPDATE work_task SET status = 'canceled', connection_id = NULL WHERE id = ? AND run_seq = ?",[task.into(),seq.into()])).await.unwrap();
    let mut reply = draft.reply.clone();
    reply.text = "stale agent overwrite".into();
    let (save, ()) = tokio::join!(
        agent::save_draft(
            &db.conn,
            &ctx,
            SaveDraftInput {
                inbox_id: key.inbox_id,
                conversation_id: key.conversation_id,
                expected_revision: 1,
                reply
            }
        ),
        async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            writer.commit().await.unwrap();
        }
    );
    assert!(matches!(save, Err(DbError::Conflict(_))));
    let current = store::thread(&db.conn, &op, key)
        .await
        .unwrap()
        .draft
        .unwrap();
    assert_eq!(current.revision, 1);
    assert_eq!(current.reply, draft.reply);
}

#[tokio::test]
async fn agent_and_operator_draft_writers_share_one_revision_cas() {
    let dir = tempfile::tempdir().unwrap();
    let db = db::init_database(dir.path(), "ops-draft-race")
        .await
        .unwrap();
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let (task, seq) = start(&db).await;
    let ctx = context(task, seq);
    let input = |text: &str| {
        let mut reply = draft.reply.clone();
        reply.text = text.into();
        SaveDraftInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            expected_revision: 1,
            reply,
        }
    };
    let (a, b) = tokio::join!(
        agent::save_draft(&db.conn, &ctx, input("agent edit")),
        store::save_draft(&db.conn, &op, input("operator edit"))
    );
    assert!(a.is_ok() ^ b.is_ok());
    let winning = a.or(b).unwrap();
    let current = store::thread(&db.conn, &op, key)
        .await
        .unwrap()
        .draft
        .unwrap();
    assert_eq!(current.revision, 2);
    assert_eq!(current.reply, winning.reply);
}

#[tokio::test]
async fn private_notes_do_not_change_agent_thread_or_list_metadata() {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let (task, seq) = start(&db).await;
    let ctx = context(task, seq);
    let input = || TicketsInput {
        inbox_id: key.inbox_id,
        status: None,
        page: 0,
    };
    let before_thread =
        serde_json::to_value(agent::thread(&db.conn, &ctx, key).await.unwrap()).unwrap();
    let before_list =
        serde_json::to_value(agent::tickets(&db.conn, &ctx, input()).await.unwrap()).unwrap();
    store::add_note(
        &db.conn,
        &op,
        NoteInput {
            inbox_id: key.inbox_id,
            conversation_id: key.conversation_id,
            content: "hidden note".into(),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        serde_json::to_value(agent::thread(&db.conn, &ctx, key).await.unwrap()).unwrap(),
        before_thread
    );
    assert_eq!(
        serde_json::to_value(agent::tickets(&db.conn, &ctx, input()).await.unwrap()).unwrap(),
        before_list
    );
}
