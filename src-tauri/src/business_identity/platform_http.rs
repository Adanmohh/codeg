//! Separate original-operator middleware; member auth is never tried here.
use super::{
    platform::{self, PlatformContext},
    IdentityError,
};
use crate::{
    app_state::AppState,
    web::auth::{self, AuthenticatedOperator},
};
use axum::{
    http::{header, HeaderValue},
    middleware,
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
async fn context(
    Extension(_): Extension<AuthenticatedOperator>,
    Json(_): Json<Input<Empty>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({"platformOperator":true}))
}
async fn list(
    Extension(state): Extension<Arc<AppState>>,
    Extension(marker): Extension<AuthenticatedOperator>,
    Json(_): Json<Input<Empty>>,
) -> Result<Json<Vec<platform::TenantSummary>>, IdentityError> {
    platform::list(&state.db.conn, &PlatformContext::from_operator(&marker))
        .await
        .map(Json)
}
macro_rules! handler {
    ($name:ident,$core:path,$input:ty,$result:ty) => {
        async fn $name(
            Extension(state): Extension<Arc<AppState>>,
            Extension(marker): Extension<AuthenticatedOperator>,
            Json(body): Json<Input<$input>>,
        ) -> Result<Json<$result>, IdentityError> {
            $core(
                &state.db.conn,
                &PlatformContext::from_operator(&marker),
                body.input,
            )
            .await
            .map(Json)
        }
    };
}
handler!(
    create,
    platform::create,
    platform::CreateTenantInput,
    platform::ProvisionResult
);
handler!(
    status,
    platform::status,
    platform::StatusInput,
    super::types::Organization
);
handler!(
    reissue,
    platform::reissue,
    platform::ReissueInput,
    platform::ProvisionResult
);
pub(crate) fn router(token: String) -> Router {
    let routes = Router::new()
        .route("/context", post(context))
        .route("/tenants/list", post(list))
        .route("/tenants/create", post(create))
        .route("/tenants/status", post(status))
        .route("/tenants/reissue-owner-credential", post(reissue))
        .layer(middleware::from_fn(move |req, next| {
            auth::require_token(req, next, token.clone())
        }))
        .layer(middleware::from_fn(
            |req, next: middleware::Next| async move {
                let mut response = next.run(req).await;
                response
                    .headers_mut()
                    .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
                response
            },
        ));
    Router::new().nest("/platform/business", routes)
}
