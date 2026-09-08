//! Additive routes inside the accepted business Principal middleware.
use super::{
    error::{Error, Reason},
    services::Services,
    types::*,
};
use crate::{app_state::AppState, business_identity::Principal, ops::Operator};
use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
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
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Identity(e) => e.into_response(),
            e => e.command_error().into_response(),
        }
    }
}
fn services(value: Option<Extension<Arc<Services>>>) -> Result<Arc<Services>, Error> {
    match value {
        Some(Extension(value)) => Ok(value),
        None => Services::production(),
    }
}
macro_rules! handler {
    ($name:ident,$input:ty,$result:ty) => {
        async fn $name(
            Extension(state): Extension<Arc<AppState>>,
            Extension(p): Extension<Principal>,
            injected: Option<Extension<Arc<Services>>>,
            body: Result<Json<Input<$input>>, JsonRejection>,
        ) -> Result<Json<$result>, Error> {
            let Json(body) = body.map_err(|_| super::error::invalid())?;
            let services = services(injected)?;
            super::$name(&state.db.conn, &p, &services, body.input)
                .await
                .map(Json)
        }
    };
}
handler!(bindings_list, PageInput, BindingList);
handler!(bindings_status, BindingInput, BindingView);
handler!(bindings_update, UpdateBindingInput, BindingAdmin);
handler!(bindings_disable, DisableBindingInput, BindingAdmin);
handler!(grants_list, BindingPageInput, GrantPage);
handler!(grants_upsert, UpsertGrantInput, GrantResult);
handler!(grants_revoke, RevokeGrantInput, GrantResult);
async fn bindings_create(
    Extension(state): Extension<Arc<AppState>>,
    Extension(p): Extension<Principal>,
    injected: Option<Extension<Arc<Services>>>,
    body: Result<Json<Input<CreateBindingInput>>, JsonRejection>,
) -> Result<Json<BindingAdmin>, Error> {
    let Json(body) = body.map_err(|_| super::error::invalid())?;
    // Only the verified original operator transport resolves the legacy account.
    // Ordinary member core never constructs or impersonates this capability.
    let op = if p.is_operator() && !matches!(&body.input.source, SourceSetup::Fireflies { .. }) {
        Some(Operator::server().map_err(|_| Reason::BindingUnavailable)?)
    } else {
        None
    };
    let services = services(injected)?;
    super::bindings_create(&state.db.conn, &p, &services, op.as_ref(), body.input)
        .await
        .map(Json)
}
pub(crate) fn router() -> Router {
    Router::new().nest(
        "/intake",
        Router::new()
            .route("/bindings/list", post(bindings_list))
            .route("/bindings/status", post(bindings_status))
            .route("/bindings/create", post(bindings_create))
            .route("/bindings/update", post(bindings_update))
            .route("/bindings/disable", post(bindings_disable))
            .route("/grants/list", post(grants_list))
            .route("/grants/upsert", post(grants_upsert))
            .route("/grants/revoke", post(grants_revoke)),
    )
}
