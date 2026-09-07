//! Typed, opt-in email-review notices using the existing Telegram backend.
//! Locator delivery never grants approval or changes work_task/ACP state.
mod entity;
pub mod types;

use crate::{
    chat_channel::{
        backends::telegram::{OpsSendError, TelegramBackend},
        types::TelegramConfig,
    },
    db::{
        entities::{chat_channel, ops_proposal},
        error::DbError,
    },
    ops::{review, Operator},
};
use chrono::Utc;
use entity::{config, notice};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection,
    DatabaseTransaction, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, QuerySelect, Set,
    Statement, TransactionTrait,
};
use sha2::{Digest, Sha256};
use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};
use types::*;

pub struct TelegramRuntime {
    token: Arc<dyn Fn(i32) -> Option<String> + Send + Sync>,
    secret_lock: Arc<std::sync::Mutex<()>>,
    #[cfg(test)]
    mock: Option<std::net::SocketAddr>,
}
impl TelegramRuntime {
    pub(crate) fn production() -> Arc<Self> {
        static RUNTIME: OnceLock<Arc<TelegramRuntime>> = OnceLock::new();
        RUNTIME
            .get_or_init(|| {
                Arc::new(Self {
                    token: Arc::new(crate::keyring_store::get_channel_token),
                    secret_lock: Arc::new(std::sync::Mutex::new(())),
                    #[cfg(test)]
                    mock: None,
                })
            })
            .clone()
    }
    #[cfg(test)]
    pub(crate) fn fixture(
        token: impl Fn(i32) -> Option<String> + Send + Sync + 'static,
        address: std::net::SocketAddr,
    ) -> Arc<Self> {
        assert!(address.ip().is_loopback());
        Arc::new(Self {
            token: Arc::new(token),
            secret_lock: Arc::new(std::sync::Mutex::new(())),
            mock: Some(address),
        })
    }
    async fn secret(&self, channel_id: i32) -> Option<String> {
        let read = self.token.clone();
        let lock = self.secret_lock.clone();
        // Existing OS keyring calls are synchronous. Keep them off the channel
        // scheduler's executor; at most one lookup can remain blocked. A late
        // read cannot create a client or send anything after our timeout.
        tokio::time::timeout(
            Duration::from_secs(2),
            tokio::task::spawn_blocking(move || {
                let _guard = lock.try_lock().ok()?;
                read(channel_id)
            }),
        )
        .await
        .ok()?
        .ok()?
    }
    async fn backend(&self, row: &config::Model) -> Option<TelegramBackend> {
        let token = self.secret(row.channel_id).await?;
        let id = private_id(&row.private_user_id).ok()?;
        #[cfg(test)]
        if let Some(address) = self.mock {
            drop(token);
            return Some(TelegramBackend::ops_local_mock(row.channel_id, id, address));
        }
        TelegramBackend::for_ops(row.channel_id, token, id).ok()
    }
}

fn invalid(message: &str) -> DbError {
    DbError::Validation(message.into())
}
fn conflict() -> DbError {
    DbError::Conflict("Telegram settings changed; reload before saving".into())
}
fn private_id(value: &str) -> Result<i64, DbError> {
    value.parse::<i64>().ok().filter(|id| *id > 0 && id.to_string() == value)
        .ok_or_else(|| invalid("Enter the canonical positive numeric Telegram user ID for the authorized private recipient"))
}
fn origin(value: &str) -> Result<String, DbError> {
    let url =
        reqwest::Url::parse(value).map_err(|_| invalid("Enter a protected HTTPS review origin"))?;
    let loopback = matches!(url.host_str(), Some("127.0.0.1" | "[::1]" | "localhost"));
    if value.len() > 2048
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
        || !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
    {
        return Err(invalid("Use a protected HTTPS origin without credentials, path, query or fragment. Loopback HTTP is only for local preview"));
    }
    Ok(url.origin().ascii_serialization())
}
fn channel_hash(row: &chat_channel::Model, user: &str) -> Result<String, DbError> {
    let parsed: TelegramConfig = serde_json::from_str(&row.config_json)
        .map_err(|_| invalid("This channel has invalid Telegram configuration"))?;
    private_id(user)?;
    if row.channel_type != "telegram" || parsed.topic_mode || parsed.chat_id != user {
        return Err(invalid(
            "Choose a Telegram channel configured for this exact private user ID with topics off",
        ));
    }
    Ok(format!(
        "{:x}",
        Sha256::digest(format!(
            "{}:{}:{}",
            row.id, row.channel_type, row.config_json
        ))
    ))
}
async fn channel_current<C: ConnectionTrait>(db: &C, row: &config::Model) -> Result<bool, DbError> {
    Ok(chat_channel::Entity::find_by_id(row.channel_id)
        .one(db)
        .await?
        .and_then(|ch| channel_hash(&ch, &row.private_user_id).ok())
        .is_some_and(|hash| hash == row.channel_sha256))
}
async fn writer(db: &DatabaseConnection, account_id: i32) -> Result<DatabaseTransaction, DbError> {
    let txn = db.begin().await?;
    // Acquire SQLite writer ownership before reads, including first configure.
    txn.execute(Statement::from_sql_and_values(
        txn.get_database_backend(),
        "UPDATE ops_telegram_config SET updated_at = updated_at WHERE account_id = ?",
        [account_id.into()],
    ))
    .await?;
    Ok(txn)
}

pub async fn status(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &TelegramRuntime,
) -> Result<Status, DbError> {
    let row = config::Entity::find_by_id(op.account_id()).one(db).await?;
    let state = match &row {
        None => "not_configured",
        Some(row) if !row.enabled => "disabled",
        Some(row) if !channel_current(db, row).await? => "recipient_changed",
        Some(row) if runtime.secret(row.channel_id).await.is_none() => "missing_token",
        Some(_) => "ready",
    };
    let enabled = row.as_ref().is_some_and(|r| r.enabled);
    let channels = chat_channel::Entity::find()
        .filter(chat_channel::Column::ChannelType.eq("telegram"))
        .order_by_asc(chat_channel::Column::Id)
        .all(db)
        .await?
        .into_iter()
        .map(|ch| ChannelChoice {
            id: ch.id,
            name: ch.name,
        })
        .collect();
    let notices = notice::Entity::find()
        .filter(notice::Column::AccountId.eq(op.account_id()))
        .order_by_desc(notice::Column::CreatedAt)
        .limit(50)
        .all(db)
        .await?
        .into_iter()
        .map(|n| Notice {
            proposal_id: n.proposal_id,
            task_id: n.task_id,
            run_seq: n.run_seq,
            // A process may have died in checking/sending. No retry follows;
            // keep that uncertainty visible instead of pretending delivery.
            status: if n.status == "sending" || n.status == "checking" {
                "unknown".into()
            } else {
                n.status
            },
            updated_at: n.updated_at.to_rfc3339(),
        })
        .collect();
    Ok(Status {
        enabled,
        state,
        channels,
        notices,
        configuration: row.map(|r| Configuration {
            channel_id: r.channel_id,
            private_user_id: r.private_user_id,
            review_origin: r.review_origin,
            revision: r.revision,
        }),
    })
}
pub async fn configure(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &TelegramRuntime,
    input: ConfigureInput,
) -> Result<Status, DbError> {
    let review_origin = origin(&input.review_origin)?;
    let txn = writer(db, op.account_id()).await?;
    let old = config::Entity::find_by_id(op.account_id())
        .one(&txn)
        .await?;
    if old.as_ref().map(|r| &r.revision) != input.expected_revision.as_ref() {
        return Err(conflict());
    }
    let channel = chat_channel::Entity::find_by_id(input.channel_id)
        .one(&txn)
        .await?
        .ok_or_else(|| invalid("Choose an existing Telegram channel"))?;
    let hash = channel_hash(&channel, &input.private_user_id)?;
    let row = config::Model {
        account_id: op.account_id(),
        enabled: input.enabled,
        channel_id: input.channel_id,
        private_user_id: input.private_user_id,
        channel_sha256: hash,
        review_origin,
        revision: uuid::Uuid::new_v4().to_string(),
        updated_by: op.actor().into(),
        updated_at: Utc::now(),
    };
    let active = row.into_active_model().reset_all();
    if old.is_some() {
        active.update(&txn).await?;
    } else {
        active.insert(&txn).await?;
    }
    txn.commit().await?;
    // No provider calls, channel activation, credential writes, or polling.
    status(db, op, runtime).await
}
pub async fn disable(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &TelegramRuntime,
) -> Result<Status, DbError> {
    config::Entity::update_many()
        .col_expr(config::Column::Enabled, Expr::value(false))
        .col_expr(
            config::Column::Revision,
            Expr::value(uuid::Uuid::new_v4().to_string()),
        )
        .col_expr(config::Column::UpdatedBy, Expr::value(op.actor()))
        .col_expr(config::Column::UpdatedAt, Expr::value(Utc::now()))
        .filter(config::Column::AccountId.eq(op.account_id()))
        .exec(db)
        .await?;
    status(db, op, runtime).await
}

async fn claim(
    db: &DatabaseConnection,
    cfg: &config::Model,
    proposal_id: i32,
    claim_id: &str,
) -> Result<Option<notice::Model>, DbError> {
    let txn = writer(db, cfg.account_id).await?;
    if !config_matches(&txn, cfg).await? {
        return Ok(None);
    }
    let old = notice::Entity::find()
        .filter(notice::Column::ProposalId.eq(proposal_id))
        .one(&txn)
        .await?;
    if let Some(old) = &old {
        // Only pre-send recipient checks can be reclaimed. A paused old worker
        // must present its claim_id after the network await and cannot send on
        // a replacement lease. Never reclaim sending/unknown/failed/sent.
        if old.status != "preflight_failed"
            && !(old.status == "checking"
                && old.updated_at < Utc::now() - chrono::Duration::seconds(30))
        {
            return Ok(None);
        }
    }
    let Some(snapshot) = review::notice_snapshot(&txn, cfg.account_id, proposal_id).await? else {
        return Ok(None);
    };
    let now = Utc::now();
    let active = notice::ActiveModel {
        id: Set(old
            .as_ref()
            .map(|n| n.id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())),
        account_id: Set(cfg.account_id),
        proposal_id: Set(snapshot.proposal_id),
        task_id: Set(snapshot.task_id),
        run_seq: Set(snapshot.run_seq),
        action_kind: Set("email_reply".into()),
        snapshot_sha256: Set(snapshot.sha256),
        config_revision: Set(cfg.revision.clone()),
        channel_id: Set(cfg.channel_id),
        claim_id: Set(claim_id.into()),
        private_user_id: Set(cfg.private_user_id.clone()),
        channel_sha256: Set(cfg.channel_sha256.clone()),
        status: Set("checking".into()),
        provider_message_id: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };
    let row = if old.is_some() {
        active.update(&txn).await?
    } else {
        active.insert(&txn).await?
    };
    txn.commit().await?;
    Ok(Some(row))
}
async fn config_matches<C: ConnectionTrait>(db: &C, cfg: &config::Model) -> Result<bool, DbError> {
    let current = config::Entity::find_by_id(cfg.account_id).one(db).await?;
    Ok(
        current.is_some_and(|c| c.enabled && c.revision == cfg.revision)
            && channel_current(db, cfg).await?,
    )
}
async fn live<C: ConnectionTrait>(db: &C, row: &notice::Model) -> Result<bool, DbError> {
    let Some(cfg) = config::Entity::find_by_id(row.account_id).one(db).await? else {
        return Ok(false);
    };
    if !cfg.enabled
        || cfg.revision != row.config_revision
        || cfg.channel_id != row.channel_id
        || cfg.private_user_id != row.private_user_id
        || cfg.channel_sha256 != row.channel_sha256
        || !channel_current(db, &cfg).await?
    {
        return Ok(false);
    }
    Ok(review::notice_snapshot(db, row.account_id, row.proposal_id)
        .await?
        .is_some_and(|s| {
            s.task_id == row.task_id && s.run_seq == row.run_seq && s.sha256 == row.snapshot_sha256
        }))
}
async fn finish(
    db: &DatabaseConnection,
    row: &notice::Model,
    state: &str,
    receipt: Option<String>,
) -> Result<(), DbError> {
    notice::Entity::update_many()
        .col_expr(notice::Column::Status, Expr::value(state))
        .col_expr(notice::Column::ProviderMessageId, Expr::value(receipt))
        .col_expr(notice::Column::UpdatedAt, Expr::value(Utc::now()))
        .filter(notice::Column::Id.eq(&row.id))
        .filter(notice::Column::ClaimId.eq(&row.claim_id))
        .filter(notice::Column::Status.is_in(["checking", "sending"]))
        .exec(db)
        .await?;
    Ok(())
}
async fn dispatch(
    db: &DatabaseConnection,
    cfg: &config::Model,
    backend: &TelegramBackend,
    row: &notice::Model,
) -> Result<(), DbError> {
    if backend.verify_ops_private_recipient().await.is_err() {
        return finish(db, row, "preflight_failed", None).await;
    }
    // Recipient lookup awaits the network; repeat configuration and complete
    // snapshot checks under writer ownership afterwards, before dispatch.
    let txn = writer(db, row.account_id).await?;
    if !live(&txn, row).await? {
        txn.rollback().await?;
        return finish(db, row, "obsolete", None).await;
    }
    let changed = notice::Entity::update_many()
        .col_expr(notice::Column::Status, Expr::value("sending"))
        .col_expr(notice::Column::UpdatedAt, Expr::value(Utc::now()))
        .filter(notice::Column::Id.eq(&row.id))
        .filter(notice::Column::Status.eq("checking"))
        .filter(notice::Column::ClaimId.eq(&row.claim_id))
        .exec(&txn)
        .await?;
    txn.commit().await?;
    if changed.rows_affected != 1 {
        return Ok(());
    }
    let link = format!("{}/ops-review?notice={}", cfg.review_origin, row.id);
    let (state, receipt) = match backend.send_ops_review_link(&link).await {
        Ok(receipt) => ("sent", Some(receipt)),
        Err(OpsSendError::Rejected | OpsSendError::Unavailable) => ("failed", None),
        Err(OpsSendError::Unknown) => ("unknown", None),
    };
    finish(db, row, state, receipt).await
}
async fn scan_account(
    db: &DatabaseConnection,
    cfg: &config::Model,
    runtime: &TelegramRuntime,
    claim_id: &str,
) -> Result<(), DbError> {
    if !cfg.enabled || !channel_current(db, cfg).await? {
        return Ok(());
    }
    // Missing credentials must not consume any notification claim/proposal.
    let Some(backend) = runtime.backend(cfg).await else {
        return Ok(());
    };
    let candidates = ops_proposal::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT p.* FROM ops_proposal p
         JOIN ops_reply_draft d ON d.id = json_extract(p.payload_json, '$.draftId')
         JOIN work_task t ON t.id = p.task_id JOIN folder f ON f.id = t.folder_id
         LEFT JOIN ops_telegram_notice n ON n.proposal_id = p.id
         WHERE p.action_name = 'ops.email.reply' AND p.status = 'pending' AND d.account_id = ?
         AND d.revision = json_extract(p.payload_json, '$.draftRevision')
         AND t.run_seq = p.run_seq AND t.status = 'awaiting_input' AND t.deleted_at IS NULL
         AND f.deleted_at IS NULL AND (n.id IS NULL OR n.status = 'preflight_failed'
         OR (n.status = 'checking' AND n.updated_at < ?)) ORDER BY p.id ASC LIMIT 20",
            [
                cfg.account_id.into(),
                (Utc::now() - chrono::Duration::seconds(30)).into(),
            ],
        ))
        .all(db)
        .await?;
    for p in candidates {
        if let Some(row) = claim(db, cfg, p.id, claim_id).await? {
            dispatch(db, cfg, &backend, &row).await?;
        }
    }
    Ok(())
}
/// Called by the existing channel scheduler. With no explicit Ops opt-in this
/// performs only a local config query, even if legacy live channels are enabled.
pub(crate) async fn tick(
    db: &DatabaseConnection,
    runtime: &TelegramRuntime,
) -> Result<(), DbError> {
    bounded_scan(db, runtime, None, Duration::from_secs(8)).await
}
pub async fn notify_now(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &TelegramRuntime,
) -> Result<Status, DbError> {
    bounded_scan(db, runtime, Some(op.account_id()), Duration::from_secs(8)).await?;
    status(db, op, runtime).await
}

async fn bounded_scan(
    db: &DatabaseConnection,
    runtime: &TelegramRuntime,
    account: Option<i32>,
    budget: Duration,
) -> Result<(), DbError> {
    let claim_id = uuid::Uuid::new_v4().to_string();
    let result = tokio::time::timeout(budget, async {
        let mut query = config::Entity::find().filter(config::Column::Enabled.eq(true));
        if let Some(account) = account {
            query = query.filter(config::Column::AccountId.eq(account));
        }
        let configs = query
            .order_by_asc(config::Column::AccountId)
            .limit(32)
            .all(db)
            .await?;
        for cfg in configs {
            scan_account(db, &cfg, runtime, &claim_id).await?;
        }
        Ok::<_, DbError>(())
    })
    .await;
    if matches!(&result, Ok(Ok(()))) {
        return Ok(());
    }
    // Cancellation/error after POST began cannot imply failure or trigger a
    // retry. Settle only this scan's claims. Cleanup has its own one-second
    // bound so a blocked database cannot stall unrelated channel schedules.
    let cleanup = tokio::time::timeout(Duration::from_secs(1), async {
        for (before, after) in [("checking", "preflight_failed"), ("sending", "unknown")] {
            notice::Entity::update_many()
                .col_expr(notice::Column::Status, Expr::value(after))
                .col_expr(notice::Column::UpdatedAt, Expr::value(Utc::now()))
                .filter(notice::Column::ClaimId.eq(&claim_id))
                .filter(notice::Column::Status.eq(before))
                .exec(db)
                .await?;
        }
        Ok::<_, DbError>(())
    })
    .await;
    // If shutdown prevents cleanup, durable sending still maps to unconfirmed
    // and is never reclaimable. Only a pre-send checking lease may expire.
    if let Ok(Err(error)) = cleanup {
        return Err(error);
    }
    result.unwrap_or(Ok(()))
}
pub async fn resolve(
    db: &DatabaseConnection,
    op: &Operator,
    input: ResolveInput,
) -> Result<Resolution, DbError> {
    let unavailable = || Resolution {
        state: "unavailable",
        proposal: None,
    };
    if uuid::Uuid::parse_str(&input.notice)
        .ok()
        .is_none_or(|id| id.to_string() != input.notice)
    {
        return Ok(unavailable());
    }
    let Some(row) = notice::Entity::find_by_id(input.notice)
        .filter(notice::Column::AccountId.eq(op.account_id()))
        .one(db)
        .await?
    else {
        return Ok(unavailable());
    };
    if !matches!(row.status.as_str(), "sent" | "unknown" | "sending") || !live(db, &row).await? {
        return Ok(unavailable());
    }
    let proposal = review::get(
        db,
        op,
        crate::ops::types::ProposalInput {
            id: row.proposal_id,
        },
    )
    .await?;
    let displayed_hash = proposal
        .payload
        .as_ref()
        .and_then(|payload| serde_json::to_vec(payload).ok())
        .map(|bytes| format!("{:x}", Sha256::digest(bytes)));
    if proposal.stale
        || proposal.status != "pending"
        || proposal.task_id != row.task_id
        || proposal.run_seq != row.run_seq
        || displayed_hash.as_deref() != Some(&row.snapshot_sha256)
    {
        return Ok(unavailable());
    }
    Ok(Resolution {
        state: "ready",
        proposal: Some(proposal),
    })
}

#[cfg(test)]
pub(crate) mod tests;
