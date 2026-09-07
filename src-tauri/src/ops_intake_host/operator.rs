//! Authenticated human-only local mutations. No source timestamps/actors in input.
use super::{runtime::HostRuntime, store::*, types::*};
use crate::{
    ops::Operator,
    ops_intake::{self, EvidenceAttachment, EvidenceField, EvidenceProvenance},
};
use sea_orm::{ConnectionTrait, DatabaseConnection, TransactionTrait};
use serde_json::json;

pub async fn status(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
) -> Result<Status, HostError> {
    let mut products = vec![];
    for row in db.query_all(sql("SELECT config_json FROM ops_intake_host_product WHERE account_id=? ORDER BY product_id LIMIT 32", vec![op.account_id().into()])).await? {
        let p: StoredProduct = decode(&row.try_get::<String>("", "config_json")?)?;
        products.push(Product { binding:p.binding, origin:p.origin,
            intake_credential_present:runtime.credential(&p.bearer_ref).is_ok(), app_key_present:runtime.credential(&p.key_ref).is_ok() });
    }
    let mut folders = vec![];
    for row in db.query_all(sql("SELECT id,name FROM folder WHERE parent_id IS NULL AND deleted_at IS NULL ORDER BY name LIMIT 200",vec![])).await? {
        folders.push(FolderChoice { id:row.try_get("","id")?,name:row.try_get("","name")? });
    }
    Ok(Status {
        products,
        folders,
        adapter_installed: super::process::python().is_file(),
        in_app_available: false,
    })
}

pub async fn configure(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    input: ConfigureInput,
) -> Result<Status, HostError> {
    let mut guard = runtime.guard(&input.binding.product_id).await?;
    input.binding.validate()?;
    if !identifier(&input.binding.product_id) || input.origin.len() > 2048 {
        return Err(HostError::InvalidInput);
    }
    let url = reqwest::Url::parse(&input.origin).map_err(|_| HostError::InvalidInput)?;
    let scheme_ok = url.scheme() == "https"
        || (cfg!(any(test, feature = "test-utils"))
            && url.scheme() == "http"
            && matches!(url.host_str(), Some("127.0.0.1" | "[::1]")));
    if !scheme_ok
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.path() != "/"
        || url.query().is_some()
        || url.fragment().is_some()
        || db
            .query_one(sql(
                "SELECT id FROM folder WHERE id=? AND parent_id IS NULL AND deleted_at IS NULL",
                vec![input.binding.folder_id.into()],
            ))
            .await?
            .is_none()
    {
        return Err(HostError::InvalidInput);
    }
    let previous = db
        .query_one(sql(
            "SELECT account_id,config_json FROM ops_intake_host_product WHERE product_id=?",
            vec![input.binding.product_id.clone().into()],
        ))
        .await?;
    if previous
        .as_ref()
        .is_some_and(|r| r.try_get::<i32>("", "account_id").ok() != Some(op.account_id()))
    {
        return Err(HostError::AccessDenied);
    }
    let old: Option<StoredProduct> = previous
        .as_ref()
        .map(|r| -> Result<_, HostError> { decode(&r.try_get::<String>("", "config_json")?) })
        .transpose()?;
    if input
        .intake_bearer
        .as_ref()
        .is_some_and(|s| s.is_empty() || s.len() > 8192 || s.chars().any(char::is_control))
    {
        return Err(HostError::InvalidInput);
    }
    if let Some(pem) = &input.app_private_key {
        if pem.len() > 16384 {
            return Err(HostError::InvalidInput);
        }
        crate::ops_intake::github::GithubAppClient::new(Some(
            crate::ops_intake::github::GithubAppConfig {
                app_id: input.binding.app_id.clone(),
                private_key_pem: pem.clone(),
            },
        ))?;
    }
    let mut p = StoredProduct {
        binding: input.binding,
        origin: input.origin.trim_end_matches('/').into(),
        bearer_ref: old.as_ref().and_then(|p| p.bearer_ref.clone()),
        key_ref: old.as_ref().and_then(|p| p.key_ref.clone()),
    };
    let mut created: Vec<String> = vec![];
    for (value, slot) in [
        (input.intake_bearer, &mut p.bearer_ref),
        (input.app_private_key, &mut p.key_ref),
    ] {
        if let Some(value) = value {
            let reference = format!("ops-intake-{}", uuid::Uuid::new_v4());
            if let Err(e) = runtime.secrets.set(&reference, &value) {
                for r in &created {
                    let _ = runtime.secrets.delete(r);
                }
                return Err(e);
            }
            *slot = Some(reference.clone());
            created.push(reference);
        }
    }
    let commit=async {
        let txn=db.begin().await?;
        let changed=txn.execute(sql("INSERT INTO ops_intake_host_product(product_id,account_id,config_json) VALUES(?,?,?) ON CONFLICT(product_id) DO UPDATE SET config_json=excluded.config_json WHERE account_id=excluded.account_id",vec![p.binding.product_id.clone().into(),op.account_id().into(),encode(&p)?.into()])).await?;
        if changed.rows_affected()!=1 { return Err(HostError::AccessDenied); }
        txn.execute(sql("UPDATE ops_intake_source SET fetched_at=0 WHERE product_id=?",vec![p.binding.product_id.clone().into()])).await?;
        txn.execute(sql("UPDATE ops_intake_host_snapshot SET verified_at=NULL WHERE product_id=?",vec![p.binding.product_id.clone().into()])).await?;
        txn.commit().await?;
        Ok::<_,HostError>(())
    }.await;
    if let Err(e) = commit {
        for r in created {
            let _ = runtime.secrets.delete(&r);
        }
        return Err(e);
    }
    // Until both stores match, every host action fails closed in enabled().
    ops_intake::configure_repository(db, &p.binding).await?;
    guard.match_config(&p)?;
    if let Some(old) = old {
        for r in [old.bearer_ref, old.key_ref].into_iter().flatten() {
            if p.bearer_ref.as_ref() != Some(&r) && p.key_ref.as_ref() != Some(&r) {
                let _ = runtime.secrets.delete(&r);
            }
        }
    }
    status(db, op, runtime).await
}

pub async fn list(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    input: ListInput,
) -> Result<Listing, HostError> {
    let mut guard = runtime.guard(&input.product_id).await?;
    let p = enabled(db, op.account_id(), &input.product_id).await?;
    let page: Page = guard
        .read(
            runtime,
            &p,
            "list",
            json!({"source":"testflight","cursor":input.cursor}),
        )
        .await?;
    if page.items.len() > 100 || page.source_health != "ok" {
        return Err(HostError::SourceUnavailable);
    }
    let mut records = vec![];
    for r in page.items {
        validate_record(&r, &input.product_id)?;
        listed(db, &r).await?;
        let source = SourceInput {
            product_id: input.product_id.clone(),
            ulid: r.source_ref.ulid.clone(),
        };
        records.push(snapshot(db, &source).await?);
    }
    Ok(Listing {
        records,
        next_cursor: page.next_cursor,
        scan_complete: page.scan_complete,
    })
}

pub async fn refresh(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    source: SourceInput,
) -> Result<Detail, HostError> {
    let mut guard = runtime.guard(&source.product_id).await?;
    let p = enabled(db, op.account_id(), &source.product_id).await?;
    let previous = snapshot(db, &source).await?;
    // A canceled request cannot leave an earlier successful read looking fresh.
    invalidate(db, &source, HostError::SourceUnavailable).await?;
    guard.match_config(&p)?;
    let read = tokio::time::timeout(std::time::Duration::from_secs(50), async {
        if guard.bridge.is_none() {
            let mut cursor = None;
            let mut found = false;
            for _ in 0..5 {
                let page: Page = guard
                    .read(
                        runtime,
                        &p,
                        "list",
                        json!({"source":"testflight","cursor":cursor}),
                    )
                    .await?;
                found = page.items.iter().any(|r| {
                    r.source_ref.ulid == source.ulid && r.source_ref.product_id == source.product_id
                });
                if found {
                    break;
                }
                cursor = page.next_cursor;
                if cursor.is_none() {
                    break;
                }
            }
            if !found {
                return Err(HostError::SourceUnavailable);
            }
        }
        let r: Record = guard
            .read(
                runtime,
                &p,
                "get",
                serde_json::to_value(&previous.record.source_ref)
                    .map_err(|_| HostError::InvalidInput)?,
            )
            .await?;
        validate_record(&r, &source.product_id)?;
        if r.source_ref.ulid != source.ulid {
            return Err(HostError::SourceUnavailable);
        }
        Ok(r)
    })
    .await
    .unwrap_or(Err(HostError::SourceUnavailable));
    let r = match read {
        Ok(r) => r,
        Err(e) => {
            invalidate(db, &source, e).await?;
            return Err(e);
        }
    };
    listed(db, &r).await?;
    let at = chrono::DateTime::parse_from_rfc3339(&r.fetched_at)
        .map_err(|_| HostError::SourceUnavailable)?
        .timestamp();
    ops_intake::record_source(db, &r.source_ref.source(), &r.source_revision, at).await?;
    db.execute(sql("UPDATE ops_intake_host_snapshot SET verified_at=?,error=NULL WHERE product_id=? AND ulid=? AND json_extract(record_json,'$.source_revision')=?",vec![at.into(),source.product_id.clone().into(),source.ulid.clone().into(),r.source_revision.into()])).await?;
    detail(db, op, source).await
}

pub async fn detail(
    db: &DatabaseConnection,
    op: &Operator,
    source: SourceInput,
) -> Result<Detail, HostError> {
    let p = enabled(db, op.account_id(), &source.product_id).await?;
    let snapshot = snapshot(db, &source).await?;
    let draft = ensure_draft(db, &source).await?;
    let mut tasks = vec![];
    for r in db.query_all(sql("SELECT id,title,run_seq FROM work_task WHERE folder_id=? AND deleted_at IS NULL AND status IN ('running','awaiting_input') ORDER BY id DESC LIMIT 50",vec![p.binding.folder_id.into()])).await? {
        tasks.push(TaskChoice {id:r.try_get("","id")?,title:r.try_get("","title")?,run_seq:r.try_get("","run_seq")?});
    }
    let proposals = super::review::proposals(db, op.account_id(), &source).await?;
    let receipt = ops_intake::filing_status(
        db,
        &snapshot.record.source_ref.source(),
        p.binding.repository_id,
    )
    .await?;
    let handoff_unknown=db.query_one(sql("SELECT proposal_id FROM ops_intake_host_handoff WHERE product_id=? AND ulid=? AND state='unknown' LIMIT 1",vec![source.product_id.clone().into(),source.ulid.clone().into()])).await?.is_some();
    let (fix_task_id, fix_task_conflict) =
        match super::fix_task::linked(db, op.account_id(), &source).await {
            Ok(id) => (id, false),
            Err(HostError::Conflict) => (None, true),
            Err(e) => return Err(e),
        };
    Ok(Detail {
        snapshot,
        draft,
        tasks,
        proposals,
        receipt,
        handoff_unknown,
        fix_task_id,
        fix_task_conflict,
    })
}

pub async fn save(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    input: SaveInput,
) -> Result<Draft, HostError> {
    let _guard = runtime.guard(&input.source.product_id).await?;
    enabled(db, op.account_id(), &input.source.product_id).await?;
    let mut draft = ensure_draft(db, &input.source).await?;
    if draft.revision != input.expected_revision {
        return Err(HostError::Conflict);
    }
    if !ops_intake::reviewable_text(&input.title, 800)
        || !ops_intake::reviewable_text(&input.summary, 8000)
        || input.labels.len() > 20
        || input
            .labels
            .iter()
            .any(|s| !ops_intake::reviewable_text(s, 50))
    {
        return Err(HostError::InvalidInput);
    }
    draft.title = input.title;
    draft.summary = input.summary;
    draft.labels = input.labels;
    draft.confirmed_severity = input.confirmed_severity;
    draft.prepared = None;
    cas(db, &mut draft, input.expected_revision).await?;
    Ok(draft)
}

pub async fn attach(
    db: &DatabaseConnection,
    op: &Operator,
    runtime: &HostRuntime,
    input: AttachInput,
) -> Result<Draft, HostError> {
    let _guard = runtime.guard(&input.source.product_id).await?;
    enabled(db, op.account_id(), &input.source.product_id).await?;
    let snapshot = snapshot(db, &input.source).await?;
    fresh(&snapshot)?;
    let mut draft = ensure_draft(db, &input.source).await?;
    if draft.revision != input.expected_revision {
        return Err(HostError::Conflict);
    }
    let provenance = match input.field {
        EvidenceField::Build if snapshot.record.build_number.as_deref() == Some(&input.value) => {
            EvidenceProvenance::AscBuild
        }
        EvidenceField::Build if snapshot.record.build_number.is_some() => {
            return Err(HostError::InvalidEvidence)
        }
        EvidenceField::Build | EvidenceField::Screen | EvidenceField::Reciter => {
            EvidenceProvenance::HumanReport
        }
        EvidenceField::Log if input.session_ulid.is_some() => EvidenceProvenance::SessionDiagnostic,
        EvidenceField::Log => EvidenceProvenance::LocalDiagnostic,
    };
    let proof = ops_intake::attach_evidence(
        db,
        EvidenceAttachment {
            source_ref: snapshot.record.source_ref.source(),
            source_revision: snapshot.record.source_revision,
            field: input.field,
            value: input.value,
            content: input.content.into_bytes(),
            captured_at: input.captured_at.clone(),
            session_ulid: input.session_ulid.clone(),
            provenance,
            expires_at: None,
        },
        op.actor(),
    )
    .await?;
    let old = draft.proofs.insert(
        input.field.as_str().into(),
        Proof {
            proof: proof.clone(),
            captured_at: input.captured_at,
            session_ulid: input.session_ulid,
        },
    );
    draft.prepared = None;
    if let Err(e) = cas(db, &mut draft, input.expected_revision).await {
        ops_intake::revoke_evidence(db, &proof.artifact_id).await?;
        return Err(e);
    }
    if let Some(old) = old {
        ops_intake::revoke_evidence(db, &old.proof.artifact_id).await?;
    }
    Ok(draft)
}
