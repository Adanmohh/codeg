use super::types::*;
use serde_json::json;
mod candidate_cases;
mod epoch_migration_cases;
mod import_cases;
mod legacy_cases;
mod migration_cases;
mod reader_cases;
mod recovery_cases;
mod setup_cases;
mod support;
mod tenancy_cases;
mod tenant_http_cases;

#[test]
fn intake_closed_inputs_reject_identity_queries_secret_refs_and_null_replacement() {
    let base = json!({"operationId":"00000000-0000-4000-8000-000000000001","label":"Meetings","domain":"feedback","sourceOwnerId":"00000000-0000-4000-8000-000000000002",
        "source":{"kind":"fireflies","apiKey":"synthetic-fixture"},"publicationDomains":[],"retainedTaskText":false});
    assert!(serde_json::from_value::<CreateBindingInput>(base.clone()).is_ok());
    for field in [
        "organizationId",
        "actorId",
        "role",
        "credentialRef",
        "endpoint",
        "query",
    ] {
        let mut invalid = base.clone();
        invalid[field] = json!("forged");
        assert!(serde_json::from_value::<CreateBindingInput>(invalid).is_err());
        let mut invalid = base.clone();
        invalid["source"][field] = json!("forged");
        assert!(serde_json::from_value::<CreateBindingInput>(invalid).is_err());
    }
    for key in ["", "bad\r\nHeader:value", "with space"] {
        let mut invalid = base.clone();
        invalid["source"]["apiKey"] = json!(key);
        assert!(serde_json::from_value::<CreateBindingInput>(invalid).is_err());
    }
    let mut update = json!({"operationId":"00000000-0000-4000-8000-000000000001","bindingId":"00000000-0000-4000-8000-000000000002","expectedRevision":1,"label":"Meetings","enabled":true,"publicationDomains":[],"retainedTaskText":false});
    assert!(serde_json::from_value::<UpdateBindingInput>(update.clone()).is_ok());
    update["credential"] = Value::Null;
    assert!(serde_json::from_value::<UpdateBindingInput>(update).is_err());
    use serde_json::Value;
}

#[tokio::test]
async fn intake_migration_retains_task_rows_and_rejects_foreign_member_bindings() {
    use crate::{
        business_identity::{self, store, types::BootstrapInput},
        db::test_helpers::fresh_in_memory_db,
    };
    use sea_orm::{ConnectionTrait, DbBackend, Statement};
    let db = fresh_in_memory_db().await;
    store::bootstrap(
        &db.conn,
        BootstrapInput {
            organization_name: "Synthetic".into(),
            owner_name: "Owner".into(),
        },
    )
    .await
    .unwrap();
    let p = business_identity::operator_principal(&db.conn)
        .await
        .unwrap();
    let rejected = db.conn.execute(Statement::from_sql_and_values(DbBackend::Sqlite,
        "INSERT INTO business_intake_binding(id,organization_id,kind,label,domain,source_owner_id,owner_authority_revision,resource_json,publication_domains,retained_task_text,created_at,updated_at) VALUES(?,?,'email','Inbox','feedback',?,1,'{}','[]',0,'now','now')",
        [uuid::Uuid::new_v4().to_string().into(), p.organization_id().into(),uuid::Uuid::new_v4().to_string().into()])).await;
    assert!(rejected.is_err());
    let row = db
        .conn
        .query_one(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT count(*) AS count FROM business_intake_binding".to_owned(),
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<i64>("", "count").unwrap(), 0);
    let tables = db.conn.query_one(Statement::from_string(DbBackend::Sqlite,"SELECT count(*) AS count FROM sqlite_master WHERE type='table' AND name IN ('business_task','ops_ticket_message','business_intake_binding','business_intake_import','business_intake_decision')".to_owned())).await.unwrap().unwrap();
    assert_eq!(tables.try_get::<i64>("", "count").unwrap(), 5);
}
