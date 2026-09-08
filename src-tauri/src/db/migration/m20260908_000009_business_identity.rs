use sea_orm::{ConnectionTrait, TransactionTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tx = manager.get_connection().begin().await?;
        tx.execute_unprepared(r#"
CREATE TABLE business_organization (
    id TEXT PRIMARY KEY NOT NULL,
    singleton INTEGER NOT NULL UNIQUE CHECK (singleton = 1),
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 120),
    created_at TEXT NOT NULL
);
CREATE TABLE business_member (
    id TEXT PRIMARY KEY NOT NULL,
    organization_id TEXT NOT NULL REFERENCES business_organization(id) ON DELETE RESTRICT,
    display_name TEXT NOT NULL CHECK (length(display_name) BETWEEN 1 AND 120),
    kind TEXT NOT NULL CHECK (kind IN ('human', 'agent')),
    role TEXT NOT NULL CHECK (role IN ('owner', 'admin', 'manager', 'member', 'viewer')),
    domains_json TEXT NOT NULL CHECK (json_valid(domains_json) AND json_type(domains_json) = 'array'),
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'revoked')),
    revision INTEGER NOT NULL DEFAULT 1 CHECK (revision > 0),
    operator_owner INTEGER NOT NULL DEFAULT 0 CHECK (operator_owner IN (0, 1)),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE (organization_id, id),
    CHECK (kind != 'agent' OR (role = 'member' AND operator_owner = 0)),
    CHECK (operator_owner = 0 OR (kind = 'human' AND role = 'owner' AND status = 'active'))
);
CREATE UNIQUE INDEX business_one_operator_owner ON business_member(organization_id) WHERE operator_owner = 1;
CREATE TABLE business_credential (
    id TEXT PRIMARY KEY NOT NULL,
    organization_id TEXT NOT NULL,
    member_id TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE CHECK (length(token_hash) = 64),
    label TEXT NOT NULL CHECK (length(label) BETWEEN 1 AND 120),
    created_at TEXT NOT NULL,
    revoked_at TEXT,
    FOREIGN KEY (organization_id, member_id) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT
);
CREATE INDEX business_credential_member ON business_credential(organization_id, member_id);
CREATE TABLE business_identity_event (
    id TEXT PRIMARY KEY NOT NULL,
    organization_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    action TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (organization_id, actor_id) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT
);
CREATE TRIGGER business_identity_event_no_update BEFORE UPDATE ON business_identity_event
BEGIN SELECT RAISE(ABORT, 'Business identity history is immutable'); END;
CREATE TRIGGER business_identity_event_no_delete BEFORE DELETE ON business_identity_event
BEGIN SELECT RAISE(ABORT, 'Business identity history is immutable'); END;
CREATE TRIGGER business_member_identity_immutable BEFORE UPDATE OF id, organization_id, kind, operator_owner ON business_member
WHEN NEW.id != OLD.id OR NEW.organization_id != OLD.organization_id OR NEW.kind != OLD.kind OR NEW.operator_owner != OLD.operator_owner
BEGIN SELECT RAISE(ABORT, 'Business member identity is immutable'); END;
CREATE TRIGGER business_credential_identity_immutable BEFORE UPDATE OF id, organization_id, member_id, token_hash ON business_credential
BEGIN SELECT RAISE(ABORT, 'Business credential identity is immutable'); END;
CREATE TRIGGER business_credential_no_reactivation BEFORE UPDATE OF revoked_at ON business_credential
WHEN OLD.revoked_at IS NOT NULL AND (NEW.revoked_at IS NULL OR NEW.revoked_at != OLD.revoked_at)
BEGIN SELECT RAISE(ABORT, 'Business credential revocation is permanent'); END;
"#).await?;
        tx.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tx = manager.get_connection().begin().await?;
        let row = tx
            .query_one(sea_orm::Statement::from_string(
                sea_orm::DbBackend::Sqlite,
                "SELECT COUNT(*) AS count FROM business_organization".to_owned(),
            ))
            .await?
            .ok_or_else(|| DbErr::Custom("Cannot inspect business identities".into()))?;
        if row.try_get::<i64>("", "count")? != 0 {
            return Err(DbErr::Custom(
                "Cannot remove initialized business identities and history".into(),
            ));
        }
        tx.execute_unprepared("DROP TABLE business_identity_event; DROP TABLE business_credential; DROP TABLE business_member; DROP TABLE business_organization;").await?;
        tx.commit().await
    }
}
