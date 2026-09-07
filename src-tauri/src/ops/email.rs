//! Per-inbox configuration and bounded manual pull. No scheduler or agent route.
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock, Weak},
    time::Duration,
};
use tokio::sync::{Mutex as AsyncMutex, OwnedMutexGuard};

use super::{command_error, email_entity::config, types::*, Operator};
use crate::{
    app_error::{AppCommandError, AppErrorCode},
    db::service::ticket_service::{self as tickets, Scope},
    email_transport::{self, PullOptions, ResendClient},
};

pub(crate) trait SecretStore: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str) -> Result<(), ()>;
    fn delete(&self, key: &str) -> Result<(), ()>;
}
struct ExistingStore;
impl SecretStore for ExistingStore {
    fn get(&self, key: &str) -> Option<String> {
        crate::keyring_store::get_token(key)
    }
    fn set(&self, key: &str, value: &str) -> Result<(), ()> {
        crate::keyring_store::set_token(key, value).map_err(|_| ())
    }
    fn delete(&self, key: &str) -> Result<(), ()> {
        crate::keyring_store::delete_token(key).map_err(|_| ())
    }
}

type InboxLocks = Mutex<HashMap<(i32, i32), Weak<AsyncMutex<()>>>>;
pub struct EmailRuntime {
    secrets: Box<dyn SecretStore>,
    locks: InboxLocks,
    #[cfg(test)]
    mock: Option<std::net::SocketAddr>,
}
impl EmailRuntime {
    pub(crate) fn production() -> Arc<Self> {
        static RUNTIME: OnceLock<Arc<EmailRuntime>> = OnceLock::new();
        RUNTIME
            .get_or_init(|| {
                Arc::new(Self {
                    secrets: Box::new(ExistingStore),
                    locks: Mutex::new(HashMap::new()),
                    #[cfg(test)]
                    mock: None,
                })
            })
            .clone()
    }
    #[cfg(test)]
    pub(crate) fn fixture(secrets: Box<dyn SecretStore>, mock: std::net::SocketAddr) -> Arc<Self> {
        Arc::new(Self {
            secrets,
            locks: Mutex::new(HashMap::new()),
            mock: Some(mock),
        })
    }
    pub(super) fn guard(&self, scope: Scope) -> Result<OwnedMutexGuard<()>, AppCommandError> {
        let mut locks = self.locks.lock().map_err(|_| {
            AppCommandError::new(
                AppErrorCode::TaskExecutionFailed,
                "Inbox operation unavailable",
            )
        })?;
        locks.retain(|_, lock| lock.strong_count() > 0);
        let lock = locks.entry((scope.account_id, scope.inbox_id)).or_default();
        let mutex = lock.upgrade().unwrap_or_else(|| {
            let mutex = Arc::new(AsyncMutex::new(()));
            *lock = Arc::downgrade(&mutex);
            mutex
        });
        mutex.try_lock_owned().map_err(|_| {
            AppCommandError::new(
                AppErrorCode::TurnInProgress,
                "This inbox has an operation in progress. Wait and refresh its status",
            )
        })
    }
    pub(super) async fn client(
        &self,
        db: &DatabaseConnection,
        op: &Operator,
        inbox_id: i32,
    ) -> Result<ResendClient, AppCommandError> {
        let row = get_config(db, op, inbox_id).await?.ok_or_else(missing)?;
        let key = self.secrets.get(&row.credential_ref).ok_or_else(missing)?;
        #[cfg(test)]
        if let Some(address) = self.mock {
            // The credential is a synthetic fixture; the accepted client's only
            // endpoint override is compile-time test-only and loopback checked.
            drop(key);
            return ResendClient::local_mock(
                db,
                op.scope(inbox_id),
                address,
                Duration::from_millis(1500),
            )
            .await
            .map_err(transport_error);
        }
        ResendClient::new(db, op.scope(inbox_id), &key)
            .await
            .map_err(transport_error)
    }
}

fn missing() -> AppCommandError {
    AppCommandError::configuration_missing(
        "Connect a Resend key for this inbox first. The proposal remains pending; nothing was sent",
    )
}
fn secret_error() -> AppCommandError {
    AppCommandError::new(
        AppErrorCode::ConfigurationInvalid,
        "The existing credential store could not save this inbox key",
    )
}
pub(super) fn transport_error(error: email_transport::Error) -> AppCommandError {
    // The accepted client emits only fixed, redacted errors. No provider body,
    // headers, key, raw metadata or credential-wide counts reach the operator UI.
    AppCommandError::new(AppErrorCode::NetworkError, error.to_string())
}
async fn get_config(
    db: &DatabaseConnection,
    op: &Operator,
    inbox_id: i32,
) -> Result<Option<config::Model>, AppCommandError> {
    tickets::require_inbox(db, op.scope(inbox_id))
        .await
        .map_err(command_error)?;
    config::Entity::find_by_id(inbox_id)
        .filter(config::Column::AccountId.eq(op.account_id))
        .one(db)
        .await
        .map_err(|e| command_error(e.into()))
}
pub async fn status(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &EmailRuntime,
    input: EmailInboxInput,
) -> Result<EmailStatus, AppCommandError> {
    let row = get_config(db, op, input.inbox_id).await?;
    Ok(EmailStatus {
        inbox_id: input.inbox_id,
        configured: row
            .as_ref()
            .is_some_and(|r| runtime.secrets.get(&r.credential_ref).is_some()),
        last_pull_at: row
            .as_ref()
            .and_then(|r| r.last_pull_at.map(|t| t.to_rfc3339())),
        last_pull_status: row.as_ref().and_then(|r| r.last_pull_status.clone()),
        last_pull_error: row.and_then(|r| r.last_pull_error),
    })
}
pub async fn configure(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &EmailRuntime,
    input: EmailConfigureInput,
) -> Result<EmailStatus, AppCommandError> {
    let _guard = runtime.guard(op.scope(input.inbox_id))?;
    if input.api_key.is_empty()
        || input.api_key.len() > 4096
        || !input.api_key.bytes().all(|b| b.is_ascii_graphic())
    {
        return Err(AppCommandError::invalid_input(
            "Enter a valid Resend API key",
        ));
    }
    let row = match get_config(db, op, input.inbox_id).await? {
        Some(row) => row,
        None => config::ActiveModel {
            inbox_id: Set(input.inbox_id),
            account_id: Set(op.account_id),
            // Random per-database reference avoids OS-keyring collisions between
            // local workspaces whose integer inbox IDs happen to match.
            credential_ref: Set(format!("ops-resend:{}", uuid::Uuid::new_v4())),
            last_pull_at: Set(None),
            last_pull_status: Set(None),
            last_pull_error: Set(None),
        }
        .insert(db)
        .await
        .map_err(|e| command_error(e.into()))?,
    };
    runtime
        .secrets
        .set(&row.credential_ref, &input.api_key)
        .map_err(|_| secret_error())?;
    status(
        db,
        op,
        runtime,
        EmailInboxInput {
            inbox_id: input.inbox_id,
        },
    )
    .await
}
pub async fn disconnect(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &EmailRuntime,
    input: EmailInboxInput,
) -> Result<EmailStatus, AppCommandError> {
    let _guard = runtime.guard(op.scope(input.inbox_id))?;
    if let Some(row) = get_config(db, op, input.inbox_id).await? {
        runtime
            .secrets
            .delete(&row.credential_ref)
            .map_err(|_| secret_error())?;
    }
    status(db, op, runtime, input).await
}
pub async fn pull(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &EmailRuntime,
    input: EmailInboxInput,
) -> Result<PullResult, AppCommandError> {
    let _guard = runtime.guard(op.scope(input.inbox_id))?;
    let client = runtime.client(db, op, input.inbox_id).await?;
    let row = get_config(db, op, input.inbox_id)
        .await?
        .ok_or_else(missing)?;
    let mut model = row.into_active_model();
    model.last_pull_status = Set(Some("pulling".into()));
    model.last_pull_error = Set(None);
    let row = model
        .update(db)
        .await
        .map_err(|e| command_error(e.into()))?;
    // Whole manual pass deadline, in addition to the client's request and memory
    // bounds. Cancellation is safe: ingestion is per-message and replay dedups.
    let result = tokio::time::timeout(
        Duration::from_secs(55),
        client.pull_received(
            db,
            PullOptions {
                page_size: 100,
                max_pages: 5,
            },
        ),
    )
    .await
    .unwrap_or(Err(email_transport::Error::Transport { timeout: true }));
    let mut model = row.into_active_model();
    model.last_pull_at = Set(Some(Utc::now()));
    model.last_pull_status = Set(Some(
        if result.is_ok() { "complete" } else { "failed" }.into(),
    ));
    model.last_pull_error = Set(result.as_ref().err().map(ToString::to_string));
    model
        .update(db)
        .await
        .map_err(|e| command_error(e.into()))?;
    let summary = result.map_err(transport_error)?;
    Ok(PullResult {
        inserted: summary.inserted,
        duplicates: summary.duplicates,
    })
}
