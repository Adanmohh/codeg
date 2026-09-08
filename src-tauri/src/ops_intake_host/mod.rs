//! Trusted operator host over accepted intake/approval packs. No generic agent API.
pub mod agent;
pub(crate) mod business;
pub mod fix_task;
pub mod operator;
pub(crate) mod notice;
mod process;
pub mod review;
mod runtime;
mod store;
pub mod types;
pub use runtime::HostRuntime;

pub(crate) fn command_error(error: types::HostError) -> crate::app_error::AppCommandError {
    use crate::app_error::{AppCommandError, AppErrorCode};
    use types::HostError as E;
    let code = match error {
        E::NotConfigured | E::AdapterMissing => AppErrorCode::ConfigurationMissing,
        E::AccessDenied => AppErrorCode::PermissionDenied,
        E::Conflict | E::StaleSource => AppErrorCode::TurnInProgress,
        E::SourceUnavailable => AppErrorCode::NetworkError,
        E::StorageUnavailable => AppErrorCode::DatabaseError,
        _ => AppErrorCode::InvalidInput,
    };
    AppCommandError::new(code, error.to_string())
}

#[cfg(test)]
pub(crate) mod tests;
