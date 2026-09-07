//! Optional, default-off Ops notification binding and durable single attempts.
use sea_orm::TransactionTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(
            "CREATE TABLE ops_telegram_config (
                account_id INTEGER PRIMARY KEY CHECK(account_id > 0),
                enabled BOOLEAN NOT NULL DEFAULT 0,
                channel_id INTEGER NOT NULL, private_user_id TEXT NOT NULL,
                channel_sha256 TEXT NOT NULL, review_origin TEXT NOT NULL,
                revision TEXT NOT NULL, updated_by TEXT NOT NULL, updated_at TEXT NOT NULL,
                FOREIGN KEY(channel_id) REFERENCES chat_channel(id) ON DELETE CASCADE
            );
            CREATE TABLE ops_telegram_notice (
                id TEXT PRIMARY KEY, account_id INTEGER NOT NULL,
                proposal_id INTEGER NOT NULL UNIQUE, task_id INTEGER NOT NULL, run_seq INTEGER NOT NULL,
                action_kind TEXT NOT NULL CHECK(action_kind = 'email_reply'),
                snapshot_sha256 TEXT NOT NULL, config_revision TEXT NOT NULL,
                channel_id INTEGER NOT NULL, private_user_id TEXT NOT NULL,
                channel_sha256 TEXT NOT NULL, claim_id TEXT NOT NULL,
                status TEXT NOT NULL CHECK(status IN ('checking','preflight_failed','sending','sent','failed','unknown','obsolete')),
                provider_message_id TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
                FOREIGN KEY(proposal_id) REFERENCES ops_proposal(id)
            );
            CREATE INDEX idx_ops_telegram_notice_scope ON ops_telegram_notice(account_id, created_at);",
        ).await?;
        txn.commit().await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared("DROP TABLE ops_telegram_notice; DROP TABLE ops_telegram_config")
            .await?;
        txn.commit().await
    }
}
