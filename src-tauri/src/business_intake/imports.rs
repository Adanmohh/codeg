//! Durable human-driven steps. Claim before one read; revalidate before commit.
use super::{
    access::{self, Use},
    common::*,
    error::{self, Error, Reason},
    fireflies::{Listed, Normalized, ReadError},
    legacy, records,
    services::Services,
    sources,
    types::*,
};
use crate::business_identity::{self as identity, Principal};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult, TransactionTrait,
};
use serde_json::json;

async fn model(tx: &DatabaseTransaction, p: &Principal, id: &str) -> Result<records::Import> {
    uuid(id)?;
    records::Import::find_by_statement(sql(
        "SELECT * FROM business_intake_import WHERE organization_id=? AND id=?",
        vec![p.organization_id().into(), id.into()],
    ))
    .one(tx)
    .await?
    .ok_or_else(error::missing)
}
fn terminal(row: &records::Import) -> bool {
    matches!(row.state.as_str(), "complete" | "failed" | "cancelled")
}
fn live(row: &records::Import) -> bool {
    row.state == "running"
        && row
            .lease_until
            .as_deref()
            .is_some_and(|s| unexpired(Some(s)))
}
fn view(row: records::Import) -> Result<Import> {
    let terminal = terminal(&row);
    let live = live(&row);
    let retry_ready = row
        .next_attempt_at
        .as_deref()
        .is_none_or(|s| !unexpired(Some(s)));
    let expired = row.state == "running" && !live;
    Ok(Import {
        id: row.id,
        binding_id: row.binding_id,
        revision: row.revision,
        state: if expired {
            ImportState::Waiting
        } else {
            vocabulary(&row.state)?
        },
        coverage: vocabulary(&row.coverage)?,
        discovered: row.discovered.try_into().map_err(|_| storage())?,
        completed: row.completed.try_into().map_err(|_| storage())?,
        failed: row.failed.try_into().map_err(|_| storage())?,
        next_attempt_at: row.next_attempt_at,
        error_code: if expired {
            Some(Reason::RequestTimeout)
        } else {
            row.error_code.as_deref().map(vocabulary).transpose()?
        },
        capabilities: ImportCapabilities {
            advance: !terminal && !live && retry_ready,
            cancel: !terminal,
        },
    })
}
async fn insert(
    tx: &DatabaseTransaction,
    p: &Principal,
    binding_id: &str,
    selection: &Selection,
    source: Option<&records::Source>,
) -> Result<String> {
    let import_id = id();
    tx.execute(sql("INSERT INTO business_intake_import(id,organization_id,binding_id,requester_id,state,coverage,selection_json,scan_done,discovered,created_at,updated_at) VALUES(?,?,?,?,'queued','not_started',?,?,?,?,?)",
        vec![import_id.clone().into(),p.organization_id().into(),binding_id.into(),p.member_id().into(),json(selection)?.into(),source.is_some().into(),i64::from(source.is_some()).into(),now().into(),now().into()])).await?;
    if let Some(source) = source {
        add_item(tx, p, binding_id, &import_id, &source.id).await?;
    }
    audit(tx, p, "import_started", &import_id, 1).await?;
    Ok(import_id)
}
async fn add_item(
    tx: &DatabaseTransaction,
    p: &Principal,
    binding_id: &str,
    import_id: &str,
    source_id: &str,
) -> Result<()> {
    tx.execute(sql("INSERT INTO business_intake_import_item(organization_id,binding_id,import_id,source_id,state) VALUES(?,?,?,?,'pending') ON CONFLICT(organization_id,import_id,source_id) DO NOTHING",
        vec![p.organization_id().into(),binding_id.into(),import_id.into(),source_id.into()])).await?;
    Ok(())
}
fn window(from: &str, to: &str) -> Result<()> {
    let from = instant(from)?;
    let to = instant(to)?;
    if from >= to || (to - from) > chrono::Duration::days(31) {
        return Err(error::invalid());
    }
    Ok(())
}
pub(super) async fn start(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: StartImportInput,
) -> Result<Import> {
    uuid(&input.operation_id)?;
    let digest = digest(&json!({"binding":input.binding_id,"selection":input.selection}))?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let c = access::check_binding(&tx, p, services, &input.binding_id, Use::Import).await?;
    if let Some(id) = receipt(&tx, p, &input.operation_id, "imports/start", &digest).await? {
        return view(model(&tx, p, &id).await?);
    }
    let source = match &input.selection {
        Selection::Window { from_date, to_date } => {
            if c.binding.kind != "fireflies" {
                return Err(error::invalid());
            }
            window(from_date, to_date)?;
            None
        }
        Selection::Record { source_id } => {
            let s = access::source(&tx, p, source_id).await?;
            if s.binding_id != c.binding.id {
                return Err(error::missing());
            }
            Some(s)
        }
    };
    let id = insert(&tx, p, &c.binding.id, &input.selection, source.as_ref()).await?;
    record(&tx, p, &input.operation_id, "imports/start", &digest, &id).await?;
    let result = view(model(&tx, p, &id).await?)?;
    tx.commit().await?;
    Ok(result)
}
pub(super) async fn capture(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: CaptureInput,
) -> Result<Import> {
    uuid(&input.operation_id)?;
    let digest = digest(&json!({"binding":input.binding_id,"ref":input.reference}))?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let c = access::check_binding(&tx, p, services, &input.binding_id, Use::Import).await?;
    if let Some(id) = receipt(&tx, p, &input.operation_id, "imports/capture", &digest).await? {
        return view(model(&tx, p, &id).await?);
    }
    let projection = legacy::project(
        &tx,
        &parse(&c.binding.resource_json)?,
        &input.reference,
        services,
    )
    .await?;
    let kind = match input.reference {
        LegacyRef::Email { .. } => SourceKind::Email,
        LegacyRef::HafidhTestflight { .. } => SourceKind::HafidhTestflight,
    };
    let s = sources::ensure(
        &tx,
        p,
        &c,
        kind,
        &json(&input.reference)?,
        &projection.title,
    )
    .await?;
    // Admission verifies an existing public cached record, but does not mint a
    // content version or freshness. The durable step performs that observation.
    let id = insert(
        &tx,
        p,
        &c.binding.id,
        &Selection::Record {
            source_id: s.id.clone(),
        },
        Some(&s),
    )
    .await?;
    record(&tx, p, &input.operation_id, "imports/capture", &digest, &id).await?;
    let result = view(model(&tx, p, &id).await?)?;
    tx.commit().await?;
    Ok(result)
}
pub(super) async fn list(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: ImportsInput,
) -> Result<ImportPage> {
    let offset = page(input.page)?;
    let tx = db.begin().await?;
    let c = access::check_binding(&tx, p, services, &input.binding_id, Use::Import).await?;
    let rows=records::Import::find_by_statement(sql("SELECT * FROM business_intake_import WHERE organization_id=? AND binding_id=? AND (? OR state IN ('queued','running','waiting')) ORDER BY created_at DESC,id LIMIT 51 OFFSET ?",
        vec![p.organization_id().into(),c.binding.id.into(),(input.view==ImportView::All).into(),offset.into()])).all(&tx).await?;
    let has_more = rows.len() > 50;
    Ok(ImportPage {
        items: rows.into_iter().take(50).map(view).collect::<Result<_>>()?,
        page: input.page,
        has_more,
    })
}
pub(super) async fn get(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: ImportInput,
) -> Result<Import> {
    let tx = db.begin().await?;
    let row = model(&tx, p, &input.import_id).await?;
    access::check_binding(&tx, p, services, &row.binding_id, Use::Import).await?;
    view(row)
}
pub(super) async fn cancel(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: ImportRevisionInput,
) -> Result<Import> {
    uuid(&input.operation_id)?;
    revision(input.expected_revision)?;
    let digest = digest(&json!({"import":input.import_id,"revision":input.expected_revision}))?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let row = model(&tx, p, &input.import_id).await?;
    access::check_binding(&tx, p, services, &row.binding_id, Use::Import).await?;
    if receipt(&tx, p, &input.operation_id, "imports/cancel", &digest)
        .await?
        .is_some()
    {
        return view(row);
    }
    if row.revision != input.expected_revision || terminal(&row) {
        return Err(error::conflict());
    }
    tx.execute(sql("UPDATE business_intake_import SET revision=?,state='cancelled',attempt_id=NULL,lease_until=NULL,next_attempt_at=NULL,updated_at=? WHERE organization_id=? AND id=? AND revision=?",
        vec![next(row.revision)?.into(),now().into(),p.organization_id().into(),row.id.clone().into(),row.revision.into()])).await?;
    audit(&tx, p, "import_cancelled", &row.id, next(row.revision)?).await?;
    record(
        &tx,
        p,
        &input.operation_id,
        "imports/cancel",
        &digest,
        &row.id,
    )
    .await?;
    let result = view(model(&tx, p, &row.id).await?)?;
    tx.commit().await?;
    Ok(result)
}

enum Work {
    Page {
        from: String,
        to: String,
        offset: i64,
    },
    Detail(records::Source),
}
enum Observation {
    Page(Vec<Listed>),
    Detail(Normalized),
}
struct Claim {
    import_id: String,
    attempt_id: String,
    epoch: i64,
    work: Work,
    resource: ResourceIdentity,
    credential_ref: Option<String>,
}
pub(super) async fn advance(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: ImportRevisionInput,
) -> Result<Import> {
    uuid(&input.operation_id)?;
    revision(input.expected_revision)?;
    let digest = digest(&json!({"import":input.import_id,"revision":input.expected_revision}))?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let row = model(&tx, p, &input.import_id).await?;
    let c = access::check_binding(&tx, p, services, &row.binding_id, Use::Import).await?;
    if receipt(&tx, p, &input.operation_id, "imports/advance", &digest)
        .await?
        .is_some()
    {
        return view(row);
    }
    if row.revision != input.expected_revision || terminal(&row) {
        return Err(error::conflict());
    }
    if live(&row) {
        return Err(Reason::ImportBusy.into());
    }
    if row
        .next_attempt_at
        .as_deref()
        .is_some_and(|v| unexpired(Some(v)))
    {
        return Err(Reason::RetryLater.into());
    }
    if row.attempt_count >= 3 {
        tx.execute(sql("UPDATE business_intake_import SET revision=?,state='failed',error_code='request_timeout',attempt_id=NULL,lease_until=NULL,updated_at=? WHERE organization_id=? AND id=?",vec![next(row.revision)?.into(),now().into(),p.organization_id().into(),row.id.clone().into()])).await?;
        record(
            &tx,
            p,
            &input.operation_id,
            "imports/advance",
            &digest,
            &row.id,
        )
        .await?;
        audit(&tx, p, "import_exhausted", &row.id, next(row.revision)?).await?;
        let result = view(model(&tx, p, &row.id).await?)?;
        tx.commit().await?;
        return Ok(result);
    }
    let pending=tx.query_one(sql("SELECT source_id FROM business_intake_import_item WHERE organization_id=? AND import_id=? AND state='pending' ORDER BY source_id LIMIT 1",vec![p.organization_id().into(),row.id.clone().into()])).await?;
    let work = if let Some(item) = pending {
        let source_id: String = item.try_get("", "source_id")?;
        let source = access::source(&tx, p, &source_id).await?;
        if source.binding_id != c.binding.id {
            return Err(storage());
        }
        Work::Detail(sources::fence(&tx, p, &source).await?)
    } else if !row.scan_done {
        let selection: Selection = parse(&row.selection_json)?;
        match selection {
            Selection::Window { from_date, to_date } => {
                window(&from_date, &to_date)?;
                Work::Page {
                    from: from_date,
                    to: to_date,
                    offset: row.scan_offset,
                }
            }
            _ => return Err(storage()),
        }
    } else {
        return Err(error::conflict());
    };
    let claim = Claim {
        import_id: row.id.clone(),
        attempt_id: id(),
        epoch: c.binding.access_epoch,
        work,
        resource: parse(&c.binding.resource_json)?,
        credential_ref: c.binding.credential_ref,
    };
    tx.execute(sql("UPDATE business_intake_import SET revision=?,state='running',attempt_id=?,attempt_actor_id=?,lease_until=?,attempt_count=?,attempt_epoch=?,next_attempt_at=NULL,error_code=NULL,updated_at=? WHERE organization_id=? AND id=? AND revision=?",
        vec![next(row.revision)?.into(),claim.attempt_id.clone().into(),p.member_id().into(),future(60).into(),next(row.attempt_count)?.into(),claim.epoch.into(),now().into(),p.organization_id().into(),row.id.clone().into(),row.revision.into()])).await?;
    record(
        &tx,
        p,
        &input.operation_id,
        "imports/advance",
        &digest,
        &row.id,
    )
    .await?;
    audit(&tx, p, "import_claimed", &row.id, next(row.revision)?).await?;
    tx.commit().await?;
    let observation = observe(db, p, services, &claim).await;
    finish(db, p, services, claim, observation).await
}
async fn observe(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    c: &Claim,
) -> std::result::Result<Observation, ReadError> {
    match &c.resource {
        ResourceIdentity::Fireflies {
            provider_user_id,
            mine: true,
        } => {
            let reference = c
                .credential_ref
                .as_deref()
                .ok_or(Reason::CredentialUnavailable)?;
            let secret = services
                .get(reference)
                .await
                .map_err(|_| Reason::CredentialUnavailable)?
                .ok_or(Reason::CredentialUnavailable)?;
            match &c.work {
                Work::Page { from, to, offset } => services
                    .reader
                    .list(&secret, provider_user_id, from, to, *offset)
                    .await
                    .map(Observation::Page),
                Work::Detail(source) => services
                    .reader
                    .detail(&secret, provider_user_id, &source.external_id)
                    .await
                    .map(Observation::Detail),
            }
        }
        ResourceIdentity::Fireflies { mine: false, .. } => Err(Reason::BindingUnavailable.into()),
        _ => {
            let Work::Detail(source) = &c.work else {
                return Err(Reason::BindingUnavailable.into());
            };
            let tx = db.begin().await.map_err(|_| Reason::ProviderUnavailable)?;
            access::check_source(&tx, p, services, &source.id, Use::Import)
                .await
                .map_err(|_| Reason::SourceDenied)?;
            let reference = parse(&source.external_id).map_err(|_| Reason::UnsupportedSchema)?;
            legacy::project(&tx, &c.resource, &reference, services)
                .await
                .map(Observation::Detail)
                .map_err(|e| match e {
                    Error::Intake(reason) => reason.into(),
                    _ => Reason::SourceDenied.into(),
                })
        }
    }
}
async fn finish(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    claim: Claim,
    observed: std::result::Result<Observation, ReadError>,
) -> Result<Import> {
    let tx = identity::begin_write(db, p.organization_id()).await?;
    let row = model(&tx, p, &claim.import_id).await?;
    let c = access::check_binding(&tx, p, services, &row.binding_id, Use::Import).await?;
    if !live(&row)
        || row.attempt_id.as_deref() != Some(&claim.attempt_id)
        || row.attempt_epoch != Some(claim.epoch)
        || c.binding.access_epoch != claim.epoch
    {
        return Err(error::conflict());
    }
    if let Work::Detail(claimed) = &claim.work {
        let source = access::source(&tx, p, &claimed.id).await?;
        if source.refresh_fence != claimed.refresh_fence || source.revision != claimed.revision {
            return Err(error::conflict());
        }
    }
    let mut offset = row.scan_offset;
    let mut scan_done = row.scan_done;
    let mut coverage = row.coverage.clone();
    let mut next_at = None;
    let mut failure = None;
    let mut failed = row.failed;
    let mut attempt_count = 0;
    let mut state = "queued";
    match observed {
        Ok(Observation::Page(items)) => {
            let size = items.len();
            for item in items {
                let source =
                    sources::ensure(&tx, p, &c, SourceKind::Fireflies, &item.id, &item.title)
                        .await?;
                add_item(&tx, p, &c.binding.id, &row.id, &source.id).await?;
            }
            offset = offset.checked_add(50).ok_or_else(storage)?;
            scan_done = size < 50 || offset >= 250;
            coverage = if size < 50 {
                "bounded_end"
            } else if offset >= 250 {
                "capped"
            } else {
                "partial"
            }
            .into();
        }
        Ok(Observation::Detail(value)) => {
            let Work::Detail(source) = &claim.work else {
                return Err(storage());
            };
            sources::store(&tx, p, &c, source, value).await?;
            tx.execute(sql("UPDATE business_intake_import_item SET state='complete' WHERE organization_id=? AND import_id=? AND source_id=? AND state='pending'",vec![p.organization_id().into(),row.id.clone().into(),source.id.clone().into()])).await?;
        }
        Err(error) => {
            failure = Some(code(error.reason)?);
            attempt_count = row.attempt_count;
            let retry = error.retry_after.filter(|_| row.attempt_count < 3);
            if let Some(seconds) = retry {
                state = "waiting";
                next_at = Some(future(seconds.clamp(1, 60)));
            } else {
                state = "failed";
            }
            if let Work::Detail(source) = &claim.work {
                tx.execute(sql("UPDATE business_intake_source SET access=?,access_until=NULL WHERE organization_id=? AND id=? AND refresh_fence=?",vec![if error.reason==Reason::SourceDenied {"denied"}else{"unverified"}.into(),p.organization_id().into(),source.id.clone().into(),source.refresh_fence.into()])).await?;
                if state == "failed" {
                    tx.execute(sql("UPDATE business_intake_import_item SET state='failed' WHERE organization_id=? AND import_id=? AND source_id=?",vec![p.organization_id().into(),row.id.clone().into(),source.id.clone().into()])).await?;
                    failed = next(failed)?;
                }
            }
        }
    }
    let counts=tx.query_one(sql("SELECT count(*) AS discovered,coalesce(sum(state='complete'),0) AS completed,coalesce(sum(state='pending'),0) AS pending FROM business_intake_import_item WHERE organization_id=? AND import_id=?",vec![p.organization_id().into(),row.id.clone().into()])).await?.ok_or_else(storage)?;
    let discovered: i64 = counts.try_get("", "discovered")?;
    let completed: i64 = counts.try_get("", "completed")?;
    if state == "queued" && scan_done && counts.try_get::<i64>("", "pending")? == 0 {
        state = "complete";
        if coverage == "not_started" {
            coverage = "bounded_end".into();
        }
    }
    if coverage == "not_started" && discovered > 0 {
        coverage = "partial".into();
    }
    tx.execute(sql("UPDATE business_intake_import SET revision=?,state=?,coverage=?,scan_offset=?,scan_done=?,discovered=?,completed=?,failed=?,attempt_id=NULL,lease_until=NULL,attempt_count=?,next_attempt_at=?,error_code=?,updated_at=? WHERE organization_id=? AND id=? AND revision=? AND attempt_id=?",
        vec![next(row.revision)?.into(),state.into(),coverage.into(),offset.into(),scan_done.into(),discovered.into(),completed.into(),failed.into(),attempt_count.into(),next_at.into(),failure.into(),now().into(),p.organization_id().into(),row.id.clone().into(),row.revision.into(),claim.attempt_id.into()])).await?;
    audit(&tx, p, "import_observed", &row.id, next(row.revision)?).await?;
    let result = view(model(&tx, p, &row.id).await?)?;
    tx.commit().await?;
    Ok(result)
}
