//! Forward-only tenant registry rebuild. Existing child tables/IDs stay intact.
use sea_orm::sqlx::{self, pool::PoolConnection, Connection, Executor, Sqlite};
use sea_orm_migration::{prelude::*, SchemaManagerConnection};

#[derive(DeriveMigrationName)]
pub struct Migration;

// SQLite cannot change foreign_keys inside a transaction. Pin ONE connection;
// never send this PRAGMA through the pool. Cancellation closes the connection
// instead of returning disabled enforcement to another caller (SQLx0.8.6).
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
                "Tenancy rebuild requires an outer SQLite connection".into(),
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
        // The rebuild explicitly rolls back on error before this restoration.
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
            "Tenancy preserves credential/history lineage; lossy downgrade refused".into(),
        ))
    }
}
fn error(_: sqlx::Error) -> DbErr {
    DbErr::Migration("Atomic business tenancy rebuild failed".into())
}
async fn rebuild(conn: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin_with("BEGIN IMMEDIATE").await?;
    let result = async {
        // Handles process loss after our commit but before SeaORM records the
        // migration name. The marker is created in the same atomic transaction.
        let completed: i64 = sqlx::query_scalar("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='business_tenancy_metadata'")
            .fetch_one(&mut *tx).await?;
        if completed == 0 {
            (&mut *tx).execute(SCHEMA).await?;
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

const SCHEMA: &str = r#"
CREATE TABLE business_organization_new (
 id TEXT PRIMARY KEY NOT NULL,
 singleton INTEGER UNIQUE CHECK (singleton = 1),
 name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 120),
 created_at TEXT NOT NULL,
 status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','suspended')),
 revision INTEGER NOT NULL DEFAULT 1 CHECK (revision > 0),
 authorization_epoch INTEGER NOT NULL DEFAULT 1 CHECK (authorization_epoch > 0)
);
INSERT INTO business_organization_new (id,singleton,name,created_at)
 SELECT id,singleton,name,created_at FROM business_organization;
DROP TABLE business_organization;
ALTER TABLE business_organization_new RENAME TO business_organization;
CREATE TABLE business_tenancy_metadata (
 id INTEGER PRIMARY KEY CHECK (id=1),
 original_organization_id TEXT UNIQUE REFERENCES business_organization(id) ON DELETE RESTRICT
);
INSERT INTO business_tenancy_metadata (id,original_organization_id)
 VALUES (1,(SELECT id FROM business_organization WHERE singleton=1));
CREATE TRIGGER business_original_org_immutable BEFORE UPDATE OF original_organization_id ON business_tenancy_metadata
 WHEN OLD.original_organization_id IS NOT NULL AND (NEW.original_organization_id IS NULL OR NEW.original_organization_id != OLD.original_organization_id)
 BEGIN SELECT RAISE(ABORT,'Original organization is immutable'); END;
CREATE TRIGGER business_tenancy_metadata_no_delete BEFORE DELETE ON business_tenancy_metadata
 BEGIN SELECT RAISE(ABORT,'Original organization mapping is retained'); END;
CREATE TRIGGER business_organization_identity_immutable BEFORE UPDATE OF id,singleton ON business_organization
 WHEN NEW.id != OLD.id OR NEW.singleton IS NOT OLD.singleton
 BEGIN SELECT RAISE(ABORT,'Organization identity is immutable'); END;
CREATE TRIGGER business_organization_epoch_monotonic BEFORE UPDATE OF status,revision,authorization_epoch ON business_organization
 WHEN NEW.status = OLD.status OR NEW.revision != OLD.revision + 1 OR NEW.authorization_epoch != OLD.authorization_epoch + 1
 BEGIN SELECT RAISE(ABORT,'Lifecycle transitions must advance authority'); END;
CREATE TABLE business_tenant_settings (
 organization_id TEXT PRIMARY KEY NOT NULL REFERENCES business_organization(id) ON DELETE RESTRICT,
 revision INTEGER NOT NULL DEFAULT 1 CHECK (revision > 0),
 settings_json TEXT NOT NULL CHECK (json_valid(settings_json))
);
INSERT INTO business_tenant_settings (organization_id,settings_json)
 SELECT id,json_object('displayName',name,'palette','neutral','workspaceLayout','split','defaultWorkArea','tasks') FROM business_organization;
CREATE TABLE business_platform_receipt (
 operation_id TEXT PRIMARY KEY NOT NULL,
 input_hash TEXT NOT NULL CHECK (length(input_hash)=64),
 action TEXT NOT NULL CHECK(action IN ('tenant_created','tenant_status_changed','owner_credential_reissued')),
 organization_id TEXT NOT NULL REFERENCES business_organization(id) ON DELETE RESTRICT,
 result_json TEXT NOT NULL CHECK (json_valid(result_json)),
 created_at TEXT NOT NULL
);
CREATE TRIGGER business_platform_receipt_no_update BEFORE UPDATE ON business_platform_receipt
 BEGIN SELECT RAISE(ABORT,'Platform receipt is immutable'); END;
CREATE TRIGGER business_platform_receipt_no_delete BEFORE DELETE ON business_platform_receipt
 BEGIN SELECT RAISE(ABORT,'Platform receipt is immutable'); END;
CREATE TABLE business_provisioning_owner (
 organization_id TEXT PRIMARY KEY NOT NULL REFERENCES business_organization(id) ON DELETE RESTRICT,
 member_id TEXT NOT NULL,
 credential_id TEXT NOT NULL REFERENCES business_credential(id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,member_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT
);
CREATE TRIGGER business_provisioning_owner_identity_immutable BEFORE UPDATE OF organization_id,member_id ON business_provisioning_owner
 BEGIN SELECT RAISE(ABORT,'Provisioning owner is immutable'); END;
"#;
