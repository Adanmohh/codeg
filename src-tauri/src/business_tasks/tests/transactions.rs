use super::*;
use crate::db::{migration::Migrator, test_helpers::fresh_disk_db};
use sea_orm::Database;
use sea_orm_migration::{MigratorTrait, SchemaManager};
use std::{sync::Arc, time::Duration};

#[tokio::test]
async fn independent_sqlite_connections_race_one_revision_and_one_activity() {
    let dir = tempfile::tempdir().unwrap();
    let db = fresh_disk_db(dir.path()).await;
    let other = Database::connect(format!("sqlite:{}?mode=rw", dir.path().join("source.db").display())).await.unwrap();
    let (_, ctx, task) = create_owned(&db).await;
    let mut a = update_input(&task.task); a.title = "First writer".into();
    let mut b = update_input(&task.task); b.title = "Second writer".into();
    let (a, b) = tokio::join!(store::update(&db.conn, &ctx, a), store::update(&other, &ctx, b));
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    assert!(matches!(a, Err(E::Conflict)) || matches!(b, Err(E::Conflict)));
    let current = store::get(&db.conn, &ctx, TaskInput { task_id: task.task.id }).await.unwrap();
    assert_eq!(current.task.revision, 2); assert_eq!(current.activity.len(), 2);
    assert!(matches!(current.task.title.as_str(), "First writer" | "Second writer"));
}

#[tokio::test]
async fn task_writer_rechecks_original_credential_after_waiting_for_revocation() {
    let dir = tempfile::tempdir().unwrap();
    let db = fresh_disk_db(dir.path()).await;
    let other = Database::connect(format!("sqlite:{}?mode=rw", dir.path().join("source.db").display())).await.unwrap();
    let op = initialize(&db.conn).await;
    let (_, issued, cached) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    let task = store::create(&db.conn, &cached, create_input()).await.unwrap();
    let revocation = identity::begin_write(&db.conn, op.organization_id()).await.unwrap();
    revocation.execute(store::statement("UPDATE business_credential SET revoked_at = ? WHERE id = ?", vec!["2026-09-08T00:00:00Z".into(), issued.credential.id.into()])).await.unwrap();
    let started = Arc::new(tokio::sync::Notify::new());
    let entered = started.clone();
    let task_id = task.task.id.clone();
    let mut mutation = tokio::spawn(async move {
        entered.notify_one();
        store::note(&other, &cached, TextInput { task_id, expected_revision: 1, body: "Must not commit".into() }).await
    });
    started.notified().await;
    assert!(tokio::time::timeout(Duration::from_millis(50), &mut mutation).await.is_err());
    revocation.commit().await.unwrap();
    assert!(matches!(mutation.await.unwrap(), Err(E::Unauthorized)));
    let current = store::get(&db.conn, &ActorContext::authenticated(op), TaskInput { task_id: task.task.id }).await.unwrap();
    assert_eq!(current.task.revision, 1); assert_eq!(current.activity.len(), 1);
}

#[tokio::test]
async fn activity_failure_rolls_back_deliverable_and_revision_and_history_cannot_be_rewritten() {
    let db = fresh_in_memory_db().await;
    let (_, ctx, task) = create_owned(&db).await;
    db.conn.execute_unprepared("CREATE TRIGGER fixture_task_audit_failure BEFORE INSERT ON business_task_activity WHEN NEW.kind = 'submitted' BEGIN SELECT RAISE(ABORT, 'fixture audit unavailable'); END;").await.unwrap();
    let input = TextInput { task_id: task.task.id.clone(), expected_revision: 1, body: "Exact proposed work".into() };
    assert!(matches!(store::submit(&db.conn, &ctx, input.clone()).await, Err(E::Database(_))));
    assert_eq!(count(&db.conn, "SELECT COUNT(*) AS count FROM business_task_deliverable").await, 0);
    let unchanged = store::get(&db.conn, &ctx, TaskInput { task_id: task.task.id.clone() }).await.unwrap();
    assert_eq!(unchanged.task.revision, 1); assert_eq!(unchanged.task.status, TaskStatus::Todo);
    db.conn.execute_unprepared("DROP TRIGGER fixture_task_audit_failure").await.unwrap();
    let saved = store::submit(&db.conn, &ctx, input).await.unwrap();
    assert_eq!(saved.task.revision, 2);
    for sql in ["UPDATE business_task_activity SET actor_id = 'forged'", "DELETE FROM business_task_activity", "UPDATE business_task_deliverable SET body = 'forged'", "DELETE FROM business_task_deliverable", "DELETE FROM business_task"] {
        assert!(db.conn.execute_unprepared(sql).await.is_err());
    }
    assert_eq!(store::get(&db.conn, &ctx, TaskInput { task_id: task.task.id }).await.unwrap().deliverables[0].body, "Exact proposed work");
}

#[tokio::test]
async fn task_migration_is_atomic_scoped_and_refuses_to_drop_durable_history() {
    let db = fresh_in_memory_db().await;
    let migration = Migrator::migrations().into_iter().find(|m| m.name() == "m20260908_000010_business_tasks").unwrap();
    let manager = SchemaManager::new(&db.conn);
    migration.down(&manager).await.unwrap();
    db.conn.execute_unprepared("CREATE TABLE business_task_execution (id INTEGER)").await.unwrap();
    assert!(migration.up(&manager).await.is_err());
    assert!(!manager.has_table("business_task").await.unwrap());
    assert!(!manager.has_table("business_task_activity").await.unwrap());
    assert!(manager.has_table("business_member").await.unwrap());
    assert!(manager.has_table("work_task").await.unwrap());
    db.conn.execute_unprepared("DROP TABLE business_task_execution").await.unwrap();
    migration.up(&manager).await.unwrap();
    let (op, ctx, task) = create_owned(&db).await;
    assert!(migration.down(&manager).await.is_err());
    assert!(db.conn.execute(store::statement("INSERT INTO business_task (id,organization_id,title,domain,owner_id,creator_id,created_at,updated_at) VALUES (?, ?, 'forged', 'feedback', ?, ?, 'now', 'now')", vec![uuid::Uuid::new_v4().to_string().into(), "foreign-org".into(), op.member_id().into(), op.member_id().into()])).await.is_err());
    assert_eq!(store::get(&db.conn, &ctx, TaskInput { task_id: task.task.id }).await.unwrap().activity.len(), 1);
}
