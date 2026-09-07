//! Trusted internal Resend REST transport; no routes, scheduler or approval grant.
//!
//! The caller selects an authorized ticket scope and supplies its credential from
//! the existing credential store. Never expose this API directly to a web/agent
//! caller: send approval and user membership belong to the later dispatch layer.
//! No credentials, request bodies or provider error bodies are logged here.
//!
//! IntroMail patterns: 0bd24dfe284b888aa9f602fa1fd00e337ea38874.
//! Official Resend contracts and MIT parser adaptations: see NOTICE and
//! reports/email-transport.md for exact files, immutable refs and limitations.
mod client;
mod normalize;
mod pull;
#[cfg(test)]
mod tests;

pub use client::ResendClient;
pub use normalize::ReceivedHeaders;
pub use pull::{PullOptions, PullSummary};

use serde::{Deserialize, Serialize};

/// Errors intentionally contain no external error text, URLs, mail or secrets.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("invalid email transport input: {0}")]
    InvalidInput(&'static str),
    #[error("invalid Resend response: {0}")]
    Response(&'static str),
    #[error("Resend HTTP status {status}")]
    Http {
        status: u16,
        retry_after_seconds: Option<u64>,
    },
    #[error("Resend request failed (timeout: {timeout})")]
    Transport { timeout: bool },
    #[error("ticket inbox unavailable or changed")]
    Inbox,
    #[error("ticket persistence failed")]
    Persistence,
    #[error("received pagination did not advance")]
    Pagination,
    #[error("pull exceeded its page or memory budget; no new mail was ingested")]
    PullLimit,
}

/// Resend's provider UUID, distinct from an RFC Message-ID used for threading.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct EmailId(String);

impl EmailId {
    pub fn parse(value: &str) -> Result<Self, Error> {
        let id = uuid::Uuid::parse_str(value)
            .map_err(|_| Error::InvalidInput("provider UUID"))?
            .to_string();
        if !id.eq_ignore_ascii_case(value) {
            return Err(Error::InvalidInput("canonical provider UUID"));
        }
        Ok(Self(id))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EmailId {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Error> {
        Self::parse(&value)
    }
}

impl From<EmailId> for String {
    fn from(value: EmailId) -> Self {
        value.0
    }
}

/// Exclusive cursors mirror the official SDK's discriminated pagination options.
#[derive(Clone)]
pub enum Cursor {
    After(EmailId),
    Before(EmailId),
}

pub struct ListOptions {
    pub limit: u8,
    pub cursor: Option<Cursor>,
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            limit: 20,
            cursor: None,
        }
    }
}

/// Credential-wide provider metadata, NOT an authorized user/inbox view.
/// Resend has no inbox filter on this endpoint. Use `pull_received` for scoped
/// persistence and never surface this raw response through a route.
#[derive(Deserialize)]
pub struct ReceivedPage {
    pub object: String,
    pub has_more: bool,
    pub data: Vec<ReceivedSummary>,
}

#[derive(Deserialize)]
pub struct ReceivedSummary {
    pub id: EmailId,
    pub from: String,
    pub to: Vec<String>,
    pub created_at: String,
    pub subject: Option<String>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub reply_to: Option<Vec<String>>,
    /// Present in SDK types but absent from the pinned OpenAPI list schema.
    #[serde(default)]
    pub received_for: Vec<String>,
}

/// Direct GET response. Intentionally ignores raw/signed download URLs and
/// attachments: this adapter neither fetches them nor forwards bearer credentials.
#[derive(Deserialize)]
pub struct ReceivedEmail {
    pub object: String,
    #[serde(flatten)]
    pub envelope: ReceivedSummary,
    pub message_id: Option<String>,
    pub text: Option<String>,
    pub html: Option<String>,
    #[serde(default)]
    pub headers: ReceivedHeaders,
}

/// Plain addr-spec recipients only (display names belong to later composition).
/// Caller persists the same idempotency key, Message-ID and payload across retries.
/// No arbitrary From or custom header map can override the selected inbox.
pub struct SendEmail {
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub reply_to: Vec<String>,
    pub subject: String,
    pub text: Option<String>,
    pub html: Option<String>,
    pub message_id: String,
    pub in_reply_to: Option<String>,
    pub references: Vec<String>,
    pub idempotency_key: String,
}

#[derive(Deserialize)]
pub struct SentEmail {
    pub id: EmailId,
}
