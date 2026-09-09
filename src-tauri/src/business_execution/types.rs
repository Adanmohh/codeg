//! Wire contract3f164c2a. Public task projections are deliberately distinct from
//! private asset/session metadata; neither contains credentials or stored grants.
use serde::{Deserialize, Serialize};

macro_rules! vocabulary {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $($variant),+ }
    };
}
vocabulary!(Mode { Chat, Terminal });
vocabulary!(Custody {
    OriginalOperator,
    IsolatedMember
});
vocabulary!(Readiness { Ready, Blocked });
vocabulary!(SetupReason {
    MissingClient,
    MissingConfiguration,
    ModelUnavailable,
    ProfileUnavailable,
    TenantExecutionUnavailable,
    NativeBoundaryUnavailable,
    ClientResumeUnsupported,
});
vocabulary!(SessionStatus {
    Starting,
    Idle,
    Running,
    AwaitingInput,
    Stopped,
    Failed,
    Interrupted,
    Revoked,
    Closed,
});
vocabulary!(SessionReason {
    MissingClient,
    MissingConfiguration,
    ModelUnavailable,
    ProfileUnavailable,
    TenantExecutionUnavailable,
    NativeBoundaryUnavailable,
    ClientResumeUnsupported,
    AuthorityChanged,
    LaunchUncertain,
    ProcessGone,
});
vocabulary!(OperationKind {
    Start,
    Continue,
    Prompt,
    Attach,
    Stop,
    TerminalWrite,
    ImportOutput,
    Submit
});
vocabulary!(OperationStatus {
    Pending,
    Confirmed,
    Failed,
    Uncertain
});
vocabulary!(OperationReason {
    Invalid,
    Unauthorized,
    Forbidden,
    Missing,
    Conflict,
    Busy,
    Unavailable,
    SetupRequired,
    AuthorityChanged,
    Cancelled,
    LaunchUncertain,
    PromptUncertain,
    ContentChanged,
    ContentUnavailable,
    TransportUnavailable,
    RateLimited,
});
vocabulary!(Disposition { Preview, Download });
vocabulary!(OutputStatus {
    Available,
    Changed,
    Unsupported
});
vocabulary!(Visibility { Private, Task });
vocabulary!(AuthorityKind {
    Operator,
    Credential
});
vocabulary!(MessageRole { User, Assistant });
vocabulary!(ToolStatus {
    Running,
    Completed,
    Failed
});
vocabulary!(ResetReason {
    Initial,
    CursorExpired,
    CursorInvalid
});
vocabulary!(DetachReason {
    AuthorityChanged,
    GenerationChanged,
    ProcessGone,
    Lagged,
    ServerShutdown
});

macro_rules! input {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        pub struct $name { $(pub $field: $ty),* }
    };
}
input!(TaskInput { task_id: String });
input!(SessionInput { session_id: String });
input!(OperationInput {
    operation_id: String,
    kind: OperationKind
});
input!(SessionListInput { task_id: String, cursor: Option<String>, limit: u32 });
input!(StartInput {
    operation_id: String,
    task_id: String,
    expected_task_revision: i64,
    profile_id: String,
    expected_profile_revision: i64,
    mode: Mode,
});
input!(SessionMutationInput {
    operation_id: String,
    session_id: String,
    expected_session_revision: i64,
});
input!(PromptInput {
    operation_id: String, session_id: String, expected_session_revision: i64,
    text: String, inputs: Vec<InputRef>,
});
input!(EventsInput {
    operation_id: String, session_id: String, expected_generation: i64, cursor: Option<String>,
});
input!(HistoryInput { session_id: String, before_cursor: Option<String>, limit: u32 });
input!(TerminalWriteInput {
    operation_id: String,
    session_id: String,
    expected_generation: i64,
    data: String,
});
input!(ResizeInput {
    session_id: String,
    expected_generation: i64,
    cols: u16,
    rows: u16
});
input!(OutputListInput { session_id: String, cursor: Option<String>, limit: u32 });
input!(ImportOutputInput {
    operation_id: String, session_id: String, output_id: String, expected_output_revision: i64,
    title: String, asset_id: Option<String>, expected_asset_revision: Option<i64>,
});
input!(AssetListInput { task_id: String, query: Option<String>, cursor: Option<String>, limit: u32 });
input!(VersionsInput { asset_id: String, cursor: Option<String>, limit: u32 });
input!(AssetInput {
    asset_id: String,
    version_id: String
});
input!(ContentInput {
    asset_id: String,
    version_id: String,
    disposition: Disposition
});
input!(SubmitInput {
    operation_id: String, task_id: String, expected_task_revision: i64,
    versions: Vec<VersionSelection>, body: String,
});
input!(VersionSelection {
    asset_id: String,
    version_id: String
});
input!(PublishedInput {
    task_id: String,
    deliverable_id: String,
    asset_id: String,
    version_id: String
});
input!(PublishedContentInput {
    task_id: String,
    deliverable_id: String,
    asset_id: String,
    version_id: String,
    disposition: Disposition,
});

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum InputRef {
    Asset {
        #[serde(rename = "assetId")]
        asset_id: String,
        #[serde(rename = "versionId")]
        version_id: String,
    },
    Task {
        #[serde(rename = "taskId")]
        task_id: String,
        #[serde(rename = "expectedRevision")]
        expected_revision: i64,
    },
    AccountSnapshot {
        #[serde(rename = "snapshotId")]
        snapshot_id: String,
        #[serde(rename = "expectedRevision")]
        expected_revision: i64,
    },
}

// Output structs use the same strict serde convention for retained receipt
// decoding. They are never accepted as caller authority/provenance.
input!(ConfiguredModel {
    id: String,
    reasoning: String
});
input!(ProfileCapabilities {
    start: bool,
    r#continue: bool,
    managed_output: bool,
    office_preview: bool
});
input!(ProfileSummary {
    id: String, revision: i64, label: String, client_id: String, modes: Vec<Mode>,
    custody: Custody, model: Option<ConfiguredModel>, readiness: Readiness,
    reason: Option<SetupReason>, capabilities: ProfileCapabilities,
});
input!(SessionCapabilities {
    read: bool,
    prompt: bool,
    r#continue: bool,
    stop: bool,
    terminal_write: bool,
    import_output: bool,
});
input!(SessionSummary {
    id: String, task_id: String, profile_id: String, profile_revision: i64, revision: i64,
    generation: i64, status: SessionStatus, mode: Mode, title: String, created_at: String,
    updated_at: String, last_activity_at: String, capabilities: SessionCapabilities,
    reason: Option<SessionReason>,
});
input!(OperationSummary { id: String, status: OperationStatus, reason: Option<OperationReason> });
input!(OperationResult { operation: OperationSummary, resource_id: Option<String> });
input!(PromptResult {
    operation: OperationSummary,
    message_id: String,
    input_hash: String
});
input!(ProfilesResult { profiles: Vec<ProfileSummary>, unavailable_reason: Option<SetupReason> });
input!(SessionGetResult {
    session: SessionSummary
});
input!(SessionResult {
    session: SessionSummary,
    operation: OperationSummary
});
input!(OutputCandidate {
    id: String,
    revision: i64,
    name: String,
    media_type: String,
    byte_size: i64,
    modified_at: String,
    status: OutputStatus,
});
input!(AssetSummary {
    id: String,
    task_id: String,
    revision: i64,
    title: String,
    media_type: String,
    latest_version_id: String,
    created_at: String,
    updated_at: String,
});
input!(MemberAttribution {
    member_id: String,
    display_name: String
});
input!(PrivateProducer { session_id: String, turn_id: Option<String>, client_id: String, model: Option<String> });
input!(ReviewReference {
    task_id: String,
    deliverable_id: String,
    task_revision: i64
});
input!(AssetVersion {
    id: String, asset_id: String, version: i64, sha256: String, byte_size: i64,
    media_type: String, created_at: String, created_by: MemberAttribution,
    producer: PrivateProducer, visibility: Visibility, review_references: Vec<ReviewReference>,
});
input!(AssetCapabilities {
    read_content: bool,
    preview: bool,
    download: bool,
    submit: bool,
    add_version: bool
});
input!(AssetResult {
    asset: AssetSummary, version: AssetVersion, capabilities: AssetCapabilities,
    preview_reason: Option<OperationReason>,
});
input!(ImportResult {
    asset: AssetSummary,
    version: AssetVersion,
    operation: OperationSummary
});
input!(PublishedAssetRef {
    asset_id: String,
    version_id: String,
    title: String,
    media_type: String,
    byte_size: i64,
    sha256: String,
});
input!(PublicProducer { client_id: String, model: Option<String> });
input!(SubmissionActor {
    member_id: String,
    display_name: String,
    authority_kind: AuthorityKind
});
input!(Publication {
    deliverable_id: String,
    task_revision: i64,
    submitted_by: SubmissionActor
});
input!(PublishedVersion {
    version: PublishedAssetRef,
    created_at: String,
    producer: PublicProducer,
    publication: Publication,
});
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResult {
    pub detail: crate::business_tasks::types::Detail,
    pub operation: OperationSummary,
}

input!(MessagePart {
    message_id: String,
    role: MessageRole,
    part: u32,
    text: String,
    complete: bool
});
input!(ToolState {
    id: String,
    name: String,
    status: ToolStatus
});
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SessionEvent {
    Message { message: MessagePart },
    Tool { tool: ToolState },
    Status { status: SessionStatus },
    Terminal { data: String },
}
input!(SnapshotState { status: SessionStatus, messages: Vec<MessagePart>, tools: Vec<ToolState> });
input!(HistoryPage { messages: Vec<MessagePart>, next_before_cursor: Option<String> });

#[derive(Clone, Debug, Serialize)]
#[serde(
    tag = "type",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum EventFrame {
    Snapshot {
        session_id: String,
        generation: i64,
        operation: OperationSummary,
        cursor: String,
        reset: bool,
        reason: ResetReason,
        state: SnapshotState,
        older_cursor: Option<String>,
    },
    Replay {
        session_id: String,
        generation: i64,
        operation: OperationSummary,
        cursor: String,
        reset: bool,
        events: Vec<SessionEvent>,
    },
    Event {
        session_id: String,
        generation: i64,
        cursor: String,
        event: SessionEvent,
    },
    Heartbeat {
        session_id: String,
        generation: i64,
        cursor: String,
    },
    Detached {
        session_id: String,
        generation: i64,
        reason: DetachReason,
    },
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

input!(ContentMetadata {
    media_type: String,
    byte_size: i64,
    sha256: String,
    file_name: String
});
input!(NativeContent {
    metadata: ContentMetadata,
    bytes_base64: String
});
