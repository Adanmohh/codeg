//! Held creation port of Codeg v0.30.4 create_from_forge/insert_todo_row.
//! Initial Canceled state is intentional: the existing scheduler cannot claim it.
use super::{runtime::HostRuntime, store::*, types::*};
use crate::{
    db::service::work_task_service as tasks,
    forge::{
        self,
        envelope::{forge_untrusted_envelope, ForgeSnapshot},
        ForgeProvider, ForgeSourceMeta,
    },
    models::WorkTaskConfig,
    ops::Operator,
    ops_intake,
};
use sea_orm::{ConnectionTrait, DatabaseConnection, TransactionTrait};
use serde_json::json;

/// Shared by creation and the detail projection. A stale link is never exposed
/// after a folder/repository/App rebind, task move, deletion or receipt change.
pub(super) async fn linked<C: ConnectionTrait>(
    db: &C,
    account: i32,
    source: &SourceInput,
) -> Result<Option<i32>, HostError> {
    let p = enabled(db, account, &source.product_id).await?;
    let Some(row) = db
        .query_one(sql(
            "SELECT task_id FROM ops_intake_host_fix WHERE product_id=? AND ulid=?",
            vec![source.product_id.clone().into(), source.ulid.clone().into()],
        ))
        .await?
    else {
        return Ok(None);
    };
    let (_, issue, _) = created(db, &p, source).await?;
    let id = row.try_get("", "task_id")?;
    validate_task(db, account, &p, id, &issue).await?;
    Ok(Some(id))
}

async fn created<C: ConnectionTrait>(
    db: &C,
    p: &StoredProduct,
    source: &SourceInput,
) -> Result<(ops_intake::PreparedIssue, ops_intake::CreatedIssue, i64), HostError> {
    let row = db.query_one(sql(
        "SELECT id,state,payload_json,payload_digest,issue_json FROM ops_intake_filing WHERE source_key=? AND repository_id=? ORDER BY id DESC LIMIT 1",
        vec![source_key(source).into(), p.binding.repository_id.into()],
    )).await?.ok_or(HostError::Conflict)?;
    if row.try_get::<String>("", "state")? != "created" {
        return Err(HostError::Conflict);
    }
    let approved: ops_intake::PreparedIssue = decode(&row.try_get::<String>("", "payload_json")?)?;
    if !approved.matches_binding(&p.binding)?
        || approved.draft.source_ref.product_id != source.product_id
        || approved.draft.source_ref.ulid != source.ulid
        || approved.payload_digest()? != row.try_get::<String>("", "payload_digest")?
    {
        return Err(HostError::Conflict);
    }
    Ok((
        approved,
        decode(&row.try_get::<String>("", "issue_json")?)?,
        row.try_get("", "id")?,
    ))
}

async fn validate_task<C: ConnectionTrait>(
    db: &C,
    account: i32,
    p: &StoredProduct,
    id: i32,
    issue: &ops_intake::CreatedIssue,
) -> Result<(), HostError> {
    let key = forge::source_key(
        "github",
        "github.com",
        &p.binding.full_name,
        "issue",
        issue.number,
    )
    .map_err(|_| HostError::Conflict)?;
    let row = db.query_one(sql(
        "SELECT source_meta FROM work_task WHERE id=? AND folder_id=? AND source_kind='forge_issue' AND source_key=? AND deleted_at IS NULL",
        vec![id.into(), p.binding.folder_id.into(), key.into()],
    )).await?.ok_or(HostError::Conflict)?;
    let meta: ForgeSourceMeta =
        decode(&row.try_get::<String>("", "source_meta")?).map_err(|_| HostError::Conflict)?;
    if meta.provider != ForgeProvider::GitHub
        || meta.server_host != "github.com"
        || !meta.owner_repo.eq_ignore_ascii_case(&p.binding.full_name)
        || meta.number != issue.number
        || meta.url != issue.html_url
    {
        return Err(HostError::Conflict);
    }
    // Tasks themselves are folder-scoped, not account rows. The live bound
    // folder is the authorization boundary; reject another host product's link.
    if db.query_one(sql(
        "SELECT f.task_id FROM ops_intake_host_fix f JOIN ops_intake_host_product p ON p.product_id=f.product_id WHERE f.task_id=? AND (p.account_id<>? OR f.product_id<>?) LIMIT 1",
        vec![id.into(), account.into(), p.binding.product_id.clone().into()],
    )).await?.is_some() { return Err(HostError::Conflict); }
    Ok(())
}

pub async fn create(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    source: SourceInput,
) -> Result<Detail, HostError> {
    let _guard = runtime.guard(&source.product_id).await?;
    let p = enabled(db, op.account_id(), &source.product_id).await?;
    let txn = db.begin().await?;
    // Writer first, then verify live account/folder/binding and the actual receipt.
    txn.execute(sql(
        "UPDATE ops_intake_host_product SET account_id=account_id WHERE product_id=?",
        vec![source.product_id.clone().into()],
    ))
    .await?;
    let current = enabled(&txn, op.account_id(), &source.product_id).await?;
    if current.binding != p.binding {
        return Err(HostError::Conflict);
    }
    let (approved, issue, attempt_id) = created(&txn, &current, &source).await?;
    if linked(&txn, op.account_id(), &source).await?.is_some() {
        txn.rollback().await?;
        return super::operator::detail(db, op, source).await;
    }
    let key = forge::source_key(
        "github",
        "github.com",
        &p.binding.full_name,
        "issue",
        issue.number,
    )
    .map_err(|_| HostError::InvalidInput)?;
    let existing = tasks::other_active_with_same_source(&txn, 0, &key).await?;
    let task_id = if let Some(existing) = existing {
        validate_task(&txn, op.account_id(), &current, existing.id, &issue).await?;
        existing.id
    } else {
        let envelope = forge_untrusted_envelope(
            "github",
            &ForgeSnapshot {
                title: approved.outgoing.title.clone(),
                body: Some(approved.outgoing.body.clone()),
                labels: approved.outgoing.labels,
                author: None,
            },
        );
        let prompt=format!("Review this reported defect and propose a fix plan with meaningful tests. Deliver a report for human review before code changes. Do not send messages, release builds or merge changes.\n\n{envelope}");
        let config = WorkTaskConfig {
            display_text: prompt.clone(),
            prompt_blocks: vec![json!({"type":"text","text":prompt})],
            deliverable: Some(crate::models::DELIVERABLE_REPORT.into()),
            ..Default::default()
        };
        let title = format!("Plan fix: {}", approved.outgoing.title);
        let meta = ForgeSourceMeta {
            provider: ForgeProvider::GitHub,
            server_host: "github.com".into(),
            api_base: "https://api.github.com".into(),
            // Provenance only. This is deliberately not an existing Git PAT ID.
            // writeback=false; later forge delivery needs its own configured identity.
            account_id: format!("ops-intake-app:{}", p.binding.app_id),
            owner_repo: p.binding.full_name.clone(),
            number: issue.number,
            url: issue.html_url,
            title: approved.outgoing.title,
            base_ref: None,
            head_ref: None,
            head_sha: None,
            head_repo: None,
            result_pr: None,
            writeback: Some(false),
        };
        let now = chrono::Utc::now().to_rfc3339();
        // The borrowed row uses the same defaults, provenance and events as
        // insert_todo_row; Canceled + finished_at is committed atomically so
        // even auto_process folders require an explicit manual requeue.
        let created=txn.execute(sql("INSERT INTO work_task(folder_id,title,config,status,run_seq,sort_order,source_kind,source_key,source_meta,created_at,updated_at,finished_at) SELECT ?,?,?,'canceled',0,COALESCE(MAX(sort_order),0)+1,'forge_issue',?,?,?,?,? FROM work_task WHERE folder_id=?",vec![p.binding.folder_id.into(),title.into(),encode(&config)?.into(),key.clone().into(),encode(&meta)?.into(),now.clone().into(),now.clone().into(),now.into(),p.binding.folder_id.into()])).await?;
        let id =
            i32::try_from(created.last_insert_id()).map_err(|_| HostError::StorageUnavailable)?;
        tasks::record_event(&txn, id, "created", op.actor(), None).await?;
        tasks::record_event(
            &txn,
            id,
            "forge_linked",
            op.actor(),
            Some(json!({"source_key":key,"kind":"forge_issue"})),
        )
        .await?;
        tasks::record_event(&txn,id,"user_action",op.actor(),Some(json!({"action":"intake_fix_held","reason":"Proposed fix plan; explicit review and manual requeue required","attempt_id":attempt_id}))).await?;
        id
    };
    txn.execute(sql(
        "INSERT INTO ops_intake_host_fix(product_id,ulid,task_id) VALUES(?,?,?)",
        vec![
            source.product_id.clone().into(),
            source.ulid.clone().into(),
            task_id.into(),
        ],
    ))
    .await?;
    txn.commit().await?;
    super::operator::detail(db, op, source).await
}
