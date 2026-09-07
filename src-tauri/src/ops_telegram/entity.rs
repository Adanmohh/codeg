pub(super) mod config {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "ops_telegram_config")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub account_id: i32,
        pub enabled: bool,
        pub channel_id: i32,
        pub private_user_id: String,
        pub channel_sha256: String,
        pub review_origin: String,
        pub revision: String,
        pub updated_by: String,
        pub updated_at: DateTimeUtc,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
pub(super) mod notice {
    use sea_orm::entity::prelude::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "ops_telegram_notice")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub account_id: i32,
        pub proposal_id: i32,
        pub task_id: i32,
        pub run_seq: i32,
        pub action_kind: String,
        pub snapshot_sha256: String,
        pub config_revision: String,
        pub claim_id: String,
        pub channel_id: i32,
        pub private_user_id: String,
        pub channel_sha256: String,
        pub status: String,
        pub provider_message_id: Option<String>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}
