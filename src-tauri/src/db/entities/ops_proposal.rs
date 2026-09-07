// Ported from IntroInnovation/intromail; immutable source mapping in NOTICE.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ops_proposal")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub task_id: i32,
    pub run_seq: i32,
    pub agent_id: String,
    pub action_name: String,
    pub payload_json: String,
    pub preview_json: String,
    pub edited_payload_json: Option<String>,
    pub status: String,
    pub verdict_by: Option<String>,
    pub created_at: DateTimeUtc,
    pub resolved_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
