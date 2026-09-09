//! Typed native platform/settings/session glue. No arbitrary command dispatcher.
#![cfg(feature = "tauri-runtime")]
use crate::{
    app_error::AppCommandError,
    business_identity::{
        self as identity, native::NativeSessions, native_acl, platform, settings,
        types::Organization, IdentityError as E,
    },
    db::AppDatabase,
};
use serde::{Deserialize, Serialize};
use tauri::{Runtime, WebviewWindow};

pub(crate) fn platform_context<R: Runtime>(
    window: &WebviewWindow<R>,
) -> Result<platform::PlatformContext, AppCommandError> {
    if !native_acl::is_platform(window.label()) {
        return Err(E::Forbidden.command_error());
    }
    Ok(platform::PlatformContext::from_operator(
        &crate::web::auth::AuthenticatedOperator,
    ))
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowContext {
    pub platform_operator: bool,
    pub restricted_tenant_window: bool,
    pub tenant_window_available: bool,
}
#[tauri::command]
pub fn business_window_context<R: Runtime>(window: WebviewWindow<R>) -> WindowContext {
    WindowContext {
        platform_operator: native_acl::is_platform(window.label()),
        restricted_tenant_window: native_acl::is_tenant(window.label()),
        tenant_window_available: false,
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Control {
    Close,
    Minimize,
    ToggleMaximize,
    IsMaximized,
    StartDragging,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlInput {
    pub action: Control,
}
#[tauri::command]
pub fn business_window_control<R: Runtime>(
    window: WebviewWindow<R>,
    input: ControlInput,
) -> Result<bool, AppCommandError> {
    if !native_acl::is_tenant(window.label()) && !native_acl::is_platform(window.label()) {
        return Err(E::Forbidden.command_error());
    }
    let result = match input.action {
        Control::Close => window.close(),
        Control::Minimize => window.minimize(),
        Control::ToggleMaximize => window.is_maximized().and_then(|maximized| {
            if maximized {
                window.unmaximize()
            } else {
                window.maximize()
            }
        }),
        Control::StartDragging => window.start_dragging(),
        Control::IsMaximized => {
            return window
                .is_maximized()
                .map_err(|_| E::Forbidden.command_error())
        }
    };
    result
        .map(|()| false)
        .map_err(|_| E::Forbidden.command_error())
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginInput {
    pub token: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogoutInput {
    pub session: String,
}
#[tauri::command]
pub async fn business_session_open<R: Runtime>(
    window: WebviewWindow<R>,
    sessions: tauri::State<'_, NativeSessions>,
    db: tauri::State<'_, AppDatabase>,
    input: LoginInput,
) -> Result<identity::native::NativeSessionView, AppCommandError> {
    sessions
        .open(&db.conn, window.label(), &input.token)
        .await
        .map_err(E::command_error)
}
#[tauri::command]
pub fn business_session_close<R: Runtime>(
    window: WebviewWindow<R>,
    sessions: tauri::State<'_, NativeSessions>,
    input: LogoutInput,
) -> Result<(), AppCommandError> {
    sessions
        .close(window.label(), &input.session)
        .map_err(E::command_error)
}
#[tauri::command]
pub async fn business_settings_get<R: Runtime>(
    window: WebviewWindow<R>,
    sessions: tauri::State<'_, NativeSessions>,
    db: tauri::State<'_, AppDatabase>,
    session: Option<String>,
) -> Result<settings::SettingsView, AppCommandError> {
    let principal = sessions
        .principal(&db.conn, window.label(), session.as_deref())
        .await
        .map_err(E::command_error)?;
    settings::get(&db.conn, &principal)
        .await
        .map_err(E::command_error)
}
#[tauri::command]
pub async fn business_settings_update<R: Runtime>(
    window: WebviewWindow<R>,
    sessions: tauri::State<'_, NativeSessions>,
    db: tauri::State<'_, AppDatabase>,
    session: Option<String>,
    input: settings::UpdateSettingsInput,
) -> Result<settings::SettingsView, AppCommandError> {
    let principal = sessions
        .principal(&db.conn, window.label(), session.as_deref())
        .await
        .map_err(E::command_error)?;
    settings::update(&db.conn, &principal, input)
        .await
        .map_err(E::command_error)
}
#[tauri::command]
pub fn business_platform_context<R: Runtime>(
    window: WebviewWindow<R>,
) -> Result<serde_json::Value, AppCommandError> {
    platform_context(&window)?;
    Ok(serde_json::json!({"platformOperator":true}))
}
#[tauri::command]
pub async fn business_platform_tenants_list<R: Runtime>(
    window: WebviewWindow<R>,
    db: tauri::State<'_, AppDatabase>,
) -> Result<Vec<platform::TenantSummary>, AppCommandError> {
    platform::list(&db.conn, &platform_context(&window)?)
        .await
        .map_err(E::command_error)
}
macro_rules! command {
    ($name:ident,$core:ident,$input:ty,$result:ty) => {
        #[tauri::command]
        pub async fn $name<R: Runtime>(
            window: WebviewWindow<R>,
            db: tauri::State<'_, AppDatabase>,
            input: $input,
        ) -> Result<$result, AppCommandError> {
            platform::$core(&db.conn, &platform_context(&window)?, input)
                .await
                .map_err(E::command_error)
        }
    };
}
command!(
    business_platform_tenants_create,
    create,
    platform::CreateTenantInput,
    platform::ProvisionResult
);
command!(
    business_platform_tenants_status,
    status,
    platform::StatusInput,
    Organization
);
command!(
    business_platform_tenants_reissue_owner_credential,
    reissue,
    platform::ReissueInput,
    platform::ProvisionResult
);
