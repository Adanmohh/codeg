//! Account-scoped host snapshots and revision CAS; accepted proof store is authoritative.
use super::types::*;
use crate::ops_intake::{self, SourceKind};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, TransactionTrait, Value};

pub(super) fn sql(text: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Sqlite, text, values)
}
pub(super) fn encode<T: serde::Serialize>(value: &T) -> Result<String, HostError> {
    serde_json::to_string(value).map_err(|_| HostError::InvalidInput)
}
pub(super) fn decode<T: serde::de::DeserializeOwned>(value: &str) -> Result<T, HostError> {
    serde_json::from_str(value).map_err(|_| HostError::StorageUnavailable)
}
pub(super) fn source_key(source: &SourceInput) -> String {
    format!("{}:testflight:{}", source.product_id, source.ulid)
}
pub(super) fn identifier(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 100
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}
pub(super) async fn product<C: ConnectionTrait>(
    db: &C,
    account: i32,
    product: &str,
) -> Result<StoredProduct, HostError> {
    let row = db
        .query_one(sql(
            "SELECT config_json FROM ops_intake_host_product WHERE product_id=? AND account_id=?",
            vec![product.into(), account.into()],
        ))
        .await?
        .ok_or(HostError::AccessDenied)?;
    let p: StoredProduct = decode(&row.try_get::<String>("", "config_json")?)?;
    if db
        .query_one(sql(
            "SELECT id FROM folder WHERE id=? AND parent_id IS NULL AND deleted_at IS NULL",
            vec![p.binding.folder_id.into()],
        ))
        .await?
        .is_none()
    {
        return Err(HostError::AccessDenied);
    }
    Ok(p)
}
pub(super) async fn enabled<C: ConnectionTrait>(
    db: &C,
    account: i32,
    product_id: &str,
) -> Result<StoredProduct, HostError> {
    let p = product(db, account, product_id).await?;
    if !p.binding.enabled {
        return Err(HostError::AccessDenied);
    }
    let row = db
        .query_one(sql(
            "SELECT config_json FROM ops_intake_binding WHERE product_id=?",
            vec![product_id.into()],
        ))
        .await?
        .ok_or(HostError::NotConfigured)?;
    let bound: ops_intake::RepositoryBinding = decode(&row.try_get::<String>("", "config_json")?)?;
    if bound != p.binding {
        return Err(HostError::NotConfigured);
    }
    Ok(p)
}
pub(super) fn validate_record(record: &Record, product: &str) -> Result<(), HostError> {
    let r = &record.source_ref;
    if r.product_id != product
        || r.source != SourceKind::Testflight
        || !identifier(&r.product_id)
        || r.ulid.len() != 26
        || !r
            .ulid
            .bytes()
            .all(|c| b"0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(&c))
        || r.ulid.as_bytes()[0] > b'7'
        || record.schema_version != 1
        || !record.title_is_draft
        || record.feedback_type.is_some()
        || record.app_version.is_some()
        || record.triage.confirmed_severity.is_some()
        || record.triage.confirmed_tags.is_some()
        || record.source_revision.len() != 64
        || !record
            .source_revision
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
        || record.title.len() > 20000
        || record.description.len() > 80000
        || record.screenshots.len() > 50
        || chrono::DateTime::parse_from_rfc3339(&record.fetched_at).is_err()
    {
        return Err(HostError::SourceUnavailable);
    }
    Ok(())
}
pub(super) async fn snapshot<C: ConnectionTrait>(
    db: &C,
    source: &SourceInput,
) -> Result<Snapshot, HostError> {
    let row = db.query_one(sql("SELECT record_json,verified_at,error FROM ops_intake_host_snapshot WHERE product_id=? AND ulid=?", vec![source.product_id.clone().into(), source.ulid.clone().into()])).await?.ok_or(HostError::SourceUnavailable)?;
    Ok(Snapshot {
        record: decode(&row.try_get::<String>("", "record_json")?)?,
        verified_at: row.try_get("", "verified_at")?,
        error: row.try_get("", "error")?,
    })
}
pub(super) fn fresh(s: &Snapshot) -> Result<(), HostError> {
    if s.error.is_some()
        || !s.verified_at.is_some_and(|v| {
            v >= chrono::Utc::now().timestamp() - ops_intake::SOURCE_MAX_AGE_SECONDS
                && v <= chrono::Utc::now().timestamp() + 30
        })
    {
        return Err(HostError::StaleSource);
    }
    Ok(())
}
pub(super) async fn listed(db: &DatabaseConnection, r: &Record) -> Result<(), HostError> {
    validate_record(r, &r.source_ref.product_id)?;
    let source = SourceInput {
        product_id: r.source_ref.product_id.clone(),
        ulid: r.source_ref.ulid.clone(),
    };
    let txn = db.begin().await?;
    // Listed changes invalidate old proof freshness, but never grant freshness.
    txn.execute(sql("INSERT INTO ops_intake_host_snapshot(product_id,ulid,record_json) VALUES(?,?,?) ON CONFLICT(product_id,ulid) DO UPDATE SET record_json=excluded.record_json,verified_at=CASE WHEN json_extract(record_json,'$.source_revision')=json_extract(excluded.record_json,'$.source_revision') THEN verified_at ELSE NULL END", vec![source.product_id.clone().into(), source.ulid.clone().into(), encode(r)?.into()])).await?;
    txn.execute(sql(
        "UPDATE ops_intake_source SET fetched_at=0 WHERE source_key=? AND revision<>?",
        vec![source_key(&source).into(), r.source_revision.clone().into()],
    ))
    .await?;
    txn.commit().await?;
    Ok(())
}
pub(super) async fn invalidate(
    db: &DatabaseConnection,
    source: &SourceInput,
    error: HostError,
) -> Result<(), HostError> {
    let txn = db.begin().await?;
    txn.execute(sql(
        "UPDATE ops_intake_source SET fetched_at=0 WHERE source_key=?",
        vec![source_key(source).into()],
    ))
    .await?;
    txn.execute(sql("UPDATE ops_intake_host_snapshot SET verified_at=NULL,error=? WHERE product_id=? AND ulid=?", vec![error.to_string().into(), source.product_id.clone().into(), source.ulid.clone().into()])).await?;
    txn.commit().await?;
    Ok(())
}
pub(super) async fn invalidate_product(
    db: &DatabaseConnection,
    product: &str,
) -> Result<(), HostError> {
    let txn = db.begin().await?;
    txn.execute(sql(
        "UPDATE ops_intake_source SET fetched_at=0 WHERE product_id=?",
        vec![product.into()],
    ))
    .await?;
    txn.execute(sql(
        "UPDATE ops_intake_host_snapshot SET verified_at=NULL WHERE product_id=?",
        vec![product.into()],
    ))
    .await?;
    txn.commit().await?;
    Ok(())
}
pub(super) async fn draft<C: ConnectionTrait>(
    db: &C,
    source: &SourceInput,
) -> Result<Draft, HostError> {
    let row = db
        .query_one(sql(
            "SELECT draft_json FROM ops_intake_host_draft WHERE product_id=? AND ulid=?",
            vec![source.product_id.clone().into(), source.ulid.clone().into()],
        ))
        .await?
        .ok_or(HostError::Conflict)?;
    decode(&row.try_get::<String>("", "draft_json")?)
}
pub(super) async fn ensure_draft(
    db: &DatabaseConnection,
    source: &SourceInput,
) -> Result<Draft, HostError> {
    let snapshot = snapshot(db, source).await?;
    let r = &snapshot.record;
    let mut d = Draft {
        id: uuid::Uuid::new_v4().to_string(),
        revision: 1,
        source_revision: r.source_revision.clone(),
        title: r.title.clone(),
        summary: r.description.clone(),
        labels: vec![],
        confirmed_severity: None,
        proofs: Default::default(),
        prepared: None,
    };
    db.execute(sql("INSERT INTO ops_intake_host_draft(id,product_id,ulid,revision,draft_json) VALUES(?,?,?,?,?) ON CONFLICT(product_id,ulid) DO NOTHING", vec![d.id.clone().into(), source.product_id.clone().into(), source.ulid.clone().into(), 1.into(), encode(&d)?.into()])).await?;
    let previous = draft(db, source).await?;
    if previous.source_revision == r.source_revision {
        return Ok(previous);
    }
    d.id = previous.id;
    cas(db, &mut d, previous.revision).await?;
    Ok(d)
}
pub(super) async fn cas(
    db: &DatabaseConnection,
    d: &mut Draft,
    expected: i32,
) -> Result<(), HostError> {
    d.revision = expected.checked_add(1).ok_or(HostError::Conflict)?;
    let result = db
        .execute(sql(
            "UPDATE ops_intake_host_draft SET revision=?,draft_json=? WHERE id=? AND revision=?",
            vec![
                d.revision.into(),
                encode(d)?.into(),
                d.id.clone().into(),
                expected.into(),
            ],
        ))
        .await?;
    if result.rows_affected() != 1 {
        return Err(HostError::Conflict);
    }
    Ok(())
}
