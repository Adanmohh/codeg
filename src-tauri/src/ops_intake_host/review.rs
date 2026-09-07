//! Closed GitHub review adapter; delegates every gate decision to accepted core.
use super::{runtime::HostRuntime, store::*, types::*};
use crate::{
    db::{
        error::DbError,
        service::ops_approvals::{
            self as approvals, Action, ActionContext, Decision, ProposalOutcome,
        },
    },
    ops::Operator,
    ops_intake::{self, EvidenceSet, GithubIssueAction, IssueDraftV1, PreparedIssue},
};
use sea_orm::{ConnectionTrait, DatabaseConnection, TransactionTrait};
use serde_json::Value;

fn gate_error(_: HostError) -> DbError {
    DbError::Validation("Intake draft, scope or evidence changed; refresh and review again".into())
}
pub(super) struct HostIssueAction {
    pub account: i32,
    pub source: SourceInput,
    pub draft_id: String,
    pub revision: i32,
    pub prepared: PreparedIssue,
    pub connection: Option<String>,
}
impl HostIssueAction {
    async fn check(&self, payload: &Value, ctx: &ActionContext<'_>) -> Result<(), DbError> {
        enabled(ctx.db, self.account, &self.source.product_id)
            .await
            .map_err(gate_error)?;
        let snapshot = snapshot(ctx.db, &self.source).await.map_err(gate_error)?;
        fresh(&snapshot).map_err(gate_error)?;
        let d = draft(ctx.db, &self.source).await.map_err(gate_error)?;
        if d.id != self.draft_id
            || d.revision != self.revision
            || d.prepared.as_ref() != Some(&self.prepared)
            || self
                .prepared
                .payload()
                .map_err(HostError::from)
                .map_err(gate_error)?
                != *payload
        {
            return Err(gate_error(HostError::Conflict));
        }
        if let Some(connection) = &self.connection {
            let found=ctx.db.query_one(sql("SELECT id FROM work_task WHERE id=? AND run_seq=? AND connection_id=? AND deleted_at IS NULL AND status IN ('running','awaiting_input')",vec![self.prepared.draft.task_id.into(),self.prepared.draft.run_seq.into(),connection.clone().into()])).await?;
            if found.is_none() {
                return Err(gate_error(HostError::TaskRequired));
            }
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl Action for HostIssueAction {
    fn name(&self) -> &'static str {
        GithubIssueAction.name()
    }
    fn domain(&self) -> &'static str {
        GithubIssueAction.domain()
    }
    fn validate(&self, payload: &Value) -> Result<(), DbError> {
        GithubIssueAction.validate(payload)
    }
    async fn resource(
        &self,
        payload: &Value,
        ctx: &ActionContext<'_>,
    ) -> Result<Option<String>, DbError> {
        self.check(payload, ctx).await?;
        GithubIssueAction.resource(payload, ctx).await
    }
    async fn check_permission(
        &self,
        payload: &Value,
        ctx: &ActionContext<'_>,
    ) -> Result<Decision, DbError> {
        self.check(payload, ctx).await?;
        GithubIssueAction.check_permission(payload, ctx).await
    }
    fn is_destructive(&self, payload: &Value) -> Result<bool, DbError> {
        GithubIssueAction.is_destructive(payload)
    }
    fn private_fields(&self) -> &'static [&'static str] {
        GithubIssueAction.private_fields()
    }
}

pub(super) async fn prepare_for(
    db: &DatabaseConnection,
    account: i32,
    source: &SourceInput,
    expected: i32,
    task_id: i32,
    required_run: Option<i32>,
) -> Result<Draft, HostError> {
    let p = enabled(db, account, &source.product_id).await?;
    let snap = snapshot(db, source).await?;
    fresh(&snap)?;
    if db.query_one(sql("SELECT proposal_id FROM ops_intake_host_handoff WHERE product_id=? AND ulid=? AND state='unknown' LIMIT 1",vec![source.product_id.clone().into(),source.ulid.clone().into()])).await?.is_some() {return Err(HostError::Conflict);}
    if ops_intake::filing_status(
        db,
        &snap.record.source_ref.source(),
        p.binding.repository_id,
    )
    .await?
    .is_some_and(|r| r.state != ops_intake::FilingState::Failed)
    {
        return Err(HostError::Conflict);
    }
    let task=db.query_one(sql("SELECT run_seq FROM work_task WHERE id=? AND folder_id=? AND deleted_at IS NULL AND status IN ('running','awaiting_input')",vec![task_id.into(),p.binding.folder_id.into()])).await?.ok_or(HostError::TaskRequired)?;
    let run_seq: i32 = task.try_get("", "run_seq")?;
    if required_run.is_some_and(|r| r != run_seq) {
        return Err(HostError::TaskRequired);
    }
    let mut d = super::store::ensure_draft(db, source).await?;
    if d.revision != expected {
        return Err(HostError::Conflict);
    }
    let severity = match d.confirmed_severity {
        Some(Severity::High) => "high",
        Some(Severity::Medium) => "medium",
        Some(Severity::Low) => "low",
        None => return Err(HostError::SeverityRequired),
    };
    let proof = |field: &str| {
        d.proofs
            .get(field)
            .map(|p| p.proof.clone())
            .ok_or(HostError::InvalidEvidence)
    };
    let issue = IssueDraftV1 {
        schema_version: 1,
        template_version: ops_intake::TEMPLATE_VERSION.into(),
        task_id,
        run_seq,
        source_ref: snap.record.source_ref.source(),
        source_revision: snap.record.source_revision,
        title: d.title.clone(),
        summary: format!("{}\n\nSeverity: {severity} (human-confirmed)", d.summary),
        labels: d.labels.clone(),
        evidence: EvidenceSet {
            build: proof("build")?,
            screen: proof("screen")?,
            reciter: proof("reciter")?,
            log: proof("log")?,
        },
    };
    let prepared = ops_intake::prepare(db, issue).await?;
    if d.prepared.as_ref() == Some(&prepared) {
        return Ok(d);
    }
    d.prepared = Some(prepared);
    cas(db, &mut d, expected).await?;
    Ok(d)
}
pub async fn prepare(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    input: PrepareInput,
) -> Result<Draft, HostError> {
    let _guard = runtime.guard(&input.source.product_id).await?;
    prepare_for(
        db,
        op.account_id(),
        &input.source,
        input.expected_revision,
        input.task_id,
        None,
    )
    .await
}

pub(super) async fn rows(
    db: &DatabaseConnection,
    account: i32,
    source: &SourceInput,
) -> Result<Vec<sea_orm::QueryResult>, HostError> {
    enabled(db, account, &source.product_id).await?;
    Ok(db.query_all(sql("SELECT p.id,p.status,p.payload_json,p.task_id,p.run_seq FROM ops_proposal p WHERE p.action_name='github.create_issue' AND json_extract(CASE WHEN json_valid(p.payload_json) THEN p.payload_json ELSE '{}' END,'$.draft.source_ref.product_id')=? AND json_extract(CASE WHEN json_valid(p.payload_json) THEN p.payload_json ELSE '{}' END,'$.draft.source_ref.ulid')=? ORDER BY p.id DESC LIMIT 20",vec![source.product_id.clone().into(),source.ulid.clone().into()])).await?)
}
pub(super) async fn proposals(
    db: &DatabaseConnection,
    account: i32,
    source: &SourceInput,
) -> Result<Vec<Proposal>, HostError> {
    let d = draft(db, source).await?;
    let snapshot = snapshot(db, source).await?;
    let mut result = vec![];
    for row in rows(db, account, source).await? {
        let status: String = row.try_get("", "status")?;
        let payload: Option<PreparedIssue> = if status == "pending" {
            Some(decode(&row.try_get::<String>("", "payload_json")?)?)
        } else {
            None
        };
        let mut stale = fresh(&snapshot).is_err();
        if let Some(p) = &payload {
            stale |= d.prepared.as_ref() != Some(p)
                || ops_intake::prepare(db, p.draft.clone()).await.is_err();
        }
        result.push(Proposal {
            id: row.try_get("", "id")?,
            status,
            payload,
            stale,
        });
    }
    Ok(result)
}
async fn pending(
    db: &DatabaseConnection,
    account: i32,
    source: &SourceInput,
    id: i32,
    expected: &PreparedIssue,
) -> Result<(), HostError> {
    let row = rows(db, account, source)
        .await?
        .into_iter()
        .find(|r| r.try_get::<i32>("", "id").ok() == Some(id))
        .ok_or(HostError::AccessDenied)?;
    if row.try_get::<String>("", "status")? != "pending"
        || decode::<PreparedIssue>(&row.try_get::<String>("", "payload_json")?)? != *expected
    {
        return Err(HostError::Conflict);
    }
    Ok(())
}
pub async fn deny(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    input: DenyInput,
) -> Result<Detail, HostError> {
    let _guard = runtime.guard(&input.source.product_id).await?;
    pending(
        db,
        op.account_id(),
        &input.source,
        input.proposal_id,
        &input.expected_payload,
    )
    .await?;
    let p = input.expected_payload;
    approvals::deny(
        db,
        p.draft.task_id,
        p.draft.run_seq,
        input.proposal_id,
        op.actor(),
        &GithubIssueAction,
    )
    .await?;
    super::operator::detail(db, op, input.source).await
}
pub async fn approve(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    input: ReviewInput,
) -> Result<Detail, HostError> {
    let mut guard = runtime.guard(&input.source.product_id).await?;
    let config = enabled(db, op.account_id(), &input.source.product_id).await?;
    pending(
        db,
        op.account_id(),
        &input.source,
        input.proposal_id,
        &input.expected_payload,
    )
    .await?;
    let d = draft(db, &input.source).await?;
    let p = input.approved_payload;
    if d.prepared.as_ref() != Some(&p)
        || p.draft.source_ref != input.expected_payload.draft.source_ref
        || p.draft.task_id != input.expected_payload.draft.task_id
        || p.draft.run_seq != input.expected_payload.draft.run_seq
    {
        return Err(HostError::Conflict);
    }
    let client = runtime.client(&mut guard, &config)?; // Missing key preserves pending approval.
    let action = HostIssueAction {
        account: op.account_id(),
        source: input.source.clone(),
        draft_id: d.id,
        revision: d.revision,
        prepared: p.clone(),
        connection: None,
    };
    // Durable before consuming approval. If canceled/crashed, never invent a retry.
    let changed=db.execute(sql("INSERT INTO ops_intake_host_handoff(proposal_id,product_id,ulid,state) VALUES(?,?,?,'unknown') ON CONFLICT(proposal_id) DO UPDATE SET state='unknown' WHERE state='not_authorized'",vec![input.proposal_id.into(),input.source.product_id.clone().into(),input.source.ulid.clone().into()])).await?;
    if changed.rows_affected() != 1 {
        return Err(HostError::Conflict);
    }
    let authorized = approvals::approve(
        db,
        p.draft.task_id,
        p.draft.run_seq,
        input.proposal_id,
        op.actor(),
        &action,
        approvals::Review {
            expected_payload: &input.expected_payload.payload()?,
            approved_payload: p.payload()?,
        },
    )
    .await;
    let authorized = match authorized {
        Ok(a) => a,
        Err(e) => {
            db.execute(sql(
                "UPDATE ops_intake_host_handoff SET state='not_authorized' WHERE proposal_id=?",
                vec![input.proposal_id.into()],
            ))
            .await?;
            return Err(e.into());
        }
    };
    // Only this owned handoff reaches the existing client/receipt state machine.
    match ops_intake::dispatch(db, &client, authorized).await {
        Ok(_) => {
            db.execute(sql(
                "UPDATE ops_intake_host_handoff SET state='recorded' WHERE proposal_id=?",
                vec![input.proposal_id.into()],
            ))
            .await?;
        }
        Err(_) => { /* unknown handoff remains durable, without exception/HTTP text */ }
    }
    super::operator::detail(db, op, input.source).await
}
pub async fn reconcile(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    source: SourceInput,
) -> Result<Detail, HostError> {
    let mut guard = runtime.guard(&source.product_id).await?;
    let config = enabled(db, op.account_id(), &source.product_id).await?;
    let snap = snapshot(db, &source).await?;
    let receipt = ops_intake::filing_status(
        db,
        &snap.record.source_ref.source(),
        config.binding.repository_id,
    )
    .await?
    .ok_or(HostError::Conflict)?;
    let client = runtime.client(&mut guard, &config)?;
    ops_intake::reconcile_filing(db, &client, receipt.attempt_id).await?;
    db.execute(sql(
        "UPDATE ops_intake_host_handoff SET state='recorded' WHERE proposal_id=?",
        vec![receipt.proposal_id.into()],
    ))
    .await?;
    super::operator::detail(db, op, source).await
}

pub(super) async fn propose(
    db: &DatabaseConnection,
    ctx: &crate::ops::agent::RunContext,
    draft_id: &str,
    expected_revision: i32,
    runtime: &HostRuntime,
) -> Result<super::agent::Proposed, HostError> {
    let row=db.query_one(sql("SELECT d.product_id,d.ulid FROM ops_intake_host_draft d JOIN ops_intake_host_product p ON p.product_id=d.product_id WHERE d.id=? AND p.account_id=?",vec![draft_id.into(),ctx.account_id.into()])).await?.ok_or(HostError::AccessDenied)?;
    let source = SourceInput {
        product_id: row.try_get("", "product_id")?,
        ulid: row.try_get("", "ulid")?,
    };
    let _guard = runtime.guard(&source.product_id).await?;
    crate::ops::agent::context(db, ctx).await?;
    let current = draft(db, &source).await?;
    if current.revision != expected_revision {
        return Err(HostError::Conflict);
    }
    // Core owns one pending proposal per run. A repeated bridge read returns
    // that exact live review without revising its draft or acquiring another wait.
    let txn = db.begin().await?;
    if let Some(row) = txn.query_one(sql(
        "SELECT id,payload_json FROM ops_proposal WHERE task_id=? AND run_seq=? AND agent_id=? AND action_name='github.create_issue' AND status='pending'",
        vec![ctx.task_id.into(), ctx.run_seq.into(), ctx.agent_id.clone().into()],
    )).await? {
        let prepared: PreparedIssue = decode(&row.try_get::<String>("", "payload_json")?)?;
        let action = HostIssueAction { account: ctx.account_id, source: source.clone(),
            draft_id: current.id, revision: current.revision, prepared: prepared.clone(),
            connection: Some(ctx.connection_id.clone()) };
        action.resource(&prepared.payload()?, &ActionContext { db: &txn, agent: &ctx.agent_id, actor: None }).await?;
        let id = row.try_get("", "id")?;
        txn.commit().await?;
        return Ok(super::agent::Proposed { status: "pending", proposal_id: Some(id), prepared });
    }
    // An unrelated ACP/proposal wait cannot be cleared or replaced by this tool.
    // Reject before prepare's CAS so an existing human review stays unchanged.
    if txn.query_one(sql("SELECT id FROM work_task WHERE id=? AND run_seq=? AND connection_id=? AND status='running' AND deleted_at IS NULL",
        vec![ctx.task_id.into(),ctx.run_seq.into(),ctx.connection_id.clone().into()])).await?.is_none() {
        return Err(HostError::TaskRequired);
    }
    txn.commit().await?;
    let d = prepare_for(
        db,
        ctx.account_id,
        &source,
        expected_revision,
        ctx.task_id,
        Some(ctx.run_seq),
    )
    .await?;
    let prepared = d.prepared.clone().ok_or(HostError::Conflict)?;
    let action = HostIssueAction {
        account: ctx.account_id,
        source,
        draft_id: d.id,
        revision: d.revision,
        prepared: prepared.clone(),
        connection: Some(ctx.connection_id.clone()),
    };
    let outcome = approvals::propose(
        db,
        ctx.task_id,
        ctx.run_seq,
        &ctx.agent_id,
        &action,
        prepared.payload()?,
    )
    .await?;
    match outcome {
        ProposalOutcome::Pending(row) => Ok(super::agent::Proposed {
            status: "pending",
            proposal_id: Some(row.id),
            prepared,
        }),
        ProposalOutcome::Denied(_) => Ok(super::agent::Proposed {
            status: "denied",
            proposal_id: None,
            prepared,
        }),
        ProposalOutcome::Authorized(_) => Err(HostError::AccessDenied), // Destructive floor cannot authorize here.
    }
}
