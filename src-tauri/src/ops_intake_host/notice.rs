//! Closed read-only host projection. No Operator fabrication, network or writes.
use super::{store::*, types::*};
use crate::{
    ops::Operator,
    ops_intake::{self, PreparedIssue, RepositoryBinding},
};
use sea_orm::ConnectionTrait;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub(crate) struct SnapshotBinding {
    pub proposal_id: i32,
    pub task_id: i32,
    pub run_seq: i32,
    pub sha256: String,
}

#[derive(Serialize)]
pub struct IssueReview {
    pub source: SourceInput,
    pub binding: RepositoryBinding,
    pub detail: Detail,
}

async fn current<C: ConnectionTrait>(
    db: &C,
    account: i32,
    id: i32,
) -> Result<(SnapshotBinding, IssueReview), HostError> {
    let row = db.query_one(sql(
        "SELECT payload_json,task_id,run_seq FROM ops_proposal WHERE id=? AND action_name='github.create_issue' AND status='pending'",
        vec![id.into()],
    )).await?.ok_or(HostError::Conflict)?;
    let p: PreparedIssue = serde_json::from_str(&row.try_get::<String>("", "payload_json")?)
        .map_err(|_| HostError::InvalidInput)?;
    let source = SourceInput {
        product_id: p.draft.source_ref.product_id.clone(),
        ulid: p.draft.source_ref.ulid.clone(),
    };
    let product = enabled(db, account, &source.product_id).await?;
    let snapshot = super::store::snapshot(db, &source).await?;
    fresh(&snapshot)?;
    let d = draft(db, &source).await?;
    if snapshot.record.source_ref.source() != p.draft.source_ref
        || snapshot.record.source_revision != p.draft.source_revision
        || d.source_revision != p.draft.source_revision
        || d.prepared.as_ref() != Some(&p)
        || p.draft.task_id != row.try_get::<i32>("", "task_id")?
        || p.draft.run_seq != row.try_get::<i32>("", "run_seq")?
        || !p.matches_binding(&product.binding)?
    {
        return Err(HostError::Conflict);
    }
    // The accepted validator hashes every stored proof byte, checks revocation,
    // source freshness and the complete repository/App/installation binding.
    ops_intake::validate_prepared(db, &p).await?;
    let task = db.query_one(sql(
        "SELECT connection_id FROM work_task WHERE id=? AND run_seq=? AND folder_id=? AND status='awaiting_input' AND deleted_at IS NULL",
        vec![p.draft.task_id.into(), p.draft.run_seq.into(), product.binding.folder_id.into()],
    )).await?.ok_or(HostError::TaskRequired)?;
    let connection: Option<String> = task.try_get("", "connection_id")?;
    let stored = db.query_one(sql(
        "SELECT d.id,d.revision,COALESCE(b.revision,0) AS binding_revision FROM ops_intake_host_draft d LEFT JOIN ops_telegram_issue_binding b ON b.product_id=d.product_id WHERE d.product_id=? AND d.ulid=?",
        vec![source.product_id.clone().into(), source.ulid.clone().into()],
    )).await?.ok_or(HostError::Conflict)?;
    if stored.try_get::<String>("", "id")? != d.id
        || stored.try_get::<i32>("", "revision")? != d.revision
    {
        return Err(HostError::Conflict);
    }
    let handoff_unknown = db.query_one(sql(
        "SELECT proposal_id FROM ops_intake_host_handoff WHERE product_id=? AND ulid=? AND state='unknown' LIMIT 1",
        vec![source.product_id.clone().into(),source.ulid.clone().into()],
    )).await?.is_some();
    // Bind all host draft metadata too: saving an identical payload is still a
    // new review revision. Configuration epochs also prevent rebind/restore ABA.
    let bytes = serde_json::to_vec(&(
        account,
        id,
        &connection,
        &product,
        stored.try_get::<i64>("", "binding_revision")?,
        &d,
    ))
    .map_err(|_| HostError::StorageUnavailable)?;
    let binding = SnapshotBinding {
        proposal_id: id,
        task_id: p.draft.task_id,
        run_seq: p.draft.run_seq,
        sha256: format!("{:x}", Sha256::digest(bytes)),
    };
    Ok((
        binding,
        IssueReview {
            source,
            binding: product.binding,
            detail: Detail {
                snapshot,
                draft: d,
                tasks: vec![],
                proposals: vec![Proposal {
                    id,
                    status: "pending".into(),
                    payload: Some(p),
                    stale: false,
                }],
                receipt: None,
                handoff_unknown,
                fix_task_id: None,
                fix_task_conflict: false,
            },
        },
    ))
}

pub(crate) async fn snapshot<C: ConnectionTrait>(
    db: &C,
    account: i32,
    id: i32,
) -> Result<Option<SnapshotBinding>, HostError> {
    match current(db, account, id).await {
        Ok((binding, _)) => Ok(Some(binding)),
        Err(HostError::StorageUnavailable) => Err(HostError::StorageUnavailable),
        Err(_) => Ok(None),
    }
}

pub(crate) async fn projection<C: ConnectionTrait>(
    db: &C,
    op: &Operator,
    id: i32,
    expected: &str,
) -> Result<Option<IssueReview>, HostError> {
    match current(db, op.account_id(), id).await {
        Ok((binding, review)) if binding.sha256 == expected && !review.detail.handoff_unknown => {
            Ok(Some(review))
        }
        Err(HostError::StorageUnavailable) => Err(HostError::StorageUnavailable),
        _ => Ok(None),
    }
}
