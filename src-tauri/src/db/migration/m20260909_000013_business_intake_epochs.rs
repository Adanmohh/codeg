//! B-owned forward integration for both000011/000012 installation orders.
use sea_orm::{ConnectionTrait, DbBackend, Statement, TransactionTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLES: [&str; 4] = [
    "business_intake_setup",
    "business_intake_import",
    "business_intake_source",
    "business_intake_candidate",
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tx = manager.get_connection().begin().await?;
        // Acquire SQLite's writer before inspecting columns. This is the same
        // metadata lock used by protected provisioning; no Principal is invented.
        tx.execute_unprepared("UPDATE business_tenancy_metadata SET id=id WHERE id=1")
            .await?;
        for table in TABLES {
            let exists = tx
                .query_one(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    "SELECT count(*) AS count FROM sqlite_master WHERE type='table' AND name=?",
                    [table.into()],
                ))
                .await?
                .ok_or_else(invalid)?;
            if exists.try_get::<i64>("", "count")? != 1 {
                return Err(invalid());
            }
            let present = tx.query_one(Statement::from_sql_and_values(DbBackend::Sqlite,
                "SELECT count(*) AS count FROM pragma_table_info(?) WHERE name='authorization_epoch'",
                [table.into()])).await?.ok_or_else(invalid)?;
            if present.try_get::<i64>("", "count")? == 0 {
                // Existing rows stay NULL. Never infer epoch1/current authority
                // from preserved content, a credential, or migration execution.
                tx.execute_unprepared(&format!("ALTER TABLE {table} ADD COLUMN authorization_epoch INTEGER CHECK(authorization_epoch > 0)"))
                    .await?;
            }
        }
        if !tx
            .query_all(Statement::from_string(
                DbBackend::Sqlite,
                "PRAGMA foreign_key_check".to_owned(),
            ))
            .await?
            .is_empty()
        {
            return Err(invalid());
        }
        // Per-column checks make the schema-commit/SeaORM-receipt retry harmless.
        tx.commit().await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Migration(
            "Intake captured authority/history must be retained".into(),
        ))
    }
}
fn invalid() -> DbErr {
    DbErr::Migration("Cannot validate retained intake schema".into())
}
