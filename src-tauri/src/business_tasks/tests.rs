//! Real SQLite authorization/CAS behavior; no provider, engine or bearer logs.
use super::{store, types::*, vocabulary::*, ActorContext};
use crate::business_identity::{
    self as identity, store as identities, types::*, IdentityError as E, Principal,
};
use crate::db::{test_helpers::fresh_in_memory_db, AppDatabase};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::json;

mod fixture;
mod http;
mod transactions;

pub(crate) async fn initialize(conn: &DatabaseConnection) -> Principal {
    identities::bootstrap(
        conn,
        BootstrapInput {
            organization_name: "Synthetic task team".into(),
            owner_name: "Synthetic owner".into(),
        },
    )
    .await
    .unwrap();
    identity::operator_principal(conn).await.unwrap()
}
async fn human(
    conn: &DatabaseConnection,
    op: &Principal,
    role: Role,
    domains: Vec<Domain>,
) -> (Member, IssuedCredential, ActorContext) {
    let member = identities::create_member(
        conn,
        op,
        CreateMemberInput {
            organization_id: op.organization_id().into(),
            display_name: format!("Synthetic {role:?}"),
            kind: MemberKind::Human,
            role,
            domains,
        },
    )
    .await
    .unwrap();
    let credential = identities::issue_credential(
        conn,
        op,
        IssueCredentialInput {
            organization_id: op.organization_id().into(),
            member_id: member.id.clone(),
            label: "Synthetic task session".into(),
        },
    )
    .await
    .unwrap();
    let principal = identities::resolve_credential(conn, &credential.token)
        .await
        .unwrap();
    (member, credential, ActorContext::authenticated(principal))
}
fn create_input() -> CreateInput {
    CreateInput {
        title: "Follow up on customer feedback".into(),
        notes: "Human work; no engineering folder".into(),
        domain: Domain::Feedback,
        priority: TaskPriority::Normal,
        due_date: Some("2028-02-29".into()),
        owner_id: None,
        assignee_id: None,
        reviewer_id: None,
    }
}
fn update_input(task: &Task) -> UpdateInput {
    UpdateInput {
        task_id: task.id.clone(),
        expected_revision: task.revision,
        title: task.title.clone(),
        notes: task.notes.clone(),
        domain: task.domain,
        priority: task.priority,
        due_date: task.due_date.clone(),
    }
}
async fn count(conn: &DatabaseConnection, sql: &str) -> i64 {
    conn.query_one(store::statement(sql, vec![]))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "count")
        .unwrap()
}
async fn create_owned(db: &AppDatabase) -> (Principal, ActorContext, Detail) {
    let op = initialize(&db.conn).await;
    let ctx = ActorContext::authenticated(op.clone());
    let detail = store::create(&db.conn, &ctx, create_input()).await.unwrap();
    (op, ctx, detail)
}

#[tokio::test]
async fn intake_task_transaction_rolls_back_creation_and_preserves_explicit_editor_owner() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let (editor, _, ctx) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    let (_, _, other) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    let tx = identity::begin_write(&db.conn, op.organization_id())
        .await
        .unwrap();
    let prepared = store::prepare_in_transaction(&tx, &ctx, create_input())
        .await
        .unwrap();
    assert_eq!(prepared.owner_id, editor.id);
    assert_eq!(prepared.due_date.as_deref(), Some("2028-02-29"));
    assert!(matches!(
        store::create_in_transaction(&tx, &other, prepared.clone().into()).await,
        Err(E::Forbidden)
    ));
    let created = store::create_in_transaction(&tx, &ctx, prepared.into())
        .await
        .unwrap();
    assert_eq!(created.activity.len(), 1);
    tx.rollback().await.unwrap();
    assert_eq!(
        count(&db.conn, "SELECT COUNT(*) AS count FROM business_task").await,
        0
    );
    assert_eq!(
        count(
            &db.conn,
            "SELECT COUNT(*) AS count FROM business_task_activity"
        )
        .await,
        0
    );
}

#[tokio::test]
async fn intake_task_source_link_is_atomic_text_free_and_invalidates_review() {
    let db = fresh_in_memory_db().await;
    let (op, ctx, created) = create_owned(&db).await;
    let reviewed = store::progress(
        &db.conn,
        &ctx,
        ProgressInput {
            task_id: created.task.id.clone(),
            expected_revision: 1,
            status: ProgressStatus::Review,
        },
    )
    .await
    .unwrap();
    let link_id = uuid::Uuid::new_v4().to_string();
    let tx = identity::begin_write(&db.conn, op.organization_id())
        .await
        .unwrap();
    assert!(matches!(
        store::link_source_in_transaction(
            &tx,
            &ctx,
            &created.task.id,
            1,
            Domain::Feedback,
            &link_id
        )
        .await,
        Err(E::Conflict)
    ));
    let linked = store::link_source_in_transaction(
        &tx,
        &ctx,
        &created.task.id,
        2,
        Domain::Feedback,
        &link_id,
    )
    .await
    .unwrap();
    assert_eq!(linked.task.title, reviewed.task.title);
    assert_eq!(linked.task.notes, reviewed.task.notes);
    assert_eq!(linked.task.status, TaskStatus::InProgress);
    assert_eq!(
        linked.activity.last().unwrap().payload,
        json!({"linkId": link_id})
    );
    tx.rollback().await.unwrap();
    let unchanged = store::get(
        &db.conn,
        &ctx,
        TaskInput {
            task_id: created.task.id,
        },
    )
    .await
    .unwrap();
    assert_eq!(unchanged.task.revision, 2);
    assert_eq!(unchanged.task.status, TaskStatus::Review);
    assert_eq!(unchanged.activity.len(), 2);
}

#[tokio::test]
async fn human_only_work_progresses_to_real_human_review_without_folder_or_agent() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let (member, _, ctx) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    let (_, _, reviewer) = human(&db.conn, &op, Role::Manager, vec![Domain::Feedback]).await;
    let task = store::create(&db.conn, &ctx, create_input()).await.unwrap();
    assert_eq!(task.task.owner_id, member.id);
    assert!(
        task.task.assignee_id.is_none()
            && task.task.reviewer_id.is_none()
            && task.execution.is_none()
    );
    assert_eq!(task.task.due_date.as_deref(), Some("2028-02-29"));
    assert_eq!(
        count(&db.conn, "SELECT COUNT(*) AS count FROM work_task").await,
        0
    );
    let running = store::progress(
        &db.conn,
        &ctx,
        ProgressInput {
            task_id: task.task.id.clone(),
            expected_revision: 1,
            status: ProgressStatus::InProgress,
        },
    )
    .await
    .unwrap();
    let review = store::progress(
        &db.conn,
        &ctx,
        ProgressInput {
            task_id: task.task.id,
            expected_revision: running.task.revision,
            status: ProgressStatus::Review,
        },
    )
    .await
    .unwrap();
    assert!(review.deliverables.is_empty());
    let input = ReviewInput {
        task_id: review.task.id,
        expected_revision: review.task.revision,
        decision: ReviewDecision::Accept,
        comment: "Checked the customer follow-up".into(),
    };
    assert!(matches!(
        store::review(&db.conn, &ctx, input.clone()).await,
        Err(E::Forbidden)
    ));
    let done = store::review(&db.conn, &reviewer, input).await.unwrap();
    assert_eq!(done.task.status, TaskStatus::Done);
    assert_eq!(done.task.revision, 4);
    assert_eq!(done.activity.len(), 4);
    assert_eq!(
        done.activity.last().unwrap().actor.id,
        reviewer.principal.member_id()
    );
    assert_eq!(done.task.due_date.as_deref(), Some("2028-02-29"));
}

#[tokio::test]
async fn domain_visibility_viewer_denial_and_destination_references_are_real_authorization() {
    let db = fresh_in_memory_db().await;
    let (op, owner, task) = create_owned(&db).await;
    let (_, _, viewer) = human(&db.conn, &op, Role::Viewer, vec![Domain::Feedback]).await;
    let (_, _, other_domain) = human(&db.conn, &op, Role::Manager, vec![Domain::Marketing]).await;
    let (_, _, other_member) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    let detail_input = || TaskInput {
        task_id: task.task.id.clone(),
    };
    let read = store::get(&db.conn, &viewer, detail_input()).await.unwrap();
    assert!(
        !read.task.capabilities.edit
            && !read.task.capabilities.comment
            && !read.task.capabilities.review
    );
    assert!(matches!(
        store::get(&db.conn, &other_domain, detail_input()).await,
        Err(E::NotFound)
    ));
    assert!(store::list(&db.conn, &other_domain, ListInput::default())
        .await
        .unwrap()
        .tasks
        .is_empty());
    for ctx in [&viewer, &other_member] {
        assert!(matches!(
            store::update(&db.conn, ctx, update_input(&task.task)).await,
            Err(E::Forbidden)
        ));
    }
    assert!(matches!(
        store::create(&db.conn, &viewer, create_input()).await,
        Err(E::Forbidden)
    ));
    assert!(matches!(
        store::create(&db.conn, &other_domain, create_input()).await,
        Err(E::Forbidden)
    ));
    let mut invalid_owner = create_input();
    invalid_owner.owner_id = Some(viewer.principal.member_id().into());
    assert!(matches!(
        store::create(&db.conn, &owner, invalid_owner).await,
        Err(E::NotFound)
    ));
    let mut invalid_reviewer = create_input();
    invalid_reviewer.reviewer_id = Some(other_member.principal.member_id().into());
    assert!(matches!(
        store::create(&db.conn, &owner, invalid_reviewer).await,
        Err(E::NotFound)
    ));
    let mut missing_ref = create_input();
    missing_ref.assignee_id = Some(uuid::Uuid::new_v4().to_string());
    assert!(matches!(
        store::create(&db.conn, &owner, missing_ref).await,
        Err(E::NotFound)
    ));
    let mut unauthorized_assign = create_input();
    unauthorized_assign.assignee_id = Some(op.member_id().into());
    assert!(matches!(
        store::create(&db.conn, &other_member, unauthorized_assign).await,
        Err(E::Forbidden)
    ));
    // Destination exists but current assignee cannot access it: same check on
    // metadata movement as on creation/assignment, unlike the source omission.
    let assigned = store::assign(
        &db.conn,
        &owner,
        AssignInput {
            task_id: task.task.id,
            expected_revision: 1,
            owner_id: op.member_id().into(),
            assignee_id: Some(other_member.principal.member_id().into()),
            reviewer_id: None,
        },
    )
    .await
    .unwrap();
    let mut moved = update_input(&assigned.task);
    moved.domain = Domain::Marketing;
    assert!(matches!(
        store::update(&db.conn, &owner, moved).await,
        Err(E::NotFound)
    ));
    assert_eq!(
        count(&db.conn, "SELECT COUNT(*) AS count FROM business_task").await,
        1
    );
}

#[tokio::test]
async fn edited_payload_invalidates_review_and_stale_acceptance_cannot_complete_it() {
    let db = fresh_in_memory_db().await;
    let (_, ctx, task) = create_owned(&db).await;
    let submitted = store::submit(
        &db.conn,
        &ctx,
        TextInput {
            task_id: task.task.id,
            expected_revision: 1,
            body: "Exact original deliverable".into(),
        },
    )
    .await
    .unwrap();
    let accept = ReviewInput {
        task_id: submitted.task.id.clone(),
        expected_revision: submitted.task.revision,
        decision: ReviewDecision::Accept,
        comment: String::new(),
    };
    let mut edit = update_input(&submitted.task);
    edit.notes = "Changed required outcome".into();
    let edited = store::update(&db.conn, &ctx, edit).await.unwrap();
    assert_eq!(edited.task.status, TaskStatus::InProgress);
    assert!(edited.task.current_deliverable_id.is_none());
    assert_eq!(edited.deliverables[0].body, "Exact original deliverable");
    assert!(matches!(
        store::review(&db.conn, &ctx, accept.clone()).await,
        Err(E::Conflict)
    ));
    let mut fresh = accept;
    fresh.expected_revision = edited.task.revision;
    assert!(matches!(
        store::review(&db.conn, &ctx, fresh).await,
        Err(E::Forbidden)
    ));
    let pending = store::progress(
        &db.conn,
        &ctx,
        ProgressInput {
            task_id: edited.task.id,
            expected_revision: edited.task.revision,
            status: ProgressStatus::Review,
        },
    )
    .await
    .unwrap();
    let done = store::review(
        &db.conn,
        &ctx,
        ReviewInput {
            task_id: pending.task.id,
            expected_revision: pending.task.revision,
            decision: ReviewDecision::Accept,
            comment: "Human checked changed work".into(),
        },
    )
    .await
    .unwrap();
    let archived = store::archive(
        &db.conn,
        &ctx,
        ArchiveInput {
            task_id: done.task.id,
            expected_revision: done.task.revision,
            archived: true,
        },
    )
    .await
    .unwrap();
    assert!(archived.task.archived_at.is_some());
    assert_eq!(archived.deliverables.len(), 1);
    assert!(store::list(&db.conn, &ctx, ListInput::default())
        .await
        .unwrap()
        .tasks
        .is_empty());
    assert_eq!(
        store::list(
            &db.conn,
            &ctx,
            ListInput {
                archived: true,
                ..Default::default()
            }
        )
        .await
        .unwrap()
        .tasks
        .len(),
        1
    );
}

#[tokio::test]
async fn due_dates_are_calendar_values_and_closed_inputs_cannot_bypass_review_or_identity() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let ctx = ActorContext::authenticated(op);
    for date in [
        "2026-02-29",
        "0000-01-01",
        "2026-2-09",
        "2026-09-08T00:00:00Z",
        "2026-09-08+03:00",
        "2026-13-01",
    ] {
        let mut input = create_input();
        input.due_date = Some(date.into());
        assert!(matches!(
            store::create(&db.conn, &ctx, input).await,
            Err(E::Invalid(_))
        ));
    }
    for date in ["0001-01-01", "2028-02-29", "9999-12-31"] {
        let mut input = create_input();
        input.due_date = Some(date.into());
        let task = store::create(&db.conn, &ctx, input).await.unwrap();
        assert_eq!(task.task.due_date.as_deref(), Some(date));
        let mut update = update_input(&task.task);
        update.due_date = None;
        assert!(store::update(&db.conn, &ctx, update)
            .await
            .unwrap()
            .task
            .due_date
            .is_none());
    }
    for status in ["done", "cancelled"] {
        assert!(serde_json::from_value::<ProgressInput>(
            json!({"taskId":"ignored", "expectedRevision":1, "status":status})
        )
        .is_err());
    }
    for field in [
        "actor",
        "organizationId",
        "role",
        "status",
        "revision",
        "credential",
    ] {
        let mut input = json!({"title":"Closed", "domain":"feedback"});
        input[field] = json!("forged");
        assert!(serde_json::from_value::<CreateInput>(input).is_err());
    }
}

#[tokio::test]
async fn list_filters_are_bounded_and_literal_and_mine_uses_real_ownership() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let (_, _, member) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    let owner = ActorContext::authenticated(op);
    for i in 0..52 {
        let mut input = create_input();
        input.title = format!("Shared follow up {i}");
        store::create(&db.conn, &owner, input).await.unwrap();
    }
    let mut mine = create_input();
    mine.title = "A literal 100% customer request".into();
    store::create(&db.conn, &member, mine).await.unwrap();
    let page = store::list(&db.conn, &member, ListInput::default())
        .await
        .unwrap();
    assert_eq!(page.tasks.len(), 50);
    assert!(page.has_more);
    let next = store::list(
        &db.conn,
        &member,
        ListInput {
            page: 1,
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(next.tasks.len(), 3);
    assert!(!next.has_more);
    let mine = store::list(
        &db.conn,
        &member,
        ListInput {
            view: TaskView::Mine,
            ..Default::default()
        },
    )
    .await
    .unwrap();
    assert_eq!(mine.tasks.len(), 1);
    assert_eq!(
        store::list(
            &db.conn,
            &member,
            ListInput {
                query: Some("%".into()),
                ..Default::default()
            }
        )
        .await
        .unwrap()
        .tasks
        .len(),
        1
    );
}
