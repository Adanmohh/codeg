//! Desk identity resolution through Codeg's existing private task/run index.
//! No operator principal, caller identity, or second Ops validator is created.
use super::{TaskEngine, MAX_DELEGATION_CHAIN_HOPS};
use crate::acp::desk::{DeskCall, DeskError, DeskResponse};
use crate::acp::types::ConnectionStatus;
use crate::db::entities::work_task::{Model, WorkTaskStatus};
use crate::db::service::work_task_service;

impl TaskEngine {
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

    pub(super) async fn desk_call(&self, connection: &str, _request: DeskCall) -> DeskResponse {
        let _binding = self.request_lock.lock().await;
        match self.desk_scope(connection).await {
            // Accepted ops::agent helpers are the only domain implementation.
            // Until integrated, even a valid binding cannot read or mutate Ops.
            Ok(_) => DeskResponse::rejected(DeskError::Unavailable),
            Err(error) => DeskResponse::rejected(error),
        }
    }
}
