//! One atomic SQLite migration for private intake. Existing Codeg patterns: NOTICE.
use sea_orm::{ConnectionTrait, TransactionTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tx = manager.get_connection().begin().await?;
        let marker = tx.query_one(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Sqlite,
            "SELECT count(*) AS count FROM sqlite_master WHERE type='table' AND name='business_intake_migration'".to_owned(),
        )).await?.ok_or_else(|| DbErr::Migration("Cannot inspect intake migration".into()))?;
        if marker.try_get::<i64>("", "count")? == 1 {
            // SQLite schema commit can precede SeaORM's separate migration receipt.
            // Never rebuild/erase retained source history on a receipt retry.
            return tx.commit().await;
        }
        tx.execute_unprepared(r#"
CREATE TABLE business_intake_binding (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL REFERENCES business_organization(id) ON DELETE RESTRICT,
 kind TEXT NOT NULL CHECK(kind IN ('fireflies','email','hafidh_testflight')),
 label TEXT NOT NULL CHECK(length(trim(label)) BETWEEN 1 AND 120),
 domain TEXT NOT NULL CHECK(domain IN ('marketing','channels','ads','website','feedback','engineering')),
 source_owner_id TEXT NOT NULL, owner_authority_revision INTEGER NOT NULL CHECK(owner_authority_revision > 0),
 resource_json TEXT NOT NULL CHECK(json_valid(resource_json)), credential_ref TEXT,
 revision INTEGER NOT NULL DEFAULT 1 CHECK(revision > 0), access_epoch INTEGER NOT NULL DEFAULT 1 CHECK(access_epoch > 0),
 enabled BOOLEAN NOT NULL DEFAULT 0 CHECK(enabled IN (0,1)),
 publication_domains TEXT NOT NULL CHECK(json_valid(publication_domains)), retained_task_text BOOLEAN NOT NULL CHECK(retained_task_text IN (0,1)),
 created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
 UNIQUE(organization_id,id),
 FOREIGN KEY(organization_id,source_owner_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(credential_ref) REFERENCES business_intake_setup(credential_ref) ON DELETE RESTRICT
);
CREATE TABLE business_intake_grant (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, binding_id TEXT NOT NULL, member_id TEXT NOT NULL,
 revision INTEGER NOT NULL CHECK(revision > 0), state TEXT NOT NULL CHECK(state IN ('active','revoked')),
 scope TEXT NOT NULL CHECK(scope = 'binding_current_and_future_sources'),
 can_read BOOLEAN NOT NULL CHECK(can_read IN (0,1)), can_import BOOLEAN NOT NULL CHECK(can_import IN (0,1)), can_triage BOOLEAN NOT NULL CHECK(can_triage IN (0,1)),
 publication_domains TEXT NOT NULL CHECK(json_valid(publication_domains)), expires_at TEXT,
 CHECK(can_read = 1 OR (can_import = 0 AND can_triage = 0 AND publication_domains = '[]')),
 UNIQUE(organization_id,binding_id,member_id), UNIQUE(organization_id,binding_id,id),
 FOREIGN KEY(organization_id,binding_id) REFERENCES business_intake_binding(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,member_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_intake_setup (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, actor_id TEXT NOT NULL, operation_id TEXT NOT NULL,
 digest TEXT NOT NULL, binding_id TEXT NOT NULL, base_revision INTEGER, base_epoch INTEGER,
 owner_authority_revision INTEGER NOT NULL CHECK(owner_authority_revision > 0),
 plan_json TEXT NOT NULL CHECK(json_valid(plan_json)), credential_ref TEXT NOT NULL UNIQUE,
 state TEXT NOT NULL CHECK(state IN ('staged','active','retired')), expires_at TEXT NOT NULL, created_at TEXT NOT NULL, cleanup_at TEXT,
 UNIQUE(organization_id,actor_id,operation_id),
 FOREIGN KEY(organization_id,actor_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_intake_source (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, binding_id TEXT NOT NULL, kind TEXT NOT NULL,
 external_id TEXT NOT NULL, title TEXT NOT NULL, revision INTEGER CHECK(revision > 0),
 refresh_fence INTEGER NOT NULL DEFAULT 1 CHECK(refresh_fence > 0), observed_epoch INTEGER, observed_at TEXT, access_until TEXT,
 access TEXT NOT NULL DEFAULT 'unverified' CHECK(access IN ('unverified','fresh','denied')),
 content TEXT NOT NULL DEFAULT 'missing' CHECK(content IN ('missing','available','unsupported')),
 summary TEXT NOT NULL DEFAULT 'missing' CHECK(summary IN ('missing','empty','available','unsupported')),
 provider_summary_status TEXT, seeded BOOLEAN NOT NULL DEFAULT 0 CHECK(seeded IN (0,1)),
 UNIQUE(organization_id,binding_id,kind,external_id), UNIQUE(organization_id,id), UNIQUE(organization_id,binding_id,id),
 FOREIGN KEY(organization_id,binding_id) REFERENCES business_intake_binding(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,id,revision) REFERENCES business_intake_version(organization_id,source_id,revision) ON DELETE RESTRICT
);
CREATE INDEX business_intake_source_list ON business_intake_source(organization_id,binding_id,id);
CREATE TABLE business_intake_version (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, source_id TEXT NOT NULL, revision INTEGER NOT NULL CHECK(revision > 0),
 digest TEXT NOT NULL, normalization_version INTEGER NOT NULL CHECK(normalization_version > 0), observed_at TEXT NOT NULL,
 provider_revision TEXT, title TEXT NOT NULL,
 UNIQUE(organization_id,source_id,revision),
 FOREIGN KEY(organization_id,source_id) REFERENCES business_intake_source(organization_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_intake_passage (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, source_id TEXT NOT NULL, source_revision INTEGER NOT NULL,
 ordinal INTEGER NOT NULL CHECK(ordinal >= 0), kind TEXT NOT NULL CHECK(kind IN ('sentence','summary','email_message','feedback')),
 text TEXT NOT NULL CHECK(length(text) <= 20000), provider_index INTEGER CHECK(provider_index >= 0),
 start REAL CHECK(start >= 0), end REAL CHECK(end >= start),
 UNIQUE(organization_id,source_id,source_revision,ordinal), UNIQUE(organization_id,source_id,source_revision,id),
 FOREIGN KEY(organization_id,source_id,source_revision) REFERENCES business_intake_version(organization_id,source_id,revision) ON DELETE RESTRICT
);
CREATE TABLE business_intake_candidate (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, source_id TEXT NOT NULL,
 revision INTEGER NOT NULL CHECK(revision > 0), source_revision INTEGER NOT NULL CHECK(source_revision > 0), prepared_epoch INTEGER NOT NULL CHECK(prepared_epoch > 0),
 state TEXT NOT NULL CHECK(state IN ('pending','accepted','linked','discarded')),
 origin TEXT NOT NULL CHECK(origin IN ('source_review','human_selection')),
 passage_ids TEXT NOT NULL CHECK(json_valid(passage_ids)), draft_json TEXT CHECK(draft_json IS NULL OR json_valid(draft_json)),
 owner_suggestion TEXT CHECK(length(owner_suggestion) <= 240), due_suggestion TEXT CHECK(length(due_suggestion) <= 240),
 created_by TEXT NOT NULL, updated_by TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
 UNIQUE(organization_id,id), UNIQUE(organization_id,source_id,id),
 FOREIGN KEY(organization_id,source_id,source_revision) REFERENCES business_intake_version(organization_id,source_id,revision) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,created_by) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,updated_by) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT
);
CREATE UNIQUE INDEX business_intake_one_seed ON business_intake_candidate(organization_id,source_id) WHERE origin = 'source_review';
CREATE INDEX business_intake_candidate_list ON business_intake_candidate(organization_id,source_id,state,id);
CREATE TABLE business_intake_import (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, binding_id TEXT NOT NULL, requester_id TEXT NOT NULL,
 revision INTEGER NOT NULL DEFAULT 1 CHECK(revision > 0), state TEXT NOT NULL CHECK(state IN ('queued','running','waiting','complete','failed','cancelled')),
 coverage TEXT NOT NULL CHECK(coverage IN ('not_started','partial','bounded_end','capped')),
 selection_json TEXT NOT NULL CHECK(json_valid(selection_json)), scan_offset INTEGER NOT NULL DEFAULT 0 CHECK(scan_offset BETWEEN 0 AND 250),
 scan_done BOOLEAN NOT NULL DEFAULT 0 CHECK(scan_done IN (0,1)), discovered INTEGER NOT NULL DEFAULT 0 CHECK(discovered >= 0),
 completed INTEGER NOT NULL DEFAULT 0 CHECK(completed >= 0), failed INTEGER NOT NULL DEFAULT 0 CHECK(failed >= 0),
 attempt_id TEXT, attempt_actor_id TEXT, lease_until TEXT, attempt_count INTEGER NOT NULL DEFAULT 0 CHECK(attempt_count BETWEEN 0 AND 3),
 attempt_epoch INTEGER, next_attempt_at TEXT, error_code TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL,
 UNIQUE(organization_id,id), UNIQUE(organization_id,binding_id,id),
 FOREIGN KEY(organization_id,binding_id) REFERENCES business_intake_binding(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,requester_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,attempt_actor_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT
);
CREATE INDEX business_intake_import_list ON business_intake_import(organization_id,binding_id,created_at DESC,id);
CREATE TABLE business_intake_import_item (
 organization_id TEXT NOT NULL, binding_id TEXT NOT NULL, import_id TEXT NOT NULL, source_id TEXT NOT NULL,
 state TEXT NOT NULL CHECK(state IN ('pending','complete','failed')),
 PRIMARY KEY(organization_id,import_id,source_id),
 FOREIGN KEY(organization_id,binding_id,import_id) REFERENCES business_intake_import(organization_id,binding_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,binding_id,source_id) REFERENCES business_intake_source(organization_id,binding_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_intake_decision (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, candidate_id TEXT NOT NULL, source_id TEXT NOT NULL,
 from_revision INTEGER NOT NULL CHECK(from_revision > 0), source_revision INTEGER NOT NULL CHECK(source_revision > 0),
 kind TEXT NOT NULL CHECK(kind IN ('accepted','linked','discarded')), actor_id TEXT NOT NULL, task_id TEXT, task_revision INTEGER,
 publication_domain TEXT, reviewed_draft TEXT CHECK(reviewed_draft IS NULL OR json_valid(reviewed_draft)),
 passage_ids TEXT NOT NULL CHECK(json_valid(passage_ids)), created_at TEXT NOT NULL,
 CHECK((kind = 'discarded' AND task_id IS NULL AND task_revision IS NULL AND publication_domain IS NULL) OR
       (kind != 'discarded' AND task_id IS NOT NULL AND task_revision > 0 AND publication_domain IS NOT NULL)),
 UNIQUE(organization_id,candidate_id), UNIQUE(organization_id,id),
 FOREIGN KEY(organization_id,source_id,candidate_id) REFERENCES business_intake_candidate(organization_id,source_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,source_id,source_revision) REFERENCES business_intake_version(organization_id,source_id,revision) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,actor_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,task_id) REFERENCES business_task(organization_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_intake_link (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, source_id TEXT NOT NULL, candidate_id TEXT NOT NULL, decision_id TEXT NOT NULL, task_id TEXT NOT NULL,
 UNIQUE(organization_id,candidate_id,task_id),
 FOREIGN KEY(organization_id,source_id,candidate_id) REFERENCES business_intake_candidate(organization_id,source_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,decision_id) REFERENCES business_intake_decision(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,task_id) REFERENCES business_task(organization_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_intake_operation (
 organization_id TEXT NOT NULL, actor_id TEXT NOT NULL, operation_id TEXT NOT NULL, operation TEXT NOT NULL, digest TEXT NOT NULL,
 result_id TEXT NOT NULL, created_at TEXT NOT NULL, PRIMARY KEY(organization_id,actor_id,operation_id),
 FOREIGN KEY(organization_id,actor_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_intake_audit (
 id TEXT PRIMARY KEY NOT NULL, organization_id TEXT NOT NULL, actor_id TEXT NOT NULL, action TEXT NOT NULL, resource_id TEXT NOT NULL,
 revision INTEGER NOT NULL CHECK(revision > 0), created_at TEXT NOT NULL,
 FOREIGN KEY(organization_id,actor_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_intake_legacy_generation (
 kind TEXT NOT NULL, resource_id TEXT NOT NULL, generation INTEGER NOT NULL CHECK(typeof(generation)='integer' AND generation > 0),
 changing BOOLEAN NOT NULL DEFAULT 0 CHECK(changing IN (0,1)),
 PRIMARY KEY(kind,resource_id)
);
CREATE TRIGGER business_intake_binding_identity BEFORE UPDATE OF id,organization_id,kind,domain,source_owner_id,resource_json,created_at ON business_intake_binding
BEGIN SELECT RAISE(ABORT,'Source binding identity is immutable'); END;
CREATE TRIGGER business_intake_binding_revision BEFORE UPDATE ON business_intake_binding
WHEN NEW.revision <= OLD.revision OR NEW.access_epoch <= OLD.access_epoch
BEGIN SELECT RAISE(ABORT,'Source access must advance'); END;
CREATE TRIGGER business_intake_source_identity BEFORE UPDATE OF id,organization_id,binding_id,kind,external_id ON business_intake_source
BEGIN SELECT RAISE(ABORT,'Source identity is immutable'); END;
CREATE TRIGGER business_intake_candidate_terminal BEFORE UPDATE ON business_intake_candidate WHEN OLD.state != 'pending'
BEGIN SELECT RAISE(ABORT,'Source decision is terminal'); END;
"#).await?;
        // A retained counter fences even configuration change-away-and-back.
        // Never derive authority from a reusable hash of current settings.
        tx.execute_unprepared("INSERT INTO business_intake_legacy_generation(kind,resource_id,generation) SELECT 'email',CAST(id AS TEXT),1 FROM ops_ticket_inbox; INSERT INTO business_intake_legacy_generation(kind,resource_id,generation) SELECT 'hafidh_testflight',product_id,1 FROM ops_intake_host_product;").await?;
        for (table, kind, key, columns, changed) in [
            ("ops_ticket_inbox","email","id","account_id,email_address,channel_type","OLD.account_id IS NOT NEW.account_id OR OLD.email_address IS NOT NEW.email_address OR OLD.channel_type IS NOT NEW.channel_type"),
            ("ops_email_config","email","inbox_id","account_id,credential_ref","OLD.account_id IS NOT NEW.account_id OR OLD.credential_ref IS NOT NEW.credential_ref"),
            ("ops_intake_host_product","hafidh_testflight","product_id","account_id,config_json","OLD.account_id IS NOT NEW.account_id OR OLD.config_json IS NOT NEW.config_json"),
            ("ops_intake_binding","hafidh_testflight","product_id","config_json","OLD.config_json IS NOT NEW.config_json"),
        ] {
            for (event, row, predicate) in [("INSERT","NEW","1"),("DELETE","OLD","1"),("UPDATE","NEW",changed)] {
                let update = if event=="UPDATE" {format!(" OF {columns}")} else {String::new()};
                tx.execute_unprepared(&format!("CREATE TRIGGER business_intake_fence_{table}_{event} AFTER {event}{update} ON {table} WHEN {predicate} BEGIN INSERT INTO business_intake_legacy_generation(kind,resource_id,generation) VALUES('{kind}',CAST({row}.{key} AS TEXT),1) ON CONFLICT(kind,resource_id) DO UPDATE SET generation=generation+1; END;")).await?;
            }
        }
        for (event, row) in [("INSERT", "NEW"), ("DELETE", "OLD"), ("UPDATE", "NEW")] {
            let clause = if event == "UPDATE" {
                " OF parent_id,deleted_at,path"
            } else {
                ""
            };
            let predicate = if event == "UPDATE" {
                " WHEN OLD.parent_id IS NOT NEW.parent_id OR OLD.deleted_at IS NOT NEW.deleted_at OR OLD.path IS NOT NEW.path"
            } else {
                ""
            };
            tx.execute_unprepared(&format!("CREATE TRIGGER business_intake_fence_folder_{event} AFTER {event}{clause} ON folder{predicate} BEGIN UPDATE business_intake_legacy_generation SET generation=generation+1 WHERE kind='hafidh_testflight' AND resource_id IN (SELECT product_id FROM ops_intake_host_product WHERE json_extract(config_json,'$.binding.folder_id')={row}.id); END;")).await?;
        }
        for table in [
            "binding",
            "grant",
            "setup",
            "source",
            "version",
            "passage",
            "candidate",
            "import",
            "import_item",
            "decision",
            "link",
            "operation",
            "audit",
        ] {
            tx.execute_unprepared(&format!("CREATE TRIGGER business_intake_{table}_retain BEFORE DELETE ON business_intake_{table} BEGIN SELECT RAISE(ABORT,'Intake history is retained'); END;")).await?;
        }
        for table in [
            "version",
            "passage",
            "decision",
            "link",
            "operation",
            "audit",
        ] {
            tx.execute_unprepared(&format!("CREATE TRIGGER business_intake_{table}_immutable BEFORE UPDATE ON business_intake_{table} BEGIN SELECT RAISE(ABORT,'Intake history is immutable'); END;")).await?;
        }
        tx.execute_unprepared("CREATE TABLE business_intake_migration (id INTEGER PRIMARY KEY CHECK(id=1)); INSERT INTO business_intake_migration VALUES(1);").await?;
        tx.commit().await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tx = manager.get_connection().begin().await?;
        let row = tx.query_one(sea_orm::Statement::from_string(sea_orm::DbBackend::Sqlite,
            "SELECT (SELECT count(*) FROM business_intake_binding)+(SELECT count(*) FROM business_intake_setup) AS count".to_owned())).await?
            .ok_or_else(|| DbErr::Custom("Cannot inspect intake history".into()))?;
        if row.try_get::<i64>("", "count")? != 0 {
            return Err(DbErr::Custom(
                "Cannot remove retained source history or credentials".into(),
            ));
        }
        for table in [
            "ops_ticket_inbox",
            "ops_email_config",
            "ops_intake_host_product",
            "ops_intake_binding",
            "folder",
        ] {
            for event in ["INSERT", "DELETE", "UPDATE"] {
                tx.execute_unprepared(&format!(
                    "DROP TRIGGER business_intake_fence_{table}_{event}"
                ))
                .await?;
            }
        }
        for table in [
            "audit",
            "operation",
            "link",
            "decision",
            "import_item",
            "import",
            "candidate",
            "passage",
            "version",
            "source",
            "grant",
            "binding",
            "setup",
            "legacy_generation",
            "migration",
        ] {
            tx.execute_unprepared(&format!("DROP TABLE business_intake_{table}"))
                .await?;
        }
        tx.commit().await
    }
}
