//! Durable email adapter glue; atomic SQLite DDL following Codeg patterns.
use sea_orm::TransactionTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(
            "CREATE TABLE ops_email_config (
                inbox_id INTEGER PRIMARY KEY, account_id INTEGER NOT NULL,
                credential_ref TEXT NOT NULL UNIQUE,
                last_pull_at TEXT, last_pull_status TEXT, last_pull_error TEXT,
                FOREIGN KEY(inbox_id, account_id) REFERENCES ops_ticket_inbox(id, account_id)
            );
            CREATE TABLE ops_email_attempt (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id INTEGER NOT NULL, inbox_id INTEGER NOT NULL, conversation_id INTEGER NOT NULL,
                proposal_id INTEGER NOT NULL UNIQUE, task_id INTEGER NOT NULL, run_seq INTEGER NOT NULL,
                draft_id INTEGER NOT NULL, draft_revision INTEGER NOT NULL,
                payload_json TEXT NOT NULL CHECK(json_valid(payload_json)), payload_sha256 TEXT NOT NULL,
                message_id TEXT NOT NULL UNIQUE, idempotency_key TEXT NOT NULL UNIQUE,
                actor TEXT NOT NULL,
                status TEXT NOT NULL CHECK(status IN ('reserved','sending','not_sent','failed','unknown','receipt_recorded','sent')),
                provider_id TEXT UNIQUE, error TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
                UNIQUE(draft_id, draft_revision),
                FOREIGN KEY(proposal_id) REFERENCES ops_proposal(id),
                FOREIGN KEY(draft_id) REFERENCES ops_reply_draft(id),
                FOREIGN KEY(conversation_id, account_id, inbox_id) REFERENCES ops_ticket_conversation(id, account_id, inbox_id)
            );
            CREATE UNIQUE INDEX idx_ops_email_unresolved_thread ON ops_email_attempt(account_id, inbox_id, conversation_id)
                WHERE status IN ('reserved','sending','unknown','receipt_recorded');
            CREATE INDEX idx_ops_email_attempt_scope ON ops_email_attempt(account_id, inbox_id, id);",
        ).await?;
        txn.commit().await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared("DROP TABLE ops_email_attempt; DROP TABLE ops_email_config")
            .await?;
        txn.commit().await
    }
}
