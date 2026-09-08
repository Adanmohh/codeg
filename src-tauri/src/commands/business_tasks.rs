//! Native owner authentication over the same task core as HTTP and agents.
#![cfg(feature = "tauri-runtime")]
use crate::{
    app_error::AppCommandError,
    business_identity::{self, IdentityError},
    business_tasks::{self, store, types::*, ActorContext},
    db::AppDatabase,
};

macro_rules! command {
    ($name:ident, $core:ident, $input:ty, $result:ty) => {
        #[tauri::command]
        pub async fn $name(
            db: tauri::State<'_, AppDatabase>,
            input: $input,
        ) -> Result<$result, AppCommandError> {
            let principal = business_identity::operator_principal(&db.conn)
                .await
                .map_err(IdentityError::command_error)?;
            store::$core(&db.conn, &ActorContext::authenticated(principal), input)
                .await
                .map_err(IdentityError::command_error)
        }
    };
}
command!(business_tasks_list, list, ListInput, TaskPage);
command!(business_tasks_get, get, TaskInput, Detail);
command!(business_tasks_create, create, CreateInput, Detail);
command!(business_tasks_update, update, UpdateInput, Detail);
command!(business_tasks_assign, assign, AssignInput, Detail);
command!(business_tasks_progress, progress, ProgressInput, Detail);
command!(business_tasks_note, note, TextInput, Detail);
command!(business_tasks_submit, submit, TextInput, Detail);
command!(business_tasks_review, review, ReviewInput, Detail);
command!(business_tasks_cancel, cancel, RevisionInput, Detail);
command!(business_tasks_archive, archive, ArchiveInput, Detail);

#[tauri::command]
pub async fn business_tasks_link_execution(
    db: tauri::State<'_, AppDatabase>,
    input: LinkExecutionInput,
) -> Result<Detail, AppCommandError> {
    let principal = business_identity::operator_principal(&db.conn)
        .await
        .map_err(IdentityError::command_error)?;
    business_tasks::link_execution(ActorContext::authenticated(principal), input)
        .await
        .map_err(IdentityError::command_error)
}
