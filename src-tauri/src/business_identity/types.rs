use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    Marketing,
    Channels,
    Ads,
    Website,
    Feedback,
    Engineering,
}
impl Domain {
    pub const ALL: [Self; 6] = [
        Self::Marketing,
        Self::Channels,
        Self::Ads,
        Self::Website,
        Self::Feedback,
        Self::Engineering,
    ];
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberKind {
    Human,
    Agent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    Manager,
    Member,
    Viewer,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberStatus {
    Active,
    Revoked,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Read,
    Create,
    Contribute,
    Assign,
    Review,
    ManageMembers,
    ManageTenantSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Member {
    pub id: String,
    pub organization_id: String,
    pub display_name: String,
    pub kind: MemberKind,
    pub role: Role,
    pub domains: Vec<Domain>,
    pub status: MemberStatus,
    pub revision: i64,
    pub operator_owner: bool,
    pub created_at: String,
    pub updated_at: String,
}
impl Member {
    /// A current reference's grants, never proof of caller authentication.
    pub fn allows(&self, permission: Permission, domain: Option<Domain>) -> bool {
        if self.status != MemberStatus::Active || domain.is_some_and(|d| !self.domains.contains(&d))
        {
            return false;
        }
        if self.kind == MemberKind::Agent {
            return self.role == Role::Member
                && matches!(permission, Permission::Read | Permission::Contribute);
        }
        match permission {
            Permission::Read => true,
            Permission::Create | Permission::Contribute => self.role != Role::Viewer,
            Permission::Assign | Permission::Review => {
                matches!(self.role, Role::Owner | Role::Admin | Role::Manager)
            }
            Permission::ManageMembers | Permission::ManageTenantSettings => {
                matches!(self.role, Role::Owner | Role::Admin)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub status: OrganizationStatus,
    pub revision: i64,
    pub authorization_epoch: i64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationStatus {
    Active,
    Suspended,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    pub manage_members: bool,
    pub manage_tenant_settings: bool,
    pub legacy_operator: bool,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub needs_bootstrap: bool,
    pub organization: Option<Organization>,
    pub member: Option<Member>,
    pub operator: bool,
    pub capabilities: Capabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize, sea_orm::FromQueryResult)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Credential {
    pub id: String,
    pub organization_id: String,
    pub member_id: String,
    pub label: String,
    pub created_at: String,
    pub revoked_at: Option<String>,
}
// Deliberately no Debug/Clone: the only response that includes plaintext.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuedCredential {
    pub credential: Credential,
    pub token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BootstrapInput {
    pub organization_name: String,
    pub owner_name: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MembersInput {
    pub organization_id: String,
    pub domain: Option<Domain>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateMemberInput {
    pub organization_id: String,
    pub display_name: String,
    pub kind: MemberKind,
    pub role: Role,
    pub domains: Vec<Domain>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateMemberInput {
    pub organization_id: String,
    pub member_id: String,
    pub expected_revision: i64,
    pub display_name: String,
    pub role: Role,
    pub domains: Vec<Domain>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevokeMemberInput {
    pub organization_id: String,
    pub member_id: String,
    pub expected_revision: i64,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IssueCredentialInput {
    pub organization_id: String,
    pub member_id: String,
    pub label: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CredentialsInput {
    pub organization_id: String,
    pub member_id: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevokeCredentialInput {
    pub organization_id: String,
    pub credential_id: String,
}
