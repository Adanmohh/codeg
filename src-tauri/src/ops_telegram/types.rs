use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConfigureInput {
    pub channel_id: i32,
    pub private_user_id: String,
    pub review_origin: String,
    pub enabled: bool,
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
    pub channel_id: i32,
    pub private_user_id: String,
    pub review_origin: String,
    pub revision: String,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notice {
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
}
