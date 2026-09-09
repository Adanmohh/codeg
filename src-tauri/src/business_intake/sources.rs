//! Immutable normalized versions and checked, freshness-limited disclosures.
use super::{
    access::{self, Checked, Use},
    common::*,
    error::{self, Error, Reason},
    fireflies::Normalized,
    legacy, records,
    services::Services,
    types::*,
};
use crate::business_identity::Principal;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult, TransactionTrait,
};

pub(super) async fn version(
    tx: &DatabaseTransaction,
    p: &Principal,
    source_id: &str,
    revision: i64,
) -> Result<records::Version> {
    records::Version::find_by_statement(sql("SELECT digest,provider_revision FROM business_intake_version WHERE organization_id=? AND source_id=? AND revision=?",vec![p.organization_id().into(),source_id.into(),revision.into()])).one(tx).await?.ok_or_else(storage)
}
pub(super) async fn inspect(
    tx: &DatabaseTransaction,
    p: &Principal,
    services: &Services,
    s: &records::Source,
    c: &Checked,
) -> Result<SourceSummary> {
    let valid = s.revision.is_some()
        && s.access == "fresh"
        && s.observed_epoch == Some(c.binding.access_epoch)
        && s.authorization_epoch == Some(p.authorization_epoch())
        && unexpired(c.grant.expires_at.as_deref())
        && s.access_until
            .as_deref()
            .is_some_and(|v| unexpired(Some(v)));
    let mut access = if valid {
        AccessState::Fresh
    } else if s.access == "denied" {
        AccessState::Denied
    } else if s.revision.is_none() {
        AccessState::Unverified
    } else {
        AccessState::Expired
    };
    if valid && s.kind != "fireflies" {
        let current = legacy::project(
            tx,
            &parse(&c.binding.resource_json)?,
            &parse(&s.external_id)?,
            services,
        )
        .await;
        access = match current {
            Ok(current)
                if version(tx, p, &s.id, s.revision.ok_or_else(storage)?)
                    .await?
                    .digest
                    == current.digest()? =>
            {
                AccessState::Fresh
            }
            Ok(_)
            | Err(Error::Intake(
                Reason::SourceExpired | Reason::SourceDenied | Reason::UnsupportedSchema,
            )) => AccessState::Expired,
            Err(e) => return Err(e),
        };
    }
    let access_valid_until = match (&s.access_until, &c.grant.expires_at) {
        (Some(a), Some(g)) => Some(if instant(g)? < instant(a)? {
            g.clone()
        } else {
            a.clone()
        }),
        (a, _) => a.clone(),
    };
    Ok(SourceSummary {
        id: s.id.clone(),
        binding_id: s.binding_id.clone(),
        kind: vocabulary(&s.kind)?,
        title: s.title.clone(),
        revision: s.revision,
        observed_at: s.observed_at.clone(),
        access_valid_until,
        access,
        content: vocabulary(&s.content)?,
        summary: vocabulary(&s.summary)?,
        provider_summary_status: s.provider_summary_status.clone(),
        requires_refresh: access != AccessState::Fresh,
    })
}
pub(super) async fn fresh(
    tx: &DatabaseTransaction,
    p: &Principal,
    services: &Services,
    s: &records::Source,
    c: &Checked,
) -> Result<SourceSummary> {
    let view = inspect(tx, p, services, s, c).await?;
    if view.access != AccessState::Fresh {
        return Err(Reason::SourceExpired.into());
    }
    Ok(view)
}
pub(super) async fn list(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: BindingPageInput,
) -> Result<SourcePage> {
    let offset = page(input.page)?;
    let tx = db.begin().await?;
    let c = access::check_binding(&tx, p, services, &input.binding_id, Use::Read).await?;
    let rows=records::Source::find_by_statement(sql("SELECT * FROM business_intake_source WHERE organization_id=? AND binding_id=? ORDER BY id LIMIT 51 OFFSET ?",vec![p.organization_id().into(),c.binding.id.clone().into(),offset.into()])).all(&tx).await?;
    let has_more = rows.len() > 50;
    let mut items = Vec::new();
    for row in rows.into_iter().take(50) {
        items.push(inspect(&tx, p, services, &row, &c).await?);
    }
    Ok(SourcePage {
        items,
        page: input.page,
        has_more,
    })
}
pub(super) async fn get(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: SourceInput,
) -> Result<SourceDetail> {
    let tx = db.begin().await?;
    let (s, c) = access::check_source(&tx, p, services, &input.source_id, Use::Read).await?;
    let source = inspect(&tx, p, services, &s, &c).await?;
    let readable = source.access == AccessState::Fresh;
    let passages = if readable {
        passages(&tx, p, &s.id, s.revision.ok_or_else(storage)?).await?
    } else {
        vec![]
    };
    let row=tx.query_one(sql("SELECT count(*) AS count FROM business_intake_candidate WHERE organization_id=? AND source_id=?",vec![p.organization_id().into(),s.id.into()])).await?.ok_or_else(storage)?;
    Ok(SourceDetail {
        source,
        disclosure: if readable {
            Disclosure::Fresh
        } else {
            Disclosure::MetadataOnly
        },
        passages,
        candidate_count: row
            .try_get::<i64>("", "count")?
            .try_into()
            .map_err(|_| storage())?,
    })
}
pub(super) async fn passages(
    tx: &DatabaseTransaction,
    p: &Principal,
    source_id: &str,
    source_revision: i64,
) -> Result<Vec<Passage>> {
    let rows=records::Passage::find_by_statement(sql("SELECT * FROM business_intake_passage WHERE organization_id=? AND source_id=? AND source_revision=? ORDER BY ordinal",vec![p.organization_id().into(),source_id.into(),source_revision.into()])).all(tx).await?;
    rows.into_iter()
        .map(|r| {
            Ok(Passage {
                id: r.id,
                source_revision: r.source_revision,
                kind: vocabulary(&r.kind)?,
                text: r.text,
                index: r.provider_index,
                start: r.start,
                end: r.end,
            })
        })
        .collect()
}
pub(super) async fn selection(
    tx: &DatabaseTransaction,
    p: &Principal,
    source_id: &str,
    source_revision: i64,
    ids: &[String],
) -> Result<Vec<Passage>> {
    if ids.is_empty()
        || ids.len() > 20
        || ids.iter().enumerate().any(|(i, id)| ids[..i].contains(id))
    {
        return Err(error::invalid());
    }
    for id in ids {
        uuid(id)?;
    }
    let mut rows = passages(tx, p, source_id, source_revision).await?;
    rows.retain(|r| ids.contains(&r.id));
    if rows.len() != ids.len()
        || rows.iter().map(|r| r.text.chars().count()).sum::<usize>() > 20_000
    {
        return Err(error::invalid());
    }
    rows.sort_by_key(|r| ids.iter().position(|id| *id == r.id));
    Ok(rows)
}
pub(super) async fn ensure(
    tx: &DatabaseTransaction,
    p: &Principal,
    c: &Checked,
    kind: SourceKind,
    external_id: &str,
    title: &str,
) -> Result<records::Source> {
    if code(kind)? != c.binding.kind {
        return Err(error::invalid());
    }
    tx.execute(sql("INSERT INTO business_intake_source(id,organization_id,binding_id,kind,external_id,title) VALUES(?,?,?,?,?,?) ON CONFLICT(organization_id,binding_id,kind,external_id) DO NOTHING",
        vec![id().into(),p.organization_id().into(),c.binding.id.clone().into(),code(kind)?.into(),external_id.into(),title.into()])).await?;
    records::Source::find_by_statement(sql("SELECT * FROM business_intake_source WHERE organization_id=? AND binding_id=? AND kind=? AND external_id=?",vec![p.organization_id().into(),c.binding.id.clone().into(),code(kind)?.into(),external_id.into()])).one(tx).await?.ok_or_else(storage)
}
pub(super) async fn fence(
    tx: &DatabaseTransaction,
    p: &Principal,
    s: &records::Source,
) -> Result<records::Source> {
    let result=tx.execute(sql("UPDATE business_intake_source SET refresh_fence=?,access='unverified',access_until=NULL WHERE organization_id=? AND id=? AND refresh_fence=?",
        vec![next(s.refresh_fence)?.into(),p.organization_id().into(),s.id.clone().into(),s.refresh_fence.into()])).await?;
    if result.rows_affected() != 1 {
        return Err(error::conflict());
    }
    access::source(tx, p, &s.id).await
}
pub(super) async fn store(
    tx: &DatabaseTransaction,
    p: &Principal,
    c: &Checked,
    claimed: &records::Source,
    value: Normalized,
) -> Result<()> {
    let s = access::source(tx, p, &claimed.id).await?;
    if s.binding_id != c.binding.id
        || s.refresh_fence != claimed.refresh_fence
        || s.revision != claimed.revision
    {
        return Err(error::conflict());
    }
    let fingerprint = value.digest()?;
    let changed = match s.revision {
        Some(rev) => version(tx, p, &s.id, rev).await?.digest != fingerprint,
        None => true,
    };
    let current_revision = if changed {
        next(s.revision.unwrap_or(0))?
    } else {
        s.revision.ok_or_else(storage)?
    };
    let mut until = chrono::Utc::now() + chrono::Duration::seconds(300);
    for value in [value.expires_at.as_deref(), c.grant.expires_at.as_deref()]
        .into_iter()
        .flatten()
    {
        until = until.min(instant(value)?);
    }
    if until <= chrono::Utc::now() {
        return Err(Reason::SourceExpired.into());
    }
    if changed {
        tx.execute(sql("INSERT INTO business_intake_version(id,organization_id,source_id,revision,digest,normalization_version,observed_at,provider_revision,title) VALUES(?,?,?,?,?,1,?,?,?)",
            vec![id().into(),p.organization_id().into(),s.id.clone().into(),current_revision.into(),fingerprint.into(),now().into(),value.provider_revision.into(),value.title.clone().into()])).await?;
        for (ordinal, passage) in value.passages.iter().enumerate() {
            tx.execute(sql("INSERT INTO business_intake_passage(id,organization_id,source_id,source_revision,ordinal,kind,text,provider_index,start,end) VALUES(?,?,?,?,?,?,?,?,?,?)",
                vec![id().into(),p.organization_id().into(),s.id.clone().into(),current_revision.into(),i64::try_from(ordinal).map_err(|_|storage())?.into(),code(passage.kind)?.into(),passage.text.clone().into(),passage.index.into(),passage.start.into(),passage.end.into()])).await?;
        }
        // Preserve the exact edited preview and old selected passages. The next
        // human must explicitly rebase; terminal candidates are immutable.
        let changed_rows=records::Candidate::find_by_statement(sql("SELECT * FROM business_intake_candidate WHERE organization_id=? AND source_id=? AND state='pending'",vec![p.organization_id().into(),s.id.clone().into()])).all(tx).await?;
        for row in changed_rows {
            tx.execute(sql("UPDATE business_intake_candidate SET revision=?,updated_by=?,updated_at=? WHERE organization_id=? AND id=? AND revision=?",vec![next(row.revision)?.into(),p.member_id().into(),now().into(),p.organization_id().into(),row.id.into(),row.revision.into()])).await?;
        }
    }
    let mut seeded = s.seeded;
    if !s.seeded && !value.passages.is_empty() {
        let all = passages(tx, p, &s.id, current_revision).await?;
        let summary: Vec<_> = all
            .iter()
            .filter(|p| p.kind == PassageKind::Summary)
            .collect();
        let choices = if summary.is_empty() {
            all.iter().collect()
        } else {
            summary
        };
        let mut selected = Vec::new();
        let mut length = 0;
        for row in choices {
            if selected.len() == 20 || length + row.text.chars().count() > 20_000 {
                break;
            }
            length += row.text.chars().count();
            selected.push(row.id.clone());
        }
        if !selected.is_empty() {
            tx.execute(sql("INSERT INTO business_intake_candidate(id,organization_id,source_id,revision,source_revision,prepared_epoch,authorization_epoch,state,origin,passage_ids,created_by,updated_by,created_at,updated_at) VALUES(?,?,?,1,?,?,?,'pending','source_review',?,?,?,?,?)",
                vec![id().into(),p.organization_id().into(),s.id.clone().into(),current_revision.into(),c.binding.access_epoch.into(),p.authorization_epoch().into(),json(&selected)?.into(),p.member_id().into(),p.member_id().into(),now().into(),now().into()])).await?;
            seeded = true;
        }
    }
    let result=tx.execute(sql("UPDATE business_intake_source SET title=?,revision=?,observed_epoch=?,authorization_epoch=?,observed_at=?,access_until=?,access='fresh',content=?,summary=?,provider_summary_status=?,seeded=? WHERE organization_id=? AND id=? AND refresh_fence=? AND revision IS ?",
        vec![value.title.into(),current_revision.into(),c.binding.access_epoch.into(),p.authorization_epoch().into(),now().into(),until.to_rfc3339().into(),code(value.content)?.into(),code(value.summary)?.into(),value.provider_status.into(),seeded.into(),p.organization_id().into(),s.id.clone().into(),claimed.refresh_fence.into(),claimed.revision.into()])).await?;
    if result.rows_affected() != 1 {
        return Err(error::conflict());
    }
    audit(tx, p, "source_observed", &s.id, current_revision).await
}
