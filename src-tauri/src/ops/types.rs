use serde::{Deserialize, Serialize};
use serde_json::Value;

// Every request is closed, including nested payloads. Identities, scope and
// action names are never accepted from a caller-selected actor/account field.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ThreadInput {
    pub inbox_id: i32,
    pub conversation_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InboxInput {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TicketsInput {
    pub inbox_id: i32,
    pub status: Option<i32>,
    #[serde(default)]
    pub page: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoteInput {
    pub inbox_id: i32,
    pub conversation_id: i32,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Reply {
    pub inbox_id: i32,
    pub conversation_id: i32,
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub text: String,
    pub in_reply_to: Option<String>,
    pub references: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveDraftInput {
    pub inbox_id: i32,
    pub conversation_id: i32,
    pub expected_revision: i32,
    pub reply: Reply,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplyPayload {
    pub draft_id: i32,
    pub draft_revision: i32,
    pub reply: Reply,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProposalInput {
    pub id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReviewInput {
    pub id: i32,
    pub expected_payload: Value,
    pub approved_payload: ReplyPayload,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DenyInput {
    pub id: i32,
    pub expected_payload: Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Inbox {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub account_id: i32,
    pub operator: &'static str,
    pub inboxes: Vec<Inbox>,
    pub email_transport: &'static str,
    pub transport_message: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket {
    pub id: i32,
    pub inbox_id: i32,
    pub subject: String,
    pub contact: String,
    pub status: i32,
    pub updated_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketPage {
    pub items: Vec<Ticket>,
    pub has_more: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub private: bool,
    pub outgoing: bool,
    pub author: String,
    pub status: i32,
    pub source_id: Option<String>,
    pub created_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Draft {
    pub id: i32,
    pub revision: i32,
    pub reply: Reply,
    pub updated_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub ticket: Ticket,
    pub contact_email: String,
    pub messages: Vec<Message>,
    pub draft: Option<Draft>,
    pub suggested_reply: Reply,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Proposal {
    pub id: i32,
    pub task_id: i32,
    pub run_seq: i32,
    pub inbox_id: i32,
    pub conversation_id: i32,
    pub status: String,
    pub stale: bool,
    pub reason: Option<&'static str>,
    pub payload: Option<ReplyPayload>,
    pub created_at: String,
    pub delivery: Option<DeliveryStatus>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmailInboxInput {
    pub inbox_id: i32,
}

// Deliberately neither Debug nor Serialize; reads never return the secret.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EmailConfigureInput {
    pub inbox_id: i32,
    pub api_key: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailStatus {
    pub inbox_id: i32,
    pub configured: bool,
    pub last_pull_at: Option<String>,
    pub last_pull_status: Option<String>,
    pub last_pull_error: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PullResult {
    pub inserted: usize,
    pub duplicates: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryStatus {
    pub id: i32,
    pub proposal_id: i32,
    pub status: String,
    pub message_id: String,
    pub provider_id: Option<String>,
    pub error: Option<String>,
    pub updated_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Morning {
    pub tasks: Vec<crate::models::WorkTaskInfo>,
    pub tickets: Vec<Ticket>,
    pub proposals: Vec<Proposal>,
}
