//! IntroMail approval core port. See NOTICE for exact source-to-port mapping.
//!
//! This is a trusted Rust service seam, not an agent-callable approval API.
//! Adapters must resolve Action from a trusted registry and derive actor from
//! authentication. An agent must never supply its own Action or human identity.
//! No external execution occurs here. Dispatch ONLY AuthorizedAction::into_payload
//! after commit; do not substitute a subsequently edited draft or replay a
//! resolved row. A crash after commit requires a new proposal/human review.
mod gating;
mod redaction;
pub use gating::{Decision, GateResult, Verdict};

use chrono::Utc;
use sea_orm::{sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait,
    DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde_json::{json, Value};
use crate::db::{entities::{ops_audit_log as audit, ops_proposal as proposal,
    work_task::{self, WorkTaskStatus}}, error::DbError};
use super::work_task_service::record_event;

/// Trusted domain pack, ported from IntroMail Action. Validation must check the
/// complete payload without rewriting it, including all recipients/attachments.
/// Defaults are deliberately conservative for newly registered actions.
pub trait Action: Send + Sync {
    fn name(&self) -> &'static str;
    fn domain(&self) -> &'static str;
    fn validate(&self, payload: &Value) -> Result<(), DbError>;
    fn resource(&self, _payload: &Value) -> Result<Option<String>, DbError> { Ok(None) }
    fn check_permission(&self, _payload: &Value, _agent: &str) -> Result<Decision, DbError> {
        Ok(Decision::Passthrough)
    }
    fn is_destructive(&self, _payload: &Value) -> Result<bool, DbError> { Ok(true) }
    fn private_fields(&self) -> &'static [&'static str] { &[] }
    fn preview(&self, payload: &Value) -> Result<Value, DbError> { Ok(payload.clone()) }
}

/// Owned, non-Clone, non-Serialize execution handoff. It is never a mutable
/// draft reference. Private fields prevent construction by untrusted callers.
pub struct AuthorizedAction {
    proposal_id: i32,
    action_name: String,
    payload: Value,
}
impl AuthorizedAction {
    pub fn proposal_id(&self) -> i32 { self.proposal_id }
    pub fn action_name(&self) -> &str { &self.action_name }
    pub fn into_payload(self) -> Value { self.payload }
}

pub enum ProposalOutcome {
    Denied(GateResult),
    Pending(proposal::Model),
    Authorized(AuthorizedAction),
}

fn conflict() -> DbError { DbError::Conflict("stale or already resolved approval".into()) }
fn validate(action: &dyn Action, payload: &Value) -> Result<(), DbError> {
    // Never persist a credential or later execute a scrubbed substitute for it.
    if !payload.is_object() || redaction::scrub(payload, &[]) != *payload {
        return Err(DbError::Validation("payload must be an object without credentials".into()));
    }
    action.validate(payload).map_err(|_| DbError::Validation("invalid action payload".into()))
}

async fn audit_record<C: ConnectionTrait>(conn: &C, task_id: i32, run_seq: i32,
    actor: &str, action: &str, detail: Value) -> Result<(), DbError> {
    audit::ActiveModel {
        task_id: Set(task_id), run_seq: Set(run_seq), actor: Set(actor.to_owned()),
        action: Set(action.to_owned()), detail: Set(redaction::scrub(&detail, &[]).to_string()),
        created_at: Set(Utc::now()), ..Default::default()
    }.insert(conn).await?;
    Ok(())
}

/// CAS is the first operation in each transaction: acquires SQLite's write
/// lock before reading policy or proposal. A racing generation cannot change
/// beneath a successful approval. A no-op status CAS is used for auto/deny.
async fn task_cas(txn: &DatabaseTransaction, task_id: i32, run_seq: i32,
    from: WorkTaskStatus, to: WorkTaskStatus) -> Result<(), DbError> {
    let changed = work_task::Entity::update_many()
        .col_expr(work_task::Column::Status, Expr::value(super::work_task_service::status_str(to)))
        .col_expr(work_task::Column::UpdatedAt, Expr::value(Utc::now()))
        .filter(work_task::Column::Id.eq(task_id))
        .filter(work_task::Column::RunSeq.eq(run_seq))
        .filter(work_task::Column::Status.eq(from))
        .filter(work_task::Column::DeletedAt.is_null())
        .exec(txn).await?;
    if changed.rows_affected != 1 { return Err(conflict()); }
    // Same live-folder condition as the upstream work-task query boundary.
    let task = work_task::Entity::find_by_id(task_id).one(txn).await?.ok_or_else(conflict)?;
    if crate::db::entities::folder::Entity::find_by_id(task.folder_id).one(txn).await?.is_none() {
        return Err(conflict());
    }
    if from != to {
        record_event(txn, task_id, "status_changed", "engine", Some(json!({
            "from": super::work_task_service::status_str(from),
            "to": super::work_task_service::status_str(to), "run_seq": run_seq
        }))).await?;
    }
    Ok(())
}

/// Every proposed operation runs the gate and logs its verdict in the same
/// transaction. Only one pending proposal can own a task's waiting state.
pub async fn propose(conn: &DatabaseConnection, task_id: i32, run_seq: i32,
    agent: &str, action: &dyn Action, payload: Value) -> Result<ProposalOutcome, DbError> {
    validate(action, &payload)?;
    if agent.trim().is_empty() || action.name().is_empty() || action.domain().is_empty() {
        return Err(DbError::Validation("missing action identity".into()));
    }
    let txn = conn.begin().await?;
    task_cas(&txn, task_id, run_seq, WorkTaskStatus::Running, WorkTaskStatus::Running).await?;
    let result = gating::gate(&txn, action, &payload, agent).await?;
    audit_record(&txn, task_id, run_seq, agent, "agent.gate", json!({
        "action": action.name(), "gate": result
    })).await?;
    if result.verdict == Verdict::Deny {
        txn.commit().await?;
        return Ok(ProposalOutcome::Denied(result));
    }
    let auto = result.verdict == Verdict::Auto;
    let preview = action.preview(&payload).map_err(|_| DbError::Validation("invalid preview".into()))?;
    let fields = if auto { action.private_fields() } else { &[] };
    let row = proposal::ActiveModel {
        task_id: Set(task_id), run_seq: Set(run_seq), agent_id: Set(agent.into()),
        action_name: Set(action.name().into()),
        payload_json: Set(redaction::scrub(&payload, fields).to_string()),
        preview_json: Set(redaction::scrub(&preview, fields).to_string()),
        status: Set(if auto { "auto" } else { "pending" }.into()),
        created_at: Set(Utc::now()), resolved_at: Set(auto.then(Utc::now)),
        ..Default::default()
    }.insert(&txn).await?;
    if !auto {
        task_cas(&txn, task_id, run_seq, WorkTaskStatus::Running, WorkTaskStatus::AwaitingInput).await?;
    }
    audit_record(&txn, task_id, run_seq, agent,
        if auto { "agent.proposal_auto" } else { "agent.proposal_pending" },
        json!({"proposal_id": row.id})).await?;
    txn.commit().await?;
    Ok(if auto { ProposalOutcome::Authorized(AuthorizedAction {
        proposal_id: row.id, action_name: row.action_name, payload,
    }) } else { ProposalOutcome::Pending(row) })
}

/// The human submits the COMPLETE exact payload they reviewed (edited or
/// original), plus the original payload snapshot. That snapshot is an optimistic
/// binding against stale cards. Validation never normalizes the reviewed value.
/// Policy is rechecked for the edited resource; a deny cannot be human-overridden.
/// Ask/needs_review/destructive are satisfied by this explicit human decision.
pub async fn approve(conn: &DatabaseConnection, task_id: i32, run_seq: i32,
    proposal_id: i32, actor: &str, action: &dyn Action,
    expected_payload: &Value, approved_payload: Value) -> Result<AuthorizedAction, DbError> {
    validate(action, &approved_payload)?;
    let txn = conn.begin().await?;
    task_cas(&txn, task_id, run_seq, WorkTaskStatus::AwaitingInput, WorkTaskStatus::Running).await?;
    let row = pending(&txn, task_id, run_seq, proposal_id, actor, action).await?;
    let original: Value = serde_json::from_str(&row.payload_json).map_err(|_| conflict())?;
    if original != *expected_payload { return Err(conflict()); }
    let gate = gating::gate(&txn, action, &approved_payload, &row.agent_id).await?;
    // Ask short-circuits the source gate. Independently recheck the action's
    // deny at resolution so an edited forbidden payload cannot hide behind ask.
    if gate.verdict == Verdict::Deny || action.check_permission(&approved_payload, &row.agent_id)? == Decision::Deny {
        // Roll back the speculative resume before recording the rejected attempt.
        txn.rollback().await?;
        audit_record(conn, task_id, run_seq, actor, "agent.approval_blocked", json!({"proposal_id": proposal_id})).await?;
        return Err(DbError::Validation("approval blocked by policy".into()));
    }
    resolve(&txn, &row, actor, action, "approved", Some(&approved_payload)).await?;
    txn.commit().await?;
    Ok(AuthorizedAction { proposal_id, action_name: row.action_name, payload: approved_payload })
}

pub async fn deny(conn: &DatabaseConnection, task_id: i32, run_seq: i32,
    proposal_id: i32, actor: &str, action: &dyn Action) -> Result<(), DbError> {
    let txn = conn.begin().await?;
    task_cas(&txn, task_id, run_seq, WorkTaskStatus::AwaitingInput, WorkTaskStatus::Running).await?;
    let row = pending(&txn, task_id, run_seq, proposal_id, actor, action).await?;
    resolve(&txn, &row, actor, action, "denied", None).await?;
    txn.commit().await?;
    Ok(())
}

async fn pending(txn: &DatabaseTransaction, task_id: i32, run_seq: i32,
    id: i32, actor: &str, action: &dyn Action) -> Result<proposal::Model, DbError> {
    if actor.trim().is_empty() { return Err(DbError::Validation("human actor required".into())); }
    let row = proposal::Entity::find_by_id(id)
        .filter(proposal::Column::TaskId.eq(task_id))
        .filter(proposal::Column::RunSeq.eq(run_seq))
        .filter(proposal::Column::Status.eq("pending"))
        .filter(proposal::Column::ActionName.eq(action.name()))
        .one(txn).await?.ok_or_else(conflict)?;
    if actor == row.agent_id { return Err(DbError::Validation("agent cannot approve itself".into())); }
    Ok(row)
}

async fn resolve(txn: &DatabaseTransaction, row: &proposal::Model,
    actor: &str, action: &dyn Action, status: &str, edited: Option<&Value>) -> Result<(), DbError> {
    let scrub_stored = |raw: &str| -> Result<String, DbError> {
        let value: Value = serde_json::from_str(raw).map_err(|_| conflict())?;
        Ok(redaction::scrub(&value, action.private_fields()).to_string())
    };
    let changed = proposal::Entity::update_many().set(proposal::ActiveModel {
        status: Set(status.into()), verdict_by: Set(Some(actor.into())),
        payload_json: Set(scrub_stored(&row.payload_json)?),
        preview_json: Set(scrub_stored(&row.preview_json)?),
        edited_payload_json: Set(edited.map(|v| redaction::scrub(v, action.private_fields()).to_string())),
        resolved_at: Set(Some(Utc::now())), ..Default::default()
    }).filter(proposal::Column::Id.eq(row.id))
        .filter(proposal::Column::RunSeq.eq(row.run_seq))
        .filter(proposal::Column::Status.eq("pending"))
        .filter(proposal::Column::PayloadJson.eq(&row.payload_json))
        .exec(txn).await?;
    if changed.rows_affected != 1 { return Err(conflict()); }
    audit_record(txn, row.task_id, row.run_seq, actor,
        if status == "approved" { "agent.proposal_approved" } else { "agent.proposal_denied" },
        json!({"proposal_id": row.id, "edited": edited.is_some()})).await
}

#[cfg(test)]
mod tests;
