//! Native owner over the same bounded core; no shared-member credential fallback.
#![cfg(feature = "tauri-runtime")]
use crate::{
    app_error::AppCommandError,
    business_identity::{self, IdentityError},
    business_intake::{
        self as intake,
        error::{Error, Reason},
        services::Services,
        types::*,
    },
    db::AppDatabase,
};
macro_rules! command {
    ($name:ident,$core:ident,$input:ty,$result:ty) => {
        #[tauri::command]
        pub async fn $name(
            db: tauri::State<'_, AppDatabase>,
            input: $input,
        ) -> Result<$result, AppCommandError> {
            let principal = business_identity::operator_principal(&db.conn)
                .await
                .map_err(IdentityError::command_error)?;
            let services = Services::production().map_err(Error::command_error)?;
            intake::$core(&db.conn, &principal, &services, input)
                .await
                .map_err(Error::command_error)
        }
    };
}
command!(
    business_intake_bindings_list,
    bindings_list,
    PageInput,
    BindingList
);
command!(
    business_intake_bindings_status,
    bindings_status,
    BindingInput,
    BindingView
);
command!(
    business_intake_bindings_update,
    bindings_update,
    UpdateBindingInput,
    BindingAdmin
);
command!(
    business_intake_bindings_disable,
    bindings_disable,
    DisableBindingInput,
    BindingAdmin
);
command!(
    business_intake_grants_list,
    grants_list,
    BindingPageInput,
    GrantPage
);
command!(
    business_intake_grants_upsert,
    grants_upsert,
    UpsertGrantInput,
    GrantResult
);
command!(
    business_intake_grants_revoke,
    grants_revoke,
    RevokeGrantInput,
    GrantResult
);
#[tauri::command]
pub async fn business_intake_bindings_create(
    db: tauri::State<'_, AppDatabase>,
    input: CreateBindingInput,
) -> Result<BindingAdmin, AppCommandError> {
    let p = business_identity::operator_principal(&db.conn)
        .await
        .map_err(IdentityError::command_error)?;
    let op = if matches!(&input.source, SourceSetup::Fireflies { .. }) {
        None
    } else {
        Some(
            crate::ops::Operator::desktop()
                .map_err(|_| Error::from(Reason::BindingUnavailable).command_error())?,
        )
    };
    let services = Services::production().map_err(Error::command_error)?;
    intake::bindings_create(&db.conn, &p, &services, op.as_ref(), input)
        .await
        .map_err(Error::command_error)
}
