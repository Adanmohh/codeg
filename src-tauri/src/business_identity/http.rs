//! Separate business-only bearer middleware. Never mounted over legacy routes.
use super::{operator_principal, store, types::*, IdentityError, Principal};
use crate::app_state::AppState;
use axum::{
    extract::Request,
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Extension, Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
enum Session {
    Operator,
    Member(Principal),
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input<T> {
    input: T,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Empty {}

impl IntoResponse for IdentityError {
    fn into_response(self) -> Response {
        // Existing AppCommandError authentication failures refer to provider
        // setup and map to 422. This is a session rejection, so use 401 here.
        if matches!(self, Self::Unauthorized) {
            (StatusCode::UNAUTHORIZED, Json(self.command_error())).into_response()
        } else {
            self.command_error().into_response()
        }
    }
}

async fn authenticate(
    state: &AppState,
    operator_token: &str,
    request: &mut Request,
) -> Result<(), IdentityError> {
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(IdentityError::Unauthorized)?;
    let session = if !operator_token.is_empty() && token == operator_token {
        Session::Operator
    } else {
        Session::Member(store::resolve_credential(&state.db.conn, token).await?)
    };
    let principal = match &session {
        Session::Operator => match operator_principal(&state.db.conn).await {
            Ok(p) => Some(p),
            Err(IdentityError::BootstrapRequired)
                if matches!(request.uri().path(), "/context" | "/bootstrap") =>
            {
                None
            }
            Err(e) => return Err(e),
        },
        Session::Member(p) => Some(p.clone()),
    };
    if let Some(p) = principal {
        request.extensions_mut().insert(p);
    }
    request.extensions_mut().insert(session);
    Ok(())
}
async fn require_session(
    mut request: Request,
    next: Next,
    state: Arc<AppState>,
    operator_token: String,
) -> Response {
    let mut response = match authenticate(&state, &operator_token, &mut request).await {
        Ok(()) => next.run(request).await,
        Err(e) => e.into_response(),
    };
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

async fn context(
    Extension(state): Extension<Arc<AppState>>,
    Extension(session): Extension<Session>,
    Json(_): Json<Input<Empty>>,
) -> Result<Json<Context>, IdentityError> {
    match session {
        Session::Operator => store::operator_context(&state.db.conn).await,
        Session::Member(principal) => {
            // Keep authorization and context reads in one snapshot.
            use sea_orm::TransactionTrait;
            let tx = state.db.conn.begin().await?;
            store::context(&tx, &principal).await
        }
    }
    .map(Json)
}
async fn bootstrap(
    Extension(state): Extension<Arc<AppState>>,
    Extension(session): Extension<Session>,
    Json(body): Json<Input<BootstrapInput>>,
) -> Result<Json<Context>, IdentityError> {
    if !matches!(session, Session::Operator) {
        return Err(IdentityError::Forbidden);
    }
    store::bootstrap(&state.db.conn, body.input).await.map(Json)
}
macro_rules! input_handler {
    ($name:ident, $core:path, $input:ty, $result:ty) => {
        async fn $name(
            Extension(state): Extension<Arc<AppState>>,
            Extension(principal): Extension<Principal>,
            Json(body): Json<Input<$input>>,
        ) -> Result<Json<$result>, IdentityError> {
            $core(&state.db.conn, &principal, body.input)
                .await
                .map(Json)
        }
    };
}
input_handler!(members_list, store::list_members, MembersInput, Vec<Member>);
input_handler!(
    members_create,
    store::create_member,
    CreateMemberInput,
    Member
);
input_handler!(
    members_update,
    store::update_member,
    UpdateMemberInput,
    Member
);
input_handler!(
    members_revoke,
    store::revoke_member,
    RevokeMemberInput,
    Member
);
input_handler!(
    credentials_issue,
    store::issue_credential,
    IssueCredentialInput,
    IssuedCredential
);
input_handler!(
    credentials_list,
    store::list_credentials,
    CredentialsInput,
    Vec<Credential>
);
input_handler!(
    credentials_revoke,
    store::revoke_credential,
    RevokeCredentialInput,
    Credential
);

pub(crate) fn router(state: Arc<AppState>, operator_token: String) -> Router {
    let routes = Router::new()
        .route("/context", post(context))
        .route("/bootstrap", post(bootstrap))
        .route("/members/list", post(members_list))
        .route("/members/create", post(members_create))
        .route("/members/update", post(members_update))
        .route("/members/revoke", post(members_revoke))
        .route("/credentials/issue", post(credentials_issue))
        .route("/credentials/list", post(credentials_list))
        .route("/credentials/revoke", post(credentials_revoke));
    // Merge the separately owned relative /tasks router HERE, before this
    // shared auth layer, once the task module is integrated.
    Router::new().nest(
        "/business",
        routes.layer(middleware::from_fn(move |req, next| {
            require_session(req, next, state.clone(), operator_token.clone())
        })),
    )
}
