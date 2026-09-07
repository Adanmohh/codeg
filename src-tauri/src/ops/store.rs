use chrono::Utc;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, Statement, TransactionTrait,
};
use serde_json::Value;

use super::{draft_entity as draft, types::*, Operator};
use crate::db::{
    entities::ops_ticket::{conversation, inbox, message},
    error::DbError,
    service::{ticket_service as tickets, work_task_service},
};

pub const TRANSPORT_MESSAGE: &str = "Connect Resend per inbox. Drafts and private notes stay local; every send requires human approval.";
const PAGE_SIZE: u64 = 50;

pub async fn context(db: &DatabaseConnection, op: &Operator) -> Result<Context, DbError> {
    Ok(Context {
        account_id: op.account_id,
        operator: op.actor,
        inboxes: tickets::list_inboxes(db, op.account_id)
            .await?
            .into_iter()
            .map(inbox_dto)
            .collect(),
        email_transport: "per_inbox",
        transport_message: TRANSPORT_MESSAGE,
    })
}

fn inbox_dto(row: inbox::Model) -> Inbox {
    Inbox {
        id: row.id,
        name: row.name,
        email: row.email_address,
    }
}

pub async fn create_inbox(
    db: &DatabaseConnection,
    op: &Operator,
    input: InboxInput,
) -> Result<Inbox, DbError> {
    if input.name.trim().is_empty() || input.name.len() > 120 || !mailbox(&input.email) {
        return Err(DbError::Validation(
            "Enter an inbox name and a plain email address".into(),
        ));
    }
    Ok(inbox_dto(
        tickets::create_inbox(db, op.account_id, input.name.trim(), &input.email).await?,
    ))
}

async fn ticket_dto<C: ConnectionTrait>(
    db: &C,
    op: &Operator,
    row: conversation::Model,
) -> Result<Ticket, DbError> {
    let contact = tickets::get_contact(db, op.scope(row.inbox_id), row.id).await?;
    let attributes: Value = serde_json::from_str(&row.additional_attributes).unwrap_or_default();
    Ok(Ticket {
        id: row.id,
        inbox_id: row.inbox_id,
        subject: attributes["mail_subject"]
            .as_str()
            .unwrap_or("(No subject)")
            .to_owned(),
        contact: if contact.name.is_empty() {
            contact.email
        } else {
            contact.name
        },
        status: row.status,
        updated_at: row.last_activity_at.to_rfc3339(),
    })
}

pub async fn list_tickets<C: ConnectionTrait>(
    db: &C,
    op: &Operator,
    input: TicketsInput,
) -> Result<TicketPage, DbError> {
    tickets::require_inbox(db, op.scope(input.inbox_id)).await?;
    if input.page > 10000 || input.status.is_some_and(|s| !(0..=3).contains(&s)) {
        return Err(DbError::Validation("Invalid ticket page or status".into()));
    }
    let mut query = conversation::Entity::find()
        .filter(conversation::Column::AccountId.eq(op.account_id))
        .filter(conversation::Column::InboxId.eq(input.inbox_id));
    if let Some(status) = input.status {
        query = query.filter(conversation::Column::Status.eq(status));
    }
    let mut rows = query
        .order_by_desc(conversation::Column::LastActivityAt)
        .order_by_desc(conversation::Column::Id)
        .offset(u64::from(input.page) * PAGE_SIZE)
        .limit(PAGE_SIZE + 1)
        .all(db)
        .await?;
    let has_more = rows.len() > PAGE_SIZE as usize;
    rows.truncate(PAGE_SIZE as usize);
    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        items.push(ticket_dto(db, op, row).await?);
    }
    Ok(TicketPage { items, has_more })
}

pub(super) async fn find_draft<C: ConnectionTrait>(
    db: &C,
    op: &Operator,
    input: ThreadInput,
) -> Result<Option<draft::Model>, DbError> {
    Ok(draft::Entity::find()
        .filter(draft::Column::AccountId.eq(op.account_id))
        .filter(draft::Column::InboxId.eq(input.inbox_id))
        .filter(draft::Column::ConversationId.eq(input.conversation_id))
        .one(db)
        .await?)
}

fn draft_dto(row: draft::Model) -> Result<Draft, DbError> {
    Ok(Draft {
        id: row.id,
        revision: row.revision,
        reply: serde_json::from_str(&row.reply_json)
            .map_err(|_| DbError::Validation("Stored draft is unreadable".into()))?,
        updated_at: row.updated_at.to_rfc3339(),
    })
}

fn message_dto(row: message::Model) -> Message {
    Message {
        id: row.id,
        content: row.content,
        private: row.private,
        outgoing: row.message_type == 1,
        author: row.sender_id,
        status: row.status,
        source_id: row.source_id,
        created_at: row.created_at.to_rfc3339(),
    }
}

pub async fn thread(
    db: &DatabaseConnection,
    op: &Operator,
    input: ThreadInput,
) -> Result<Thread, DbError> {
    thread_for_view(db, op, input, tickets::MessageView::Internal, None).await
}

pub(super) async fn thread_for_view(
    db: &DatabaseConnection,
    op: &Operator,
    input: ThreadInput,
    view: tickets::MessageView,
    run: Option<&super::agent::RunContext>,
) -> Result<Thread, DbError> {
    let txn = db.begin().await?;
    if let Some(ctx) = run {
        super::agent::require_live(&txn, ctx).await?;
    }
    let scope = op.scope(input.inbox_id);
    let inbox = tickets::require_inbox(&txn, scope).await?;
    let row = tickets::get_conversation(&txn, scope, input.conversation_id).await?;
    let contact = tickets::get_contact(&txn, scope, row.id).await?;
    let messages = tickets::list_messages(&txn, scope, row.id, view).await?;
    let ticket = ticket_dto(&txn, op, row).await?;
    let last_public = messages
        .iter()
        .rev()
        .find(|m| !m.private && m.source_id.is_some());
    let in_reply_to = last_public.and_then(|m| m.source_id.clone());
    let attributes: Value = last_public
        .and_then(|m| serde_json::from_str(&m.content_attributes).ok())
        .unwrap_or_default();
    let mut references: Vec<String> = attributes["email"]["references"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    if let Some(id) = &in_reply_to {
        if !references.contains(id) {
            references.push(id.clone());
        }
    }
    let suggested_reply = Reply {
        inbox_id: input.inbox_id,
        conversation_id: input.conversation_id,
        from: inbox.email_address,
        to: vec![contact.email.clone()],
        cc: vec![],
        bcc: vec![],
        subject: if ticket.subject.to_ascii_lowercase().starts_with("re:") {
            ticket.subject.clone()
        } else {
            format!("Re: {}", ticket.subject)
        },
        text: String::new(),
        in_reply_to,
        references,
    };
    let draft = find_draft(&txn, op, input)
        .await?
        .map(draft_dto)
        .transpose()?;
    txn.commit().await?;
    Ok(Thread {
        ticket,
        contact_email: contact.email,
        messages: messages.into_iter().map(message_dto).collect(),
        draft,
        suggested_reply,
    })
}

pub async fn add_note(
    db: &DatabaseConnection,
    op: &Operator,
    input: NoteInput,
) -> Result<Message, DbError> {
    if input.content.trim().is_empty() {
        return Err(DbError::Validation("Write a private note first".into()));
    }
    Ok(message_dto(
        tickets::add_private_note(
            db,
            op.scope(input.inbox_id),
            input.conversation_id,
            op.actor,
            &input.content,
        )
        .await?,
    ))
}

// Plain addr-spec subset, consistent with existing local ticket inputs. The
// provider adapter must additionally validate its own supported RFC subset.
fn mailbox(value: &str) -> bool {
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && !domain.is_empty()
        && value.len() <= 254
        && value.is_ascii()
        && !value
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || "<>(),:;\\\"@".contains(c) && c != '@')
        && !domain.contains('@')
}

fn header_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 998
        && value.contains('@')
        && value.is_ascii()
        && !value
            .chars()
            .any(|c| c.is_control() || c.is_whitespace() || "<>".contains(c))
}

pub(super) fn validate_reply(reply: &Reply, sending: bool) -> Result<(), DbError> {
    let count = reply.to.len() + reply.cc.len() + reply.bcc.len();
    if reply.inbox_id <= 0
        || reply.conversation_id <= 0
        || !mailbox(&reply.from)
        || count > 50
        || reply
            .to
            .iter()
            .chain(&reply.cc)
            .chain(&reply.bcc)
            .any(|a| !mailbox(a))
        || reply.subject.len() > 998
        || reply.subject.chars().any(char::is_control)
        || reply.text.chars().count() > 150000
        || reply.text.contains('\0')
        || reply.references.len() > 100
        || reply.references.iter().any(|id| !header_id(id))
        || reply
            .in_reply_to
            .as_deref()
            .is_some_and(|id| !header_id(id))
        || (sending && (reply.to.is_empty() || reply.text.trim().is_empty()))
    {
        return Err(DbError::Validation(
            "Check recipients, sender, message and thread headers; attachments are not supported"
                .into(),
        ));
    }
    Ok(())
}

pub(super) async fn check_reply_scope<C: ConnectionTrait>(
    db: &C,
    op: &Operator,
    reply: &Reply,
) -> Result<(), DbError> {
    let scope = op.scope(reply.inbox_id);
    let inbox = tickets::require_inbox(db, scope).await?;
    tickets::get_conversation(db, scope, reply.conversation_id).await?;
    if reply.from != inbox.email_address {
        return Err(DbError::Validation(
            "Sender must match this inbox's email address".into(),
        ));
    }
    let contact = tickets::get_contact(db, scope, reply.conversation_id).await?;
    if contact.blocked {
        return Err(DbError::Validation("This contact is blocked".into()));
    }
    Ok(())
}

pub async fn save_draft(
    db: &DatabaseConnection,
    op: &Operator,
    input: SaveDraftInput,
) -> Result<Draft, DbError> {
    save_draft_for_run(db, op, input, None, op.actor).await
}

pub(super) async fn save_draft_for_run(
    db: &DatabaseConnection,
    op: &Operator,
    input: SaveDraftInput,
    run: Option<&super::agent::RunContext>,
    actor: &str,
) -> Result<Draft, DbError> {
    validate_reply(&input.reply, false)?;
    if input.expected_revision < 0
        || input.expected_revision == i32::MAX
        || input.inbox_id != input.reply.inbox_id
        || input.conversation_id != input.reply.conversation_id
    {
        return Err(DbError::Validation(
            "Draft scope or revision does not match".into(),
        ));
    }
    let txn = db.begin().await?;
    // Codeg canvas writer-first transaction pattern. Serializes other pooled
    // writers before reading the draft, including concurrent first saves.
    txn.execute(Statement::from_sql_and_values(
        txn.get_database_backend(),
        "UPDATE ops_ticket_inbox SET updated_at = updated_at WHERE id = ? AND account_id = ?",
        [input.inbox_id.into(), op.account_id.into()],
    ))
    .await?;
    if let Some(ctx) = run {
        super::agent::require_live(&txn, ctx).await?;
    }
    check_reply_scope(&txn, op, &input.reply).await?;
    let key = ThreadInput {
        inbox_id: input.inbox_id,
        conversation_id: input.conversation_id,
    };
    let existing = find_draft(&txn, op, key).await?;
    if existing.as_ref().map_or(0, |d| d.revision) != input.expected_revision {
        return Err(DbError::Conflict("stale draft".into()));
    }
    let json = serde_json::to_string(&input.reply)
        .map_err(|_| DbError::Validation("Invalid reply".into()))?;
    let now = Utc::now();
    if let Some(row) = existing {
        let result = draft::Entity::update_many()
            .col_expr(draft::Column::ReplyJson, Expr::value(json))
            .col_expr(
                draft::Column::Revision,
                Expr::value(input.expected_revision + 1),
            )
            .col_expr(draft::Column::UpdatedAt, Expr::value(now))
            .col_expr(draft::Column::UpdatedBy, Expr::value(actor))
            .filter(draft::Column::Id.eq(row.id))
            .filter(draft::Column::Revision.eq(input.expected_revision))
            .exec(&txn)
            .await?;
        if result.rows_affected != 1 {
            return Err(DbError::Conflict("stale draft".into()));
        }
    } else {
        draft::ActiveModel {
            account_id: Set(op.account_id),
            inbox_id: Set(input.inbox_id),
            conversation_id: Set(input.conversation_id),
            revision: Set(1),
            reply_json: Set(json),
            updated_by: Set(actor.into()),
            updated_at: Set(now),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
    }
    let result = draft_dto(
        find_draft(&txn, op, key)
            .await?
            .ok_or_else(|| DbError::Conflict("missing draft".into()))?,
    )?;
    txn.commit().await?;
    Ok(result)
}

pub async fn morning(db: &DatabaseConnection, op: &Operator) -> Result<Morning, DbError> {
    let tasks = work_task_service::list(db, None).await?;
    let rows = conversation::Entity::find()
        .filter(conversation::Column::AccountId.eq(op.account_id))
        .filter(conversation::Column::Status.ne(1))
        .order_by_desc(conversation::Column::LastActivityAt)
        .limit(50)
        .all(db)
        .await?;
    let mut tickets = Vec::with_capacity(rows.len());
    for row in rows {
        tickets.push(ticket_dto(db, op, row).await?);
    }
    Ok(Morning {
        tasks,
        tickets,
        proposals: super::review::list(db, op).await?,
    })
}
