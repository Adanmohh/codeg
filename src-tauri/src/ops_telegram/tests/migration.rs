//! Upgrade real proposal-bound synthetic email rows; never contact a provider.
use super::*;
use sea_orm_migration::MigrationTrait;

fn migration() -> Box<dyn MigrationTrait> {
    db::migration::Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260908_000008_ops_telegram_issues")
        .expect("named forward migration must be registered")
}

async fn rows(db: &AppDatabase) -> Vec<notice::Model> {
    notice::Entity::find()
        .order_by_asc(notice::Column::ProposalId)
        .all(&db.conn)
        .await
        .unwrap()
}

async fn seeded() -> (AppDatabase, config::Model, Vec<notice::Model>) {
    let db = fresh_in_memory_db().await;
    let (op, key) = seed(&db, 1).await;
    let draft = saved(&db, &op, key).await;
    let provider = Provider::new().await;
    settings(&db, &op, &provider.runtime, true).await;
    let cfg = config::Entity::find_by_id(op.account_id())
        .one(&db.conn)
        .await
        .unwrap()
        .unwrap();
    for state in ["sent", "unknown", "checking", "preflight_failed"] {
        let proposal = pending(&db, &op, &draft).await;
        let claimed = claim(
            &db.conn,
            &cfg,
            proposal.id,
            &uuid::Uuid::new_v4().to_string(),
        )
        .await
        .unwrap()
        .unwrap();
        let mut stored = claimed.into_active_model();
        stored.status = Set(state.into());
        stored.provider_message_id = Set((state == "sent").then(|| "456".into()));
        stored.update(&db.conn).await.unwrap();
    }
    assert_eq!(provider.sends(), 0);
    let notices = rows(&db).await;
    (db, cfg, notices)
}

#[tokio::test]
async fn issue_upgrade_preserves_email_rows_and_defaults_off() {
    let migration = migration();
    let (db, cfg, before) = seeded().await;
    let manager = SchemaManager::new(&db.conn);
    migration.down(&manager).await.unwrap();
    assert!(!manager
        .has_column("ops_telegram_config", "github_issues_enabled")
        .await
        .unwrap());
    migration.up(&manager).await.unwrap();
    assert_eq!(rows(&db).await, before);
    assert_eq!(
        config::Entity::find_by_id(cfg.account_id)
            .one(&db.conn)
            .await
            .unwrap(),
        Some(cfg)
    );
    let row = db
        .conn
        .query_one(Statement::from_string(
            db.conn.get_database_backend(),
            "SELECT github_issues_enabled FROM ops_telegram_config".to_owned(),
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<i32>("", "github_issues_enabled").unwrap(), 0);
    for table in [
        "ops_proposal",
        "ops_reply_draft",
        "ops_intake_filing",
        "work_task",
    ] {
        assert!(manager.has_table(table).await.unwrap());
    }
    // Proposal uniqueness survives the table rebuild.
    let mut duplicate = before[0].clone().into_active_model().reset_all();
    duplicate.id = Set(uuid::Uuid::new_v4().to_string());
    assert!(duplicate.insert(&db.conn).await.is_err());
}

#[tokio::test]
async fn issue_upgrade_mid_ddl_failure_restores_original_rows_and_schema() {
    let migration = migration();
    let (db, _, before) = seeded().await;
    let manager = SchemaManager::new(&db.conn);
    migration.down(&manager).await.unwrap();
    // Force the last CREATE INDEX to fail AFTER copying, dropping and renaming
    // the notice table. The surrounding transaction must restore every row.
    db.conn
        .execute_unprepared(
            "DROP INDEX idx_ops_telegram_notice_scope;
         CREATE TABLE ops_telegram_migration_collision (id INTEGER);
         CREATE INDEX idx_ops_telegram_notice_scope ON ops_telegram_migration_collision(id)",
        )
        .await
        .unwrap();
    assert!(migration.up(&manager).await.is_err());
    assert!(!manager
        .has_column("ops_telegram_config", "github_issues_enabled")
        .await
        .unwrap());
    assert_eq!(rows(&db).await, before);
    assert!(!manager.has_table("ops_telegram_notice_next").await.unwrap());
    db.conn
        .execute_unprepared("DROP TABLE ops_telegram_migration_collision")
        .await
        .unwrap();
    migration.up(&manager).await.unwrap();
    assert_eq!(rows(&db).await, before);
}

#[tokio::test]
async fn issue_schema_is_closed_and_lossy_rollback_is_rejected() {
    let migration = migration();
    let (db, _, _) = seeded().await;
    let manager = SchemaManager::new(&db.conn);
    assert!(db
        .conn
        .execute_unprepared("UPDATE ops_telegram_notice SET action_kind = 'arbitrary_executor'")
        .await
        .is_err());
    db.conn
        .execute_unprepared(
            "UPDATE ops_telegram_notice SET action_kind = 'github_issue' WHERE status = 'unknown'",
        )
        .await
        .unwrap();
    let before = rows(&db).await;
    assert!(migration.down(&manager).await.is_err());
    assert_eq!(rows(&db).await, before);
    assert!(manager
        .has_column("ops_telegram_config", "github_issues_enabled")
        .await
        .unwrap());
}
