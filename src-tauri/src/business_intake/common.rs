use super::{
    error::{self, Error, Reason},
    records,
};
use crate::business_identity::{IdentityError, Principal};
use sea_orm::{ConnectionTrait, DatabaseTransaction, DbBackend, FromQueryResult, Statement, Value};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};

pub(super) type Result<T> = std::result::Result<T, Error>;
pub(super) fn sql(sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values)
}
pub(super) fn id() -> String {
    uuid::Uuid::new_v4().to_string()
}
pub(super) fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}
pub(super) fn future(seconds: i64) -> String {
    (chrono::Utc::now() + chrono::Duration::seconds(seconds)).to_rfc3339()
}
pub(super) fn storage() -> Error {
    IdentityError::Database(sea_orm::DbErr::Custom("Invalid intake storage".into())).into()
}
pub(super) fn parse<T: DeserializeOwned>(s: &str) -> Result<T> {
    serde_json::from_str(s).map_err(|_| storage())
}
pub(super) fn json<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|_| storage())
}
pub(super) fn code<T: Serialize>(value: T) -> Result<String> {
    serde_json::to_value(value)
        .map_err(|_| storage())?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(storage)
}
pub(super) fn vocabulary<T: DeserializeOwned>(value: &str) -> Result<T> {
    serde_json::from_value(serde_json::Value::String(value.to_owned())).map_err(|_| storage())
}
pub(super) fn digest<T: Serialize>(value: &T) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(json(value)?.as_bytes())))
}
pub(super) fn uuid(value: &str) -> Result<()> {
    if uuid::Uuid::parse_str(value).is_ok_and(|id| id.to_string() == value) {
        Ok(())
    } else {
        Err(error::invalid())
    }
}
pub(super) fn revision(value: i64) -> Result<()> {
    if value > 0 {
        Ok(())
    } else {
        Err(error::invalid())
    }
}
pub(super) fn next(value: i64) -> Result<i64> {
    value.checked_add(1).ok_or_else(error::conflict)
}
pub(super) fn page(value: u32) -> Result<i64> {
    if value <= 100_000 {
        Ok(i64::from(value) * 50)
    } else {
        Err(error::invalid())
    }
}
pub(super) fn text(value: &str, limit: usize, required: bool) -> bool {
    (!required || !value.trim().is_empty())
        && value.chars().count() <= limit
        && !value
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
}
pub(super) fn instant(value: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    let value = chrono::DateTime::parse_from_rfc3339(value).map_err(|_| error::invalid())?;
    if value.offset().local_minus_utc() != 0 {
        return Err(error::invalid());
    }
    Ok(value.with_timezone(&chrono::Utc))
}
pub(super) fn unexpired(value: Option<&str>) -> bool {
    value.is_none_or(|value| instant(value).is_ok_and(|date| date > chrono::Utc::now()))
}
pub(super) async fn receipt(
    tx: &DatabaseTransaction,
    p: &Principal,
    operation_id: &str,
    operation: &str,
    digest: &str,
) -> Result<Option<String>> {
    uuid(operation_id)?;
    let existing = records::Operation::find_by_statement(sql("SELECT operation,digest,result_id FROM business_intake_operation WHERE organization_id=? AND actor_id=? AND operation_id=?",
        vec![p.organization_id().into(),p.member_id().into(),operation_id.into()])).one(tx).await?;
    match existing {
        Some(row) if row.operation == operation && row.digest == digest => Ok(Some(row.result_id)),
        Some(_) => Err(error::conflict()),
        None => Ok(None),
    }
}
pub(super) async fn record(
    tx: &DatabaseTransaction,
    p: &Principal,
    operation_id: &str,
    operation: &str,
    digest: &str,
    result_id: &str,
) -> Result<()> {
    tx.execute(sql("INSERT INTO business_intake_operation(organization_id,actor_id,operation_id,operation,digest,result_id,created_at) VALUES(?,?,?,?,?,?,?)",
        vec![p.organization_id().into(),p.member_id().into(),operation_id.into(),operation.into(),digest.into(),result_id.into(),now().into()])).await?;
    Ok(())
}
pub(super) async fn audit(
    tx: &DatabaseTransaction,
    p: &Principal,
    action: &str,
    resource_id: &str,
    revision: i64,
) -> Result<()> {
    tx.execute(sql("INSERT INTO business_intake_audit(id,organization_id,actor_id,action,resource_id,revision,created_at) VALUES(?,?,?,?,?,?,?)",
        vec![id().into(),p.organization_id().into(),p.member_id().into(),action.into(),resource_id.into(),revision.into(),now().into()])).await?;
    Ok(())
}
/// Applies to core work in both transports. Only isolated staged secret I/O may
/// outlive cancellation; no detached task can commit a database transaction.
pub(crate) async fn bounded<T>(work: impl std::future::Future<Output = Result<T>>) -> Result<T> {
    tokio::time::timeout(std::time::Duration::from_secs(15), work)
        .await
        .map_err(|_| Error::from(Reason::RequestTimeout))?
}
