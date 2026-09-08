//! Pure cached TestFlight projection. Does not refresh, mint proof or inspect reporters.
use super::{
    store,
    types::{HostError, SourceInput},
};
use crate::{
    ops::Operator,
    ops_intake::{SourceKind, SourceRef, SOURCE_MAX_AGE_SECONDS},
};
use sea_orm::ConnectionTrait;

pub(crate) struct PublicSnapshot {
    pub title: String,
    pub description: String,
    pub revision: String,
    pub valid_until: i64,
}
pub(crate) async fn resolve<C: ConnectionTrait>(
    db: &C,
    op: &Operator,
    product_id: &str,
) -> Result<i32, HostError> {
    credential(db, op.account_id(), product_id).await?;
    Ok(op.account_id())
}
pub(crate) async fn credential<C: ConnectionTrait>(
    db: &C,
    account_id: i32,
    product_id: &str,
) -> Result<String, HostError> {
    if !store::identifier(product_id) {
        return Err(HostError::InvalidInput);
    }
    store::enabled(db, account_id, product_id)
        .await?
        .bearer_ref
        .ok_or(HostError::NotConfigured)
}
pub(crate) async fn snapshot<C: ConnectionTrait>(
    db: &C,
    account_id: i32,
    product_id: &str,
    ulid: &str,
) -> Result<PublicSnapshot, HostError> {
    credential(db, account_id, product_id).await?;
    SourceRef {
        product_id: product_id.into(),
        source: SourceKind::Testflight,
        ulid: ulid.into(),
    }
    .validate()
    .map_err(|_| HostError::InvalidInput)?;
    let snapshot = store::snapshot(
        db,
        &SourceInput {
            product_id: product_id.into(),
            ulid: ulid.into(),
        },
    )
    .await?;
    store::validate_record(&snapshot.record, product_id)?;
    if snapshot.record.source_ref.ulid != ulid {
        return Err(HostError::SourceUnavailable);
    }
    store::fresh(&snapshot)?;
    let valid_until = snapshot
        .verified_at
        .and_then(|v| v.checked_add(SOURCE_MAX_AGE_SECONDS))
        .ok_or(HostError::StaleSource)?;
    Ok(PublicSnapshot {
        title: snapshot.record.title,
        description: snapshot.record.description,
        revision: snapshot.record.source_revision,
        valid_until,
    })
}
