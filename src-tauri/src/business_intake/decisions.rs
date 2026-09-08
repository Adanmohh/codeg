//! One writer owns task mutation, terminal disposition, private link and receipt.
use super::{
    access::{self, Use},
    candidates,
    common::*,
    error::{self, Error, Reason},
    records,
    services::Services,
    sources,
    types::*,
};
use crate::{
    business_identity::{self as identity, Domain, IdentityError, Principal},
    business_tasks::{store as tasks, types::Detail, ActorContext},
};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult, TransactionTrait,
};
use serde_json::json;

async fn view(tx: &DatabaseTransaction, p: &Principal, row: records::Decision) -> Result<Decision> {
    let task = match &row.task_id {
        None => DecisionTask::None,
        Some(id) => match candidates::permitted(
            tasks::get_in_transaction(tx, &ActorContext::authenticated(p.clone()), id).await,
        )? {
            Some(_) => DecisionTask::Accessible {
                task_id: id.clone(),
                task_revision: row.task_revision.ok_or_else(storage)?,
            },
            None => DecisionTask::Restricted,
        },
    };
    Ok(Decision {
        id: row.id,
        candidate_id: row.candidate_id,
        from_revision: row.from_revision,
        source_revision: row.source_revision,
        kind: vocabulary(&row.kind)?,
        actor_id: row.actor_id,
        task,
        created_at: row.created_at,
    })
}
/// Only called after current source authorization; target visibility is separate.
pub(super) async fn for_candidate(
    tx: &DatabaseTransaction,
    p: &Principal,
    candidate_id: &str,
) -> Result<Option<Decision>> {
    let row = records::Decision::find_by_statement(sql(
        "SELECT * FROM business_intake_decision WHERE organization_id=? AND candidate_id=?",
        vec![p.organization_id().into(), candidate_id.into()],
    ))
    .one(tx)
    .await?;
    match row {
        Some(row) => view(tx, p, row).await.map(Some),
        None => Ok(None),
    }
}
async fn by_id(
    tx: &DatabaseTransaction,
    p: &Principal,
    decision_id: &str,
) -> Result<records::Decision> {
    records::Decision::find_by_statement(sql(
        "SELECT * FROM business_intake_decision WHERE organization_id=? AND id=?",
        vec![p.organization_id().into(), decision_id.into()],
    ))
    .one(tx)
    .await?
    .ok_or_else(storage)
}
async fn replay(
    tx: &DatabaseTransaction,
    p: &Principal,
    candidate_id: &str,
    decision_id: &str,
) -> Result<DecisionResult> {
    let row = by_id(tx, p, decision_id).await?;
    if row.candidate_id != candidate_id {
        return Err(storage());
    }
    let task_id = row.task_id.as_deref().ok_or_else(storage)?;
    let task =
        tasks::get_in_transaction(tx, &ActorContext::authenticated(p.clone()), task_id).await?;
    Ok(DecisionResult {
        decision: view(tx, p, row).await?,
        task,
        replayed: true,
    })
}
struct Operation<'a> {
    id: &'a str,
    name: &'a str,
    digest: &'a str,
}
enum Outcome<'a> {
    Accepted {
        task: &'a Detail,
        link_id: &'a str,
        draft: &'a str,
    },
    Linked {
        task: &'a Detail,
        link_id: &'a str,
    },
    Discarded,
}
async fn complete(
    tx: &DatabaseTransaction,
    p: &Principal,
    row: &records::Candidate,
    op: Operation<'_>,
    outcome: Outcome<'_>,
) -> Result<Decision> {
    let (kind, task, link_id, draft) = match outcome {
        Outcome::Accepted {
            task,
            link_id,
            draft,
        } => (
            CandidateState::Accepted,
            Some(task),
            Some(link_id),
            Some(draft),
        ),
        Outcome::Linked { task, link_id } => {
            (CandidateState::Linked, Some(task), Some(link_id), None)
        }
        Outcome::Discarded => (CandidateState::Discarded, None, None, None),
    };
    let decision_id = id();
    let changed = tx.execute(sql("UPDATE business_intake_candidate SET revision=?,state=?,updated_by=?,updated_at=? WHERE organization_id=? AND id=? AND revision=? AND state='pending'",
        vec![next(row.revision)?.into(), code(kind)?.into(), p.member_id().into(), now().into(), p.organization_id().into(), row.id.clone().into(), row.revision.into()])).await?;
    if changed.rows_affected() != 1 {
        return Err(error::conflict());
    }
    tx.execute(sql("INSERT INTO business_intake_decision(id,organization_id,candidate_id,source_id,from_revision,source_revision,kind,actor_id,task_id,task_revision,publication_domain,reviewed_draft,passage_ids,created_at) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        vec![decision_id.clone().into(), p.organization_id().into(), row.id.clone().into(), row.source_id.clone().into(), row.revision.into(), row.source_revision.into(), code(kind)?.into(), p.member_id().into(),
            task.map(|d| d.task.id.clone()).into(), task.map(|d| d.task.revision).into(), task.map(|d| code(d.task.domain)).transpose()?.into(), draft.into(), row.passage_ids.clone().into(), now().into()])).await?;
    if let (Some(task), Some(link_id)) = (task, link_id) {
        tx.execute(sql("INSERT INTO business_intake_link(id,organization_id,source_id,candidate_id,decision_id,task_id) VALUES(?,?,?,?,?,?)",
            vec![link_id.into(), p.organization_id().into(), row.source_id.clone().into(), row.id.clone().into(), decision_id.clone().into(), task.task.id.clone().into()])).await?;
    }
    audit(tx, p, op.name, &row.id, next(row.revision)?).await?;
    record(tx, p, op.id, op.name, op.digest, &decision_id).await?;
    view(tx, p, by_id(tx, p, &decision_id).await?).await
}
async fn publication(
    tx: &DatabaseTransaction,
    p: &Principal,
    services: &Services,
    row: &records::Candidate,
    expected_source_revision: i64,
    destination: Domain,
) -> Result<(records::Source, access::Checked)> {
    revision(expected_source_revision)?;
    let (s, c) =
        access::check_source(tx, p, services, &row.source_id, Use::Publish(destination)).await?;
    sources::fresh(tx, p, services, &s, &c).await?;
    if s.revision != Some(expected_source_revision) || candidates::needs_rebase(row, &s, &c) {
        return Err(Reason::RebaseRequired.into());
    }
    sources::selection(
        tx,
        p,
        &s.id,
        row.source_revision,
        &parse::<Vec<String>>(&row.passage_ids)?,
    )
    .await?;
    Ok((s, c))
}
pub(super) async fn accept(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: AcceptCandidateInput,
) -> Result<DecisionResult> {
    let digest = digest(
        &json!({"candidate":input.candidate_id,"revision":input.expected_revision,"sourceRevision":input.expected_source_revision,"domain":input.publish_to_domain}),
    )?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let row = candidates::row(&tx, p, &input.candidate_id).await?;
    access::check_source(&tx, p, services, &row.source_id, Use::Read).await?;
    if let Some(id) = receipt(&tx, p, &input.operation_id, "candidates/accept", &digest).await? {
        return replay(&tx, p, &row.id, &id).await;
    }
    candidates::pending(&row, input.expected_revision)?;
    let (s, c) = publication(
        &tx,
        p,
        services,
        &row,
        input.expected_source_revision,
        input.publish_to_domain,
    )
    .await?;
    let stored = row.draft_json.as_deref().ok_or_else(error::invalid)?;
    let draft: PreparedTask = parse(stored)?;
    if draft.domain != input.publish_to_domain {
        return Err(Reason::PublicationNotAllowed.into());
    }
    let task =
        tasks::create_in_transaction(&tx, &ActorContext::authenticated(p.clone()), draft.into())
            .await?;
    let link_id = id();
    let decision = complete(
        &tx,
        p,
        &row,
        Operation {
            id: &input.operation_id,
            name: "candidates/accept",
            digest: &digest,
        },
        Outcome::Accepted {
            task: &task,
            link_id: &link_id,
            draft: stored,
        },
    )
    .await?;
    // A deadline may pass while resolving references. Drop the complete outer
    // transaction if content or this human's grant expired before commit.
    sources::fresh(&tx, p, services, &s, &c).await?;
    tx.commit().await?;
    Ok(DecisionResult {
        decision,
        task,
        replayed: false,
    })
}
pub(super) async fn link(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: LinkCandidateInput,
) -> Result<DecisionResult> {
    uuid(&input.task_id)?;
    revision(input.expected_task_revision)?;
    let digest = digest(
        &json!({"candidate":input.candidate_id,"revision":input.expected_revision,"sourceRevision":input.expected_source_revision,"domain":input.publish_to_domain,"task":input.task_id,"taskRevision":input.expected_task_revision}),
    )?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let row = candidates::row(&tx, p, &input.candidate_id).await?;
    access::check_source(&tx, p, services, &row.source_id, Use::Read).await?;
    if let Some(id) = receipt(&tx, p, &input.operation_id, "candidates/link", &digest).await? {
        return replay(&tx, p, &row.id, &id).await;
    }
    candidates::pending(&row, input.expected_revision)?;
    let (s, c) = publication(
        &tx,
        p,
        services,
        &row,
        input.expected_source_revision,
        input.publish_to_domain,
    )
    .await?;
    let link_id = id();
    let task = tasks::link_source_in_transaction(
        &tx,
        &ActorContext::authenticated(p.clone()),
        &input.task_id,
        input.expected_task_revision,
        input.publish_to_domain,
        &link_id,
    )
    .await?;
    let decision = complete(
        &tx,
        p,
        &row,
        Operation {
            id: &input.operation_id,
            name: "candidates/link",
            digest: &digest,
        },
        Outcome::Linked {
            task: &task,
            link_id: &link_id,
        },
    )
    .await?;
    sources::fresh(&tx, p, services, &s, &c).await?;
    tx.commit().await?;
    Ok(DecisionResult {
        decision,
        task,
        replayed: false,
    })
}
pub(super) async fn discard(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: DiscardInput,
) -> Result<Decision> {
    let digest =
        digest(&json!({"candidate":input.candidate_id,"revision":input.expected_revision}))?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let row = candidates::row(&tx, p, &input.candidate_id).await?;
    access::check_source(&tx, p, services, &row.source_id, Use::Triage).await?;
    if let Some(id) = receipt(&tx, p, &input.operation_id, "candidates/discard", &digest).await? {
        return view(&tx, p, by_id(&tx, p, &id).await?).await;
    }
    candidates::pending(&row, input.expected_revision)?;
    let result = complete(
        &tx,
        p,
        &row,
        Operation {
            id: &input.operation_id,
            name: "candidates/discard",
            digest: &digest,
        },
        Outcome::Discarded,
    )
    .await?;
    tx.commit().await?;
    Ok(result)
}
pub(super) async fn task_sources(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: TaskInput,
) -> Result<TaskSources> {
    uuid(&input.task_id)?;
    let tx = db.begin().await?;
    access::human(&tx, p).await?;
    tasks::get_in_transaction(&tx, &ActorContext::authenticated(p.clone()), &input.task_id).await?;
    let rows = records::Link::find_by_statement(sql("SELECT id,source_id FROM business_intake_link WHERE organization_id=? AND task_id=? ORDER BY id",
        vec![p.organization_id().into(), input.task_id.into()])).all(&tx).await?;
    let mut links = Vec::new();
    for row in rows {
        let source = match access::check_source(&tx, p, services, &row.source_id, Use::Read).await {
            Ok((s, c)) => Some(sources::inspect(&tx, p, services, &s, &c).await?),
            Err(Error::Identity(IdentityError::Forbidden | IdentityError::NotFound))
            | Err(Error::Intake(
                Reason::BindingDisabled
                | Reason::BindingUnavailable
                | Reason::CredentialUnavailable
                | Reason::SourceDenied,
            )) => None,
            Err(e) => return Err(e),
        };
        links.push(SourceLinkView {
            link_id: row.id,
            accessible: source.is_some(),
            source,
        });
    }
    Ok(TaskSources { links })
}
