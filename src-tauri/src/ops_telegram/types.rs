use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    #[sea_orm(string_value = "email_reply")]
    EmailReply,
    #[sea_orm(string_value = "github_issue")]
    GithubIssue,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigureInput {
    pub channel_id: i32,
    pub private_user_id: String,
    pub review_origin: String,
    pub enabled: bool,
    #[serde(default)]
    pub github_issues_enabled: bool,
    pub expected_revision: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResolveInput {
    pub notice: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelChoice {
    pub id: i32,
    pub name: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub github_issues_enabled: bool,
    pub channel_id: i32,
    pub private_user_id: String,
    pub review_origin: String,
    pub revision: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notice {
    pub action_kind: ActionKind,
    pub proposal_id: i32,
    pub task_id: i32,
    pub run_seq: i32,
    pub status: String,
    pub updated_at: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub enabled: bool,
    pub state: &'static str,
    pub configuration: Option<Configuration>,
    pub channels: Vec<ChannelChoice>,
    pub notices: Vec<Notice>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub state: &'static str,
    pub proposal: Option<crate::ops::types::Proposal>,
    pub issue: Option<crate::ops_intake_host::notice::IssueReview>,
}
