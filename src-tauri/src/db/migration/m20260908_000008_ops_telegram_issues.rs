//! Extend the closed notice family without changing durable delivery claims.
use sea_orm::{DatabaseTransaction, Statement, TransactionTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

async fn rebuild(txn: &DatabaseTransaction, issues: bool) -> Result<(), DbErr> {
    let kinds = if issues {
        "action_kind IN ('email_reply','github_issue')"
    } else {
        "action_kind = 'email_reply'"
    };
    txn.execute_unprepared(&format!(
        "CREATE TABLE ops_telegram_notice_next (
            id TEXT PRIMARY KEY, account_id INTEGER NOT NULL,
            proposal_id INTEGER NOT NULL UNIQUE, task_id INTEGER NOT NULL, run_seq INTEGER NOT NULL,
            action_kind TEXT NOT NULL CHECK({kinds}),
            snapshot_sha256 TEXT NOT NULL, config_revision TEXT NOT NULL,
            channel_id INTEGER NOT NULL, private_user_id TEXT NOT NULL,
            channel_sha256 TEXT NOT NULL, claim_id TEXT NOT NULL,
            status TEXT NOT NULL CHECK(status IN ('checking','preflight_failed','sending','sent','failed','unknown','obsolete')),
            provider_message_id TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
            FOREIGN KEY(proposal_id) REFERENCES ops_proposal(id)
        );
        INSERT INTO ops_telegram_notice_next
            (id,account_id,proposal_id,task_id,run_seq,action_kind,snapshot_sha256,
             config_revision,channel_id,private_user_id,channel_sha256,claim_id,
             status,provider_message_id,created_at,updated_at)
        SELECT id,account_id,proposal_id,task_id,run_seq,action_kind,snapshot_sha256,
             config_revision,channel_id,private_user_id,channel_sha256,claim_id,
             status,provider_message_id,created_at,updated_at FROM ops_telegram_notice;
        DROP TABLE ops_telegram_notice;
        ALTER TABLE ops_telegram_notice_next RENAME TO ops_telegram_notice;
        CREATE INDEX idx_ops_telegram_notice_scope ON ops_telegram_notice(account_id,created_at);"
    ))
    .await?;
    Ok(())
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SeaORM's SQLite migrator does not wrap DDL in a transaction.
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(
            "ALTER TABLE ops_telegram_config ADD COLUMN github_issues_enabled BOOLEAN NOT NULL DEFAULT 0 CHECK(github_issues_enabled IN (0,1))",
        ).await?;
        // A rebind then restore must not revive an old phone locator. Keep the
        // epoch separate from the accepted host schema, including tombstones.
        txn.execute_unprepared(
            "CREATE TABLE ops_telegram_issue_binding (product_id TEXT PRIMARY KEY, revision INTEGER NOT NULL);
             CREATE TRIGGER ops_telegram_issue_insert AFTER INSERT ON ops_intake_host_product BEGIN
               INSERT INTO ops_telegram_issue_binding VALUES(NEW.product_id,1)
               ON CONFLICT(product_id) DO UPDATE SET revision=revision+1; END;
             CREATE TRIGGER ops_telegram_issue_update AFTER UPDATE ON ops_intake_host_product BEGIN
               INSERT INTO ops_telegram_issue_binding VALUES(NEW.product_id,1)
               ON CONFLICT(product_id) DO UPDATE SET revision=revision+1; END;
             CREATE TRIGGER ops_telegram_issue_delete AFTER DELETE ON ops_intake_host_product BEGIN
               INSERT INTO ops_telegram_issue_binding VALUES(OLD.product_id,1)
               ON CONFLICT(product_id) DO UPDATE SET revision=revision+1; END;",
        ).await?;
        rebuild(&txn, true).await?;
        txn.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        // Lock before checking so a concurrent issue claim cannot race rollback.
        txn.execute_unprepared("UPDATE ops_telegram_config SET updated_at=updated_at")
            .await?;
        if txn
            .query_one(Statement::from_string(
                txn.get_database_backend(),
                "SELECT id FROM ops_telegram_notice WHERE action_kind='github_issue' LIMIT 1"
                    .to_owned(),
            ))
            .await?
            .is_some()
        {
            return Err(DbErr::Custom(
                "Cannot roll back while GitHub issue notices exist; delivery records must be retained".into(),
            ));
        }
        rebuild(&txn, false).await?;
        txn.execute_unprepared(
            "DROP TRIGGER ops_telegram_issue_insert; DROP TRIGGER ops_telegram_issue_update;
             DROP TRIGGER ops_telegram_issue_delete; DROP TABLE ops_telegram_issue_binding",
        )
        .await?;
        txn.execute_unprepared("ALTER TABLE ops_telegram_config DROP COLUMN github_issues_enabled")
            .await?;
        txn.commit().await
    }
}
