//! Trusted Hafidh issue/evidence pack, additive to the accepted approvals core.
//! UI owns authenticated adapters/registry. No generic network or agent approval
//! endpoint is exposed here. See reports/intake-github.md for the host contract.
pub mod github;
mod store;
mod types;
pub use store::{
    attach_evidence, configure_repository, filing_status, prepare, record_source, revoke_evidence,
};
pub use types::*;

use crate::db::{
    error::DbError,
    service::ops_approvals::{Action, ActionContext, AuthorizedAction, Decision},
};
use github::{GithubAppClient, SendOutcome};
use sea_orm::DatabaseConnection;
use serde_json::Value;

pub struct GithubIssueAction;
fn prepared(payload: &Value) -> Result<PreparedIssue, IntakeError> {
    let value: PreparedIssue =
        serde_json::from_value(payload.clone()).map_err(|_| IntakeError::InvalidPayload)?;
    value.validate()?;
    Ok(value)
}
fn gate_error(error: IntakeError) -> DbError {
    DbError::Validation(error.to_string())
}

#[async_trait::async_trait]
impl Action for GithubIssueAction {
    fn name(&self) -> &'static str {
        "github.create_issue"
    }
    fn domain(&self) -> &'static str {
        "github"
    }
    fn validate(&self, payload: &Value) -> Result<(), DbError> {
        prepared(payload).map(|_| ()).map_err(gate_error)
    }
    async fn resource(
        &self,
        payload: &Value,
        _: &ActionContext<'_>,
    ) -> Result<Option<String>, DbError> {
        Ok(Some(prepared(payload).map_err(gate_error)?.repository))
    }
    async fn check_permission(
        &self,
        payload: &Value,
        ctx: &ActionContext<'_>,
    ) -> Result<Decision, DbError> {
        let p = prepared(payload).map_err(gate_error)?;
        if store::validate_bound(ctx.db, &p).await.is_err()
            || !store::agent_allowed(ctx.db, ctx.agent, &p.repository)
                .await
                .map_err(gate_error)?
        {
            return Ok(Decision::Deny);
        }
        Ok(Decision::Passthrough)
    }
    fn is_destructive(&self, _: &Value) -> Result<bool, DbError> {
        Ok(true)
    }
    fn private_fields(&self) -> &'static [&'static str] {
        &["body", "summary", "value"]
    }
}

/// Only the committed, non-Clone/non-Serialize authorization handoff is accepted.
/// Nothing is reloaded from an editable draft after review. A preflight failure
/// consumes the capability too: prepare and review a new proposal to try again.
pub async fn dispatch(
    conn: &DatabaseConnection,
    client: &GithubAppClient,
    authorized: AuthorizedAction,
) -> Result<FilingReceipt, IntakeError> {
    if authorized.action_name() != "github.create_issue" {
        return Err(IntakeError::InvalidPayload);
    }
    let proposal_id = authorized.proposal_id();
    let p = prepared(&authorized.into_payload())?;
    let _operation = client.operation.lock().await;
    let binding = store::validate_bound(conn, &p).await?;
    if let Some(receipt) = store::filing_status(conn, &p.draft.source_ref, p.repository_id).await? {
        if receipt.state != FilingState::Failed {
            return Err(IntakeError::Conflict);
        }
        if receipt
            .retry_after
            .is_some_and(|at| at > chrono::Utc::now().timestamp())
        {
            return Err(IntakeError::UpstreamUnavailable);
        }
    }
    let token = client.installation_token(&binding).await?;
    client
        .check_labels(&binding, &token, &p.outgoing.labels)
        .await?;
    // Recheck in the transaction AFTER network preflight, before the POST.
    let attempt_id = store::reserve(conn, proposal_id, &p).await?;
    match client.create_issue(&binding, &token, &p.outgoing).await {
        SendOutcome::Created(issue) => {
            store::finish(conn, attempt_id, &p, Some(issue), None, None).await
        }
        SendOutcome::Failed(code, retry_after) => {
            store::finish(conn, attempt_id, &p, None, Some(code), retry_after).await
        }
        SendOutcome::Unknown => store::finish(conn, attempt_id, &p, None, None, None).await,
    }
}

/// Trusted human status adapter only; reconciles an unknown receipt using GETs.
/// Never retries a POST, even when the bounded read finds no matching issue.
pub async fn reconcile_filing(
    conn: &DatabaseConnection,
    client: &GithubAppClient,
    attempt_id: i64,
) -> Result<FilingReceipt, IntakeError> {
    let _operation = client.operation.lock().await;
    let (receipt, p) = store::attempt(conn, attempt_id).await?;
    if receipt.state != FilingState::Unknown {
        return Ok(receipt);
    }
    let b = store::binding(conn, &p.draft.source_ref.product_id).await?;
    if b.repository_id != p.repository_id
        || b.full_name != p.repository
        || types::json_digest(&b)? != p.binding_digest
    {
        return Err(IntakeError::AccessDenied);
    }
    match client.reconcile(&b, &p).await? {
        Some(issue) => store::finish(conn, attempt_id, &p, Some(issue), None, None).await,
        None => Ok(receipt),
    }
}

#[cfg(test)]
mod tests;
