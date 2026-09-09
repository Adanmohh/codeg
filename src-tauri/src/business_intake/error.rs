use crate::{
    app_error::{AppCommandError, AppErrorCode},
    business_identity::IdentityError,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reason {
    BindingMissing,
    BindingDisabled,
    CredentialUnavailable,
    BindingUnavailable,
    SourceExpired,
    RebaseRequired,
    ImportBusy,
    RetryLater,
    SourceDenied,
    PublicationNotAllowed,
    UnsupportedSchema,
    ProviderUnavailable,
    RequestTimeout,
}
impl Reason {
    pub(super) fn key(self) -> &'static str {
        match self {
            Self::BindingMissing => "binding_missing",
            Self::BindingDisabled => "binding_disabled",
            Self::CredentialUnavailable => "credential_unavailable",
            Self::BindingUnavailable => "binding_unavailable",
            Self::SourceExpired => "source_expired",
            Self::RebaseRequired => "rebase_required",
            Self::ImportBusy => "import_busy",
            Self::RetryLater => "retry_later",
            Self::SourceDenied => "source_denied",
            Self::PublicationNotAllowed => "publication_not_allowed",
            Self::UnsupportedSchema => "unsupported_schema",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::RequestTimeout => "request_timeout",
        }
    }
    fn command_error(self) -> AppCommandError {
        let code = match self {
            Self::BindingMissing | Self::BindingDisabled | Self::CredentialUnavailable => {
                AppErrorCode::ConfigurationMissing
            }
            Self::BindingUnavailable => AppErrorCode::ConfigurationInvalid,
            Self::SourceExpired | Self::RebaseRequired | Self::ImportBusy | Self::RetryLater => {
                AppErrorCode::AlreadyExists
            }
            Self::SourceDenied | Self::PublicationNotAllowed => AppErrorCode::PermissionDenied,
            Self::UnsupportedSchema | Self::ProviderUnavailable | Self::RequestTimeout => {
                AppErrorCode::NetworkError
            }
        };
        AppCommandError::new(code, "Source operation unavailable").with_i18n(
            format!("business.intake.{}", self.key()),
            Default::default(),
        )
    }
}
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Business intake authorization or storage failed")]
    Identity(#[from] IdentityError),
    #[error("Source operation unavailable")]
    Intake(Reason),
}
impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::Identity(IdentityError::Database(value))
    }
}
impl From<Reason> for Error {
    fn from(value: Reason) -> Self {
        Self::Intake(value)
    }
}
impl Error {
    pub fn command_error(self) -> AppCommandError {
        match self {
            Self::Identity(error) => error.command_error(),
            Self::Intake(reason) => reason.command_error(),
        }
    }
}
pub(super) fn invalid() -> Error {
    IdentityError::Invalid("Invalid source input").into()
}
pub(super) fn conflict() -> Error {
    IdentityError::Conflict.into()
}
pub(super) fn missing() -> Error {
    IdentityError::NotFound.into()
}
