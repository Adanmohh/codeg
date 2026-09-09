//! Forward-only scan ordering. Migration000014 and captured authority stay intact.
//! Reuses Codeg's additive writer/receipt-retry pattern; exact sources in NOTICE.
use sea_orm::{ConnectionTrait, DbBackend, Statement, TransactionTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const SCHEMA: &str = r#"
CREATE TABLE business_execution_scan (
 organization_id TEXT NOT NULL,
 session_id TEXT NOT NULL,
 generation INTEGER NOT NULL,
 accepted_scan INTEGER NOT NULL DEFAULT 0
  CHECK(typeof(accepted_scan)='integer' AND accepted_scan BETWEEN 0 AND 9007199254740991),
 PRIMARY KEY(organization_id,session_id,generation),
 FOREIGN KEY(organization_id,session_id,generation)
  REFERENCES business_execution_generation(organization_id,session_id,generation) ON DELETE RESTRICT
);
-- Zero means no scan accepted by this forward mechanism, never a refreshed grant
-- or a claim that previously retained output bytes are current.
INSERT INTO business_execution_scan(organization_id,session_id,generation)
 SELECT organization_id,session_id,generation FROM business_execution_generation;
CREATE TRIGGER business_execution_scan_created AFTER INSERT ON business_execution_generation
 BEGIN INSERT INTO business_execution_scan(organization_id,session_id,generation)
  VALUES(NEW.organization_id,NEW.session_id,NEW.generation); END;
CREATE TRIGGER business_execution_scan_identity
 BEFORE UPDATE OF organization_id,session_id,generation ON business_execution_scan
 BEGIN SELECT RAISE(ABORT,'Scan generation identity is immutable'); END;
CREATE TRIGGER business_execution_scan_monotonic BEFORE UPDATE OF accepted_scan ON business_execution_scan
 WHEN NEW.accepted_scan != OLD.accepted_scan + 1
 BEGIN SELECT RAISE(ABORT,'Accepted scan ordering is monotonic'); END;
CREATE TRIGGER business_execution_scan_retained BEFORE DELETE ON business_execution_scan
 BEGIN SELECT RAISE(ABORT,'Accepted scan ordering is retained'); END;
CREATE TABLE business_execution_scan_metadata (id INTEGER PRIMARY KEY CHECK(id=1));
INSERT INTO business_execution_scan_metadata VALUES(1);
"#;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tx = manager.get_connection().begin().await?;
        let foreign_keys = tx
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                "PRAGMA foreign_keys".to_owned(),
            ))
            .await?
            .ok_or_else(invalid)?;
        if foreign_keys.try_get::<i64>("", "foreign_keys")? != 1 {
            return Err(invalid());
        }
        // Acquire the existing SQLite writer before checking the committed marker.
        // No FK disabling, history rewrite, runtime authority or pool reacquisition.
        tx.execute_unprepared("UPDATE business_execution_metadata SET id=id WHERE id=1")
            .await?;
        let completed = tx.query_one(Statement::from_string(DbBackend::Sqlite,
            "SELECT count(*) AS n FROM sqlite_master WHERE type='table' AND name='business_execution_scan_metadata'".to_owned()))
            .await?.ok_or_else(invalid)?;
        if completed.try_get::<i64>("", "n")? == 0 {
            tx.execute_unprepared(SCHEMA).await?;
        }
        // Receipt retry preserves every accepted counter. A partial/mismatched
        // schema fails closed instead of backfilling or resetting live ordering.
        let missing = tx.query_one(Statement::from_string(DbBackend::Sqlite,
            "SELECT count(*) AS n FROM business_execution_generation g LEFT JOIN business_execution_scan s ON s.organization_id=g.organization_id AND s.session_id=g.session_id AND s.generation=g.generation WHERE s.session_id IS NULL".to_owned()))
            .await?.ok_or_else(invalid)?;
        if missing.try_get::<i64>("", "n")? != 0
            || !tx
                .query_all(Statement::from_string(
                    DbBackend::Sqlite,
                    "PRAGMA foreign_key_check".to_owned(),
                ))
                .await?
                .is_empty()
        {
            return Err(invalid());
        }
        tx.commit().await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Migration(
            "Accepted output scan ordering must be retained".into(),
        ))
    }
}

fn invalid() -> DbErr {
    DbErr::Migration("Cannot validate retained execution scan state".into())
}
