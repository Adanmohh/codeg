//! Fixed-workspace observations. Source IDs and revisions are server-owned;
//! listing never imports or publishes bytes. See NOTICE for the intake fence port.
use super::{common::*, files, records, scope, types::*, validation};
use crate::business_identity::{self as identity, Principal};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Semaphore;

pub(super) static IO_SLOTS: Semaphore = Semaphore::const_new(2);
const MAX_CANDIDATES: usize = 512;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Source {
    pub session_id: String,
    pub task_id: String,
    pub session_fence: i64,
    pub generation: i64,
    pub admission_id: String,
    pub inspect_workspace: bool,
}
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Observed {
    pub available: bool,
    pub file: files::Observation,
}

pub(super) async fn source(
    tx: &DatabaseTransaction,
    principal: &Principal,
    session_id: &str,
) -> Result<Source> {
    let session = scope::session(tx, principal, session_id).await?;
    if session.status == "starting" {
        return Err(OperationReason::Busy.into());
    }
    // Stop may have fenced the live generation already. Its last actually bound
    // workspace remains private/readable while the original authority is current.
    // A new generation cannot silently reuse observations from that old workspace.
    let row = tx.query_one(statement(
        "SELECT generation,admission_id FROM business_execution_generation WHERE organization_id=? AND session_id=? AND generation<=? AND engine_json IS NOT NULL ORDER BY generation DESC LIMIT 1",
        vec![principal.organization_id().into(), session_id.into(), session.generation.into()],
    )).await?.ok_or(OperationReason::Unavailable)?;
    Ok(Source {
        session_id: session.id,
        task_id: session.task_id,
        session_fence: session.generation,
        generation: row.try_get("", "generation")?,
        admission_id: row.try_get("", "admission_id")?,
        inspect_workspace: matches!(
            session.status.as_str(),
            "idle" | "running" | "awaiting_input"
        ),
    })
}
pub(super) async fn revalidate(
    tx: &DatabaseTransaction,
    principal: &Principal,
    expected: &Source,
) -> Result<()> {
    if source(tx, principal, &expected.session_id).await? != *expected {
        return Err(OperationReason::Conflict.into());
    }
    Ok(())
}
pub(super) async fn observation(
    tx: &DatabaseTransaction,
    principal: &Principal,
    source: &Source,
    output_id: &str,
) -> Result<(records::Output, Observed)> {
    revalidate(tx, principal, source).await?;
    let row = records::Output::find_by_statement(statement(
        "SELECT * FROM business_execution_output WHERE organization_id=? AND session_id=? AND generation=? AND id=?",
        vec![principal.organization_id().into(), source.session_id.clone().into(), source.generation.into(), output_id.into()],
    )).one(tx).await?.ok_or(OperationReason::Missing)?;
    let observed = decode(&row.metadata_json)?;
    Ok((row, observed))
}
fn next(revision: i64) -> Result<i64> {
    let next = revision.checked_add(1).ok_or(OperationReason::Conflict)?;
    validation::revision(next)?;
    Ok(next)
}
async fn retain(
    tx: &DatabaseTransaction,
    principal: &Principal,
    source: &Source,
    found: Vec<files::Candidate>,
) -> Result<()> {
    let old = records::Output::find_by_statement(statement(
        "SELECT * FROM business_execution_output WHERE organization_id=? AND session_id=? AND generation=? ORDER BY id LIMIT 513",
        vec![principal.organization_id().into(), source.session_id.clone().into(), source.generation.into()],
    )).all(tx).await?;
    let mut remaining: HashMap<_, _> = old
        .into_iter()
        .map(|row| (row.relative_path.clone(), row))
        .collect();
    let mut retained = remaining.len();
    if retained > MAX_CANDIDATES || found.len() > MAX_CANDIDATES {
        return Err(OperationReason::RateLimited.into());
    }
    for candidate in found {
        let observed = Observed {
            available: true,
            file: candidate.observation,
        };
        // Validate a representable UTC timestamp before retaining/exposing a row.
        observed.file.modified_at()?;
        let json = encode(&observed)?;
        if let Some(old) = remaining.remove(&candidate.relative) {
            let before: Observed = decode(&old.metadata_json)?;
            if before != observed {
                let changed = tx.execute(statement(
                    "UPDATE business_execution_output SET revision=?,metadata_json=? WHERE organization_id=? AND session_id=? AND generation=? AND id=? AND revision=?",
                    vec![next(old.revision)?.into(), json.into(), principal.organization_id().into(), source.session_id.clone().into(), source.generation.into(), old.id.into(), old.revision.into()],
                )).await?.rows_affected();
                if changed != 1 {
                    return Err(OperationReason::Conflict.into());
                }
            }
        } else {
            retained += 1;
            if retained > MAX_CANDIDATES {
                return Err(OperationReason::RateLimited.into());
            }
            tx.execute(statement(
                "INSERT INTO business_execution_output(id,organization_id,session_id,generation,revision,relative_path,metadata_json) VALUES(?,?,?,?,1,?,?)",
                vec![id().into(), principal.organization_id().into(), source.session_id.clone().into(), source.generation.into(), candidate.relative.into(), json.into()],
            )).await?;
        }
    }
    for old in remaining.into_values() {
        let mut observed: Observed = decode(&old.metadata_json)?;
        if observed.available {
            observed.available = false;
            let changed = tx.execute(statement(
                "UPDATE business_execution_output SET revision=?,metadata_json=? WHERE organization_id=? AND session_id=? AND generation=? AND id=? AND revision=?",
                vec![next(old.revision)?.into(), encode(&observed)?.into(), principal.organization_id().into(), source.session_id.clone().into(), source.generation.into(), old.id.into(), old.revision.into()],
            )).await?.rows_affected();
            if changed != 1 {
                return Err(OperationReason::Conflict.into());
            }
        }
    }
    Ok(())
}
fn view(row: records::Output) -> Result<OutputCandidate> {
    let observed: Observed = decode(&row.metadata_json)?;
    let name = row
        .relative_path
        .rsplit('/')
        .next()
        .ok_or(OperationReason::Unavailable)?;
    Ok(OutputCandidate {
        id: row.id,
        revision: row.revision,
        name: name.into(),
        media_type: observed.file.media_type.clone(),
        byte_size: observed.file.byte_size,
        modified_at: observed.file.modified_at()?,
        status: if observed.available {
            OutputStatus::Available
        } else {
            OutputStatus::Changed
        },
    })
}
async fn cursor(
    tx: &DatabaseTransaction,
    principal: &Principal,
    source: &Source,
    value: Option<&str>,
) -> Result<()> {
    if let Some(value) = value {
        validation::uuid(value)?;
        observation(tx, principal, source, value).await?;
    }
    Ok(())
}
async fn revisions(
    tx: &DatabaseTransaction,
    principal: &Principal,
    source: &Source,
) -> Result<Vec<(String, i64)>> {
    let rows = tx.query_all(statement(
        "SELECT id,revision FROM business_execution_output WHERE organization_id=? AND session_id=? AND generation=? ORDER BY id LIMIT 513",
        vec![principal.organization_id().into(), source.session_id.clone().into(), source.generation.into()],
    )).await?;
    if rows.len() > MAX_CANDIDATES {
        return Err(OperationReason::RateLimited.into());
    }
    rows.into_iter()
        .map(|row| Ok((row.try_get("", "id")?, row.try_get("", "revision")?)))
        .collect()
}
pub(super) async fn list(
    db: &DatabaseConnection,
    files: &files::Files,
    principal: &Principal,
    input: OutputListInput,
) -> Result<Page<OutputCandidate>> {
    list_with(db, files, principal, input, std::future::ready(())).await
}
async fn list_with(
    db: &DatabaseConnection,
    files: &files::Files,
    principal: &Principal,
    input: OutputListInput,
    before_commit: impl std::future::Future<Output = ()>,
) -> Result<Page<OutputCandidate>> {
    validation::uuid(&input.session_id)?;
    validation::page(input.limit, 50, input.cursor.as_deref())?;
    let (expected, before) = {
        let tx = db.begin().await?;
        let expected = source(&tx, principal, &input.session_id).await?;
        cursor(&tx, principal, &expected, input.cursor.as_deref()).await?;
        let before = revisions(&tx, principal, &expected).await?;
        (expected, before)
    };
    // Ended runs expose only observations captured while active. Scanning them
    // again could promote late bytes written after the stop fence. Import still
    // compares the exact old observation against the source before retaining it.
    let found = if expected.inspect_workspace {
        let permit = IO_SLOTS
            .acquire()
            .await
            .map_err(|_| OperationReason::Unavailable)?;
        let files = files.clone();
        let admission_id = expected.admission_id.clone();
        Some(
            tokio::task::spawn_blocking(move || {
                let _permit = permit;
                files.scan(&admission_id)
            })
            .await
            .map_err(|_| OperationReason::Unavailable)??,
        )
    } else {
        None
    };
    before_commit.await;
    let tx = identity::begin_write(db, principal.organization_id()).await?;
    revalidate(&tx, principal, &expected).await?;
    // An older scan must not replace observations committed by a later scan.
    // Compare the bounded generation snapshot in the same writer as all updates.
    if revisions(&tx, principal, &expected).await? != before {
        return Err(OperationReason::Conflict.into());
    }
    if let Some(found) = found {
        retain(&tx, principal, &expected, found).await?;
    }
    cursor(&tx, principal, &expected, input.cursor.as_deref()).await?;
    let mut rows = records::Output::find_by_statement(statement(
        "SELECT * FROM business_execution_output WHERE organization_id=? AND session_id=? AND generation=? AND (? IS NULL OR id<?) ORDER BY id DESC LIMIT ?",
        vec![principal.organization_id().into(), expected.session_id.into(), expected.generation.into(), input.cursor.clone().into(), input.cursor.into(), (i64::from(input.limit)+1).into()],
    )).all(&tx).await?;
    let more = rows.len() > input.limit as usize;
    rows.truncate(input.limit as usize);
    let next_cursor = if more {
        rows.last().map(|r| r.id.clone())
    } else {
        None
    };
    let items = rows.into_iter().map(view).collect::<Result<Vec<_>>>()?;
    tx.commit().await?;
    Ok(Page { items, next_cursor })
}

#[cfg(all(test, unix))]
mod tests;
