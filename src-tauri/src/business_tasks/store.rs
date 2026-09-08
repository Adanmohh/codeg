//! Codeg task CAS + event transaction, extended with current org authorization.
use super::{
    agent,
    entity::{activity, deliverable, execution, task},
    policy,
    types::*,
    validation::*,
    vocabulary::*,
    ActorContext,
};
use crate::business_identity::{
    self as identity, IdentityError as E, Member, MemberKind, Permission,
};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, FromQueryResult,
    Statement, TransactionTrait, Value,
};
use serde_json::json;

pub(super) fn statement(sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values)
}
fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}
fn id() -> String {
    uuid::Uuid::new_v4().to_string()
}
fn kind(member: &Member) -> &'static str {
    match member.kind {
        MemberKind::Human => "human",
        MemberKind::Agent => "agent",
    }
}
pub(super) fn domain(row: &task::Model) -> Result<TaskDomain, E> {
    serde_json::from_value(json!(row.domain)).map_err(|_| E::Invalid("Invalid stored task domain"))
}
fn metadata(title: &str, notes: &str, due_date: Option<&str>) -> Result<(), E> {
    if !valid_text(title, MAX_TITLE, true) || !valid_text(notes, MAX_TEXT, false) {
        return Err(E::Invalid(
            "Use a nonblank title up to 240 characters and notes up to 20000 characters",
        ));
    }
    if !valid_due_date(due_date) {
        return Err(E::Invalid("Use a valid calendar date YYYY-MM-DD"));
    }
    Ok(())
}
fn body(text: &str, required: bool) -> Result<(), E> {
    if valid_text(text, MAX_TEXT, required) {
        Ok(())
    } else {
        Err(E::Invalid("Use plain text up to 20000 characters"))
    }
}
pub(super) async fn model<C: ConnectionTrait>(
    conn: &C,
    org: &str,
    task_id: &str,
) -> Result<task::Model, E> {
    task::Model::find_by_statement(statement(
        "SELECT * FROM business_task WHERE organization_id = ? AND id = ?",
        vec![org.into(), task_id.into()],
    ))
    .one(conn)
    .await?
    .ok_or(E::NotFound)
}
async fn visible<C: ConnectionTrait>(
    conn: &C,
    ctx: &ActorContext,
    row: &task::Model,
    permission: Permission,
) -> Result<Member, E> {
    let member = identity::authorize(
        conn,
        &ctx.principal,
        &row.organization_id,
        Permission::Read,
        Some(domain(row)?),
    )
    .await
    .map_err(|e| {
        if matches!(e, E::Forbidden) {
            E::NotFound
        } else {
            e
        }
    })?;
    agent::scope(conn, ctx, row, member.kind).await?;
    if permission != Permission::Read {
        identity::authorize(
            conn,
            &ctx.principal,
            &row.organization_id,
            permission,
            Some(domain(row)?),
        )
        .await?;
    }
    Ok(member)
}
pub(super) async fn references<C: ConnectionTrait>(conn: &C, row: &task::Model) -> Result<(), E> {
    let d = domain(row)?;
    let owner = identity::active_reference(conn, &row.organization_id, &row.owner_id, d).await?;
    if owner.kind != MemberKind::Human || !owner.allows(Permission::Contribute, Some(d)) {
        return Err(E::NotFound);
    }
    if let Some(id) = &row.assignee_id {
        let assignee = identity::active_reference(conn, &row.organization_id, id, d).await?;
        if !assignee.allows(Permission::Contribute, Some(d)) {
            return Err(E::NotFound);
        }
    }
    if let Some(id) = &row.reviewer_id {
        let reviewer = identity::active_reference(conn, &row.organization_id, id, d).await?;
        if reviewer.kind != MemberKind::Human || !reviewer.allows(Permission::Review, Some(d)) {
            return Err(E::NotFound);
        }
    }
    Ok(())
}
async fn caps<C: ConnectionTrait>(
    conn: &C,
    ctx: &ActorContext,
    row: &task::Model,
    member: &Member,
) -> Result<Capabilities, E> {
    let d = domain(row)?;
    let mut result = policy::capabilities(row, member, d);
    match references(conn, row).await {
        Ok(()) => {}
        Err(E::NotFound) => {
            result.edit = false;
            result.progress = false;
            result.review = false;
            result.comment = false;
            result.submit = false;
            result.link_execution = false;
        }
        Err(e) => return Err(e),
    }
    if member.kind == MemberKind::Agent {
        match identity::authorize(
            conn,
            &ctx.principal,
            &row.organization_id,
            Permission::Contribute,
            Some(d),
        )
        .await
        {
            Ok(_) => {}
            Err(E::Forbidden) => {
                result.progress = false;
                result.comment = false;
                result.submit = false;
            }
            Err(e) => return Err(e),
        }
    }
    if result.link_execution {
        result.link_execution = match &row.assignee_id {
            Some(id) => identity::active_reference(conn, &row.organization_id, id, d)
                .await
                .is_ok_and(|m| {
                    m.kind == MemberKind::Agent && m.allows(Permission::Contribute, Some(d))
                }),
            None => false,
        };
    }
    Ok(result)
}
async fn task_dto<C: ConnectionTrait>(
    conn: &C,
    ctx: &ActorContext,
    row: task::Model,
    member: &Member,
) -> Result<Task, E> {
    let capabilities = caps(conn, ctx, &row, member).await?;
    let d = domain(&row)?;
    Ok(Task {
        id: row.id,
        organization_id: row.organization_id,
        title: row.title,
        notes: row.notes,
        domain: d,
        status: row.status,
        priority: row.priority,
        due_date: row.due_date,
        owner_id: row.owner_id,
        assignee_id: row.assignee_id,
        creator_id: row.creator_id,
        reviewer_id: row.reviewer_id,
        revision: row.revision,
        current_deliverable_id: row.current_deliverable_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
        archived_at: row.archived_at,
        capabilities,
    })
}
async fn detail_in<C: ConnectionTrait>(
    conn: &C,
    ctx: &ActorContext,
    row: task::Model,
) -> Result<Detail, E> {
    let actor = visible(conn, ctx, &row, Permission::Read).await?;
    let activity = activity::Model::find_by_statement(statement(
        "SELECT * FROM business_task_activity WHERE organization_id = ? AND task_id = ? ORDER BY revision",
        vec![row.organization_id.clone().into(), row.id.clone().into()],
    )).all(conn).await?.into_iter().map(|a| Ok(Activity {
        id: a.id, revision: a.revision, kind: a.kind,
        actor: Actor { id: a.actor_id, display_name: a.actor_name, kind: a.actor_kind },
        payload: serde_json::from_str(&a.payload_json).map_err(|_| E::Invalid("Invalid stored task activity"))?, created_at: a.created_at,
    })).collect::<Result<Vec<_>, E>>()?;
    let deliverables = deliverable::Model::find_by_statement(statement(
        "SELECT * FROM business_task_deliverable WHERE organization_id = ? AND task_id = ? ORDER BY revision",
        vec![row.organization_id.clone().into(), row.id.clone().into()],
    )).all(conn).await?.into_iter().map(|d| Deliverable { id: d.id, revision: d.revision,
        author: Actor { id: d.author_id, display_name: d.author_name, kind: d.author_kind }, body: d.body, created_at: d.created_at }).collect();
    let execution = match &row.current_execution_id {
        None => None,
        Some(id) => {
            let binding = execution::Model::find_by_statement(statement(
                "SELECT * FROM business_task_execution WHERE organization_id = ? AND task_id = ? AND id = ?",
                vec![row.organization_id.clone().into(), row.id.clone().into(), id.clone().into()],
            )).one(conn).await?.ok_or(E::NotFound)?;
            let active = agent::active(conn, &row, &binding).await?;
            Some(Execution {
                work_task_id: binding.work_task_id,
                run_seq: binding.run_seq,
                agent_member_id: binding.agent_member_id,
                active,
            })
        }
    };
    Ok(Detail {
        task: task_dto(conn, ctx, row, &actor).await?,
        activity,
        deliverables,
        execution,
    })
}

pub async fn get(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: TaskInput,
) -> Result<Detail, E> {
    let tx = conn.begin().await?;
    let row = model(&tx, ctx.principal.organization_id(), &input.task_id).await?;
    let detail = detail_in(&tx, ctx, row).await?;
    tx.commit().await?;
    Ok(detail)
}

pub async fn list(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: ListInput,
) -> Result<TaskPage, E> {
    if input.page > 100_000
        || input
            .query
            .as_deref()
            .is_some_and(|q| !valid_text(q, 240, false))
    {
        return Err(E::Invalid("Invalid task filter"));
    }
    let tx = conn.begin().await?;
    let org = ctx.principal.organization_id();
    let actor =
        identity::authorize(&tx, &ctx.principal, org, Permission::Read, input.domain).await?;
    // Agent context has one linked task; no organization-wide list surface.
    if actor.kind != MemberKind::Human || ctx.live.is_some() {
        return Err(E::Forbidden);
    }
    let allowed: Vec<_> = TaskDomain::ALL
        .into_iter()
        .filter(|d| {
            actor.allows(Permission::Read, Some(*d))
                && input.domain.is_none_or(|wanted| wanted == *d)
        })
        .collect();
    let can_create = allowed
        .iter()
        .any(|d| actor.allows(Permission::Create, Some(*d)));
    if allowed.is_empty() {
        return Ok(TaskPage {
            tasks: vec![],
            page: input.page,
            has_more: false,
            can_create,
        });
    }
    let placeholders = vec!["?"; allowed.len()].join(",");
    let mut sql = format!("SELECT * FROM business_task WHERE organization_id = ? AND domain IN ({placeholders}) AND archived_at IS {}", if input.archived { "NOT NULL" } else { "NULL" });
    let mut values: Vec<Value> = vec![org.into()];
    values.extend(allowed.into_iter().map(|d| domain_key(d).into()));
    if matches!(input.view, TaskView::Mine) {
        sql.push_str(" AND (owner_id = ? OR assignee_id = ? OR reviewer_id = ?)");
        values.extend([
            actor.id.clone().into(),
            actor.id.clone().into(),
            actor.id.clone().into(),
        ]);
    }
    if let Some(s) = input.status {
        sql.push_str(" AND status = ?");
        values.push(status_key(s).into());
    }
    if let Some(q) = input.query.filter(|q| !q.trim().is_empty()) {
        sql.push_str(
            " AND (instr(lower(title), lower(?)) > 0 OR instr(lower(notes), lower(?)) > 0)",
        );
        values.extend([q.clone().into(), q.into()]);
    }
    sql.push_str(" ORDER BY updated_at DESC, id LIMIT 51 OFFSET ?");
    values.push((i64::from(input.page) * 50).into());
    let mut rows = task::Model::find_by_statement(statement(&sql, values))
        .all(&tx)
        .await?;
    let has_more = rows.len() > 50;
    rows.truncate(50);
    let mut tasks = Vec::with_capacity(rows.len());
    for row in rows {
        tasks.push(task_dto(&tx, ctx, row, &actor).await?);
    }
    tx.commit().await?;
    Ok(TaskPage {
        tasks,
        page: input.page,
        has_more,
        can_create,
    })
}

async fn audit<C: ConnectionTrait>(
    conn: &C,
    row: &task::Model,
    actor: &Member,
    action: &str,
    payload: serde_json::Value,
) -> Result<(), E> {
    conn.execute(statement(
        "INSERT INTO business_task_activity (id,organization_id,task_id,revision,kind,actor_id,actor_name,actor_kind,payload_json,created_at) VALUES (?,?,?,?,?,?,?,?,?,?)",
        vec![id().into(), row.organization_id.clone().into(), row.id.clone().into(), row.revision.into(), action.into(),
            actor.id.clone().into(), actor.display_name.clone().into(), kind(actor).into(), payload.to_string().into(), row.updated_at.clone().into()],
    )).await?;
    Ok(())
}

pub async fn create(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: CreateInput,
) -> Result<Detail, E> {
    metadata(&input.title, &input.notes, input.due_date.as_deref())?;
    let org = ctx.principal.organization_id();
    let tx = identity::begin_write(conn, org).await?;
    let actor = identity::authorize(
        &tx,
        &ctx.principal,
        org,
        Permission::Create,
        Some(input.domain),
    )
    .await?;
    if actor.kind != MemberKind::Human || ctx.live.is_some() {
        return Err(E::Forbidden);
    }
    let owner_id = input.owner_id.unwrap_or_else(|| actor.id.clone());
    if owner_id != actor.id
        || input
            .assignee_id
            .as_deref()
            .is_some_and(|id| id != actor.id)
        || input.reviewer_id.is_some()
    {
        identity::authorize(
            &tx,
            &ctx.principal,
            org,
            Permission::Assign,
            Some(input.domain),
        )
        .await?;
    }
    let timestamp = now();
    let row = task::Model {
        id: id(),
        organization_id: org.to_owned(),
        title: input.title,
        notes: input.notes,
        domain: domain_key(input.domain).to_owned(),
        status: TaskStatus::Todo,
        priority: input.priority,
        due_date: input.due_date,
        owner_id,
        assignee_id: input.assignee_id,
        creator_id: actor.id.clone(),
        reviewer_id: input.reviewer_id,
        revision: 1,
        current_deliverable_id: None,
        current_execution_id: None,
        created_at: timestamp.clone(),
        updated_at: timestamp,
        archived_at: None,
    };
    references(&tx, &row).await?;
    tx.execute(statement("INSERT INTO business_task (id,organization_id,title,notes,domain,priority,due_date,owner_id,assignee_id,creator_id,reviewer_id,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)",
        vec![row.id.clone().into(), org.into(), row.title.clone().into(), row.notes.clone().into(), row.domain.clone().into(), priority_key(row.priority).into(),
            row.due_date.clone().into(), row.owner_id.clone().into(), row.assignee_id.clone().into(), row.creator_id.clone().into(), row.reviewer_id.clone().into(), row.created_at.clone().into(), row.updated_at.clone().into()])).await?;
    audit(&tx, &row, &actor, "created", json!({"title": row.title, "domain": input.domain, "ownerId": row.owner_id, "assigneeId": row.assignee_id, "reviewerId": row.reviewer_id, "dueDate": row.due_date})).await?;
    let detail = detail_in(&tx, ctx, row).await?;
    tx.commit().await?;
    Ok(detail)
}

async fn begin_task(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    task_id: &str,
    revision: i64,
) -> Result<(DatabaseTransaction, task::Model, Member, Capabilities), E> {
    if revision <= 0 {
        return Err(E::Invalid("expectedRevision must be positive"));
    }
    let tx = identity::begin_write(conn, ctx.principal.organization_id()).await?;
    let row = model(&tx, ctx.principal.organization_id(), task_id).await?;
    let actor = visible(&tx, ctx, &row, Permission::Contribute).await?;
    if row.revision != revision {
        return Err(E::Conflict);
    }
    let capabilities = caps(&tx, ctx, &row, &actor).await?;
    Ok((tx, row, actor, capabilities))
}
fn require(allowed: bool) -> Result<(), E> {
    if allowed {
        Ok(())
    } else {
        Err(E::Forbidden)
    }
}

async fn revoke<C: ConnectionTrait>(conn: &C, row: &task::Model) -> Result<(), E> {
    conn.execute(statement("UPDATE business_task_execution SET revoked_at = ? WHERE organization_id = ? AND task_id = ? AND revoked_at IS NULL",
        vec![now().into(), row.organization_id.clone().into(), row.id.clone().into()])).await?;
    Ok(())
}
fn invalidate_review(row: &mut task::Model) {
    if row.status == TaskStatus::Review {
        row.status = TaskStatus::InProgress;
    }
    row.current_deliverable_id = None;
}
async fn finish(
    tx: DatabaseTransaction,
    ctx: &ActorContext,
    mut row: task::Model,
    actor: &Member,
    action: &str,
    payload: serde_json::Value,
) -> Result<Detail, E> {
    let expected = row.revision;
    row.revision = expected.checked_add(1).ok_or(E::Conflict)?;
    row.updated_at = now();
    let changed = tx.execute(statement("UPDATE business_task SET title=?,notes=?,domain=?,status=?,priority=?,due_date=?,owner_id=?,assignee_id=?,reviewer_id=?,revision=?,current_deliverable_id=?,current_execution_id=?,updated_at=?,archived_at=? WHERE organization_id=? AND id=? AND revision=?",
        vec![row.title.clone().into(), row.notes.clone().into(), row.domain.clone().into(), status_key(row.status).into(), priority_key(row.priority).into(), row.due_date.clone().into(),
            row.owner_id.clone().into(), row.assignee_id.clone().into(), row.reviewer_id.clone().into(), row.revision.into(), row.current_deliverable_id.clone().into(), row.current_execution_id.clone().into(), row.updated_at.clone().into(), row.archived_at.clone().into(), row.organization_id.clone().into(), row.id.clone().into(), expected.into()])).await?;
    if changed.rows_affected() != 1 {
        return Err(E::Conflict);
    }
    audit(&tx, &row, actor, action, payload).await?;
    let detail = detail_in(&tx, ctx, row).await?;
    tx.commit().await?;
    Ok(detail)
}

pub async fn update(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: UpdateInput,
) -> Result<Detail, E> {
    metadata(&input.title, &input.notes, input.due_date.as_deref())?;
    let (tx, mut row, actor, caps) =
        begin_task(conn, ctx, &input.task_id, input.expected_revision).await?;
    require(caps.edit)?;
    identity::authorize(
        &tx,
        &ctx.principal,
        &row.organization_id,
        Permission::Contribute,
        Some(input.domain),
    )
    .await?;
    let changed_domain = domain(&row)? != input.domain;
    row.title = input.title;
    row.notes = input.notes;
    row.domain = domain_key(input.domain).into();
    row.priority = input.priority;
    row.due_date = input.due_date;
    references(&tx, &row).await?;
    if changed_domain {
        revoke(&tx, &row).await?;
    }
    invalidate_review(&mut row);
    let payload = json!({"title": row.title, "notes": row.notes, "domain": input.domain, "priority": row.priority, "dueDate": row.due_date});
    finish(tx, ctx, row, &actor, "updated", payload).await
}

pub async fn assign(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: AssignInput,
) -> Result<Detail, E> {
    let (tx, mut row, actor, caps) =
        begin_task(conn, ctx, &input.task_id, input.expected_revision).await?;
    require(caps.assign)?;
    row.owner_id = input.owner_id;
    row.assignee_id = input.assignee_id;
    row.reviewer_id = input.reviewer_id;
    references(&tx, &row).await?;
    revoke(&tx, &row).await?;
    invalidate_review(&mut row);
    let payload = json!({"ownerId": row.owner_id, "assigneeId": row.assignee_id, "reviewerId": row.reviewer_id});
    finish(tx, ctx, row, &actor, "assigned", payload).await
}

pub async fn progress(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: ProgressInput,
) -> Result<Detail, E> {
    let (tx, mut row, actor, caps) =
        begin_task(conn, ctx, &input.task_id, input.expected_revision).await?;
    require(caps.progress)?;
    references(&tx, &row).await?;
    if policy::terminal(row.status) {
        revoke(&tx, &row).await?;
        row.current_deliverable_id = None;
    }
    let previous = row.status;
    row.status = input.status.into();
    finish(
        tx,
        ctx,
        row,
        &actor,
        "progressed",
        json!({"from": previous, "to": input.status}),
    )
    .await
}

pub async fn note(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: TextInput,
) -> Result<Detail, E> {
    body(&input.body, true)?;
    let (tx, row, actor, caps) =
        begin_task(conn, ctx, &input.task_id, input.expected_revision).await?;
    require(caps.comment)?;
    references(&tx, &row).await?;
    finish(tx, ctx, row, &actor, "note", json!({"body": input.body})).await
}

pub async fn submit(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: TextInput,
) -> Result<Detail, E> {
    body(&input.body, true)?;
    let (tx, mut row, actor, caps) =
        begin_task(conn, ctx, &input.task_id, input.expected_revision).await?;
    require(caps.submit)?;
    references(&tx, &row).await?;
    let deliverable_id = id();
    tx.execute(statement("INSERT INTO business_task_deliverable (id,organization_id,task_id,revision,author_id,author_name,author_kind,body,execution_id,created_at) VALUES (?,?,?,?,?,?,?,?,?,?)",
        vec![deliverable_id.clone().into(), row.organization_id.clone().into(), row.id.clone().into(), row.revision.checked_add(1).ok_or(E::Conflict)?.into(), actor.id.clone().into(), actor.display_name.clone().into(), kind(&actor).into(), input.body.into(),
            ctx.live.as_ref().and(row.current_execution_id.clone()).into(), now().into()])).await?;
    row.current_deliverable_id = Some(deliverable_id.clone());
    row.status = TaskStatus::Review;
    finish(
        tx,
        ctx,
        row,
        &actor,
        "submitted",
        json!({"deliverableId": deliverable_id}),
    )
    .await
}

pub async fn review(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: ReviewInput,
) -> Result<Detail, E> {
    body(&input.comment, false)?;
    let (tx, mut row, actor, caps) =
        begin_task(conn, ctx, &input.task_id, input.expected_revision).await?;
    require(caps.review)?;
    references(&tx, &row).await?;
    row.status = match input.decision {
        ReviewDecision::Accept => TaskStatus::Done,
        ReviewDecision::Return => TaskStatus::InProgress,
    };
    let payload = json!({"decision": input.decision, "comment": input.comment, "deliverableId": row.current_deliverable_id, "reviewedRevision": row.revision});
    if row.status == TaskStatus::Done {
        revoke(&tx, &row).await?;
    }
    finish(tx, ctx, row, &actor, "reviewed", payload).await
}

pub async fn cancel(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: RevisionInput,
) -> Result<Detail, E> {
    let (tx, mut row, actor, caps) =
        begin_task(conn, ctx, &input.task_id, input.expected_revision).await?;
    require(caps.cancel)?;
    row.status = TaskStatus::Cancelled;
    revoke(&tx, &row).await?;
    finish(tx, ctx, row, &actor, "cancelled", json!({})).await
}

pub async fn archive(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: ArchiveInput,
) -> Result<Detail, E> {
    let (tx, mut row, actor, caps) =
        begin_task(conn, ctx, &input.task_id, input.expected_revision).await?;
    require(caps.archive)?;
    row.archived_at = input.archived.then(now);
    revoke(&tx, &row).await?;
    finish(
        tx,
        ctx,
        row,
        &actor,
        "archived",
        json!({"archived": input.archived}),
    )
    .await
}

/// Only the engine may supply live fields, under its existing request lock.
pub(crate) async fn check_link(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: &LinkExecutionInput,
) -> Result<(), E> {
    let tx = conn.begin().await?;
    let row = model(&tx, ctx.principal.organization_id(), &input.task_id).await?;
    let actor = visible(&tx, ctx, &row, Permission::Contribute).await?;
    require(caps(&tx, ctx, &row, &actor).await?.link_execution && ctx.live.is_none())?;
    if input.expected_revision <= 0 || input.work_task_id <= 0 {
        return Err(E::Invalid("Use positive revision and executor IDs"));
    }
    if row.revision != input.expected_revision {
        return Err(E::Conflict);
    }
    tx.commit().await?;
    Ok(())
}

/// Authorization above is only preflight before looking up an executor. The
/// actual write repeats all checks below after acquiring SQLite writer ownership.
pub(crate) async fn link(
    conn: &DatabaseConnection,
    ctx: &ActorContext,
    input: LinkExecutionInput,
    live: agent::LiveExecution,
) -> Result<Detail, E> {
    let (tx, mut row, actor, caps) =
        begin_task(conn, ctx, &input.task_id, input.expected_revision).await?;
    require(caps.link_execution && ctx.live.is_none())?;
    references(&tx, &row).await?;
    require(input.work_task_id == live.work_task_id)?;
    agent::require_live(&tx, &live).await?;
    let agent_id = row.assignee_id.clone().ok_or(E::Forbidden)?;
    identity::agent_principal(&tx, &ctx.principal, &agent_id).await?;
    let occupied = tx.query_one(statement("SELECT id FROM business_task_execution WHERE work_task_id = ? AND run_seq = ?",
        vec![live.work_task_id.into(), live.run_seq.into()])).await?;
    if occupied.is_some() {
        return Err(E::Conflict);
    }
    let grant = identity::delegation_grant(&ctx.principal)?.to_storage()?;
    revoke(&tx, &row).await?;
    let binding_id = id();
    tx.execute(statement("INSERT INTO business_task_execution (id,organization_id,task_id,work_task_id,run_seq,connection_id,agent_member_id,agent_key,delegation_json,linked_by,created_at) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        vec![binding_id.clone().into(), row.organization_id.clone().into(), row.id.clone().into(), live.work_task_id.into(), live.run_seq.into(), live.connection_id.into(), agent_id.clone().into(), live.agent_key.into(), grant.into(), actor.id.clone().into(), now().into()])).await?;
    row.current_execution_id = Some(binding_id);
    invalidate_review(&mut row);
    finish(
        tx,
        ctx,
        row,
        &actor,
        "execution_linked",
        json!({"workTaskId": live.work_task_id, "runSeq": live.run_seq, "agentMemberId": agent_id}),
    )
    .await
}
