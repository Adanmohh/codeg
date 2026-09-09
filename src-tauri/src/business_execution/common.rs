//! Codeg/IntroMail-derived transaction and safe-error glue; see NOTICE.
use super::types::OperationReason;
use crate::business_identity::IdentityError;
use sea_orm::{DbBackend, Statement, Value};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("Business execution operation unavailable")]
pub struct Error(pub OperationReason);
impl From<OperationReason> for Error {
    fn from(reason: OperationReason) -> Self {
        Self(reason)
    }
}
impl From<sea_orm::DbErr> for Error {
    fn from(_: sea_orm::DbErr) -> Self {
        Self(OperationReason::Unavailable)
    }
}
impl From<IdentityError> for Error {
    fn from(error: IdentityError) -> Self {
        Self(match error {
            IdentityError::Unauthorized => OperationReason::Unauthorized,
            IdentityError::Forbidden => OperationReason::Forbidden,
            IdentityError::NotFound => OperationReason::Missing,
            IdentityError::Conflict => OperationReason::Conflict,
            IdentityError::Invalid(_) => OperationReason::Invalid,
            IdentityError::BootstrapRequired => OperationReason::SetupRequired,
            IdentityError::Database(_) | IdentityError::Random => OperationReason::Unavailable,
        })
    }
}

pub(super) fn statement(sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values)
}
pub(super) fn id() -> String {
    uuid::Uuid::new_v4().to_string()
}
pub(super) fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}
pub(super) fn encode<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|_| OperationReason::Invalid.into())
}
pub(super) fn decode<T: DeserializeOwned>(value: &str) -> Result<T> {
    serde_json::from_str(value).map_err(|_| OperationReason::Unavailable.into())
}
pub(super) fn key<T: Serialize>(value: T) -> Result<String> {
    serde_json::to_value(value)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .ok_or_else(|| OperationReason::Unavailable.into())
}
pub(super) fn digest(value: &[u8]) -> String {
    format!("{:x}", Sha256::digest(value))
}
