use super::vocabulary::{ProgressStatus, TaskDomain, TaskPriority, TaskStatus};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateInput {
    pub title: String,
    #[serde(default)]
    pub notes: String,
    pub domain: TaskDomain,
    #[serde(default)]
    pub priority: TaskPriority,
    pub due_date: Option<String>,
    pub owner_id: Option<String>,
    pub assignee_id: Option<String>,
    pub reviewer_id: Option<String>,
}

/// Exact human-prepared input with all defaults resolved, never an authority.
/// Intake may persist this privately; creation repeats current authorization.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PreparedTask {
    pub title: String,
    pub notes: String,
    pub domain: TaskDomain,
    pub priority: TaskPriority,
    pub due_date: Option<String>,
    pub owner_id: String,
    pub assignee_id: Option<String>,
    pub reviewer_id: Option<String>,
}

impl From<PreparedTask> for CreateInput {
    fn from(value: PreparedTask) -> Self {
        Self {
            title: value.title,
            notes: value.notes,
            domain: value.domain,
            priority: value.priority,
            due_date: value.due_date,
            owner_id: Some(value.owner_id),
            assignee_id: value.assignee_id,
            reviewer_id: value.reviewer_id,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateInput {
    pub task_id: String,
    pub expected_revision: i64,
    pub title: String,
    pub notes: String,
    pub domain: TaskDomain,
    pub priority: TaskPriority,
    pub due_date: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AssignInput {
    pub task_id: String,
    pub expected_revision: i64,
    pub owner_id: String,
    pub assignee_id: Option<String>,
    pub reviewer_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TaskInput {
    pub task_id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RevisionInput {
    pub task_id: String,
    pub expected_revision: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProgressInput {
    pub task_id: String,
    pub expected_revision: i64,
    pub status: ProgressStatus,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TextInput {
    pub task_id: String,
    pub expected_revision: i64,
    pub body: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    Accept,
    Return,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReviewInput {
    pub task_id: String,
    pub expected_revision: i64,
    pub decision: ReviewDecision,
    #[serde(default)]
    pub comment: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArchiveInput {
    pub task_id: String,
    pub expected_revision: i64,
    pub archived: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LinkExecutionInput {
    pub task_id: String,
    pub expected_revision: i64,
    pub work_task_id: i32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskView {
    #[default]
    Shared,
    Mine,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ListInput {
    #[serde(default)]
    pub view: TaskView,
    pub domain: Option<TaskDomain>,
    pub status: Option<TaskStatus>,
    pub query: Option<String>,
    #[serde(default)]
    pub page: u32,
    #[serde(default)]
    pub archived: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    pub edit: bool,
    pub assign: bool,
    pub progress: bool,
    pub review: bool,
    pub comment: bool,
    pub submit: bool,
    pub cancel: bool,
    pub archive: bool,
    pub link_execution: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub organization_id: String,
    pub title: String,
    pub notes: String,
    pub domain: TaskDomain,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<String>,
    pub owner_id: String,
    pub assignee_id: Option<String>,
    pub creator_id: String,
    pub reviewer_id: Option<String>,
    pub revision: i64,
    pub current_deliverable_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub capabilities: Capabilities,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskPage {
    pub tasks: Vec<Task>,
    pub page: u32,
    pub has_more: bool,
    pub can_create: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub id: String,
    pub display_name: String,
    pub kind: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: String,
    pub revision: i64,
    pub kind: String,
    pub actor: Actor,
    pub payload: serde_json::Value,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Deliverable {
    pub id: String,
    pub revision: i64,
    pub author: Actor,
    pub body: String,
    /// Exact selected immutable versions, never the asset's private latest state.
    pub assets: Vec<crate::business_execution::types::PublishedAssetRef>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    pub work_task_id: i32,
    pub run_seq: i32,
    pub agent_member_id: String,
    pub active: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Detail {
    pub task: Task,
    pub activity: Vec<Activity>,
    pub deliverables: Vec<Deliverable>,
    pub execution: Option<Execution>,
}
