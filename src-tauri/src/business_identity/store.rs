//! SeaORM/SQLite glue over the current principal. All identity writes take the
//! same writer lock used by business_tasks before reading authorization.
use super::{
    authorize, begin_write, operator_principal, types::*, Authority, IdentityError as E, Principal,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rngs::OsRng, RngCore};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DbBackend, FromQueryResult, Statement, TransactionTrait,
    Value,
};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};

pub(crate) fn statement(sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values)
}
pub(super) fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}
pub(super) fn id() -> String {
    uuid::Uuid::new_v4().to_string()
}
pub(super) fn parse<T: DeserializeOwned>(text: &str) -> Result<T, E> {
    serde_json::from_str(text).map_err(|_| E::Invalid("Invalid stored business identity"))
}
pub(super) fn enum_text<T: Serialize>(value: T) -> Result<String, E> {
    serde_json::to_value(value)
        .map_err(|_| E::Invalid("Invalid identity value"))?
        .as_str()
        .map(str::to_owned)
        .ok_or(E::Invalid("Invalid identity value"))
}
pub(super) fn name(value: &str) -> Result<String, E> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.chars().count() > 120 || trimmed.chars().any(char::is_control)
    {
        return Err(E::Invalid(
            "Use between 1 and 120 characters without control characters",
        ));
    }
    Ok(trimmed.to_owned())
}
pub(super) fn domains_json(domains: &[Domain]) -> Result<String, E> {
    if domains.is_empty()
        || domains.len() > Domain::ALL.len()
        || domains
            .iter()
            .enumerate()
            .any(|(i, d)| domains[..i].contains(d))
    {
        return Err(E::Invalid("Select distinct permitted domains"));
    }
    serde_json::to_string(domains).map_err(|_| E::Invalid("Invalid domains"))
}

#[derive(FromQueryResult)]
struct MemberRow {
    id: String,
    organization_id: String,
    display_name: String,
    kind: String,
    role: String,
    domains_json: String,
    status: String,
    revision: i64,
    operator_owner: bool,
    created_at: String,
    updated_at: String,
}
impl MemberRow {
    fn into_member(self) -> Result<Member, E> {
        Ok(Member {
            id: self.id,
            organization_id: self.organization_id,
            display_name: self.display_name,
            kind: parse(&format!("\"{}\"", self.kind))?,
            role: parse(&format!("\"{}\"", self.role))?,
            domains: parse(&self.domains_json)?,
            status: parse(&format!("\"{}\"", self.status))?,
            revision: self.revision,
            operator_owner: self.operator_owner,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}
pub(crate) async fn member<C: ConnectionTrait>(
    conn: &C,
    org: &str,
    member_id: &str,
) -> Result<Option<Member>, E> {
    MemberRow::find_by_statement(statement(
        "SELECT * FROM business_member WHERE organization_id = ? AND id = ?",
        vec![org.into(), member_id.into()],
    ))
    .one(conn)
    .await?
    .map(MemberRow::into_member)
    .transpose()
}
pub(crate) async fn organization<C: ConnectionTrait>(conn: &C) -> Result<Option<Organization>, E> {
    let row = conn
        .query_one(statement(
            "SELECT original_organization_id FROM business_tenancy_metadata WHERE id = 1",
            vec![],
        ))
        .await?
        .ok_or(E::BootstrapRequired)?;
    match row.try_get::<Option<String>>("", "original_organization_id")? {
        Some(id) => organization_by_id(conn, &id).await,
        None => Ok(None),
    }
}
pub(crate) async fn organization_by_id<C: ConnectionTrait>(
    conn: &C,
    org: &str,
) -> Result<Option<Organization>, E> {
    conn.query_one(statement("SELECT id, name, status, revision, authorization_epoch FROM business_organization WHERE id = ?", vec![org.into()])).await?
    .map(|row| {
        Ok(Organization {
            id: row.try_get("", "id")?,
            name: row.try_get("", "name")?,
            status: parse(&format!("\"{}\"", row.try_get::<String>("", "status")?))?,
            revision: row.try_get("", "revision")?,
            authorization_epoch: row.try_get("", "authorization_epoch")?,
        })
    })
    .transpose()
}
pub(crate) async fn operator_owner<C: ConnectionTrait>(conn: &C, org: &str) -> Result<Member, E> {
    MemberRow::find_by_statement(statement(
        "SELECT * FROM business_member WHERE organization_id = ? AND operator_owner = 1 AND status = 'active'",
        vec![org.into()],
    )).one(conn).await?.ok_or(E::Unauthorized)?.into_member()
}
pub(crate) async fn credential_active<C: ConnectionTrait>(
    conn: &C,
    org: &str,
    member_id: &str,
    credential_id: &str,
) -> Result<bool, E> {
    Ok(conn.query_one(statement(
        "SELECT id FROM business_credential WHERE organization_id = ? AND member_id = ? AND id = ? AND revoked_at IS NULL",
        vec![org.into(), member_id.into(), credential_id.into()],
    )).await?.is_some())
}
pub(crate) async fn resolve_credential<C: ConnectionTrait>(
    conn: &C,
    token: &str,
) -> Result<Principal, E> {
    let bytes = token
        .strip_prefix("bdm_")
        .filter(|s| s.len() == 43)
        .and_then(|s| URL_SAFE_NO_PAD.decode(s).ok())
        .filter(|s| s.len() == 32)
        .ok_or(E::Unauthorized)?;
    if format!("bdm_{}", URL_SAFE_NO_PAD.encode(bytes)) != token {
        return Err(E::Unauthorized);
    }
    let digest = format!("{:x}", Sha256::digest(token.as_bytes()));
    let row = conn.query_one(statement(
        "SELECT c.id, c.organization_id, c.member_id, o.authorization_epoch FROM business_credential c JOIN business_organization o ON o.id=c.organization_id WHERE c.token_hash = ? AND c.revoked_at IS NULL AND o.status='active'",
        vec![digest.into()],
    )).await?.ok_or(E::Unauthorized)?;
    let principal = Principal {
        organization_id: row.try_get("", "organization_id")?,
        member_id: row.try_get("", "member_id")?,
        authority: Authority::Credential(row.try_get("", "id")?),
        authorization_epoch: row.try_get("", "authorization_epoch")?,
        native_session_active: None,
    };
    super::current_human(conn, &principal).await?;
    Ok(principal)
}

pub(super) async fn event<C: ConnectionTrait>(
    conn: &C,
    org: &str,
    actor: &str,
    action: &str,
    subject: &str,
) -> Result<(), E> {
    conn.execute(statement(
        "INSERT INTO business_identity_event (id, organization_id, actor_id, action, subject_id, created_at) VALUES (?, ?, ?, ?, ?, ?)",
        vec![id().into(), org.into(), actor.into(), action.into(), subject.into(), now().into()],
    )).await?;
    Ok(())
}

/// Called only from verified original operator HTTP or local native boundary.
pub(crate) async fn bootstrap(
    conn: &DatabaseConnection,
    input: BootstrapInput,
) -> Result<Context, E> {
    let organization_name = name(&input.organization_name)?;
    let owner_name = name(&input.owner_name)?;
    let tx = conn.begin().await?;
    let organization_id = id();
    let timestamp = now();
    // This first write serializes competing bootstrap calls; no first-reader
    // claim race and no rename/replacement when already initialized.
    let created = tx.execute(statement(
        "INSERT INTO business_organization (id, singleton, name, created_at) VALUES (?, 1, ?, ?) ON CONFLICT(singleton) DO NOTHING",
        vec![organization_id.clone().into(), organization_name.into(), timestamp.clone().into()],
    )).await?;
    if created.rows_affected() == 1 {
        tx.execute(statement("UPDATE business_tenancy_metadata SET original_organization_id = ? WHERE id=1 AND original_organization_id IS NULL", vec![organization_id.clone().into()])).await?;
        super::settings::initialize(&tx, &organization_id, &name(&input.organization_name)?)
            .await?;
        let owner_id = id();
        tx.execute(statement(
            "INSERT INTO business_member (id, organization_id, display_name, kind, role, domains_json, operator_owner, created_at, updated_at) VALUES (?, ?, ?, 'human', 'owner', ?, 1, ?, ?)",
            vec![owner_id.clone().into(), organization_id.clone().into(), owner_name.into(), domains_json(&Domain::ALL)?.into(), timestamp.clone().into(), timestamp.into()],
        )).await?;
        event(
            &tx,
            &organization_id,
            &owner_id,
            "organization_bootstrapped",
            &organization_id,
        )
        .await?;
    }
    let principal = operator_principal(&tx).await?;
    let result = context(&tx, &principal).await?;
    tx.commit().await?;
    Ok(result)
}
pub async fn context<C: ConnectionTrait>(conn: &C, principal: &Principal) -> Result<Context, E> {
    let member = authorize(
        conn,
        principal,
        principal.organization_id(),
        Permission::Read,
        None,
    )
    .await?;
    Ok(Context {
        needs_bootstrap: false,
        organization: organization_by_id(conn, principal.organization_id()).await?,
        capabilities: Capabilities {
            manage_members: member.allows(Permission::ManageMembers, None),
            manage_tenant_settings: member.allows(Permission::ManageTenantSettings, None),
            legacy_operator: principal.is_operator(),
        },
        member: Some(member),
        operator: principal.is_operator(),
    })
}
pub(crate) async fn operator_context(conn: &DatabaseConnection) -> Result<Context, E> {
    let tx = conn.begin().await?;
    match operator_principal(&tx).await {
        Ok(p) => context(&tx, &p).await,
        Err(E::BootstrapRequired) => Ok(Context {
            needs_bootstrap: true,
            organization: None,
            member: None,
            operator: true,
            capabilities: Capabilities {
                manage_members: true,
                manage_tenant_settings: true,
                legacy_operator: true,
            },
        }),
        Err(e) => Err(e),
    }
}

pub async fn list_members(
    conn: &DatabaseConnection,
    principal: &Principal,
    input: MembersInput,
) -> Result<Vec<Member>, E> {
    let tx = conn.begin().await?;
    let actor = authorize(
        &tx,
        principal,
        &input.organization_id,
        Permission::Read,
        input.domain,
    )
    .await?;
    if actor.kind != MemberKind::Human {
        return Err(E::Forbidden);
    }
    let rows = MemberRow::find_by_statement(statement(
        "SELECT * FROM business_member WHERE organization_id = ? ORDER BY display_name, id LIMIT 501",
        vec![input.organization_id.into()],
    )).all(&tx).await?;
    if rows.len() > 500 {
        return Err(E::Invalid("Organization directory limit exceeded"));
    }
    let mut result = Vec::new();
    for row in rows {
        let member = row.into_member()?;
        if (member.status == MemberStatus::Active || actor.allows(Permission::ManageMembers, None))
            && member.domains.iter().any(|d| {
                actor.domains.contains(d) && input.domain.is_none_or(|selected| selected == *d)
            })
        {
            result.push(member);
        }
    }
    Ok(result)
}

fn grant_ceiling(
    actor: &Member,
    kind: MemberKind,
    role: Role,
    domains: &[Domain],
) -> Result<(), E> {
    if kind == MemberKind::Agent && role != Role::Member {
        return Err(E::Invalid("Agent identities use the member role"));
    }
    if actor.role != Role::Owner
        && (matches!(role, Role::Owner | Role::Admin)
            || domains.iter().any(|d| !actor.domains.contains(d)))
    {
        return Err(E::Forbidden);
    }
    Ok(())
}
fn manage_target(actor: &Member, target: &Member) -> Result<(), E> {
    if actor.role != Role::Owner && target.domains.iter().any(|d| !actor.domains.contains(d)) {
        return Err(E::NotFound);
    }
    if actor.role != Role::Owner
        && (matches!(target.role, Role::Owner | Role::Admin)
            || target.domains.iter().any(|d| !actor.domains.contains(d)))
    {
        return Err(E::Forbidden);
    }
    Ok(())
}

pub async fn create_member(
    conn: &DatabaseConnection,
    principal: &Principal,
    input: CreateMemberInput,
) -> Result<Member, E> {
    let display_name = name(&input.display_name)?;
    let domains = domains_json(&input.domains)?;
    let tx = begin_write(conn, &input.organization_id).await?;
    let actor = authorize(
        &tx,
        principal,
        &input.organization_id,
        Permission::ManageMembers,
        None,
    )
    .await?;
    grant_ceiling(&actor, input.kind, input.role, &input.domains)?;
    let member_id = id();
    let timestamp = now();
    tx.execute(statement(
        "INSERT INTO business_member (id, organization_id, display_name, kind, role, domains_json, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        vec![member_id.clone().into(), input.organization_id.clone().into(), display_name.into(), enum_text(input.kind)?.into(), enum_text(input.role)?.into(), domains.into(), timestamp.clone().into(), timestamp.into()],
    )).await?;
    event(
        &tx,
        &input.organization_id,
        &actor.id,
        "member_created",
        &member_id,
    )
    .await?;
    let result = member(&tx, &input.organization_id, &member_id)
        .await?
        .ok_or(E::NotFound)?;
    tx.commit().await?;
    Ok(result)
}

pub async fn update_member(
    conn: &DatabaseConnection,
    principal: &Principal,
    input: UpdateMemberInput,
) -> Result<Member, E> {
    if input.expected_revision <= 0 {
        return Err(E::Invalid("A positive expectedRevision is required"));
    }
    let display_name = name(&input.display_name)?;
    let domains = domains_json(&input.domains)?;
    let tx = begin_write(conn, &input.organization_id).await?;
    let actor = authorize(
        &tx,
        principal,
        &input.organization_id,
        Permission::ManageMembers,
        None,
    )
    .await?;
    let target = member(&tx, &input.organization_id, &input.member_id)
        .await?
        .ok_or(E::NotFound)?;
    manage_target(&actor, &target)?;
    if target.operator_owner || target.status != MemberStatus::Active {
        return Err(E::Forbidden);
    }
    if target.role == Role::Owner && input.role != Role::Owner {
        retain_owner(&tx, &target).await?;
    }
    grant_ceiling(&actor, target.kind, input.role, &input.domains)?;
    let changed = tx.execute(statement(
        "UPDATE business_member SET display_name = ?, role = ?, domains_json = ?, revision = revision + 1, updated_at = ? WHERE organization_id = ? AND id = ? AND revision = ? AND status = 'active'",
        vec![display_name.into(), enum_text(input.role)?.into(), domains.into(), now().into(), input.organization_id.clone().into(), input.member_id.clone().into(), input.expected_revision.into()],
    )).await?;
    if changed.rows_affected() != 1 {
        return Err(E::Conflict);
    }
    event(
        &tx,
        &input.organization_id,
        &actor.id,
        "member_updated",
        &input.member_id,
    )
    .await?;
    let result = member(&tx, &input.organization_id, &input.member_id)
        .await?
        .ok_or(E::NotFound)?;
    tx.commit().await?;
    Ok(result)
}

pub async fn revoke_member(
    conn: &DatabaseConnection,
    principal: &Principal,
    input: RevokeMemberInput,
) -> Result<Member, E> {
    if input.expected_revision <= 0 {
        return Err(E::Invalid("A positive expectedRevision is required"));
    }
    let tx = begin_write(conn, &input.organization_id).await?;
    let actor = authorize(
        &tx,
        principal,
        &input.organization_id,
        Permission::ManageMembers,
        None,
    )
    .await?;
    let target = member(&tx, &input.organization_id, &input.member_id)
        .await?
        .ok_or(E::NotFound)?;
    manage_target(&actor, &target)?;
    if target.operator_owner {
        return Err(E::Forbidden);
    }
    if target.revision != input.expected_revision {
        return Err(E::Conflict);
    }
    if target.status == MemberStatus::Revoked {
        return Ok(target);
    }
    retain_owner(&tx, &target).await?;
    let timestamp = now();
    let changed = tx.execute(statement(
        "UPDATE business_member SET status = 'revoked', revision = revision + 1, updated_at = ? WHERE organization_id = ? AND id = ? AND revision = ?",
        vec![timestamp.clone().into(), input.organization_id.clone().into(), input.member_id.clone().into(), input.expected_revision.into()],
    )).await?;
    if changed.rows_affected() != 1 {
        return Err(E::Conflict);
    }
    tx.execute(statement(
        "UPDATE business_credential SET revoked_at = ? WHERE organization_id = ? AND member_id = ? AND revoked_at IS NULL",
        vec![timestamp.into(), input.organization_id.clone().into(), input.member_id.clone().into()],
    )).await?;
    event(
        &tx,
        &input.organization_id,
        &actor.id,
        "member_revoked",
        &input.member_id,
    )
    .await?;
    let result = member(&tx, &input.organization_id, &input.member_id)
        .await?
        .ok_or(E::NotFound)?;
    tx.commit().await?;
    Ok(result)
}

const CREDENTIAL_COLUMNS: &str = "id, organization_id, member_id, label, created_at, revoked_at";
pub(super) async fn credential<C: ConnectionTrait>(
    conn: &C,
    org: &str,
    credential_id: &str,
) -> Result<Credential, E> {
    Credential::find_by_statement(statement(
        &format!("SELECT {CREDENTIAL_COLUMNS} FROM business_credential WHERE organization_id = ? AND id = ?"),
        vec![org.into(), credential_id.into()],
    )).one(conn).await?.ok_or(E::NotFound)
}
pub async fn issue_credential(
    conn: &DatabaseConnection,
    principal: &Principal,
    input: IssueCredentialInput,
) -> Result<IssuedCredential, E> {
    let label = name(&input.label)?;
    let tx = begin_write(conn, &input.organization_id).await?;
    let actor = authorize(
        &tx,
        principal,
        &input.organization_id,
        Permission::ManageMembers,
        None,
    )
    .await?;
    let target = member(&tx, &input.organization_id, &input.member_id)
        .await?
        .ok_or(E::NotFound)?;
    manage_target(&actor, &target)?;
    if target.kind != MemberKind::Human || target.status != MemberStatus::Active {
        return Err(E::Forbidden);
    }
    let issued = mint_credential(&tx, &input.organization_id, &input.member_id, &label).await?;
    event(
        &tx,
        &input.organization_id,
        &actor.id,
        "credential_issued",
        &issued.credential.id,
    )
    .await?;
    tx.commit().await?;
    Ok(issued)
}

// Shared existing opaque credential primitive; caller owns authorization and
// the surrounding writer transaction. Platform provisioning never impersonates
// a tenant Principal to issue its initial credential.
pub(super) async fn mint_credential<C: ConnectionTrait>(
    conn: &C,
    org: &str,
    member_id: &str,
    label: &str,
) -> Result<IssuedCredential, E> {
    let mut bytes = [0u8; 32];
    OsRng.try_fill_bytes(&mut bytes).map_err(|_| E::Random)?;
    let token = format!("bdm_{}", URL_SAFE_NO_PAD.encode(bytes));
    let token_hash = format!("{:x}", Sha256::digest(token.as_bytes()));
    let credential_id = id();
    conn.execute(statement("INSERT INTO business_credential (id,organization_id,member_id,token_hash,label,created_at) VALUES (?,?,?,?,?,?)",vec![credential_id.clone().into(),org.into(),member_id.into(),token_hash.into(),label.into(),now().into()])).await?;
    Ok(IssuedCredential {
        credential: credential(conn, org, &credential_id).await?,
        token,
    })
}
async fn retain_owner<C: ConnectionTrait>(conn: &C, target: &Member) -> Result<(), E> {
    if target.kind == MemberKind::Human
        && target.role == Role::Owner
        && target.status == MemberStatus::Active
    {
        let other=conn.query_one(statement("SELECT id FROM business_member WHERE organization_id=? AND id!=? AND kind='human' AND role='owner' AND status='active' LIMIT 1",vec![target.organization_id.clone().into(),target.id.clone().into()])).await?;
        if other.is_none() {
            return Err(E::Forbidden);
        }
    }
    Ok(())
}
pub async fn list_credentials(
    conn: &DatabaseConnection,
    principal: &Principal,
    input: CredentialsInput,
) -> Result<Vec<Credential>, E> {
    let tx = conn.begin().await?;
    let actor = authorize(
        &tx,
        principal,
        &input.organization_id,
        Permission::Read,
        None,
    )
    .await?;
    if actor.kind != MemberKind::Human {
        return Err(E::Forbidden);
    }
    if input.member_id != actor.id {
        authorize(
            &tx,
            principal,
            &input.organization_id,
            Permission::ManageMembers,
            None,
        )
        .await?;
        let target = member(&tx, &input.organization_id, &input.member_id)
            .await?
            .ok_or(E::NotFound)?;
        manage_target(&actor, &target)?;
    }
    Ok(Credential::find_by_statement(statement(
        &format!("SELECT {CREDENTIAL_COLUMNS} FROM business_credential WHERE organization_id = ? AND member_id = ? ORDER BY created_at, id"),
        vec![input.organization_id.into(), input.member_id.into()],
    )).all(&tx).await?)
}
pub async fn revoke_credential(
    conn: &DatabaseConnection,
    principal: &Principal,
    input: RevokeCredentialInput,
) -> Result<Credential, E> {
    let tx = begin_write(conn, &input.organization_id).await?;
    let actor = authorize(
        &tx,
        principal,
        &input.organization_id,
        Permission::Read,
        None,
    )
    .await?;
    if actor.kind != MemberKind::Human {
        return Err(E::Forbidden);
    }
    if !actor.allows(Permission::ManageMembers, None)
        && tx.query_one(statement(
            "SELECT id FROM business_credential WHERE organization_id = ? AND id = ? AND member_id = ?",
            vec![input.organization_id.clone().into(), input.credential_id.clone().into(), actor.id.clone().into()],
        )).await?.is_none()
    {
        return Err(E::NotFound);
    }
    let existing = credential(&tx, &input.organization_id, &input.credential_id).await?;
    if existing.member_id != actor.id {
        authorize(
            &tx,
            principal,
            &input.organization_id,
            Permission::ManageMembers,
            None,
        )
        .await?;
        let target = member(&tx, &input.organization_id, &existing.member_id)
            .await?
            .ok_or(E::NotFound)?;
        manage_target(&actor, &target)?;
    }
    if existing.revoked_at.is_some() {
        return Ok(existing);
    }
    tx.execute(statement(
        "UPDATE business_credential SET revoked_at = ? WHERE organization_id = ? AND id = ? AND revoked_at IS NULL",
        vec![now().into(), input.organization_id.clone().into(), input.credential_id.clone().into()],
    )).await?;
    event(
        &tx,
        &input.organization_id,
        &actor.id,
        "credential_revoked",
        &input.credential_id,
    )
    .await?;
    let result = credential(&tx, &input.organization_id, &input.credential_id).await?;
    tx.commit().await?;
    Ok(result)
}
