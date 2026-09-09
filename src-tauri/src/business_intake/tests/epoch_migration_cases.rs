//! Actual receipt ordering with the immutable pre-tenancy B migration fixture.
use super::super::common::sql;
use crate::{
    business_identity::{self, store, types::*},
    db::migration::Migrator,
};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use sea_orm_migration::{prelude::*, MigratorTrait};

#[path = "fixtures/retained_intake_000011.rs"]
mod retained;
const B: &str = "m20260908_000011_business_intake";
const TENANCY: &str = "m20260908_000012_business_tenancy";
const EPOCH: &str = "m20260909_000013_business_intake_epochs";
const TABLES: [&str; 4] = [
    "business_intake_setup",
    "business_intake_import",
    "business_intake_source",
    "business_intake_candidate",
];

struct OldB;
impl MigrationName for OldB {
    fn name(&self) -> &str {
        B
    }
}
#[async_trait::async_trait]
impl MigrationTrait for OldB {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        retained::Migration.up(m).await
    }
    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        retained::Migration.down(m).await
    }
}
struct BeforeTenancy;
#[async_trait::async_trait]
impl MigratorTrait for BeforeTenancy {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        Migrator::migrations()
            .into_iter()
            .take_while(|m| m.name() != TENANCY)
            .map(|m| {
                if m.name() == B {
                    Box::new(OldB) as Box<dyn MigrationTrait>
                } else {
                    m
                }
            })
            .collect()
    }
}
struct AOnly;
#[async_trait::async_trait]
impl MigratorTrait for AOnly {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        Migrator::migrations()
            .into_iter()
            .filter(|m| m.name() != B && m.name() != EPOCH)
            .collect()
    }
}
async fn number(db: &DatabaseConnection, query: &str) -> i64 {
    db.query_one(sql(query, vec![]))
        .await
        .unwrap()
        .unwrap()
        .try_get_by_index(0)
        .unwrap()
}
async fn columns(db: &DatabaseConnection) -> Vec<i64> {
    let mut result = vec![];
    for table in TABLES {
        result.push(number(db,&format!("SELECT count(*) FROM pragma_table_info('{table}') WHERE name='authorization_epoch'")).await);
    }
    result
}
// Capture the original columns, so adding NULL authority fields is allowed but
// changing any retained value, ID, timestamp or row cardinality is not.
struct Snapshot {
    queries: Vec<String>,
    values: Vec<Vec<String>>,
}
impl Snapshot {
    async fn take(db: &DatabaseConnection, tables: &[&str]) -> Self {
        let mut queries = vec![];
        for table in tables {
            let cols = db
                .query_all(sql(&format!("PRAGMA table_info({table})"), vec![]))
                .await
                .unwrap();
            let names = cols
                .into_iter()
                .map(|r| format!("\"{}\"", r.try_get::<String>("", "name").unwrap()))
                .collect::<Vec<_>>()
                .join(",");
            queries.push(format!(
                "SELECT json_array({names}) AS value FROM {table} ORDER BY rowid"
            ));
        }
        let values = Self::values(db, &queries).await;
        Self { queries, values }
    }
    async fn values(db: &DatabaseConnection, queries: &[String]) -> Vec<Vec<String>> {
        let mut result = vec![];
        for query in queries {
            result.push(
                db.query_all(sql(query, vec![]))
                    .await
                    .unwrap()
                    .into_iter()
                    .map(|r| r.try_get("", "value").unwrap())
                    .collect(),
            );
        }
        result
    }
    async fn unchanged(&self, db: &DatabaseConnection) {
        assert_eq!(Self::values(db, &self.queries).await, self.values);
    }
}
async fn populate_old_b(db: &DatabaseConnection) {
    db.execute_unprepared(r#"
INSERT INTO business_organization VALUES('retained-org',1,'Retained original','then');
INSERT INTO business_member(id,organization_id,display_name,kind,role,domains_json,operator_owner,created_at,updated_at) VALUES('human','retained-org','Retained owner','human','owner','["feedback"]',1,'then','then');
INSERT INTO business_credential VALUES('credential','retained-org','human','aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa','retained','then',NULL);
INSERT INTO business_identity_event VALUES('identity-event','retained-org','human','retained','human','then');
INSERT INTO business_task(id,organization_id,title,notes,domain,owner_id,creator_id,created_at,updated_at) VALUES('task','retained-org','Reviewed task','Public text only','feedback','human','human','then','then');
INSERT INTO business_task_activity(id,organization_id,task_id,revision,kind,actor_id,actor_name,actor_kind,payload_json,created_at) VALUES('activity','retained-org','task',1,'created','human','Retained owner','human','{}','then');
INSERT INTO business_intake_setup(id,organization_id,actor_id,operation_id,digest,binding_id,owner_authority_revision,plan_json,credential_ref,state,expires_at,created_at) VALUES('setup','retained-org','human','operation','digest','binding',1,'{"domain":"feedback"}','synthetic-retained-reference','active','2099-01-01T00:00:00Z','then');
INSERT INTO business_intake_binding(id,organization_id,kind,label,domain,source_owner_id,owner_authority_revision,resource_json,credential_ref,publication_domains,retained_task_text,created_at,updated_at) VALUES('binding','retained-org','fireflies','Retained meetings','feedback','human',1,'{"kind":"fireflies","providerUserId":"synthetic-provider","mine":true}','synthetic-retained-reference','["feedback"]',1,'then','then');
INSERT INTO business_intake_grant(id,organization_id,binding_id,member_id,revision,state,scope,can_read,can_import,can_triage,publication_domains) VALUES('grant','retained-org','binding','human',1,'active','binding_current_and_future_sources',1,1,1,'["feedback"]');
INSERT INTO business_intake_source(id,organization_id,binding_id,kind,external_id,title) VALUES('source','retained-org','binding','fireflies','meeting','Private title');
INSERT INTO business_intake_version(id,organization_id,source_id,revision,digest,normalization_version,observed_at,title) VALUES('version','retained-org','source',1,'digest',1,'then','Private title');
UPDATE business_intake_source SET revision=1,access='fresh',observed_epoch=1,access_until='2099-01-01T00:00:00Z',seeded=1 WHERE id='source';
INSERT INTO business_intake_passage(id,organization_id,source_id,source_revision,ordinal,kind,text) VALUES('passage','retained-org','source',1,0,'sentence','Retained private passage');
INSERT INTO business_intake_candidate(id,organization_id,source_id,revision,source_revision,prepared_epoch,state,origin,passage_ids,draft_json,created_by,updated_by,created_at,updated_at) VALUES('candidate','retained-org','source',1,1,1,'accepted','source_review','["passage"]','{"notes":"Public text only"}','human','human','then','then');
INSERT INTO business_intake_import(id,organization_id,binding_id,requester_id,state,coverage,selection_json,attempt_id,attempt_actor_id,lease_until,attempt_count,attempt_epoch,created_at,updated_at) VALUES('import','retained-org','binding','human','running','partial','{"kind":"record","sourceId":"source"}','attempt','human','2099-01-01T00:00:00Z',1,1,'then','then');
INSERT INTO business_intake_import_item VALUES('retained-org','binding','import','source','pending');
INSERT INTO business_intake_decision(id,organization_id,candidate_id,source_id,from_revision,source_revision,kind,actor_id,task_id,task_revision,publication_domain,reviewed_draft,passage_ids,created_at) VALUES('decision','retained-org','candidate','source',1,1,'accepted','human','task',1,'feedback','{"notes":"Public text only"}','["passage"]','then');
INSERT INTO business_intake_link VALUES('link','retained-org','source','candidate','decision','task');
INSERT INTO business_intake_operation VALUES('retained-org','human','receipt','candidates/accept','digest','decision','then');
INSERT INTO business_intake_audit VALUES('audit','retained-org','human','candidate_accepted','candidate',1,'then');
"#).await.unwrap();
}
const RETAINED: [&str; 19] = [
    "business_organization",
    "business_member",
    "business_credential",
    "business_identity_event",
    "business_task",
    "business_task_activity",
    "business_intake_binding",
    "business_intake_setup",
    "business_intake_grant",
    "business_intake_source",
    "business_intake_version",
    "business_intake_passage",
    "business_intake_candidate",
    "business_intake_import",
    "business_intake_import_item",
    "business_intake_decision",
    "business_intake_link",
    "business_intake_operation",
    "business_intake_audit",
];

#[tokio::test]
async fn intake_epoch_migration_old_populated_b_then_tenancy_and_atomic_epoch_rollback() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    BeforeTenancy::up(&db, None).await.unwrap();
    populate_old_b(&db).await;
    let retained = Snapshot::take(&db, &RETAINED).await;
    assert_eq!(columns(&db).await, vec![0, 0, 0, 0]);
    // Real Migrator commits000012 before000013. Fail the latter after earlier
    // nullable-column ALTERs; the transaction must roll those ALTERs back.
    db.execute_unprepared("ALTER TABLE business_intake_candidate RENAME TO held_candidate; CREATE VIEW business_intake_candidate AS SELECT * FROM held_candidate;").await.unwrap();
    assert!(Migrator::up(&db, None).await.is_err());
    assert_eq!(columns(&db).await, vec![0, 0, 0, 0]);
    assert_eq!(
        number(
            &db,
            &format!("SELECT count(*) FROM seaql_migrations WHERE version='{TENANCY}'")
        )
        .await,
        1
    );
    assert_eq!(
        number(
            &db,
            &format!("SELECT count(*) FROM seaql_migrations WHERE version='{EPOCH}'")
        )
        .await,
        0
    );
    db.execute_unprepared("DROP VIEW business_intake_candidate; ALTER TABLE held_candidate RENAME TO business_intake_candidate;").await.unwrap();
    retained.unchanged(&db).await;
    Migrator::up(&db, None).await.unwrap();
    retained.unchanged(&db).await;
    assert_eq!(columns(&db).await, vec![1, 1, 1, 1]);
    for table in TABLES {
        assert_eq!(
            number(
                &db,
                &format!("SELECT count(*) FROM {table} WHERE authorization_epoch IS NOT NULL")
            )
            .await,
            0
        );
    }
    assert!(db
        .query_all(sql("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
    assert_eq!(number(&db, "PRAGMA foreign_keys").await, 1);
    assert!(Migrator::down(&db, Some(1)).await.is_err());
    retained.unchanged(&db).await;
}

#[tokio::test]
async fn intake_epoch_migration_already_recorded_a_tenancy_then_b_and_b_receipt_retry() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    AOnly::up(&db, None).await.unwrap();
    store::bootstrap(
        &db,
        BootstrapInput {
            organization_name: "Existing A-only tenant".into(),
            owner_name: "Existing owner".into(),
        },
    )
    .await
    .unwrap();
    let op = business_identity::operator_principal(&db).await.unwrap();
    crate::business_tasks::store::create(
        &db,
        &crate::business_tasks::ActorContext::authenticated(op),
        serde_json::from_value(
            serde_json::json!({"title":"Retained human work","domain":"feedback"}),
        )
        .unwrap(),
    )
    .await
    .unwrap();
    let retained = Snapshot::take(&db, &RETAINED[..6]).await;
    let receipt = number(
        &db,
        &format!("SELECT applied_at FROM seaql_migrations WHERE version='{TENANCY}'"),
    )
    .await;
    db.execute_unprepared(&format!("CREATE TRIGGER reject_b_receipt BEFORE INSERT ON seaql_migrations WHEN NEW.version='{B}' BEGIN SELECT RAISE(ABORT,'synthetic receipt failure'); END;")).await.unwrap();
    assert!(Migrator::up(&db, None).await.is_err());
    assert_eq!(
        number(&db, "SELECT count(*) FROM business_intake_migration").await,
        1
    );
    assert_eq!(
        number(
            &db,
            &format!("SELECT count(*) FROM seaql_migrations WHERE version='{B}'")
        )
        .await,
        0
    );
    db.execute_unprepared("DROP TRIGGER reject_b_receipt")
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();
    assert_eq!(columns(&db).await, vec![1, 1, 1, 1]);
    assert_eq!(
        number(
            &db,
            &format!("SELECT applied_at FROM seaql_migrations WHERE version='{TENANCY}'")
        )
        .await,
        receipt
    );
    retained.unchanged(&db).await;
    assert!(Migrator::get_pending_migrations(&db)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn intake_epoch_migration_schema_commit_receipt_retry_preserves_null_and_captured_values() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    BeforeTenancy::up(&db, None).await.unwrap();
    populate_old_b(&db).await;
    let retained = Snapshot::take(&db, &RETAINED).await;
    db.execute_unprepared(&format!("CREATE TRIGGER reject_epoch_receipt BEFORE INSERT ON seaql_migrations WHEN NEW.version='{EPOCH}' BEGIN SELECT RAISE(ABORT,'synthetic receipt failure'); END;")).await.unwrap();
    assert!(Migrator::up(&db, None).await.is_err());
    assert_eq!(columns(&db).await, vec![1, 1, 1, 1]);
    retained.unchanged(&db).await;
    assert_eq!(
        number(
            &db,
            &format!("SELECT count(*) FROM seaql_migrations WHERE version='{EPOCH}'")
        )
        .await,
        0
    );
    // A later captured value must survive schema receipt reconciliation too.
    db.execute_unprepared("UPDATE business_intake_source SET authorization_epoch=73 WHERE id='source'; DROP TRIGGER reject_epoch_receipt;").await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    assert_eq!(
        number(
            &db,
            "SELECT authorization_epoch FROM business_intake_source WHERE id='source'"
        )
        .await,
        73
    );
    for table in [
        "business_intake_setup",
        "business_intake_import",
        "business_intake_candidate",
    ] {
        assert_eq!(
            number(
                &db,
                &format!("SELECT count(*) FROM {table} WHERE authorization_epoch IS NOT NULL")
            )
            .await,
            0
        );
    }
    retained.unchanged(&db).await;
    Migrator::up(&db, None).await.unwrap();
    assert_eq!(
        number(
            &db,
            &format!("SELECT count(*) FROM seaql_migrations WHERE version='{EPOCH}'")
        )
        .await,
        1
    );
    assert!(db
        .query_all(sql("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn intake_epoch_migration_fresh_combined_has_no_default_authority() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::up(&db, None).await.unwrap();
    let names = Migrator::migrations()
        .into_iter()
        .map(|m| m.name().to_owned())
        .collect::<Vec<_>>();
    assert!(names.iter().position(|n| n == B) < names.iter().position(|n| n == TENANCY));
    assert!(names.iter().position(|n| n == TENANCY) < names.iter().position(|n| n == EPOCH));
    for table in TABLES {
        assert_eq!(number(&db,&format!("SELECT count(*) FROM pragma_table_info('{table}') WHERE name='authorization_epoch' AND dflt_value IS NULL AND [notnull]=0")).await,1);
    }
    assert_eq!(number(&db, "PRAGMA foreign_keys").await, 1);
}
