//! Desk identity resolution through Codeg's existing private task/run index.
//! No operator principal, caller identity, or second Ops validator is created.
use super::{TaskEngine, MAX_DELEGATION_CHAIN_HOPS};
use crate::acp::desk::{DeskCall, DeskError, DeskResponse, DeskTool};
use crate::acp::types::ConnectionStatus;
use crate::app_error::AppErrorCode;
use crate::db::entities::work_task::{Model, WorkTaskStatus};
use crate::db::error::DbError;
use crate::db::service::{ops_approvals::ProposalOutcome, work_task_service};
use crate::ops::{agent, review};
use crate::ops_intake_host::{agent as intake, types::HostError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContextInput {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ProposeReplyInput {
    draft_id: i32,
    expected_revision: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ProposeIssueInput {
    draft_id: String,
    expected_revision: i32,
}

fn intake_error(error: HostError) -> DeskError {
    match error {
        HostError::NotConfigured => DeskError::ProductMissing,
        HostError::AmbiguousProduct => DeskError::AmbiguousProduct,
        HostError::StaleSource => DeskError::CacheExpired,
        HostError::InvalidEvidence => DeskError::EvidenceRequired,
        HostError::SeverityRequired => DeskError::SeverityRequired,
        HostError::InvalidInput => DeskError::InvalidInput,
        HostError::Conflict | HostError::TaskRequired => DeskError::Stale,
        HostError::AccessDenied => DeskError::Denied,
        HostError::StorageUnavailable => DeskError::Storage,
        HostError::AdapterMissing | HostError::SourceUnavailable => DeskError::Unavailable,
    }
}

fn input<T: DeserializeOwned>(value: Value) -> Result<T, DeskError> {
    serde_json::from_value(value).map_err(|_| DeskError::InvalidInput)
}

fn value<T: Serialize>(dto: T) -> Result<Value, DeskError> {
    serde_json::to_value(dto).map_err(|_| DeskError::Storage)
}

fn ops_error(error: DbError) -> DeskError {
    // Reuse the accepted domain projection, then reduce it to the closed IPC
    // categories. No SQL, provider error or input-derived message is returned.
    match agent::command_error(error).code {
        AppErrorCode::InvalidInput => DeskError::InvalidInput,
        AppErrorCode::NotFound | AppErrorCode::PermissionDenied => DeskError::Denied,
        AppErrorCode::TurnInProgress => DeskError::Stale,
        AppErrorCode::DatabaseError => DeskError::Storage,
        _ => DeskError::Unavailable,
    }
}

impl TaskEngine {
    /// Actual protected operator transport may explicitly entrust its local
    /// execution to a business task/agent. A member role or agent type cannot.
    /// This records source ownership only; it does not start or prompt a run.
    pub(crate) async fn entrust_business_execution(
        &self,
        ctx: crate::business_tasks::ActorContext,
        input: crate::business_tasks::types::LinkExecutionInput,
    ) -> Result<crate::business_tasks::types::Detail, crate::business_identity::IdentityError> {
        crate::business_tasks::require_operator(&ctx)?;
        let _binding = self.request_lock.lock().await;
        let live = self.business_execution_source(&ctx, &input).await?;
        crate::business_tasks::store::entrust(&self.db.conn, &ctx, input, live).await
    }

    /// Link only an existing generation under the same private ancestry lock
    /// as agent calls. Business authorization and lineage storage are in core.
    pub(crate) async fn link_business_execution(
        &self,
        ctx: crate::business_tasks::ActorContext,
        input: crate::business_tasks::types::LinkExecutionInput,
    ) -> Result<crate::business_tasks::types::Detail, crate::business_identity::IdentityError> {
        let _binding = self.request_lock.lock().await;
        let live = self.business_execution_source(&ctx, &input).await?;
        crate::business_tasks::store::link(&self.db.conn, &ctx, input, live).await
    }

    /// Caller holds request_lock until the eventual business writer commits.
    /// Liveness alone is not authority: store::link requires prior entrustment.
    async fn business_execution_source(
        &self,
        ctx: &crate::business_tasks::ActorContext,
        input: &crate::business_tasks::types::LinkExecutionInput,
    ) -> Result<crate::business_tasks::agent::LiveExecution, crate::business_identity::IdentityError> {
        use crate::business_identity::IdentityError as E;
        crate::business_tasks::store::check_link(&self.db.conn, ctx, input).await?;
        let row = work_task_service::get_model(&self.db.conn, input.work_task_id).await.map_err(|_| E::NotFound)?;
        let root = row.connection_id.as_deref().ok_or(E::Conflict)?;
        let (row, agent_key) = self.desk_scope(root).await.map_err(|_| E::Conflict)?;
        Ok(crate::business_tasks::agent::LiveExecution {
            work_task_id: row.id, run_seq: row.run_seq,
            connection_id: row.connection_id.ok_or(E::Conflict)?, agent_key,
        })
    }

    /// Caller holds request_lock through the eventual Ops transaction, so
    /// retirement cannot detach/rebind this ancestry between lookup and commit.
    /// Ops must also recheck the live row under its writer lock for cancellation.
    pub(super) async fn desk_scope(&self, connection: &str) -> Result<(Model, String), DeskError> {
        let ((task_id, run_seq), _) = self
            .task_for_connection(connection)
            .await
            .ok_or(DeskError::Stale)?;
        let task = work_task_service::get_model(&self.db.conn, task_id)
            .await
            .map_err(|_| DeskError::Stale)?;
        if task.run_seq != run_seq
            || !matches!(
                task.status,
                WorkTaskStatus::Running | WorkTaskStatus::AwaitingInput
            )
        {
            return Err(DeskError::Stale);
        }
        let root = task.connection_id.as_deref().ok_or(DeskError::Stale)?;
        let mut cursor = connection.to_owned();
        let mut agent_id = String::new();
        // Verify the actual stored root and every live ancestor, including a
        // disconnected intermediate delegate whose delayed mapping still exists.
        for _ in 0..=MAX_DELEGATION_CHAIN_HOPS {
            let state = self
                .manager
                .get_state(&cursor)
                .await
                .ok_or(DeskError::Denied)?;
            let state = state.read().await;
            if state.connection_id != cursor
                || !matches!(
                    state.status,
                    ConnectionStatus::Connected | ConnectionStatus::Prompting
                )
            {
                return Err(DeskError::Denied);
            }
            if cursor == connection {
                // Intromail policy rows address a persistent agent principal.
                // Codeg's existing wire/registry key is stable across launches;
                // Display names and per-launch UUIDs are not policy identities.
                agent_id = state.agent_type.as_wire().into_owned();
            }
            drop(state);
            if cursor == root {
                return Ok((task, agent_id));
            }
            // An indexed connection other than the stored root is an old or
            // inconsistent binding, even if its numeric task/run happen to match.
            if self.index.lock().await.contains_key(&cursor) {
                return Err(DeskError::Stale);
            }
            cursor = self
                .delegation_parents
                .lock()
                .await
                .get(&cursor)
                .cloned()
                .ok_or(DeskError::Stale)?;
        }
        Err(DeskError::Denied)
    }

    pub(super) async fn desk_call(&self, connection: &str, request: DeskCall) -> DeskResponse {
        let _binding = self.request_lock.lock().await;
        match self.desk_operation(connection, request).await {
            Ok(value) => DeskResponse::success(value),
            Err(error) => DeskResponse::rejected(error),
        }
    }

    async fn desk_operation(
        &self,
        connection: &str,
        request: DeskCall,
    ) -> Result<Value, DeskError> {
        let (task, agent_id) = self.desk_scope(connection).await?;
        if request.tool.is_business() {
            // A delegated child (even another Pi) is not the business member
            // explicitly entrusted to the root. No implicit identity minting.
            if task.connection_id.as_deref() != Some(connection) {
                return Err(DeskError::Denied);
            }
            // Business work needs no mailbox account. Resolve the private
            // generation mapping before entering the common authorized core.
            let live = crate::business_tasks::agent::LiveExecution {
                work_task_id: task.id, run_seq: task.run_seq,
                connection_id: task.connection_id.ok_or(DeskError::Stale)?,
                agent_key: agent_id,
            };
            return crate::business_tasks::agent::call(&self.db.conn, live, request).await;
        }
        let ctx = agent::RunContext {
            account_id: agent::account_id().map_err(|_| DeskError::Unavailable)?,
            task_id: task.id,
            run_seq: task.run_seq,
            connection_id: task.connection_id.ok_or(DeskError::Stale)?,
            agent_id,
        };
        let db = &self.db.conn;
        match request.tool {
            DeskTool::DeskBusinessTask | DeskTool::DeskBusinessProgress | DeskTool::DeskBusinessNote | DeskTool::DeskBusinessSubmit => Err(DeskError::Denied),
            DeskTool::HafidhIntakeStatus => value(intake::cached_status(db, &ctx, input(request.input)?).await.map_err(intake_error)?),
            DeskTool::HafidhFeedbackList => value(intake::cached_list(db, &ctx, input(request.input)?).await.map_err(intake_error)?),
            DeskTool::HafidhFeedbackGet => value(intake::cached_get(db, &ctx, input(request.input)?).await.map_err(intake_error)?),
            DeskTool::DeskProposeIssue => {
                let draft: ProposeIssueInput = input(request.input)?;
                let proposal = intake::prepare_and_propose(db, &ctx, &draft.draft_id, draft.expected_revision).await.map_err(intake_error)?;
                if proposal.status != "pending" { return Err(DeskError::Denied) }
                // PreparedIssue contains proof objects and binding digests. Keep
                // those inside the host, including the rendered body which
                // carries evidence references. Return only public metadata
                // and a review locator; never an execution capability.
                Ok(json!({ "proposalId": proposal.proposal_id, "status": "pending",
                    "draftId": draft.draft_id, "submittedRevision": draft.expected_revision,
                    "title": proposal.prepared.outgoing.title,
                    "labels": proposal.prepared.outgoing.labels }))
            }
            DeskTool::DeskContext => {
                let _: ContextInput = input(request.input)?;
                let context = agent::context(db, &ctx).await.map_err(ops_error)?;
                Ok(json!({
                    "taskId": ctx.task_id, "runSeq": ctx.run_seq,
                    "accountId": context.account_id, "inboxes": context.inboxes,
                }))
            }
            DeskTool::DeskTickets => value(
                agent::tickets(db, &ctx, input(request.input)?)
                    .await
                    .map_err(ops_error)?,
            ),
            DeskTool::DeskThread => value(
                agent::thread(db, &ctx, input(request.input)?)
                    .await
                    .map_err(ops_error)?,
            ),
            DeskTool::DeskSaveReply => value(
                agent::save_draft(db, &ctx, input(request.input)?)
                    .await
                    .map_err(ops_error)?,
            ),
            DeskTool::DeskProposeReply => {
                let draft: ProposeReplyInput = input(request.input)?;
                match review::propose_reply(
                    db,
                    ctx.task_id,
                    ctx.run_seq,
                    &ctx.agent_id,
                    ctx.account_id,
                    draft.draft_id,
                    draft.expected_revision,
                )
                .await
                .map_err(ops_error)?
                {
                    ProposalOutcome::Denied(_) => Err(DeskError::Denied),
                    ProposalOutcome::Pending(row) => Ok(json!({
                        "proposalId": row.id, "status": "pending",
                        "draftId": draft.draft_id, "revision": draft.expected_revision,
                    })),
                    // The accepted email pack is always destructive. Even if a
                    // future pack violates that contract, no capability crosses
                    // this boundary and no dispatcher is invoked here.
                    ProposalOutcome::Authorized(_) => Err(DeskError::Denied),
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "desk/tests.rs"]
mod tests;
