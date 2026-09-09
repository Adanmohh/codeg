//! Real14->15 schema/receipt paths with synthetic admitted generation and files.
//! No installed fixture is migrated and no engine is started.
use super::*;
use crate::{
    business_execution::{session_store, session_tests},
    db::migration::Migrator,
};
use sea_orm::Database;
use sea_orm_migration::{MigratorTrait, SchemaManager};

const MIGRATION: &str = "m20260909_000015_business_execution_scans";
struct Case {
    _directory: tempfile::TempDir,
    db: DatabaseConnection,
    principal: Principal,
    files: files::Files,
    session_id: String,
    output_id: String,
}
async fn before_scan_migration() -> Case {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    let index = Migrator::migrations()
        .iter()
        .position(|m| m.name() == MIGRATION)
        .unwrap();
    Migrator::up(&db, Some(index as u32)).await.unwrap();
    let (principal, task) = crate::business_execution::tests::task(&db).await;
    let profile = session_store::sync_profiles(&db, &principal, vec![session_tests::profile("a")])
        .await
        .unwrap()
        .profiles
        .remove(0);
    let reserved =
        session_store::reserve_start(&db, &principal, &session_tests::start(&task, &profile))
            .await
            .unwrap();
    let session_id = reserved.result.session.id;
    session_store::complete_launch(&db, &reserved.admission.unwrap(), &session_tests::link())
        .await
        .unwrap();
    let tx = db.begin().await.unwrap();
    let generation = session_store::generation(&tx, &principal, &session_id, 1)
        .await
        .unwrap();
    tx.rollback().await.unwrap();
    let directory = tempfile::tempdir().unwrap();
    let files = files::Files::new(directory.path().into());
    let workspace = files.workspace(&generation.admission_id).unwrap();
    std::fs::write(workspace.join("retained.md"), b"Existing generation output").unwrap();
    let candidate = files.scan(&generation.admission_id).unwrap().remove(0);
    let output_id = id();
    db.execute(statement(
        "INSERT INTO business_execution_output(id,organization_id,session_id,generation,revision,relative_path,metadata_json) VALUES(?,?,?,1,7,?,?)",
        vec![output_id.clone().into(), principal.organization_id().into(), session_id.clone().into(), candidate.relative.into(), encode(&Observed { available: true, file: candidate.observation }).unwrap().into()],
    )).await.unwrap();
    Case {
        _directory: directory,
        db,
        principal,
        files,
        session_id,
        output_id,
    }
}
async fn retained(db: &DatabaseConnection) -> Vec<Vec<String>> {
    let mut snapshot = vec![];
    for table in [
        "business_task",
        "business_task_activity",
        "business_task_deliverable",
        "business_execution_task_scope",
        "business_execution_profile",
        "business_execution_session",
        "business_execution_generation",
        "business_execution_output",
        "business_execution_operation",
        "business_execution_asset",
        "business_execution_asset_version",
        "business_execution_publication",
    ] {
        let columns = db
            .query_all(statement(&format!("PRAGMA table_info({table})"), vec![]))
            .await
            .unwrap()
            .into_iter()
            .map(|r| format!("\"{}\"", r.try_get::<String>("", "name").unwrap()))
            .collect::<Vec<_>>()
            .join(",");
        snapshot.push(
            db.query_all(statement(
                &format!("SELECT json_array({columns}) AS value FROM {table} ORDER BY rowid"),
                vec![],
            ))
            .await
            .unwrap()
            .into_iter()
            .map(|r| r.try_get("", "value").unwrap())
            .collect(),
        );
    }
    snapshot
}
async fn receipts(db: &DatabaseConnection) -> Vec<String> {
    db.query_all(statement(
        "SELECT json_array(version,applied_at) AS value FROM seaql_migrations ORDER BY version",
        vec![],
    ))
    .await
    .unwrap()
    .into_iter()
    .map(|r| r.try_get("", "value").unwrap())
    .collect()
}
async fn count(case: &Case, generation: i64) -> Option<i64> {
    case.db.query_one(statement("SELECT accepted_scan FROM business_execution_scan WHERE organization_id=? AND session_id=? AND generation=?",
        vec![case.principal.organization_id().into(), case.session_id.clone().into(), generation.into()])).await.unwrap()
        .map(|r| r.try_get("", "accepted_scan").unwrap())
}

#[tokio::test]
async fn execution_outputs_migration_retains_generation_and_real_receipt_retry() {
    let case = before_scan_migration().await;
    let saved = retained(&case.db).await;
    let old_receipts = receipts(&case.db).await;
    case.db.execute_unprepared("CREATE TRIGGER synthetic_scan_receipt_failure BEFORE INSERT ON seaql_migrations WHEN NEW.version='m20260909_000015_business_execution_scans' BEGIN SELECT RAISE(ABORT,'synthetic receipt failure'); END;").await.unwrap();
    // SeaORM inserts its receipt after migration.up commits. Exercise that real
    // failure window, not a claim that the two commits are atomic.
    assert!(Migrator::up(&case.db, None).await.is_err());
    assert!(SchemaManager::new(&case.db)
        .has_table("business_execution_scan_metadata")
        .await
        .unwrap());
    assert_eq!(count(&case, 1).await, Some(0));
    assert_eq!(retained(&case.db).await, saved);
    assert_eq!(receipts(&case.db).await, old_receipts);
    // Test-only access in the committed-schema/missing-receipt interval proves
    // retry cannot reset an accepted fence. It does not enable serving failed startup.
    let page = list(
        &case.db,
        &case.files,
        &case.principal,
        OutputListInput {
            session_id: case.session_id.clone(),
            cursor: None,
            limit: 50,
        },
    )
    .await
    .unwrap();
    assert_eq!(page.items[0].id, case.output_id);
    assert_eq!(page.items[0].revision, 7);
    assert_eq!(count(&case, 1).await, Some(1));
    case.db
        .execute_unprepared("DROP TRIGGER synthetic_scan_receipt_failure")
        .await
        .unwrap();
    Migrator::up(&case.db, None).await.unwrap();
    assert_eq!(count(&case, 1).await, Some(1));
    assert_eq!(retained(&case.db).await, saved);
    let new_receipts = receipts(&case.db).await;
    assert_eq!(new_receipts.len(), old_receipts.len() + 1);
    assert_eq!(
        new_receipts
            .into_iter()
            .filter(|r| !r.contains(MIGRATION))
            .collect::<Vec<_>>(),
        old_receipts
    );
    assert!(case
        .db
        .query_all(statement("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
    let fk: i64 = case
        .db
        .query_one(statement("PRAGMA foreign_keys", vec![]))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "foreign_keys")
        .unwrap();
    assert_eq!(fk, 1);
}

#[tokio::test]
async fn execution_outputs_migration_failure_rolls_back_schema_seed_and_receipt() {
    let case = before_scan_migration().await;
    let saved = retained(&case.db).await;
    let old_receipts = receipts(&case.db).await;
    // This name collides late, after sidecar creation/seed and earlier triggers.
    // The pre-existing synthetic object must survive the rolled-back migration.
    case.db.execute_unprepared("CREATE TRIGGER business_execution_scan_retained AFTER INSERT ON business_execution_generation BEGIN SELECT 1; END;").await.unwrap();
    assert!(Migrator::up(&case.db, None).await.is_err());
    let manager = SchemaManager::new(&case.db);
    assert!(!manager.has_table("business_execution_scan").await.unwrap());
    assert!(!manager
        .has_table("business_execution_scan_metadata")
        .await
        .unwrap());
    let triggers: i64 = case.db.query_one(statement("SELECT count(*) AS n FROM sqlite_master WHERE type='trigger' AND name='business_execution_scan_created'", vec![])).await.unwrap().unwrap().try_get("", "n").unwrap();
    assert_eq!(triggers, 0);
    assert_eq!(retained(&case.db).await, saved);
    assert_eq!(receipts(&case.db).await, old_receipts);
    case.db
        .execute_unprepared("DROP TRIGGER business_execution_scan_retained")
        .await
        .unwrap();
    Migrator::up(&case.db, None).await.unwrap();
    assert_eq!(count(&case, 1).await, Some(0));
    assert_eq!(retained(&case.db).await, saved);
}

#[tokio::test]
async fn execution_outputs_migration_generation_fk_and_rollback_keep_exact_binding() {
    let case = before_scan_migration().await;
    Migrator::up(&case.db, None).await.unwrap();
    for generation in [2, 3] {
        let tx = identity::begin_write(&case.db, case.principal.organization_id())
            .await
            .unwrap();
        tx.execute(statement("INSERT INTO business_execution_generation(organization_id,session_id,generation,admission_id,initial_cursor,created_at) VALUES(?,?,?,?,?,?)",
            vec![case.principal.organization_id().into(), case.session_id.clone().into(), generation.into(), id().into(), id().into(), now().into()])).await.unwrap();
        let created: i64 = tx.query_one(statement("SELECT accepted_scan FROM business_execution_scan WHERE organization_id=? AND session_id=? AND generation=?",
            vec![case.principal.organization_id().into(), case.session_id.clone().into(), generation.into()])).await.unwrap().unwrap().try_get("", "accepted_scan").unwrap();
        assert_eq!(created, 0);
        if generation == 2 {
            tx.rollback().await.unwrap();
            assert_eq!(count(&case, 2).await, None);
        } else {
            tx.commit().await.unwrap();
            assert_eq!(count(&case, 3).await, Some(0));
        }
    }
    for (organization, session, generation) in [
        (id(), case.session_id.clone(), 1),
        (case.principal.organization_id().to_owned(), id(), 1),
        (
            case.principal.organization_id().to_owned(),
            case.session_id.clone(),
            99,
        ),
    ] {
        assert!(case.db.execute(statement("INSERT INTO business_execution_scan(organization_id,session_id,generation) VALUES(?,?,?)", vec![organization.into(), session.into(), generation.into()])).await.is_err());
    }
    assert!(case.db.execute(statement("DELETE FROM business_execution_generation WHERE organization_id=? AND session_id=? AND generation=3", vec![case.principal.organization_id().into(), case.session_id.clone().into()])).await.is_err());
    assert!(case
        .db
        .execute_unprepared("DELETE FROM business_execution_scan")
        .await
        .is_err());
    assert!(case
        .db
        .execute_unprepared("UPDATE business_execution_scan SET accepted_scan=-1")
        .await
        .is_err());
    assert!(case
        .db
        .execute_unprepared("UPDATE business_execution_scan SET accepted_scan=2")
        .await
        .is_err());
    assert!(case
        .db
        .execute_unprepared("UPDATE business_execution_scan SET generation=generation")
        .await
        .is_err());
    assert_eq!(count(&case, 1).await, Some(0));
    assert_eq!(count(&case, 3).await, Some(0));
    let migration = Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == MIGRATION)
        .unwrap();
    assert!(migration.down(&SchemaManager::new(&case.db)).await.is_err());
    assert!(case
        .db
        .query_all(statement("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
}
