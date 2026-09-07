//! Tauri's human boundary over the same typed service as protected HTTP.
#![cfg(feature = "tauri-runtime")]
use crate::{
    app_error::AppCommandError,
    db::AppDatabase,
    ops::{command_error, Operator},
    ops_telegram::{self as core, types::*, TelegramRuntime},
};
macro_rules! command {
    ($name:ident, $core:path) => {
        #[tauri::command]
        pub async fn $name(db: tauri::State<'_, AppDatabase>) -> Result<Status, AppCommandError> {
            $core(
                &db.conn,
                &Operator::desktop()?,
                &TelegramRuntime::production(),
            )
            .await
            .map_err(command_error)
        }
    };
}
command!(ops_telegram_status, core::status);
command!(ops_telegram_disable, core::disable);
command!(ops_telegram_notify, core::notify_now);
#[tauri::command]
pub async fn ops_telegram_configure(
    db: tauri::State<'_, AppDatabase>,
    input: ConfigureInput,
) -> Result<Status, AppCommandError> {
    core::configure(
        &db.conn,
        &Operator::desktop()?,
        &TelegramRuntime::production(),
        input,
    )
    .await
    .map_err(command_error)
}
#[tauri::command]
pub async fn ops_telegram_resolve(
    db: tauri::State<'_, AppDatabase>,
    input: ResolveInput,
) -> Result<Resolution, AppCommandError> {
    core::resolve(&db.conn, &Operator::desktop()?, input)
        .await
        .map_err(command_error)
}
