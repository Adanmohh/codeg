//! Private review state; publication always reuses the current task core.
use super::{
    access::{self, Checked, Use},
    common::*,
    decisions,
    error::{self, Reason},
    records,
    services::Services,
    sources,
    types::*,
};
use crate::{
    business_identity::{self as identity, IdentityError, Permission, Principal},
    business_tasks::{store as tasks, ActorContext},
};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult, TransactionTrait,
};
use serde_json::json;

pub(super) async fn row(
    tx: &DatabaseTransaction,
    p: &Principal,
    candidate_id: &str,
) -> Result<records::Candidate> {
    uuid(candidate_id)?;
    records::Candidate::find_by_statement(sql(
        "SELECT * FROM business_intake_candidate WHERE organization_id=? AND id=?",
        vec![p.organization_id().into(), candidate_id.into()],
    ))
    .one(tx)
    .await?
    .ok_or_else(error::missing)
}
pub(super) fn pending(row: &records::Candidate, expected: i64) -> Result<()> {
    revision(expected)?;
    if row.state != "pending" || row.revision != expected {
        return Err(error::conflict());
    }
    Ok(())
}
pub(super) fn needs_rebase(
    row: &records::Candidate,
    source: &records::Source,
    c: &Checked,
) -> bool {
    source.revision != Some(row.source_revision) || row.prepared_epoch != c.binding.access_epoch
}
/// Capabilities can hide an unavailable action, never an authentication/storage failure.
pub(super) fn permitted<T>(result: std::result::Result<T, IdentityError>) -> Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(IdentityError::Forbidden | IdentityError::NotFound | IdentityError::Invalid(_)) => {
            Ok(None)
        }
        Err(e) => Err(e.into()),
    }
}
async fn view(
    tx: &DatabaseTransaction,
    p: &Principal,
    row: &records::Candidate,
    s: &records::Source,
    c: &Checked,
    source: &SourceSummary,
) -> Result<Candidate> {
    let readable = source.access == AccessState::Fresh;
    let requires_rebase = needs_rebase(row, s, c);
    let pending = row.state == "pending";
    let triage = pending
        && c.grant.can_triage
        && c.member
            .allows(Permission::Contribute, Some(vocabulary(&c.binding.domain)?));
    let select = readable && triage;
    let edit = select && !c.publication_domains.is_empty();
    let draft: Option<PreparedTask> = if readable {
        row.draft_json.as_deref().map(parse).transpose()?
    } else {
        None
    };
    let draft = draft.filter(|d| c.member.allows(Permission::Read, Some(d.domain)));
    let mut accept = false;
    if edit && !requires_rebase {
        if let Some(draft) = &draft {
            if c.publication_domains.contains(&draft.domain) {
                accept = permitted(
                    tasks::prepare_in_transaction(
                        tx,
                        &ActorContext::authenticated(p.clone()),
                        draft.clone().into(),
                    )
                    .await,
                )?
                .is_some();
            }
        }
    }
    Ok(Candidate {
        id: row.id.clone(),
        source_id: row.source_id.clone(),
        revision: row.revision,
        source_revision: row.source_revision,
        state: vocabulary(&row.state)?,
        origin: vocabulary(&row.origin)?,
        requires_rebase,
        disclosure: if readable {
            Disclosure::Fresh
        } else {
            Disclosure::MetadataOnly
        },
        has_prepared_draft: row.draft_json.is_some(),
        draft,
        owner_suggestion: if readable {
            row.owner_suggestion.clone()
        } else {
            None
        },
        due_suggestion: if readable {
            row.due_suggestion.clone()
        } else {
            None
        },
        capabilities: CandidateCapabilities {
            select,
            edit,
            accept,
            link: edit && !requires_rebase,
            discard: triage,
            publication_domains: c.publication_domains.clone(),
        },
    })
}
pub(super) async fn detail(
    tx: &DatabaseTransaction,
    p: &Principal,
    services: &Services,
    candidate_id: &str,
) -> Result<CandidateDetail> {
    let row = row(tx, p, candidate_id).await?;
    let (s, c) = access::check_source(tx, p, services, &row.source_id, Use::Read).await?;
    let source = sources::inspect(tx, p, services, &s, &c).await?;
    let passages = if source.access == AccessState::Fresh {
        sources::selection(
            tx,
            p,
            &s.id,
            row.source_revision,
            &parse::<Vec<String>>(&row.passage_ids)?,
        )
        .await?
    } else {
        vec![]
    };
    let candidate = view(tx, p, &row, &s, &c, &source).await?;
    let decision = decisions::for_candidate(tx, p, &row.id).await?;
    Ok(CandidateDetail {
        candidate,
        passages,
        source,
        decision,
    })
}
pub(super) async fn get(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: CandidateInput,
) -> Result<CandidateDetail> {
    let tx = db.begin().await?;
    detail(&tx, p, services, &input.candidate_id).await
}
pub(super) async fn list(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: CandidatesInput,
) -> Result<CandidatePage> {
    let offset = page(input.page)?;
    let tx = db.begin().await?;
    let (s, c) = access::check_source(&tx, p, services, &input.source_id, Use::Read).await?;
    let source = sources::inspect(&tx, p, services, &s, &c).await?;
    let rows = records::Candidate::find_by_statement(sql(
        "SELECT * FROM business_intake_candidate WHERE organization_id=? AND source_id=? AND state=? ORDER BY id LIMIT 51 OFFSET ?",
        vec![p.organization_id().into(), s.id.clone().into(), code(input.state)?.into(), offset.into()],
    )).all(&tx).await?;
    let has_more = rows.len() > 50;
    let mut items = Vec::new();
    for row in rows.into_iter().take(50) {
        items.push(view(&tx, p, &row, &s, &c, &source).await?);
    }
    Ok(CandidatePage {
        items,
        page: input.page,
        has_more,
    })
}
async fn current_selection(
    tx: &DatabaseTransaction,
    p: &Principal,
    services: &Services,
    s: &records::Source,
    c: &Checked,
    expected: i64,
    ids: &[String],
) -> Result<()> {
    revision(expected)?;
    sources::fresh(tx, p, services, s, c).await?;
    if s.revision != Some(expected) {
        return Err(Reason::RebaseRequired.into());
    }
    sources::selection(tx, p, &s.id, expected, ids).await?;
    Ok(())
}
pub(super) async fn create(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: CreateCandidateInput,
) -> Result<Candidate> {
    let digest = digest(
        &json!({"source":input.source_id,"revision":input.expected_source_revision,"passages":input.passage_ids}),
    )?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let (s, c) = access::check_source(&tx, p, services, &input.source_id, Use::Triage).await?;
    if let Some(id) = receipt(&tx, p, &input.operation_id, "candidates/create", &digest).await? {
        return Ok(detail(&tx, p, services, &id).await?.candidate);
    }
    current_selection(
        &tx,
        p,
        services,
        &s,
        &c,
        input.expected_source_revision,
        &input.passage_ids,
    )
    .await?;
    let id = id();
    tx.execute(sql("INSERT INTO business_intake_candidate(id,organization_id,source_id,revision,source_revision,prepared_epoch,state,origin,passage_ids,created_by,updated_by,created_at,updated_at) VALUES(?,?,?,1,?,?,'pending','human_selection',?,?,?,?,?)",
        vec![id.clone().into(), p.organization_id().into(), s.id.into(), input.expected_source_revision.into(), c.binding.access_epoch.into(), json(&input.passage_ids)?.into(), p.member_id().into(), p.member_id().into(), now().into(), now().into()])).await?;
    audit(&tx, p, "candidate_created", &id, 1).await?;
    record(
        &tx,
        p,
        &input.operation_id,
        "candidates/create",
        &digest,
        &id,
    )
    .await?;
    let result = detail(&tx, p, services, &id).await?.candidate;
    tx.commit().await?;
    Ok(result)
}
struct Revision<'a> {
    candidate_id: &'a str,
    expected: i64,
    source_revision: i64,
    passage_ids: &'a [String],
}
struct Change<'a> {
    draft: Option<&'a str>,
    owner_suggestion: Option<&'a str>,
    due_suggestion: Option<&'a str>,
    epoch: i64,
}
async fn save(
    tx: &DatabaseTransaction,
    p: &Principal,
    r: &Revision<'_>,
    change: Change<'_>,
) -> Result<()> {
    let result = tx.execute(sql("UPDATE business_intake_candidate SET revision=?,source_revision=?,prepared_epoch=?,passage_ids=?,draft_json=?,owner_suggestion=?,due_suggestion=?,updated_by=?,updated_at=? WHERE organization_id=? AND id=? AND revision=? AND state='pending'",
        vec![next(r.expected)?.into(), r.source_revision.into(), change.epoch.into(), json(r.passage_ids)?.into(), change.draft.into(), change.owner_suggestion.into(), change.due_suggestion.into(), p.member_id().into(), now().into(), p.organization_id().into(), r.candidate_id.into(), r.expected.into()])).await?;
    if result.rows_affected() != 1 {
        return Err(error::conflict());
    }
    Ok(())
}
pub(super) async fn select(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: SelectCandidateInput,
) -> Result<CandidateDetail> {
    let digest = digest(
        &json!({"candidate":input.candidate_id,"revision":input.expected_revision,"sourceRevision":input.expected_source_revision,"passages":input.passage_ids}),
    )?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let row = row(&tx, p, &input.candidate_id).await?;
    let (s, c) = access::check_source(&tx, p, services, &row.source_id, Use::Triage).await?;
    if receipt(&tx, p, &input.operation_id, "candidates/select", &digest)
        .await?
        .is_some()
    {
        return detail(&tx, p, services, &row.id).await;
    }
    pending(&row, input.expected_revision)?;
    current_selection(
        &tx,
        p,
        services,
        &s,
        &c,
        input.expected_source_revision,
        &input.passage_ids,
    )
    .await?;
    save(
        &tx,
        p,
        &Revision {
            candidate_id: &row.id,
            expected: row.revision,
            source_revision: input.expected_source_revision,
            passage_ids: &input.passage_ids,
        },
        Change {
            draft: row.draft_json.as_deref(),
            owner_suggestion: row.owner_suggestion.as_deref(),
            due_suggestion: row.due_suggestion.as_deref(),
            epoch: c.binding.access_epoch,
        },
    )
    .await?;
    audit(&tx, p, "candidate_selected", &row.id, next(row.revision)?).await?;
    record(
        &tx,
        p,
        &input.operation_id,
        "candidates/select",
        &digest,
        &row.id,
    )
    .await?;
    let result = detail(&tx, p, services, &row.id).await?;
    tx.commit().await?;
    Ok(result)
}
pub(super) async fn edit(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: EditCandidateInput,
) -> Result<CandidateDetail> {
    if [&input.owner_suggestion, &input.due_suggestion]
        .into_iter()
        .flatten()
        .any(|v| !text(v, 240, false))
    {
        return Err(error::invalid());
    }
    let task = &input.task;
    let digest = digest(
        &json!({"candidate":input.candidate_id,"revision":input.expected_revision,"sourceRevision":input.expected_source_revision,"passages":input.passage_ids,
        "task":{"title":task.title,"notes":task.notes,"domain":task.domain,"priority":task.priority,"dueDate":task.due_date,"ownerId":task.owner_id,"assigneeId":task.assignee_id,"reviewerId":task.reviewer_id},
        "ownerSuggestion":input.owner_suggestion,"dueSuggestion":input.due_suggestion}),
    )?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let row = row(&tx, p, &input.candidate_id).await?;
    let (s, c) = access::check_source(
        &tx,
        p,
        services,
        &row.source_id,
        Use::Publish(input.task.domain),
    )
    .await?;
    if receipt(&tx, p, &input.operation_id, "candidates/edit", &digest)
        .await?
        .is_some()
    {
        return detail(&tx, p, services, &row.id).await;
    }
    pending(&row, input.expected_revision)?;
    current_selection(
        &tx,
        p,
        services,
        &s,
        &c,
        input.expected_source_revision,
        &input.passage_ids,
    )
    .await?;
    let draft =
        tasks::prepare_in_transaction(&tx, &ActorContext::authenticated(p.clone()), input.task)
            .await?;
    let draft = json(&draft)?;
    save(
        &tx,
        p,
        &Revision {
            candidate_id: &row.id,
            expected: row.revision,
            source_revision: input.expected_source_revision,
            passage_ids: &input.passage_ids,
        },
        Change {
            draft: Some(&draft),
            owner_suggestion: input.owner_suggestion.as_deref(),
            due_suggestion: input.due_suggestion.as_deref(),
            epoch: c.binding.access_epoch,
        },
    )
    .await?;
    audit(&tx, p, "candidate_edited", &row.id, next(row.revision)?).await?;
    record(
        &tx,
        p,
        &input.operation_id,
        "candidates/edit",
        &digest,
        &row.id,
    )
    .await?;
    let result = detail(&tx, p, services, &row.id).await?;
    tx.commit().await?;
    Ok(result)
}
