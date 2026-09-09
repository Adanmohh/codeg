//! Codeg's pinned-connection rebuild/receipt pattern from migration000012.
//! Retains task history while enabling exact file-only human submissions.
//! Exact Apache source attribution is in NOTICE; applied migrations are untouched.
use sea_orm::sqlx::{self, pool::PoolConnection, Connection, Executor, Sqlite};
use sea_orm_migration::{prelude::*, SchemaManagerConnection};

#[derive(DeriveMigrationName)]
pub struct Migration;

struct MigrationConnection {
    connection: PoolConnection<Sqlite>,
    restored: bool,
}
impl Drop for MigrationConnection {
    fn drop(&mut self) {
        if !self.restored {
            self.connection.close_on_drop();
        }
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let SchemaManagerConnection::Connection(db) = manager.get_connection() else {
            return Err(DbErr::Migration(
                "Execution rebuild requires an outer SQLite connection".into(),
            ));
        };
        let mut guard = MigrationConnection {
            connection: db
                .get_sqlite_connection_pool()
                .acquire()
                .await
                .map_err(error)?,
            restored: false,
        };
        (&mut *guard.connection)
            .execute("PRAGMA foreign_keys = OFF")
            .await
            .map_err(error)?;
        let result = rebuild(&mut guard.connection).await;
        (&mut *guard.connection)
            .execute("PRAGMA foreign_keys = ON")
            .await
            .map_err(error)?;
        let enabled: i64 = sqlx::query_scalar("PRAGMA foreign_keys")
            .fetch_one(&mut *guard.connection)
            .await
            .map_err(error)?;
        if enabled != 1 {
            return Err(DbErr::Migration(
                "Could not restore SQLite foreign keys".into(),
            ));
        }
        guard.restored = true;
        result.map_err(error)
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Migration(
            "Execution retains receipts, managed files and task history; lossy downgrade refused"
                .into(),
        ))
    }
}

fn error(_: sqlx::Error) -> DbErr {
    DbErr::Migration("Atomic business execution migration failed".into())
}

async fn rebuild(conn: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin_with("BEGIN IMMEDIATE").await?;
    let result = async {
        let completed: i64 = sqlx::query_scalar("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='business_execution_metadata'")
            .fetch_one(&mut *tx).await?;
        if completed == 0 {
            (&mut *tx).execute(include_str!("m20260909_000014_business_execution.sql")).await?;
        }
        let violations = sqlx::query("PRAGMA foreign_key_check").fetch_all(&mut *tx).await?;
        if !violations.is_empty() {
            return Err(sqlx::Error::Protocol("Retained foreign key violation".into()));
        }
        Ok(())
    }.await;
    match result {
        Ok(()) => tx.commit().await,
        Err(e) => {
            tx.rollback().await?;
            Err(e)
        }
    }
}
