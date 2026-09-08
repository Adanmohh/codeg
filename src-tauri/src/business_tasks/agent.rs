//! Backend-only executor binding. No bearer/actor/run can be deserialized here.
use super::{
    entity::{execution, task},
    store::{model, statement},
    ActorContext,
};
use crate::acp::desk::{DeskCall, DeskError, DeskTool};
use crate::business_identity::{self as identity, IdentityError as E, MemberKind};
use crate::db::entities::{folder, work_task};
use crate::models::WorkTaskStatus;
use sea_orm::DatabaseConnection;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, QueryFilter};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

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
pub(super) async fn require_live<C: ConnectionTrait>(
    conn: &C,
    live: &LiveExecution,
) -> Result<(), E> {
    let row = work_task::Entity::find_by_id(live.work_task_id)
        .filter(work_task::Column::RunSeq.eq(live.run_seq))
        .filter(work_task::Column::DeletedAt.is_null())
        .one(conn)
        .await?
        .ok_or(E::Conflict)?;
    if live.run_seq <= 0
        || live.agent_key.is_empty()
        || live.connection_id.is_empty()
        || row.connection_id.as_deref() != Some(live.connection_id.as_str())
        || !matches!(
            row.status,
            WorkTaskStatus::Running | WorkTaskStatus::AwaitingInput
        )
        || folder::Entity::find_by_id(row.folder_id)
            .filter(folder::Column::DeletedAt.is_null())
            .one(conn)
            .await?
            .is_none()
    {
        return Err(E::Conflict);
    }
    Ok(())
}

pub(crate) async fn resolve<C: ConnectionTrait>(
    conn: &C,
    live: LiveExecution,
) -> Result<(ActorContext, String), E> {
    require_live(conn, &live).await?;
    let binding = execution::Model::find_by_statement(statement(
        "SELECT * FROM business_task_execution WHERE work_task_id = ? AND run_seq = ? AND connection_id = ? AND agent_key = ? AND revoked_at IS NULL",
        vec![live.work_task_id.into(), live.run_seq.into(), live.connection_id.clone().into(), live.agent_key.clone().into()],
    )).one(conn).await?.ok_or(E::NotFound)?;
    let row = model(conn, &binding.organization_id, &binding.task_id).await?;
    require_binding(&row, &binding, &live)?;
    let principal = identity::agent_principal_from_binding(
        conn,
        &binding.organization_id,
        &binding.agent_member_id,
        &binding.delegation_json,
    )
    .await?;
    Ok((
        ActorContext {
            principal,
            live: Some(live),
        },
        row.id,
    ))
}

fn require_binding(
    row: &task::Model,
    binding: &execution::Model,
    live: &LiveExecution,
) -> Result<(), E> {
    if binding.revoked_at.is_some()
        || row.archived_at.is_some()
        || super::policy::terminal(row.status)
        || row.current_execution_id.as_deref() != Some(binding.id.as_str())
        || row.assignee_id.as_deref() != Some(binding.agent_member_id.as_str())
        || binding.work_task_id != live.work_task_id
        || binding.run_seq != live.run_seq
        || binding.connection_id != live.connection_id
        || binding.agent_key != live.agent_key
    {
        return Err(E::Conflict);
    }
    Ok(())
}

pub(super) async fn active<C: ConnectionTrait>(
    conn: &C,
    row: &task::Model,
    binding: &execution::Model,
) -> Result<bool, E> {
    let checked = async {
        let live = LiveExecution {
            work_task_id: binding.work_task_id,
            run_seq: binding.run_seq,
            connection_id: binding.connection_id.clone(),
            agent_key: binding.agent_key.clone(),
        };
        require_binding(row, binding, &live)?;
        require_live(conn, &live).await?;
        super::store::references(conn, row).await?;
        let principal = identity::agent_principal_from_binding(
            conn,
            &row.organization_id,
            &binding.agent_member_id,
            &binding.delegation_json,
        )
        .await?;
        identity::authorize(
            conn,
            &principal,
            &row.organization_id,
            identity::Permission::Contribute,
            Some(super::store::domain(row)?),
        )
        .await?;
        Ok::<(), E>(())
    }
    .await;
    match checked {
        Ok(()) => Ok(true),
        Err(e @ (E::Database(_) | E::Random)) => Err(e),
        Err(_) => Ok(false),
    }
}

/// Called again inside every read snapshot / writer transaction, including
/// original credential revalidation in authorize. Resolving once is not a grant.
pub(super) async fn scope<C: ConnectionTrait>(
    conn: &C,
    ctx: &ActorContext,
    row: &task::Model,
    kind: MemberKind,
) -> Result<(), E> {
    if kind != MemberKind::Agent {
        return if ctx.live.is_none() {
            Ok(())
        } else {
            Err(E::Forbidden)
        };
    }
    let live = ctx.live.as_ref().ok_or(E::Forbidden)?;
    require_live(conn, live).await?;
    let binding = execution::Model::find_by_statement(statement(
        "SELECT * FROM business_task_execution WHERE organization_id = ? AND task_id = ? AND id = ? AND agent_member_id = ?",
        vec![row.organization_id.clone().into(), row.id.clone().into(), row.current_execution_id.clone().into(), ctx.principal.member_id().into()],
    )).one(conn).await?.ok_or(E::NotFound)?;
    require_binding(row, &binding, live)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Empty {}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Progress {
    expected_revision: i64,
    status: super::vocabulary::ProgressStatus,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Text {
    expected_revision: i64,
    body: String,
}
fn input<T: DeserializeOwned>(value: Value) -> Result<T, E> {
    serde_json::from_value(value).map_err(|_| E::Invalid("Invalid task contribution"))
}
fn error(error: E) -> DeskError {
    match error {
        E::Unauthorized | E::Forbidden | E::NotFound => DeskError::Denied,
        E::Conflict => DeskError::Stale,
        E::Invalid(_) => DeskError::InvalidInput,
        E::Database(_) | E::Random => DeskError::Storage,
        E::BootstrapRequired => DeskError::Unavailable,
    }
}
pub(crate) async fn call(
    conn: &DatabaseConnection,
    live: LiveExecution,
    request: DeskCall,
) -> Result<Value, DeskError> {
    async fn operation(
        conn: &DatabaseConnection,
        live: LiveExecution,
        request: DeskCall,
    ) -> Result<Value, E> {
        let (ctx, task_id) = resolve(conn, live).await?;
        let detail = match request.tool {
            DeskTool::DeskBusinessTask => {
                let _: Empty = input(request.input)?;
                super::store::get(conn, &ctx, super::types::TaskInput { task_id }).await?
            }
            DeskTool::DeskBusinessProgress => {
                let request: Progress = input(request.input)?;
                super::store::progress(
                    conn,
                    &ctx,
                    super::types::ProgressInput {
                        task_id,
                        expected_revision: request.expected_revision,
                        status: request.status,
                    },
                )
                .await?
            }
            DeskTool::DeskBusinessNote | DeskTool::DeskBusinessSubmit => {
                let text: Text = input(request.input)?;
                let text = super::types::TextInput {
                    task_id,
                    expected_revision: text.expected_revision,
                    body: text.body,
                };
                if request.tool == DeskTool::DeskBusinessSubmit {
                    super::store::submit(conn, &ctx, text).await?
                } else {
                    super::store::note(conn, &ctx, text).await?
                }
            }
            _ => return Err(E::Forbidden),
        };
        serde_json::to_value(detail).map_err(|_| E::Invalid("Invalid task response"))
    }
    operation(conn, live, request).await.map_err(error)
}
