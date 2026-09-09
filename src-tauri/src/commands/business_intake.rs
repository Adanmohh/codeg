//! Native caller-window identity over the same bounded HTTP/core operations.
#![cfg(feature = "tauri-runtime")]
use crate::{
    app_error::AppCommandError,
    business_identity::{native::NativeSessions, IdentityError},
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
pub async fn business_intake_bindings_create<R: tauri::Runtime>(
    window: tauri::WebviewWindow<R>,
    sessions: tauri::State<'_, NativeSessions>,
    session: Option<String>,
    db: tauri::State<'_, AppDatabase>,
    input: CreateBindingInput,
) -> Result<BindingAdmin, AppCommandError> {
    let p = sessions
        .principal(&db.conn, window.label(), session.as_deref())
        .await
        .map_err(IdentityError::command_error)?;
    let op = if !p.is_operator() || matches!(&input.source, SourceSetup::Fireflies { .. }) {
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

command!(
    business_intake_sources_list,
    sources_list,
    BindingPageInput,
    SourcePage
);
command!(
    business_intake_sources_get,
    sources_get,
    SourceInput,
    SourceDetail
);
command!(
    business_intake_imports_start,
    imports_start,
    StartImportInput,
    Import
);
command!(
    business_intake_imports_capture,
    imports_capture,
    CaptureInput,
    Import
);
command!(
    business_intake_imports_list,
    imports_list,
    ImportsInput,
    ImportPage
);
command!(
    business_intake_imports_get,
    imports_get,
    ImportInput,
    Import
);
command!(
    business_intake_imports_advance,
    imports_advance,
    ImportRevisionInput,
    Import
);
command!(
    business_intake_imports_cancel,
    imports_cancel,
    ImportRevisionInput,
    Import
);
command!(
    business_intake_candidates_list,
    candidates_list,
    CandidatesInput,
    CandidatePage
);
command!(
    business_intake_candidates_get,
    candidates_get,
    CandidateInput,
    CandidateDetail
);
command!(
    business_intake_candidates_create,
    candidates_create,
    CreateCandidateInput,
    Candidate
);
command!(
    business_intake_candidates_select,
    candidates_select,
    SelectCandidateInput,
    CandidateDetail
);
command!(
    business_intake_candidates_edit,
    candidates_edit,
    EditCandidateInput,
    CandidateDetail
);
command!(
    business_intake_candidates_accept,
    candidates_accept,
    AcceptCandidateInput,
    DecisionResult
);
command!(
    business_intake_candidates_link,
    candidates_link,
    LinkCandidateInput,
    DecisionResult
);
command!(
    business_intake_candidates_discard,
    candidates_discard,
    DiscardInput,
    Decision
);
command!(
    business_intake_tasks_sources,
    tasks_sources,
    TaskInput,
    TaskSources
);
