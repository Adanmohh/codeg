//! Desktop uses the same authenticated operator and host functions as HTTP.
#![cfg(feature = "tauri-runtime")]
use crate::{
    app_error::AppCommandError,
    db::AppDatabase,
    ops::Operator,
    ops_intake_host::{command_error, fix_task, operator, review, types::*, HostRuntime},
};
macro_rules! command {
    ($name:ident,$core:path,$input:ty,$result:ty) => {
        #[tauri::command]
        pub async fn $name(
            db: tauri::State<'_, AppDatabase>,
            input: $input,
        ) -> Result<$result, AppCommandError> {
            $core(
                &db.conn,
                &Operator::desktop()?,
                &HostRuntime::production(),
                input,
            )
            .await
            .map_err(command_error)
        }
    };
}
command!(
    ops_intake_configure,
    operator::configure,
    ConfigureInput,
    Status
);
command!(ops_intake_list, operator::list, ListInput, Listing);
command!(ops_intake_refresh, operator::refresh, SourceInput, Detail);
command!(ops_intake_save, operator::save, SaveInput, Draft);
command!(ops_intake_attach, operator::attach, AttachInput, Draft);
command!(ops_intake_prepare, review::prepare, PrepareInput, Draft);
command!(ops_intake_approve, review::approve, ReviewInput, Detail);
command!(ops_intake_deny, review::deny, DenyInput, Detail);
command!(ops_intake_reconcile, review::reconcile, SourceInput, Detail);
command!(ops_intake_fix, fix_task::create, SourceInput, Detail);
#[tauri::command]
pub async fn ops_intake_status(
    db: tauri::State<'_, AppDatabase>,
) -> Result<Status, AppCommandError> {
    operator::status(&db.conn, &Operator::desktop()?, &HostRuntime::production())
        .await
        .map_err(command_error)
}
#[tauri::command]
pub async fn ops_intake_detail(
    db: tauri::State<'_, AppDatabase>,
    input: SourceInput,
) -> Result<Detail, AppCommandError> {
    operator::detail(&db.conn, &Operator::desktop()?, input)
        .await
        .map_err(command_error)
}
