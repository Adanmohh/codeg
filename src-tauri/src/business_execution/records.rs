//! Private SQL projections, never serialized onto either transport.
use sea_orm::FromQueryResult;

#[derive(Clone, FromQueryResult)]
pub(super) struct Profile {
    pub id: String,
    pub client_id: String,
    pub config_key: String,
    pub config_hash: String,
    pub revision: i64,
    pub summary_json: String,
    pub retired_at: Option<String>,
}
#[derive(Clone, FromQueryResult)]
pub(super) struct Session {
    pub id: String,
    pub organization_id: String,
    pub task_id: String,
    pub member_id: String,
    pub authority_json: String,
    pub authorization_epoch: i64,
    pub task_scope_epoch: i64,
    pub profile_id: String,
    pub profile_revision: i64,
    pub revision: i64,
    pub generation: i64,
    pub mode: String,
    pub status: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub last_activity_at: String,
}
#[derive(Clone, FromQueryResult)]
pub(super) struct Generation {
    pub admission_id: String,
    pub initial_cursor: String,
    pub engine_json: Option<String>,
}
#[derive(Clone, FromQueryResult)]
pub(super) struct Operation {
    pub operation_id: String,
    pub kind: String,
    pub authority_json: String,
    pub authorization_epoch: i64,
    pub input_hash: String,
    pub input_json: String,
    pub task_id: String,
    pub session_id: Option<String>,
    pub generation: Option<i64>,
    pub status: String,
    pub reason: Option<String>,
    pub resource_id: Option<String>,
    pub result_json: Option<String>,
}

#[derive(Clone, FromQueryResult)]
pub(super) struct Output {
    pub id: String,
    pub generation: i64,
    pub revision: i64,
    pub relative_path: String,
    pub metadata_json: String,
}
#[derive(Clone, FromQueryResult)]
pub(super) struct Asset {
    pub id: String,
    pub organization_id: String,
    pub task_id: String,
    pub member_id: String,
    pub revision: i64,
    pub title: String,
    pub media_type: String,
    pub latest_version_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Clone, FromQueryResult)]
pub(super) struct Version {
    pub id: String,
    pub organization_id: String,
    pub task_id: String,
    pub asset_id: String,
    pub version: i64,
    pub title: String,
    pub sha256: String,
    pub byte_size: i64,
    pub media_type: String,
    pub object_id: String,
    pub created_at: String,
    pub created_by: String,
    pub producer_session_id: String,
    pub producer_turn_id: Option<String>,
    pub producer_client_id: String,
    pub producer_model: Option<String>,
}
