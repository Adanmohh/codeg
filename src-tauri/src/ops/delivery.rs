//! Durable send adapter. Status/receipt reads never dispatch or mint a new key.
use super::{email_entity::attempt, review::AuthorizedReply, store, types::*, Operator};
use crate::{
    db::{
        entities::{folder, ops_proposal, work_task},
        error::DbError,
        service::ticket_service::{self as tickets, Scope},
    },
    email_transport::{self, ResendClient, SendEmail},
    models::WorkTaskStatus,
};
use chrono::Utc;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, QueryFilter, Set, Statement, TransactionTrait,
};
use sha2::{Digest, Sha256};

fn conflict() -> DbError {
    DbError::Conflict("An earlier delivery attempt must be resolved before another send".into())
}
fn frozen(payload: &ReplyPayload) -> Result<(String, String), DbError> {
    let json =
        serde_json::to_string(payload).map_err(|_| DbError::Validation("Invalid reply".into()))?;
    let hash = format!("{:x}", Sha256::digest(json.as_bytes()));
    Ok((json, hash))
}
pub(super) async fn for_proposal(
    db: &DatabaseConnection,
    op: &Operator,
    proposal_id: i32,
) -> Result<Option<DeliveryStatus>, DbError> {
    Ok(attempt::Entity::find()
        .filter(attempt::Column::AccountId.eq(op.account_id))
        .filter(attempt::Column::ProposalId.eq(proposal_id))
        .one(db)
        .await?
        .map(dto))
}
fn dto(row: attempt::Model) -> DeliveryStatus {
    DeliveryStatus {
        id: row.id,
        proposal_id: row.proposal_id,
        status: row.status,
        message_id: row.message_id,
        provider_id: row.provider_id,
        error: row.error,
        updated_at: row.updated_at.to_rfc3339(),
    }
}

/// Called after configured-client preflight, before consuming authorization.
/// Reservation is immutable, never reusable for a changed approval. If the
/// process dies after reserve, it cannot replay the attempt from stored JSON.
pub(super) async fn reserve(
    db: &DatabaseConnection,
    op: &Operator,
    proposal: &ops_proposal::Model,
    payload: &ReplyPayload,
) -> Result<(), DbError> {
    let (json, hash) = frozen(payload)?;
    let txn = db.begin().await?;
    txn.execute(Statement::from_sql_and_values(
        txn.get_database_backend(),
        "UPDATE ops_ticket_inbox SET updated_at = updated_at WHERE account_id = ? AND id = ?",
        [op.account_id.into(), payload.reply.inbox_id.into()],
    ))
    .await?;
    store::check_reply_scope(&txn, op, &payload.reply).await?;
    let pending = ops_proposal::Entity::find_by_id(proposal.id)
        .filter(ops_proposal::Column::Status.eq("pending"))
        .filter(ops_proposal::Column::PayloadJson.eq(&proposal.payload_json))
        .one(&txn)
        .await?;
    if pending.is_none() {
        return Err(conflict());
    }
    let existing = txn.query_one(Statement::from_sql_and_values(txn.get_database_backend(),
        "SELECT id FROM ops_email_attempt WHERE proposal_id = ? OR (draft_id = ? AND draft_revision = ?)
         OR (account_id = ? AND inbox_id = ? AND conversation_id = ? AND status IN ('reserved','sending','unknown','receipt_recorded')) LIMIT 1",
        [proposal.id.into(), payload.draft_id.into(), payload.draft_revision.into(), op.account_id.into(), payload.reply.inbox_id.into(), payload.reply.conversation_id.into()])).await?;
    if existing.is_some() {
        return Err(conflict());
    }
    let uuid = uuid::Uuid::new_v4();
    let now = Utc::now();
    attempt::ActiveModel {
        account_id: Set(op.account_id),
        inbox_id: Set(payload.reply.inbox_id),
        conversation_id: Set(payload.reply.conversation_id),
        proposal_id: Set(proposal.id),
        task_id: Set(proposal.task_id),
        run_seq: Set(proposal.run_seq),
        draft_id: Set(payload.draft_id),
        draft_revision: Set(payload.draft_revision),
        payload_json: Set(json),
        payload_sha256: Set(hash),
        message_id: Set(format!(
            "ops-{uuid}@{}",
            payload.reply.from.split_once('@').ok_or_else(conflict)?.1
        )),
        idempotency_key: Set(format!("ops-reply-{uuid}")),
        actor: Set(op.actor.into()),
        status: Set("reserved".into()),
        provider_id: Set(None),
        error: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(&txn)
    .await?;
    txn.commit().await?;
    Ok(())
}

pub(super) async fn not_authorized(
    db: &DatabaseConnection,
    proposal_id: i32,
) -> Result<(), DbError> {
    attempt::Entity::update_many()
        .col_expr(attempt::Column::Status, Expr::value("not_sent"))
        .col_expr(
            attempt::Column::Error,
            Expr::value("Approval failed before dispatch; nothing was sent"),
        )
        .col_expr(attempt::Column::UpdatedAt, Expr::value(Utc::now()))
        .filter(attempt::Column::ProposalId.eq(proposal_id))
        .filter(attempt::Column::Status.eq("reserved"))
        .exec(db)
        .await?;
    Ok(())
}

async fn live<C: ConnectionTrait>(db: &C, row: &attempt::Model) -> Result<bool, DbError> {
    let task = work_task::Entity::find_by_id(row.task_id)
        .filter(work_task::Column::RunSeq.eq(row.run_seq))
        .filter(work_task::Column::DeletedAt.is_null())
        .one(db)
        .await?;
    let Some(task) = task else {
        return Ok(false);
    };
    Ok(matches!(
        task.status,
        WorkTaskStatus::Running | WorkTaskStatus::AwaitingInput
    ) && folder::Entity::find_by_id(task.folder_id)
        .filter(folder::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .is_some())
}

/// This is the sole network-send call in Ops. Only a committed, owned approval
/// can claim a reservation. No code path reconstructs authorization from a row.
pub(super) async fn dispatch(
    db: &DatabaseConnection,
    client: ResendClient,
    authorized: AuthorizedReply,
) -> Result<DeliveryStatus, DbError> {
    let (proposal_id, payload) = authorized.into_parts();
    let (json, hash) = frozen(&payload)?;
    let txn = db.begin().await?;
    let changed = attempt::Entity::update_many()
        .col_expr(attempt::Column::Status, Expr::value("sending"))
        .col_expr(attempt::Column::UpdatedAt, Expr::value(Utc::now()))
        .filter(attempt::Column::ProposalId.eq(proposal_id))
        .filter(attempt::Column::Status.eq("reserved"))
        .filter(attempt::Column::PayloadJson.eq(json))
        .filter(attempt::Column::PayloadSha256.eq(hash))
        .exec(&txn)
        .await?;
    if changed.rows_affected != 1 {
        return Err(conflict());
    }
    let row = attempt::Entity::find()
        .filter(attempt::Column::ProposalId.eq(proposal_id))
        .one(&txn)
        .await?
        .ok_or_else(conflict)?;
    let approved = ops_proposal::Entity::find_by_id(proposal_id)
        .filter(ops_proposal::Column::Status.eq("approved"))
        .filter(ops_proposal::Column::TaskId.eq(row.task_id))
        .filter(ops_proposal::Column::RunSeq.eq(row.run_seq))
        .one(&txn)
        .await?;
    if approved.is_none() || !live(&txn, &row).await? {
        txn.rollback().await?;
        not_authorized(db, proposal_id).await?;
        return status_by_id(db, row.id).await;
    }
    txn.commit().await?;
    // Payload comes from the consumed AuthorizedReply, never a mutable draft.
    let reply = payload.reply;
    let email = SendEmail {
        to: reply.to,
        cc: reply.cc,
        bcc: reply.bcc,
        reply_to: vec![],
        subject: reply.subject,
        text: Some(reply.text),
        html: None,
        message_id: row.message_id.clone(),
        in_reply_to: reply.in_reply_to,
        references: reply.references,
        idempotency_key: row.idempotency_key.clone(),
    };
    match client.send(db, &email).await {
        Ok(receipt) => {
            // Persist evidence BEFORE touching ticket history. If this write
            // fails, 'sending' is an unknown outcome: no resend route exists.
            let changed = attempt::Entity::update_many()
                .col_expr(
                    attempt::Column::ProviderId,
                    Expr::value(receipt.id.as_str()),
                )
                .col_expr(attempt::Column::Status, Expr::value("receipt_recorded"))
                .col_expr(attempt::Column::UpdatedAt, Expr::value(Utc::now()))
                .filter(attempt::Column::Id.eq(row.id))
                .filter(attempt::Column::Status.eq("sending"))
                .exec(db)
                .await?;
            if changed.rows_affected != 1 {
                return Err(conflict());
            }
            // A local persistence failure does not erase the provider receipt.
            if finish_receipt(db, row.id).await.is_err() {
                attempt::Entity::update_many().col_expr(attempt::Column::Error,
                    Expr::value("Provider accepted this reply; local thread recording is pending. Do not resend"))
                    .filter(attempt::Column::Id.eq(row.id)).filter(attempt::Column::Status.eq("receipt_recorded")).exec(db).await?;
            }
        }
        Err(error) => {
            let definite = matches!(
                error,
                email_transport::Error::InvalidInput(_)
                    | email_transport::Error::Inbox
                    | email_transport::Error::Http {
                        status: 400 | 401 | 403 | 404 | 422 | 429,
                        ..
                    }
            );
            let status = if definite { "failed" } else { "unknown" };
            let message = if definite {
                "Provider did not accept this reply. Correct configuration or payload and prepare a new reviewed draft"
            } else {
                "Delivery outcome is unknown. Do not resend or create a new delivery key; reconcile with the provider first"
            };
            attempt::Entity::update_many()
                .col_expr(attempt::Column::Status, Expr::value(status))
                .col_expr(attempt::Column::Error, Expr::value(message))
                .col_expr(attempt::Column::UpdatedAt, Expr::value(Utc::now()))
                .filter(attempt::Column::Id.eq(row.id))
                .filter(attempt::Column::Status.eq("sending"))
                .exec(db)
                .await?;
        }
    }
    status_by_id(db, row.id).await
}

async fn status_by_id(db: &DatabaseConnection, id: i32) -> Result<DeliveryStatus, DbError> {
    Ok(dto(attempt::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(conflict)?))
}

/// Idempotent database-only receipt completion. Reuses the immutable attempt
/// envelope and receipt; it has no client/key/dispatch access and cannot resend.
async fn finish_receipt(db: &DatabaseConnection, id: i32) -> Result<(), DbError> {
    let row = attempt::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(conflict)?;
    if row.status == "sent" {
        return Ok(());
    }
    if row.status != "receipt_recorded" || row.provider_id.is_none() {
        return Err(conflict());
    }
    let payload: ReplyPayload = serde_json::from_str(&row.payload_json).map_err(|_| conflict())?;
    if frozen(&payload)?.1 != row.payload_sha256 {
        return Err(conflict());
    }
    tickets::record_public_reply(
        db,
        Scope {
            account_id: row.account_id,
            inbox_id: row.inbox_id,
        },
        row.conversation_id,
        &row.actor,
        &payload.reply.text,
        &row.message_id,
    )
    .await?;
    attempt::Entity::update_many()
        .col_expr(attempt::Column::Status, Expr::value("sent"))
        .col_expr(attempt::Column::Error, Expr::value(Option::<String>::None))
        .col_expr(attempt::Column::UpdatedAt, Expr::value(Utc::now()))
        .filter(attempt::Column::Id.eq(id))
        .filter(attempt::Column::Status.eq("receipt_recorded"))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn reconcile_receipt(
    db: &DatabaseConnection,
    op: &Operator,
    input: ProposalInput,
) -> Result<DeliveryStatus, DbError> {
    let row = attempt::Entity::find()
        .filter(attempt::Column::ProposalId.eq(input.id))
        .filter(attempt::Column::AccountId.eq(op.account_id))
        .one(db)
        .await?
        .ok_or_else(|| DbError::NotFound("receipt".into()))?;
    tickets::get_conversation(db, op.scope(row.inbox_id), row.conversation_id).await?;
    finish_receipt(db, row.id).await?;
    status_by_id(db, row.id).await
}
