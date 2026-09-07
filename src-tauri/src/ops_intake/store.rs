//! Local trust bindings and SQLite CAS/receipts. No network in transactions.
use super::types::*;
use chrono::Utc;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DbBackend, QueryResult, Statement, TransactionTrait, Value,
};

pub(super) fn sql(text: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Sqlite, text, values)
}
fn encoded<T: serde::Serialize>(value: &T) -> Result<String, IntakeError> {
    serde_json::to_string(value).map_err(|_| IntakeError::InvalidPayload)
}

/// Trusted host settings only. Authentication/permission to configure belongs
/// to the adapter. Never expose as an agent action or a generic SQL endpoint.
pub async fn configure_repository(
    conn: &DatabaseConnection,
    binding: &RepositoryBinding,
) -> Result<(), IntakeError> {
    binding.validate()?;
    conn.execute(sql("INSERT INTO ops_intake_binding(product_id,folder_id,config_json) VALUES(?,?,?) ON CONFLICT(product_id) DO UPDATE SET folder_id=excluded.folder_id,config_json=excluded.config_json",
        vec![binding.product_id.clone().into(), binding.folder_id.into(), encoded(binding)?.into()])).await?;
    Ok(())
}
pub(super) async fn binding<C: ConnectionTrait>(
    conn: &C,
    product: &str,
) -> Result<RepositoryBinding, IntakeError> {
    let row = conn
        .query_one(sql(
            "SELECT config_json FROM ops_intake_binding WHERE product_id=?",
            vec![product.into()],
        ))
        .await?
        .ok_or(IntakeError::NotConfigured)?;
    let b: RepositoryBinding = serde_json::from_str(&row.try_get::<String>("", "config_json")?)
        .map_err(|_| IntakeError::StorageUnavailable)?;
    b.validate()?;
    if !b.enabled {
        return Err(IntakeError::AccessDenied);
    }
    Ok(b)
}

/// Host read adapter calls this only after a successful fresh authorized GET.
/// The timestamp is supplied by that adapter, never by an agent proposal.
pub async fn record_source(
    conn: &DatabaseConnection,
    source: &SourceRef,
    revision: &str,
    fetched_at: i64,
) -> Result<(), IntakeError> {
    source.validate()?;
    if !sha(revision) || !fresh(fetched_at) {
        return Err(IntakeError::StaleSource);
    }
    binding(conn, &source.product_id).await?;
    conn.execute(sql("INSERT INTO ops_intake_source(source_key,product_id,revision,fetched_at) VALUES(?,?,?,?) ON CONFLICT(source_key) DO UPDATE SET revision=excluded.revision,fetched_at=excluded.fetched_at WHERE excluded.fetched_at>=ops_intake_source.fetched_at",
        vec![source.key().into(), source.product_id.clone().into(), revision.into(), fetched_at.into()])).await?;
    Ok(())
}
fn fresh(at: i64) -> bool {
    let now = Utc::now().timestamp();
    at <= now + 30 && at >= now - SOURCE_MAX_AGE_SECONDS
}
async fn source_current<C: ConnectionTrait>(
    conn: &C,
    source: &SourceRef,
    revision: &str,
) -> Result<(), IntakeError> {
    let row = conn
        .query_one(sql(
            "SELECT revision,fetched_at FROM ops_intake_source WHERE source_key=? AND product_id=?",
            vec![source.key().into(), source.product_id.clone().into()],
        ))
        .await?
        .ok_or(IntakeError::StaleSource)?;
    if row.try_get::<String>("", "revision")? != revision || !fresh(row.try_get("", "fetched_at")?)
    {
        return Err(IntakeError::StaleSource);
    }
    Ok(())
}

/// Only an authenticated human attachment flow may construct this input.
/// Store sanitized bytes, not external paths, raw buckets or an agent boolean.
pub async fn attach_evidence(
    conn: &DatabaseConnection,
    attachment: EvidenceAttachment,
    human_actor: &str,
) -> Result<EvidenceRef, IntakeError> {
    let a = attachment;
    a.source_ref.validate()?;
    // Preserve the authenticated adapter's canonical principal, including
    // operator:http / operator:desktop. This argument is never a wire field.
    if human_actor.trim().is_empty()
        || human_actor.trim() != human_actor
        || human_actor.len() > 128
        || human_actor.chars().any(char::is_control)
        || !evidence_value(&a.value)
        || a.content.is_empty()
        || a.content.len() > 8192
    {
        return Err(IntakeError::InvalidEvidence);
    }
    let text = std::str::from_utf8(&a.content).map_err(|_| IntakeError::InvalidEvidence)?;
    if !public_text(text, 8192)
        || !text.contains(&a.value)
        || a.session_ulid.as_ref().is_some_and(|s| !ulid(s))
        || a.expires_at.is_some_and(|t| t <= Utc::now().timestamp())
        || a.captured_at
            .as_ref()
            .is_some_and(|s| chrono::DateTime::parse_from_rfc3339(s).is_err())
    {
        return Err(IntakeError::InvalidEvidence);
    }
    match (&a.field, &a.provenance) {
        (EvidenceField::Build, EvidenceProvenance::AscBuild) => {
            if !a.value.bytes().all(|c| c.is_ascii_digit() || c == b'.')
                || a.value.split('.').any(|p| p.is_empty())
            {
                return Err(IntakeError::InvalidEvidence);
            }
        }
        (EvidenceField::Build, EvidenceProvenance::HumanReport) => {
            // Human-entered build must be explicit; a marketing version alone
            // cannot stand in for it. ASC dotted build identifiers use AscBuild.
            if !a.value.bytes().all(|c| c.is_ascii_digit()) || a.value == "0" {
                return Err(IntakeError::InvalidEvidence);
            }
        }
        (
            EvidenceField::Screen | EvidenceField::Reciter,
            EvidenceProvenance::HumanReport | EvidenceProvenance::Recorder,
        ) => {}
        (EvidenceField::Log, EvidenceProvenance::LocalDiagnostic) => {}
        (EvidenceField::Log, EvidenceProvenance::SessionDiagnostic) if a.session_ulid.is_some() => {
        }
        _ => return Err(IntakeError::InvalidEvidence),
    }
    let txn = conn.begin().await?;
    // Acquire write lock before checking mutable source revision.
    txn.execute(sql(
        "UPDATE ops_intake_source SET fetched_at=fetched_at WHERE source_key=?",
        vec![a.source_ref.key().into()],
    ))
    .await?;
    source_current(&txn, &a.source_ref, &a.source_revision).await?;
    let proof = EvidenceRef {
        artifact_id: uuid::Uuid::new_v4().to_string(),
        sha256: digest(&a.content),
        value: a.value,
    };
    txn.execute(sql("INSERT INTO ops_intake_evidence(artifact_id,source_key,revision,field,value,content,sha256,reviewed_by,captured_at,session_ulid,expires_at) VALUES(?,?,?,?,?,?,?,?,?,?,?)",
        vec![proof.artifact_id.clone().into(), a.source_ref.key().into(), a.source_revision.into(), a.field.as_str().into(), proof.value.clone().into(), a.content.into(), proof.sha256.clone().into(), human_actor.into(), a.captured_at.into(), a.session_ulid.into(), a.expires_at.into()])).await?;
    txn.commit().await?;
    Ok(proof)
}

/// Host-only revocation; an already sent issue is never changed implicitly.
pub async fn revoke_evidence(
    conn: &DatabaseConnection,
    artifact_id: &str,
) -> Result<(), IntakeError> {
    conn.execute(sql(
        "DELETE FROM ops_intake_evidence WHERE artifact_id=?",
        vec![artifact_id.into()],
    ))
    .await?;
    Ok(())
}

pub(super) async fn validate_bound<C: ConnectionTrait>(
    conn: &C,
    prepared: &PreparedIssue,
) -> Result<RepositoryBinding, IntakeError> {
    prepared.validate()?;
    let draft = &prepared.draft;
    let b = binding(conn, &draft.source_ref.product_id).await?;
    if b.repository_id != prepared.repository_id
        || b.full_name != prepared.repository
        || json_digest(&b)? != prepared.binding_digest
    {
        return Err(IntakeError::AccessDenied);
    }
    let task = conn.query_one(sql("SELECT w.id FROM work_task w JOIN folder f ON f.id=w.folder_id WHERE w.id=? AND w.run_seq=? AND w.folder_id=? AND w.deleted_at IS NULL AND f.deleted_at IS NULL AND w.status IN ('running','awaiting_input')",
        vec![draft.task_id.into(), draft.run_seq.into(), b.folder_id.into()])).await?;
    if task.is_none() {
        return Err(IntakeError::Conflict);
    }
    source_current(conn, &draft.source_ref, &draft.source_revision).await?;
    for (field, proof) in draft.evidence.fields() {
        let row = conn.query_one(sql("SELECT value,content,sha256,expires_at FROM ops_intake_evidence WHERE artifact_id=? AND source_key=? AND revision=? AND field=?",
            vec![proof.artifact_id.clone().into(), draft.source_ref.key().into(), draft.source_revision.clone().into(), field.as_str().into()])).await?.ok_or(IntakeError::InvalidEvidence)?;
        let bytes: Vec<u8> = row.try_get("", "content")?;
        if bytes.is_empty()
            || digest(&bytes) != proof.sha256
            || row.try_get::<String>("", "sha256")? != proof.sha256
            || row.try_get::<String>("", "value")? != proof.value
            || row
                .try_get::<Option<i64>>("", "expires_at")?
                .is_some_and(|t| t <= Utc::now().timestamp())
        {
            return Err(IntakeError::InvalidEvidence);
        }
    }
    Ok(b)
}

pub(super) async fn agent_allowed<C: ConnectionTrait>(
    conn: &C,
    agent: &str,
    repository: &str,
) -> Result<bool, IntakeError> {
    let row = conn.query_one(sql("SELECT mode FROM ops_agent_scope WHERE agent_id=? AND domain='github' AND (resource=? OR resource IS NULL) ORDER BY resource IS NOT NULL DESC LIMIT 1",
        vec![agent.into(), repository.into()])).await?;
    Ok(match row {
        Some(r) => matches!(
            r.try_get::<String>("", "mode")?.as_str(),
            "propose" | "act_low_risk"
        ),
        // Match the accepted gate's missing-scope default. Propose still hits
        // the destructive floor; only explicit read/unknown modes deny here.
        None => true,
    })
}

pub async fn prepare(
    conn: &DatabaseConnection,
    draft: IssueDraftV1,
) -> Result<PreparedIssue, IntakeError> {
    draft.validate()?;
    let b = binding(conn, &draft.source_ref.product_id).await?;
    let mut prepared = PreparedIssue {
        draft,
        repository_id: b.repository_id,
        repository: b.full_name.clone(),
        binding_digest: json_digest(&b)?,
        outgoing: OutgoingIssue {
            title: String::new(),
            body: String::new(),
            labels: vec![],
        },
    };
    prepared.outgoing = prepared.render();
    validate_bound(conn, &prepared).await?;
    Ok(prepared)
}

pub(super) async fn audit<C: ConnectionTrait>(
    conn: &C,
    p: &PreparedIssue,
    actor: &str,
    action: &str,
    detail: serde_json::Value,
) -> Result<(), IntakeError> {
    conn.execute(sql("INSERT INTO ops_audit_log(task_id,run_seq,actor,action,detail,created_at) VALUES(?,?,?,?,?,?)",
        vec![p.draft.task_id.into(), p.draft.run_seq.into(), actor.into(), action.into(), detail.to_string().into(), Utc::now().to_rfc3339().into()])).await?;
    Ok(())
}

/// Write lock, fresh evidence and approval/task binding, then durable unknown
/// reservation. Crash at any point after commit cannot open a retry window.
pub(super) async fn reserve(
    conn: &DatabaseConnection,
    proposal_id: i32,
    p: &PreparedIssue,
) -> Result<i64, IntakeError> {
    let txn = conn.begin().await?;
    txn.execute(sql(
        "UPDATE work_task SET run_seq=run_seq WHERE id=?",
        vec![p.draft.task_id.into()],
    ))
    .await?;
    validate_bound(&txn, p).await?;
    let proposal = txn.query_one(sql("SELECT agent_id,verdict_by FROM ops_proposal WHERE id=? AND task_id=? AND run_seq=? AND action_name='github.create_issue' AND status='approved'",
        vec![proposal_id.into(), p.draft.task_id.into(), p.draft.run_seq.into()])).await?.ok_or(IntakeError::Conflict)?;
    let agent: String = proposal.try_get("", "agent_id")?;
    let actor: Option<String> = proposal.try_get("", "verdict_by")?;
    if actor.as_deref().is_none_or(|a| a.is_empty() || a == agent)
        || !agent_allowed(&txn, &agent, &p.repository).await?
    {
        return Err(IntakeError::AccessDenied);
    }
    let denied = txn.query_one(sql("SELECT id FROM ops_agent_rule WHERE agent_id=? AND domain='github' AND behavior='deny' AND (resource IS NULL OR resource=?) AND (action_name IS NULL OR action_name='github.create_issue') LIMIT 1",
        vec![agent.into(), p.repository.clone().into()])).await?;
    if denied.is_some() {
        return Err(IntakeError::AccessDenied);
    }
    let active = txn.query_one(sql("SELECT id FROM ops_intake_filing WHERE proposal_id=? OR (source_key=? AND repository_id=? AND (state IN ('unknown','created') OR retry_after>?)) LIMIT 1",
        vec![proposal_id.into(), p.draft.source_ref.key().into(), p.repository_id.into(), Utc::now().timestamp().into()])).await?;
    if active.is_some() {
        return Err(IntakeError::Conflict);
    }
    let inserted = txn.execute(sql("INSERT INTO ops_intake_filing(source_key,repository_id,proposal_id,task_id,run_seq,payload_digest,payload_json,started_at,state) VALUES(?,?,?,?,?,?,?,?,'unknown')",
        vec![p.draft.source_ref.key().into(), p.repository_id.into(), proposal_id.into(), p.draft.task_id.into(), p.draft.run_seq.into(), p.payload_digest()?.into(), encoded(p)?.into(), Utc::now().timestamp().into()])).await?;
    let id = inserted.last_insert_id() as i64;
    audit(&txn, p, actor.as_deref().ok_or(IntakeError::AccessDenied)?, "github.filing_reserved", serde_json::json!({"attempt_id":id,"proposal_id":proposal_id,"payload_sha256":p.payload_digest()?})).await?;
    txn.commit().await?;
    Ok(id)
}

fn receipt(row: &QueryResult) -> Result<FilingReceipt, IntakeError> {
    let issue: Option<String> = row.try_get("", "issue_json")?;
    Ok(FilingReceipt {
        attempt_id: row.try_get("", "id")?,
        proposal_id: row.try_get("", "proposal_id")?,
        state: match row.try_get::<String>("", "state")?.as_str() {
            "created" => FilingState::Created,
            "failed" => FilingState::Failed,
            _ => FilingState::Unknown,
        },
        issue: issue
            .map(|s| serde_json::from_str(&s))
            .transpose()
            .map_err(|_| IntakeError::StorageUnavailable)?,
        error_code: row.try_get("", "error_code")?,
        retry_after: row.try_get("", "retry_after")?,
    })
}
/// Adapter must authorize product access before calling; no external side effect.
pub async fn filing_status(
    conn: &DatabaseConnection,
    source: &SourceRef,
    repository_id: i64,
) -> Result<Option<FilingReceipt>, IntakeError> {
    source.validate()?;
    conn.query_one(sql("SELECT * FROM ops_intake_filing WHERE source_key=? AND repository_id=? ORDER BY id DESC LIMIT 1",
        vec![source.key().into(), repository_id.into()])).await?.as_ref().map(receipt).transpose()
}
pub(super) async fn attempt(
    conn: &DatabaseConnection,
    id: i64,
) -> Result<(FilingReceipt, PreparedIssue), IntakeError> {
    let row = conn
        .query_one(sql(
            "SELECT * FROM ops_intake_filing WHERE id=?",
            vec![id.into()],
        ))
        .await?
        .ok_or(IntakeError::Conflict)?;
    let p: PreparedIssue = serde_json::from_str(&row.try_get::<String>("", "payload_json")?)
        .map_err(|_| IntakeError::StorageUnavailable)?;
    if p.payload_digest()? != row.try_get::<String>("", "payload_digest")? {
        return Err(IntakeError::InvalidPayload);
    }
    Ok((receipt(&row)?, p))
}
pub(super) async fn finish(
    conn: &DatabaseConnection,
    id: i64,
    p: &PreparedIssue,
    issue: Option<CreatedIssue>,
    error: Option<&str>,
    retry_after: Option<i64>,
) -> Result<FilingReceipt, IntakeError> {
    let state = if issue.is_some() {
        "created"
    } else if error.is_some() {
        "failed"
    } else {
        "unknown"
    };
    let txn = conn.begin().await?;
    let changed = txn.execute(sql("UPDATE ops_intake_filing SET state=?,issue_json=?,error_code=?,retry_after=? WHERE id=? AND state='unknown' AND payload_digest=?",
        vec![state.into(), issue.as_ref().map(encoded).transpose()?.into(), error.into(), retry_after.into(), id.into(), p.payload_digest()?.into()])).await?;
    if changed.rows_affected() != 1 {
        return Err(IntakeError::Conflict);
    }
    audit(
        &txn,
        p,
        "github_app",
        "github.filing_outcome",
        serde_json::json!({"attempt_id":id,"state":state,"error_code":error}),
    )
    .await?;
    txn.commit().await?;
    Ok(attempt(conn, id).await?.0)
}
