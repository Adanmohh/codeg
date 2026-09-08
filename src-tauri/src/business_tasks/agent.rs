//! Backend-only executor binding. No bearer/actor/run can be deserialized here.
use super::{entity::{execution, task}, store::{model, statement}, ActorContext};
use crate::business_identity::{self as identity, IdentityError as E, MemberKind};
use crate::db::entities::{folder, work_task};
use crate::models::WorkTaskStatus;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, QueryFilter};

/// Constructed only from the existing TaskEngine private index and live
/// connection ancestry while its request lock remains held through commit.
pub(crate) struct LiveExecution {
    pub work_task_id: i32,
    pub run_seq: i32,
    pub connection_id: String,
    pub agent_key: String,
}

/// Same current task/folder checks as accepted Ops RunContext, without its
/// unrelated mailbox account configuration. No new executor lifecycle.
pub(super) async fn require_live<C: ConnectionTrait>(conn: &C, live: &LiveExecution) -> Result<(), E> {
    let row = work_task::Entity::find_by_id(live.work_task_id)
        .filter(work_task::Column::RunSeq.eq(live.run_seq))
        .filter(work_task::Column::DeletedAt.is_null())
        .one(conn).await?.ok_or(E::Conflict)?;
    if live.run_seq <= 0 || live.agent_key.is_empty() || live.connection_id.is_empty()
        || row.connection_id.as_deref() != Some(live.connection_id.as_str())
        || !matches!(row.status, WorkTaskStatus::Running | WorkTaskStatus::AwaitingInput)
        || folder::Entity::find_by_id(row.folder_id)
            .filter(folder::Column::DeletedAt.is_null()).one(conn).await?.is_none()
    { return Err(E::Conflict); }
    Ok(())
}

pub(crate) async fn resolve<C: ConnectionTrait>(conn: &C, live: LiveExecution) -> Result<(ActorContext, String), E> {
    require_live(conn, &live).await?;
    let binding = execution::Model::find_by_statement(statement(
        "SELECT * FROM business_task_execution WHERE work_task_id = ? AND run_seq = ? AND connection_id = ? AND agent_key = ? AND revoked_at IS NULL",
        vec![live.work_task_id.into(), live.run_seq.into(), live.connection_id.clone().into(), live.agent_key.clone().into()],
    )).one(conn).await?.ok_or(E::NotFound)?;
    let row = model(conn, &binding.organization_id, &binding.task_id).await?;
    require_binding(&row, &binding, &live).await?;
    let principal = identity::agent_principal_from_binding(conn, &binding.organization_id,
        &binding.agent_member_id, &binding.delegation_json).await?;
    Ok((ActorContext { principal, live: Some(live) }, row.id))
}

async fn require_binding(row: &task::Model, binding: &execution::Model, live: &LiveExecution) -> Result<(), E> {
    if binding.revoked_at.is_some() || row.archived_at.is_some()
        || super::policy::terminal(row.status)
        || row.current_execution_id.as_deref() != Some(binding.id.as_str())
        || row.assignee_id.as_deref() != Some(binding.agent_member_id.as_str())
        || binding.work_task_id != live.work_task_id || binding.run_seq != live.run_seq
        || binding.connection_id != live.connection_id || binding.agent_key != live.agent_key
    { return Err(E::Conflict); }
    Ok(())
}

/// Called again inside every read snapshot / writer transaction, including
/// original credential revalidation in authorize. Resolving once is not a grant.
pub(super) async fn scope<C: ConnectionTrait>(conn: &C, ctx: &ActorContext, row: &task::Model, kind: MemberKind) -> Result<(), E> {
    if kind != MemberKind::Agent {
        return if ctx.live.is_none() { Ok(()) } else { Err(E::Forbidden) };
    }
    let live = ctx.live.as_ref().ok_or(E::Forbidden)?;
    require_live(conn, live).await?;
    let binding = execution::Model::find_by_statement(statement(
        "SELECT * FROM business_task_execution WHERE organization_id = ? AND task_id = ? AND id = ? AND agent_member_id = ?",
        vec![row.organization_id.clone().into(), row.id.clone().into(), row.current_execution_id.clone().into(), ctx.principal.member_id().into()],
    )).one(conn).await?.ok_or(E::NotFound)?;
    require_binding(row, &binding, live).await
}
