pub mod config {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "ops_email_config")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub inbox_id: i32,
        pub account_id: i32,
        pub credential_ref: String,
        pub last_pull_at: Option<DateTimeUtc>,
        pub last_pull_status: Option<String>,
        pub last_pull_error: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub mod attempt {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "ops_email_attempt")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub account_id: i32,
        pub inbox_id: i32,
        pub conversation_id: i32,
        pub proposal_id: i32,
        pub task_id: i32,
        pub run_seq: i32,
        pub draft_id: i32,
        pub draft_revision: i32,
        pub payload_json: String,
        pub payload_sha256: String,
        pub message_id: String,
        pub idempotency_key: String,
        pub actor: String,
        pub status: String,
        pub provider_id: Option<String>,
        pub error: Option<String>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
