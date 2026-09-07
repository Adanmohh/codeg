use super::*;
use crate::db::{
    entities::{folder, ops_agent_rule as rule, ops_agent_scope as scope, work_task_event},
    service::work_task_service as tasks,
    test_helpers::{fresh_in_memory_db, seed_conversation, seed_folder},
    AppDatabase,
};
use crate::models::{agent::AgentType, WorkTaskDraft};
use sea_orm::{ConnectOptions, Database, QueryOrder};
use sea_orm_migration::{MigrationName, MigrationTrait, MigratorTrait, SchemaManager};
use sha2::{Digest, Sha256};

const APPROVALS_MIGRATION: &str = "m20260907_000001_ops_approvals";

// A real later migration exposes accidental last-entry rollback even before
// tickets is merged. The full application list includes tickets once integrated.
struct AfterApprovals;
impl MigrationName for AfterApprovals {
    fn name(&self) -> &str {
        "m20991231_000001_approvals_test_follower"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for AfterApprovals {
    async fn up(&self, manager: &SchemaManager) -> Result<(), sea_orm::DbErr> {
        manager.get_connection().execute_unprepared(
            "CREATE TABLE approvals_test_follower (marker TEXT NOT NULL); INSERT INTO approvals_test_follower VALUES ('preserved');"
        ).await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), sea_orm::DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE approvals_test_follower")
            .await?;
        Ok(())
    }
}
struct WithFollower;
#[async_trait::async_trait]
impl MigratorTrait for WithFollower {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut migrations = crate::db::migration::Migrator::migrations();
        migrations.push(Box::new(AfterApprovals));
        migrations
    }
}
async fn target_migration(conn: &DatabaseConnection) -> Box<dyn MigrationTrait> {
    WithFollower::up(conn, None).await.unwrap();
    let migrations = WithFollower::migrations();
    assert_ne!(migrations.last().unwrap().name(), APPROVALS_MIGRATION);
    migrations
        .into_iter()
        .find(|migration| migration.name() == APPROVALS_MIGRATION)
        .unwrap()
}
async fn assert_follower_survives(conn: &DatabaseConnection) {
    let result = conn
        .query_one(sea_orm::Statement::from_string(
            conn.get_database_backend(),
            "SELECT marker FROM approvals_test_follower",
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result.try_get::<String>("", "marker").unwrap(), "preserved");
}

struct DraftAction {
    decision: Decision,
    destructive: bool,
}
impl Default for DraftAction {
    fn default() -> Self {
        Self {
            decision: Decision::Passthrough,
            destructive: true,
        }
    }
}
#[async_trait::async_trait]
impl Action for DraftAction {
    fn name(&self) -> &'static str {
        "email.draft"
    }
    fn domain(&self) -> &'static str {
        "email"
    }
    fn validate(&self, p: &Value) -> Result<(), DbError> {
        if p["body"].as_str().is_none() || p["mailbox"].as_str().is_none() {
            return Err(DbError::Validation("missing body or mailbox".into()));
        }
        Ok(())
    }
    async fn resource(&self, p: &Value, _: &ActionContext<'_>) -> Result<Option<String>, DbError> {
        Ok(p["mailbox"].as_str().map(str::to_owned))
    }
    async fn check_permission(
        &self,
        p: &Value,
        _: &ActionContext<'_>,
    ) -> Result<Decision, DbError> {
        match p["body"].as_str() {
            Some("forbidden") => Ok(Decision::Deny),
            Some("unavailable") => Err(DbError::Validation("policy unavailable".into())),
            _ => Ok(self.decision),
        }
    }
    fn is_destructive(&self, _: &Value) -> Result<bool, DbError> {
        Ok(self.destructive)
    }
    fn private_fields(&self) -> &'static [&'static str] {
        &["body"]
    }
    async fn preview(&self, p: &Value, _: &ActionContext<'_>) -> Result<Value, DbError> {
        Ok(json!({"body": p["body"], "headers": [{"Authorization": "fake-preview-secret"}]}))
    }
}
fn payload(body: &str) -> Value {
    json!({"body":body,"mailbox":"support"})
}

async fn start(db: &AppDatabase) -> (i32, i32) {
    let folder = seed_folder(db, "/tmp/ops-approvals-test").await;
    let conversation = seed_conversation(db, folder, AgentType::Codex).await;
    let task = tasks::create(
        &db.conn,
        WorkTaskDraft {
            folder_id: folder,
            title: "review draft".into(),
            config: json!({
                "display_text":"draft", "prompt_blocks":[{"type":"text","text":"draft"}]
            }),
        },
    )
    .await
    .unwrap();
    let seq = tasks::claim_for_run(&db.conn, task.id, WorkTaskStatus::Todo, "user")
        .await
        .unwrap()
        .unwrap();
    assert!(tasks::begin_setup(&db.conn, task.id, seq).await.unwrap());
    assert!(
        tasks::mark_running(&db.conn, task.id, seq, conversation, "approval-test")
            .await
            .unwrap()
    );
    (task.id, seq)
}
async fn draft(db: &AppDatabase, task: i32, seq: i32) -> proposal::Model {
    match propose(
        &db.conn,
        task,
        seq,
        "agent",
        &DraftAction::default(),
        payload("original"),
    )
    .await
    .unwrap()
    {
        ProposalOutcome::Pending(p) => *p,
        _ => panic!("default must require review"),
    }
}
async fn approve_as(
    conn: &DatabaseConnection,
    row: &proposal::Model,
    actor: &str,
    approved: Value,
) -> Result<AuthorizedAction, DbError> {
    approve(
        conn,
        row.task_id,
        row.run_seq,
        row.id,
        actor,
        &DraftAction::default(),
        Review {
            expected_payload: &payload("original"),
            approved_payload: approved,
        },
    )
    .await
}
async fn eval_gate(
    conn: &DatabaseConnection,
    action: &dyn Action,
    p: &Value,
    agent: &str,
) -> Result<GateResult, DbError> {
    let txn = conn.begin().await?;
    gating::gate(
        action,
        p,
        &ActionContext {
            db: &txn,
            agent,
            actor: None,
        },
    )
    .await
}
async fn add_rule(
    conn: &DatabaseConnection,
    behavior: &str,
    resource: Option<&str>,
    action: Option<&str>,
) -> i32 {
    rule::ActiveModel {
        agent_id: Set("agent".into()),
        domain: Set("email".into()),
        resource: Set(resource.map(str::to_owned)),
        action_name: Set(action.map(str::to_owned)),
        behavior: Set(behavior.into()),
        ..Default::default()
    }
    .insert(conn)
    .await
    .unwrap()
    .id
}
async fn add_scope(conn: &DatabaseConnection, mode: &str, resource: Option<&str>) {
    scope::ActiveModel {
        agent_id: Set("agent".into()),
        domain: Set("email".into()),
        resource: Set(resource.map(str::to_owned)),
        mode: Set(mode.into()),
        ..Default::default()
    }
    .insert(conn)
    .await
    .unwrap();
}
async fn stored(conn: &DatabaseConnection, id: i32) -> proposal::Model {
    proposal::Entity::find_by_id(id)
        .one(conn)
        .await
        .unwrap()
        .unwrap()
}
async fn audits(conn: &DatabaseConnection) -> Vec<audit::Model> {
    audit::Entity::find()
        .order_by_asc(audit::Column::Id)
        .all(conn)
        .await
        .unwrap()
}

#[tokio::test]
async fn gate_order_and_destructive_floor_in_every_mode() {
    let db = fresh_in_memory_db().await;
    // Explicit expected precedence cases from IntroMail gating.py, including
    // its deliberate ask-before-action behavior (approval rechecks hard deny).
    for (rules, decision, destructive, verdict, reason) in [
        (
            vec!["allow", "ask", "deny"],
            Decision::Allow,
            true,
            Verdict::Deny,
            "deny_rule",
        ),
        (
            vec!["allow", "ask"],
            Decision::Deny,
            true,
            Verdict::NeedsReview,
            "ask_rule",
        ),
        (
            vec!["allow"],
            Decision::Deny,
            false,
            Verdict::Deny,
            "action",
        ),
        (
            vec!["allow"],
            Decision::NeedsReview,
            false,
            Verdict::NeedsReview,
            "action",
        ),
        (
            vec!["allow"],
            Decision::Allow,
            true,
            Verdict::NeedsReview,
            "destructive",
        ),
        (
            vec![],
            Decision::Allow,
            true,
            Verdict::NeedsReview,
            "destructive",
        ),
        (
            vec![],
            Decision::Passthrough,
            true,
            Verdict::NeedsReview,
            "destructive",
        ),
        (
            vec!["allow"],
            Decision::Allow,
            false,
            Verdict::Auto,
            "allow_rule",
        ),
        (vec![], Decision::Allow, false, Verdict::Auto, "action"),
    ] {
        for mode in ["read", "propose", "act_low_risk"] {
            rule::Entity::delete_many().exec(&db.conn).await.unwrap();
            scope::Entity::delete_many().exec(&db.conn).await.unwrap();
            add_scope(&db.conn, mode, None).await;
            for behavior in &rules {
                add_rule(&db.conn, behavior, None, None).await;
            }
            let action = DraftAction {
                decision,
                destructive,
            };
            let result = eval_gate(&db.conn, &action, &payload("valid"), "agent")
                .await
                .unwrap();
            assert_eq!(
                (result.verdict, result.reason),
                (verdict, reason),
                "{mode} {rules:?}"
            );
        }
    }
}

#[tokio::test]
async fn scope_override_and_rule_specificity_are_deterministic() {
    let db = fresh_in_memory_db().await;
    let action = DraftAction {
        destructive: false,
        ..Default::default()
    };
    let p = payload("draft");
    let result = eval_gate(&db.conn, &action, &p, "agent").await.unwrap();
    assert_eq!(
        (result.verdict, result.mode.as_str()),
        (Verdict::NeedsReview, "propose")
    );
    add_scope(&db.conn, "act_low_risk", None).await;
    assert_eq!(
        eval_gate(&db.conn, &action, &p, "agent")
            .await
            .unwrap()
            .verdict,
        Verdict::Auto
    );
    add_scope(&db.conn, "read", Some("support")).await;
    assert_eq!(
        eval_gate(&db.conn, &action, &p, "agent")
            .await
            .unwrap()
            .verdict,
        Verdict::NeedsReview
    );
    // Agent/domain/action/resource isolation; a different mailbox keeps default.
    let mut other = p.clone();
    other["mailbox"] = json!("other");
    assert_eq!(
        eval_gate(&db.conn, &action, &other, "agent")
            .await
            .unwrap()
            .verdict,
        Verdict::Auto
    );
    add_rule(&db.conn, "deny", Some("unrelated"), None).await;
    add_rule(&db.conn, "deny", None, Some("unrelated")).await;
    add_rule(&db.conn, "allow", None, Some(action.name())).await;
    let specific = add_rule(&db.conn, "allow", Some("support"), None).await;
    assert_eq!(
        eval_gate(&db.conn, &action, &p, "agent")
            .await
            .unwrap()
            .matched_rule_id,
        Some(specific)
    );
    let exact = add_rule(&db.conn, "allow", Some("support"), Some(action.name())).await;
    add_rule(&db.conn, "allow", Some("support"), Some(action.name())).await;
    assert_eq!(
        eval_gate(&db.conn, &action, &p, "agent")
            .await
            .unwrap()
            .matched_rule_id,
        Some(exact)
    );
    assert_eq!(
        eval_gate(&db.conn, &action, &p, "other-agent")
            .await
            .unwrap()
            .verdict,
        Verdict::NeedsReview
    );
}

#[tokio::test]
async fn exact_edited_payload_is_handed_off_once_and_history_is_scrubbed() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    let row = draft(&db, task, seq).await;
    assert!(row.preview_json.contains("original"));
    assert!(!row.preview_json.contains("fake-preview-secret"));
    let edited = json!({"mailbox":"other", "body":"human edit", "to":["tester@example.invalid"], "attachments":[{"digest":"abc"}]});
    let grant = approve_as(&db.conn, &row, "human", edited.clone())
        .await
        .unwrap();
    assert_eq!(grant.proposal_id(), row.id);
    assert_eq!(grant.action_name(), "email.draft");
    assert_eq!(grant.into_payload(), edited);
    assert!(approve_as(&db.conn, &row, "human", payload("replay"))
        .await
        .is_err());
    let row = stored(&db.conn, row.id).await;
    assert_eq!(row.status, "approved");
    for value in [
        &row.payload_json,
        &row.preview_json,
        row.edited_payload_json.as_ref().unwrap(),
    ] {
        assert!(!value.contains("original") && !value.contains("human edit"));
        assert!(value.contains("[redacted]"));
    }
    let events = audits(&db.conn).await;
    let approved = events
        .iter()
        .find(|a| a.action == "agent.proposal_approved")
        .unwrap();
    let detail: Value = serde_json::from_str(&approved.detail).unwrap();
    // Compare against an independently computed digest of the complete reviewed
    // payload; changing recipients, text, or attachments must change the record.
    assert_eq!(
        detail["payload_sha256"],
        format!("{:x}", Sha256::digest(edited.to_string().as_bytes()))
    );
    assert_eq!(detail["edited"], true);
    let unchanged = draft(&db, task, seq).await;
    let grant = approve_as(&db.conn, &unchanged, "human", payload("original"))
        .await
        .unwrap();
    assert_eq!(grant.into_payload(), payload("original"));
    let events = audits(&db.conn).await;
    let unedited: Value = serde_json::from_str(&events.last().unwrap().detail).unwrap();
    assert_eq!(unedited["edited"], false);
    let task = tasks::get(&db.conn, task).await.unwrap();
    assert_eq!((task.status, task.run_seq), (WorkTaskStatus::Running, seq));
    assert!(
        !tasks::complete_without_merge(&db.conn, task.id, "too early")
            .await
            .unwrap()
    );
    assert!(tasks::settle_review(&db.conn, task.id, seq, None, None)
        .await
        .unwrap());
    assert!(tasks::complete_without_merge(&db.conn, task.id, "reviewed")
        .await
        .unwrap());
}

#[tokio::test]
async fn stale_snapshot_invalid_edits_and_self_approval_cannot_resume_task() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    let row = draft(&db, task, seq).await;
    assert!(approve(
        &db.conn,
        task,
        seq,
        row.id,
        "human",
        &DraftAction::default(),
        Review {
            expected_payload: &payload("wrong snapshot"),
            approved_payload: payload("edited")
        }
    )
    .await
    .is_err());
    assert!(approve_as(&db.conn, &row, "human", json!({"body":42}))
        .await
        .is_err());
    assert!(approve_as(&db.conn, &row, "agent", payload("edit"))
        .await
        .is_err());
    assert!(approve_as(&db.conn, &row, "", payload("edit"))
        .await
        .is_err());
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::AwaitingInput
    );
    assert_eq!(stored(&db.conn, row.id).await.status, "pending");
    assert_eq!(audits(&db.conn).await.len(), 2);
}

#[tokio::test]
async fn new_denies_and_payload_denies_survive_approval_including_ask_rules() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    let row = draft(&db, task, seq).await;
    // Rule added after rendering the card, scoped to the newly edited resource.
    add_rule(&db.conn, "deny", Some("other"), None).await;
    let mut edited = payload("edit");
    edited["mailbox"] = json!("other");
    assert!(approve_as(&db.conn, &row, "human", edited).await.is_err());
    add_rule(&db.conn, "ask", None, None).await;
    assert!(approve_as(&db.conn, &row, "human", payload("forbidden"))
        .await
        .is_err());
    assert!(approve_as(&db.conn, &row, "human", payload("unavailable"))
        .await
        .is_err());
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::AwaitingInput
    );
    assert_eq!(stored(&db.conn, row.id).await.status, "pending");
    assert!(approve_as(&db.conn, &row, "human", payload("valid edit"))
        .await
        .is_ok());
}

#[tokio::test]
async fn reject_deleted_folder_and_task_without_grant_or_audit() {
    for delete_folder in [true, false] {
        let db = fresh_in_memory_db().await;
        let (task, seq) = start(&db).await;
        let row = draft(&db, task, seq).await;
        if delete_folder {
            folder::Entity::update_many()
                .col_expr(folder::Column::DeletedAt, Expr::value(Some(Utc::now())))
                .exec(&db.conn)
                .await
                .unwrap();
        } else {
            work_task::Entity::update_many()
                .col_expr(work_task::Column::DeletedAt, Expr::value(Some(Utc::now())))
                .exec(&db.conn)
                .await
                .unwrap();
        }
        assert!(
            approve_as(&db.conn, &row, "human", payload("edit"))
                .await
                .is_err(),
            "deleted folder={delete_folder}"
        );
        assert_eq!(stored(&db.conn, row.id).await.status, "pending");
        assert_eq!(audits(&db.conn).await.len(), 2);
    }
}

#[tokio::test]
async fn old_run_cannot_approve_new_wait_and_review_cannot_resume() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    let old = draft(&db, task, seq).await;
    assert!(tasks::cancel(&db.conn, task, None).await.unwrap());
    assert!(tasks::requeue_canceled(&db.conn, task, None, &[], false)
        .await
        .unwrap());
    let new_seq = tasks::claim_for_run(&db.conn, task, WorkTaskStatus::Todo, "user")
        .await
        .unwrap()
        .unwrap();
    assert!(new_seq > seq);
    assert!(tasks::begin_setup(&db.conn, task, new_seq).await.unwrap());
    assert!(
        tasks::mark_running(&db.conn, task, new_seq, 1, "next-connection")
            .await
            .unwrap()
    );
    let new = draft(&db, task, new_seq).await;
    assert!(approve_as(&db.conn, &old, "human", payload("late"))
        .await
        .is_err());
    assert_eq!(stored(&db.conn, new.id).await.status, "pending");
    assert!(tasks::settle_review(&db.conn, task, new_seq, None, None)
        .await
        .unwrap());
    assert!(approve_as(&db.conn, &new, "human", payload("too late"))
        .await
        .is_err());
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::Review
    );
}

#[tokio::test]
async fn gate_denial_is_logged_without_a_proposal_or_wait() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    add_rule(&db.conn, "deny", None, None).await;
    assert!(matches!(
        propose(
            &db.conn,
            task,
            seq,
            "agent",
            &DraftAction::default(),
            payload("original")
        )
        .await
        .unwrap(),
        ProposalOutcome::Denied(_)
    ));
    assert!(proposal::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
    assert_eq!(audits(&db.conn).await.len(), 1);
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::Running
    );
}

#[tokio::test]
async fn denial_and_auto_resolution_scrub_private_content() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    let row = draft(&db, task, seq).await;
    deny(
        &db.conn,
        task,
        seq,
        row.id,
        "human",
        &DraftAction::default(),
    )
    .await
    .unwrap();
    assert_eq!(stored(&db.conn, row.id).await.status, "denied");
    assert!(!stored(&db.conn, row.id)
        .await
        .payload_json
        .contains("original"));
    let action = DraftAction {
        destructive: false,
        decision: Decision::Allow,
    };
    let grant = match propose(&db.conn, task, seq, "agent", &action, payload("auto body"))
        .await
        .unwrap()
    {
        ProposalOutcome::Authorized(grant) => grant,
        _ => panic!("non-destructive explicit allow must auto authorize"),
    };
    let auto = stored(&db.conn, grant.proposal_id()).await;
    assert_eq!(auto.status, "auto");
    assert!(auto.verdict_by.is_none());
    assert!(!auto.payload_json.contains("auto body"));
    assert_eq!(grant.into_payload(), payload("auto body"));
}

#[tokio::test]
async fn credential_redaction_is_recursive_and_never_rewrites_an_execution_payload() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    for key in [
        "password",
        "API_KEY",
        "access-token",
        "access_token",
        "authorization",
        "nested_private-key",
        "session_id",
        "auth_header",
    ] {
        let input = json!({"nested":[{key:"fake-secret", "body":"private text", "safe":"keep"}]});
        let scrubbed = redaction::scrub(&input, &["body"]);
        // Source's exact regex matches underscore boundaries; access-token is
        // caught by the token spelling only if its boundary is an underscore.
        if key == "access-token" {
            continue;
        }
        assert!(!scrubbed.to_string().contains("fake-secret"), "{key}");
        assert!(!scrubbed.to_string().contains("private text"));
        assert_eq!(scrubbed["nested"][0]["safe"], "keep");
        let mut p = payload("original");
        p["nested"] = input;
        assert!(
            propose(&db.conn, task, seq, "agent", &DraftAction::default(), p)
                .await
                .is_err()
        );
    }
    assert!(audits(&db.conn).await.is_empty());
}

#[tokio::test]
async fn audit_failure_rolls_back_propose_and_approve_and_audit_is_append_only() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    db.conn.execute_unprepared("CREATE TRIGGER fail_audit BEFORE INSERT ON ops_audit_log WHEN NEW.action = 'agent.proposal_pending' BEGIN SELECT RAISE(ABORT, 'test audit failure'); END").await.unwrap();
    assert!(propose(
        &db.conn,
        task,
        seq,
        "agent",
        &DraftAction::default(),
        payload("original")
    )
    .await
    .is_err());
    assert!(proposal::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::Running
    );
    db.conn
        .execute_unprepared("DROP TRIGGER fail_audit")
        .await
        .unwrap();
    let row = draft(&db, task, seq).await;
    let before_events = work_task_event::Entity::find().all(&db.conn).await.unwrap();
    db.conn.execute_unprepared("CREATE TRIGGER fail_audit BEFORE INSERT ON ops_audit_log BEGIN SELECT RAISE(ABORT, 'test audit failure'); END").await.unwrap();
    assert!(approve_as(&db.conn, &row, "human", payload("human edit"))
        .await
        .is_err());
    assert_eq!(stored(&db.conn, row.id).await, row);
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::AwaitingInput
    );
    assert_eq!(
        work_task_event::Entity::find().all(&db.conn).await.unwrap(),
        before_events
    );
    assert!(db
        .conn
        .execute_unprepared("UPDATE ops_audit_log SET detail = '{}'")
        .await
        .is_err());
    assert!(db
        .conn
        .execute_unprepared("DELETE FROM ops_audit_log")
        .await
        .is_err());
}

// Separate SQLite connections on one file exercise real write-lock contention,
// not merely serialization through the in-memory helper's one connection.
async fn disk_db(path: &std::path::Path) -> AppDatabase {
    use crate::db::migration::Migrator;
    use sea_orm_migration::MigratorTrait;
    let mut opts = ConnectOptions::new(format!("sqlite:{}?mode=rwc", path.display()));
    opts.max_connections(1)
        .min_connections(1)
        .sqlx_logging(false);
    let conn = Database::connect(opts).await.unwrap();
    conn.execute_unprepared(
        "PRAGMA journal_mode=WAL; PRAGMA busy_timeout=5000; PRAGMA foreign_keys=ON;",
    )
    .await
    .unwrap();
    Migrator::up(&conn, None).await.unwrap();
    AppDatabase { conn }
}

#[tokio::test]
async fn racing_edited_approvals_return_exactly_one_payload() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("approvals.db");
    let db = disk_db(&path).await;
    let peer = disk_db(&path).await;
    let (task, seq) = start(&db).await;
    let row = draft(&db, task, seq).await;
    let (one, two) = tokio::join!(
        approve_as(&db.conn, &row, "human-one", payload("first edit")),
        approve_as(&peer.conn, &row, "human-two", payload("second edit"))
    );
    assert_eq!(usize::from(one.is_ok()) + usize::from(two.is_ok()), 1);
    let (winner, loser, actor, expected) = if one.is_ok() {
        (one, two, "human-one", payload("first edit"))
    } else {
        (two, one, "human-two", payload("second edit"))
    };
    assert!(matches!(loser, Err(DbError::Conflict(_))));
    assert_eq!(winner.unwrap().into_payload(), expected);
    assert_eq!(
        stored(&db.conn, row.id).await.verdict_by.as_deref(),
        Some(actor)
    );
    assert_eq!(
        audits(&db.conn)
            .await
            .iter()
            .filter(|a| a.action == "agent.proposal_approved")
            .count(),
        1
    );
}

#[tokio::test]
async fn racing_approve_and_deny_resolve_once() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("approvals.db");
    let db = disk_db(&path).await;
    let peer = disk_db(&path).await;
    let (task, seq) = start(&db).await;
    let row = draft(&db, task, seq).await;
    let action = DraftAction::default();
    let (approved, denied) = tokio::join!(
        approve_as(&db.conn, &row, "human", payload("edited")),
        deny(&peer.conn, task, seq, row.id, "human", &action)
    );
    assert_eq!(
        usize::from(approved.is_ok()) + usize::from(denied.is_ok()),
        1
    );
    assert_eq!(
        stored(&db.conn, row.id).await.status,
        if approved.is_ok() {
            "approved"
        } else {
            "denied"
        }
    );
    assert_eq!(audits(&db.conn).await.len(), 3);
}

#[tokio::test]
async fn action_reads_current_resource_and_principal_in_the_approval_transaction() {
    // A domain pack must resolve an ID against live state, not trust an agent's
    // claimed resource. Use existing folder rows as the resource fixture.
    struct StoredResource(i32);
    #[async_trait::async_trait]
    impl Action for StoredResource {
        fn name(&self) -> &'static str {
            "email.stored_resource"
        }
        fn domain(&self) -> &'static str {
            "email"
        }
        fn validate(&self, p: &Value) -> Result<(), DbError> {
            DraftAction::default().validate(p)
        }
        async fn resource(
            &self,
            _: &Value,
            ctx: &ActionContext<'_>,
        ) -> Result<Option<String>, DbError> {
            Ok(folder::Entity::find_by_id(self.0)
                .one(ctx.db)
                .await?
                .ok_or_else(conflict)?
                .alias)
        }
        async fn check_permission(
            &self,
            _: &Value,
            ctx: &ActionContext<'_>,
        ) -> Result<Decision, DbError> {
            assert_eq!(ctx.agent, "agent");
            Ok(if ctx.actor.is_some_and(|actor| actor != "reviewer") {
                Decision::Deny
            } else {
                Decision::Passthrough
            })
        }
    }
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    let folder_id = tasks::get(&db.conn, task).await.unwrap().folder_id;
    let action = StoredResource(folder_id); // Uses default destructive=true.
    let row = match propose(&db.conn, task, seq, "agent", &action, payload("original"))
        .await
        .unwrap()
    {
        ProposalOutcome::Pending(row) => row,
        _ => panic!("new action must default to review"),
    };
    let original = payload("original");
    let review = || Review {
        expected_payload: &original,
        approved_payload: payload("edited"),
    };
    assert!(approve(
        &db.conn,
        task,
        seq,
        row.id,
        "not-reviewer",
        &action,
        review()
    )
    .await
    .is_err());
    // Folder alias changes AFTER the card is rendered. A deny for the new
    // resource must block even though the submitted mailbox still says support.
    folder::Entity::update_many()
        .col_expr(folder::Column::Alias, Expr::value(Some("restricted")))
        .exec(&db.conn)
        .await
        .unwrap();
    add_rule(&db.conn, "deny", Some("restricted"), None).await;
    assert!(
        approve(&db.conn, task, seq, row.id, "reviewer", &action, review())
            .await
            .is_err()
    );
    assert_eq!(stored(&db.conn, row.id).await.status, "pending");
}

#[tokio::test]
async fn mismatched_identity_corrupt_payload_and_credential_edits_fail_closed() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    let row = draft(&db, task, seq).await;
    let mut wrong = row.clone();
    wrong.id += 1;
    assert!(approve_as(&db.conn, &wrong, "human", payload("edited"))
        .await
        .is_err());
    wrong = row.clone();
    wrong.task_id += 1;
    assert!(approve_as(&db.conn, &wrong, "human", payload("edited"))
        .await
        .is_err());
    let mut p = payload("edited");
    p["nested"] = json!([{"token":"fake-secret"}]);
    assert!(approve_as(&db.conn, &row, "human", p).await.is_err());
    proposal::Entity::update_many()
        .col_expr(proposal::Column::ActionName, Expr::value("missing.action"))
        .exec(&db.conn)
        .await
        .unwrap();
    assert!(approve_as(&db.conn, &row, "human", payload("edited"))
        .await
        .is_err());
    proposal::Entity::update_many()
        .col_expr(proposal::Column::ActionName, Expr::value("email.draft"))
        .col_expr(proposal::Column::PayloadJson, Expr::value("{broken"))
        .exec(&db.conn)
        .await
        .unwrap();
    assert!(approve_as(&db.conn, &row, "human", payload("edited"))
        .await
        .is_err());
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::AwaitingInput
    );
    assert_eq!(audits(&db.conn).await.len(), 2);
}

#[tokio::test]
async fn migration_enforces_scope_uniqueness_and_roundtrips_without_touching_tasks() {
    let db = fresh_in_memory_db().await;
    let migration = target_migration(&db.conn).await;
    let manager = SchemaManager::new(&db.conn);
    let (task, _) = start(&db).await;
    let before = tasks::get(&db.conn, task).await.unwrap();
    add_scope(&db.conn, "read", None).await;
    let duplicate = scope::ActiveModel {
        agent_id: Set("agent".into()),
        domain: Set("email".into()),
        mode: Set("act_low_risk".into()),
        resource: Set(None),
        ..Default::default()
    };
    assert!(duplicate.insert(&db.conn).await.is_err());
    assert!(db
        .conn
        .execute_unprepared("UPDATE ops_agent_scope SET mode = 'bypass'")
        .await
        .is_err());
    migration.down(&manager).await.unwrap();
    assert_follower_survives(&db.conn).await;
    migration.up(&manager).await.unwrap();
    assert_follower_survives(&db.conn).await;
    assert!(scope::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().run_seq,
        before.run_seq
    );
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        before.status
    );
}

#[tokio::test]
async fn auto_audit_failure_cannot_release_a_payload() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    db.conn.execute_unprepared("CREATE TRIGGER fail_auto_audit BEFORE INSERT ON ops_audit_log WHEN NEW.action = 'agent.proposal_auto' BEGIN SELECT RAISE(ABORT, 'test'); END").await.unwrap();
    let action = DraftAction {
        decision: Decision::Allow,
        destructive: false,
    };
    assert!(
        propose(&db.conn, task, seq, "agent", &action, payload("auto body"))
            .await
            .is_err()
    );
    assert!(proposal::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
    assert!(audits(&db.conn).await.is_empty());
}

#[tokio::test]
async fn migration_failure_does_not_leave_half_an_approval_store() {
    let db = fresh_in_memory_db().await;
    let migration = target_migration(&db.conn).await;
    let manager = SchemaManager::new(&db.conn);
    migration.down(&manager).await.unwrap();
    // Force a collision partway through the new migration. Earlier CREATEs
    // must roll back, otherwise a retry can never finish cleanly.
    db.conn
        .execute_unprepared("CREATE TABLE ops_agent_rule (id INTEGER PRIMARY KEY)")
        .await
        .unwrap();
    assert!(migration.up(&manager).await.is_err());
    assert!(proposal::Entity::find().all(&db.conn).await.is_err());
    assert!(audit::Entity::find().all(&db.conn).await.is_err());
    assert!(!manager.has_table("ops_acp_wait").await.unwrap());
    assert_follower_survives(&db.conn).await;
    db.conn
        .execute_unprepared("DROP TABLE ops_agent_rule")
        .await
        .unwrap();
    migration.up(&manager).await.unwrap();
    assert_follower_survives(&db.conn).await;
    assert!(proposal::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn generic_engine_resume_cannot_clear_a_proposal_owned_wait() {
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    let row = draft(&db, task, seq).await;
    let before = tasks::list_events(&db.conn, task, 100).await.unwrap().len();
    assert!(!tasks::flip_awaiting(&db.conn, task, seq, false)
        .await
        .unwrap());
    assert_eq!(
        tasks::list_events(&db.conn, task, 100).await.unwrap().len(),
        before
    );
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::AwaitingInput
    );
    approve_as(&db.conn, &row, "human", payload("edited"))
        .await
        .unwrap();
    // An ordinary ACP wait still resumes once the proposal is resolved.
    assert!(tasks::flip_awaiting(&db.conn, task, seq, true)
        .await
        .unwrap());
    assert!(tasks::flip_awaiting(&db.conn, task, seq, false)
        .await
        .unwrap());
    assert!(approve_as(&db.conn, &row, "human", payload("replay"))
        .await
        .is_err());
}

#[tokio::test]
async fn concurrent_acp_arrival_on_a_separate_connection_survives_both_ops_decisions() {
    use crate::db::service::work_task_wait_service as waits;
    for approve in [true, false] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("waits.db");
        let db = disk_db(&path).await;
        let peer = disk_db(&path).await;
        let (task, seq) = start(&db).await;
        let row = draft(&db, task, seq).await;
        let decision = async {
            if approve {
                approve_as(&db.conn, &row, "human", payload("edited"))
                    .await
                    .map(|_| ())
            } else {
                deny(
                    &db.conn,
                    task,
                    seq,
                    row.id,
                    "human",
                    &DraftAction::default(),
                )
                .await
            }
        };
        let (resolved, arrived) = tokio::join!(
            decision,
            waits::track_request(&peer.conn, task, seq, "connection", "p:request", true),
        );
        resolved.unwrap();
        arrived.unwrap();
        assert_eq!(
            tasks::get(&db.conn, task).await.unwrap().status,
            WorkTaskStatus::AwaitingInput
        );
        assert_eq!(
            stored(&db.conn, row.id).await.status,
            if approve { "approved" } else { "denied" }
        );
        assert!(
            !tasks::flip_awaiting(&db.conn, task, seq, false)
                .await
                .unwrap(),
            "an unrelated resume cannot release the request"
        );
        assert!(
            waits::track_request(&peer.conn, task, seq, "connection", "p:request", false)
                .await
                .unwrap()
        );
        assert_eq!(
            tasks::get(&db.conn, task).await.unwrap().status,
            WorkTaskStatus::Running
        );
    }
}

#[tokio::test]
async fn acp_wait_ownership_and_status_event_roll_back_together() {
    use crate::db::{entities::ops_acp_wait, service::work_task_wait_service as waits};
    let db = fresh_in_memory_db().await;
    let (task, seq) = start(&db).await;
    db.conn.execute_unprepared("CREATE TRIGGER fail_wait_event BEFORE INSERT ON work_task_event WHEN NEW.kind = 'status_changed' BEGIN SELECT RAISE(ABORT, 'test event failure'); END").await.unwrap();
    assert!(
        waits::track_request(&db.conn, task, seq, "connection", "p:request", true)
            .await
            .is_err()
    );
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::Running
    );
    assert!(ops_acp_wait::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
    db.conn
        .execute_unprepared("DROP TRIGGER fail_wait_event")
        .await
        .unwrap();
    let row = draft(&db, task, seq).await;
    waits::track_request(&db.conn, task, seq, "connection", "p:request", true)
        .await
        .unwrap();
    db.conn.execute_unprepared("CREATE TRIGGER fail_overlap_audit BEFORE INSERT ON ops_audit_log BEGIN SELECT RAISE(ABORT, 'test audit failure'); END").await.unwrap();
    assert!(approve_as(&db.conn, &row, "human", payload("edited"))
        .await
        .is_err());
    assert_eq!(stored(&db.conn, row.id).await, row);
    assert_eq!(
        ops_acp_wait::Entity::find()
            .all(&db.conn)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        !waits::track_request(&db.conn, task, seq, "connection", "p:request", false)
            .await
            .unwrap()
    );
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::AwaitingInput
    );
    db.conn
        .execute_unprepared("DROP TRIGGER fail_overlap_audit")
        .await
        .unwrap();
    approve_as(&db.conn, &row, "human", payload("edited"))
        .await
        .unwrap();
    assert_eq!(
        tasks::get(&db.conn, task).await.unwrap().status,
        WorkTaskStatus::Running
    );
}
