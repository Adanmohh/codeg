//! Intersection of the actual Principal, current source grant and immutable binding.
use super::{
    common::*,
    error::{self, Error, Reason},
    legacy, records,
    services::Services,
    types::*,
};
use crate::business_identity::{
    self as identity, Domain, IdentityError, Member, MemberKind, Permission, Principal,
};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult, TransactionTrait,
};
use serde_json::json;

#[derive(Clone, Copy)]
pub(super) enum Use {
    Read,
    Import,
    Triage,
    Publish(Domain),
}
pub(super) struct Checked {
    pub binding: records::Binding,
    pub member: Member,
    pub grant: records::Grant,
    pub publication_domains: Vec<Domain>,
}
pub(super) async fn human(tx: &DatabaseTransaction, p: &Principal) -> Result<Member> {
    let member = identity::authorize(tx, p, p.organization_id(), Permission::Read, None).await?;
    if member.kind != MemberKind::Human {
        return Err(IdentityError::Forbidden.into());
    }
    Ok(member)
}
pub(super) async fn require_setup(tx: &DatabaseTransaction, p: &Principal) -> Result<Member> {
    let member = human(tx, p).await?;
    identity::authorize(
        tx,
        p,
        p.organization_id(),
        Permission::ManageTenantSettings,
        None,
    )
    .await?;
    Ok(member)
}
fn manages(p: &Principal, member: &Member, b: &records::Binding) -> Result<bool> {
    Ok(member.allows(Permission::ManageTenantSettings, None)
        && member.allows(Permission::Contribute, Some(vocabulary(&b.domain)?))
        && (p.is_operator() || b.kind == "fireflies"))
}
pub(super) async fn require_binding_setup(
    tx: &DatabaseTransaction,
    p: &Principal,
    b: &records::Binding,
) -> Result<Member> {
    let member = require_setup(tx, p).await?;
    if !manages(p, &member, b)? {
        return Err(error::missing());
    }
    Ok(member)
}
pub(super) fn setup_domain(member: &Member, domain: Domain, publication: &[Domain]) -> Result<()> {
    if !member.allows(Permission::Contribute, Some(domain))
        || publication
            .iter()
            .any(|d| !member.allows(Permission::Create, Some(*d)))
    {
        return Err(IdentityError::Forbidden.into());
    }
    Ok(())
}
pub(super) async fn binding(
    tx: &DatabaseTransaction,
    p: &Principal,
    id: &str,
) -> Result<records::Binding> {
    uuid(id)?;
    records::Binding::find_by_statement(sql(
        "SELECT * FROM business_intake_binding WHERE organization_id=? AND id=?",
        vec![p.organization_id().into(), id.into()],
    ))
    .one(tx)
    .await?
    .ok_or_else(error::missing)
}
async fn grant(
    tx: &DatabaseTransaction,
    p: &Principal,
    binding_id: &str,
) -> Result<Option<records::Grant>> {
    Ok(records::Grant::find_by_statement(sql("SELECT * FROM business_intake_grant WHERE organization_id=? AND binding_id=? AND member_id=?",
        vec![p.organization_id().into(),binding_id.into(),p.member_id().into()])).one(tx).await?)
}
fn readable(member: &Member, grant: &records::Grant, domain: Domain) -> bool {
    grant.state == "active"
        && grant.can_read
        && unexpired(grant.expires_at.as_deref())
        && member.allows(Permission::Read, Some(domain))
}
pub(super) async fn owner(
    tx: &DatabaseTransaction,
    p: &Principal,
    id: &str,
    domain: Domain,
) -> Result<Member> {
    uuid(id)?;
    let owner = identity::active_reference(tx, p.organization_id(), id, domain).await?;
    if owner.kind != MemberKind::Human || !owner.allows(Permission::Contribute, Some(domain)) {
        return Err(error::missing());
    }
    Ok(owner)
}
async fn ready(
    tx: &DatabaseTransaction,
    p: &Principal,
    b: &records::Binding,
    services: &Services,
) -> Result<()> {
    if !b.enabled {
        return Err(Reason::BindingDisabled.into());
    }
    let domain = vocabulary(&b.domain)?;
    match owner(tx, p, &b.source_owner_id, domain).await {
        Ok(owner) if owner.revision == b.owner_authority_revision => {}
        Ok(_) | Err(Error::Identity(IdentityError::NotFound)) => {
            return Err(Reason::BindingUnavailable.into())
        }
        Err(e) => return Err(e),
    }
    match parse::<ResourceIdentity>(&b.resource_json)? {
        ResourceIdentity::Fireflies { mine: true, .. } => {
            let reference = b
                .credential_ref
                .as_deref()
                .ok_or(Reason::CredentialUnavailable)?;
            if services.get(reference).await?.is_none() {
                return Err(Reason::CredentialUnavailable.into());
            }
        }
        ResourceIdentity::Fireflies { mine: false, .. } => {
            return Err(Reason::BindingUnavailable.into())
        }
        resource => {
            legacy::check(tx, &resource, services).await?;
        }
    }
    Ok(())
}
pub(super) async fn check_binding(
    tx: &DatabaseTransaction,
    p: &Principal,
    services: &Services,
    id: &str,
    usage: Use,
) -> Result<Checked> {
    let member = human(tx, p).await?;
    let binding = binding(tx, p, id).await?;
    let domain: Domain = vocabulary(&binding.domain)?;
    let grant = grant(tx, p, id)
        .await?
        .filter(|g| readable(&member, g, domain))
        .ok_or_else(error::missing)?;
    ready(tx, p, &binding, services).await?;
    let contribute = member.allows(Permission::Contribute, Some(domain));
    let ceiling: Vec<Domain> = parse(&binding.publication_domains)?;
    let publication_domains: Vec<Domain> = parse::<Vec<Domain>>(&grant.publication_domains)?
        .into_iter()
        .filter(|d| {
            binding.retained_task_text
                && contribute
                && grant.can_triage
                && ceiling.contains(d)
                && member.allows(Permission::Create, Some(*d))
        })
        .collect();
    let allowed = match usage {
        Use::Read => true,
        Use::Import => contribute && grant.can_import,
        Use::Triage => contribute && grant.can_triage,
        Use::Publish(domain) => publication_domains.contains(&domain),
    };
    if !allowed {
        return Err(if matches!(usage, Use::Publish(_)) {
            Reason::PublicationNotAllowed.into()
        } else {
            IdentityError::Forbidden.into()
        });
    }
    Ok(Checked {
        binding,
        member,
        grant,
        publication_domains,
    })
}
pub(super) async fn source(
    tx: &DatabaseTransaction,
    p: &Principal,
    id: &str,
) -> Result<records::Source> {
    uuid(id)?;
    records::Source::find_by_statement(sql(
        "SELECT * FROM business_intake_source WHERE organization_id=? AND id=?",
        vec![p.organization_id().into(), id.into()],
    ))
    .one(tx)
    .await?
    .ok_or_else(error::missing)
}
pub(super) async fn check_source(
    tx: &DatabaseTransaction,
    p: &Principal,
    services: &Services,
    id: &str,
    usage: Use,
) -> Result<(records::Source, Checked)> {
    let source = source(tx, p, id).await?;
    let checked = check_binding(tx, p, services, &source.binding_id, usage).await?;
    Ok((source, checked))
}
pub(super) async fn credential_state(
    tx: &DatabaseTransaction,
    b: &records::Binding,
    services: &Services,
) -> Result<CredentialState> {
    let present = match b.credential_ref.as_deref() {
        Some(reference) => services.get(reference).await?.is_some(),
        None => legacy::check(tx, &parse(&b.resource_json)?, services)
            .await
            .is_ok(),
    };
    Ok(if present {
        CredentialState::Present
    } else {
        CredentialState::Missing
    })
}
pub(super) async fn admin(
    tx: &DatabaseTransaction,
    b: &records::Binding,
    services: &Services,
) -> Result<BindingAdmin> {
    Ok(BindingAdmin {
        id: b.id.clone(),
        kind: vocabulary(&b.kind)?,
        label: b.label.clone(),
        domain: vocabulary(&b.domain)?,
        source_owner_id: b.source_owner_id.clone(),
        revision: b.revision,
        access_epoch: b.access_epoch,
        enabled: b.enabled,
        publication_domains: parse(&b.publication_domains)?,
        retained_task_text: b.retained_task_text,
        credential_state: credential_state(tx, b, services).await?,
        resource: parse(&b.resource_json)?,
    })
}
async fn view(
    tx: &DatabaseTransaction,
    p: &Principal,
    b: records::Binding,
    services: &Services,
    setup: bool,
) -> Result<BindingView> {
    let capabilities = match check_binding(tx, p, services, &b.id, Use::Read).await {
        Ok(c) => BindingCapabilities {
            read: true,
            import: c.grant.can_import
                && c.member
                    .allows(Permission::Contribute, Some(vocabulary(&b.domain)?)),
            triage: c.grant.can_triage
                && c.member
                    .allows(Permission::Contribute, Some(vocabulary(&b.domain)?)),
            publication_domains: c.publication_domains,
        },
        Err(Error::Identity(IdentityError::Unauthorized)) => {
            return Err(IdentityError::Unauthorized.into())
        }
        Err(Error::Identity(IdentityError::Database(e))) => return Err(e.into()),
        Err(_) => BindingCapabilities {
            read: false,
            import: false,
            triage: false,
            publication_domains: vec![],
        },
    };
    let admin = if setup {
        Some(admin(tx, &b, services).await?)
    } else {
        None
    };
    Ok(BindingView {
        binding: BindingSummary {
            id: b.id.clone(),
            kind: vocabulary(&b.kind)?,
            label: b.label.clone(),
            domain: vocabulary(&b.domain)?,
            revision: b.revision,
            access_epoch: b.access_epoch,
            enabled: b.enabled,
            credential_state: credential_state(tx, &b, services).await?,
            capabilities,
        },
        admin,
    })
}
pub(super) async fn list(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: PageInput,
) -> Result<BindingList> {
    let offset = page(input.page)?;
    let tx = db.begin().await?;
    let member = human(&tx, p).await?;
    let setup = member.allows(Permission::ManageTenantSettings, None);
    if setup {
        require_setup(&tx, p).await?;
    }
    let setup_domains = Domain::ALL
        .into_iter()
        .filter(|d| setup && member.allows(Permission::Contribute, Some(*d)))
        .collect::<Vec<_>>();
    let domains = Domain::ALL
        .into_iter()
        .filter(|d| member.allows(Permission::Read, Some(*d)))
        .collect::<Vec<_>>();
    let rows = records::Binding::find_by_statement(sql(
        "SELECT b.* FROM business_intake_binding b WHERE b.organization_id=? AND ((b.domain IN (SELECT value FROM json_each(?)) AND (? OR b.kind='fireflies')) OR (b.domain IN (SELECT value FROM json_each(?)) AND EXISTS(SELECT 1 FROM business_intake_grant g WHERE g.organization_id=b.organization_id AND g.binding_id=b.id AND g.member_id=? AND g.state='active' AND g.can_read=1 AND (g.expires_at IS NULL OR g.expires_at>?)))) ORDER BY b.id LIMIT 51 OFFSET ?",
        vec![p.organization_id().into(),json(&setup_domains)?.into(),p.is_operator().into(),json(&domains)?.into(),p.member_id().into(),now().into(),offset.into()])).all(&tx).await?;
    let has_more = rows.len() > 50;
    let mut items = Vec::new();
    for b in rows.into_iter().take(50) {
        let admin = manages(p, &member, &b)?;
        items.push(view(&tx, p, b, services, admin).await?);
    }
    let setup_kinds = if setup_domains.is_empty() {
        vec![]
    } else if p.is_operator() {
        vec![
            SourceKind::Fireflies,
            SourceKind::Email,
            SourceKind::HafidhTestflight,
        ]
    } else {
        vec![SourceKind::Fireflies]
    };
    Ok(BindingList {
        can_manage_setup: !setup_kinds.is_empty(),
        setup_kinds,
        setup_domains,
        items,
        page: input.page,
        has_more,
    })
}
pub(super) async fn status(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: BindingInput,
) -> Result<BindingView> {
    let tx = db.begin().await?;
    let member = human(&tx, p).await?;
    let b = binding(&tx, p, &input.binding_id).await?;
    let setup = manages(p, &member, &b)?;
    if setup {
        require_binding_setup(&tx, p, &b).await?;
    } else {
        let domain = vocabulary(&b.domain)?;
        if !grant(&tx, p, &b.id)
            .await?
            .is_some_and(|g| readable(&member, &g, domain))
        {
            return Err(error::missing());
        }
    }
    view(&tx, p, b, services, setup).await
}
pub(super) fn validate_publication(domains: &[Domain], retained: bool) -> Result<()> {
    if domains.len() > 6
        || (!domains.is_empty() && !retained)
        || domains
            .iter()
            .enumerate()
            .any(|(i, d)| domains[..i].contains(d))
    {
        return Err(error::invalid());
    }
    Ok(())
}
pub(super) async fn bump(
    tx: &DatabaseTransaction,
    p: &Principal,
    b: &records::Binding,
) -> Result<()> {
    let result = tx.execute(sql("UPDATE business_intake_binding SET revision=?,access_epoch=?,updated_at=? WHERE organization_id=? AND id=? AND revision=?",
        vec![next(b.revision)?.into(),next(b.access_epoch)?.into(),now().into(),p.organization_id().into(),b.id.clone().into(),b.revision.into()])).await?;
    if result.rows_affected() != 1 {
        return Err(error::conflict());
    }
    Ok(())
}
fn grant_dto(binding_id: &str, g: records::Grant) -> Result<Grant> {
    Ok(Grant {
        id: g.id,
        binding_id: binding_id.into(),
        member_id: g.member_id,
        revision: g.revision,
        state: vocabulary(&g.state)?,
        scope: GrantScope::BindingCurrentAndFutureSources,
        read: g.can_read,
        import: g.can_import,
        triage: g.can_triage,
        publication_domains: parse(&g.publication_domains)?,
        expires_at: g.expires_at,
    })
}
async fn grant_by_id(
    tx: &DatabaseTransaction,
    p: &Principal,
    binding_id: &str,
    grant_id: &str,
) -> Result<records::Grant> {
    records::Grant::find_by_statement(sql(
        "SELECT * FROM business_intake_grant WHERE organization_id=? AND binding_id=? AND id=?",
        vec![
            p.organization_id().into(),
            binding_id.into(),
            grant_id.into(),
        ],
    ))
    .one(tx)
    .await?
    .ok_or_else(error::missing)
}
pub(super) async fn grants_list(
    db: &DatabaseConnection,
    p: &Principal,
    _services: &Services,
    input: BindingPageInput,
) -> Result<GrantPage> {
    let offset = page(input.page)?;
    let tx = db.begin().await?;
    require_setup(&tx, p).await?;
    require_binding_setup(&tx, p, &binding(&tx, p, &input.binding_id).await?).await?;
    let rows = records::Grant::find_by_statement(sql("SELECT * FROM business_intake_grant WHERE organization_id=? AND binding_id=? ORDER BY id LIMIT 51 OFFSET ?",vec![p.organization_id().into(),input.binding_id.clone().into(),offset.into()])).all(&tx).await?;
    let has_more = rows.len() > 50;
    let items = rows
        .into_iter()
        .take(50)
        .map(|g| grant_dto(&input.binding_id, g))
        .collect::<Result<_>>()?;
    Ok(GrantPage {
        items,
        page: input.page,
        has_more,
    })
}
pub(super) async fn grants_upsert(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: UpsertGrantInput,
) -> Result<GrantResult> {
    uuid(&input.operation_id)?;
    uuid(&input.member_id)?;
    revision(input.expected_binding_revision)?;
    if let Some(r) = input.expected_grant_revision {
        revision(r)?;
    }
    validate_publication(&input.publication_domains, input.read && input.triage)?;
    if (input.import || input.triage) && !input.read {
        return Err(error::invalid());
    }
    let expires = input
        .expires_at
        .as_deref()
        .map(instant)
        .transpose()?
        .map(|v| v.to_rfc3339());
    if !unexpired(expires.as_deref()) {
        return Err(error::invalid());
    }
    let digest = digest(
        &json!({"binding":input.binding_id,"revision":input.expected_binding_revision,"member":input.member_id,"grantRevision":input.expected_grant_revision,"scope":input.scope,"read":input.read,"import":input.import,"triage":input.triage,"domains":input.publication_domains,"expires":expires}),
    )?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    require_setup(&tx, p).await?;
    let b = binding(&tx, p, &input.binding_id).await?;
    let manager = require_binding_setup(&tx, p, &b).await?;
    setup_domain(&manager, vocabulary(&b.domain)?, &input.publication_domains)?;
    if let Some(id) = receipt(&tx, p, &input.operation_id, "grants/upsert", &digest).await? {
        return Ok(GrantResult {
            binding: admin(&tx, &b, services).await?,
            grant: grant_dto(&b.id, grant_by_id(&tx, p, &b.id, &id).await?)?,
        });
    }
    if b.revision != input.expected_binding_revision {
        return Err(error::conflict());
    }
    let d: Domain = vocabulary(&b.domain)?;
    let member = identity::active_reference(&tx, p.organization_id(), &input.member_id, d).await?;
    if member.kind != MemberKind::Human
        || ((input.import || input.triage) && !member.allows(Permission::Contribute, Some(d)))
    {
        return Err(IdentityError::Forbidden.into());
    }
    let ceiling: Vec<Domain> = parse(&b.publication_domains)?;
    if input
        .publication_domains
        .iter()
        .any(|d| !ceiling.contains(d) || !member.allows(Permission::Create, Some(*d)))
    {
        return Err(Reason::PublicationNotAllowed.into());
    }
    let old=records::Grant::find_by_statement(sql("SELECT * FROM business_intake_grant WHERE organization_id=? AND binding_id=? AND member_id=?",vec![p.organization_id().into(),b.id.clone().into(),input.member_id.clone().into()])).one(&tx).await?;
    if old.as_ref().map(|g| g.revision) != input.expected_grant_revision {
        return Err(error::conflict());
    }
    let gid = old.as_ref().map(|g| g.id.clone()).unwrap_or_else(id);
    let rev = old
        .as_ref()
        .map(|g| next(g.revision))
        .transpose()?
        .unwrap_or(1);
    tx.execute(sql("INSERT INTO business_intake_grant(id,organization_id,binding_id,member_id,revision,state,scope,can_read,can_import,can_triage,publication_domains,expires_at) VALUES(?,?,?,?,?,'active','binding_current_and_future_sources',?,?,?,?,?) ON CONFLICT(organization_id,binding_id,member_id) DO UPDATE SET revision=excluded.revision,state='active',can_read=excluded.can_read,can_import=excluded.can_import,can_triage=excluded.can_triage,publication_domains=excluded.publication_domains,expires_at=excluded.expires_at",
        vec![gid.clone().into(),p.organization_id().into(),b.id.clone().into(),input.member_id.into(),rev.into(),input.read.into(),input.import.into(),input.triage.into(),json(&input.publication_domains)?.into(),expires.into()])).await?;
    bump(&tx, p, &b).await?;
    audit(&tx, p, "grant_upserted", &gid, rev).await?;
    record(&tx, p, &input.operation_id, "grants/upsert", &digest, &gid).await?;
    let result = GrantResult {
        binding: admin(&tx, &binding(&tx, p, &b.id).await?, services).await?,
        grant: grant_dto(&b.id, grant_by_id(&tx, p, &b.id, &gid).await?)?,
    };
    tx.commit().await?;
    Ok(result)
}
pub(super) async fn grants_revoke(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: RevokeGrantInput,
) -> Result<GrantResult> {
    uuid(&input.operation_id)?;
    uuid(&input.grant_id)?;
    revision(input.expected_binding_revision)?;
    revision(input.expected_grant_revision)?;
    let digest = digest(
        &json!({"binding":input.binding_id,"bindingRevision":input.expected_binding_revision,"grant":input.grant_id,"revision":input.expected_grant_revision}),
    )?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    require_setup(&tx, p).await?;
    let b = binding(&tx, p, &input.binding_id).await?;
    require_binding_setup(&tx, p, &b).await?;
    let g = grant_by_id(&tx, p, &b.id, &input.grant_id).await?;
    if receipt(&tx, p, &input.operation_id, "grants/revoke", &digest)
        .await?
        .is_some()
    {
        return Ok(GrantResult {
            binding: admin(&tx, &b, services).await?,
            grant: grant_dto(&b.id, g)?,
        });
    }
    if b.revision != input.expected_binding_revision || g.revision != input.expected_grant_revision
    {
        return Err(error::conflict());
    }
    tx.execute(sql("UPDATE business_intake_grant SET revision=?,state='revoked',can_read=0,can_import=0,can_triage=0,publication_domains='[]' WHERE organization_id=? AND binding_id=? AND id=? AND revision=?",vec![next(g.revision)?.into(),p.organization_id().into(),b.id.clone().into(),g.id.clone().into(),g.revision.into()])).await?;
    bump(&tx, p, &b).await?;
    audit(&tx, p, "grant_revoked", &g.id, next(g.revision)?).await?;
    record(&tx, p, &input.operation_id, "grants/revoke", &digest, &g.id).await?;
    let result = GrantResult {
        binding: admin(&tx, &binding(&tx, p, &b.id).await?, services).await?,
        grant: grant_dto(&b.id, grant_by_id(&tx, p, &b.id, &g.id).await?)?,
    };
    tx.commit().await?;
    Ok(result)
}
