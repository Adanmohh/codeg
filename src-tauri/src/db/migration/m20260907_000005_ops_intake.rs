//! Evidence and durable filing receipt glue. SQLite atomic pattern: codeg / NOTICE.
use sea_orm_migration::{prelude::*, sea_orm::TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(
            r#"
            CREATE TABLE ops_intake_binding (
                product_id TEXT PRIMARY KEY,
                folder_id INTEGER NOT NULL REFERENCES folder(id),
                config_json TEXT NOT NULL
            );
            CREATE TABLE ops_intake_source (
                source_key TEXT PRIMARY KEY,
                product_id TEXT NOT NULL REFERENCES ops_intake_binding(product_id),
                revision TEXT NOT NULL,
                fetched_at INTEGER NOT NULL
            );
            CREATE TABLE ops_intake_evidence (
                artifact_id TEXT PRIMARY KEY,
                source_key TEXT NOT NULL REFERENCES ops_intake_source(source_key),
                revision TEXT NOT NULL,
                field TEXT NOT NULL CHECK(field IN ('build','screen','reciter','log')),
                value TEXT NOT NULL,
                content BLOB NOT NULL,
                sha256 TEXT NOT NULL,
                reviewed_by TEXT NOT NULL,
                captured_at TEXT,
                session_ulid TEXT,
                expires_at INTEGER
            );
            CREATE TABLE ops_intake_filing (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                source_key TEXT NOT NULL REFERENCES ops_intake_source(source_key),
                repository_id INTEGER NOT NULL,
                proposal_id INTEGER NOT NULL UNIQUE REFERENCES ops_proposal(id),
                task_id INTEGER NOT NULL REFERENCES work_task(id),
                run_seq INTEGER NOT NULL,
                payload_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                started_at INTEGER NOT NULL,
                state TEXT NOT NULL CHECK(state IN ('unknown','failed','created')),
                issue_json TEXT,
                error_code TEXT,
                retry_after INTEGER
            );
            CREATE UNIQUE INDEX ops_intake_filing_active
                ON ops_intake_filing(source_key, repository_id)
                WHERE state IN ('unknown','created');
        "#,
        )
        .await?;
        txn.commit().await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared("DROP TABLE ops_intake_filing; DROP TABLE ops_intake_evidence; DROP TABLE ops_intake_source; DROP TABLE ops_intake_binding;").await?;
        txn.commit().await
    }
}
