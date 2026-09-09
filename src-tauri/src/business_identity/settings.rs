//! Closed presentation settings; never a source of resource permissions.
use super::{authorize, begin_write, store, IdentityError as E, Permission, Principal};
use sea_orm::{ConnectionTrait, DatabaseConnection, TransactionTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Palette {
    Neutral,
    Blue,
    Violet,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceLayout {
    Split,
    Stacked,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkArea {
    Tasks,
    Conversations,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Settings {
    pub display_name: String,
    pub palette: Palette,
    pub workspace_layout: WorkspaceLayout,
    pub default_work_area: WorkArea,
}
impl Settings {
    pub fn defaults(display_name: String) -> Self {
        Self {
            display_name,
            palette: Palette::Neutral,
            workspace_layout: WorkspaceLayout::Split,
            default_work_area: WorkArea::Tasks,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SettingsView {
    pub organization_id: String,
    pub revision: i64,
    pub settings: Settings,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateSettingsInput {
    pub expected_revision: i64,
    pub settings: Settings,
}
pub(super) async fn initialize<C: ConnectionTrait>(
    conn: &C,
    org: &str,
    name: &str,
) -> Result<(), E> {
    let json = serde_json::to_string(&Settings::defaults(name.into()))
        .map_err(|_| E::Invalid("Invalid settings"))?;
    conn.execute(store::statement(
        "INSERT INTO business_tenant_settings (organization_id,settings_json) VALUES (?,?)",
        vec![org.into(), json.into()],
    ))
    .await?;
    Ok(())
}
async fn view<C: ConnectionTrait>(conn: &C, org: &str) -> Result<SettingsView, E> {
    let row = conn
        .query_one(store::statement(
            "SELECT revision, settings_json FROM business_tenant_settings WHERE organization_id=?",
            vec![org.into()],
        ))
        .await?
        .ok_or(E::NotFound)?;
    Ok(SettingsView {
        organization_id: org.into(),
        revision: row.try_get("", "revision")?,
        settings: store::parse(&row.try_get::<String>("", "settings_json")?)?,
    })
}
pub async fn get(conn: &DatabaseConnection, principal: &Principal) -> Result<SettingsView, E> {
    let tx = conn.begin().await?;
    authorize(
        &tx,
        principal,
        principal.organization_id(),
        Permission::Read,
        None,
    )
    .await?;
    view(&tx, principal.organization_id()).await
}
pub async fn update(
    conn: &DatabaseConnection,
    principal: &Principal,
    mut input: UpdateSettingsInput,
) -> Result<SettingsView, E> {
    input.settings.display_name = store::name(&input.settings.display_name)?;
    if input.expected_revision <= 0 {
        return Err(E::Invalid("A positive expectedRevision is required"));
    }
    let tx = begin_write(conn, principal.organization_id()).await?;
    let actor = authorize(
        &tx,
        principal,
        principal.organization_id(),
        Permission::ManageTenantSettings,
        None,
    )
    .await?;
    let json =
        serde_json::to_string(&input.settings).map_err(|_| E::Invalid("Invalid settings"))?;
    let changed = tx.execute(store::statement("UPDATE business_tenant_settings SET revision=revision+1,settings_json=? WHERE organization_id=? AND revision=?",vec![json.into(),principal.organization_id().into(),input.expected_revision.into()])).await?;
    if changed.rows_affected() != 1 {
        return Err(E::Conflict);
    }
    store::event(
        &tx,
        principal.organization_id(),
        &actor.id,
        "settings_updated",
        principal.organization_id(),
    )
    .await?;
    let result = view(&tx, principal.organization_id()).await?;
    tx.commit().await?;
    Ok(result)
}
