//! Closed platform operations. No implicit tenant role or member impersonation.
use super::{
    settings,
    store::{self, statement},
    types::*,
    IdentityError as E,
};
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, TransactionTrait};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// The original authenticated transport marker is the only HTTP constructor.
/// No role strings, Deserialize, public fields or tenant selector authority.
pub(crate) struct PlatformContext {
    _private: (),
}
impl PlatformContext {
    pub(crate) fn from_operator(_: &crate::web::auth::AuthenticatedOperator) -> Self {
        Self { _private: () }
    }
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateTenantInput {
    pub operation_id: String,
    pub organization_name: String,
    pub owner_name: String,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatusInput {
    pub operation_id: String,
    pub organization_id: String,
    pub expected_revision: i64,
    pub expected_authorization_epoch: i64,
    pub status: OrganizationStatus,
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReissueInput {
    pub operation_id: String,
    pub organization_id: String,
    pub expected_revision: i64,
    pub expected_authorization_epoch: i64,
    pub owner_member_id: String,
    pub expected_owner_revision: i64,
    pub expected_credential_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProvisionMetadata {
    pub organization: Organization,
    pub owner_member_id: String,
    pub owner_revision: i64,
    pub credential: Credential,
}
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Delivery {
    IssuedOnce,
    AlreadyProvisioned,
}
// No Debug/Clone; secret only on the initial successful issuance.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisionResult {
    #[serde(flatten)]
    pub metadata: ProvisionMetadata,
    pub delivery: Delivery,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantSummary {
    pub organization: Organization,
    pub owner_member_id: Option<String>,
    pub owner_revision: Option<i64>,
    pub owner_status: Option<MemberStatus>,
    pub credential: Option<Credential>,
}
fn digest<T: Serialize>(operation_id: &str, action: &str, input: &T) -> Result<String, E> {
    if uuid::Uuid::parse_str(operation_id).is_err() {
        return Err(E::Invalid("Use a UUID operationId"));
    }
    let bytes =
        serde_json::to_vec(&(action, input)).map_err(|_| E::Invalid("Invalid platform request"))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
async fn writer(conn: &DatabaseConnection) -> Result<DatabaseTransaction, E> {
    let tx = conn.begin().await?;
    let changed = tx
        .execute(statement(
            "UPDATE business_tenancy_metadata SET id=id WHERE id=1",
            vec![],
        ))
        .await?;
    if changed.rows_affected() != 1 {
        return Err(E::BootstrapRequired);
    }
    Ok(tx)
}
async fn replay<T: DeserializeOwned>(
    tx: &DatabaseTransaction,
    id: &str,
    hash: &str,
) -> Result<Option<T>, E> {
    let Some(row) = tx
        .query_one(statement(
            "SELECT input_hash,result_json FROM business_platform_receipt WHERE operation_id=?",
            vec![id.into()],
        ))
        .await?
    else {
        return Ok(None);
    };
    if row.try_get::<String>("", "input_hash")? != hash {
        return Err(E::Conflict);
    }
    Ok(Some(store::parse(
        &row.try_get::<String>("", "result_json")?,
    )?))
}
async fn receipt<T: Serialize>(
    tx: &DatabaseTransaction,
    id: &str,
    hash: &str,
    action: &str,
    org: &str,
    result: &T,
) -> Result<(), E> {
    let json = serde_json::to_string(result).map_err(|_| E::Invalid("Invalid platform result"))?;
    tx.execute(statement("INSERT INTO business_platform_receipt (operation_id,input_hash,action,organization_id,result_json,created_at) VALUES (?,?,?,?,?,?)",vec![id.into(),hash.into(),action.into(),org.into(),json.into(),store::now().into()])).await?;
    Ok(())
}
pub(crate) async fn create(
    conn: &DatabaseConnection,
    _platform: &PlatformContext,
    mut input: CreateTenantInput,
) -> Result<ProvisionResult, E> {
    input.organization_name = store::name(&input.organization_name)?;
    input.owner_name = store::name(&input.owner_name)?;
    let hash = digest(&input.operation_id, "tenant_created", &input)?;
    let tx = writer(conn).await?;
    if let Some(metadata) = replay(&tx, &input.operation_id, &hash).await? {
        return Ok(ProvisionResult {
            metadata,
            delivery: Delivery::AlreadyProvisioned,
            token: None,
        });
    }
    let org = store::id();
    let owner = store::id();
    let now = store::now();
    tx.execute(statement(
        "INSERT INTO business_organization (id,name,created_at) VALUES (?,?,?)",
        vec![
            org.clone().into(),
            input.organization_name.clone().into(),
            now.clone().into(),
        ],
    ))
    .await?;
    tx.execute(statement("INSERT INTO business_member (id,organization_id,display_name,kind,role,domains_json,created_at,updated_at) VALUES (?,?,?,'human','owner',?,?,?)",vec![owner.clone().into(),org.clone().into(),input.owner_name.into(),store::domains_json(&Domain::ALL)?.into(),now.clone().into(),now.into()])).await?;
    settings::initialize(&tx, &org, &input.organization_name).await?;
    let issued = store::mint_credential(&tx, &org, &owner, "Initial tenant owner").await?;
    tx.execute(statement("INSERT INTO business_provisioning_owner (organization_id,member_id,credential_id) VALUES (?,?,?)",vec![org.clone().into(),owner.clone().into(),issued.credential.id.clone().into()])).await?;
    let metadata = ProvisionMetadata {
        organization: store::organization_by_id(&tx, &org)
            .await?
            .ok_or(E::NotFound)?,
        owner_member_id: owner,
        owner_revision: 1,
        credential: issued.credential,
    };
    receipt(
        &tx,
        &input.operation_id,
        &hash,
        "tenant_created",
        &org,
        &metadata,
    )
    .await?;
    tx.commit().await?;
    Ok(ProvisionResult {
        metadata,
        delivery: Delivery::IssuedOnce,
        token: Some(issued.token),
    })
}
pub(crate) async fn list(
    conn: &DatabaseConnection,
    _platform: &PlatformContext,
) -> Result<Vec<TenantSummary>, E> {
    let tx = conn.begin().await?;
    let rows = tx
        .query_all(statement(
            "SELECT id FROM business_organization ORDER BY created_at,id LIMIT 501",
            vec![],
        ))
        .await?;
    if rows.len() > 500 {
        return Err(E::Invalid("Tenant registry limit exceeded"));
    }
    let mut result = Vec::new();
    for row in rows {
        let id: String = row.try_get("", "id")?;
        let organization = store::organization_by_id(&tx, &id)
            .await?
            .ok_or(E::NotFound)?;
        let mut summary = TenantSummary {
            organization,
            owner_member_id: None,
            owner_revision: None,
            owner_status: None,
            credential: None,
        };
        if let Some(row)=tx.query_one(statement("SELECT member_id,credential_id FROM business_provisioning_owner WHERE organization_id=?",vec![id.clone().into()])).await? {
            let member_id:String=row.try_get("","member_id")?;
            let owner=store::member(&tx,&id,&member_id).await?.ok_or(E::NotFound)?;
            summary.owner_revision=Some(owner.revision);summary.owner_status=Some(owner.status);summary.owner_member_id=Some(member_id);
            summary.credential=Some(store::credential(&tx,&id,&row.try_get::<String>("","credential_id")?).await?);
        }
        result.push(summary);
    }
    Ok(result)
}
async fn current(
    tx: &DatabaseTransaction,
    org: &str,
    revision: i64,
    epoch: i64,
) -> Result<Organization, E> {
    let result = store::organization_by_id(tx, org)
        .await?
        .ok_or(E::NotFound)?;
    if result.revision != revision || result.authorization_epoch != epoch {
        return Err(E::Conflict);
    }
    Ok(result)
}
pub(crate) async fn status(
    conn: &DatabaseConnection,
    _platform: &PlatformContext,
    input: StatusInput,
) -> Result<Organization, E> {
    let hash = digest(&input.operation_id, "tenant_status_changed", &input)?;
    let tx = writer(conn).await?;
    if let Some(result) = replay(&tx, &input.operation_id, &hash).await? {
        return Ok(result);
    }
    let org = current(
        &tx,
        &input.organization_id,
        input.expected_revision,
        input.expected_authorization_epoch,
    )
    .await?;
    if org.status == input.status {
        return Err(E::Conflict);
    }
    tx.execute(statement("UPDATE business_organization SET status=?,revision=revision+1,authorization_epoch=authorization_epoch+1 WHERE id=? AND revision=? AND authorization_epoch=?",vec![store::enum_text(input.status)?.into(),org.id.clone().into(),org.revision.into(),org.authorization_epoch.into()])).await?;
    let result = store::organization_by_id(&tx, &org.id)
        .await?
        .ok_or(E::NotFound)?;
    receipt(
        &tx,
        &input.operation_id,
        &hash,
        "tenant_status_changed",
        &org.id,
        &result,
    )
    .await?;
    tx.commit().await?;
    Ok(result)
}
pub(crate) async fn reissue(
    conn: &DatabaseConnection,
    _platform: &PlatformContext,
    input: ReissueInput,
) -> Result<ProvisionResult, E> {
    let hash = digest(&input.operation_id, "owner_credential_reissued", &input)?;
    let tx = writer(conn).await?;
    if let Some(metadata) = replay(&tx, &input.operation_id, &hash).await? {
        return Ok(ProvisionResult {
            metadata,
            delivery: Delivery::AlreadyProvisioned,
            token: None,
        });
    }
    let org = current(
        &tx,
        &input.organization_id,
        input.expected_revision,
        input.expected_authorization_epoch,
    )
    .await?;
    if org.status != OrganizationStatus::Active {
        return Err(E::Forbidden);
    }
    let lineage=tx.query_one(statement("SELECT member_id,credential_id FROM business_provisioning_owner WHERE organization_id=?",vec![org.id.clone().into()])).await?.ok_or(E::NotFound)?;
    if lineage.try_get::<String>("", "member_id")? != input.owner_member_id
        || lineage.try_get::<String>("", "credential_id")? != input.expected_credential_id
    {
        return Err(E::Conflict);
    }
    let owner = store::member(&tx, &org.id, &input.owner_member_id)
        .await?
        .ok_or(E::NotFound)?;
    if owner.kind != MemberKind::Human
        || owner.role != Role::Owner
        || owner.status != MemberStatus::Active
    {
        return Err(E::Forbidden);
    }
    if owner.revision != input.expected_owner_revision {
        return Err(E::Conflict);
    }
    if !store::credential_active(&tx, &org.id, &owner.id, &input.expected_credential_id).await? {
        return Err(E::Conflict);
    }
    tx.execute(statement("UPDATE business_credential SET revoked_at=? WHERE organization_id=? AND id=? AND revoked_at IS NULL",vec![store::now().into(),org.id.clone().into(),input.expected_credential_id.into()])).await?;
    let issued = store::mint_credential(&tx, &org.id, &owner.id, "Recovered tenant owner").await?;
    tx.execute(statement(
        "UPDATE business_provisioning_owner SET credential_id=? WHERE organization_id=?",
        vec![issued.credential.id.clone().into(), org.id.clone().into()],
    ))
    .await?;
    let metadata = ProvisionMetadata {
        organization: org,
        owner_member_id: owner.id,
        owner_revision: owner.revision,
        credential: issued.credential,
    };
    receipt(
        &tx,
        &input.operation_id,
        &hash,
        "owner_credential_reissued",
        &metadata.organization.id,
        &metadata,
    )
    .await?;
    tx.commit().await?;
    Ok(ProvisionResult {
        metadata,
        delivery: Delivery::IssuedOnce,
        token: Some(issued.token),
    })
}
