//! Business vocabulary over Codeg's existing typed task-state pattern.
//! See NOTICE for the Apache source mapping; these are not executor states.
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
pub use crate::business_identity::Domain as TaskDomain;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    #[sea_orm(string_value = "todo")]
    Todo,
    #[sea_orm(string_value = "in_progress")]
    InProgress,
    #[sea_orm(string_value = "review")]
    Review,
    #[sea_orm(string_value = "done")]
    Done,
    #[sea_orm(string_value = "cancelled")]
    Cancelled,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::None)")]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    #[sea_orm(string_value = "low")]
    Low,
    #[default]
    #[sea_orm(string_value = "normal")]
    Normal,
    #[sea_orm(string_value = "high")]
    High,
    #[sea_orm(string_value = "urgent")]
    Urgent,
}

/// Progress inputs cannot express a completed or accepted task. Completion is
/// only available through the separate human-review operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressStatus {
    Todo,
    InProgress,
    Review,
}

impl From<ProgressStatus> for TaskStatus {
    fn from(status: ProgressStatus) -> Self {
        match status {
            ProgressStatus::Todo => Self::Todo,
            ProgressStatus::InProgress => Self::InProgress,
            ProgressStatus::Review => Self::Review,
        }
    }
}

pub(super) fn domain_key(domain: TaskDomain) -> &'static str {
    match domain {
        TaskDomain::Marketing => "marketing",
        TaskDomain::Channels => "channels",
        TaskDomain::Ads => "ads",
        TaskDomain::Website => "website",
        TaskDomain::Feedback => "feedback",
        TaskDomain::Engineering => "engineering",
    }
}

pub(super) fn status_key(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Todo => "todo",
        TaskStatus::InProgress => "in_progress",
        TaskStatus::Review => "review",
        TaskStatus::Done => "done",
        TaskStatus::Cancelled => "cancelled",
    }
}

pub(super) fn priority_key(priority: TaskPriority) -> &'static str {
    match priority {
        TaskPriority::Low => "low",
        TaskPriority::Normal => "normal",
        TaskPriority::High => "high",
        TaskPriority::Urgent => "urgent",
    }
}
