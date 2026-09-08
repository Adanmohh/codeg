//! R1 regressions use real CAS-minted, live indexed runs, never live models.
use super::*;

async fn unlinked() -> Fixture {
    let (engine, id) = running_task().await;
    unlinked_fixture_engine(engine, id).await
}
async fn entrust(f: &mut Fixture) {
    f.task = f
        .engine
        .entrust_business_execution(
            ActorContext::authenticated(f.operator.clone()),
            link_input(f, f.task.task.revision),
        )
        .await
        .unwrap();
}
async fn snapshot(f: &Fixture) -> Value {
    let detail = business::store::get(
        &f.engine.db.conn,
        &ActorContext::authenticated(f.operator.clone()),
        dto::TaskInput {
            task_id: f.task.task.id.clone(),
        },
    )
    .await
    .unwrap();
    let mut counts = Vec::new();
    for table in [
        "business_task_execution_authority",
        "business_task_execution",
        "business_task_activity",
        "business_task_deliverable",
    ] {
        let row = f
            .engine
            .db
            .conn
            .query_one(Statement::from_string(
                sea_orm::DbBackend::Sqlite,
                format!("SELECT COUNT(*) AS count FROM {table}"),
            ))
            .await
            .unwrap()
            .unwrap();
        counts.push(row.try_get::<i64>("", "count").unwrap());
    }
    json!({"detail": detail, "counts": counts})
}

#[tokio::test]
async fn desk_business_unrelated_live_run_and_owner_role_cannot_mint_source_ownership() {
    let mut f = unlinked().await;
    assert_eq!(
        f.engine.desk_scope(PARENT_CONN).await.unwrap().0.id,
        f.engineering_id
    );
    let before = snapshot(&f).await;
    assert!(matches!(
        f.engine
            .link_business_execution(
                ActorContext::authenticated(f.delegator.clone()),
                link_input(&f, 1),
            )
            .await,
        Err(identity::IdentityError::Forbidden)
    ));
    assert_eq!(snapshot(&f).await, before);
    assert!(!context(&f).await.ok);
    let owner_credential = identities::issue_credential(
        &f.engine.db.conn,
        &f.operator,
        IssueCredentialInput {
            organization_id: f.operator.organization_id().into(),
            member_id: f.operator.member_id().into(),
            label: "Owner role is not operator transport".into(),
        },
    )
    .await
    .unwrap();
    let owner = identities::resolve_credential(&f.engine.db.conn, &owner_credential.token)
        .await
        .unwrap();
    assert!(!owner.is_operator());
    for principal in [owner, f.delegator.clone()] {
        assert!(matches!(
            f.engine
                .entrust_business_execution(
                    ActorContext::authenticated(principal),
                    link_input(&f, 1),
                )
                .await,
            Err(identity::IdentityError::Forbidden)
        ));
        assert_eq!(snapshot(&f).await, before);
    }
    entrust(&mut f).await;
    assert!(
        !context(&f).await.ok,
        "source ownership alone delegates nothing"
    );
    let linked = f
        .engine
        .link_business_execution(
            ActorContext::authenticated(f.delegator.clone()),
            link_input(&f, 2),
        )
        .await
        .unwrap();
    assert_eq!(linked.task.revision, 3);
    assert_eq!(linked.activity[1].actor.id, f.operator.member_id());
    assert_eq!(linked.activity[2].actor.id, f.delegator.member_id());
    assert!(context(&f).await.ok);
    // Source proof and original linker lineage cannot be rewritten or deleted.
    for sql in [
        "UPDATE business_task_execution_authority SET agent_member_id = 'forged'",
        "DELETE FROM business_task_execution_authority",
        "UPDATE business_task_execution SET delegation_json = '{}'",
        "DELETE FROM business_task_execution",
    ] {
        assert!(f.engine.db.conn.execute_unprepared(sql).await.is_err());
    }
}

#[tokio::test]
async fn desk_business_wrong_agent_and_assignment_away_back_do_not_resurrect_entrustment() {
    let mut f = unlinked().await;
    entrust(&mut f).await;
    let other = identities::create_member(
        &f.engine.db.conn,
        &f.operator,
        CreateMemberInput {
            organization_id: f.operator.organization_id().into(),
            display_name: "Different Pi business identity".into(),
            kind: MemberKind::Agent,
            role: Role::Member,
            domains: vec![Domain::Feedback],
        },
    )
    .await
    .unwrap();
    for (assignee, expected_error) in [(other.id, "wrong"), (f.agent_id.clone(), "restored")] {
        f.task = business::store::assign(
            &f.engine.db.conn,
            &ActorContext::authenticated(f.operator.clone()),
            dto::AssignInput {
                task_id: f.task.task.id.clone(),
                expected_revision: f.task.task.revision,
                owner_id: f.delegator.member_id().into(),
                assignee_id: Some(assignee),
                reviewer_id: None,
            },
        )
        .await
        .unwrap();
        let before = snapshot(&f).await;
        let result = f
            .engine
            .link_business_execution(
                ActorContext::authenticated(f.delegator.clone()),
                link_input(&f, f.task.task.revision),
            )
            .await;
        assert!(matches!(
            (&result, expected_error),
            (Err(identity::IdentityError::Forbidden), "wrong")
                | (Err(identity::IdentityError::Conflict), "restored")
        ));
        assert_eq!(snapshot(&f).await, before);
    }
}

#[tokio::test]
async fn desk_business_scope_and_cancel_reopen_away_back_invalidate_source_revision() {
    for change in ["domain", "cancel"] {
        let mut f = unlinked().await;
        if change == "domain" {
            for member in [f.delegator.member_id(), f.agent_id.as_str()] {
                identities::update_member(
                    &f.engine.db.conn,
                    &f.operator,
                    UpdateMemberInput {
                        organization_id: f.operator.organization_id().into(),
                        member_id: member.into(),
                        expected_revision: 1,
                        display_name: "Scoped participant".into(),
                        role: if member == f.agent_id {
                            Role::Member
                        } else {
                            Role::Manager
                        },
                        domains: vec![Domain::Feedback, Domain::Marketing],
                    },
                )
                .await
                .unwrap();
            }
        }
        entrust(&mut f).await;
        let ctx = ActorContext::authenticated(f.operator.clone());
        if change == "domain" {
            for domain in [Domain::Marketing, Domain::Feedback] {
                f.task = business::store::update(
                    &f.engine.db.conn,
                    &ctx,
                    dto::UpdateInput {
                        task_id: f.task.task.id.clone(),
                        expected_revision: f.task.task.revision,
                        title: f.task.task.title.clone(),
                        notes: f.task.task.notes.clone(),
                        domain,
                        priority: f.task.task.priority,
                        due_date: f.task.task.due_date.clone(),
                    },
                )
                .await
                .unwrap();
            }
        } else {
            f.task = business::store::cancel(
                &f.engine.db.conn,
                &ctx,
                dto::RevisionInput {
                    task_id: f.task.task.id.clone(),
                    expected_revision: f.task.task.revision,
                },
            )
            .await
            .unwrap();
            f.task = business::store::progress(
                &f.engine.db.conn,
                &ctx,
                dto::ProgressInput {
                    task_id: f.task.task.id.clone(),
                    expected_revision: f.task.task.revision,
                    status: business::vocabulary::ProgressStatus::Todo,
                },
            )
            .await
            .unwrap();
        }
        let before = snapshot(&f).await;
        assert!(
            matches!(
                f.engine
                    .link_business_execution(
                        ActorContext::authenticated(f.delegator.clone()),
                        link_input(&f, f.task.task.revision),
                    )
                    .await,
                Err(identity::IdentityError::Conflict)
            ),
            "no resurrection after {change}"
        );
        assert_eq!(snapshot(&f).await, before);
    }
}

#[tokio::test]
async fn desk_business_retirement_precedes_queued_link_and_child_pi_cannot_impersonate_member() {
    let mut f = unlinked().await;
    entrust(&mut f).await;
    let before = snapshot(&f).await;
    // Hold the actual lifecycle mutex, then queue retirement before linking.
    // Tokio1.49's documented FIFO order ensures retirement is observed first.
    let guard = f.engine.request_lock.lock().await;
    let engine = f.engine.clone();
    let id = f.engineering_id;
    let mut retirement =
        tokio::spawn(async move { engine.retire_connection(PARENT_CONN, id).await });
    assert!(
        tokio::time::timeout(Duration::from_millis(30), &mut retirement)
            .await
            .is_err()
    );
    let engine = f.engine.clone();
    let ctx = ActorContext::authenticated(f.delegator.clone());
    let input = link_input(&f, 2);
    let mut linking = tokio::spawn(async move { engine.link_business_execution(ctx, input).await });
    assert!(
        tokio::time::timeout(Duration::from_millis(30), &mut linking)
            .await
            .is_err()
    );
    drop(guard);
    retirement.await.unwrap();
    assert!(matches!(
        linking.await.unwrap(),
        Err(identity::IdentityError::Conflict)
    ));
    assert_eq!(snapshot(&f).await, before);
    assert!(f.engine.desk_scope(PARENT_CONN).await.is_err());

    let f = fixture().await;
    let before = snapshot(&f).await;
    let (listener, _, token) = bridge(&f.engine, "same-pi-delegated-child").await;
    f.engine
        .delegation_parents
        .lock()
        .await
        .insert("same-pi-delegated-child".into(), PARENT_CONN.into());
    assert!(f.engine.desk_scope("same-pi-delegated-child").await.is_ok());
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskBusinessSubmit,
            json!({"expectedRevision":3,"body":"Not the entrusted business member"})
        )
        .await
        .code,
        Some(DeskError::Denied)
    );
    assert_eq!(snapshot(&f).await, before);
}

#[tokio::test]
async fn desk_business_source_revalidated_after_database_writer_changes_assignment() {
    let dir = tempfile::tempdir().unwrap();
    let (engine, id) =
        running_task_in(crate::db::test_helpers::fresh_disk_db(dir.path()).await).await;
    let mut f = unlinked_fixture_engine(engine, id).await;
    entrust(&mut f).await;
    let other = sea_orm::Database::connect(format!(
        "sqlite:{}?mode=rw",
        dir.path().join("source.db").display()
    ))
    .await
    .unwrap();
    let writer = identity::begin_write(&other, f.operator.organization_id())
        .await
        .unwrap();
    // Stand in for the competing assignment writer's final row. Its commit is
    // deliberately held after the link preflight's snapshot, before link CAS.
    writer
        .execute(Statement::from_sql_and_values(
            sea_orm::DbBackend::Sqlite,
            "UPDATE business_task SET assignee_id = ?, revision = revision + 1 WHERE id = ?",
            vec![f.operator.member_id().into(), f.task.task.id.clone().into()],
        ))
        .await
        .unwrap();
    let engine = f.engine.clone();
    let ctx = ActorContext::authenticated(f.delegator.clone());
    let input = link_input(&f, 2);
    let mut linking = tokio::spawn(async move { engine.link_business_execution(ctx, input).await });
    assert!(
        tokio::time::timeout(Duration::from_millis(80), &mut linking)
            .await
            .is_err()
    );
    assert!(
        f.engine.request_lock.try_lock().is_err(),
        "link holds engine ownership through writer wait"
    );
    writer.commit().await.unwrap();
    assert!(matches!(
        linking.await.unwrap(),
        Err(identity::IdentityError::Conflict)
    ));
    let after = snapshot(&f).await;
    assert_eq!(after["detail"]["task"]["revision"], 3);
    assert_eq!(
        after["counts"],
        json!([1, 0, 2, 0]),
        "rejected link adds no revision/activity/binding"
    );
    assert!(after["detail"]["execution"].is_null());
}
