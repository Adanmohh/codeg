//! Ordered first-match gate, ported from IntroMail gating.py (see NOTICE).
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use serde_json::Value;

use super::Action;
use crate::db::{entities::{ops_agent_rule as rule, ops_agent_scope as scope}, error::DbError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decision { Allow, Deny, NeedsReview, Passthrough }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict { Auto, NeedsReview, Deny }

#[derive(Debug, Serialize)]
pub struct GateResult {
    pub verdict: Verdict,
    pub reason: &'static str,
    pub mode: String,
    pub matched_rule_id: Option<i32>,
}

pub(super) async fn gate<C: ConnectionTrait>(
    conn: &C, action: &dyn Action, payload: &Value, agent: &str,
) -> Result<GateResult, DbError> {
    let resource = action.resource(payload)?;
    let scopes = scope::Entity::find()
        .filter(scope::Column::AgentId.eq(agent))
        .filter(scope::Column::Domain.eq(action.domain()))
        .all(conn).await?;
    let mode = scopes.iter().find(|s| resource.is_some() && s.resource == resource)
        .or_else(|| scopes.iter().find(|s| s.resource.is_none()))
        .map(|s| s.mode.as_str()).unwrap_or("propose").to_owned();
    let mut rules = rule::Entity::find()
        .filter(rule::Column::AgentId.eq(agent))
        .filter(rule::Column::Domain.eq(action.domain()))
        .all(conn).await?;
    rules.retain(|r| (r.resource.is_none() || r.resource == resource)
        && (r.action_name.is_none() || r.action_name.as_deref() == Some(action.name())));
    rules.sort_by_key(|r| (std::cmp::Reverse(r.resource.is_some()), std::cmp::Reverse(r.action_name.is_some()), r.id));
    let result = |verdict, reason, matched_rule_id| GateResult { verdict, reason, mode: mode.clone(), matched_rule_id };
    // 1. Deny; 2. ask. Neither allow rules nor modes can skip these.
    if let Some(r) = rules.iter().find(|r| r.behavior == "deny") {
        return Ok(result(Verdict::Deny, "deny_rule", Some(r.id)));
    }
    if let Some(r) = rules.iter().find(|r| r.behavior == "ask") {
        return Ok(result(Verdict::NeedsReview, "ask_rule", Some(r.id)));
    }
    // 3. Payload-aware check; errors propagate, never permit execution.
    let decision = action.check_permission(payload, agent)?;
    match decision {
        Decision::Deny => return Ok(result(Verdict::Deny, "action", None)),
        Decision::NeedsReview => return Ok(result(Verdict::NeedsReview, "action", None)),
        _ => {}
    }
    // 4. Destructive floor. NO mode or allow can bypass it.
    if action.is_destructive(payload)? {
        return Ok(result(Verdict::NeedsReview, "destructive", None));
    }
    // 5. Standing allow, then action allow.
    if let Some(r) = rules.iter().find(|r| r.behavior == "allow") {
        return Ok(result(Verdict::Auto, "allow_rule", Some(r.id)));
    }
    if decision == Decision::Allow {
        return Ok(result(Verdict::Auto, "action", None));
    }
    // 6. Unknown/missing mode fails to review.
    Ok(result(if mode == "act_low_risk" { Verdict::Auto } else { Verdict::NeedsReview }, "mode", None))
}
