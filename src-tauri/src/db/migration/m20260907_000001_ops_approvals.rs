//! IntroMail Proposal/AuditLog/AgentRule/AgentScope port; see NOTICE.
use sea_orm_migration::{prelude::*, sea_orm::TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite is the shared desktop/server store. Partial indexes close the
        // NULL-domain-default uniqueness hole in the original scope schema.
        // SeaORM does not wrap SQLite migrations in a transaction. Keep all
        // four related tables/indexes/triggers atomic on failure.
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared(r#"
            CREATE TABLE ops_proposal (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task_id INTEGER NOT NULL REFERENCES work_task(id),
                run_seq INTEGER NOT NULL,
                agent_id TEXT NOT NULL,
                action_name TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                preview_json TEXT NOT NULL,
                edited_payload_json TEXT,
                status TEXT NOT NULL CHECK(status IN ('pending','approved','denied','auto')),
                verdict_by TEXT,
                created_at TEXT NOT NULL,
                resolved_at TEXT
            );
            CREATE UNIQUE INDEX ops_proposal_pending ON ops_proposal(task_id, run_seq) WHERE status = 'pending';
            CREATE TABLE ops_audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task_id INTEGER NOT NULL,
                run_seq INTEGER NOT NULL,
                actor TEXT NOT NULL,
                action TEXT NOT NULL,
                detail TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE INDEX ops_audit_task ON ops_audit_log(task_id, run_seq, id);
            CREATE TRIGGER ops_audit_no_update BEFORE UPDATE ON ops_audit_log BEGIN SELECT RAISE(ABORT, 'audit is append only'); END;
            CREATE TRIGGER ops_audit_no_delete BEFORE DELETE ON ops_audit_log BEGIN SELECT RAISE(ABORT, 'audit is append only'); END;
            CREATE TABLE ops_agent_rule (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                agent_id TEXT NOT NULL,
                domain TEXT NOT NULL,
                resource TEXT,
                action_name TEXT,
                behavior TEXT NOT NULL CHECK(behavior IN ('deny','ask','allow'))
            );
            CREATE INDEX ops_rule_match ON ops_agent_rule(agent_id, domain, behavior);
            CREATE TABLE ops_agent_scope (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                agent_id TEXT NOT NULL,
                domain TEXT NOT NULL,
                resource TEXT,
                mode TEXT NOT NULL CHECK(mode IN ('read','propose','act_low_risk'))
            );
            CREATE UNIQUE INDEX ops_scope_resource ON ops_agent_scope(agent_id, domain, resource) WHERE resource IS NOT NULL;
            CREATE UNIQUE INDEX ops_scope_default ON ops_agent_scope(agent_id, domain) WHERE resource IS NULL;
        "#).await?;
        txn.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;
        txn.execute_unprepared("DROP TABLE ops_proposal; DROP TABLE ops_audit_log; DROP TABLE ops_agent_rule; DROP TABLE ops_agent_scope;").await?;
        txn.commit().await
    }
}
