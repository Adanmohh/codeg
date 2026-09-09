//! Durable exact-input admission. A replay returns a receipt, never permission to
//! repeat an external effect. All caller paths hold the identity writer first.
use super::{common::*, records, scope, types::*};
use crate::business_identity::Principal;
use sea_orm::{ConnectionTrait, DatabaseTransaction, FromQueryResult};
use serde::Serialize;

impl records::Operation {
    pub(super) fn summary(&self) -> Result<OperationSummary> {
        Ok(OperationSummary {
            id: self.operation_id.clone(),
            status: decode(&encode(&self.status)?)?,
            reason: self
                .reason
                .as_ref()
                .map(|r| decode(&encode(r)?))
                .transpose()?,
        })
    }
}
pub(super) async fn find(
    tx: &DatabaseTransaction,
    principal: &Principal,
    kind: OperationKind,
    operation_id: &str,
) -> Result<Option<records::Operation>> {
    scope::operator(tx, principal).await?;
    let row = records::Operation::find_by_statement(statement(
        "SELECT * FROM business_execution_operation WHERE organization_id=? AND member_id=? AND kind=? AND operation_id=?",
        vec![principal.organization_id().into(), principal.member_id().into(), key(kind)?.into(), operation_id.into()]))
        .one(tx).await?;
    if let Some(row) = &row {
        if row.authority_json != scope::authority(principal)?
            || row.authorization_epoch != principal.authorization_epoch()
        {
            return Err(OperationReason::AuthorityChanged.into());
        }
        scope::task(tx, principal, &row.task_id, false).await?;
        if let Some(session_id) = &row.session_id {
            scope::session(tx, principal, session_id).await?;
        }
    }
    Ok(row)
}
pub(super) async fn replay<T: Serialize>(
    tx: &DatabaseTransaction,
    principal: &Principal,
    kind: OperationKind,
    operation_id: &str,
    input: &T,
) -> Result<Option<records::Operation>> {
    let row = find(tx, principal, kind, operation_id).await?;
    if let Some(row) = &row {
        let json = encode(input)?;
        if row.input_json != json || row.input_hash != digest(json.as_bytes()) {
            return Err(OperationReason::Conflict.into());
        }
    }
    Ok(row)
}
pub(super) struct Target<'a> {
    pub task_id: &'a str,
    pub session_id: Option<&'a str>,
    pub generation: Option<i64>,
}
pub(super) fn require_target(
    row: &records::Operation,
    target: &Target<'_>,
    resource_id: &str,
) -> Result<()> {
    if row.task_id != target.task_id
        || row.session_id.as_deref() != target.session_id
        || row.generation != target.generation
        || row.resource_id.as_deref() != Some(resource_id)
    {
        return Err(OperationReason::Conflict.into());
    }
    Ok(())
}
pub(super) struct Completion<'a, T> {
    pub target: Target<'a>,
    pub resource_id: &'a str,
    pub status: OperationStatus,
    pub reason: Option<OperationReason>,
    pub result: Option<&'a T>,
}
pub(super) async fn reserve<T: Serialize>(
    tx: &DatabaseTransaction,
    principal: &Principal,
    kind: OperationKind,
    operation_id: &str,
    input: &T,
    target: Target<'_>,
    resource_id: &str,
) -> Result<()> {
    scope::operator(tx, principal).await?;
    let json = encode(input)?;
    let stamp = now();
    tx.execute(statement("INSERT INTO business_execution_operation(operation_id,organization_id,member_id,kind,authority_json,authorization_epoch,input_hash,input_json,task_id,session_id,generation,status,resource_id,created_at,updated_at) VALUES(?,?,?,?,?,?,?,?,?,?,?,'pending',?,?,?)",
        vec![operation_id.into(), principal.organization_id().into(), principal.member_id().into(), key(kind)?.into(), scope::authority(principal)?.into(), principal.authorization_epoch().into(), digest(json.as_bytes()).into(), json.into(), target.task_id.into(), target.session_id.map(str::to_owned).into(), target.generation.into(), resource_id.into(), stamp.clone().into(), stamp.into()])).await?;
    Ok(())
}
pub(super) async fn complete<T: Serialize>(
    tx: &DatabaseTransaction,
    principal: &Principal,
    kind: OperationKind,
    operation_id: &str,
    completion: Completion<'_, T>,
) -> Result<()> {
    // Repeat the captured lineage/current resource check after any external work.
    let row = find(tx, principal, kind, operation_id)
        .await?
        .ok_or(OperationReason::Missing)?;
    require_target(&row, &completion.target, completion.resource_id)?;
    if !matches!(row.status.as_str(), "pending" | "uncertain") {
        return Err(OperationReason::Conflict.into());
    }
    let count = tx.execute(statement("UPDATE business_execution_operation SET status=?,reason=?,result_json=?,updated_at=? WHERE organization_id=? AND member_id=? AND kind=? AND operation_id=? AND status IN ('pending','uncertain')",
        vec![key(completion.status)?.into(), completion.reason.map(key).transpose()?.into(), completion.result.map(encode).transpose()?.into(), now().into(), principal.organization_id().into(), principal.member_id().into(), key(kind)?.into(), operation_id.into()])).await?.rows_affected();
    if count != 1 {
        return Err(OperationReason::Conflict.into());
    }
    Ok(())
}
