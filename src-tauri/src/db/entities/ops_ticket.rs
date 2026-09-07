//! MIT schema transcription from Chatwoot v4.17.1. See NOTICE and reports/tickets.md.
//! Ticket tables are separate from codeg ACP conversations.

pub mod inbox {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "ops_ticket_inbox")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub account_id: i32,
        pub name: String,
        pub email_address: String,
        pub channel_type: String,
        pub enable_auto_assignment: bool,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod contact {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "ops_ticket_contact")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub account_id: i32,
        pub name: String,
        pub email: String,
        pub phone_number: Option<String>,
        pub identifier: Option<String>,
        pub contact_type: i32,
        pub blocked: bool,
        pub additional_attributes: String,
        pub custom_attributes: String,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod contact_inbox {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "ops_ticket_contact_inbox")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub account_id: i32,
        pub contact_id: i32,
        pub inbox_id: i32,
        pub source_id: String,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod conversation {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "ops_ticket_conversation")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub account_id: i32,
        pub inbox_id: i32,
        pub contact_id: i32,
        pub contact_inbox_id: i32,
        pub uuid: String,
        pub status: i32,
        pub priority: Option<i32>,
        pub assignee_id: Option<String>,
        pub assignee_agent_bot_id: Option<String>,
        pub team_id: Option<String>,
        pub additional_attributes: String,
        pub custom_attributes: String,
        pub last_activity_at: DateTimeUtc,
        pub snoozed_until: Option<DateTimeUtc>,
        pub waiting_since: Option<DateTimeUtc>,
        pub first_reply_created_at: Option<DateTimeUtc>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod message {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "ops_ticket_message")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub account_id: i32,
        pub inbox_id: i32,
        pub conversation_id: i32,
        pub message_type: i32,
        pub private: bool,
        pub content: String,
        pub content_type: i32,
        pub status: i32,
        pub sender_type: String,
        pub sender_id: String,
        pub source_id: Option<String>,
        pub content_attributes: String,
        pub additional_attributes: String,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

