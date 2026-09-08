//! Existing operator marker and POST transport, with fixed JSON rejection errors.
use crate::{
    app_error::AppCommandError,
    app_state::AppState,
    ops::Operator,
    ops_intake_host::{command_error, fix_task, operator, review, types::*, HostRuntime},
    web::auth::AuthenticatedOperator,
};
use axum::{
    extract::{rejection::JsonRejection, DefaultBodyLimit},
    routing::post,
    Extension, Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input<T> {
    input: T,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Empty {}
fn runtime(value: Option<Extension<Arc<HostRuntime>>>) -> Arc<HostRuntime> {
    value
        .map(|Extension(value)| value)
        .unwrap_or_else(HostRuntime::production)
}
fn input<T>(value: Result<Json<Input<T>>, JsonRejection>) -> Result<T, AppCommandError> {
    value
        .map(|Json(value)| value.input)
        .map_err(|_| command_error(HostError::InvalidInput))
}
macro_rules! handler {
    ($name:ident,$core:path,$input:ty,$result:ty) => {
        async fn $name(
            Extension(state): Extension<Arc<AppState>>,
            Extension(_human): Extension<AuthenticatedOperator>,
            rt: Option<Extension<Arc<HostRuntime>>>,
            body: Result<Json<Input<$input>>, JsonRejection>,
        ) -> Result<Json<$result>, AppCommandError> {
            let input = input(body)?;
            Ok(Json(
                $core(&state.db.conn, &Operator::server()?, &runtime(rt), input)
                    .await
                    .map_err(command_error)?,
            ))
        }
    };
}
handler!(
    ops_intake_configure,
    operator::configure,
    ConfigureInput,
    Status
);
handler!(ops_intake_list, operator::list, ListInput, Listing);
handler!(ops_intake_refresh, operator::refresh, SourceInput, Detail);
handler!(ops_intake_save, operator::save, SaveInput, Draft);
handler!(ops_intake_attach, operator::attach, AttachInput, Draft);
handler!(ops_intake_prepare, review::prepare, PrepareInput, Draft);
handler!(ops_intake_approve, review::approve, ReviewInput, Detail);
handler!(ops_intake_deny, review::deny, DenyInput, Detail);
handler!(ops_intake_reconcile, review::reconcile, SourceInput, Detail);
handler!(ops_intake_fix, fix_task::create, SourceInput, Detail);
async fn ops_intake_status(
    Extension(state): Extension<Arc<AppState>>,
    Extension(_human): Extension<AuthenticatedOperator>,
    rt: Option<Extension<Arc<HostRuntime>>>,
    body: Result<Json<Input<Empty>>, JsonRejection>,
) -> Result<Json<Status>, AppCommandError> {
    input(body)?;
    Ok(Json(
        operator::status(&state.db.conn, &Operator::server()?, &runtime(rt))
            .await
            .map_err(command_error)?,
    ))
}
async fn ops_intake_detail(
    Extension(state): Extension<Arc<AppState>>,
    Extension(_human): Extension<AuthenticatedOperator>,
    body: Result<Json<Input<SourceInput>>, JsonRejection>,
) -> Result<Json<Detail>, AppCommandError> {
    Ok(Json(
        operator::detail(&state.db.conn, &Operator::server()?, input(body)?)
            .await
            .map_err(command_error)?,
    ))
}
pub fn router() -> Router {
    Router::new()
        .route("/ops_intake_status", post(ops_intake_status))
        .route("/ops_intake_configure", post(ops_intake_configure))
        .route("/ops_intake_list", post(ops_intake_list))
        .route("/ops_intake_detail", post(ops_intake_detail))
        .route("/ops_intake_refresh", post(ops_intake_refresh))
        .route("/ops_intake_save", post(ops_intake_save))
        .route("/ops_intake_attach", post(ops_intake_attach))
        .route("/ops_intake_prepare", post(ops_intake_prepare))
        .route("/ops_intake_approve", post(ops_intake_approve))
        .route("/ops_intake_deny", post(ops_intake_deny))
        .route("/ops_intake_reconcile", post(ops_intake_reconcile))
        .route("/ops_intake_fix", post(ops_intake_fix))
        .layer(DefaultBodyLimit::max(128 * 1024))
}
