//! Native sessions have actual invoking-window attribution, never ambient owner
//! fallback in a tenant window. Production tenant window creation is unavailable
//! until the separately reported Tauri shared channel queue is fully isolated.
use super::{native_acl, store, types::Context, IdentityError as E, Principal};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

#[derive(Default)]
pub struct NativeSessions(Mutex<HashMap<String, Session>>);
struct Session {
    id: String,
    principal: Option<Principal>,
    active: Arc<AtomicBool>,
}
impl NativeSessions {
    fn clear_entry(entries: &mut HashMap<String, Session>, window: &str) {
        if let Some(entry) = entries.remove(window) {
            entry.active.store(false, Ordering::SeqCst);
        }
    }
    pub fn clear_window(&self, window: &str) {
        Self::clear_entry(
            &mut self.0.lock().unwrap_or_else(|p| p.into_inner()),
            window,
        );
    }
    pub async fn open(
        &self,
        conn: &DatabaseConnection,
        window: &str,
        token: &str,
    ) -> Result<NativeSessionView, E> {
        if !native_acl::is_tenant(window) {
            return Err(E::Forbidden);
        }
        let id = store::id();
        let active = Arc::new(AtomicBool::new(true));
        {
            let mut entries = self.0.lock().map_err(|_| E::Unauthorized)?;
            Self::clear_entry(&mut entries, window);
            entries.insert(
                window.into(),
                Session {
                    id: id.clone(),
                    principal: None,
                    active: active.clone(),
                },
            );
        }
        // Only canonical bdm_ credentials resolve; CODEG_TOKEN is never tried.
        let result = async {
            let mut principal = store::resolve_credential(conn, token).await?;
            principal.native_session_active = Some(active.clone());
            let context = store::context(conn, &principal).await?;
            let mut entries = self.0.lock().map_err(|_| E::Unauthorized)?;
            let entry = entries
                .get_mut(window)
                .filter(|s| s.id == id && s.active.load(Ordering::SeqCst))
                .ok_or(E::Unauthorized)?;
            entry.principal = Some(principal);
            Ok(NativeSessionView {
                session: id.clone(),
                context,
            })
        }
        .await;
        if result.is_err() {
            self.close(window, &id)?;
        }
        result
    }
    pub fn close(&self, window: &str, id: &str) -> Result<(), E> {
        let mut entries = self.0.lock().map_err(|_| E::Unauthorized)?;
        if entries.get(window).is_some_and(|s| s.id == id) {
            Self::clear_entry(&mut entries, window);
        }
        Ok(())
    }
    pub async fn principal(
        &self,
        conn: &DatabaseConnection,
        window: &str,
        id: Option<&str>,
    ) -> Result<Principal, E> {
        if native_acl::is_platform(window) {
            if id.is_some() {
                return Err(E::Forbidden);
            }
            return super::operator_principal(conn).await;
        }
        if !native_acl::is_tenant(window) {
            return Err(E::Forbidden);
        }
        let principal = {
            let entries = self.0.lock().map_err(|_| E::Unauthorized)?;
            entries
                .get(window)
                .filter(|s| Some(s.id.as_str()) == id && s.active.load(Ordering::SeqCst))
                .and_then(|s| s.principal.clone())
                .ok_or(E::Unauthorized)?
        };
        super::authorize(
            conn,
            &principal,
            principal.organization_id(),
            super::Permission::Read,
            None,
        )
        .await?;
        Ok(principal)
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeSessionView {
    pub session: String,
    pub context: Context,
}
