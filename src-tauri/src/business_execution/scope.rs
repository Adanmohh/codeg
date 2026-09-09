//! Current original-operator and task scope, reused before every private access.
//! Principal is supplied by the existing transport; no authority constructor.
use super::{common::*, records, types::OperationReason as R};
use crate::{
    business_identity::{self as identity, Member, Permission, Principal},
    business_tasks,
};
use sea_orm::{ConnectionTrait, DatabaseTransaction, FromQueryResult};

pub(super) async fn operator<C: ConnectionTrait>(
    conn: &C,
    principal: &Principal,
) -> Result<Member> {
    if !principal.is_operator() {
        return Err(R::Forbidden.into());
    }
    Ok(identity::authorize(
        conn,
        principal,
        principal.organization_id(),
        Permission::Read,
        None,
    )
    .await?)
}
pub(super) fn authority(principal: &Principal) -> Result<String> {
    if !principal.is_operator() {
        return Err(R::Forbidden.into());
    }
    Ok(identity::delegation_grant(principal)?.to_storage()?)
}
pub(super) struct TaskScope {
    pub task: business_tasks::types::Task,
    pub epoch: i64,
}
pub(super) async fn task(
    tx: &DatabaseTransaction,
    principal: &Principal,
    task_id: &str,
    contribute: bool,
) -> Result<TaskScope> {
    operator(tx, principal).await?;
    let detail = business_tasks::store::get_in_transaction(
        tx,
        &business_tasks::ActorContext::authenticated(principal.clone()),
        task_id,
    )
    .await?;
    if contribute && (!detail.task.capabilities.submit || detail.task.archived_at.is_some()) {
        return Err(R::Forbidden.into());
    }
    let row = tx
        .query_one(statement(
            "SELECT epoch FROM business_execution_task_scope WHERE organization_id=? AND task_id=?",
            vec![principal.organization_id().into(), task_id.into()],
        ))
        .await?
        .ok_or(R::Missing)?;
    Ok(TaskScope {
        task: detail.task,
        epoch: row.try_get("", "epoch")?,
    })
}
pub(super) async fn profile<C: ConnectionTrait>(
    conn: &C,
    principal: &Principal,
    profile_id: &str,
) -> Result<records::Profile> {
    operator(conn, principal).await?;
    records::Profile::find_by_statement(statement(
        "SELECT * FROM business_execution_profile WHERE organization_id=? AND member_id=? AND id=?",
        vec![
            principal.organization_id().into(),
            principal.member_id().into(),
            profile_id.into(),
        ],
    ))
    .one(conn)
    .await?
    .ok_or_else(|| R::Missing.into())
}
pub(super) async fn session(
    tx: &DatabaseTransaction,
    principal: &Principal,
    session_id: &str,
) -> Result<records::Session> {
    operator(tx, principal).await?;
    let row = records::Session::find_by_statement(statement(
        "SELECT * FROM business_execution_session WHERE organization_id=? AND member_id=? AND id=?",
        vec![
            principal.organization_id().into(),
            principal.member_id().into(),
            session_id.into(),
        ],
    ))
    .one(tx)
    .await?
    .ok_or(R::Missing)?;
    validate_session(tx, principal, &row).await?;
    Ok(row)
}
pub(super) async fn validate_session(
    tx: &DatabaseTransaction,
    principal: &Principal,
    row: &records::Session,
) -> Result<()> {
    let current = task(tx, principal, &row.task_id, false).await?;
    let profile = profile(tx, principal, &row.profile_id).await?;
    if row.organization_id != principal.organization_id()
        || row.member_id != principal.member_id()
        || row.authorization_epoch != principal.authorization_epoch()
        || row.authority_json != authority(principal)?
        || row.task_scope_epoch != current.epoch
        || profile.revision != row.profile_revision
        || profile.retired_at.is_some()
        || matches!(row.status.as_str(), "revoked" | "closed")
    {
        return Err(R::AuthorityChanged.into());
    }
    Ok(())
}
