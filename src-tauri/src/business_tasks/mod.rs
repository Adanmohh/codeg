//! Shared business records, separate from engineering execution generations.
//! Codeg Apache task/CAS/transport patterns and exact sources are in NOTICE.
pub(crate) mod agent;
mod entity;
pub(crate) mod http;
mod policy;
pub mod store;
pub mod types;
mod validation;
pub mod vocabulary;

use crate::business_identity::Principal;

/// Authenticated transport or trusted live-run bridge only. No caller actor
/// fields, serialization, or alternate identity constructor.
pub struct ActorContext {
    principal: Principal,
    live: Option<agent::LiveExecution>,
}
impl ActorContext {
    pub fn authenticated(principal: Principal) -> Self {
        Self {
            principal,
            live: None,
        }
    }
}

pub(crate) async fn link_execution(
    ctx: ActorContext,
    input: types::LinkExecutionInput,
) -> Result<types::Detail, crate::business_identity::IdentityError> {
    crate::work_task::engine()
        .ok_or(crate::business_identity::IdentityError::Invalid(
            "No live executor is available",
        ))?
        .link_business_execution(ctx, input)
        .await
}

#[cfg(test)]
mod tests;
