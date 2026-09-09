//! Private persistence models. None serialize to the business transport.
use sea_orm::FromQueryResult;

#[derive(Clone, FromQueryResult)]
pub(super) struct Binding {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub domain: String,
    pub source_owner_id: String,
    pub owner_authority_revision: i64,
    pub resource_json: String,
    pub credential_ref: Option<String>,
    pub revision: i64,
    pub access_epoch: i64,
    pub enabled: bool,
    pub publication_domains: String,
    pub retained_task_text: bool,
}
#[derive(FromQueryResult)]
pub(super) struct Grant {
    pub id: String,
    pub member_id: String,
    pub revision: i64,
    pub state: String,
    pub can_read: bool,
    pub can_import: bool,
    pub can_triage: bool,
    pub publication_domains: String,
    pub expires_at: Option<String>,
}
#[derive(FromQueryResult)]
pub(super) struct Setup {
    pub authorization_epoch: Option<i64>,
    pub id: String,
    pub digest: String,
    pub binding_id: String,
    pub base_revision: Option<i64>,
    pub base_epoch: Option<i64>,
    pub owner_authority_revision: i64,
    pub plan_json: String,
    pub credential_ref: String,
    pub state: String,
    pub expires_at: String,
}
#[derive(Clone, FromQueryResult)]
pub(super) struct Source {
    pub authorization_epoch: Option<i64>,
    pub id: String,
    pub binding_id: String,
    pub kind: String,
    pub external_id: String,
    pub title: String,
    pub revision: Option<i64>,
    pub refresh_fence: i64,
    pub observed_epoch: Option<i64>,
    pub observed_at: Option<String>,
    pub access_until: Option<String>,
    pub access: String,
    pub content: String,
    pub summary: String,
    pub provider_summary_status: Option<String>,
    pub seeded: bool,
}
#[derive(FromQueryResult)]
pub(super) struct Version {
    pub digest: String,
}
#[derive(FromQueryResult)]
pub(super) struct Passage {
    pub id: String,
    pub source_revision: i64,
    pub kind: String,
    pub text: String,
    pub provider_index: Option<i64>,
    pub start: Option<f64>,
    pub end: Option<f64>,
}
#[derive(FromQueryResult)]
pub(super) struct Candidate {
    pub authorization_epoch: Option<i64>,
    pub id: String,
    pub source_id: String,
    pub revision: i64,
    pub source_revision: i64,
    pub prepared_epoch: i64,
    pub state: String,
    pub origin: String,
    pub passage_ids: String,
    pub draft_json: Option<String>,
    pub owner_suggestion: Option<String>,
    pub due_suggestion: Option<String>,
}
#[derive(FromQueryResult)]
pub(super) struct Import {
    pub authorization_epoch: Option<i64>,
    pub id: String,
    pub binding_id: String,
    pub revision: i64,
    pub state: String,
    pub coverage: String,
    pub selection_json: String,
    pub scan_offset: i64,
    pub scan_done: bool,
    pub discovered: i64,
    pub completed: i64,
    pub failed: i64,
    pub attempt_id: Option<String>,
    pub lease_until: Option<String>,
    pub attempt_count: i64,
    pub attempt_epoch: Option<i64>,
    pub next_attempt_at: Option<String>,
    pub error_code: Option<String>,
}
#[derive(FromQueryResult)]
pub(super) struct Decision {
    pub id: String,
    pub candidate_id: String,
    pub from_revision: i64,
    pub source_revision: i64,
    pub kind: String,
    pub actor_id: String,
    pub task_id: Option<String>,
    pub task_revision: Option<i64>,
    pub created_at: String,
}
#[derive(FromQueryResult)]
pub(super) struct Link {
    pub id: String,
    pub source_id: String,
}
#[derive(FromQueryResult)]
pub(super) struct Operation {
    pub operation: String,
    pub digest: String,
    pub result_id: String,
}
