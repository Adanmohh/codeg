//! Local host state only; existing intake proof/receipt and approvals remain authoritative.
use sea_orm_migration::{prelude::*, sea_orm::TransactionTrait};
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(r#"
            CREATE TABLE ops_intake_host_product (
                product_id TEXT PRIMARY KEY, account_id INTEGER NOT NULL CHECK(account_id>0),
                config_json TEXT NOT NULL
            );
            CREATE TABLE ops_intake_host_snapshot (
                product_id TEXT NOT NULL REFERENCES ops_intake_host_product(product_id),
                ulid TEXT NOT NULL, record_json TEXT NOT NULL, verified_at INTEGER, error TEXT,
                PRIMARY KEY(product_id,ulid)
            );
            CREATE TABLE ops_intake_host_draft (
                id TEXT PRIMARY KEY, product_id TEXT NOT NULL, ulid TEXT NOT NULL,
                revision INTEGER NOT NULL, draft_json TEXT NOT NULL,
                UNIQUE(product_id,ulid),
                FOREIGN KEY(product_id,ulid) REFERENCES ops_intake_host_snapshot(product_id,ulid)
            );
            CREATE TABLE ops_intake_host_handoff (
                proposal_id INTEGER PRIMARY KEY REFERENCES ops_proposal(id),
                product_id TEXT NOT NULL, ulid TEXT NOT NULL,
                state TEXT NOT NULL CHECK(state IN ('unknown','not_authorized','recorded')),
                FOREIGN KEY(product_id,ulid) REFERENCES ops_intake_host_snapshot(product_id,ulid)
            );
            CREATE TABLE ops_intake_host_fix (
                product_id TEXT NOT NULL, ulid TEXT NOT NULL, task_id INTEGER NOT NULL REFERENCES work_task(id),
                PRIMARY KEY(product_id,ulid),
                FOREIGN KEY(product_id,ulid) REFERENCES ops_intake_host_snapshot(product_id,ulid)
            );
        "#).await?;
        txn.commit().await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared("DROP TABLE ops_intake_host_fix; DROP TABLE ops_intake_host_handoff; DROP TABLE ops_intake_host_draft; DROP TABLE ops_intake_host_snapshot; DROP TABLE ops_intake_host_product;").await?;
        Ok(())
    }
}
