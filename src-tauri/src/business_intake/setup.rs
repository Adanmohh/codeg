//! Protected setup: reserve an immutable secret reference, verify, then activate.
//! Secret I/O and SQLite are separate resources. Only activation is transactional.
use super::{
    access,
    common::*,
    error::{self, Error, Reason},
    legacy, records,
    services::Services,
    types::*,
};
use crate::{
    business_identity::{self as identity, Domain, Principal},
    ops::Operator,
};
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, FromQueryResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    operation: String,
    label: String,
    domain: Domain,
    owner_id: String,
    enabled: bool,
    publication_domains: Vec<Domain>,
    retained_task_text: bool,
    resource: Option<ResourceIdentity>,
}
fn label(value: &str) -> Result<()> {
    if !text(value, 120, true) || value.chars().any(char::is_control) {
        Err(error::invalid())
    } else {
        Ok(())
    }
}
async fn attempt(
    tx: &DatabaseTransaction,
    p: &Principal,
    operation_id: &str,
) -> Result<Option<records::Setup>> {
    Ok(records::Setup::find_by_statement(sql("SELECT * FROM business_intake_setup WHERE organization_id=? AND actor_id=? AND operation_id=?",
        vec![p.organization_id().into(),p.member_id().into(),operation_id.into()])).one(tx).await?)
}
async fn replay(
    tx: &DatabaseTransaction,
    p: &Principal,
    services: &Services,
    operation_id: &str,
    operation: &str,
    digest: &str,
) -> Result<Option<BindingAdmin>> {
    if let Some(id) = receipt(tx, p, operation_id, operation, digest).await? {
        return Ok(Some(
            access::admin(tx, &access::binding(tx, p, &id).await?, services).await?,
        ));
    }
    if let Some(a) = attempt(tx, p, operation_id).await? {
        if a.digest != digest || parse::<Plan>(&a.plan_json)?.operation != operation {
            return Err(error::conflict());
        }
        return Err(if a.state == "staged" && unexpired(Some(&a.expires_at)) {
            Reason::ImportBusy.into()
        } else {
            Reason::CredentialUnavailable.into()
        });
    }
    Ok(None)
}
async fn insert_binding(
    tx: &DatabaseTransaction,
    p: &Principal,
    id: &str,
    plan: &Plan,
    resource: &ResourceIdentity,
    owner_revision: i64,
    reference: Option<&str>,
) -> Result<()> {
    let kind = match resource {
        ResourceIdentity::Fireflies { .. } => SourceKind::Fireflies,
        ResourceIdentity::Email { .. } => SourceKind::Email,
        ResourceIdentity::HafidhTestflight { .. } => SourceKind::HafidhTestflight,
    };
    tx.execute(sql("INSERT INTO business_intake_binding(id,organization_id,kind,label,domain,source_owner_id,owner_authority_revision,resource_json,credential_ref,enabled,publication_domains,retained_task_text,created_at,updated_at) VALUES(?,?,?,?,?,?,?,?,?,0,?,?,?,?)",
        vec![id.into(),p.organization_id().into(),code(kind)?.into(),plan.label.clone().into(),code(plan.domain)?.into(),plan.owner_id.clone().into(),owner_revision.into(),json(resource)?.into(),reference.into(),json(&plan.publication_domains)?.into(),plan.retained_task_text.into(),now().into(),now().into()])).await?;
    audit(tx, p, "binding_created", id, 1).await
}
async fn update_binding(
    tx: &DatabaseTransaction,
    p: &Principal,
    b: &records::Binding,
    plan: &Plan,
    owner_revision: i64,
    reference: Option<&str>,
) -> Result<()> {
    let result=tx.execute(sql("UPDATE business_intake_binding SET label=?,enabled=?,publication_domains=?,retained_task_text=?,owner_authority_revision=?,credential_ref=?,revision=?,access_epoch=?,updated_at=? WHERE organization_id=? AND id=? AND revision=? AND access_epoch=?",
        vec![plan.label.clone().into(),plan.enabled.into(),json(&plan.publication_domains)?.into(),plan.retained_task_text.into(),owner_revision.into(),reference.into(),next(b.revision)?.into(),next(b.access_epoch)?.into(),now().into(),p.organization_id().into(),b.id.clone().into(),b.revision.into(),b.access_epoch.into()])).await?;
    if result.rows_affected() != 1 {
        return Err(error::conflict());
    }
    audit(tx, p, "binding_updated", &b.id, next(b.revision)?).await
}
async fn reserve(
    tx: &DatabaseTransaction,
    p: &Principal,
    operation_id: &str,
    digest: &str,
    binding_id: &str,
    base: Option<&records::Binding>,
    owner_revision: i64,
    plan: &Plan,
) -> Result<String> {
    let reference = format!("business-intake:{}", id());
    tx.execute(sql("INSERT INTO business_intake_setup(id,organization_id,actor_id,operation_id,digest,binding_id,base_revision,base_epoch,owner_authority_revision,plan_json,credential_ref,state,expires_at,created_at) VALUES(?,?,?,?,?,?,?,?,?,?,?,'staged',?,?)",
        vec![id().into(),p.organization_id().into(),p.member_id().into(),operation_id.into(),digest.into(),binding_id.into(),base.map(|b|b.revision).into(),base.map(|b|b.access_epoch).into(),owner_revision.into(),json(plan)?.into(),reference.clone().into(),future(15).into(),now().into()])).await?;
    Ok(reference)
}

pub(super) async fn create(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    operator: Option<&Operator>,
    input: CreateBindingInput,
) -> Result<BindingAdmin> {
    uuid(&input.operation_id)?;
    uuid(&input.source_owner_id)?;
    label(&input.label)?;
    access::validate_publication(&input.publication_domains, input.retained_task_text)?;
    let fingerprint = match &input.source {
        SourceSetup::Fireflies { api_key } => {
            json!({"kind":"fireflies","keyDigest":digest(&(&input.operation_id,&api_key.0))?})
        }
        SourceSetup::Email { inbox_id } => json!({"kind":"email","inboxId":inbox_id}),
        SourceSetup::HafidhTestflight { product_id } => {
            json!({"kind":"hafidh_testflight","productId":product_id})
        }
    };
    let plan = Plan {
        operation: "bindings/create".into(),
        label: input.label,
        domain: input.domain,
        owner_id: input.source_owner_id,
        enabled: false,
        publication_domains: input.publication_domains,
        retained_task_text: input.retained_task_text,
        resource: None,
    };
    let digest = digest(&json!({"plan":plan,"source":fingerprint}))?;
    cleanup(db, p, services).await?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    access::require_setup(&tx, p).await?;
    if let Some(result) = replay(
        &tx,
        p,
        services,
        &input.operation_id,
        &plan.operation,
        &digest,
    )
    .await?
    {
        return Ok(result);
    }
    let owner = access::owner(&tx, p, &plan.owner_id, plan.domain).await?;
    let binding_id = id();
    match input.source {
        SourceSetup::Fireflies { api_key } => {
            let reference = reserve(
                &tx,
                p,
                &input.operation_id,
                &digest,
                &binding_id,
                None,
                owner.revision,
                &plan,
            )
            .await?;
            tx.commit().await?;
            stage_and_activate(db, p, services, &input.operation_id, &reference, api_key).await
        }
        source => {
            let op = operator.ok_or(identity::IdentityError::Forbidden)?;
            let resource = legacy::resolve(&tx, op, &source).await?;
            legacy::check(&tx, &resource, services).await?;
            insert_binding(&tx, p, &binding_id, &plan, &resource, owner.revision, None).await?;
            record(
                &tx,
                p,
                &input.operation_id,
                &plan.operation,
                &digest,
                &binding_id,
            )
            .await?;
            let result =
                access::admin(&tx, &access::binding(&tx, p, &binding_id).await?, services).await?;
            tx.commit().await?;
            Ok(result)
        }
    }
}
pub(super) async fn update(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: UpdateBindingInput,
) -> Result<BindingAdmin> {
    uuid(&input.operation_id)?;
    revision(input.expected_revision)?;
    label(&input.label)?;
    access::validate_publication(&input.publication_domains, input.retained_task_text)?;
    let key_digest = input
        .credential
        .as_ref()
        .map(|CredentialReplacement::Fireflies { api_key }| {
            digest(&(&input.operation_id, &api_key.0))
        })
        .transpose()?;
    let digest = digest(
        &json!({"binding":input.binding_id,"revision":input.expected_revision,"label":input.label,"enabled":input.enabled,"domains":input.publication_domains,"retained":input.retained_task_text,"keyDigest":key_digest}),
    )?;
    cleanup(db, p, services).await?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    access::require_setup(&tx, p).await?;
    if let Some(result) = replay(
        &tx,
        p,
        services,
        &input.operation_id,
        "bindings/update",
        &digest,
    )
    .await?
    {
        return Ok(result);
    }
    let b = access::binding(&tx, p, &input.binding_id).await?;
    if b.revision != input.expected_revision {
        return Err(error::conflict());
    }
    let domain = vocabulary(&b.domain)?;
    let owner = access::owner(&tx, p, &b.source_owner_id, domain).await?;
    let resource: ResourceIdentity = parse(&b.resource_json)?;
    let plan = Plan {
        operation: "bindings/update".into(),
        label: input.label,
        domain,
        owner_id: b.source_owner_id.clone(),
        enabled: input.enabled,
        publication_domains: input.publication_domains,
        retained_task_text: input.retained_task_text,
        resource: Some(resource.clone()),
    };
    match input.credential {
        Some(CredentialReplacement::Fireflies { api_key }) => {
            if !matches!(resource, ResourceIdentity::Fireflies { mine: true, .. }) {
                return Err(error::invalid());
            }
            let reference = reserve(
                &tx,
                p,
                &input.operation_id,
                &digest,
                &b.id,
                Some(&b),
                owner.revision,
                &plan,
            )
            .await?;
            tx.commit().await?;
            stage_and_activate(db, p, services, &input.operation_id, &reference, api_key).await
        }
        None => {
            // Revalidation cannot retarget a changed legacy association.
            match resource {
                ResourceIdentity::Fireflies { mine: true, .. } => {
                    if input.enabled
                        && services
                            .get(
                                b.credential_ref
                                    .as_deref()
                                    .ok_or(Reason::CredentialUnavailable)?,
                            )
                            .await?
                            .is_none()
                    {
                        return Err(Reason::CredentialUnavailable.into());
                    }
                }
                ResourceIdentity::Fireflies { mine: false, .. } => {
                    return Err(Reason::BindingUnavailable.into())
                }
                _ => legacy::check(&tx, &resource, services).await?,
            }
            update_binding(
                &tx,
                p,
                &b,
                &plan,
                owner.revision,
                b.credential_ref.as_deref(),
            )
            .await?;
            record(&tx, p, &input.operation_id, &plan.operation, &digest, &b.id).await?;
            let result =
                access::admin(&tx, &access::binding(&tx, p, &b.id).await?, services).await?;
            tx.commit().await?;
            Ok(result)
        }
    }
}
pub(super) async fn disable(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    input: DisableBindingInput,
) -> Result<BindingAdmin> {
    uuid(&input.operation_id)?;
    revision(input.expected_revision)?;
    let digest = digest(&json!({"binding":input.binding_id,"revision":input.expected_revision}))?;
    let tx = identity::begin_write(db, p.organization_id()).await?;
    access::require_setup(&tx, p).await?;
    let b = access::binding(&tx, p, &input.binding_id).await?;
    if receipt(&tx, p, &input.operation_id, "bindings/disable", &digest)
        .await?
        .is_some()
    {
        return access::admin(&tx, &b, services).await;
    }
    if b.revision != input.expected_revision {
        return Err(error::conflict());
    }
    tx.execute(sql("UPDATE business_intake_binding SET enabled=0,revision=?,access_epoch=?,updated_at=? WHERE organization_id=? AND id=? AND revision=?",
        vec![next(b.revision)?.into(),next(b.access_epoch)?.into(),now().into(),p.organization_id().into(),b.id.clone().into(),b.revision.into()])).await?;
    audit(&tx, p, "binding_disabled", &b.id, next(b.revision)?).await?;
    record(
        &tx,
        p,
        &input.operation_id,
        "bindings/disable",
        &digest,
        &b.id,
    )
    .await?;
    let result = access::admin(&tx, &access::binding(&tx, p, &b.id).await?, services).await?;
    tx.commit().await?;
    Ok(result)
}

async fn stage_and_activate(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    operation_id: &str,
    reference: &str,
    key: Secret,
) -> Result<BindingAdmin> {
    let verified = async {
        services.set(reference, key.0).await?;
        let stored = services
            .get(reference)
            .await?
            .ok_or(Reason::CredentialUnavailable)?;
        services
            .reader
            .user(&stored)
            .await
            .map_err(|e| Error::from(e.reason))
    }
    .await;
    let user = match verified {
        Ok(user) => user,
        Err(error) => {
            retire(db, p, operation_id, reference).await;
            return Err(error);
        }
    };
    let result = activate(db, p, services, operation_id, reference, &user).await;
    if result.is_err() {
        // Never delete after an ambiguous commit. Resolve active references and
        // receipt under current setup authority on an identical retry/cleanup.
        retire(db, p, operation_id, reference).await;
    }
    result
}
async fn activate(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    operation_id: &str,
    reference: &str,
    user: &str,
) -> Result<BindingAdmin> {
    let tx = identity::begin_write(db, p.organization_id()).await?;
    access::require_setup(&tx, p).await?;
    let a = attempt(&tx, p, operation_id)
        .await?
        .ok_or_else(error::missing)?;
    let plan: Plan = parse(&a.plan_json)?;
    if let Some(result) = receipt(&tx, p, operation_id, &plan.operation, &a.digest).await? {
        return access::admin(&tx, &access::binding(&tx, p, &result).await?, services).await;
    }
    if a.credential_ref != reference || a.state != "staged" || !unexpired(Some(&a.expires_at)) {
        return Err(Reason::RequestTimeout.into());
    }
    let owner = access::owner(&tx, p, &plan.owner_id, plan.domain).await?;
    if owner.revision != a.owner_authority_revision {
        return Err(Reason::BindingUnavailable.into());
    }
    let resource = ResourceIdentity::Fireflies {
        provider_user_id: user.into(),
        mine: true,
    };
    if let Some(revision) = a.base_revision {
        let b = access::binding(&tx, p, &a.binding_id).await?;
        if b.revision != revision || Some(b.access_epoch) != a.base_epoch {
            return Err(error::conflict());
        }
        if plan.resource.as_ref() != Some(&resource)
            || parse::<ResourceIdentity>(&b.resource_json)? != resource
        {
            return Err(Reason::BindingUnavailable.into());
        }
        update_binding(&tx, p, &b, &plan, owner.revision, Some(reference)).await?;
        if let Some(old) = b.credential_ref {
            tx.execute(sql("UPDATE business_intake_setup SET state='retired' WHERE organization_id=? AND credential_ref=? AND NOT EXISTS(SELECT 1 FROM business_intake_binding WHERE credential_ref=?)",
                vec![p.organization_id().into(),old.clone().into(),old.into()])).await?;
        }
    } else {
        insert_binding(
            &tx,
            p,
            &a.binding_id,
            &plan,
            &resource,
            owner.revision,
            Some(reference),
        )
        .await?;
    }
    tx.execute(sql("UPDATE business_intake_setup SET state='active' WHERE organization_id=? AND id=? AND state='staged'",vec![p.organization_id().into(),a.id.into()])).await?;
    record(
        &tx,
        p,
        operation_id,
        &plan.operation,
        &a.digest,
        &a.binding_id,
    )
    .await?;
    let result = access::admin(
        &tx,
        &access::binding(&tx, p, &a.binding_id).await?,
        services,
    )
    .await?;
    match tx.commit().await {
        Ok(()) => Ok(result),
        Err(error) => {
            // A fresh writer may prove committed activation; inability to prove
            // it returns a safe error and leaves the protected staged ledger.
            if let Ok(tx) = identity::begin_write(db, p.organization_id()).await {
                access::require_setup(&tx, p).await?;
                if let Some(id) = receipt(&tx, p, operation_id, &plan.operation, &a.digest).await? {
                    let b = access::binding(&tx, p, &id).await?;
                    if b.credential_ref.as_deref() == Some(reference) {
                        return access::admin(&tx, &b, services).await;
                    }
                }
            }
            Err(error.into())
        }
    }
}
async fn retire(db: &DatabaseConnection, p: &Principal, operation_id: &str, reference: &str) {
    let Ok(tx) = identity::begin_write(db, p.organization_id()).await else {
        return;
    };
    if access::require_setup(&tx, p).await.is_err() {
        return;
    }
    if tx.execute(sql("UPDATE business_intake_setup SET state='retired' WHERE organization_id=? AND actor_id=? AND operation_id=? AND credential_ref=? AND state='staged' AND NOT EXISTS(SELECT 1 FROM business_intake_binding WHERE credential_ref=?)",
        vec![p.organization_id().into(),p.member_id().into(),operation_id.into(),reference.into(),reference.into()])).await.is_ok() {let _=tx.commit().await;}
}
/// Bounded round-robin retry of only ledger-owned, proven unreferenced keys.
/// A late blocking store may recreate an orphan; its retained row is retried later.
async fn cleanup(db: &DatabaseConnection, p: &Principal, services: &Services) -> Result<()> {
    let tx = identity::begin_write(db, p.organization_id()).await?;
    access::require_setup(&tx, p).await?;
    let rows=records::Setup::find_by_statement(sql("SELECT s.* FROM business_intake_setup s WHERE s.organization_id=? AND (s.state='retired' OR (s.state='staged' AND s.expires_at<=?)) AND NOT EXISTS(SELECT 1 FROM business_intake_binding b WHERE b.credential_ref=s.credential_ref) ORDER BY COALESCE(s.cleanup_at,s.created_at),s.id LIMIT 4",vec![p.organization_id().into(),now().into()])).all(&tx).await?;
    for row in &rows {
        tx.execute(sql("UPDATE business_intake_setup SET state='retired',cleanup_at=? WHERE organization_id=? AND id=?",vec![now().into(),p.organization_id().into(),row.id.clone().into()])).await?;
    }
    tx.commit().await?;
    for row in rows {
        let _ = services.delete(&row.credential_ref).await;
    }
    Ok(())
}
