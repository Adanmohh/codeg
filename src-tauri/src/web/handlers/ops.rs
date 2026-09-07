//! Codeg's protected POST-command pattern; no public agent/receipt routes.
use crate::{
    app_error::AppCommandError,
    app_state::AppState,
    ops::{command_error, review, store, types::*, Operator},
    web::auth::AuthenticatedOperator,
};
use axum::{routing::post, Extension, Json, Router};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input<T> {
    input: T,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Empty {}

macro_rules! read_handler {
    ($name:ident, $core:path, $result:ty) => {
        async fn $name(
            Extension(state): Extension<Arc<AppState>>,
            Extension(_human): Extension<AuthenticatedOperator>,
            Json(_): Json<Empty>,
        ) -> Result<Json<$result>, AppCommandError> {
            Ok(Json(
                $core(&state.db.conn, &Operator::server()?)
                    .await
                    .map_err(command_error)?,
            ))
        }
    };
}
macro_rules! input_handler {
    ($name:ident, $core:path, $input:ty, $result:ty) => {
        async fn $name(
            Extension(state): Extension<Arc<AppState>>,
            Extension(_human): Extension<AuthenticatedOperator>,
            Json(params): Json<Input<$input>>,
        ) -> Result<Json<$result>, AppCommandError> {
            Ok(Json(
                $core(&state.db.conn, &Operator::server()?, params.input)
                    .await
                    .map_err(command_error)?,
            ))
        }
    };
}
read_handler!(ops_context, store::context, Context);
read_handler!(ops_proposals, review::list, Vec<Proposal>);
read_handler!(ops_morning, store::morning, Morning);
input_handler!(ops_inbox_create, store::create_inbox, InboxInput, Inbox);
input_handler!(ops_tickets, store::list_tickets, TicketsInput, TicketPage);
input_handler!(ops_thread, store::thread, ThreadInput, Thread);
input_handler!(ops_note_add, store::add_note, NoteInput, Message);
input_handler!(ops_draft_save, store::save_draft, SaveDraftInput, Draft);
input_handler!(ops_proposal_get, review::get, ProposalInput, Proposal);
input_handler!(ops_proposal_deny, review::deny, DenyInput, Proposal);
async fn ops_proposal_approve(
    Extension(state): Extension<Arc<AppState>>,
    Extension(_human): Extension<AuthenticatedOperator>,
    Json(params): Json<Input<ReviewInput>>,
) -> Result<Json<Proposal>, AppCommandError> {
    Ok(Json(
        review::approve(&state.db.conn, &Operator::server()?, params.input).await?,
    ))
}

pub fn router() -> Router {
    Router::new()
        .route("/ops_context", post(ops_context))
        .route("/ops_inbox_create", post(ops_inbox_create))
        .route("/ops_tickets", post(ops_tickets))
        .route("/ops_thread", post(ops_thread))
        .route("/ops_note_add", post(ops_note_add))
        .route("/ops_draft_save", post(ops_draft_save))
        .route("/ops_proposals", post(ops_proposals))
        .route("/ops_proposal_get", post(ops_proposal_get))
        .route("/ops_proposal_approve", post(ops_proposal_approve))
        .route("/ops_proposal_deny", post(ops_proposal_deny))
        .route("/ops_morning", post(ops_morning))
}
