//! Codeg SeaORM task/event entities; organization business fields are glue.
pub(super) mod task {
    use super::super::vocabulary::{TaskPriority, TaskStatus};
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "business_task")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub organization_id: String,
        pub title: String,
        pub notes: String,
        pub domain: String,
        pub status: TaskStatus,
        pub priority: TaskPriority,
        pub due_date: Option<String>,
        pub owner_id: String,
        pub assignee_id: Option<String>,
        pub creator_id: String,
        pub reviewer_id: Option<String>,
        pub revision: i64,
        pub current_deliverable_id: Option<String>,
        pub current_execution_id: Option<String>,
        pub created_at: String,
        pub updated_at: String,
        pub archived_at: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub(super) mod activity {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "business_task_activity")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub organization_id: String,
        pub task_id: String,
        pub revision: i64,
        pub kind: String,
        pub actor_id: String,
        pub actor_name: String,
        pub actor_kind: String,
        pub payload_json: String,
        pub created_at: String,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub(super) mod deliverable {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "business_task_deliverable")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub organization_id: String,
        pub task_id: String,
        pub revision: i64,
        pub author_id: String,
        pub author_name: String,
        pub author_kind: String,
        pub body: String,
        pub execution_id: Option<String>,
        pub created_at: String,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub(super) mod execution {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "business_task_execution")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub organization_id: String,
        pub task_id: String,
        pub authority_id: String,
        pub work_task_id: i32,
        pub run_seq: i32,
        pub connection_id: String,
        pub agent_member_id: String,
        pub agent_key: String,
        /// Private identity-owned delegation handle; never projected to DTOs.
        pub delegation_json: String,
        pub linked_by: String,
        pub created_at: String,
        pub revoked_at: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

/// Protected operator entrustment, never deserialized from or projected to an
/// agent/member response. A wire agent type is not this business member identity.
pub(super) mod execution_authority {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "business_task_execution_authority")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub organization_id: String,
        pub task_id: String,
        pub domain: String,
        pub task_revision: i64,
        pub work_task_id: i32,
        pub run_seq: i32,
        pub connection_id: String,
        pub agent_member_id: String,
        pub agent_key: String,
        pub entrusted_by: String,
        pub created_at: String,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
