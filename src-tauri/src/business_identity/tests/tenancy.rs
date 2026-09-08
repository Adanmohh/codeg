use super::*;
use crate::db::migration::Migrator;
use sea_orm::Database;
use sea_orm_migration::{MigratorTrait, SchemaManager};

#[tokio::test]
async fn tenancy_epoch_away_back_never_refreshes_captured_human_or_grant() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let (_, issued, principal) = human(&db.conn, &op, Role::Manager, vec![Domain::Feedback]).await;
    assert_eq!(principal.authorization_epoch(), 1);
    let agent = store::create_member(
        &db.conn,
        &op,
        CreateMemberInput {
            organization_id: op.organization_id().into(),
            display_name: "Scoped agent".into(),
            kind: MemberKind::Agent,
            role: Role::Member,
            domains: vec![Domain::Feedback],
        },
    )
    .await
    .unwrap();
    let grant = delegation_grant(&principal).unwrap().to_storage().unwrap();
    let mut legacy: Value = serde_json::from_str(&grant).unwrap();
    legacy
        .as_object_mut()
        .unwrap()
        .remove("authorization_epoch");
    assert!(agent_principal_from_binding(
        &db.conn,
        op.organization_id(),
        &agent.id,
        &legacy.to_string()
    )
    .await
    .is_ok());
    for state in ["suspended", "active"] {
        db.conn.execute(store::statement("UPDATE business_organization SET status=?,revision=revision+1,authorization_epoch=authorization_epoch+1 WHERE id=?",vec![state.into(),op.organization_id().into()])).await.unwrap();
        assert!(matches!(
            authorize(
                &db.conn,
                &principal,
                op.organization_id(),
                Permission::Read,
                None
            )
            .await,
            Err(IdentityError::Unauthorized)
        ));
        assert!(
            agent_principal_from_binding(&db.conn, op.organization_id(), &agent.id, &grant)
                .await
                .is_err()
        );
        assert!(agent_principal_from_binding(
            &db.conn,
            op.organization_id(),
            &agent.id,
            &legacy.to_string()
        )
        .await
        .is_err());
        if state == "suspended" {
            assert!(store::resolve_credential(&db.conn, &issued.token)
                .await
                .is_err());
        }
    }
    let fresh = store::resolve_credential(&db.conn, &issued.token)
        .await
        .unwrap();
    assert_eq!(fresh.authorization_epoch(), 3);
    assert!(authorize(
        &db.conn,
        &fresh,
        op.organization_id(),
        Permission::Read,
        None
    )
    .await
    .is_ok());
}

#[tokio::test]
async fn tenancy_settings_cas_and_current_authority_preserve_other_fields() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let (_, _, viewer) = human(&db.conn, &op, Role::Viewer, vec![Domain::Feedback]).await;
    let initial = settings::get(&db.conn, &viewer).await.unwrap();
    assert_eq!(initial.settings.palette, settings::Palette::Neutral);
    let input = || settings::UpdateSettingsInput {
        expected_revision: 1,
        settings: settings::Settings::defaults("New display".into()),
    };
    assert!(matches!(
        settings::update(&db.conn, &viewer, input()).await,
        Err(IdentityError::Forbidden)
    ));
    let saved = settings::update(&db.conn, &op, input()).await.unwrap();
    assert_eq!(saved.revision, 2);
    assert!(matches!(
        settings::update(&db.conn, &op, input()).await,
        Err(IdentityError::Conflict)
    ));
    let org = store::organization(&db.conn).await.unwrap().unwrap();
    assert_eq!(org.name, "Synthetic team");
    assert_eq!(org.authorization_epoch, 1);
    assert_eq!(org.revision, 1);
    for invalid in [
        json!({"displayName":"x","palette":"evil","workspaceLayout":"split","defaultWorkArea":"tasks"}),
        json!({"displayName":"x","palette":"neutral","workspaceLayout":"split","defaultWorkArea":"tasks","css":"body{}"}),
    ] {
        assert!(serde_json::from_value::<settings::Settings>(invalid).is_err());
    }
}

#[tokio::test]
async fn tenancy_migration_preserves_populated_ids_hashes_history_and_fk_after_failure() {
    let conn = Database::connect("sqlite::memory:").await.unwrap();
    let manager = SchemaManager::new(&conn);
    let migrations = Migrator::migrations();
    for m in migrations
        .iter()
        .take_while(|m| m.name() != "m20260908_000012_business_tenancy")
    {
        m.up(&manager).await.unwrap();
    }
    conn.execute_unprepared("INSERT INTO business_organization VALUES ('original',1,'Original','now'); INSERT INTO business_member (id,organization_id,display_name,kind,role,domains_json,operator_owner,created_at,updated_at) VALUES ('owner','original','Owner','human','owner','[\"feedback\"]',1,'now','now'); INSERT INTO business_credential VALUES ('credential','original','owner','aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa','kept','now',NULL); INSERT INTO business_identity_event VALUES ('history','original','owner','preserve','owner','now'); CREATE TABLE business_tenant_settings (sentinel INTEGER); INSERT INTO business_tenant_settings VALUES(123);").await.unwrap();
    let migration = migrations
        .iter()
        .find(|m| m.name() == "m20260908_000012_business_tenancy")
        .unwrap();
    assert!(migration.up(&manager).await.is_err());
    assert!(!manager
        .has_table("business_tenancy_metadata")
        .await
        .unwrap());
    assert!(!manager
        .has_table("business_organization_new")
        .await
        .unwrap());
    let fk = conn
        .query_one(store::statement("PRAGMA foreign_keys", vec![]))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fk.try_get::<i64>("", "foreign_keys").unwrap(), 1);
    assert!(conn
        .execute_unprepared(
            "INSERT INTO business_organization VALUES ('cannot-create-second',1,'Other','now')"
        )
        .await
        .is_err());
    let sentinel = conn
        .query_one(store::statement(
            "SELECT sentinel FROM business_tenant_settings",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(sentinel.try_get::<i64>("", "sentinel").unwrap(), 123);
    conn.execute_unprepared("DROP TABLE business_tenant_settings")
        .await
        .unwrap();
    migration.up(&manager).await.unwrap();
    // Re-entry after commit-before-migration-ledger loss is harmless.
    migration.up(&manager).await.unwrap();
    let org = store::organization(&conn).await.unwrap().unwrap();
    assert_eq!(org.id, "original");
    let kept = conn
        .query_one(store::statement(
            "SELECT token_hash FROM business_credential WHERE id='credential'",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        kept.try_get::<String>("", "token_hash").unwrap(),
        "a".repeat(64)
    );
    assert!(conn
        .execute_unprepared("DELETE FROM business_identity_event")
        .await
        .is_err());
    assert!(conn.execute_unprepared("INSERT INTO business_credential VALUES ('bad','wrong','owner','bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb','bad','now',NULL)").await.is_err());
    assert!(conn
        .query_all(store::statement("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
    assert!(migration.down(&manager).await.is_err());
    // Writer lock is a no-op on ID; immutable identity trigger permits it.
    begin_write(&conn, "original")
        .await
        .unwrap()
        .rollback()
        .await
        .unwrap();
}
