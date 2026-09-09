//! Synthetic SQLite/closed-wire checks. No client, native store or fixture use.
use super::{types::*, validation};
use crate::{business_identity as identity, business_tasks as tasks, db::migration::Migrator};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use serde_json::json;

fn stmt(sql: &str, values: Vec<sea_orm::Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values)
}
async fn scalar(conn: &DatabaseConnection, sql: &str) -> i64 {
    conn.query_one(stmt(sql, vec![]))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "value")
        .unwrap()
}
async fn before_execution() -> DatabaseConnection {
    let conn = Database::connect("sqlite::memory:").await.unwrap();
    let index = Migrator::migrations()
        .iter()
        .position(|m| m.name() == "m20260909_000014_business_execution")
        .unwrap();
    Migrator::up(&conn, Some(index as u32)).await.unwrap();
    conn
}
async fn task(conn: &DatabaseConnection) -> (identity::Principal, tasks::types::Detail) {
    identity::store::bootstrap(
        conn,
        identity::types::BootstrapInput {
            organization_name: "Synthetic execution".into(),
            owner_name: "Synthetic operator".into(),
        },
    )
    .await
    .unwrap();
    let principal = identity::operator_principal(conn).await.unwrap();
    let detail = tasks::store::create(
        conn,
        &tasks::ActorContext::authenticated(principal.clone()),
        tasks::types::CreateInput {
            title: "Write customer follow-up".into(),
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
    .unwrap();
    (principal, detail)
}

#[tokio::test]
async fn execution_migration_retains_review_history_and_real_receipt_retry() {
    let conn = before_execution().await;
    let (op, original) = task(&conn).await;
    let ctx = tasks::ActorContext::authenticated(op.clone());
    let submitted = tasks::store::submit(
        &conn,
        &ctx,
        tasks::types::TextInput {
            task_id: original.task.id.clone(),
            expected_revision: 1,
            body: "Original reviewed text".into(),
        },
    )
    .await
    .unwrap();
    let saved = serde_json::to_value(&submitted).unwrap();
    let receipts = scalar(&conn, "SELECT count(*) AS value FROM seaql_migrations").await;
    Migrator::up(&conn, None).await.unwrap();
    let reread = tasks::store::get(
        &conn,
        &ctx,
        tasks::types::TaskInput {
            task_id: original.task.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(serde_json::to_value(reread).unwrap(), saved);
    assert_eq!(
        scalar(&conn, "SELECT count(*) AS value FROM seaql_migrations").await,
        receipts + 1
    );
    assert_eq!(
        scalar(
            &conn,
            "SELECT count(*) AS value FROM business_execution_session"
        )
        .await,
        0
    );
    assert!(conn
        .execute_unprepared("UPDATE business_task_deliverable SET body='changed'")
        .await
        .is_err());
    assert!(conn
        .execute_unprepared("DELETE FROM business_task_deliverable")
        .await
        .is_err());
    assert!(conn
        .execute_unprepared("DELETE FROM business_task_activity")
        .await
        .is_err());
    assert!(conn
        .execute_unprepared("UPDATE business_task SET current_deliverable_id='foreign'")
        .await
        .is_err());
    assert!(tasks::store::submit(
        &conn,
        &ctx,
        tasks::types::TextInput {
            task_id: original.task.id,
            expected_revision: 2,
            body: " \n\t".into(),
        }
    )
    .await
    .is_err());
    // Actual SeaORM ledger loss after committed DDL: marker path must validate
    // all retained FKs and record the receipt without rerunning the rebuild.
    conn.execute_unprepared(
        "DELETE FROM seaql_migrations WHERE version='m20260909_000014_business_execution'",
    )
    .await
    .unwrap();
    Migrator::up(&conn, None).await.unwrap();
    assert_eq!(
        scalar(&conn, "SELECT count(*) AS value FROM seaql_migrations").await,
        receipts + 1
    );
    let fk = conn
        .query_one(stmt("PRAGMA foreign_keys", vec![]))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fk.try_get::<i64>("", "foreign_keys").unwrap(), 1);
    assert!(conn
        .query_all(stmt("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn execution_migration_failure_rolls_back_rebuild_and_restores_enforcement() {
    let conn = before_execution().await;
    let (op, task) = task(&conn).await;
    let submitted = tasks::store::submit(
        &conn,
        &tasks::ActorContext::authenticated(op.clone()),
        tasks::types::TextInput {
            task_id: task.task.id.clone(),
            expected_revision: task.task.revision,
            body: "Keep these exact bytes during failed DDL".into(),
        },
    )
    .await
    .unwrap();
    let saved = serde_json::to_value(&submitted).unwrap();
    let receipts = scalar(&conn, "SELECT count(*) AS value FROM seaql_migrations").await;
    // Fail after deliverable replacement, then verify old schema/data/receipts
    // were restored atomically and the same pooled connection enforces FKs.
    conn.execute_unprepared("CREATE TABLE business_execution_profile(sentinel TEXT); INSERT INTO business_execution_profile VALUES('retain');").await.unwrap();
    assert!(Migrator::up(&conn, None).await.is_err());
    let manager = SchemaManager::new(&conn);
    assert!(!manager
        .has_table("business_execution_metadata")
        .await
        .unwrap());
    assert!(!manager
        .has_table("business_task_deliverable_new")
        .await
        .unwrap());
    assert_eq!(
        scalar(&conn, "SELECT count(*) AS value FROM seaql_migrations").await,
        receipts
    );
    assert_eq!(
        scalar(&conn, "SELECT count(*) AS value FROM business_task").await,
        1
    );
    let reread = tasks::store::get(
        &conn,
        &tasks::ActorContext::authenticated(op),
        tasks::types::TaskInput {
            task_id: task.task.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(serde_json::to_value(reread).unwrap(), saved);
    assert_eq!(
        scalar(
            &conn,
            "SELECT count(*) AS value FROM business_execution_profile WHERE sentinel='retain'"
        )
        .await,
        1
    );
    let schema: String = conn
        .query_one(stmt(
            "SELECT sql FROM sqlite_master WHERE name='business_task_deliverable'",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "sql")
        .unwrap();
    assert!(schema.contains("BETWEEN 1 AND 20000"));
    assert!(conn
        .execute(stmt(
            "UPDATE business_task SET owner_id='missing' WHERE id=?",
            vec![task.task.id.into()]
        ))
        .await
        .is_err());
    conn.execute_unprepared("DROP TABLE business_execution_profile")
        .await
        .unwrap();
    Migrator::up(&conn, None).await.unwrap();
    assert!(conn
        .query_all(stmt("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn execution_migration_retains_original_linked_lineage_and_deliverable_columns() {
    let conn = before_execution().await;
    let (op, task) = task(&conn).await;
    let agent = identity::store::create_member(
        &conn,
        &op,
        identity::types::CreateMemberInput {
            organization_id: op.organization_id().into(),
            display_name: "Retained producer".into(),
            kind: identity::MemberKind::Agent,
            role: identity::Role::Member,
            domains: vec![identity::Domain::Feedback],
        },
    )
    .await
    .unwrap();
    // This is retained-schema data, not an engine ownership/admission assertion.
    // The original opaque grant is stored verbatim and never reconstructed.
    let grant = identity::delegation_grant(&op)
        .unwrap()
        .to_storage()
        .unwrap();
    conn.execute(stmt("INSERT INTO business_task_execution_authority(id,organization_id,task_id,domain,task_revision,work_task_id,run_seq,connection_id,agent_member_id,agent_key,entrusted_by,created_at) VALUES('retained-authority',?,?,'feedback',1,2147000,1,'synthetic-retained-run',?,'pi',?,'then')",
        vec![op.organization_id().into(), task.task.id.clone().into(), agent.id.clone().into(), op.member_id().into()])).await.unwrap();
    conn.execute(stmt("INSERT INTO business_task_execution(id,organization_id,task_id,authority_id,work_task_id,run_seq,connection_id,agent_member_id,agent_key,delegation_json,linked_by,created_at) VALUES('retained-execution',?,?,'retained-authority',2147000,1,'synthetic-retained-run',?,'pi',?,?,'then')",
        vec![op.organization_id().into(), task.task.id.clone().into(), agent.id.clone().into(), grant.into(), op.member_id().into()])).await.unwrap();
    conn.execute(stmt("INSERT INTO business_task_deliverable(id,organization_id,task_id,revision,author_id,author_name,author_kind,body,execution_id,created_at) VALUES('retained-agent-output',?,?,2,?,'Retained producer','agent','Exact retained agent text','retained-execution','then')",
        vec![op.organization_id().into(), task.task.id.clone().into(), agent.id.into()])).await.unwrap();
    conn.execute(stmt("UPDATE business_task SET current_deliverable_id='retained-agent-output',current_execution_id='retained-execution',revision=2,status='review' WHERE id=?", vec![task.task.id.into()])).await.unwrap();
    let mut queries = vec![];
    for table in [
        "business_task",
        "business_task_activity",
        "business_task_deliverable",
        "business_task_execution",
        "business_task_execution_authority",
        "business_execution_authority_epoch",
        "business_member",
    ] {
        let columns = conn
            .query_all(stmt(&format!("PRAGMA table_info({table})"), vec![]))
            .await
            .unwrap();
        let names = columns
            .into_iter()
            .map(|r| format!("\"{}\"", r.try_get::<String>("", "name").unwrap()))
            .collect::<Vec<_>>()
            .join(",");
        queries.push(format!(
            "SELECT json_array({names}) AS value FROM {table} ORDER BY rowid"
        ));
    }
    let mut saved = vec![];
    for query in &queries {
        saved.push(
            conn.query_all(stmt(query, vec![]))
                .await
                .unwrap()
                .into_iter()
                .map(|r| r.try_get::<String>("", "value").unwrap())
                .collect::<Vec<_>>(),
        );
    }
    Migrator::up(&conn, None).await.unwrap();
    // Retry after receipt loss must preserve every captured value too.
    conn.execute_unprepared(
        "DELETE FROM seaql_migrations WHERE version='m20260909_000014_business_execution'",
    )
    .await
    .unwrap();
    Migrator::up(&conn, None).await.unwrap();
    for (query, before) in queries.iter().zip(saved) {
        let after = conn
            .query_all(stmt(query, vec![]))
            .await
            .unwrap()
            .into_iter()
            .map(|r| r.try_get::<String>("", "value").unwrap())
            .collect::<Vec<_>>();
        assert_eq!(after, before);
    }
    assert!(conn
        .query_all(stmt("PRAGMA foreign_key_check", vec![]))
        .await
        .unwrap()
        .is_empty());
    assert!(conn
        .execute_unprepared("UPDATE business_task_deliverable SET execution_id=NULL")
        .await
        .is_err());
}

#[test]
fn execution_wire_rejects_authority_fields_and_unimplemented_account_context() {
    let id = uuid::Uuid::new_v4().to_string();
    let mut value = json!({"operationId":id,"taskId":id,"expectedTaskRevision":1,"profileId":id,"expectedProfileRevision":1,"mode":"chat"});
    let allowed: StartInput = serde_json::from_value(value.clone()).unwrap();
    validation::start(&allowed).unwrap();
    for key in [
        "actor",
        "organizationId",
        "cwd",
        "env",
        "command",
        "agentMemberId",
    ] {
        value[key] = json!("forged");
        assert!(
            serde_json::from_value::<StartInput>(value.clone()).is_err(),
            "{key}"
        );
        value.as_object_mut().unwrap().remove(key);
    }
    let input = PromptInput {
        operation_id: id.clone(),
        session_id: id.clone(),
        expected_session_revision: 1,
        text: "Use current account facts".into(),
        inputs: vec![InputRef::AccountSnapshot {
            snapshot_id: id.clone(),
            expected_revision: 1,
        }],
    };
    assert_eq!(
        validation::prompt(&input),
        Err(OperationReason::Unavailable)
    );
    let mut submit = SubmitInput {
        operation_id: id.clone(),
        task_id: id.clone(),
        expected_task_revision: 1,
        versions: vec![],
        body: String::new(),
    };
    assert_eq!(validation::submit(&submit), Err(OperationReason::Invalid));
    submit.versions.push(VersionSelection {
        asset_id: id.clone(),
        version_id: id,
    });
    validation::submit(&submit).unwrap();
    submit.versions.push(submit.versions[0].clone());
    assert_eq!(validation::submit(&submit), Err(OperationReason::Invalid));
    assert_eq!(
        validation::revision(9_007_199_254_740_992),
        Err(OperationReason::Invalid)
    );
}
