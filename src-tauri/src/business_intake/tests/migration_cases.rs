use super::{super::*, support::*};
use crate::{
    business_tasks::{self, ActorContext},
    db::migration::Migrator,
};
use sea_orm::ConnectionTrait;
use sea_orm_migration::{MigratorTrait, SchemaManager};
use serde_json::json;

#[tokio::test]
async fn intake_populated_upgrade_retains_tasks_and_tickets_and_ddl_failure_is_atomic() {
    let f = Fixture::new().await;
    let task=business_tasks::store::create(&f.db.conn,&ActorContext::authenticated(f.op.clone()),serde_json::from_value(json!({"title":"Retained human task","notes":"Exact business notes","domain":"feedback","dueDate":"2026-09-09"})).unwrap()).await.unwrap();
    let (op, key) = crate::ops::tests::seed(&f.db, 27).await;
    let before = crate::ops::store::thread(&f.db.conn, &op, key)
        .await
        .unwrap();
    let migration = Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260908_000011_business_intake")
        .unwrap();
    let manager = SchemaManager::new(&f.db.conn);
    migration.down(&manager).await.unwrap();
    f.db.conn
        .execute_unprepared("CREATE TABLE business_intake_candidate(id INTEGER PRIMARY KEY)")
        .await
        .unwrap();
    assert!(migration.up(&manager).await.is_err());
    assert!(!manager.has_table("business_intake_binding").await.unwrap());
    assert!(manager
        .has_table("business_intake_candidate")
        .await
        .unwrap());
    f.db.conn
        .execute_unprepared("DROP TABLE business_intake_candidate")
        .await
        .unwrap();
    migration.up(&manager).await.unwrap();
    Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260909_000013_business_intake_epochs")
        .unwrap()
        .up(&manager)
        .await
        .unwrap();
    let after = business_tasks::store::get(
        &f.db.conn,
        &ActorContext::authenticated(f.op.clone()),
        business_tasks::types::TaskInput {
            task_id: task.task.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        serde_json::to_value(after).unwrap(),
        serde_json::to_value(task).unwrap()
    );
    assert_eq!(
        serde_json::to_value(
            crate::ops::store::thread(&f.db.conn, &op, key)
                .await
                .unwrap()
        )
        .unwrap(),
        serde_json::to_value(before).unwrap()
    );
    f.binding().await;
    assert!(migration.down(&manager).await.is_err());
    assert!(manager.has_table("business_intake_binding").await.unwrap());
}

#[tokio::test]
async fn intake_existing_member_from_another_organization_cannot_become_source_owner() {
    // Accepted v1 has one org/backend. Use an actual second org/database rather
    // than disabling that production constraint to fake a cross-org fixture.
    let f = Fixture::new().await;
    let other = Fixture::new().await;
    let mut input = f.create_input(&common::id());
    input.source_owner_id = other.op.member_id().into();
    assert!(bindings_create(&f.db.conn, &f.op, &f.services, None, input)
        .await
        .is_err());
    assert_eq!(count(&f.db.conn, "business_intake_setup").await, 0);
    assert_eq!(f.mock.count(), 0);
    assert!(f.secrets.values.lock().unwrap().is_empty());
}
