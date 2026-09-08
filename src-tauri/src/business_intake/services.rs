//! Dependency injection only; no scheduler, second credential store or runtime.
use super::{common::Result, error::Reason, fireflies::Reader};
use crate::ops::email::{ExistingStore, SecretStore};
use std::sync::{Arc, OnceLock};

pub(crate) struct Services {
    pub(super) reader: Reader,
    secrets: Arc<dyn SecretStore>,
}
impl Services {
    pub(crate) fn production() -> Result<Arc<Self>> {
        static INSTANCE: OnceLock<std::result::Result<Arc<Services>, Reason>> = OnceLock::new();
        INSTANCE
            .get_or_init(|| {
                Ok(Arc::new(Self {
                    reader: Reader::production().map_err(|e| e.reason)?,
                    secrets: Arc::new(ExistingStore),
                }))
            })
            .clone()
            .map_err(Into::into)
    }
    #[cfg(test)]
    pub(super) fn fixture(reader: Reader, secrets: Arc<dyn SecretStore>) -> Arc<Self> {
        Arc::new(Self { reader, secrets })
    }
    pub(super) async fn get(&self, reference: &str) -> Result<Option<String>> {
        let secrets = self.secrets.clone();
        let reference = reference.to_owned();
        tokio::task::spawn_blocking(move || secrets.get(&reference))
            .await
            .map_err(|_| Reason::CredentialUnavailable.into())
    }
    pub(super) async fn set(&self, reference: &str, value: String) -> Result<()> {
        let secrets = self.secrets.clone();
        let reference = reference.to_owned();
        tokio::task::spawn_blocking(move || secrets.set(&reference, &value))
            .await
            .map_err(|_| Reason::CredentialUnavailable)?
            .map_err(|_| Reason::CredentialUnavailable.into())
    }
    pub(super) async fn delete(&self, reference: &str) -> Result<()> {
        let secrets = self.secrets.clone();
        let reference = reference.to_owned();
        tokio::task::spawn_blocking(move || secrets.delete(&reference))
            .await
            .map_err(|_| Reason::CredentialUnavailable)?
            .map_err(|_| Reason::CredentialUnavailable.into())
    }
}
