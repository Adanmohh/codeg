//! Native owner boundary over the same business core. No caller identity input.
#![cfg(feature = "tauri-runtime")]
use crate::{
    app_error::AppCommandError,
    business_identity::{self, store, types::*, IdentityError},
    db::AppDatabase,
};

#[tauri::command]
pub async fn business_context(
    db: tauri::State<'_, AppDatabase>,
) -> Result<Context, AppCommandError> {
    store::operator_context(&db.conn)
        .await
        .map_err(IdentityError::command_error)
}
#[tauri::command]
pub async fn business_bootstrap(
    db: tauri::State<'_, AppDatabase>,
    input: BootstrapInput,
) -> Result<Context, AppCommandError> {
    store::bootstrap(&db.conn, input)
        .await
        .map_err(IdentityError::command_error)
}
macro_rules! input_command {
    ($name:ident, $core:path, $input:ty, $result:ty) => {
        #[tauri::command]
        pub async fn $name(
            db: tauri::State<'_, AppDatabase>,
            input: $input,
        ) -> Result<$result, AppCommandError> {
            let principal = business_identity::operator_principal(&db.conn)
                .await
                .map_err(IdentityError::command_error)?;
            $core(&db.conn, &principal, input)
                .await
                .map_err(IdentityError::command_error)
        }
    };
}
input_command!(
    business_members_list,
    store::list_members,
    MembersInput,
    Vec<Member>
);
input_command!(
    business_members_create,
    store::create_member,
    CreateMemberInput,
    Member
);
input_command!(
    business_members_update,
    store::update_member,
    UpdateMemberInput,
    Member
);
input_command!(
    business_members_revoke,
    store::revoke_member,
    RevokeMemberInput,
    Member
);
input_command!(
    business_credentials_issue,
    store::issue_credential,
    IssueCredentialInput,
    IssuedCredential
);
input_command!(
    business_credentials_list,
    store::list_credentials,
    CredentialsInput,
    Vec<Credential>
);
input_command!(
    business_credentials_revoke,
    store::revoke_credential,
    RevokeCredentialInput,
    Credential
);
