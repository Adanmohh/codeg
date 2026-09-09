//! Native owner boundary over the same business core. No caller identity input.
#![cfg(feature = "tauri-runtime")]
use crate::{
    app_error::AppCommandError,
    business_identity::{native::NativeSessions, store, types::*, IdentityError},
    db::AppDatabase,
};

#[tauri::command]
pub async fn business_context<R: tauri::Runtime>(
    window: tauri::WebviewWindow<R>,
    sessions: tauri::State<'_, NativeSessions>,
    session: Option<String>,
    db: tauri::State<'_, AppDatabase>,
) -> Result<Context, AppCommandError> {
    if crate::business_identity::native_acl::is_platform(window.label()) && session.is_none() {
        crate::commands::business_tenancy::platform_context(&window)?;
        store::operator_context(&db.conn)
            .await
            .map_err(IdentityError::command_error)
    } else {
        let principal = sessions
            .principal(&db.conn, window.label(), session.as_deref())
            .await
            .map_err(IdentityError::command_error)?;
        store::context(&db.conn, &principal)
            .await
            .map_err(IdentityError::command_error)
    }
}
#[tauri::command]
pub async fn business_bootstrap<R: tauri::Runtime>(
    window: tauri::WebviewWindow<R>,
    db: tauri::State<'_, AppDatabase>,
    input: BootstrapInput,
) -> Result<Context, AppCommandError> {
    crate::commands::business_tenancy::platform_context(&window)?;
    store::bootstrap(&db.conn, input)
        .await
        .map_err(IdentityError::command_error)
}
macro_rules! input_command {
    ($name:ident, $core:path, $input:ty, $result:ty) => {
        #[tauri::command]
        pub async fn $name<R: tauri::Runtime>(
            window: tauri::WebviewWindow<R>,
            sessions: tauri::State<'_, NativeSessions>,
            session: Option<String>,
            db: tauri::State<'_, AppDatabase>,
            input: $input,
        ) -> Result<$result, AppCommandError> {
            let principal = sessions
                .principal(&db.conn, window.label(), session.as_deref())
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
