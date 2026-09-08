//! Only the parent-owned pi bridge may construct RunContext. No human powers here.
use super::{store, types::*, HostRuntime};
use crate::{ops::agent::RunContext, ops_intake::PreparedIssue};
use sea_orm::{ConnectionTrait, DatabaseConnection, TransactionTrait};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusInput {}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CachedListInput {
    #[serde(default)]
    pub page: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CachedGetInput {
    pub ulid: String,
}

/// Every result describes local cache state, never a new upstream observation.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheState {
    pub state: &'static str,
    pub verified_at: Option<i64>,
    pub max_age_seconds: i64,
    pub operator_action: Option<&'static str>,
}

fn cache(snapshot: Option<&Snapshot>) -> CacheState {
    let state = match snapshot {
        None => "missing",
        Some(s) if s.error.is_some() => "unavailable",
        Some(s) if s.verified_at.is_none() => "unverified",
        Some(s) if store::fresh(s).is_err() => "expired",
        Some(_) => "fresh",
    };
    CacheState {
        state,
        verified_at: snapshot.and_then(|s| s.verified_at),
        max_age_seconds: crate::ops_intake::SOURCE_MAX_AGE_SECONDS,
        operator_action: match state {
            "fresh" => None,
            "missing" => Some("Ask an operator to import TestFlight feedback in Bug intake."),
            _ => Some("Ask an operator to refresh this source in Bug intake before proposing."),
        },
    }
}

/// Explicit projection: do not serialize Draft (proofs/prepared bindings) or
/// operator Detail (other tasks, receipts, reviewer metadata) to the agent.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicDraft {
    pub draft_id: String,
    pub revision: i32,
    pub title: String,
    pub labels: Vec<String>,
    pub confirmed_severity: Option<Severity>,
    pub matches_source_revision: bool,
    pub prepared_for_current_run: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedFeedback {
    pub ulid: String,
    pub title: String,
    pub cache: CacheState,
    pub draft: Option<PublicDraft>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedPage {
    pub product_id: String,
    pub items: Vec<CachedFeedback>,
    pub next_page: Option<u32>,
    pub semantics: &'static str,
    pub operator_action: Option<&'static str>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicFeedback {
    pub description: String,
    pub source_status: String,
    pub submitted_at: Option<String>,
    pub device: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub build_number: Option<String>,
    pub platform: Option<String>,
    pub locale: Option<String>,
    pub suggested_tags: Vec<String>,
    pub suggested_severity: Severity,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedDetail {
    pub product_id: String,
    pub item: CachedFeedback,
    pub feedback: Option<PublicFeedback>,
    pub semantics: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CachedStatus {
    pub product_id: String,
    pub cached_records: i64,
    pub in_app_available: bool,
    pub semantics: &'static str,
    pub operator_action: &'static str,
}

const SEMANTICS: &str = "Cached TestFlight data only. No upstream request or freshness update. Triage suggestions are guesses; a human must confirm severity and evidence.";

/// Resolve the unique product inside the same snapshot as task/run liveness.
/// Reuses Ops' accepted validator; no operator impersonation or ACP-side SQL.
async fn product_for<C: ConnectionTrait>(db: &C, ctx: &RunContext) -> Result<String, HostError> {
    crate::ops::agent::require_live(db, ctx).await?;
    let rows = db.query_all(store::sql(
        "SELECT p.product_id FROM ops_intake_host_product p JOIN work_task w ON w.id=? WHERE p.account_id=? AND json_extract(p.config_json,'$.binding.folder_id')=w.folder_id AND json_extract(p.config_json,'$.binding.enabled')=1 ORDER BY p.product_id LIMIT 2",
        vec![ctx.task_id.into(), ctx.account_id.into()],
    )).await?;
    let row = match rows.as_slice() {
        [] => return Err(HostError::NotConfigured),
        [row] => row,
        _ => return Err(HostError::AmbiguousProduct),
    };
    let id: String = row.try_get("", "product_id")?;
    store::enabled(db, ctx.account_id, &id).await?;
    Ok(id)
}

async fn public_draft<C: ConnectionTrait>(
    db: &C,
    ctx: &RunContext,
    source: &SourceInput,
    revision: &str,
) -> Result<Option<PublicDraft>, HostError> {
    // A read must not call ensure_draft: only human flows create/reset drafts.
    let Some(row) = db
        .query_one(store::sql(
            "SELECT draft_json FROM ops_intake_host_draft WHERE product_id=? AND ulid=?",
            vec![source.product_id.clone().into(), source.ulid.clone().into()],
        ))
        .await?
    else {
        return Ok(None);
    };
    let draft: Draft = store::decode(&row.try_get::<String>("", "draft_json")?)?;
    Ok(Some(PublicDraft {
        prepared_for_current_run: draft.prepared.as_ref().is_some_and(|p| {
            p.draft.task_id == ctx.task_id
                && p.draft.run_seq == ctx.run_seq
                && p.draft.source_ref.product_id == source.product_id
                && p.draft.source_ref.ulid == source.ulid
                && p.draft.source_revision == revision
        }),
        matches_source_revision: draft.source_revision == revision,
        draft_id: draft.id,
        revision: draft.revision,
        title: draft.title,
        labels: draft.labels,
        confirmed_severity: draft.confirmed_severity,
    }))
}

pub async fn cached_status(
    db: &DatabaseConnection,
    ctx: &RunContext,
    _: StatusInput,
) -> Result<CachedStatus, HostError> {
    let txn = db.begin().await?;
    let product_id = product_for(&txn, ctx).await?;
    let count = txn
        .query_one(store::sql(
            "SELECT COUNT(*) AS n FROM ops_intake_host_snapshot WHERE product_id=?",
            vec![product_id.clone().into()],
        ))
        .await?
        .ok_or(HostError::StorageUnavailable)?;
    let result = CachedStatus { product_id, cached_records: count.try_get("", "n")?, in_app_available: false,
        semantics: SEMANTICS, operator_action: "An operator imports or refreshes sources in Bug intake. Use cached get for each source's freshness and human-prepared draft revision." };
    txn.commit().await?;
    Ok(result)
}

pub async fn cached_list(
    db: &DatabaseConnection,
    ctx: &RunContext,
    input: CachedListInput,
) -> Result<CachedPage, HostError> {
    if input.page > 10000 {
        return Err(HostError::InvalidInput);
    }
    let txn = db.begin().await?;
    let product_id = product_for(&txn, ctx).await?;
    let rows = txn.query_all(store::sql("SELECT ulid FROM ops_intake_host_snapshot WHERE product_id=? ORDER BY ulid LIMIT 11 OFFSET ?",
        vec![product_id.clone().into(), (i64::from(input.page) * 10).into()])).await?;
    let page_limit = rows.len() > 10 && input.page == 10000;
    let next_page = (rows.len() > 10 && !page_limit).then_some(input.page + 1);
    let mut items = Vec::new();
    for row in rows.iter().take(10) {
        let source = SourceInput {
            product_id: product_id.clone(),
            ulid: row.try_get("", "ulid")?,
        };
        let s = store::snapshot(&txn, &source).await?;
        store::validate_record(&s.record, &product_id)?;
        let draft = public_draft(&txn, ctx, &source, &s.record.source_revision).await?;
        items.push(CachedFeedback {
            ulid: source.ulid,
            cache: cache(Some(&s)),
            title: s.record.title,
            draft,
        });
    }
    let result = CachedPage {
        product_id,
        next_page,
        semantics: SEMANTICS,
        operator_action: if page_limit {
            Some("The local page limit was reached; more cached records exist. Ask an operator to inspect Bug intake.")
        } else {
            items.is_empty().then_some("No cached records on this page. An operator can import TestFlight feedback in Bug intake; this tool does not fetch it.")
        },
        items,
    };
    txn.commit().await?;
    Ok(result)
}

pub async fn cached_get(
    db: &DatabaseConnection,
    ctx: &RunContext,
    input: CachedGetInput,
) -> Result<CachedDetail, HostError> {
    let txn = db.begin().await?;
    let product_id = product_for(&txn, ctx).await?;
    crate::ops_intake::SourceRef {
        product_id: product_id.clone(),
        source: crate::ops_intake::SourceKind::Testflight,
        ulid: input.ulid.clone(),
    }
    .validate()?;
    let source = SourceInput {
        product_id: product_id.clone(),
        ulid: input.ulid,
    };
    let snapshot = match store::snapshot(&txn, &source).await {
        Ok(snapshot) => Some(snapshot),
        Err(HostError::SourceUnavailable) => None,
        Err(e) => return Err(e),
    };
    let mut item = CachedFeedback {
        ulid: source.ulid.clone(),
        title: String::new(),
        cache: cache(snapshot.as_ref()),
        draft: None,
    };
    let feedback = if let Some(s) = snapshot {
        store::validate_record(&s.record, &product_id)?;
        item.draft = public_draft(&txn, ctx, &source, &s.record.source_revision).await?;
        let r = s.record;
        item.title = r.title;
        Some(PublicFeedback {
            description: r.description,
            source_status: r.source_status,
            submitted_at: r.submitted_at,
            device: r.device,
            os_version: r.os_version,
            app_version: r.app_version,
            build_number: r.build_number,
            platform: r.platform,
            locale: r.locale,
            suggested_tags: r.triage.seeded_tags,
            suggested_severity: r.triage.seeded_severity,
        })
    } else {
        None
    };
    txn.commit().await?;
    Ok(CachedDetail {
        product_id,
        item,
        feedback,
        semantics: SEMANTICS,
    })
}

#[derive(Serialize)]
pub struct Proposed {
    pub status: &'static str,
    pub proposal_id: Option<i32>,
    pub prepared: PreparedIssue,
}

pub async fn prepare_and_propose(
    db: &DatabaseConnection,
    ctx: &RunContext,
    draft_id: &str,
    expected_revision: i32,
) -> Result<Proposed, HostError> {
    if !store::identifier(draft_id) || expected_revision <= 0 {
        return Err(HostError::InvalidInput);
    }
    let txn = db.begin().await?;
    let product = product_for(&txn, ctx).await?;
    if txn
        .query_one(store::sql(
            "SELECT id FROM ops_intake_host_draft WHERE id=? AND product_id=?",
            vec![draft_id.into(), product.into()],
        ))
        .await?
        .is_none()
    {
        return Err(HostError::AccessDenied);
    }
    txn.commit().await?;
    super::review::propose(
        db,
        ctx,
        draft_id,
        expected_revision,
        &HostRuntime::production(),
    )
    .await
}

#[cfg(test)]
pub(crate) mod tests;
