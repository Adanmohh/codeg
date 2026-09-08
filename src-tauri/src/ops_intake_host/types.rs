//! Host DTOs over the accepted Python projection and intake issue contract.
use crate::ops_intake::{
    EvidenceField, EvidenceRef, FilingReceipt, PreparedIssue, RepositoryBinding,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Serialize, thiserror::Error, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HostError {
    #[error("not_configured")]
    NotConfigured,
    #[error("ambiguous_product")]
    AmbiguousProduct,
    #[error("adapter_missing")]
    AdapterMissing,
    #[error("access_denied")]
    AccessDenied,
    #[error("invalid_input")]
    InvalidInput,
    #[error("invalid_evidence")]
    InvalidEvidence,
    #[error("stale_source")]
    StaleSource,
    #[error("source_unavailable")]
    SourceUnavailable,
    #[error("conflict")]
    Conflict,
    #[error("storage_unavailable")]
    StorageUnavailable,
    #[error("task_required")]
    TaskRequired,
    #[error("severity_required")]
    SeverityRequired,
}
impl From<sea_orm::DbErr> for HostError {
    fn from(_: sea_orm::DbErr) -> Self {
        Self::StorageUnavailable
    }
}
impl From<crate::ops_intake::IntakeError> for HostError {
    fn from(e: crate::ops_intake::IntakeError) -> Self {
        use crate::ops_intake::IntakeError as E;
        match e {
            E::NotConfigured => Self::NotConfigured,
            E::AccessDenied => Self::AccessDenied,
            E::InvalidEvidence => Self::InvalidEvidence,
            E::InvalidPayload => Self::InvalidInput,
            E::StaleSource => Self::StaleSource,
            E::Conflict => Self::Conflict,
            E::UpstreamUnavailable => Self::SourceUnavailable,
            E::StorageUnavailable => Self::StorageUnavailable,
        }
    }
}
impl From<crate::db::error::DbError> for HostError {
    fn from(e: crate::db::error::DbError) -> Self {
        use crate::db::error::DbError as E;
        match e {
            E::NotFound(_) => Self::AccessDenied,
            E::Conflict(_) => Self::Conflict,
            E::Validation(_) => Self::InvalidInput,
            _ => Self::StorageUnavailable,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterRef {
    pub product_id: String,
    pub source: crate::ops_intake::SourceKind,
    pub ulid: String,
    pub external_id: Option<String>,
}
impl AdapterRef {
    pub fn source(&self) -> crate::ops_intake::SourceRef {
        crate::ops_intake::SourceRef {
            product_id: self.product_id.clone(),
            source: self.source,
            ulid: self.ulid.clone(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Screenshot {
    pub ulid: String,
    pub content_type: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    High,
    Medium,
    Low,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Triage {
    pub seeded_tags: Vec<String>,
    pub seeded_severity: Severity,
    pub source_tags: Vec<String>,
    pub source_severity: Severity,
    pub confirmed_tags: Option<Vec<String>>,
    pub confirmed_severity: Option<Severity>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    pub field: EvidenceField,
    pub value: String,
    pub provenance: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub schema_version: u8,
    pub source_ref: AdapterRef,
    pub source_revision: String,
    pub fetched_at: String,
    pub title: String,
    pub title_is_draft: bool,
    pub description: String,
    pub feedback_type: Option<String>,
    pub source_status: String,
    pub submitted_at: Option<String>,
    pub source_updated_at: String,
    pub device: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub build_number: Option<String>,
    pub platform: Option<String>,
    pub locale: Option<String>,
    pub screenshots: Vec<Screenshot>,
    pub triage: Triage,
    pub evidence_candidates: Vec<Candidate>,
    pub missing_required: Vec<EvidenceField>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    pub items: Vec<Record>,
    pub next_cursor: Option<String>,
    pub scan_complete: bool,
    pub fetched_at: String,
    pub source_health: String,
}

// Secrets are write-only and deliberately lack Debug/Serialize.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigureInput {
    pub binding: RepositoryBinding,
    pub origin: String,
    pub intake_bearer: Option<String>,
    pub app_private_key: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StoredProduct {
    pub binding: RepositoryBinding,
    pub origin: String,
    pub bearer_ref: Option<String>,
    pub key_ref: Option<String>,
}
#[derive(Serialize)]
pub struct Product {
    pub binding: RepositoryBinding,
    pub origin: String,
    pub intake_credential_present: bool,
    pub app_key_present: bool,
}
#[derive(Serialize)]
pub struct FolderChoice {
    pub id: i32,
    pub name: String,
}
#[derive(Serialize)]
pub struct Status {
    pub products: Vec<Product>,
    pub folders: Vec<FolderChoice>,
    pub adapter_installed: bool,
    pub in_app_available: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductInput {
    pub product_id: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListInput {
    pub product_id: String,
    pub cursor: Option<String>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceInput {
    pub product_id: String,
    pub ulid: String,
}
#[derive(Serialize)]
pub struct Snapshot {
    pub record: Record,
    pub verified_at: Option<i64>,
    pub error: Option<String>,
}
#[derive(Serialize)]
pub struct Listing {
    pub records: Vec<Snapshot>,
    pub next_cursor: Option<String>,
    pub scan_complete: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttachInput {
    pub source: SourceInput,
    pub expected_revision: i32,
    pub field: EvidenceField,
    pub value: String,
    pub content: String,
    pub captured_at: Option<String>,
    pub session_ulid: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Proof {
    pub proof: EvidenceRef,
    pub captured_at: Option<String>,
    pub session_ulid: Option<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Draft {
    pub id: String,
    pub revision: i32,
    pub source_revision: String,
    pub title: String,
    pub summary: String,
    pub labels: Vec<String>,
    pub confirmed_severity: Option<Severity>,
    pub proofs: BTreeMap<String, Proof>,
    pub prepared: Option<PreparedIssue>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveInput {
    pub source: SourceInput,
    pub expected_revision: i32,
    pub title: String,
    pub summary: String,
    pub labels: Vec<String>,
    pub confirmed_severity: Option<Severity>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrepareInput {
    pub source: SourceInput,
    pub expected_revision: i32,
    pub task_id: i32,
}
#[derive(Serialize)]
pub struct TaskChoice {
    pub id: i32,
    pub title: String,
    pub run_seq: i32,
}
#[derive(Serialize)]
pub struct Proposal {
    pub id: i32,
    pub status: String,
    pub payload: Option<PreparedIssue>,
    pub stale: bool,
}
#[derive(Serialize)]
pub struct Detail {
    pub snapshot: Snapshot,
    pub draft: Draft,
    pub tasks: Vec<TaskChoice>,
    pub proposals: Vec<Proposal>,
    pub receipt: Option<FilingReceipt>,
    pub handoff_unknown: bool,
    pub fix_task_id: Option<i32>,
    pub fix_task_conflict: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewInput {
    pub source: SourceInput,
    pub proposal_id: i32,
    pub expected_payload: PreparedIssue,
    pub approved_payload: PreparedIssue,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DenyInput {
    pub source: SourceInput,
    pub proposal_id: i32,
    pub expected_payload: PreparedIssue,
}
