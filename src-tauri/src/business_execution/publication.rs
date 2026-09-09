//! Explicit human disclosure of exact managed versions. File verification occurs
//! outside the writer; current authority, selection, task CAS and receipt commit
//! together through the task-owned helper. No external publish or agent approval.
use super::{common::*, files::Files, receipts, records, scope, types::*, validation};
use crate::{
    business_identity::{self as identity, Principal},
    business_tasks::{self as tasks, store::managed},
};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult, TransactionTrait,
};
use tokio::sync::Semaphore;

// Bound blocking hashing independently of request cancellation. A permit is
// retained by the blocking job until the last bounded file check has finished.
static VERIFY_SLOTS: Semaphore = Semaphore::const_new(2);

#[derive(Clone, PartialEq, Eq)]
struct Retained {
    asset_id: String,
    version_id: String,
    object_id: String,
    sha256: String,
    byte_size: i64,
}

async fn selected(
    tx: &DatabaseTransaction,
    principal: &Principal,
    task_id: &str,
    selection: &VersionSelection,
) -> Result<Retained> {
    scope::task(tx, principal, task_id, false).await?;
    let row = records::Version::find_by_statement(statement(
        "SELECT v.* FROM business_execution_asset_version v JOIN business_execution_asset a ON a.organization_id=v.organization_id AND a.task_id=v.task_id AND a.id=v.asset_id WHERE v.organization_id=? AND v.task_id=? AND v.asset_id=? AND v.id=?",
        vec![principal.organization_id().into(), task_id.into(),
            selection.asset_id.clone().into(), selection.version_id.clone().into()],
    )).one(tx).await?.ok_or(OperationReason::Missing)?;
    let published = tx.query_one(statement(
        "SELECT deliverable_id FROM business_execution_publication WHERE organization_id=? AND task_id=? AND asset_id=? AND version_id=? LIMIT 1",
        vec![principal.organization_id().into(), task_id.into(), row.asset_id.clone().into(), row.id.clone().into()],
    )).await?.is_some();
    if !published {
        if row.created_by != principal.member_id() {
            return Err(OperationReason::Missing.into());
        }
        let producer = scope::session(tx, principal, &row.producer_session_id).await?;
        if producer.task_id != task_id {
            return Err(OperationReason::Missing.into());
        }
    }
    Ok(Retained {
        asset_id: row.asset_id,
        version_id: row.id,
        object_id: row.object_id,
        sha256: row.sha256,
        byte_size: row.byte_size,
    })
}

/// Called by the task-owned writer, not a caller-supplied disclosure capability.
pub(crate) async fn validate_selection(
    tx: &DatabaseTransaction,
    principal: &Principal,
    task_id: &str,
    selection: &VersionSelection,
) -> Result<()> {
    selected(tx, principal, task_id, selection).await?;
    Ok(())
}

async fn manifest(
    tx: &DatabaseTransaction,
    principal: &Principal,
    input: &SubmitInput,
) -> Result<Vec<Retained>> {
    let mut retained = Vec::with_capacity(input.versions.len());
    for selection in &input.versions {
        retained.push(selected(tx, principal, &input.task_id, selection).await?);
    }
    Ok(retained)
}

async fn replay(
    tx: &DatabaseTransaction,
    principal: &Principal,
    input: &SubmitInput,
) -> Result<Option<SubmitResult>> {
    let Some(row) = receipts::replay(
        tx,
        principal,
        OperationKind::Submit,
        &input.operation_id,
        input,
    )
    .await?
    else {
        return Ok(None);
    };
    let deliverable_id = row
        .resource_id
        .as_deref()
        .ok_or(OperationReason::Unavailable)?;
    receipts::require_target(
        &row,
        &receipts::Target {
            task_id: &input.task_id,
            session_id: None,
            generation: None,
        },
        deliverable_id,
    )?;
    if row.status != "confirmed" {
        // Submission is wholly transactional; an unknown/incomplete receipt can
        // never authorize another attempt to insert a deliverable.
        return Err(OperationReason::Conflict.into());
    }
    let detail = tasks::store::get_in_transaction(
        tx,
        &tasks::ActorContext::authenticated(principal.clone()),
        &input.task_id,
    )
    .await?;
    let delivered = detail
        .deliverables
        .iter()
        .find(|d| d.id == deliverable_id)
        .ok_or(OperationReason::Missing)?;
    if delivered.body != input.body
        || delivered.assets.len() != input.versions.len()
        || delivered
            .assets
            .iter()
            .zip(&input.versions)
            .any(|(a, b)| a.asset_id != b.asset_id || a.version_id != b.version_id)
    {
        return Err(OperationReason::Conflict.into());
    }
    Ok(Some(SubmitResult {
        detail,
        operation: row.summary()?,
    }))
}

pub(crate) async fn submit(
    db: &DatabaseConnection,
    files: &Files,
    principal: &Principal,
    input: SubmitInput,
) -> Result<SubmitResult> {
    submit_with(db, files, principal, input, std::future::ready(())).await
}

// The production path has no interposed action. Child tests can pause exactly
// after bounded file verification to exercise real writer revalidation/abort.
async fn submit_with(
    db: &DatabaseConnection,
    files: &Files,
    principal: &Principal,
    input: SubmitInput,
    before_commit: impl std::future::Future<Output = ()>,
) -> Result<SubmitResult> {
    validation::submit(&input)?;
    let tx = db.begin().await?;
    scope::operator(&tx, principal).await?;
    if let Some(result) = replay(&tx, principal, &input).await? {
        tx.commit().await?;
        return Ok(result);
    }
    let current = scope::task(&tx, principal, &input.task_id, true).await?;
    if current.task.revision != input.expected_task_revision {
        return Err(OperationReason::Conflict.into());
    }
    let retained = manifest(&tx, principal, &input).await?;
    tx.commit().await?;
    let permit = VERIFY_SLOTS
        .acquire()
        .await
        .map_err(|_| OperationReason::Unavailable)?;
    let custody = files.clone();
    let to_verify = retained.clone();
    tokio::task::spawn_blocking(move || -> Result<()> {
        let _permit = permit;
        for version in to_verify {
            // Each file is bounded to50MiB and discarded before checking the
            // next one. This read cannot write a receipt after cancellation.
            custody.content(&version.object_id, &version.sha256, version.byte_size)?;
        }
        Ok(())
    })
    .await
    .map_err(|_| OperationReason::ContentUnavailable)??;
    before_commit.await;
    commit_verified(db, principal, input, &retained).await
}

async fn commit_verified(
    db: &DatabaseConnection,
    principal: &Principal,
    input: SubmitInput,
    retained: &[Retained],
) -> Result<SubmitResult> {
    let tx = identity::begin_write(db, principal.organization_id()).await?;
    scope::operator(&tx, principal).await?;
    if let Some(result) = replay(&tx, principal, &input).await? {
        tx.commit().await?;
        return Ok(result);
    }
    if manifest(&tx, principal, &input).await? != retained {
        return Err(OperationReason::ContentChanged.into());
    }
    let deliverable_id = id();
    receipts::reserve(
        &tx,
        principal,
        OperationKind::Submit,
        &input.operation_id,
        &input,
        receipts::Target {
            task_id: &input.task_id,
            session_id: None,
            generation: None,
        },
        &deliverable_id,
    )
    .await?;
    let detail = managed::submit_in_transaction(
        &tx,
        &tasks::ActorContext::authenticated(principal.clone()),
        &input,
        &deliverable_id,
    )
    .await?;
    receipts::complete(
        &tx,
        principal,
        OperationKind::Submit,
        &input.operation_id,
        receipts::Completion {
            target: receipts::Target {
                task_id: &input.task_id,
                session_id: None,
                generation: None,
            },
            resource_id: &deliverable_id,
            status: OperationStatus::Confirmed,
            reason: None,
            result: Some(&deliverable_id),
        },
    )
    .await?;
    tx.commit().await?;
    Ok(SubmitResult {
        detail,
        operation: OperationSummary {
            id: input.operation_id,
            status: OperationStatus::Confirmed,
            reason: None,
        },
    })
}

/// Public task-reader projection; private E1 operator authority is not borrowed.
pub(crate) async fn published(
    db: &DatabaseConnection,
    principal: &Principal,
    input: PublishedInput,
) -> Result<PublishedVersion> {
    let tx = db.begin().await?;
    let read = managed::read_in_transaction(
        &tx,
        &tasks::ActorContext::authenticated(principal.clone()),
        &input,
    )
    .await?;
    tx.commit().await?;
    Ok(read.metadata)
}

/// Transport-neutral bytes. HTTP framing/native encoding must still apply their
/// final current-identity check; neither serializes a storage path or object ID.
pub(crate) struct Content {
    pub metadata: ContentMetadata,
    pub bytes: Vec<u8>,
}

fn file_name(title: &str, media_type: &str) -> Result<String> {
    let extension = match media_type {
        "text/plain" => "txt",
        "application/json" => "json",
        "application/pdf" => "pdf",
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => "docx",
        "application/vnd.openxmlformats-officedocument.presentationml.presentation" => "pptx",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => "xlsx",
        _ => return Err(OperationReason::ContentUnavailable.into()),
    };
    let mut name: String = title
        .chars()
        .take(240)
        .map(|c| {
            if c.is_control() || matches!(c, '/' | '\\' | '"' | ';' | '%' | ':') {
                '_'
            } else {
                c
            }
        })
        .collect();
    if name.trim().is_empty() || matches!(name.trim(), "." | "..") {
        name = "document".into();
    }
    if !name
        .to_ascii_lowercase()
        .ends_with(&format!(".{extension}"))
    {
        name.push('.');
        name.push_str(extension);
    }
    Ok(name)
}

pub(crate) async fn published_content(
    db: &DatabaseConnection,
    files: &Files,
    principal: &Principal,
    input: PublishedContentInput,
) -> Result<Content> {
    published_content_with(db, files, principal, input, std::future::ready(())).await
}

async fn published_content_with(
    db: &DatabaseConnection,
    files: &Files,
    principal: &Principal,
    input: PublishedContentInput,
    before_revalidate: impl std::future::Future<Output = ()>,
) -> Result<Content> {
    let reference = PublishedInput {
        task_id: input.task_id,
        deliverable_id: input.deliverable_id,
        asset_id: input.asset_id,
        version_id: input.version_id,
    };
    let ctx = tasks::ActorContext::authenticated(principal.clone());
    let tx = db.begin().await?;
    let observed = managed::read_in_transaction(&tx, &ctx, &reference).await?;
    let version = &observed.metadata.version;
    if input.disposition == Disposition::Preview
        && !matches!(
            version.media_type.as_str(),
            "text/plain" | "application/json"
        )
    {
        // No active office/PDF/image renderer or legacy watch process is enabled
        // by a read. The exact retained file remains available for download.
        return Err(OperationReason::Unavailable.into());
    }
    let metadata = ContentMetadata {
        media_type: version.media_type.clone(),
        byte_size: version.byte_size,
        sha256: version.sha256.clone(),
        file_name: file_name(&version.title, &version.media_type)?,
    };
    tx.commit().await?;
    let permit = VERIFY_SLOTS
        .acquire()
        .await
        .map_err(|_| OperationReason::Unavailable)?;
    let custody = files.clone();
    let object_id = observed.object_id.clone();
    let hash = metadata.sha256.clone();
    let size = metadata.byte_size;
    let content = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        custody.content(&object_id, &hash, size)
    })
    .await
    .map_err(|_| OperationReason::ContentUnavailable)??;
    before_revalidate.await;
    let tx = db.begin().await?;
    let current = managed::read_in_transaction(&tx, &ctx, &reference).await?;
    if current.object_id != observed.object_id
        || current.metadata.version.sha256 != metadata.sha256
        || current.metadata.version.byte_size != metadata.byte_size
    {
        return Err(OperationReason::ContentChanged.into());
    }
    tx.commit().await?;
    Ok(Content {
        metadata,
        bytes: content.bytes,
    })
}

#[cfg(all(test, unix))]
mod tests;
