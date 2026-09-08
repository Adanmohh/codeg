use super::{entity::task, types::Capabilities, vocabulary::*};
use crate::business_identity::{Member, MemberKind, Permission};

pub(super) fn terminal(status: TaskStatus) -> bool {
    matches!(status, TaskStatus::Done | TaskStatus::Cancelled)
}

pub(super) fn capabilities(row: &task::Model, actor: &Member, domain: TaskDomain) -> Capabilities {
    let human = actor.kind == MemberKind::Human;
    let contribute = actor.allows(Permission::Contribute, Some(domain));
    let manage = human && actor.allows(Permission::Assign, Some(domain));
    let involved = row.owner_id == actor.id
        || row.creator_id == actor.id
        || row.assignee_id.as_deref() == Some(actor.id.as_str());
    let own = contribute && (manage || involved);
    let open = row.archived_at.is_none() && !terminal(row.status);
    Capabilities {
        edit: human && own && open,
        assign: manage && open,
        progress: own && row.archived_at.is_none() && (human || open),
        review: human
            && actor.allows(Permission::Review, Some(domain))
            && open
            && row.status == TaskStatus::Review
            && row.reviewer_id.as_deref().is_none_or(|id| id == actor.id),
        comment: contribute && open,
        submit: own && open,
        cancel: human && own && open,
        archive: human && own && terminal(row.status),
        link_execution: manage && open && row.assignee_id.is_some(),
    }
}
