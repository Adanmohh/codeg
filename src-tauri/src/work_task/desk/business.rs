//! Full token/listener/private-generation path to the common business core.
use super::*;
use crate::business_identity::{self as identity, store as identities, types::*};
use crate::business_tasks::{self as business, types as dto, vocabulary::TaskStatus, ActorContext};

struct Fixture {
    engine: Arc<TaskEngine>,
    engineering_id: i32,
    task: dto::Detail,
    operator: identity::Principal,
    delegator: identity::Principal,
    credential_id: String,
    agent_id: String,
    listener: Arc<DelegationListener>,
    access: Arc<EngineAccess>,
    token: String,
}
async fn fixture() -> Fixture {
    let (engine, engineering_id) = running_task().await;
    fixture_engine(engine, engineering_id).await
}
async fn fixture_engine(engine: Arc<TaskEngine>, engineering_id: i32) -> Fixture {
    let db = &engine.db.conn;
    identities::bootstrap(
        db,
        BootstrapInput {
            organization_name: "Synthetic bridge team".into(),
            owner_name: "Synthetic operator".into(),
        },
    )
    .await
    .unwrap();
    let operator = identity::operator_principal(db).await.unwrap();
    let delegator_member = identities::create_member(
        db,
        &operator,
        CreateMemberInput {
            organization_id: operator.organization_id().into(),
            display_name: "Human delegator".into(),
            kind: MemberKind::Human,
            role: Role::Manager,
            domains: vec![Domain::Feedback],
        },
    )
    .await
    .unwrap();
    let issued = identities::issue_credential(
        db,
        &operator,
        IssueCredentialInput {
            organization_id: operator.organization_id().into(),
            member_id: delegator_member.id,
            label: "Synthetic delegation".into(),
        },
    )
    .await
    .unwrap();
    let delegator = identities::resolve_credential(db, &issued.token)
        .await
        .unwrap();
    let agent = identities::create_member(
        db,
        &operator,
        CreateMemberInput {
            organization_id: operator.organization_id().into(),
            display_name: "Assigned Pi".into(),
            kind: MemberKind::Agent,
            role: Role::Member,
            domains: vec![Domain::Feedback],
        },
    )
    .await
    .unwrap();
    let task = business::store::create(
        db,
        &ActorContext::authenticated(delegator.clone()),
        dto::CreateInput {
            title: "Prepare a customer follow-up".into(),
            notes: "Public task context".into(),
            domain: Domain::Feedback,
            priority: Default::default(),
            due_date: None,
            owner_id: None,
            assignee_id: Some(agent.id.clone()),
            reviewer_id: None,
        },
    )
    .await
    .unwrap();
    let (listener, access, token) = bridge(&engine, PARENT_CONN).await;
    let task = engine
        .link_business_execution(
            ActorContext::authenticated(delegator.clone()),
            dto::LinkExecutionInput {
                task_id: task.task.id,
                expected_revision: 1,
                work_task_id: engineering_id,
            },
        )
        .await
        .unwrap();
    Fixture {
        engine,
        engineering_id,
        task,
        operator,
        delegator,
        credential_id: issued.credential.id,
        agent_id: agent.id,
        listener,
        access,
        token,
    }
}

#[tokio::test]
async fn desk_business_writer_rechecks_credential_and_run_after_queued_request_and_abort() {
    for cause in ["credential", "cancel", "abort"] {
        let dir = tempfile::tempdir().unwrap();
        let db = crate::db::test_helpers::fresh_disk_db(dir.path()).await;
        let (engine, task) = running_task_in(db).await;
        let f = fixture_engine(engine, task).await;
        let writer = identity::begin_write(&f.engine.db.conn, f.operator.organization_id())
            .await
            .unwrap();
        if cause == "credential" {
            writer
                .execute(Statement::from_sql_and_values(
                    writer.get_database_backend(),
                    "UPDATE business_credential SET revoked_at = ? WHERE id = ?",
                    vec![
                        "2026-09-08T00:00:00Z".into(),
                        f.credential_id.clone().into(),
                    ],
                ))
                .await
                .unwrap();
        } else if cause == "cancel" {
            writer
                .execute(Statement::from_sql_and_values(
                    writer.get_database_backend(),
                    "UPDATE work_task SET status = 'canceled', connection_id = NULL WHERE id = ?",
                    vec![f.engineering_id.into()],
                ))
                .await
                .unwrap();
        }
        let (mut client, served) = start_call(
            f.listener.clone(),
            &f.token,
            DeskTool::DeskBusinessSubmit,
            json!({"expectedRevision":2,"body":"Queued contribution must not commit"}),
        )
        .await;
        tokio::time::timeout(Duration::from_secs(2), f.access.entered.notified())
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(!served.is_finished(), "real database writer fences request");
        if cause == "abort" {
            drop(client);
            tokio::time::timeout(Duration::from_secs(2), served)
                .await
                .unwrap()
                .unwrap();
            tokio::time::timeout(Duration::from_secs(2), f.listener.tokens.revoke(&f.token))
                .await
                .unwrap();
            writer.commit().await.unwrap();
        } else {
            writer.commit().await.unwrap();
            let response: BrokerResponse =
                tokio::time::timeout(Duration::from_secs(3), read_frame(&mut client))
                    .await
                    .unwrap()
                    .unwrap();
            served.await.unwrap();
            let outcome: DeskResponse = serde_json::from_value(response.outcome).unwrap();
            assert_eq!(
                outcome.code,
                Some(if cause == "credential" {
                    DeskError::Denied
                } else {
                    DeskError::Stale
                })
            );
        }
        let task = business::store::get(
            &f.engine.db.conn,
            &ActorContext::authenticated(f.operator.clone()),
            dto::TaskInput {
                task_id: f.task.task.id,
            },
        )
        .await
        .unwrap();
        assert_eq!(task.task.revision, 2);
        assert!(task.deliverables.is_empty());
        assert_eq!(task.activity.len(), 2);
    }
}
fn public(response: DeskResponse) -> Value {
    let value = succeeded(response);
    let text = value.to_string();
    for forbidden in [
        "delegation_json",
        "credential_id",
        "connectionId",
        "token_hash",
        "operator_owner",
        "workingDir",
    ] {
        assert!(!text.contains(forbidden), "no private lineage in response");
    }
    value
}
async fn context(f: &Fixture) -> DeskResponse {
    call(&f.listener, &f.token, DeskTool::DeskBusinessTask, json!({})).await
}

#[tokio::test]
async fn desk_business_bridge_uses_original_delegator_and_real_run_and_never_self_reviews() {
    let f = fixture().await;
    let task = public(context(&f).await);
    assert_eq!(task["task"]["id"], f.task.task.id);
    assert_eq!(task["task"]["capabilities"]["review"], false);
    assert_eq!(task["execution"]["workTaskId"], f.engineering_id);
    let submitted = public(
        call(
            &f.listener,
            &f.token,
            DeskTool::DeskBusinessSubmit,
            json!({"expectedRevision":2,"body":"Exact agent deliverable"}),
        )
        .await,
    );
    assert_eq!(submitted["task"]["status"], "review");
    assert_eq!(submitted["deliverables"][0]["author"]["id"], f.agent_id);
    assert_eq!(
        submitted["activity"][1]["actor"]["id"],
        f.delegator.member_id()
    );
    for status in ["done", "cancelled"] {
        assert_eq!(
            call(
                &f.listener,
                &f.token,
                DeskTool::DeskBusinessProgress,
                json!({"expectedRevision":3,"status":status})
            )
            .await
            .code,
            Some(DeskError::InvalidInput)
        );
    }
    for input in [
        json!({"taskId":f.task.task.id}),
        json!({"actor":"operator"}),
        json!({"organizationId":f.operator.organization_id()}),
    ] {
        assert_eq!(
            call(&f.listener, &f.token, DeskTool::DeskBusinessTask, input)
                .await
                .code,
            Some(DeskError::InvalidInput)
        );
    }
    let row = work_task_service::get_model(&f.engine.db.conn, f.engineering_id)
        .await
        .unwrap();
    let (agent, _) = business::agent::resolve(
        &f.engine.db.conn,
        business::agent::LiveExecution {
            work_task_id: row.id,
            run_seq: row.run_seq,
            connection_id: row.connection_id.unwrap(),
            agent_key: "pi".into(),
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        business::store::review(
            &f.engine.db.conn,
            &agent,
            dto::ReviewInput {
                task_id: f.task.task.id.clone(),
                expected_revision: 3,
                decision: dto::ReviewDecision::Accept,
                comment: String::new()
            }
        )
        .await,
        Err(identity::IdentityError::Forbidden)
    ));
    let done = business::store::review(
        &f.engine.db.conn,
        &ActorContext::authenticated(f.delegator),
        dto::ReviewInput {
            task_id: f.task.task.id,
            expected_revision: 3,
            decision: dto::ReviewDecision::Accept,
            comment: "Human reviewed exact deliverable".into(),
        },
    )
    .await
    .unwrap();
    assert_eq!(done.task.status, TaskStatus::Done);
    assert!(
        !call(&f.listener, &f.token, DeskTool::DeskBusinessTask, json!({}))
            .await
            .ok
    );
}

#[tokio::test]
async fn desk_business_original_member_credential_revocation_survives_persisted_link_restoration() {
    let f = fixture().await;
    assert!(context(&f).await.ok);
    let db = &f.engine.db.conn;
    // A new credential for the SAME still-active member cannot revive the old
    // delegation grant. Neither can recreating an operator principal by task ID.
    identities::issue_credential(
        db,
        &f.operator,
        IssueCredentialInput {
            organization_id: f.operator.organization_id().into(),
            member_id: f.delegator.member_id().into(),
            label: "Replacement human session".into(),
        },
    )
    .await
    .unwrap();
    identities::revoke_credential(
        db,
        &f.operator,
        RevokeCredentialInput {
            organization_id: f.operator.organization_id().into(),
            credential_id: f.credential_id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(context(&f).await.code, Some(DeskError::Denied));
    let rejected = call(
        &f.listener,
        &f.token,
        DeskTool::DeskBusinessSubmit,
        json!({"expectedRevision":2,"body":"Must not persist"}),
    )
    .await;
    assert_eq!(rejected.code, Some(DeskError::Denied));
    let task = business::store::get(
        db,
        &ActorContext::authenticated(f.operator.clone()),
        dto::TaskInput {
            task_id: f.task.task.id.clone(),
        },
    )
    .await
    .unwrap();
    assert_eq!(task.task.revision, 2);
    assert!(task.deliverables.is_empty());
    assert!(!task.execution.unwrap().active);
}

#[tokio::test]
async fn desk_business_reassignment_cancellation_and_new_generation_fence_old_launch() {
    for cause in ["reassign", "cancel", "new_run"] {
        let f = fixture().await;
        match cause {
            "reassign" => {
                business::store::assign(
                    &f.engine.db.conn,
                    &ActorContext::authenticated(f.operator.clone()),
                    dto::AssignInput {
                        task_id: f.task.task.id.clone(),
                        expected_revision: 2,
                        owner_id: f.delegator.member_id().into(),
                        assignee_id: Some(f.operator.member_id().into()),
                        reviewer_id: None,
                    },
                )
                .await
                .unwrap();
            }
            "cancel" => {
                f.engine.cancel(f.engineering_id, None).await.unwrap();
            }
            _ => {
                relaunch_on(&f.engine, f.engineering_id, "replacement-business-run").await;
            }
        }
        assert!(!context(&f).await.ok, "old run fenced after {cause}");
        assert!(
            !call(
                &f.listener,
                &f.token,
                DeskTool::DeskBusinessNote,
                json!({"expectedRevision":2,"body":"Delayed old note"})
            )
            .await
            .ok
        );
        let current = business::store::get(
            &f.engine.db.conn,
            &ActorContext::authenticated(f.operator.clone()),
            dto::TaskInput {
                task_id: f.task.task.id.clone(),
            },
        )
        .await
        .unwrap();
        assert!(current.activity.iter().all(|event| event.kind != "note"));
    }
}
