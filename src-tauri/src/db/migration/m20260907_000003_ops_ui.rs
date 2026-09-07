//! Draft CAS storage; Codeg SeaORM/SQLite patterns, see NOTICE.
use sea_orm::TransactionTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(
            "CREATE TABLE ops_reply_draft (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id INTEGER NOT NULL, inbox_id INTEGER NOT NULL,
                conversation_id INTEGER NOT NULL,
                revision INTEGER NOT NULL CHECK(revision > 0),
                reply_json TEXT NOT NULL CHECK(json_valid(reply_json)),
                updated_by TEXT NOT NULL, updated_at TEXT NOT NULL,
                UNIQUE(account_id, inbox_id, conversation_id),
                FOREIGN KEY(conversation_id, account_id, inbox_id)
                    REFERENCES ops_ticket_conversation(id, account_id, inbox_id)
            );
            CREATE INDEX idx_ops_reply_draft_scope ON ops_reply_draft(account_id, id);",
        )
        .await?;
        txn.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE ops_reply_draft")
            .await?;
        Ok(())
    }
}
