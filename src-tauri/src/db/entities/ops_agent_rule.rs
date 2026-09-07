// Ported from IntroInnovation/intromail; immutable source mapping in NOTICE.
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ops_agent_rule")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub agent_id: String,
    pub domain: String,
    pub resource: Option<String>,
    pub action_name: Option<String>,
    pub behavior: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
