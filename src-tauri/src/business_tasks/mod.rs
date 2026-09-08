//! Shared business records, separate from engineering execution generations.
//! Codeg Apache task/CAS/transport patterns and exact sources are in NOTICE.
mod entity;
mod policy;
mod validation;
pub mod vocabulary;
pub mod types;
pub mod store;
pub(crate) mod http;
pub(crate) mod agent;

use crate::business_identity::Principal;

/// Authenticated transport or trusted live-run bridge only. No caller actor
/// fields, serialization, or alternate identity constructor.
pub struct ActorContext {
    principal: Principal,
    live: Option<agent::LiveExecution>,
}
impl ActorContext {
    pub fn authenticated(principal: Principal) -> Self {
        Self { principal, live: None }
    }
}

#[cfg(test)]
mod tests;
