//! Operator-only Ops glue over the existing ticket and approval services.
//! Borrowed Codeg transport/handler patterns are mapped in NOTICE.
pub mod agent;
pub mod delivery;
mod draft_entity;
pub mod email;
mod email_entity;
pub mod review;
pub mod store;
pub mod types;

use crate::app_error::{AppCommandError, AppErrorCode};
use crate::db::error::DbError;

/// A transport-established human boundary, not an actor supplied in JSON and
/// not an OS sandbox against another process running as the same local user.
#[derive(Clone)]
pub struct Operator {
    account_id: i32,
    actor: &'static str,
}

impl Operator {
    pub fn account_id(&self) -> i32 {
        self.account_id
    }
    pub fn actor(&self) -> &'static str {
        self.actor
    }
    pub(crate) fn server() -> Result<Self, AppCommandError> {
        Self::configured("operator:http")
    }

    #[cfg(feature = "tauri-runtime")]
    pub(crate) fn desktop() -> Result<Self, AppCommandError> {
        Self::configured("operator:desktop")
    }

    fn configured(actor: &'static str) -> Result<Self, AppCommandError> {
        let account_id = configured_account_id()?;
        Ok(Self { account_id, actor })
    }

    fn scope(&self, inbox_id: i32) -> crate::db::service::ticket_service::Scope {
        crate::db::service::ticket_service::Scope {
            account_id: self.account_id,
            inbox_id,
        }
    }
}

fn configured_account_id() -> Result<i32, AppCommandError> {
    let raw = std::env::var_os("CODEG_OPS_ACCOUNT_ID");
    account_id_from(raw.as_deref())
}
fn account_id_from(raw: Option<&std::ffi::OsStr>) -> Result<i32, AppCommandError> {
    match raw {
        None => Ok(1),
        Some(value) => value
            .to_str()
            .and_then(|s| s.parse::<i32>().ok())
            .filter(|id| *id > 0)
            .ok_or_else(|| {
                AppCommandError::configuration_invalid(
                    "Ops account configuration must be a positive integer",
                )
            }),
    }
}

pub(crate) fn command_error(err: DbError) -> AppCommandError {
    match err {
        DbError::NotFound(_) => {
            AppCommandError::new(AppErrorCode::NotFound, "Item is unavailable in this inbox")
        }
        DbError::Conflict(_) => AppCommandError::new(
            AppErrorCode::TurnInProgress,
            "This item changed. Reload it before continuing",
        ),
        DbError::Validation(message) => AppCommandError::invalid_input(message),
        _ => AppCommandError::new(
            AppErrorCode::DatabaseError,
            "Ops storage is unavailable. Your change was not confirmed",
        ),
    }
}

#[cfg(test)]
pub(crate) mod tests;
