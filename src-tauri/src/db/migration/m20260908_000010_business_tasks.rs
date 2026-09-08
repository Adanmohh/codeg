//! Codeg explicit SQLite migration transaction; provenance in NOTICE.
use sea_orm::{ConnectionTrait, TransactionTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tx = manager.get_connection().begin().await?;
        tx.execute_unprepared(r#"
CREATE TABLE business_task (
    id TEXT PRIMARY KEY NOT NULL,
    organization_id TEXT NOT NULL REFERENCES business_organization(id) ON DELETE RESTRICT,
    title TEXT NOT NULL CHECK (length(trim(title)) BETWEEN 1 AND 240),
    notes TEXT NOT NULL DEFAULT '' CHECK (length(notes) <= 20000),
    domain TEXT NOT NULL CHECK (domain IN ('marketing','channels','ads','website','feedback','engineering')),
    status TEXT NOT NULL DEFAULT 'todo' CHECK (status IN ('todo','in_progress','review','done','cancelled')),
    priority TEXT NOT NULL DEFAULT 'normal' CHECK (priority IN ('low','normal','high','urgent')),
    due_date TEXT CHECK (due_date IS NULL OR (length(due_date) = 10 AND due_date GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]')),
    owner_id TEXT NOT NULL,
    assignee_id TEXT,
    creator_id TEXT NOT NULL,
    reviewer_id TEXT,
    revision INTEGER NOT NULL DEFAULT 1 CHECK (revision > 0),
    current_deliverable_id TEXT,
    current_execution_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived_at TEXT,
    UNIQUE (organization_id, id),
    CHECK (archived_at IS NULL OR status IN ('done','cancelled')),
    FOREIGN KEY (organization_id, owner_id) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT,
    FOREIGN KEY (organization_id, assignee_id) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT,
    FOREIGN KEY (organization_id, creator_id) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT,
    FOREIGN KEY (organization_id, reviewer_id) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT
);
CREATE INDEX business_task_queue ON business_task(organization_id, archived_at, domain, status, updated_at DESC, id);
CREATE INDEX business_task_assignee ON business_task(organization_id, assignee_id);
CREATE TABLE business_task_activity (
    id TEXT PRIMARY KEY NOT NULL,
    organization_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    revision INTEGER NOT NULL CHECK (revision > 0),
    kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    actor_name TEXT NOT NULL,
    actor_kind TEXT NOT NULL CHECK (actor_kind IN ('human','agent')),
    payload_json TEXT NOT NULL CHECK (json_valid(payload_json)),
    created_at TEXT NOT NULL,
    UNIQUE (organization_id, task_id, revision),
    FOREIGN KEY (organization_id, task_id) REFERENCES business_task(organization_id, id) ON DELETE RESTRICT,
    FOREIGN KEY (organization_id, actor_id) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT
);
CREATE TABLE business_task_execution (
    id TEXT PRIMARY KEY NOT NULL,
    organization_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    work_task_id INTEGER NOT NULL CHECK (work_task_id > 0),
    run_seq INTEGER NOT NULL CHECK (run_seq > 0),
    connection_id TEXT NOT NULL CHECK (length(connection_id) > 0),
    agent_member_id TEXT NOT NULL,
    agent_key TEXT NOT NULL CHECK (length(agent_key) > 0),
    delegation_json TEXT NOT NULL CHECK (json_valid(delegation_json)),
    linked_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    revoked_at TEXT,
    UNIQUE (organization_id, task_id, id),
    FOREIGN KEY (organization_id, task_id) REFERENCES business_task(organization_id, id) ON DELETE RESTRICT,
    FOREIGN KEY (organization_id, agent_member_id) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT,
    FOREIGN KEY (organization_id, linked_by) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT
);
CREATE UNIQUE INDEX business_task_one_active_run ON business_task_execution(work_task_id, run_seq) WHERE revoked_at IS NULL;
CREATE UNIQUE INDEX business_task_one_active_execution ON business_task_execution(organization_id, task_id) WHERE revoked_at IS NULL;
CREATE TABLE business_task_deliverable (
    id TEXT PRIMARY KEY NOT NULL,
    organization_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    revision INTEGER NOT NULL CHECK (revision > 0),
    author_id TEXT NOT NULL,
    author_name TEXT NOT NULL,
    author_kind TEXT NOT NULL CHECK (author_kind IN ('human','agent')),
    body TEXT NOT NULL CHECK (length(trim(body)) BETWEEN 1 AND 20000),
    execution_id TEXT,
    created_at TEXT NOT NULL,
    UNIQUE (organization_id, task_id, id),
    FOREIGN KEY (organization_id, task_id) REFERENCES business_task(organization_id, id) ON DELETE RESTRICT,
    FOREIGN KEY (organization_id, author_id) REFERENCES business_member(organization_id, id) ON DELETE RESTRICT,
    FOREIGN KEY (organization_id, task_id, execution_id) REFERENCES business_task_execution(organization_id, task_id, id) ON DELETE RESTRICT
);
CREATE INDEX business_task_deliverable_history ON business_task_deliverable(organization_id, task_id, revision);
CREATE TRIGGER business_task_identity_immutable BEFORE UPDATE OF id, organization_id, creator_id, created_at ON business_task
WHEN NEW.id != OLD.id OR NEW.organization_id != OLD.organization_id OR NEW.creator_id != OLD.creator_id OR NEW.created_at != OLD.created_at
BEGIN SELECT RAISE(ABORT, 'Business task authorship is immutable'); END;
CREATE TRIGGER business_task_no_delete BEFORE DELETE ON business_task
BEGIN SELECT RAISE(ABORT, 'Archive business tasks to retain history'); END;
CREATE TRIGGER business_task_activity_no_update BEFORE UPDATE ON business_task_activity
BEGIN SELECT RAISE(ABORT, 'Business task history is immutable'); END;
CREATE TRIGGER business_task_activity_no_delete BEFORE DELETE ON business_task_activity
BEGIN SELECT RAISE(ABORT, 'Business task history is immutable'); END;
CREATE TRIGGER business_task_deliverable_no_update BEFORE UPDATE ON business_task_deliverable
BEGIN SELECT RAISE(ABORT, 'Business task deliverables are immutable'); END;
CREATE TRIGGER business_task_deliverable_no_delete BEFORE DELETE ON business_task_deliverable
BEGIN SELECT RAISE(ABORT, 'Business task deliverables are immutable'); END;
CREATE TRIGGER business_task_execution_identity_immutable BEFORE UPDATE OF id, organization_id, task_id, work_task_id, run_seq, connection_id, agent_member_id, agent_key, delegation_json, linked_by, created_at ON business_task_execution
BEGIN SELECT RAISE(ABORT, 'Business execution lineage is immutable'); END;
CREATE TRIGGER business_task_execution_no_reactivation BEFORE UPDATE OF revoked_at ON business_task_execution
WHEN OLD.revoked_at IS NOT NULL AND (NEW.revoked_at IS NULL OR NEW.revoked_at != OLD.revoked_at)
BEGIN SELECT RAISE(ABORT, 'Business execution revocation is permanent'); END;
CREATE TRIGGER business_task_execution_no_delete BEFORE DELETE ON business_task_execution
BEGIN SELECT RAISE(ABORT, 'Business execution lineage is retained'); END;
"#).await?;
        tx.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tx = manager.get_connection().begin().await?;
        let row = tx.query_one(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Sqlite,
            "SELECT COUNT(*) AS count FROM business_task".to_owned(),
        )).await?.ok_or_else(|| DbErr::Custom("Cannot inspect business tasks".into()))?;
        if row.try_get::<i64>("", "count")? != 0 {
            return Err(DbErr::Custom("Cannot remove business tasks and retained history".into()));
        }
        tx.execute_unprepared("DROP TABLE business_task_deliverable; DROP TABLE business_task_execution; DROP TABLE business_task_activity; DROP TABLE business_task;").await?;
        tx.commit().await
    }
}
