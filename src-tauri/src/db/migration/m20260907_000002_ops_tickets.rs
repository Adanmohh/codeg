//! MIT transcription of Chatwoot v4.17.1 core ticket schema (see NOTICE).
//! SQLite composite foreign keys additionally enforce account/inbox isolation.
use sea_orm::TransactionTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SeaORM 1.1.19 does not wrap SQLite migrations in a transaction.
        // Keep all five tables atomic if any later DDL statement fails.
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(r#"
CREATE TABLE ops_ticket_inbox (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL CHECK(account_id > 0),
    name TEXT NOT NULL,
    email_address TEXT NOT NULL COLLATE NOCASE,
    channel_type TEXT NOT NULL DEFAULT 'Channel::Email',
    enable_auto_assignment BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
    UNIQUE(id, account_id), UNIQUE(account_id, email_address)
);
CREATE TABLE ops_ticket_contact (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL CHECK(account_id > 0),
    name TEXT NOT NULL DEFAULT '', email TEXT NOT NULL COLLATE NOCASE,
    phone_number TEXT, identifier TEXT,
    contact_type INTEGER NOT NULL DEFAULT 0 CHECK(contact_type BETWEEN 0 AND 2),
    blocked BOOLEAN NOT NULL DEFAULT FALSE,
    additional_attributes TEXT NOT NULL DEFAULT '{}',
    custom_attributes TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
    UNIQUE(id, account_id), UNIQUE(account_id, email), UNIQUE(account_id, identifier)
);
CREATE TABLE ops_ticket_contact_inbox (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL, contact_id INTEGER NOT NULL, inbox_id INTEGER NOT NULL,
    source_id TEXT NOT NULL,
    created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
    UNIQUE(inbox_id, source_id), UNIQUE(inbox_id, contact_id),
    UNIQUE(id, account_id, inbox_id, contact_id),
    FOREIGN KEY(contact_id, account_id) REFERENCES ops_ticket_contact(id, account_id),
    FOREIGN KEY(inbox_id, account_id) REFERENCES ops_ticket_inbox(id, account_id)
);
CREATE TABLE ops_ticket_conversation (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL, inbox_id INTEGER NOT NULL,
    contact_id INTEGER NOT NULL, contact_inbox_id INTEGER NOT NULL,
    uuid TEXT NOT NULL COLLATE NOCASE UNIQUE,
    status INTEGER NOT NULL DEFAULT 0 CHECK(status BETWEEN 0 AND 3),
    priority INTEGER CHECK(priority BETWEEN 0 AND 3),
    assignee_id TEXT, assignee_agent_bot_id TEXT, team_id TEXT,
    additional_attributes TEXT NOT NULL DEFAULT '{}',
    custom_attributes TEXT NOT NULL DEFAULT '{}',
    last_activity_at TEXT NOT NULL, snoozed_until TEXT, waiting_since TEXT,
    first_reply_created_at TEXT,
    created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
    CHECK(assignee_id IS NULL OR assignee_agent_bot_id IS NULL),
    UNIQUE(id, account_id, inbox_id),
    FOREIGN KEY(contact_inbox_id, account_id, inbox_id, contact_id)
        REFERENCES ops_ticket_contact_inbox(id, account_id, inbox_id, contact_id)
);
CREATE INDEX idx_ops_ticket_conversation_queue
    ON ops_ticket_conversation(account_id, inbox_id, status, assignee_id);
CREATE INDEX idx_ops_ticket_conversation_contact ON ops_ticket_conversation(contact_id);
CREATE INDEX idx_ops_ticket_conversation_reply
    ON ops_ticket_conversation(account_id, inbox_id, json_extract(additional_attributes, '$.in_reply_to'));
CREATE TABLE ops_ticket_message (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL, inbox_id INTEGER NOT NULL, conversation_id INTEGER NOT NULL,
    message_type INTEGER NOT NULL CHECK(message_type BETWEEN 0 AND 3),
    private BOOLEAN NOT NULL DEFAULT FALSE CHECK(private IN (0, 1)),
    content TEXT NOT NULL CHECK(length(content) <= 150000),
    content_type INTEGER NOT NULL DEFAULT 0 CHECK(content_type BETWEEN 0 AND 12),
    status INTEGER NOT NULL DEFAULT 0 CHECK(status BETWEEN 0 AND 3),
    sender_type TEXT NOT NULL, sender_id TEXT NOT NULL,
    source_id TEXT,
    content_attributes TEXT NOT NULL DEFAULT '{}',
    additional_attributes TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
    CHECK(private = 0 OR source_id IS NULL),
    UNIQUE(inbox_id, source_id),
    FOREIGN KEY(conversation_id, account_id, inbox_id)
        REFERENCES ops_ticket_conversation(id, account_id, inbox_id)
);
CREATE INDEX idx_ops_ticket_message_thread
    ON ops_ticket_message(conversation_id, account_id, created_at, id);
"#).await?;
        txn.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(
            "DROP TABLE ops_ticket_message; DROP TABLE ops_ticket_conversation; \
             DROP TABLE ops_ticket_contact_inbox; DROP TABLE ops_ticket_contact; \
             DROP TABLE ops_ticket_inbox;",
        )
        .await?;
        txn.commit().await
    }
}
