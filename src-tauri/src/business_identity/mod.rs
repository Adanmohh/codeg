//! Individual business credentials and transaction-compatible authorization.
//!
//! Token/hash/revoke pattern: owner-authorized IntroMail mcp/auth.py at
//! 0bd24dfe284b888aa9f602fa1fd00e337ea38874; exact mapping in NOTICE.
//! This module never adds member authentication to legacy operator routes.
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, TransactionTrait};

pub(crate) mod http;
pub mod store;
pub mod types;
pub use types::{Domain, Member, MemberKind, MemberStatus, Permission, Role};
#[cfg(test)]
mod tests;

#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Invalid or revoked business credential")]
    Unauthorized,
    #[error("You do not have permission for this operation")]
    Forbidden,
    #[error("Business resource not found")]
    NotFound,
    #[error("Initialize the organization with the original operator first")]
    BootstrapRequired,
    #[error("This record changed. Reload before trying again")]
    Conflict,
    #[error("{0}")]
    Invalid(&'static str),
    #[error("Business identity storage failed")]
    Database(#[from] sea_orm::DbErr),
    #[error("Could not issue a credential")]
    Random,
}
impl IdentityError {
    pub fn command_error(self) -> crate::app_error::AppCommandError {
        use crate::app_error::AppCommandError as E;
        match self {
            Self::Unauthorized => E::authentication_failed(self.to_string()),
            Self::Forbidden => E::permission_denied(self.to_string()),
            Self::NotFound => E::not_found(self.to_string()),
            Self::BootstrapRequired => E::configuration_missing(self.to_string())
                .with_i18n("business.bootstrapRequired", Default::default()),
            Self::Conflict => E::already_exists(self.to_string())
                .with_i18n("business.revisionConflict", Default::default()),
            Self::Invalid(_) => E::invalid_input(self.to_string()),
            Self::Database(_) | Self::Random => {
                E::database_error("Business identity operation failed")
            }
        }
    }
}

#[derive(Clone)]
enum Authority {
    Operator,
    Credential(String),
    Agent(Box<Principal>),
}
/// Transport/host-derived attribution. Roles are always reread from storage.
/// Not serializable/deserializable, and no public constructor or mutable fields.
#[derive(Clone)]
pub struct Principal {
    organization_id: String,
    member_id: String,
    authority: Authority,
}

/// Opaque lineage for trusted task-link storage; contains IDs, never a bearer.
/// It cannot be constructed or deserialized by callers. Restore only after
/// loading a backend-owned binding and checking its current task/run/resource.
#[derive(serde::Serialize)]
pub struct DelegationGrant {
    organization_id: String,
    delegator_id: String,
    credential_id: Option<String>,
    operator: bool,
}
impl DelegationGrant {
    pub fn to_storage(&self) -> Result<String, IdentityError> {
        serde_json::to_string(self)
            .map_err(|_| IdentityError::Invalid("Invalid delegation lineage"))
    }
}

#[allow(dead_code)] // Used by the separately owned task-link persistence.
pub(crate) fn delegation_grant(principal: &Principal) -> Result<DelegationGrant, IdentityError> {
    let credential_id = match &principal.authority {
        Authority::Credential(id) => Some(id.clone()),
        Authority::Operator => None,
        Authority::Agent(_) => return Err(IdentityError::Forbidden),
    };
    Ok(DelegationGrant {
        organization_id: principal.organization_id.clone(),
        delegator_id: principal.member_id.clone(),
        credential_id,
        operator: principal.is_operator(),
    })
}

#[allow(dead_code)] // Used only after the task module validates its stored link.
pub(crate) async fn agent_principal_from_binding<C: ConnectionTrait>(
    conn: &C,
    organization_id: &str,
    agent_member_id: &str,
    stored_grant: &str,
) -> Result<Principal, IdentityError> {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct StoredGrant {
        organization_id: String,
        delegator_id: String,
        credential_id: Option<String>,
        operator: bool,
    }
    let grant: StoredGrant =
        serde_json::from_str(stored_grant).map_err(|_| IdentityError::Unauthorized)?;
    if grant.organization_id != organization_id {
        return Err(IdentityError::NotFound);
    }
    let authority = match (grant.operator, grant.credential_id) {
        (true, None) => Authority::Operator,
        (false, Some(id)) => Authority::Credential(id),
        _ => return Err(IdentityError::Unauthorized),
    };
    let delegator = Principal {
        organization_id: grant.organization_id,
        member_id: grant.delegator_id,
        authority,
    };
    agent_principal(conn, &delegator, agent_member_id).await
}
impl Principal {
    pub fn organization_id(&self) -> &str {
        &self.organization_id
    }
    pub fn member_id(&self) -> &str {
        &self.member_id
    }
    pub fn is_operator(&self) -> bool {
        matches!(self.authority, Authority::Operator)
    }
}

/// Obtain SQLite writer ownership BEFORE authentication/resource reads.
/// Identity revocations and task mutations must commit against the same order.
pub async fn begin_write(
    conn: &DatabaseConnection,
    organization_id: &str,
) -> Result<DatabaseTransaction, IdentityError> {
    let tx = conn.begin().await?;
    let locked = tx
        .execute(store::statement(
            "UPDATE business_organization SET id = id WHERE id = ?",
            vec![organization_id.into()],
        ))
        .await?;
    if locked.rows_affected() != 1 {
        return Err(IdentityError::NotFound);
    }
    Ok(tx)
}

async fn current_human<C: ConnectionTrait>(
    conn: &C,
    principal: &Principal,
) -> Result<Member, IdentityError> {
    let member = store::member(conn, principal.organization_id(), principal.member_id())
        .await?
        .filter(|m| m.kind == MemberKind::Human && m.status == MemberStatus::Active)
        .ok_or(IdentityError::Unauthorized)?;
    match &principal.authority {
        Authority::Operator if member.operator_owner && member.role == Role::Owner => {}
        Authority::Credential(id) => {
            if !store::credential_active(
                conn,
                principal.organization_id(),
                principal.member_id(),
                id,
            )
            .await?
            {
                return Err(IdentityError::Unauthorized);
            }
        }
        _ => return Err(IdentityError::Unauthorized),
    }
    Ok(member)
}

pub async fn authorize<C: ConnectionTrait>(
    conn: &C,
    principal: &Principal,
    organization_id: &str,
    permission: Permission,
    domain: Option<Domain>,
) -> Result<Member, IdentityError> {
    if principal.organization_id() != organization_id {
        return Err(IdentityError::NotFound);
    }
    let member = match &principal.authority {
        Authority::Agent(delegator) => {
            if domain.is_none() {
                return Err(IdentityError::Forbidden);
            }
            let human = current_human(conn, delegator).await?;
            // The original delegating credential is checked above too; a cached
            // active member alone cannot keep a revoked delegation alive.
            if human.organization_id != organization_id || !human.allows(permission, domain) {
                return Err(IdentityError::Forbidden);
            }
            store::member(conn, organization_id, principal.member_id())
                .await?
                .filter(|m| m.kind == MemberKind::Agent && m.status == MemberStatus::Active)
                .ok_or(IdentityError::Unauthorized)?
        }
        _ => current_human(conn, principal).await?,
    };
    if !member.allows(permission, domain) {
        return Err(IdentityError::Forbidden);
    }
    Ok(member)
}

/// Reference lookup only. Caller authorization and all source/destination
/// resource checks must occur on the same transaction in the calling module.
pub async fn active_reference<C: ConnectionTrait>(
    conn: &C,
    organization_id: &str,
    member_id: &str,
    domain: Domain,
) -> Result<Member, IdentityError> {
    store::member(conn, organization_id, member_id)
        .await?
        .filter(|m| m.allows(Permission::Read, Some(domain)))
        .ok_or(IdentityError::NotFound)
}

/// Trusted task bridge only, after checking parent/task/run binding. No HTTP
/// constructor. Callers must still fence each read/contribution by live binding.
#[allow(dead_code)] // Consumed by the separately owned business_tasks module.
pub(crate) async fn agent_principal<C: ConnectionTrait>(
    conn: &C,
    delegator: &Principal,
    agent_member_id: &str,
) -> Result<Principal, IdentityError> {
    current_human(conn, delegator).await?;
    let agent = store::member(conn, delegator.organization_id(), agent_member_id)
        .await?
        .filter(|m| {
            m.kind == MemberKind::Agent
                && m.status == MemberStatus::Active
                && m.role == Role::Member
        })
        .ok_or(IdentityError::NotFound)?;
    Ok(Principal {
        organization_id: agent.organization_id,
        member_id: agent.id,
        authority: Authority::Agent(Box::new(delegator.clone())),
    })
}

/// Only verified original operator HTTP or native commands may call this.
pub(crate) async fn operator_principal<C: ConnectionTrait>(
    conn: &C,
) -> Result<Principal, IdentityError> {
    let org = store::organization(conn)
        .await?
        .ok_or(IdentityError::BootstrapRequired)?;
    let member = store::operator_owner(conn, &org.id).await?;
    Ok(Principal {
        organization_id: org.id,
        member_id: member.id,
        authority: Authority::Operator,
    })
}
