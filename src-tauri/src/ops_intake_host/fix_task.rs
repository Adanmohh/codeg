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

pub async fn create(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    source: SourceInput,
) -> Result<Detail, HostError> {
    let _guard = runtime.guard(&source.product_id).await?;
    let p = enabled(db, op.account_id(), &source.product_id).await?;
    let snap = snapshot(db, &source).await?;
    let receipt = ops_intake::filing_status(
        db,
        &snap.record.source_ref.source(),
        p.binding.repository_id,
    )
    .await?
    .filter(|r| r.state == ops_intake::FilingState::Created)
    .ok_or(HostError::Conflict)?;
    let issue = receipt.issue.ok_or(HostError::Conflict)?;
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
    if txn.query_one(sql("SELECT id FROM ops_intake_filing WHERE id=? AND state='created' AND source_key=? AND repository_id=?",vec![receipt.attempt_id.into(),source_key(&source).into(),p.binding.repository_id.into()])).await?.is_none() {return Err(HostError::Conflict);}
    if txn
        .query_one(sql(
            "SELECT task_id FROM ops_intake_host_fix WHERE product_id=? AND ulid=?",
            vec![source.product_id.clone().into(), source.ulid.clone().into()],
        ))
        .await?
        .is_some()
    {
        txn.rollback().await?;
        return super::operator::detail(db, op, source).await;
    }
    let row = txn
        .query_one(sql(
            "SELECT payload_json FROM ops_intake_filing WHERE id=?",
            vec![receipt.attempt_id.into()],
        ))
        .await?
        .ok_or(HostError::Conflict)?;
    let approved: ops_intake::PreparedIssue = decode(&row.try_get::<String>("", "payload_json")?)?;
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
        tasks::record_event(&txn,id,"user_action",op.actor(),Some(json!({"action":"intake_fix_held","reason":"Proposed fix plan; explicit review and manual requeue required","attempt_id":receipt.attempt_id}))).await?;
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
