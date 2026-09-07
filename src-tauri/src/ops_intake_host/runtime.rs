//! Dedicated credential references; never select a Git PAT or inherit agent env.
use super::{process::Bridge, types::*};
use crate::ops_intake::github::{GithubAppClient, GithubAppConfig};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};
use tokio::sync::{Mutex as AsyncMutex, OwnedMutexGuard};

pub(super) trait Secrets: Send + Sync {
    fn get(&self, reference: &str) -> Option<String>;
    fn set(&self, reference: &str, value: &str) -> Result<(), HostError>;
    fn delete(&self, reference: &str) -> Result<(), HostError>;
}
struct ExistingStore;
impl Secrets for ExistingStore {
    fn get(&self, reference: &str) -> Option<String> {
        crate::keyring_store::get_token(reference)
    }
    fn set(&self, reference: &str, value: &str) -> Result<(), HostError> {
        crate::keyring_store::set_token(reference, value).map_err(|_| HostError::StorageUnavailable)
    }
    fn delete(&self, reference: &str) -> Result<(), HostError> {
        crate::keyring_store::delete_token(reference).map_err(|_| HostError::StorageUnavailable)
    }
}
#[derive(Default)]
pub(super) struct ProductRuntime {
    pub bridge: Option<Bridge>,
    pub github: Option<Arc<GithubAppClient>>,
    pub config: String,
}
pub struct HostRuntime {
    pub(super) secrets: Box<dyn Secrets>,
    products: Mutex<HashMap<String, Arc<AsyncMutex<ProductRuntime>>>>,
    #[cfg(any(test, feature = "test-utils"))]
    pub(super) github_fixture: Option<std::net::SocketAddr>,
}
impl HostRuntime {
    pub fn production() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<HostRuntime>> = OnceLock::new();
        INSTANCE
            .get_or_init(|| {
                Arc::new(Self {
                    secrets: Box::new(ExistingStore),
                    products: Mutex::new(HashMap::new()),
                    #[cfg(any(test, feature = "test-utils"))]
                    github_fixture: None,
                })
            })
            .clone()
    }
    pub(super) async fn guard(
        &self,
        product: &str,
    ) -> Result<OwnedMutexGuard<ProductRuntime>, HostError> {
        let lock = {
            let mut products = self
                .products
                .lock()
                .map_err(|_| HostError::StorageUnavailable)?;
            if !products.contains_key(product) && products.len() >= 32 {
                return Err(HostError::NotConfigured);
            }
            products.entry(product.into()).or_default().clone()
        };
        // Do not queue duplicate operator sends behind an in-flight request.
        lock.try_lock_owned().map_err(|_| HostError::Conflict)
    }
    pub(super) fn credential(&self, reference: &Option<String>) -> Result<String, HostError> {
        reference
            .as_deref()
            .and_then(|r| self.secrets.get(r))
            .filter(|s| !s.is_empty())
            .ok_or(HostError::NotConfigured)
    }
    pub(super) fn client(
        &self,
        state: &mut ProductRuntime,
        p: &StoredProduct,
    ) -> Result<Arc<GithubAppClient>, HostError> {
        state.match_config(p)?;
        if let Some(client) = &state.github {
            return Ok(client.clone());
        }
        let config = Some(GithubAppConfig {
            app_id: p.binding.app_id.clone(),
            private_key_pem: self.credential(&p.key_ref)?,
        });
        #[cfg(any(test, feature = "test-utils"))]
        let client = if let Some(address) = self.github_fixture {
            GithubAppClient::loopback_fixture(config, address)?
        } else {
            GithubAppClient::new(config)?
        };
        #[cfg(not(any(test, feature = "test-utils")))]
        let client = GithubAppClient::new(config)?;
        let client = Arc::new(client);
        state.github = Some(client.clone());
        Ok(client)
    }
}
impl ProductRuntime {
    pub fn match_config(&mut self, config: &StoredProduct) -> Result<(), HostError> {
        let tag = serde_json::to_string(config).map_err(|_| HostError::StorageUnavailable)?;
        if self.config != tag {
            self.bridge = None;
            self.github = None;
            self.config = tag;
        }
        Ok(())
    }
    pub async fn read<T: serde::de::DeserializeOwned>(
        &mut self,
        runtime: &HostRuntime,
        p: &StoredProduct,
        operation: &str,
        input: serde_json::Value,
    ) -> Result<T, HostError> {
        self.match_config(p)?;
        // Take ownership before await: cancellation drops/kills the child and
        // never leaves partial protocol bytes in the next request's session.
        let mut child = match self.bridge.take() {
            Some(child) => child,
            None => Bridge::start(
                &p.origin,
                &p.binding.product_id,
                &runtime.credential(&p.bearer_ref)?,
            )?,
        };
        let result = child.read(operation, input).await;
        if result.is_ok() {
            self.bridge = Some(child);
        }
        result
    }
}
