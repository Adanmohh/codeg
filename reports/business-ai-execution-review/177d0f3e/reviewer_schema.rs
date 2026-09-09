//! Reviewer-only, frozen177d0f3e. Never compiled by a product worktree.
//! Apache Codeg fixtures: business_execution/tests.rs at177d0f3e;
//! business_identity/tests/platform.rs at9e61fe67; prior reviewer cancellation
//! probe reports/business-tenancy-review/f3b408da/independent_cases.rs atc0c7e5226.
//! These are storage/serialization probes, not E1 runtime admission assertions.
use super::*;
use sea_orm::{sqlx, ConnectOptions};
use sqlx::Connection;
use std::time::Duration;

const MIGRATION: &str = "m20260909_000014_business_execution";

#[tokio::test]
async fn review_execution_cancel_closes_pinned_connection_preserves_rows_and_retries() {
    let dir = tempfile::tempdir().unwrap();
    let url = format!("sqlite:{}?mode=rwc", dir.path().join("review.db").display());
    let mut options = ConnectOptions::new(url.clone());
    options.max_connections(1).min_connections(1);
    let conn = Database::connect(options).await.unwrap();
    let prefix = Migrator::migrations()
        .iter()
        .position(|m| m.name() == MIGRATION)
        .unwrap();
    Migrator::up(&conn, Some(prefix as u32)).await.unwrap();
    let (op, original) = task(&conn).await;
    let ctx = tasks::ActorContext::authenticated(op);
    let submitted = tasks::store::submit(
        &conn,
        &ctx,
        tasks::types::TextInput {
            task_id: original.task.id.clone(),
            expected_revision: 1,
            body: "Preserve this deliverable across cancelled DDL".into(),
        },
    )
    .await
    .unwrap();
    let saved = serde_json::to_value(submitted).unwrap();
    conn.execute_unprepared("CREATE TEMP TABLE review_connection_identity(id INTEGER)")
        .await
        .unwrap();

    let other = Database::connect(url).await.unwrap();
    let mut writer_connection = other.get_sqlite_connection_pool().acquire().await.unwrap();
    let writer = writer_connection
        .begin_with("BEGIN IMMEDIATE")
        .await
        .unwrap();
    let clone = conn.clone();
    let mut pending = tokio::spawn(async move { Migrator::up(&clone, None).await });
    assert!(
        tokio::time::timeout(Duration::from_millis(150), &mut pending)
            .await
            .is_err()
    );
    assert!(conn.get_sqlite_connection_pool().try_acquire().is_none());
    pending.abort();
    assert!(pending.await.unwrap_err().is_cancelled());
    writer.rollback().await.unwrap();

    {
        let mut replacement = tokio::time::timeout(
            Duration::from_secs(3),
            conn.get_sqlite_connection_pool().acquire(),
        )
        .await
        .unwrap()
        .unwrap();
        let old: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM sqlite_temp_master WHERE name='review_connection_identity'",
        )
        .fetch_one(&mut *replacement)
        .await
        .unwrap();
        assert_eq!(
            old, 0,
            "the FK-disabled connection must be discarded, not merely returned"
        );
        let enabled: i64 = sqlx::query_scalar("PRAGMA foreign_keys")
            .fetch_one(&mut *replacement)
            .await
            .unwrap();
        assert_eq!(enabled, 1);
    }
    assert!(!SchemaManager::new(&conn)
        .has_table("business_execution_metadata")
        .await
        .unwrap());
    assert_eq!(scalar(&conn, "SELECT count(*) AS value FROM seaql_migrations WHERE version='m20260909_000014_business_execution'").await, 0);
    let before_retry = tasks::store::get(
        &conn,
        &ctx,
        tasks::types::TaskInput {
            task_id: original.task.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(serde_json::to_value(before_retry).unwrap(), saved);
    assert!(conn
        .execute_unprepared("UPDATE business_task SET owner_id='missing-review-owner'")
        .await
        .is_err());
    Migrator::up(&conn, None).await.unwrap();
    let after_retry = tasks::store::get(
        &conn,
        &ctx,
        tasks::types::TaskInput {
            task_id: original.task.id,
        },
    )
    .await
    .unwrap();
    assert_eq!(serde_json::to_value(after_retry).unwrap(), saved);
    assert_eq!(scalar(&conn, "SELECT count(*) AS value FROM seaql_migrations WHERE version='m20260909_000014_business_execution'").await, 1);
    assert!(conn
        .query_all(stmt("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
}

async fn another_task(
    conn: &DatabaseConnection,
    principal: &identity::Principal,
) -> tasks::types::Detail {
    tasks::store::create(
        conn,
        &tasks::ActorContext::authenticated(principal.clone()),
        tasks::types::CreateInput {
            title: "Separate schema-review task".into(),
            notes: String::new(),
            domain: identity::Domain::Feedback,
            priority: tasks::vocabulary::TaskPriority::Normal,
            due_date: None,
            owner_id: None,
            assignee_id: None,
            reviewer_id: None,
        },
    )
    .await
    .unwrap()
}

async fn scope(conn: &DatabaseConnection, task_id: &str) -> i64 {
    conn.query_one(stmt(
        "SELECT epoch AS value FROM business_execution_task_scope WHERE task_id=?",
        vec![task_id.into()],
    ))
    .await
    .unwrap()
    .unwrap()
    .try_get("", "value")
    .unwrap()
}

async fn session(
    conn: &DatabaseConnection,
    principal: &identity::Principal,
    task_id: &str,
) -> (String, String) {
    let profile = uuid::Uuid::new_v4().to_string();
    let id = uuid::Uuid::new_v4().to_string();
    let grant = identity::delegation_grant(principal)
        .unwrap()
        .to_storage()
        .unwrap();
    conn.execute(stmt("INSERT INTO business_execution_profile(id,organization_id,member_id,client_id,config_key,config_hash,revision,summary_json) VALUES(?,?,?,'synthetic',?,?,1,'{}')",
        vec![profile.clone().into(), principal.organization_id().into(), principal.member_id().into(), profile.clone().into(), "a".repeat(64).into()])).await.unwrap();
    conn.execute(stmt("INSERT INTO business_execution_session(id,organization_id,task_id,member_id,authority_json,authorization_epoch,task_scope_epoch,profile_id,profile_revision,revision,generation,mode,status,title,created_at,updated_at,last_activity_at) VALUES(?,?,?,?,?,?,?,?,1,1,1,'chat','idle','Schema fixture','now','now','now')",
        vec![id.clone().into(), principal.organization_id().into(), task_id.into(), principal.member_id().into(), grant.into(), principal.authorization_epoch().into(), scope(conn, task_id).await.into(), profile.clone().into()])).await.unwrap();
    conn.execute(stmt("INSERT INTO business_execution_generation(organization_id,session_id,generation,admission_id,initial_cursor,created_at) VALUES(?,?,1,?,?,'now')",
        vec![principal.organization_id().into(), id.clone().into(), uuid::Uuid::new_v4().to_string().into(), uuid::Uuid::new_v4().to_string().into()])).await.unwrap();
    (id, profile)
}

async fn asset(
    conn: &DatabaseConnection,
    principal: &identity::Principal,
    task_id: &str,
    session_id: &str,
) -> (String, String) {
    let id = uuid::Uuid::new_v4().to_string();
    let version = uuid::Uuid::new_v4().to_string();
    conn.execute(stmt("INSERT INTO business_execution_asset(id,organization_id,task_id,member_id,revision,title,media_type,created_at,updated_at) VALUES(?,?,?,?,1,'Schema asset','text/plain','now','now')",
        vec![id.clone().into(), principal.organization_id().into(), task_id.into(), principal.member_id().into()])).await.unwrap();
    conn.execute(stmt("INSERT INTO business_execution_asset_version(id,organization_id,task_id,asset_id,version,title,sha256,byte_size,media_type,object_id,created_at,created_by,producer_session_id,producer_client_id) VALUES(?,?,?,?,1,'Immutable schema version',?,1,'text/plain',?,'now',?,?,'synthetic')",
        vec![version.clone().into(), principal.organization_id().into(), task_id.into(), id.clone().into(), "b".repeat(64).into(), uuid::Uuid::new_v4().to_string().into(), principal.member_id().into(), session_id.into()])).await.unwrap();
    conn.execute(stmt(
        "UPDATE business_execution_asset SET latest_version_id=? WHERE id=?",
        vec![version.clone().into(), id.clone().into()],
    ))
    .await
    .unwrap();
    (id, version)
}

async fn dump(conn: &DatabaseConnection) -> Vec<Vec<String>> {
    let mut all = vec![];
    for table in [
        "business_execution_profile",
        "business_execution_session",
        "business_execution_generation",
        "business_execution_operation",
        "business_execution_asset",
        "business_execution_asset_version",
        "business_execution_publication",
    ] {
        let columns = conn
            .query_all(stmt(&format!("PRAGMA table_info({table})"), vec![]))
            .await
            .unwrap()
            .into_iter()
            .map(|r| format!("\"{}\"", r.try_get::<String>("", "name").unwrap()))
            .collect::<Vec<_>>()
            .join(",");
        all.push(
            conn.query_all(stmt(
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
    all
}

async fn revoked(conn: &DatabaseConnection, id: &str) {
    let row = conn
        .query_one(stmt(
            "SELECT status,generation,revision FROM business_execution_session WHERE id=?",
            vec![id.into()],
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<String>("", "status").unwrap(), "revoked");
    assert_eq!(row.try_get::<i64>("", "generation").unwrap(), 2);
    assert_eq!(row.try_get::<i64>("", "revision").unwrap(), 2);
}

#[tokio::test]
async fn review_execution_real_tenant_task_fks_immutable_receipts_and_scope_away_back() {
    let conn = before_execution().await;
    let (op, original) = task(&conn).await;
    Migrator::up(&conn, None).await.unwrap();
    let local_other = another_task(&conn, &op).await;
    let platform = identity::platform::PlatformContext::from_operator(
        &crate::web::auth::AuthenticatedOperator,
    );
    let tenant = identity::platform::create(
        &conn,
        &platform,
        identity::platform::CreateTenantInput {
            operation_id: uuid::Uuid::new_v4().to_string(),
            organization_name: "Separate schema-review tenant".into(),
            owner_name: "Separate owner".into(),
        },
    )
    .await
    .unwrap();
    let foreign = identity::store::resolve_credential(&conn, tenant.token.as_ref().unwrap())
        .await
        .unwrap();
    let foreign_task = another_task(&conn, &foreign).await;
    assert_ne!(op.organization_id(), foreign.organization_id());
    let local_member = identity::store::create_member(
        &conn,
        &op,
        identity::types::CreateMemberInput {
            organization_id: op.organization_id().into(),
            display_name: "Other local member".into(),
            kind: identity::MemberKind::Human,
            role: identity::Role::Member,
            domains: vec![identity::Domain::Feedback],
        },
    )
    .await
    .unwrap();
    let (sid, _) = session(&conn, &op, &original.task.id).await;
    let (wrong_task_session, _) = session(&conn, &op, &local_other.task.id).await;
    let (foreign_session, foreign_profile) = session(&conn, &foreign, &foreign_task.task.id).await;
    let (aid, vid) = asset(&conn, &op, &original.task.id, &sid).await;
    let (foreign_asset, foreign_version) =
        asset(&conn, &foreign, &foreign_task.task.id, &foreign_session).await;
    let submitted = tasks::store::submit(
        &conn,
        &tasks::ActorContext::authenticated(op.clone()),
        tasks::types::TextInput {
            task_id: original.task.id.clone(),
            expected_revision: 1,
            body: "Selected schema version".into(),
        },
    )
    .await
    .unwrap();
    let deliverable = submitted.task.current_deliverable_id.as_ref().unwrap();
    conn.execute(stmt(
        "INSERT INTO business_execution_publication VALUES(?,?,?,?,?,0,'operator')",
        vec![
            op.organization_id().into(),
            original.task.id.clone().into(),
            deliverable.clone().into(),
            aid.clone().into(),
            vid.clone().into(),
        ],
    ))
    .await
    .unwrap();
    let operation = uuid::Uuid::new_v4().to_string();
    conn.execute(stmt("INSERT INTO business_execution_operation(operation_id,organization_id,member_id,kind,authority_json,authorization_epoch,input_hash,input_json,task_id,session_id,generation,status,created_at,updated_at) VALUES(?,?,?,'prompt',?,?,?,'{}',?,?,1,'confirmed','now','now')",
        vec![operation.clone().into(), op.organization_id().into(), op.member_id().into(), identity::delegation_grant(&op).unwrap().to_storage().unwrap().into(), op.authorization_epoch().into(), "c".repeat(64).into(), original.task.id.clone().into(), sid.clone().into()])).await.unwrap();

    // Every negative targets existing foreign or differently scoped records.
    // Full row snapshots prove rejection left prior positive controls unchanged.
    let before = dump(&conn).await;
    let rejected = vec![
        format!("INSERT INTO business_execution_session SELECT 'wrong-profile',organization_id,task_id,member_id,authority_json,authorization_epoch,task_scope_epoch,'{foreign_profile}',profile_revision,revision,generation,mode,status,title,created_at,updated_at,last_activity_at FROM business_execution_session WHERE id='{sid}'"),
        format!("INSERT INTO business_execution_session SELECT 'wrong-member',organization_id,task_id,'{}',authority_json,authorization_epoch,task_scope_epoch,profile_id,profile_revision,revision,generation,mode,status,title,created_at,updated_at,last_activity_at FROM business_execution_session WHERE id='{sid}'", local_member.id),
        format!("INSERT INTO business_execution_operation SELECT 'wrong-task',organization_id,member_id,kind,authority_json,authorization_epoch,input_hash,input_json,'{}',session_id,generation,status,reason,resource_id,result_json,created_at,updated_at FROM business_execution_operation WHERE operation_id='{operation}'", local_other.task.id),
        format!("INSERT INTO business_execution_operation SELECT 'wrong-generation',organization_id,member_id,kind,authority_json,authorization_epoch,input_hash,input_json,task_id,session_id,2,status,reason,resource_id,result_json,created_at,updated_at FROM business_execution_operation WHERE operation_id='{operation}'"),
        format!("INSERT INTO business_execution_asset_version SELECT 'wrong-producer',organization_id,task_id,asset_id,2,title,sha256,byte_size,media_type,'wrong-producer-object',created_at,created_by,'{wrong_task_session}',producer_turn_id,producer_client_id,producer_model FROM business_execution_asset_version WHERE id='{vid}'"),
        format!("INSERT INTO business_execution_publication VALUES('{}','{}','{deliverable}','{foreign_asset}','{foreign_version}',1,'operator')", op.organization_id(), original.task.id),
        format!("UPDATE business_execution_asset SET latest_version_id='{foreign_version}' WHERE id='{aid}'"),
        format!("UPDATE business_execution_session SET authority_json='{{}}' WHERE id='{sid}'"),
        format!("UPDATE business_execution_operation SET input_json='{{\"changed\":true}}' WHERE operation_id='{operation}'"),
        format!("UPDATE business_execution_operation SET status='pending' WHERE operation_id='{operation}'"),
        format!("DELETE FROM business_execution_operation WHERE operation_id='{operation}'"),
        format!("UPDATE business_execution_asset_version SET sha256='{}' WHERE id='{vid}'", "d".repeat(64)),
        format!("DELETE FROM business_execution_asset_version WHERE id='{vid}'"),
        format!("UPDATE business_execution_publication SET ordinal=1 WHERE version_id='{vid}'"),
        format!("DELETE FROM business_execution_publication WHERE version_id='{vid}'"),
    ];
    for sql in rejected {
        assert!(
            conn.execute_unprepared(&sql).await.is_err(),
            "unexpected accepted synthetic SQL: {sql}"
        );
        assert_eq!(dump(&conn).await, before);
    }
    conn.execute_unprepared(
        "DELETE FROM seaql_migrations WHERE version='m20260909_000014_business_execution'",
    )
    .await
    .unwrap();
    Migrator::up(&conn, None).await.unwrap();
    assert_eq!(
        dump(&conn).await,
        before,
        "receipt retry also retains populated new tables"
    );

    conn.execute(stmt(
        "UPDATE business_task SET assignee_id=? WHERE id=?",
        vec![local_member.id.into(), original.task.id.clone().into()],
    ))
    .await
    .unwrap();
    conn.execute(stmt(
        "UPDATE business_task SET assignee_id=NULL WHERE id=?",
        vec![original.task.id.clone().into()],
    ))
    .await
    .unwrap();
    assert_eq!(scope(&conn, &original.task.id).await, 3);
    revoked(&conn, &sid).await;
    assert!(conn
        .execute(stmt(
            "UPDATE business_execution_session SET status='idle',revision=3 WHERE id=?",
            vec![sid.into()]
        ))
        .await
        .is_err());

    let (cancel_session, _) = session(&conn, &op, &original.task.id).await;
    conn.execute(stmt(
        "UPDATE business_task SET status='cancelled' WHERE id=?",
        vec![original.task.id.clone().into()],
    ))
    .await
    .unwrap();
    conn.execute(stmt(
        "UPDATE business_task SET status='todo' WHERE id=?",
        vec![original.task.id.clone().into()],
    ))
    .await
    .unwrap();
    assert_eq!(scope(&conn, &original.task.id).await, 5);
    revoked(&conn, &cancel_session).await;
    let (profile_session, profile) = session(&conn, &op, &original.task.id).await;
    conn.execute(stmt(
        "UPDATE business_execution_profile SET revision=2 WHERE id=?",
        vec![profile.into()],
    ))
    .await
    .unwrap();
    revoked(&conn, &profile_session).await;
    assert!(conn
        .query_all(stmt("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
}

#[test]
fn review_execution_nested_wire_public_metadata_and_utf8_byte_bounds() {
    let id = uuid::Uuid::new_v4().to_string();
    let valid = json!({"operationId":id,"sessionId":id,"expectedSessionRevision":1,"text":"Use selected work","inputs":[{"kind":"asset","assetId":id,"versionId":id}]});
    validation::prompt(&serde_json::from_value::<PromptInput>(valid.clone()).unwrap()).unwrap();
    for extra in ["organizationId", "authority", "path", "sessionId"] {
        let mut poisoned = valid.clone();
        poisoned["inputs"][0][extra] = json!("untrusted");
        assert!(
            serde_json::from_value::<PromptInput>(poisoned).is_err(),
            "{extra}"
        );
    }
    let mut selected = json!({"operationId":id,"taskId":id,"expectedTaskRevision":1,"versions":[{"assetId":id,"versionId":id}],"body":""});
    validation::submit(&serde_json::from_value::<SubmitInput>(selected.clone()).unwrap()).unwrap();
    selected["versions"][0]["sha256"] = json!("caller-chosen hash");
    assert!(serde_json::from_value::<SubmitInput>(selected).is_err());

    let public = json!({"version":{"assetId":id,"versionId":id,"title":"Selected document","mediaType":"text/plain","byteSize":1,"sha256":"a".repeat(64)},"createdAt":"now","producer":{"clientId":"synthetic","model":null},"publication":{"deliverableId":id,"taskRevision":2,"submittedBy":{"memberId":id,"displayName":"Human","authorityKind":"operator"}}});
    let output =
        serde_json::to_value(serde_json::from_value::<PublishedVersion>(public.clone()).unwrap())
            .unwrap();
    assert_eq!(output, public);
    for extra in ["sessionId", "turnId", "profileId", "privateRefs"] {
        let mut private = public.clone();
        private["producer"][extra] = json!("private synthetic value");
        assert!(
            serde_json::from_value::<PublishedVersion>(private).is_err(),
            "{extra}"
        );
    }
    let mut message = MessagePart {
        message_id: id,
        role: MessageRole::Assistant,
        part: 0,
        text: "🦀".repeat(4096),
        complete: false,
    };
    assert_eq!(message.text.len(), validation::MAX_PART_BYTES);
    validation::event(&SessionEvent::Message {
        message: message.clone(),
    })
    .unwrap();
    message.text.push('🦀');
    assert_eq!(
        validation::event(&SessionEvent::Message { message }),
        Err(OperationReason::Invalid)
    );
    validation::event(&SessionEvent::Terminal {
        data: "x".repeat(validation::MAX_PART_BYTES),
    })
    .unwrap();
    assert_eq!(
        validation::event(&SessionEvent::Terminal {
            data: "x".repeat(validation::MAX_PART_BYTES + 1)
        }),
        Err(OperationReason::Invalid)
    );
}
