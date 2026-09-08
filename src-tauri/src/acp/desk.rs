//! Closed Desk capability wire types. The launch token, never request JSON,
//! supplies the parent connection. Ops owns domain validation and transactions.
//! Codeg v0.30.4 companion pattern; see NOTICE.

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const SCHEMA: &str = include_str!("desk_schema.json");
pub const MAX_DESK_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeskTool {
    DeskContext,
    DeskTickets,
    DeskThread,
    DeskSaveReply,
    DeskProposeReply,
    DeskProposeIssue,
    HafidhFeedbackList,
    HafidhFeedbackGet,
    HafidhIntakeStatus,
}

impl DeskTool {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "desk_context" => Some(Self::DeskContext),
            "desk_tickets" => Some(Self::DeskTickets),
            "desk_thread" => Some(Self::DeskThread),
            "desk_save_reply" => Some(Self::DeskSaveReply),
            "desk_propose_reply" => Some(Self::DeskProposeReply),
            "desk_propose_issue" => Some(Self::DeskProposeIssue),
            "hafidh_feedback_list" => Some(Self::HafidhFeedbackList),
            "hafidh_feedback_get" => Some(Self::HafidhFeedbackGet),
            "hafidh_intake_status" => Some(Self::HafidhIntakeStatus),
            _ => None,
        }
    }

    pub fn is_cached_read(self) -> bool {
        matches!(self, Self::HafidhFeedbackList | Self::HafidhFeedbackGet | Self::HafidhIntakeStatus)
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeskCall {
    pub tool: DeskTool,
    /// Deserialized into Ops' closed DTOs by the trusted bridge. No identity is
    /// accepted here; keeping the boundary opaque avoids a second Ops validator.
    pub input: Value,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeskError {
    Unavailable,
    Denied,
    Stale,
    InvalidInput,
    Storage,
    ProductMissing,
    AmbiguousProduct,
    CacheExpired,
    EvidenceRequired,
    SeverityRequired,
}

#[derive(Serialize, Deserialize)]
pub struct DeskResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<DeskError>,
}

impl DeskResponse {
    pub fn success(value: Value) -> Self {
        Self { ok: true, value: Some(value), code: None }
    }

    pub fn rejected(code: DeskError) -> Self {
        Self { ok: false, value: None, code: Some(code) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desk_wire_has_no_identity_or_execution_operation() {
        for name in ["approve", "deny", "execute", "send_email", "fetch", "private_notes"] {
            assert!(DeskTool::from_name(name).is_none());
        }
        assert!(serde_json::from_value::<DeskCall>(serde_json::json!({
            "tool": "desk_context", "input": {}, "actor": "operator"
        })).is_err());
        let schema: Value = serde_json::from_str(SCHEMA).unwrap();
        assert_eq!(schema.as_array().unwrap().len(), 9);
        for tool in schema.as_array().unwrap() {
            assert!(DeskTool::from_name(tool["name"].as_str().unwrap()).is_some());
            assert_eq!(tool["inputSchema"]["additionalProperties"], false);
        }
    }
}
