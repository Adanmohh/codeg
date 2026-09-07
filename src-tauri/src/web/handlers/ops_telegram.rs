//! Existing authenticated Codeg command boundary. No Telegram webhook/callback.
use crate::{
    app_error::AppCommandError,
    app_state::AppState,
    ops::{command_error, Operator},
    ops_telegram::{self as core, types::*, TelegramRuntime},
    web::auth::AuthenticatedOperator,
};
use axum::{routing::post, Extension, Json, Router};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Empty {}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input<T> {
    input: T,
}
fn runtime(value: Option<Extension<Arc<TelegramRuntime>>>) -> Arc<TelegramRuntime> {
    value
        .map(|Extension(r)| r)
        .unwrap_or_else(TelegramRuntime::production)
}
macro_rules! command {
    ($name:ident, $core:path) => {
        async fn $name(
            Extension(state): Extension<Arc<AppState>>,
            Extension(_human): Extension<AuthenticatedOperator>,
            injected: Option<Extension<Arc<TelegramRuntime>>>,
            Json(_): Json<Empty>,
        ) -> Result<Json<Status>, AppCommandError> {
            Ok(Json(
                $core(&state.db.conn, &Operator::server()?, &runtime(injected))
                    .await
                    .map_err(command_error)?,
            ))
        }
    };
}
command!(ops_telegram_status, core::status);
command!(ops_telegram_disable, core::disable);
command!(ops_telegram_notify, core::notify_now);
async fn ops_telegram_configure(
    Extension(state): Extension<Arc<AppState>>,
    Extension(_human): Extension<AuthenticatedOperator>,
    injected: Option<Extension<Arc<TelegramRuntime>>>,
    Json(params): Json<Input<ConfigureInput>>,
) -> Result<Json<Status>, AppCommandError> {
    Ok(Json(
        core::configure(
            &state.db.conn,
            &Operator::server()?,
            &runtime(injected),
            params.input,
        )
        .await
        .map_err(command_error)?,
    ))
}
async fn ops_telegram_resolve(
    Extension(state): Extension<Arc<AppState>>,
    Extension(_human): Extension<AuthenticatedOperator>,
    Json(params): Json<Input<ResolveInput>>,
) -> Result<Json<Resolution>, AppCommandError> {
    Ok(Json(
        core::resolve(&state.db.conn, &Operator::server()?, params.input)
            .await
            .map_err(command_error)?,
    ))
}
pub fn router() -> Router {
    Router::new()
        .route("/ops_telegram_status", post(ops_telegram_status))
        .route("/ops_telegram_configure", post(ops_telegram_configure))
        .route("/ops_telegram_disable", post(ops_telegram_disable))
        .route("/ops_telegram_notify", post(ops_telegram_notify))
        .route("/ops_telegram_resolve", post(ops_telegram_resolve))
}
