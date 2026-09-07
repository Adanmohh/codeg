//! Ticket persistence shared by desktop and server through `AppDatabase.conn`.
//! MIT port of Chatwoot v4.17.1 core ticket behavior; see NOTICE.
//! This module stores records only. It never sends mail or grants send approval.
pub mod threading;
#[cfg(test)]
mod tests;

use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, QueryOrder, Set, Statement, TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::db::entities::ops_ticket::{contact, contact_inbox, conversation, inbox, message};
use crate::db::error::DbError;
use threading::{Strategy, ThreadHeaders};

/// Trusted caller-selected routing scope, not an authorization credential.
#[derive(Debug, Clone, Copy)]
pub struct Scope {
    pub account_id: i32,
    pub inbox_id: i32,
}

pub(crate) async fn require_inbox<C: ConnectionTrait>(conn: &C, scope: Scope) -> Result<inbox::Model, DbError> {
    inbox::Entity::find_by_id(scope.inbox_id)
        .filter(inbox::Column::AccountId.eq(scope.account_id))
        .one(conn).await?
        .ok_or_else(|| DbError::NotFound("ticket inbox in account".into()))
}

pub async fn create_inbox(conn: &DatabaseConnection, account_id: i32, name: &str, email: &str) -> Result<inbox::Model, DbError> {
    let email = normalized_email(email)?;
    if account_id <= 0 || name.trim().is_empty() {
        return Err(DbError::Validation("account and inbox name required".into()));
    }
    let now = Utc::now();
    Ok(inbox::ActiveModel {
        account_id: Set(account_id), name: Set(name.trim().into()), email_address: Set(email),
        channel_type: Set("Channel::Email".into()), enable_auto_assignment: Set(false),
        created_at: Set(now), updated_at: Set(now), ..Default::default()
    }.insert(conn).await?)
}

pub async fn list_inboxes<C: ConnectionTrait>(conn: &C, account_id: i32) -> Result<Vec<inbox::Model>, DbError> {
    Ok(inbox::Entity::find().filter(inbox::Column::AccountId.eq(account_id))
        .order_by_asc(inbox::Column::Id).all(conn).await?)
}

pub async fn get_conversation<C: ConnectionTrait>(conn: &C, scope: Scope, id: i32) -> Result<conversation::Model, DbError> {
    conversation::Entity::find_by_id(id)
        .filter(conversation::Column::AccountId.eq(scope.account_id))
        .filter(conversation::Column::InboxId.eq(scope.inbox_id)).one(conn).await?
        .ok_or_else(|| DbError::NotFound("ticket conversation in inbox".into()))
}

pub async fn list_conversations<C: ConnectionTrait>(conn: &C, scope: Scope) -> Result<Vec<conversation::Model>, DbError> {
    require_inbox(conn, scope).await?;
    Ok(conversation::Entity::find()
        .filter(conversation::Column::AccountId.eq(scope.account_id))
        .filter(conversation::Column::InboxId.eq(scope.inbox_id))
        .order_by_desc(conversation::Column::LastActivityAt)
        .order_by_desc(conversation::Column::Id).all(conn).await?)
}

pub async fn get_contact<C: ConnectionTrait>(conn: &C, scope: Scope, conversation_id: i32) -> Result<contact::Model, DbError> {
    let ticket = get_conversation(conn, scope, conversation_id).await?;
    contact::Entity::find_by_id(ticket.contact_id)
        .filter(contact::Column::AccountId.eq(scope.account_id)).one(conn).await?
        .ok_or_else(|| DbError::NotFound("ticket contact".into()))
}

#[derive(Debug, Clone, Copy)]
pub enum MessageView { Internal, Public }

/// Public mirrors Chatwoot Message.chat: excludes private notes AND activity.
pub async fn list_messages<C: ConnectionTrait>(conn: &C, scope: Scope, id: i32, view: MessageView) -> Result<Vec<message::Model>, DbError> {
    get_conversation(conn, scope, id).await?;
    let mut query = message::Entity::find()
        .filter(message::Column::AccountId.eq(scope.account_id))
        .filter(message::Column::InboxId.eq(scope.inbox_id))
        .filter(message::Column::ConversationId.eq(id));
    if matches!(view, MessageView::Public) {
        query = query.filter(message::Column::Private.eq(false)).filter(message::Column::MessageType.ne(2));
    }
    Ok(query.order_by_asc(message::Column::CreatedAt).order_by_asc(message::Column::Id).all(conn).await?)
}

/// First write before any SELECT, borrowed from codeg canvas_service's claim.
/// Locks SQLite's writer across processes so duplicate ingestion cannot race.
async fn claim_writer<C: ConnectionTrait>(conn: &C, scope: Scope) -> Result<(), DbError> {
    let result = conn.execute(Statement::from_sql_and_values(conn.get_database_backend(),
        "UPDATE ops_ticket_inbox SET updated_at = updated_at WHERE id = ? AND account_id = ?",
        [scope.inbox_id.into(), scope.account_id.into()],
    )).await?;
    if result.rows_affected() == 0 { return Err(DbError::NotFound("ticket inbox in account".into())); }
    Ok(())
}

fn normalized_email(value: &str) -> Result<String, DbError> {
    // Envelope address only; MIME display-name parsing belongs to the adapter.
    let value = value.trim();
    let parts: Vec<_> = value.split('@').collect();
    if parts.len() != 2 || parts.iter().any(|p| p.is_empty()) || value.chars().any(|c| c.is_whitespace() || c.is_control() || "<>".contains(c)) {
        return Err(DbError::Validation("decoded sender email required".into()));
    }
    Ok(value.to_lowercase())
}

pub struct IncomingEmail {
    pub headers: ThreadHeaders,
    pub message_id: String,
    pub sender_email: String,
    pub sender_name: Option<String>,
    pub subject: String,
    pub content: String,
    pub auto_reply: bool,
}

pub struct IngestedEmail {
    pub conversation: conversation::Model,
    pub message: message::Model,
    /// None on idempotent replay, when no finder strategy was executed.
    pub strategy: Option<Strategy>,
    pub duplicate: bool,
}

pub async fn ingest_email(conn: &DatabaseConnection, scope: Scope, mail: IncomingEmail) -> Result<IngestedEmail, DbError> {
    let sender_email = normalized_email(&mail.sender_email)?;
    let source_id = threading::message_id(&mail.message_id);
    if source_id.is_empty() || source_id.chars().any(|c| c.is_control() || c.is_whitespace() || "<>".contains(c)) {
        return Err(DbError::Validation("decoded message-id required for idempotency".into()));
    }
    let txn = conn.begin().await?;
    claim_writer(&txn, scope).await?;
    if let Some(message) = message::Entity::find()
        .filter(message::Column::AccountId.eq(scope.account_id))
        .filter(message::Column::InboxId.eq(scope.inbox_id))
        .filter(message::Column::SourceId.eq(source_id)).one(&txn).await? {
        let conversation = get_conversation(&txn, scope, message.conversation_id).await?;
        txn.commit().await?;
        return Ok(IngestedEmail { conversation, message, strategy: None, duplicate: true });
    }
    let (existing, strategy) = threading::find(&txn, scope, &mail.headers).await?;
    let now = Utc::now();
    // Contacts are account identities. The join makes the same contact usable
    // in multiple inboxes without creating duplicate contacts.
    let sender = match contact::Entity::find()
        .filter(contact::Column::AccountId.eq(scope.account_id))
        .filter(contact::Column::Email.eq(&sender_email)).one(&txn).await? {
        Some(contact) => contact,
        None => contact::ActiveModel {
            account_id: Set(scope.account_id),
            name: Set(mail.sender_name.clone().unwrap_or_else(|| sender_email.split('@').next().unwrap_or_default().into())),
            email: Set(sender_email.clone()), phone_number: Set(None), identifier: Set(None),
            contact_type: Set(0), blocked: Set(false), additional_attributes: Set("{}".into()), custom_attributes: Set("{}".into()),
            created_at: Set(now), updated_at: Set(now), ..Default::default()
        }.insert(&txn).await?,
    };
    let conversation = match existing {
        Some(conversation) => conversation,
        None => {
            let link = match contact_inbox::Entity::find()
                .filter(contact_inbox::Column::AccountId.eq(scope.account_id))
                .filter(contact_inbox::Column::InboxId.eq(scope.inbox_id))
                .filter(contact_inbox::Column::ContactId.eq(sender.id)).one(&txn).await? {
                Some(link) => link,
                None => contact_inbox::ActiveModel {
                    account_id: Set(scope.account_id), inbox_id: Set(scope.inbox_id), contact_id: Set(sender.id),
                    source_id: Set(sender_email), created_at: Set(now), updated_at: Set(now), ..Default::default()
                }.insert(&txn).await?,
            };
            conversation::ActiveModel {
                account_id: Set(scope.account_id), inbox_id: Set(scope.inbox_id), contact_id: Set(sender.id), contact_inbox_id: Set(link.id),
                uuid: Set(uuid::Uuid::new_v4().to_string()), status: Set(if sender.blocked { 1 } else { 0 }), priority: Set(None),
                assignee_id: Set(None), assignee_agent_bot_id: Set(None), team_id: Set(None),
                additional_attributes: Set(serde_json::json!({
                    "in_reply_to": mail.headers.raw_in_reply_to,
                    "source": "email", "auto_reply": mail.auto_reply, "mail_subject": mail.subject,
                    "initiated_at": {"timestamp": now},
                }).to_string()), custom_attributes: Set("{}".into()),
                last_activity_at: Set(now), snoozed_until: Set(None), waiting_since: Set(Some(now)), first_reply_created_at: Set(None),
                created_at: Set(now), updated_at: Set(now), ..Default::default()
            }.insert(&txn).await?
        }
    };
    let message = message::ActiveModel {
        account_id: Set(scope.account_id), inbox_id: Set(scope.inbox_id), conversation_id: Set(conversation.id),
        message_type: Set(0), private: Set(false), content: Set(mail.content), content_type: Set(8), status: Set(0),
        sender_type: Set("Contact".into()), sender_id: Set(sender.id.to_string()), source_id: Set(Some(source_id.into())),
        content_attributes: Set(serde_json::json!({"email": {
            "message_id": source_id, "in_reply_to": mail.headers.in_reply_to,
            "references": mail.headers.references, "auto_reply": mail.auto_reply,
        }}).to_string()), additional_attributes: Set("{}".into()),
        created_at: Set(now), updated_at: Set(now), ..Default::default()
    }.insert(&txn).await?;
    let mut active = conversation.into_active_model();
    active.last_activity_at = Set(now); active.updated_at = Set(now);
    if !sender.blocked {
        // Email replies reopen resolved/snoozed conversations (Message.rb).
        if matches!(active.status.clone().unwrap(), 1 | 3) {
            active.status = Set(0); active.snoozed_until = Set(None);
        }
        if active.waiting_since.clone().unwrap().is_none() { active.waiting_since = Set(Some(now)); }
    }
    let conversation = active.update(&txn).await?;
    txn.commit().await?;
    Ok(IngestedEmail { conversation, message, strategy: Some(strategy), duplicate: false })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Assignment {
    Unassigned,
    User(String),
    AgentBot(String),
}

pub async fn assign(conn: &DatabaseConnection, scope: Scope, id: i32, assignment: Assignment, team_id: Option<String>) -> Result<conversation::Model, DbError> {
    let txn = conn.begin().await?;
    claim_writer(&txn, scope).await?;
    let mut active = get_conversation(&txn, scope, id).await?.into_active_model();
    let (user, bot) = match assignment {
        Assignment::Unassigned => (None, None),
        Assignment::User(id) if !id.trim().is_empty() => (Some(id), None),
        Assignment::AgentBot(id) if !id.trim().is_empty() => (None, Some(id)),
        _ => return Err(DbError::Validation("assignee identity required".into())),
    };
    active.assignee_id = Set(user); active.assignee_agent_bot_id = Set(bot);
    active.team_id = Set(team_id); active.updated_at = Set(Utc::now());
    let row = active.update(&txn).await?;
    txn.commit().await?;
    Ok(row)
}

#[derive(Debug, Clone, Copy)]
pub enum ConversationStatus { Open, Resolved, Pending, Snoozed(chrono::DateTime<Utc>) }

pub async fn set_status(conn: &DatabaseConnection, scope: Scope, id: i32, status: ConversationStatus) -> Result<conversation::Model, DbError> {
    let txn = conn.begin().await?;
    claim_writer(&txn, scope).await?;
    let mut active = get_conversation(&txn, scope, id).await?.into_active_model();
    let (status, snooze) = match status {
        ConversationStatus::Open => (0, None), ConversationStatus::Resolved => (1, None),
        ConversationStatus::Pending => (2, None), ConversationStatus::Snoozed(until) => (3, Some(until)),
    };
    active.status = Set(status); active.snoozed_until = Set(snooze);
    if status == 1 { active.waiting_since = Set(None); }
    active.updated_at = Set(Utc::now());
    let row = active.update(&txn).await?;
    txn.commit().await?;
    Ok(row)
}

/// Internal notes cannot acquire an external source-id or become public via this API.
pub async fn add_private_note(conn: &DatabaseConnection, scope: Scope, id: i32, author: &str, content: &str) -> Result<message::Model, DbError> {
    if author.trim().is_empty() { return Err(DbError::Validation("note author required".into())); }
    let txn = conn.begin().await?;
    claim_writer(&txn, scope).await?;
    let mut conversation = get_conversation(&txn, scope, id).await?.into_active_model();
    let now = Utc::now();
    let message = message::ActiveModel {
        account_id: Set(scope.account_id), inbox_id: Set(scope.inbox_id), conversation_id: Set(id),
        message_type: Set(1), private: Set(true), content: Set(content.into()), content_type: Set(0), status: Set(0),
        sender_type: Set("User".into()), sender_id: Set(author.into()), source_id: Set(None),
        content_attributes: Set("{}".into()), additional_attributes: Set("{}".into()),
        created_at: Set(now), updated_at: Set(now), ..Default::default()
    }.insert(&txn).await?;
    conversation.last_activity_at = Set(now); conversation.updated_at = Set(now);
    conversation.update(&txn).await?;
    txn.commit().await?;
    Ok(message)
}
