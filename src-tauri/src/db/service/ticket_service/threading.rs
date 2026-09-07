//! Ordered MIT port of Chatwoot mailbox/conversation_finder{,_strategies/*}.
//! Source: b354a9550e1fb59fa537a9c384232cb076213e72; see NOTICE.
use std::sync::OnceLock;

use regex::Regex;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Statement};

use super::{conversation, message, DbError, Scope};

/// Already decoded envelope addresses and ordered message-id tokens, as supplied
/// by Mail/MailPresenter upstream. The transport must parse RFC 5322 headers,
/// including folding and comments, before constructing this input.
#[derive(Debug, Default, Clone)]
pub struct ThreadHeaders {
    pub receivers: Vec<String>,
    pub in_reply_to: Vec<String>,
    pub references: Vec<String>,
    /// Decoded original header value, retained for the new-conversation fallback.
    pub raw_in_reply_to: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strategy {
    ReceiverUuid,
    InReplyTo,
    References,
    NewConversation,
}

pub(crate) fn message_id(value: &str) -> &str {
    let value = value.trim();
    value
        .strip_prefix('<')
        .and_then(|v| v.strip_suffix('>'))
        .unwrap_or(value)
}

fn receiver_uuid(receivers: &[String]) -> Option<String> {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    let pattern = PATTERN.get_or_init(|| {
        Regex::new(r"(?i)^reply\+([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})$")
            .expect("ported receiver pattern")
    });
    // Upstream extracts the first syntactically matching receiver, then looks
    // it up once. Do not silently change this to the first existing receiver.
    receivers.iter().find_map(|address| {
        let username = address.split('@').next()?;
        pattern.captures(username).map(|c| c[1].to_lowercase())
    })
}

fn pattern_uuid(value: &str) -> Option<String> {
    static MESSAGE: OnceLock<Regex> = OnceLock::new();
    static FALLBACK: OnceLock<Regex> = OnceLock::new();
    let message = MESSAGE.get_or_init(|| {
        Regex::new(r"conversation/([a-zA-Z0-9-]+)/messages/([0-9]+)@")
            .expect("ported message pattern")
    });
    let fallback = FALLBACK.get_or_init(|| {
        Regex::new(r"account/([0-9]+)/conversation/([a-zA-Z0-9-]+)@")
            .expect("ported fallback pattern")
    });
    message
        .captures(value)
        .map(|c| c[1].to_lowercase())
        .or_else(|| fallback.captures(value).map(|c| c[2].to_lowercase()))
}

async fn by_uuid<C: ConnectionTrait>(
    conn: &C,
    scope: Scope,
    uuid: &str,
) -> Result<Option<conversation::Model>, DbError> {
    Ok(conversation::Entity::find()
        .filter(conversation::Column::AccountId.eq(scope.account_id))
        .filter(conversation::Column::InboxId.eq(scope.inbox_id))
        .filter(conversation::Column::Uuid.eq(uuid))
        .one(conn)
        .await?)
}

async fn by_header<C: ConnectionTrait>(
    conn: &C,
    scope: Scope,
    values: &[String],
) -> Result<Option<conversation::Model>, DbError> {
    for value in values {
        let value = message_id(value);
        if value.is_empty() {
            continue;
        }
        if let Some(uuid) = pattern_uuid(value) {
            if let Some(conversation) = by_uuid(conn, scope, &uuid).await? {
                return Ok(Some(conversation));
            }
        }
        if let Some(message) = message::Entity::find()
            .filter(message::Column::AccountId.eq(scope.account_id))
            .filter(message::Column::InboxId.eq(scope.inbox_id))
            .filter(message::Column::Private.eq(false))
            .filter(message::Column::SourceId.eq(value))
            .one(conn)
            .await?
        {
            return super::get_conversation(conn, scope, message.conversation_id)
                .await
                .map(Some);
        }
    }
    Ok(None)
}

/// Returns an existing conversation, or a new-conversation decision with no
/// writes. The store persists new contact/conversation/message in one transaction.
/// All lookups extend upstream ReferencesStrategy's inbox isolation to account
/// and inbox scope, including the upstream account-only new-thread fallback.
pub async fn find<C: ConnectionTrait>(
    conn: &C,
    scope: Scope,
    headers: &ThreadHeaders,
) -> Result<(Option<conversation::Model>, Strategy), DbError> {
    super::require_inbox(conn, scope).await?;
    if let Some(uuid) = receiver_uuid(&headers.receivers) {
        if let Some(conversation) = by_uuid(conn, scope, &uuid).await? {
            return Ok((Some(conversation), Strategy::ReceiverUuid));
        }
    }
    for (strategy, values) in [
        (Strategy::InReplyTo, &headers.in_reply_to),
        (Strategy::References, &headers.references),
    ] {
        if let Some(conversation) = by_header(conn, scope, values).await? {
            return Ok((Some(conversation), strategy));
        }
    }
    if let Some(raw) = headers
        .raw_in_reply_to
        .as_deref()
        .filter(|s| !s.trim().is_empty())
    {
        let row = conn
            .query_one(Statement::from_sql_and_values(
                conn.get_database_backend(),
                "SELECT id FROM ops_ticket_conversation WHERE account_id = ? AND inbox_id = ? \
             AND json_extract(additional_attributes, '$.in_reply_to') = ? ORDER BY id LIMIT 1",
                [scope.account_id.into(), scope.inbox_id.into(), raw.into()],
            ))
            .await?;
        if let Some(row) = row {
            return Ok((
                Some(super::get_conversation(conn, scope, row.try_get("", "id")?).await?),
                Strategy::NewConversation,
            ));
        }
    }
    Ok((None, Strategy::NewConversation))
}

// Keep reference order, not recency order: the upstream strategies use `each`.
// No subject or contact fallback is permitted.
