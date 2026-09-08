//! Narrow stored public projection for the explicitly granted business binding.
use super::{email_entity::config, Operator};
use crate::db::{
    error::DbError,
    service::ticket_service::{self as tickets, Scope},
};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

pub(crate) async fn resolve<C: ConnectionTrait>(
    db: &C,
    op: &Operator,
    inbox_id: i32,
) -> Result<i32, DbError> {
    credential(db, op.account_id(), inbox_id).await?;
    Ok(op.account_id())
}
// Private credential reference, never a transport response or agent capability.
pub(crate) async fn credential<C: ConnectionTrait>(
    db: &C,
    account_id: i32,
    inbox_id: i32,
) -> Result<String, DbError> {
    let inbox = tickets::require_inbox(
        db,
        Scope {
            account_id,
            inbox_id,
        },
    )
    .await?;
    if inbox.channel_type != "Channel::Email" {
        return Err(DbError::NotFound("email inbox".into()));
    }
    config::Entity::find_by_id(inbox_id)
        .filter(config::Column::AccountId.eq(account_id))
        .one(db)
        .await?
        .map(|row| row.credential_ref)
        .ok_or_else(|| DbError::NotFound("configured email inbox".into()))
}
pub(crate) async fn message<C: ConnectionTrait>(
    db: &C,
    account_id: i32,
    inbox_id: i32,
    conversation_id: i32,
    message_id: i32,
) -> Result<String, DbError> {
    let row = tickets::get_public_message(
        db,
        Scope {
            account_id,
            inbox_id,
        },
        conversation_id,
        message_id,
    )
    .await?;
    // Accepted ingest_email stores its normalized text at email type8; authored
    // public replies use text type0. Read only content, never MIME attributes,
    // contact data, private notes or activity.
    if !matches!(row.content_type, 0 | 8) {
        return Err(DbError::Validation("plain public email required".into()));
    }
    Ok(row.content)
}
