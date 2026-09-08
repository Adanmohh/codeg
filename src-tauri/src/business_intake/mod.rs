//! Explicit private-source intake, using the existing identity and task core.
//! Accepted contract670af9ca/access18be55ed; exact source mapping in NOTICE.
mod access;
mod common;
pub mod error;
mod fireflies;
pub(crate) mod http;
mod imports;
pub(crate) mod legacy;
mod records;
pub(crate) mod services;
mod setup;
mod sources;
pub mod types;

use crate::business_identity::Principal;
use sea_orm::DatabaseConnection;
use services::Services;
use types::*;

macro_rules! operation {
    ($name:ident,$module:ident::$function:ident,$input:ty,$output:ty) => {
        pub(crate) async fn $name(
            db: &DatabaseConnection,
            p: &Principal,
            services: &Services,
            input: $input,
        ) -> Result<$output, error::Error> {
            common::bounded($module::$function(db, p, services, input)).await
        }
    };
}
operation!(bindings_list, access::list, PageInput, BindingList);
operation!(bindings_status, access::status, BindingInput, BindingView);
operation!(sources_list, sources::list, BindingPageInput, SourcePage);
operation!(sources_get, sources::get, SourceInput, SourceDetail);
operation!(imports_start, imports::start, StartImportInput, Import);
operation!(imports_capture, imports::capture, CaptureInput, Import);
operation!(imports_list, imports::list, ImportsInput, ImportPage);
operation!(imports_get, imports::get, ImportInput, Import);
operation!(
    imports_advance,
    imports::advance,
    ImportRevisionInput,
    Import
);
operation!(imports_cancel, imports::cancel, ImportRevisionInput, Import);
operation!(
    bindings_update,
    setup::update,
    UpdateBindingInput,
    BindingAdmin
);
operation!(
    bindings_disable,
    setup::disable,
    DisableBindingInput,
    BindingAdmin
);
operation!(
    grants_list,
    access::grants_list,
    BindingPageInput,
    GrantPage
);
operation!(
    grants_upsert,
    access::grants_upsert,
    UpsertGrantInput,
    GrantResult
);
operation!(
    grants_revoke,
    access::grants_revoke,
    RevokeGrantInput,
    GrantResult
);
pub(crate) async fn bindings_create(
    db: &DatabaseConnection,
    p: &Principal,
    services: &Services,
    operator: Option<&crate::ops::Operator>,
    input: CreateBindingInput,
) -> Result<BindingAdmin, error::Error> {
    common::bounded(setup::create(db, p, services, operator, input)).await
}
#[cfg(test)]
mod tests;
