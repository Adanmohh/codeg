//! Thin routes nested inside business_identity's authenticated boundary.
use super::{store, types::*, ActorContext};
use crate::{app_state::AppState, business_identity::{IdentityError, Principal}};
use axum::{routing::post, Extension, Json, Router};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input<T> { input: T }

macro_rules! handler {
    ($name:ident, $input:ty, $result:ty) => {
        async fn $name(
            Extension(state): Extension<Arc<AppState>>,
            Extension(principal): Extension<Principal>,
            Json(body): Json<Input<$input>>,
        ) -> Result<Json<$result>, IdentityError> {
            store::$name(&state.db.conn, &ActorContext::authenticated(principal), body.input).await.map(Json)
        }
    };
}
handler!(list, ListInput, TaskPage);
handler!(get, TaskInput, Detail);
handler!(create, CreateInput, Detail);
handler!(update, UpdateInput, Detail);
handler!(assign, AssignInput, Detail);
handler!(progress, ProgressInput, Detail);
handler!(note, TextInput, Detail);
handler!(submit, TextInput, Detail);
handler!(review, ReviewInput, Detail);
handler!(cancel, RevisionInput, Detail);
handler!(archive, ArchiveInput, Detail);

async fn link_execution(
    Extension(principal): Extension<Principal>,
    Json(body): Json<Input<LinkExecutionInput>>,
) -> Result<Json<Detail>, IdentityError> {
    super::link_execution(ActorContext::authenticated(principal), body.input).await.map(Json)
}

pub(crate) fn router() -> Router {
    Router::new().nest("/tasks", Router::new()
        .route("/list", post(list)).route("/get", post(get))
        .route("/create", post(create)).route("/update", post(update))
        .route("/assign", post(assign)).route("/progress", post(progress))
        .route("/note", post(note)).route("/submit", post(submit))
        .route("/review", post(review)).route("/cancel", post(cancel))
        .route("/archive", post(archive)).route("/link-execution", post(link_execution)))
}
