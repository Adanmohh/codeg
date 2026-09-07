//! Closed email action pack. No string-selected executor or transport credentials.
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, Statement,
};
use serde_json::Value;

use super::{draft_entity as draft, store, types::*, Operator};
use crate::{
    app_error::AppCommandError,
    db::{
        entities::{folder, ops_proposal, work_task},
        error::DbError,
        service::ops_approvals::{
            self as approvals, Action, ActionContext, AuthorizedAction, Decision, ProposalOutcome,
        },
    },
};

const ACTION: &str = "ops.email.reply";

fn conflict() -> DbError {
    DbError::Conflict("stale proposal or draft".into())
}
fn payload(value: &Value) -> Result<ReplyPayload, DbError> {
    serde_json::from_value(value.clone())
        .map_err(|_| DbError::Validation("Complete reply payload required".into()))
}

struct EmailReplyAction {
    operator: Operator,
    binding: ReplyPayload,
}

impl EmailReplyAction {
    async fn check(&self, value: &Value, ctx: &ActionContext<'_>) -> Result<(), DbError> {
        self.validate(value)?;
        let p = payload(value)?;
        store::check_reply_scope(ctx.db, &self.operator, &p.reply).await?;
        let row = draft::Entity::find_by_id(p.draft_id)
            .filter(draft::Column::AccountId.eq(self.operator.account_id))
            .filter(draft::Column::InboxId.eq(p.reply.inbox_id))
            .filter(draft::Column::ConversationId.eq(p.reply.conversation_id))
            .filter(draft::Column::Revision.eq(p.draft_revision))
            .one(ctx.db)
            .await?;
        if row.is_none() {
            return Err(conflict());
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl Action for EmailReplyAction {
    fn name(&self) -> &'static str {
        ACTION
    }
    fn domain(&self) -> &'static str {
        "email"
    }
    fn validate(&self, value: &Value) -> Result<(), DbError> {
        let p = payload(value)?;
        store::validate_reply(&p.reply, true)?;
        if p.draft_id != self.binding.draft_id
            || p.draft_revision != self.binding.draft_revision
            || p.reply.inbox_id != self.binding.reply.inbox_id
            || p.reply.conversation_id != self.binding.reply.conversation_id
        {
            return Err(DbError::Validation(
                "Review cannot change the bound draft or thread".into(),
            ));
        }
        Ok(())
    }
    async fn resource(
        &self,
        value: &Value,
        ctx: &ActionContext<'_>,
    ) -> Result<Option<String>, DbError> {
        // Resource runs even before an ask rule short-circuits the gate. Never
        // create a pending proposal for an unscoped or stale draft via ask.
        self.check(value, ctx).await?;
        Ok(Some(format!(
            "account/{}/inbox/{}",
            self.operator.account_id, self.binding.reply.inbox_id
        )))
    }
    async fn check_permission(
        &self,
        value: &Value,
        ctx: &ActionContext<'_>,
    ) -> Result<Decision, DbError> {
        self.check(value, ctx).await?;
        Ok(Decision::Passthrough)
    }
    fn is_destructive(&self, _: &Value) -> Result<bool, DbError> {
        Ok(true)
    }
    fn private_fields(&self) -> &'static [&'static str] {
        &["reply"]
    }
}

/// Trusted in-process integration seam ONLY. The future per-run capability
/// adapter must resolve task/run/agent and account from its own registry, never
/// from HTTP JSON. This module exposes no agent route or bearer token.
pub async fn propose_reply(
    db: &DatabaseConnection,
    task_id: i32,
    run_seq: i32,
    agent: &str,
    account_id: i32,
    draft_id: i32,
    expected_revision: i32,
) -> Result<ProposalOutcome, DbError> {
    let row = draft::Entity::find_by_id(draft_id)
        .filter(draft::Column::AccountId.eq(account_id))
        .filter(draft::Column::Revision.eq(expected_revision))
        .one(db)
        .await?
        .ok_or_else(conflict)?;
    let reply: Reply = serde_json::from_str(&row.reply_json).map_err(|_| conflict())?;
    let binding = ReplyPayload {
        draft_id,
        draft_revision: row.revision,
        reply,
    };
    let value = serde_json::to_value(&binding).map_err(|_| conflict())?;
    let action = EmailReplyAction {
        operator: Operator {
            account_id,
            actor: "internal:proposal",
        },
        binding,
    };
    approvals::propose(db, task_id, run_seq, agent, &action, value).await
}

async fn scoped_rows(
    db: &DatabaseConnection,
    op: &Operator,
    id: Option<i32>,
) -> Result<Vec<ops_proposal::Model>, DbError> {
    // Ownership is joined through immutable draftId, not arbitrary accountId
    // inside agent JSON. Terminal redaction preserves that ID and removes reply.
    Ok(ops_proposal::Entity::find().from_raw_sql(Statement::from_sql_and_values(
        db.get_database_backend(),
        "SELECT p.* FROM ops_proposal p JOIN ops_reply_draft d
         ON d.id = json_extract(CASE WHEN json_valid(p.payload_json) THEN p.payload_json ELSE '{}' END, '$.draftId')
         WHERE d.account_id = ? AND (? IS NULL OR p.id = ?)
         ORDER BY CASE WHEN p.status = 'pending' THEN 0 ELSE 1 END, p.id DESC LIMIT 100",
        [op.account_id.into(), id.into(), id.into()],
    )).all(db).await?)
}

async fn scoped_row(
    db: &DatabaseConnection,
    op: &Operator,
    id: i32,
) -> Result<ops_proposal::Model, DbError> {
    scoped_rows(db, op, Some(id))
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| DbError::NotFound("proposal".into()))
}

async fn dto(
    db: &DatabaseConnection,
    op: &Operator,
    row: ops_proposal::Model,
) -> Result<Proposal, DbError> {
    let raw: Value = serde_json::from_str(&row.payload_json).map_err(|_| conflict())?;
    let draft_id = raw["draftId"]
        .as_i64()
        .and_then(|id| i32::try_from(id).ok())
        .ok_or_else(conflict)?;
    let draft = draft::Entity::find_by_id(draft_id)
        .filter(draft::Column::AccountId.eq(op.account_id))
        .one(db)
        .await?
        .ok_or_else(conflict)?;
    let task = work_task::Entity::find_by_id(row.task_id).one(db).await?;
    let mut live = false;
    if let Some(task) = task {
        live = task.run_seq == row.run_seq
            && task.status == crate::models::WorkTaskStatus::AwaitingInput
            && task.deleted_at.is_none()
            && folder::Entity::find_by_id(task.folder_id)
                .filter(folder::Column::DeletedAt.is_null())
                .one(db)
                .await?
                .is_some();
    }
    let unsupported = row.action_name != ACTION;
    let pending = row.status == "pending";
    let parsed = if pending && !unsupported {
        Some(payload(&raw)?)
    } else {
        None
    };
    let draft_changed = parsed
        .as_ref()
        .is_some_and(|p| p.draft_revision != draft.revision);
    let stale = pending && (!live || draft_changed);
    Ok(Proposal {
        id: row.id,
        task_id: row.task_id,
        run_seq: row.run_seq,
        inbox_id: draft.inbox_id,
        conversation_id: draft.conversation_id,
        status: row.status,
        stale,
        reason: if unsupported {
            Some("This action has no registered review adapter")
        } else if pending && !live {
            Some("The task run has changed; this proposal can no longer be approved")
        } else if draft_changed {
            Some("A newer draft was saved; request a new proposal")
        } else {
            None
        },
        payload: parsed,
        created_at: row.created_at.to_rfc3339(),
    })
}

pub async fn list(db: &DatabaseConnection, op: &Operator) -> Result<Vec<Proposal>, DbError> {
    let mut result = vec![];
    for row in scoped_rows(db, op, None).await? {
        result.push(dto(db, op, row).await?);
    }
    Ok(result)
}

pub async fn get(
    db: &DatabaseConnection,
    op: &Operator,
    input: ProposalInput,
) -> Result<Proposal, DbError> {
    dto(db, op, scoped_row(db, op, input.id).await?).await
}

async fn review_binding(
    db: &DatabaseConnection,
    op: &Operator,
    id: i32,
    expected: &Value,
) -> Result<(ops_proposal::Model, EmailReplyAction), DbError> {
    let row = scoped_row(db, op, id).await?;
    let original: Value = serde_json::from_str(&row.payload_json).map_err(|_| conflict())?;
    if row.status != "pending" || row.action_name != ACTION || original != *expected {
        return Err(conflict());
    }
    let action = EmailReplyAction {
        operator: op.clone(),
        binding: payload(&original)?,
    };
    Ok((row, action))
}

pub async fn deny(
    db: &DatabaseConnection,
    op: &Operator,
    input: DenyInput,
) -> Result<Proposal, DbError> {
    let (row, action) = review_binding(db, op, input.id, &input.expected_payload).await?;
    // Pending payloads are immutable in the core. A racing terminal decision
    // fails its transactional pending/run CAS, including after this read.
    approvals::deny(db, row.task_id, row.run_seq, row.id, op.actor, &action).await?;
    get(db, op, ProposalInput { id: row.id }).await
}

pub async fn approve(
    db: &DatabaseConnection,
    op: &Operator,
    input: ReviewInput,
) -> Result<Proposal, AppCommandError> {
    let (_, action) = review_binding(db, op, input.id, &input.expected_payload)
        .await
        .map_err(super::command_error)?;
    action
        .validate(
            &serde_json::to_value(input.approved_payload)
                .map_err(|_| AppCommandError::invalid_input("Complete reply required"))?,
        )
        .map_err(super::command_error)?;
    let current = get(db, op, ProposalInput { id: input.id })
        .await
        .map_err(super::command_error)?;
    if current.stale {
        return Err(super::command_error(conflict()));
    }
    // Deliberately before approvals::approve. No transport/attempt store exists
    // yet: consuming authorization here would strand the exact owned payload.
    Err(AppCommandError::configuration_missing(
        store::TRANSPORT_MESSAGE,
    ))
}

/// Owned handoff for the future trusted dispatcher. It must persist an attempt,
/// message ID and idempotency key bound to this payload before invoking Resend,
/// then record a public reply only after a verified provider receipt. No Clone,
/// Serialize or database reread. This is not a send operation.
pub struct AuthorizedReply {
    proposal_id: i32,
    payload: ReplyPayload,
}

impl AuthorizedReply {
    pub fn proposal_id(&self) -> i32 {
        self.proposal_id
    }
    pub fn payload(&self) -> &ReplyPayload {
        &self.payload
    }
    pub fn into_parts(self) -> (i32, ReplyPayload) {
        (self.proposal_id, self.payload)
    }

    pub fn from_action(action: AuthorizedAction) -> Result<Self, DbError> {
        if action.action_name() != ACTION {
            return Err(DbError::Validation("Unregistered dispatch action".into()));
        }
        let proposal_id = action.proposal_id();
        let payload = payload(&action.into_payload())?;
        store::validate_reply(&payload.reply, true)?;
        Ok(Self {
            proposal_id,
            payload,
        })
    }
}

#[cfg(test)]
pub(super) async fn authorize_for_test(
    db: &DatabaseConnection,
    op: &Operator,
    input: ReviewInput,
) -> Result<AuthorizedReply, DbError> {
    let (row, action) = review_binding(db, op, input.id, &input.expected_payload).await?;
    let authorized = approvals::approve(
        db,
        row.task_id,
        row.run_seq,
        row.id,
        op.actor,
        &action,
        approvals::Review {
            expected_payload: &input.expected_payload,
            approved_payload: serde_json::to_value(input.approved_payload).unwrap(),
        },
    )
    .await?;
    AuthorizedReply::from_action(authorized)
}
