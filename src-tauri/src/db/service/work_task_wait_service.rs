//! Glue between codeg's ACP request set and IntroMail's pending proposals.
//! SQLite serializes request ownership and proposal resolution with the task CAS.
//! Keys include the run and connection so cleanup never releases another owner.
use crate::db::{
    entities::{
        ops_acp_wait as wait, ops_proposal,
        work_task::{self, WorkTaskStatus},
    },
    error::DbError,
};
use chrono::Utc;
use sea_orm::{
    sea_query::{Expr, OnConflict},
    ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};

/// Register/retract one ACP permission, question or plan request. The return
/// value reports an actual status transition, not whether ownership changed.
pub async fn track_request(
    conn: &DatabaseConnection,
    task_id: i32,
    run_seq: i32,
    connection_id: &str,
    request_key: &str,
    outstanding: bool,
) -> Result<bool, DbError> {
    let txn = conn.begin().await?;
    if outstanding {
        // A write first: even when Ops already owns AwaitingInput, persist this
        // distinct owner. Stale/canceled generations cannot acquire new waits.
        let claimed = work_task::Entity::update_many()
            .col_expr(
                work_task::Column::UpdatedAt,
                Expr::col(work_task::Column::UpdatedAt).into(),
            )
            .filter(work_task::Column::Id.eq(task_id))
            .filter(work_task::Column::RunSeq.eq(run_seq))
            .filter(
                work_task::Column::Status
                    .is_in([WorkTaskStatus::Running, WorkTaskStatus::AwaitingInput]),
            )
            .filter(work_task::Column::DeletedAt.is_null())
            .exec(&txn)
            .await?;
        if claimed.rows_affected != 1 {
            txn.rollback().await?;
            return Ok(false);
        }
        wait::Entity::insert(wait::ActiveModel {
            task_id: Set(task_id),
            run_seq: Set(run_seq),
            connection_id: Set(connection_id.into()),
            request_key: Set(request_key.into()),
            ..Default::default()
        })
        .on_conflict(
            OnConflict::columns([
                wait::Column::TaskId,
                wait::Column::RunSeq,
                wait::Column::ConnectionId,
                wait::Column::RequestKey,
            ])
            .do_nothing()
            .to_owned(),
        )
        .do_nothing()
        .exec_without_returning(&txn)
        .await?;
    } else {
        // DELETE is the first write, including stale-generation retractions.
        wait::Entity::delete_many()
            .filter(wait::Column::TaskId.eq(task_id))
            .filter(wait::Column::RunSeq.eq(run_seq))
            .filter(wait::Column::ConnectionId.eq(connection_id))
            .filter(wait::Column::RequestKey.eq(request_key))
            .exec(&txn)
            .await?;
    }
    let changed = reconcile(&txn, task_id, run_seq).await?;
    txn.commit().await?;
    Ok(changed)
}

/// Retract a retired connection/subtree. Callers serialize attribution and
/// detachment with request arrival; SQL additionally scopes every key to its run.
pub async fn clear_connections(
    conn: &DatabaseConnection,
    task_id: i32,
    run_seq: i32,
    connections: &[String],
) -> Result<bool, DbError> {
    let txn = conn.begin().await?;
    wait::Entity::delete_many()
        .filter(wait::Column::TaskId.eq(task_id))
        .filter(wait::Column::RunSeq.eq(run_seq))
        .filter(wait::Column::ConnectionId.is_in(connections.iter().cloned()))
        .exec(&txn)
        .await?;
    let changed = reconcile(&txn, task_id, run_seq).await?;
    txn.commit().await?;
    Ok(changed)
}

/// Caller must already hold SQLite's writer in this transaction. Releasing an
/// Ops proposal or ACP request resumes only when neither owner still needs input.
/// No stale-generation or terminal-state reconciliation can resume a task.
pub(super) async fn reconcile(
    txn: &DatabaseTransaction,
    task_id: i32,
    run_seq: i32,
) -> Result<bool, DbError> {
    let Some(task) = work_task::Entity::find_by_id(task_id)
        .filter(work_task::Column::RunSeq.eq(run_seq))
        .filter(
            work_task::Column::Status
                .is_in([WorkTaskStatus::Running, WorkTaskStatus::AwaitingInput]),
        )
        .filter(work_task::Column::DeletedAt.is_null())
        .one(txn)
        .await?
    else {
        return Ok(false);
    };
    let acp_pending = wait::Entity::find()
        .filter(wait::Column::TaskId.eq(task_id))
        .filter(wait::Column::RunSeq.eq(run_seq))
        .one(txn)
        .await?
        .is_some();
    let ops_pending = ops_proposal::Entity::find()
        .filter(ops_proposal::Column::TaskId.eq(task_id))
        .filter(ops_proposal::Column::RunSeq.eq(run_seq))
        .filter(ops_proposal::Column::Status.eq("pending"))
        .one(txn)
        .await?
        .is_some();
    let to = if acp_pending || ops_pending {
        WorkTaskStatus::AwaitingInput
    } else {
        WorkTaskStatus::Running
    };
    if to == task.status {
        return Ok(false);
    }
    let changed = work_task::Entity::update_many()
        .col_expr(
            work_task::Column::Status,
            Expr::value(super::work_task_service::status_str(to)),
        )
        .col_expr(work_task::Column::UpdatedAt, Expr::value(Utc::now()))
        .filter(work_task::Column::Id.eq(task_id))
        .filter(work_task::Column::RunSeq.eq(run_seq))
        .filter(work_task::Column::Status.eq(task.status))
        .filter(work_task::Column::DeletedAt.is_null())
        .exec(txn)
        .await?;
    if changed.rows_affected != 1 {
        return Ok(false);
    }
    super::work_task_service::status_changed_event(
        txn,
        task_id,
        "engine",
        Some(task.status),
        to,
        Some(serde_json::json!({"run_seq": run_seq})),
    )
    .await?;
    Ok(true)
}
