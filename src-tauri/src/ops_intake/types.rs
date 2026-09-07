//! Strict issue/evidence boundary glue; source mappings in NOTICE.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const TEMPLATE_VERSION: &str = "hafidh-issue-v1";
pub const SOURCE_MAX_AGE_SECONDS: i64 = 900;

/// Stable public errors. Never include HTTP bodies, credentials, SQL or evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum IntakeError {
    #[error("not_configured")]
    NotConfigured,
    #[error("access_denied")]
    AccessDenied,
    #[error("invalid_evidence")]
    InvalidEvidence,
    #[error("invalid_payload")]
    InvalidPayload,
    #[error("stale_source")]
    StaleSource,
    #[error("filing_conflict")]
    Conflict,
    #[error("upstream_unavailable")]
    UpstreamUnavailable,
    #[error("storage_unavailable")]
    StorageUnavailable,
}
impl From<sea_orm::DbErr> for IntakeError {
    fn from(_: sea_orm::DbErr) -> Self {
        Self::StorageUnavailable
    }
}

pub(super) fn digest(bytes: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(bytes.as_ref()))
}
pub(super) fn json_digest<T: Serialize>(value: &T) -> Result<String, IntakeError> {
    let mut value = serde_json::to_value(value).map_err(|_| IntakeError::InvalidPayload)?;
    value.sort_all_objects();
    Ok(digest(value.to_string()))
}
pub(super) fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 100
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
}
pub(super) fn ulid(value: &str) -> bool {
    value.len() == 26
        && value.as_bytes()[0] <= b'7'
        && value
            .bytes()
            .all(|c| b"0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(&c))
}
pub(super) fn sha(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

/// A bounded accidental-secret detector, followed by mandatory human review.
/// Reject instead of silently changing bytes after approval.
pub(super) fn public_text(value: &str, max: usize) -> bool {
    use std::sync::LazyLock;
    static PRIVATE: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(
        r"(?i)(?:bearer\s+\S+|(?:gh[pousr]_|github_pat_)\w+|eyJ[\w-]+\.[\w-]+\.[\w-]+|-----BEGIN .*PRIVATE KEY|[\w.+-]+@[\w.-]+\.[a-z]{2,}|https?://|s3://|hf://|/Users/|/home/|(?:password|secret|token|api[_-]?key)\s*[:=])"
    ).expect("constant regex")
    });
    !value.trim().is_empty()
        && value.trim() == value
        && value.len() <= max
        && !value
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\t')
        && !PRIVATE.is_match(value)
}
pub(super) fn evidence_value(value: &str) -> bool {
    public_text(value, 8192)
        && !matches!(
            value.to_ascii_lowercase().as_str(),
            "unknown"
                | "n/a"
                | "na"
                | "none"
                | "null"
                | "tbd"
                | "todo"
                | "dummy"
                | "test"
                | "not available"
                | "unspecified"
                | "-"
                | "?"
        )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Testflight,
    InApp,
}
impl SourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Testflight => "testflight",
            Self::InApp => "in_app",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRef {
    pub product_id: String,
    pub source: SourceKind,
    pub ulid: String,
}
impl SourceRef {
    pub(super) fn validate(&self) -> Result<(), IntakeError> {
        if !identifier(&self.product_id) || !ulid(&self.ulid) {
            return Err(IntakeError::InvalidPayload);
        }
        Ok(())
    }
    pub(super) fn key(&self) -> String {
        format!("{}:{}:{}", self.product_id, self.source.as_str(), self.ulid)
    }
}

/// Host configuration, never accepted from an agent tool or mutable draft.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryBinding {
    pub product_id: String,
    pub folder_id: i32,
    pub app_id: String,
    pub installation_id: i64,
    pub repository_id: i64,
    pub full_name: String,
    pub enabled: bool,
}
impl RepositoryBinding {
    pub(crate) fn validate(&self) -> Result<(), IntakeError> {
        let parts: Vec<_> = self.full_name.split('/').collect();
        if !identifier(&self.product_id)
            || !identifier(&self.app_id)
            || self.folder_id <= 0
            || self.installation_id <= 0
            || self.repository_id <= 0
            || parts.len() != 2
            || !parts.iter().all(|p| {
                !p.is_empty()
                    && p.len() <= 100
                    && p.bytes()
                        .all(|c| c.is_ascii_alphanumeric() || b"_-.".contains(&c))
            })
            || parts.iter().any(|p| *p == "." || *p == "..")
        {
            return Err(IntakeError::InvalidPayload);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceField {
    Build,
    Screen,
    Reciter,
    Log,
}
impl EvidenceField {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Screen => "screen",
            Self::Reciter => "reciter",
            Self::Log => "log",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceRef {
    pub artifact_id: String,
    pub sha256: String,
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceSet {
    pub build: EvidenceRef,
    pub screen: EvidenceRef,
    pub reciter: EvidenceRef,
    pub log: EvidenceRef,
}
impl EvidenceSet {
    pub(super) fn fields(&self) -> [(EvidenceField, &EvidenceRef); 4] {
        [
            (EvidenceField::Build, &self.build),
            (EvidenceField::Screen, &self.screen),
            (EvidenceField::Reciter, &self.reciter),
            (EvidenceField::Log, &self.log),
        ]
    }
}

/// Deliberately not Deserialize: only a trusted human attachment adapter builds it.
pub enum EvidenceProvenance {
    AscBuild,
    HumanReport,
    Recorder,
    LocalDiagnostic,
    SessionDiagnostic,
}

pub struct EvidenceAttachment {
    pub source_ref: SourceRef,
    pub source_revision: String,
    pub field: EvidenceField,
    pub value: String,
    /// Reviewed, sanitized UTF-8 artifact bytes, stored locally without URLs.
    pub content: Vec<u8>,
    pub captured_at: Option<String>,
    /// Required when the diagnostic came from a recitation session.
    pub session_ulid: Option<String>,
    pub provenance: EvidenceProvenance,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IssueDraftV1 {
    pub schema_version: u8,
    pub template_version: String,
    pub task_id: i32,
    pub run_seq: i32,
    pub source_ref: SourceRef,
    pub source_revision: String,
    pub title: String,
    pub summary: String,
    pub labels: Vec<String>,
    pub evidence: EvidenceSet,
}
impl IssueDraftV1 {
    pub(super) fn validate(&self) -> Result<(), IntakeError> {
        self.source_ref.validate()?;
        if self.schema_version != 1
            || self.template_version != TEMPLATE_VERSION
            || self.task_id <= 0
            || self.run_seq < 0
            || !sha(&self.source_revision)
            || !public_text(&self.title, 800)
            || self.title.chars().count() > 200
            || self.title.contains('\n')
            || !public_text(&self.summary, 8192)
            || self.labels.len() > 20
            || self.labels.iter().any(|l| {
                !public_text(l, 50)
                    || !l
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"_:-".contains(&b))
            })
        {
            return Err(IntakeError::InvalidPayload);
        }
        let unique: std::collections::HashSet<_> = self.labels.iter().collect();
        if unique.len() != self.labels.len() {
            return Err(IntakeError::InvalidPayload);
        }
        for (field, proof) in self.evidence.fields() {
            if !identifier(&proof.artifact_id)
                || !sha(&proof.sha256)
                || !evidence_value(&proof.value)
                || (field != EvidenceField::Log
                    && (proof.value.len() > 300 || proof.value.contains('\n')))
            {
                return Err(IntakeError::InvalidEvidence);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutgoingIssue {
    pub title: String,
    pub body: String,
    pub labels: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedIssue {
    pub draft: IssueDraftV1,
    pub repository_id: i64,
    pub repository: String,
    pub binding_digest: String,
    pub outgoing: OutgoingIssue,
}
impl PreparedIssue {
    pub fn payload(&self) -> Result<serde_json::Value, IntakeError> {
        serde_json::to_value(self).map_err(|_| IntakeError::InvalidPayload)
    }
    pub fn payload_digest(&self) -> Result<String, IntakeError> {
        json_digest(self)
    }
    pub fn marker(&self) -> String {
        format!(
            "<!-- hafidh-ops:{} -->",
            digest(format!(
                "{}:{}",
                self.draft.source_ref.key(),
                self.repository_id
            ))
        )
    }
    pub(super) fn render(&self) -> OutgoingIssue {
        let d = &self.draft;
        let mut body = format!(
            "{}\n\nSource: {} / {} / {}\nRevision: {}\nTemplate: {}\n",
            d.summary,
            d.source_ref.product_id,
            d.source_ref.source.as_str(),
            d.source_ref.ulid,
            d.source_revision,
            d.template_version
        );
        for (field, proof) in d.evidence.fields() {
            // Indented Markdown avoids untrusted fences/markup escaping a log block.
            body.push_str(&format!("\n## {}\n\n", field.as_str()));
            for line in proof.value.lines() {
                body.push_str(&format!("    {line}\n"));
            }
            body.push_str(&format!(
                "\nEvidence: {}\nSHA-256: {}\n",
                proof.artifact_id, proof.sha256
            ));
        }
        body.push_str(&format!("\n{}\n", self.marker()));
        OutgoingIssue {
            title: d.title.clone(),
            body,
            labels: d.labels.clone(),
        }
    }
    pub(super) fn validate(&self) -> Result<(), IntakeError> {
        self.draft.validate()?;
        if self.repository_id <= 0
            || !sha(&self.binding_digest)
            || self.outgoing != self.render()
            || self.outgoing.body.len() > 16 * 1024
        {
            return Err(IntakeError::InvalidPayload);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreatedIssue {
    pub id: i64,
    pub number: i64,
    pub html_url: String,
    pub labels: Vec<String>,
    pub labels_match: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilingState {
    Unknown,
    Failed,
    Created,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilingReceipt {
    pub attempt_id: i64,
    pub proposal_id: i32,
    pub state: FilingState,
    pub issue: Option<CreatedIssue>,
    pub error_code: Option<String>,
    pub retry_after: Option<i64>,
}
