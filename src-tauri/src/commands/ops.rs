//! Tauri boundary over the same core used by authenticated HTTP handlers.
#![cfg(feature = "tauri-runtime")]
use crate::{
    app_error::AppCommandError,
    db::AppDatabase,
    ops::{
        command_error, delivery,
        email::{self, EmailRuntime},
        review, store,
        types::*,
        Operator,
    },
};

macro_rules! read_command {
    ($name:ident, $core:path, $result:ty) => {
        #[tauri::command]
        pub async fn $name(db: tauri::State<'_, AppDatabase>) -> Result<$result, AppCommandError> {
            $core(&db.conn, &Operator::desktop()?)
                .await
                .map_err(command_error)
        }
    };
}
macro_rules! input_command {
    ($name:ident, $core:path, $input:ty, $result:ty) => {
        #[tauri::command]
        pub async fn $name(
            db: tauri::State<'_, AppDatabase>,
            input: $input,
        ) -> Result<$result, AppCommandError> {
            $core(&db.conn, &Operator::desktop()?, input)
                .await
                .map_err(command_error)
        }
    };
}
read_command!(ops_context, store::context, Context);
read_command!(ops_proposals, review::list, Vec<Proposal>);
read_command!(ops_morning, store::morning, Morning);
input_command!(ops_inbox_create, store::create_inbox, InboxInput, Inbox);
input_command!(ops_tickets, store::list_tickets, TicketsInput, TicketPage);
input_command!(ops_thread, store::thread, ThreadInput, Thread);
input_command!(ops_note_add, store::add_note, NoteInput, Message);
input_command!(ops_draft_save, store::save_draft, SaveDraftInput, Draft);
input_command!(ops_proposal_get, review::get, ProposalInput, Proposal);
input_command!(ops_proposal_deny, review::deny, DenyInput, Proposal);
input_command!(
    ops_email_reconcile_receipt,
    delivery::reconcile_receipt,
    ProposalInput,
    DeliveryStatus
);

macro_rules! email_command {
    ($name:ident, $core:path, $input:ty, $result:ty) => {
        #[tauri::command]
        pub async fn $name(
            db: tauri::State<'_, AppDatabase>,
            input: $input,
        ) -> Result<$result, AppCommandError> {
            $core(
                &db.conn,
                &Operator::desktop()?,
                &EmailRuntime::production(),
                input,
            )
            .await
        }
    };
}
email_command!(
    ops_email_status,
    email::status,
    EmailInboxInput,
    EmailStatus
);
email_command!(
    ops_email_configure,
    email::configure,
    EmailConfigureInput,
    EmailStatus
);
email_command!(
    ops_email_disconnect,
    email::disconnect,
    EmailInboxInput,
    EmailStatus
);
email_command!(ops_email_pull, email::pull, EmailInboxInput, PullResult);

#[tauri::command]
pub async fn ops_proposal_approve(
    db: tauri::State<'_, AppDatabase>,
    input: ReviewInput,
) -> Result<Proposal, AppCommandError> {
    review::approve(&db.conn, &Operator::desktop()?, input).await
}
