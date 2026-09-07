//! Trusted bridge helpers, not transport authentication. The pi bridge owns
//! token/parent attribution and must construct this context from backend state.
use super::{store, types::*, Operator};
use crate::db::{
    entities::{folder, work_task},
    error::DbError,
    service::ticket_service::MessageView,
};
use crate::models::WorkTaskStatus;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};

// Deliberately no Deserialize: this is never a request-supplied principal.
pub struct RunContext {
    pub account_id: i32,
    pub task_id: i32,
    pub run_seq: i32,
    pub connection_id: String,
    pub agent_id: String,
}

/// Trusted host configuration, shared with the operator boundary; no principal.
pub fn account_id() -> Result<i32, crate::app_error::AppCommandError> {
    super::configured_account_id()
}

/// Bridge error projection: never serialize an underlying database error.
pub fn command_error(error: DbError) -> crate::app_error::AppCommandError {
    super::command_error(error)
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub account_id: i32,
    pub inboxes: Vec<Inbox>,
}
pub async fn context(db: &DatabaseConnection, ctx: &RunContext) -> Result<Context, DbError> {
    let txn = db.begin().await?;
    require_live(&txn, ctx).await?;
    let inboxes = crate::db::service::ticket_service::list_inboxes(&txn, ctx.account_id)
        .await?
        .into_iter()
        .map(store::inbox_dto)
        .collect();
    txn.commit().await?;
    Ok(Context {
        account_id: ctx.account_id,
        inboxes,
    })
}

pub(super) async fn require_live<C: ConnectionTrait>(
    db: &C,
    ctx: &RunContext,
) -> Result<(), DbError> {
    let task = work_task::Entity::find_by_id(ctx.task_id)
        .filter(work_task::Column::RunSeq.eq(ctx.run_seq))
        .filter(work_task::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| DbError::Conflict("task run changed".into()))?;
    if ctx.account_id <= 0
        || ctx.agent_id.trim().is_empty()
        || ctx.connection_id.is_empty()
        || task.connection_id.as_deref() != Some(ctx.connection_id.as_str())
        || !matches!(
            task.status,
            WorkTaskStatus::Running | WorkTaskStatus::AwaitingInput
        )
        || folder::Entity::find_by_id(task.folder_id)
            .filter(folder::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .is_none()
    {
        return Err(DbError::Conflict("task run changed".into()));
    }
    Ok(())
}
fn scope(ctx: &RunContext) -> Operator {
    // Internal routing reuse only. This value cannot escape as a human principal
    // and never passes through Operator::server or the operator transport.
    Operator {
        account_id: ctx.account_id,
        actor: "agent:pi",
    }
}

pub async fn thread(
    db: &DatabaseConnection,
    ctx: &RunContext,
    input: ThreadInput,
) -> Result<Thread, DbError> {
    store::thread_for_view(db, &scope(ctx), input, MessageView::Public, Some(ctx)).await
}

pub async fn tickets(
    db: &DatabaseConnection,
    ctx: &RunContext,
    input: TicketsInput,
) -> Result<TicketPage, DbError> {
    let txn = db.begin().await?;
    require_live(&txn, ctx).await?;
    let result =
        store::list_tickets_for_view(&txn, &scope(ctx), input, MessageView::Public).await?;
    txn.commit().await?;
    Ok(result)
}

pub async fn save_draft(
    db: &DatabaseConnection,
    ctx: &RunContext,
    input: SaveDraftInput,
) -> Result<Draft, DbError> {
    store::save_draft_for_run(
        db,
        &scope(ctx),
        input,
        Some(ctx),
        &format!("agent:{}", ctx.agent_id),
    )
    .await
}
