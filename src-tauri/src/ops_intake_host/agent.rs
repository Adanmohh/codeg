//! Only the parent-owned pi bridge may construct RunContext. No human powers here.
use super::{types::HostError, HostRuntime};
use crate::{ops::agent::RunContext, ops_intake::PreparedIssue};
use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Serialize)]
pub struct Proposed {
    pub status: &'static str,
    pub proposal_id: Option<i32>,
    pub prepared: PreparedIssue,
}

pub async fn prepare_and_propose(
    db: &DatabaseConnection,
    ctx: &RunContext,
    draft_id: &str,
    expected_revision: i32,
) -> Result<Proposed, HostError> {
    super::review::propose(
        db,
        ctx,
        draft_id,
        expected_revision,
        &HostRuntime::production(),
    )
    .await
}
