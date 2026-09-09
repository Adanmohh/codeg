-- Apache Codeg migration000010 deliverable columns/FKs, retained exactly except
-- the body lower bound. Text-only input validation remains required in its API.
CREATE TABLE business_task_deliverable_new (
 id TEXT PRIMARY KEY NOT NULL,
 organization_id TEXT NOT NULL,
 task_id TEXT NOT NULL,
 revision INTEGER NOT NULL CHECK(revision > 0),
 author_id TEXT NOT NULL,
 author_name TEXT NOT NULL,
 author_kind TEXT NOT NULL CHECK(author_kind IN ('human','agent')),
 body TEXT NOT NULL CHECK(length(trim(body)) BETWEEN 0 AND 20000),
 execution_id TEXT,
 created_at TEXT NOT NULL,
 UNIQUE(organization_id,task_id,id),
 FOREIGN KEY(organization_id,task_id) REFERENCES business_task(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,author_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,task_id,execution_id) REFERENCES business_task_execution(organization_id,task_id,id) ON DELETE RESTRICT
);
INSERT INTO business_task_deliverable_new SELECT * FROM business_task_deliverable;
DROP TABLE business_task_deliverable;
ALTER TABLE business_task_deliverable_new RENAME TO business_task_deliverable;
CREATE INDEX business_task_deliverable_history ON business_task_deliverable(organization_id,task_id,revision);
CREATE TRIGGER business_task_deliverable_no_update BEFORE UPDATE ON business_task_deliverable
 BEGIN SELECT RAISE(ABORT,'Business task deliverables are immutable'); END;
CREATE TRIGGER business_task_deliverable_no_delete BEFORE DELETE ON business_task_deliverable
 BEGIN SELECT RAISE(ABORT,'Business task deliverables are immutable'); END;

CREATE TABLE business_execution_task_scope (
 organization_id TEXT NOT NULL,
 task_id TEXT NOT NULL,
 epoch INTEGER NOT NULL CHECK(epoch > 0),
 PRIMARY KEY(organization_id,task_id),
 FOREIGN KEY(organization_id,task_id) REFERENCES business_task(organization_id,id) ON DELETE RESTRICT
);
INSERT INTO business_execution_task_scope SELECT organization_id,id,1 FROM business_task;
CREATE TRIGGER business_execution_task_created AFTER INSERT ON business_task
 BEGIN INSERT INTO business_execution_task_scope VALUES(NEW.organization_id,NEW.id,1); END;

-- Discovered from an authorized existing client; no provider secret or arbitrary
-- launch command is stored here or accepted by a wire DTO.
CREATE TABLE business_execution_profile (
 id TEXT PRIMARY KEY NOT NULL,
 organization_id TEXT NOT NULL,
 member_id TEXT NOT NULL,
 client_id TEXT NOT NULL,
 config_key TEXT NOT NULL,
 config_hash TEXT NOT NULL CHECK(length(config_hash)=64),
 revision INTEGER NOT NULL CHECK(revision > 0),
 summary_json TEXT NOT NULL CHECK(json_valid(summary_json)),
 retired_at TEXT,
 UNIQUE(organization_id,member_id,id),
 UNIQUE(organization_id,member_id,client_id,config_key),
 FOREIGN KEY(organization_id,member_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_execution_session (
 id TEXT PRIMARY KEY NOT NULL,
 organization_id TEXT NOT NULL,
 task_id TEXT NOT NULL,
 member_id TEXT NOT NULL,
 authority_json TEXT NOT NULL CHECK(json_valid(authority_json)),
 authorization_epoch INTEGER NOT NULL CHECK(authorization_epoch > 0),
 task_scope_epoch INTEGER NOT NULL CHECK(task_scope_epoch > 0),
 profile_id TEXT NOT NULL,
 profile_revision INTEGER NOT NULL CHECK(profile_revision > 0),
 revision INTEGER NOT NULL CHECK(revision > 0),
 generation INTEGER NOT NULL CHECK(generation > 0),
 mode TEXT NOT NULL CHECK(mode IN ('chat','terminal')),
 status TEXT NOT NULL CHECK(status IN ('starting','idle','running','awaiting_input','stopped','failed','interrupted','revoked','closed')),
 title TEXT NOT NULL CHECK(length(title) BETWEEN 1 AND 240),
 created_at TEXT NOT NULL,
 updated_at TEXT NOT NULL,
 last_activity_at TEXT NOT NULL,
 UNIQUE(organization_id,id),
 UNIQUE(organization_id,task_id,member_id,id),
 FOREIGN KEY(organization_id,task_id) REFERENCES business_task(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,member_id,profile_id) REFERENCES business_execution_profile(organization_id,member_id,id) ON DELETE RESTRICT
);
CREATE INDEX business_execution_task_sessions ON business_execution_session(organization_id,task_id,member_id,updated_at,id);
CREATE TRIGGER business_execution_session_identity BEFORE UPDATE OF id,organization_id,task_id,member_id,authority_json,authorization_epoch,task_scope_epoch,profile_id,profile_revision,mode,created_at ON business_execution_session
 BEGIN SELECT RAISE(ABORT,'Execution authority is immutable'); END;
CREATE TRIGGER business_execution_session_fence BEFORE UPDATE OF revision,generation,status ON business_execution_session
 WHEN NEW.revision <= OLD.revision OR NEW.generation < OLD.generation OR (OLD.status IN ('revoked','closed') AND NEW.status != OLD.status)
 BEGIN SELECT RAISE(ABORT,'Execution revision and revocation are monotonic'); END;
CREATE TRIGGER business_execution_scope_changed AFTER UPDATE OF owner_id,assignee_id,reviewer_id,domain,status,archived_at ON business_task
 WHEN NEW.owner_id IS NOT OLD.owner_id OR NEW.assignee_id IS NOT OLD.assignee_id OR NEW.reviewer_id IS NOT OLD.reviewer_id OR NEW.domain IS NOT OLD.domain OR NEW.archived_at IS NOT OLD.archived_at OR (NEW.status != OLD.status AND (NEW.status IN ('done','cancelled') OR OLD.status IN ('done','cancelled')))
 BEGIN
  UPDATE business_execution_task_scope SET epoch=epoch+1 WHERE organization_id=NEW.organization_id AND task_id=NEW.id;
  UPDATE business_execution_session SET status='revoked',generation=generation+1,revision=revision+1,updated_at=NEW.updated_at
   WHERE organization_id=NEW.organization_id AND task_id=NEW.id AND status NOT IN ('revoked','closed');
 END;
CREATE TRIGGER business_execution_profile_changed AFTER UPDATE OF config_hash,revision,retired_at ON business_execution_profile
 WHEN NEW.config_hash != OLD.config_hash OR NEW.revision != OLD.revision OR NEW.retired_at IS NOT OLD.retired_at
 BEGIN UPDATE business_execution_session SET status='revoked',generation=generation+1,revision=revision+1
  WHERE profile_id=NEW.id AND status NOT IN ('revoked','closed'); END;
CREATE TRIGGER business_execution_member_changed AFTER UPDATE OF role,domains_json,status,operator_owner ON business_member
 WHEN NEW.role IS NOT OLD.role OR NEW.domains_json IS NOT OLD.domains_json OR NEW.status IS NOT OLD.status OR NEW.operator_owner IS NOT OLD.operator_owner
 BEGIN UPDATE business_execution_session SET status='revoked',generation=generation+1,revision=revision+1
  WHERE organization_id=NEW.organization_id AND member_id=NEW.id AND status NOT IN ('revoked','closed'); END;
CREATE TRIGGER business_execution_tenant_changed AFTER UPDATE OF status,authorization_epoch ON business_organization
 WHEN NEW.status IS NOT OLD.status OR NEW.authorization_epoch IS NOT OLD.authorization_epoch
 BEGIN UPDATE business_execution_session SET status='revoked',generation=generation+1,revision=revision+1
  WHERE organization_id=NEW.id AND status NOT IN ('revoked','closed'); END;

CREATE TABLE business_execution_generation (
 organization_id TEXT NOT NULL,
 session_id TEXT NOT NULL,
 generation INTEGER NOT NULL CHECK(generation > 0),
 admission_id TEXT NOT NULL UNIQUE,
 initial_cursor TEXT NOT NULL UNIQUE,
 engine_json TEXT CHECK(engine_json IS NULL OR json_valid(engine_json)),
 created_at TEXT NOT NULL,
 PRIMARY KEY(organization_id,session_id,generation),
 FOREIGN KEY(organization_id,session_id) REFERENCES business_execution_session(organization_id,id) ON DELETE RESTRICT
);
CREATE TABLE business_execution_operation (
 operation_id TEXT NOT NULL,
 organization_id TEXT NOT NULL,
 member_id TEXT NOT NULL,
 kind TEXT NOT NULL CHECK(kind IN ('start','continue','prompt','attach','stop','terminal_write','import_output','submit')),
 authority_json TEXT NOT NULL CHECK(json_valid(authority_json)),
 authorization_epoch INTEGER NOT NULL CHECK(authorization_epoch > 0),
 input_hash TEXT NOT NULL CHECK(length(input_hash)=64),
 input_json TEXT NOT NULL CHECK(json_valid(input_json)),
 task_id TEXT NOT NULL,
 session_id TEXT,
 generation INTEGER,
 status TEXT NOT NULL CHECK(status IN ('pending','confirmed','failed','uncertain')),
 reason TEXT,
 resource_id TEXT,
 result_json TEXT CHECK(result_json IS NULL OR json_valid(result_json)),
 created_at TEXT NOT NULL,
 updated_at TEXT NOT NULL,
 PRIMARY KEY(organization_id,member_id,kind,operation_id),
 FOREIGN KEY(organization_id,member_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,task_id) REFERENCES business_task(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,task_id,member_id,session_id) REFERENCES business_execution_session(organization_id,task_id,member_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,session_id,generation) REFERENCES business_execution_generation(organization_id,session_id,generation) ON DELETE RESTRICT
);
CREATE TRIGGER business_execution_operation_identity BEFORE UPDATE OF operation_id,organization_id,member_id,kind,authority_json,authorization_epoch,input_hash,input_json,task_id,session_id,generation,created_at ON business_execution_operation
 BEGIN SELECT RAISE(ABORT,'Operation identity is immutable'); END;
CREATE TRIGGER business_execution_operation_terminal BEFORE UPDATE ON business_execution_operation
 WHEN OLD.status IN ('confirmed','failed')
 BEGIN SELECT RAISE(ABORT,'Completed operation receipt is immutable'); END;
CREATE TRIGGER business_execution_operation_retained BEFORE DELETE ON business_execution_operation
 BEGIN SELECT RAISE(ABORT,'Operation receipts are retained'); END;

CREATE TABLE business_execution_event (
 organization_id TEXT NOT NULL,
 session_id TEXT NOT NULL,
 generation INTEGER NOT NULL,
 sequence INTEGER NOT NULL CHECK(sequence > 0),
 cursor TEXT NOT NULL UNIQUE,
 event_json TEXT NOT NULL CHECK(json_valid(event_json) AND length(CAST(event_json AS BLOB)) <= 1048576),
 created_at TEXT NOT NULL,
 PRIMARY KEY(organization_id,session_id,generation,sequence),
 FOREIGN KEY(organization_id,session_id,generation) REFERENCES business_execution_generation(organization_id,session_id,generation) ON DELETE RESTRICT
);
CREATE TABLE business_execution_output (
 id TEXT PRIMARY KEY NOT NULL,
 organization_id TEXT NOT NULL,
 session_id TEXT NOT NULL,
 generation INTEGER NOT NULL,
 revision INTEGER NOT NULL CHECK(revision > 0),
 relative_path TEXT NOT NULL,
 metadata_json TEXT NOT NULL CHECK(json_valid(metadata_json)),
 UNIQUE(organization_id,session_id,id),
 UNIQUE(organization_id,session_id,generation,relative_path),
 FOREIGN KEY(organization_id,session_id,generation) REFERENCES business_execution_generation(organization_id,session_id,generation) ON DELETE RESTRICT
);
CREATE TABLE business_execution_asset (
 id TEXT PRIMARY KEY NOT NULL,
 organization_id TEXT NOT NULL,
 task_id TEXT NOT NULL,
 member_id TEXT NOT NULL,
 revision INTEGER NOT NULL CHECK(revision > 0),
 title TEXT NOT NULL CHECK(length(trim(title)) BETWEEN 1 AND 240),
 media_type TEXT NOT NULL,
 latest_version_id TEXT,
 created_at TEXT NOT NULL,
 updated_at TEXT NOT NULL,
 UNIQUE(organization_id,task_id,id),
 FOREIGN KEY(organization_id,task_id) REFERENCES business_task(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,member_id) REFERENCES business_member(organization_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,task_id,id,latest_version_id) REFERENCES business_execution_asset_version(organization_id,task_id,asset_id,id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED
);
CREATE TABLE business_execution_asset_version (
 id TEXT PRIMARY KEY NOT NULL,
 organization_id TEXT NOT NULL,
 task_id TEXT NOT NULL,
 asset_id TEXT NOT NULL,
 version INTEGER NOT NULL CHECK(version > 0),
 title TEXT NOT NULL CHECK(length(trim(title)) BETWEEN 1 AND 240),
 sha256 TEXT NOT NULL CHECK(length(sha256)=64),
 byte_size INTEGER NOT NULL CHECK(byte_size BETWEEN 0 AND 52428800),
 media_type TEXT NOT NULL,
 object_id TEXT NOT NULL UNIQUE,
 created_at TEXT NOT NULL,
 created_by TEXT NOT NULL,
 producer_session_id TEXT NOT NULL,
 producer_turn_id TEXT,
 producer_client_id TEXT NOT NULL,
 producer_model TEXT,
 UNIQUE(organization_id,task_id,asset_id,id),
 UNIQUE(organization_id,asset_id,version),
 FOREIGN KEY(organization_id,task_id,asset_id) REFERENCES business_execution_asset(organization_id,task_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,task_id,created_by,producer_session_id) REFERENCES business_execution_session(organization_id,task_id,member_id,id) ON DELETE RESTRICT
);
CREATE TRIGGER business_execution_version_immutable BEFORE UPDATE ON business_execution_asset_version
 BEGIN SELECT RAISE(ABORT,'Managed versions are immutable'); END;
CREATE TRIGGER business_execution_version_retained BEFORE DELETE ON business_execution_asset_version
 BEGIN SELECT RAISE(ABORT,'Managed versions are retained'); END;
CREATE TABLE business_execution_publication (
 organization_id TEXT NOT NULL,
 task_id TEXT NOT NULL,
 deliverable_id TEXT NOT NULL,
 asset_id TEXT NOT NULL,
 version_id TEXT NOT NULL,
 ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 15),
 authority_kind TEXT NOT NULL CHECK(authority_kind IN ('operator','credential')),
 PRIMARY KEY(organization_id,task_id,deliverable_id,asset_id,version_id),
 UNIQUE(organization_id,task_id,deliverable_id,ordinal),
 FOREIGN KEY(organization_id,task_id,deliverable_id) REFERENCES business_task_deliverable(organization_id,task_id,id) ON DELETE RESTRICT,
 FOREIGN KEY(organization_id,task_id,asset_id,version_id) REFERENCES business_execution_asset_version(organization_id,task_id,asset_id,id) ON DELETE RESTRICT
);
CREATE TRIGGER business_execution_publication_immutable BEFORE UPDATE ON business_execution_publication
 BEGIN SELECT RAISE(ABORT,'Selected task versions are immutable'); END;
CREATE TRIGGER business_execution_publication_retained BEFORE DELETE ON business_execution_publication
 BEGIN SELECT RAISE(ABORT,'Selected task versions are retained'); END;
-- Marker and all DDL/data changes commit together; a missing SeaORM receipt can
-- safely retry without rebuilding or inventing captured authority on old rows.
CREATE TABLE business_execution_metadata (id INTEGER PRIMARY KEY CHECK(id=1));
INSERT INTO business_execution_metadata VALUES(1);
