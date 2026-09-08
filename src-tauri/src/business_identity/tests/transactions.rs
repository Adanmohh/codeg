use super::*;
use crate::db::{migration::Migrator, test_helpers::fresh_disk_db};
use sea_orm::{Database, TransactionTrait};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use std::time::Duration;

#[tokio::test]
async fn writer_serialization_fences_a_cached_principal_after_concurrent_revocation() {
    let dir = tempfile::tempdir().unwrap();
    let db = fresh_disk_db(dir.path()).await;
    let other = Database::connect(format!(
        "sqlite:{}?mode=rw",
        dir.path().join("source.db").display()
    ))
    .await
    .unwrap();
    let op = initialize(&db.conn).await;
    let org = op.organization_id().to_owned();
    let (_, issued, cached) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    db.conn.execute_unprepared("CREATE TABLE fixture_business_write (id INTEGER PRIMARY KEY, value INTEGER); INSERT INTO fixture_business_write VALUES (1, 0)").await.unwrap();
    assert!(authorize(
        &other,
        &cached,
        &org,
        Permission::Contribute,
        Some(Domain::Feedback)
    )
    .await
    .is_ok());
    let revocation = begin_write(&db.conn, &org).await.unwrap();
    revocation
        .execute(store::statement(
            "UPDATE business_credential SET revoked_at = ? WHERE id = ?",
            vec!["2026-09-08T00:00:00Z".into(), issued.credential.id.into()],
        ))
        .await
        .unwrap();
    let waiting = Arc::new(tokio::sync::Notify::new());
    let signaled = waiting.clone();
    let org_clone = org.clone();
    let mut mutation = tokio::spawn(async move {
        signaled.notify_one();
        let tx = begin_write(&other, &org_clone).await?;
        authorize(
            &tx,
            &cached,
            &org_clone,
            Permission::Contribute,
            Some(Domain::Feedback),
        )
        .await?;
        tx.execute_unprepared("UPDATE fixture_business_write SET value = value + 1 WHERE id = 1")
            .await?;
        tx.commit().await?;
        Ok::<(), IdentityError>(())
    });
    waiting.notified().await;
    // A real second SQLite connection must wait for the existing writer;
    // checking before obtaining the lock would incorrectly pass above.
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut mutation)
            .await
            .is_err()
    );
    revocation.commit().await.unwrap();
    assert!(matches!(
        mutation.await.unwrap(),
        Err(IdentityError::Unauthorized)
    ));
    let row = db
        .conn
        .query_one(store::statement(
            "SELECT value FROM fixture_business_write WHERE id = 1",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<i32>("", "value").unwrap(), 0);
}

#[tokio::test]
async fn independent_connections_cas_member_updates_and_append_one_event() {
    let dir = tempfile::tempdir().unwrap();
    let db = fresh_disk_db(dir.path()).await;
    let other = Database::connect(format!(
        "sqlite:{}?mode=rw",
        dir.path().join("source.db").display()
    ))
    .await
    .unwrap();
    let op = initialize(&db.conn).await;
    let (member, _, _) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    let input = |display: &str| UpdateMemberInput {
        organization_id: op.organization_id().into(),
        member_id: member.id.clone(),
        expected_revision: member.revision,
        display_name: display.into(),
        role: member.role,
        domains: member.domains.clone(),
    };
    let (a, b) = tokio::join!(
        store::update_member(&db.conn, &op, input("Concurrent A")),
        store::update_member(&other, &op, input("Concurrent B"))
    );
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    assert!(matches!(a, Err(IdentityError::Conflict)) || matches!(b, Err(IdentityError::Conflict)));
    let current = store::member(&db.conn, op.organization_id(), &member.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(current.revision, 2);
    assert!(current.display_name == "Concurrent A" || current.display_name == "Concurrent B");
    let row = db.conn.query_one(store::statement("SELECT COUNT(*) AS count FROM business_identity_event WHERE subject_id = ? AND action = 'member_updated'", vec![member.id.into()])).await.unwrap().unwrap();
    assert_eq!(row.try_get::<i64>("", "count").unwrap(), 1);
}

#[tokio::test]
async fn competing_operator_bootstraps_create_one_organization_and_preserve_the_winner() {
    let dir = tempfile::tempdir().unwrap();
    let db = fresh_disk_db(dir.path()).await;
    let other = Database::connect(format!(
        "sqlite:{}?mode=rw",
        dir.path().join("source.db").display()
    ))
    .await
    .unwrap();
    let input = |name: &str| BootstrapInput {
        organization_name: name.into(),
        owner_name: name.into(),
    };
    let (a, b) = tokio::join!(
        store::bootstrap(&db.conn, input("Team A")),
        store::bootstrap(&other, input("Team B"))
    );
    let a = a.unwrap();
    let b = b.unwrap();
    assert_eq!(
        a.organization.as_ref().unwrap().id,
        b.organization.as_ref().unwrap().id
    );
    assert_eq!(a.member.as_ref().unwrap().id, b.member.as_ref().unwrap().id);
    let row = db
        .conn
        .query_one(store::statement(
            "SELECT COUNT(*) AS count FROM business_member WHERE operator_owner = 1",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<i64>("", "count").unwrap(), 1);
}

#[tokio::test]
async fn identity_audit_failure_rolls_back_member_creation_and_history_is_immutable() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    db.conn.execute_unprepared("CREATE TRIGGER fixture_identity_audit_failure BEFORE INSERT ON business_identity_event WHEN NEW.action = 'member_created' BEGIN SELECT RAISE(ABORT, 'fixture audit unavailable'); END;").await.unwrap();
    let result = store::create_member(
        &db.conn,
        &op,
        CreateMemberInput {
            organization_id: op.organization_id().into(),
            display_name: "Must roll back".into(),
            kind: MemberKind::Human,
            role: Role::Member,
            domains: vec![Domain::Feedback],
        },
    )
    .await;
    assert!(matches!(result, Err(IdentityError::Database(_))));
    let row = db
        .conn
        .query_one(store::statement(
            "SELECT COUNT(*) AS count FROM business_member",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<i64>("", "count").unwrap(), 1);
    assert!(db
        .conn
        .execute_unprepared("UPDATE business_identity_event SET actor_id = 'forged'")
        .await
        .is_err());
    assert!(db
        .conn
        .execute_unprepared("DELETE FROM business_identity_event")
        .await
        .is_err());
}

#[tokio::test]
async fn explicitly_named_identity_migration_is_atomic_and_preserves_initialized_rows() {
    // Test000009 against its actual prefix, not later tenant tables that must
    // remain intact and intentionally refuse lossy down-migration.
    let db = crate::db::AppDatabase {
        conn: Database::connect("sqlite::memory:").await.unwrap(),
    };
    for earlier in Migrator::migrations()
        .into_iter()
        .take_while(|m| m.name() != "m20260908_000009_business_identity")
    {
        earlier.up(&SchemaManager::new(&db.conn)).await.unwrap();
    }
    let migration = Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260908_000009_business_identity")
        .unwrap();
    let manager = SchemaManager::new(&db.conn);
    db.conn
        .execute_unprepared("CREATE TABLE business_credential (id INTEGER)")
        .await
        .unwrap();
    assert!(migration.up(&manager).await.is_err());
    assert!(!manager.has_table("business_organization").await.unwrap());
    assert!(!manager.has_table("business_member").await.unwrap());
    assert!(manager.has_table("ops_proposal").await.unwrap());
    assert!(manager.has_table("ops_ticket_message").await.unwrap());
    db.conn
        .execute_unprepared("DROP TABLE business_credential")
        .await
        .unwrap();
    migration.up(&manager).await.unwrap();
    for later in Migrator::migrations()
        .into_iter()
        .skip_while(|m| m.name() != "m20260908_000009_business_identity")
        .skip(1)
    {
        later.up(&manager).await.unwrap();
    }
    let op = initialize(&db.conn).await;
    assert!(migration.down(&manager).await.is_err());
    assert!(
        store::member(&db.conn, op.organization_id(), op.member_id())
            .await
            .unwrap()
            .is_some()
    );
    // Composite org references and the agent-role constraint also fail closed
    // against accidental future direct SQL writers.
    assert!(db.conn.execute(store::statement("INSERT INTO business_credential (id, organization_id, member_id, token_hash, label, created_at) VALUES ('invalid-org', 'foreign-org', ?, ?, 'fixture', 'now')", vec![op.member_id().into(), "0".repeat(64).into()])).await.is_err());
    let agent_id = uuid::Uuid::new_v4().to_string();
    assert!(db.conn.execute(store::statement("INSERT INTO business_member (id, organization_id, display_name, kind, role, domains_json, created_at, updated_at) VALUES (?, ?, 'invalid agent', 'agent', 'owner', '[\"feedback\"]', 'now', 'now')", vec![agent_id.into(), op.organization_id().into()])).await.is_err());
    let _read_snapshot = db.conn.begin().await.unwrap();
}
