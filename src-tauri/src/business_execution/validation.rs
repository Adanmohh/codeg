//! Validate closed caller data before reserving any durable side effect.
use super::types::*;

pub const MAX_REVISION: i64 = 9_007_199_254_740_991;
pub const MAX_OUTPUT_BYTES: u64 = 50 * 1024 * 1024;
pub const MAX_PART_BYTES: usize = 16 * 1024;
pub const MAX_FRAME_BYTES: usize = 1024 * 1024;

pub fn uuid(value: &str) -> Result<(), OperationReason> {
    uuid::Uuid::parse_str(value)
        .map(|_| ())
        .map_err(|_| OperationReason::Invalid)
}
pub fn revision(value: i64) -> Result<(), OperationReason> {
    if (1..=MAX_REVISION).contains(&value) {
        Ok(())
    } else {
        Err(OperationReason::Invalid)
    }
}
pub fn text(value: &str, min: usize, max: usize) -> Result<(), OperationReason> {
    let length = value.chars().count();
    if length < min
        || length > max
        || value
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
    {
        return Err(OperationReason::Invalid);
    }
    Ok(())
}
pub fn page(limit: u32, maximum: u32, cursor: Option<&str>) -> Result<(), OperationReason> {
    if limit == 0 || limit > maximum || cursor.is_some_and(|c| c.len() > 256) {
        return Err(OperationReason::Invalid);
    }
    Ok(())
}
pub fn start(input: &StartInput) -> Result<(), OperationReason> {
    uuid(&input.operation_id)?;
    uuid(&input.task_id)?;
    uuid(&input.profile_id)?;
    revision(input.expected_task_revision)?;
    revision(input.expected_profile_revision)
}
pub fn mutation(input: &SessionMutationInput) -> Result<(), OperationReason> {
    uuid(&input.operation_id)?;
    uuid(&input.session_id)?;
    revision(input.expected_session_revision)
}
pub fn prompt(input: &PromptInput) -> Result<(), OperationReason> {
    uuid(&input.operation_id)?;
    uuid(&input.session_id)?;
    revision(input.expected_session_revision)?;
    text(&input.text, 1, 32_000)?;
    if input.inputs.len() > 16 {
        return Err(OperationReason::Invalid);
    }
    for reference in &input.inputs {
        match reference {
            InputRef::AccountSnapshot { .. } => return Err(OperationReason::Unavailable),
            InputRef::Asset {
                asset_id,
                version_id,
            } => {
                uuid(asset_id)?;
                uuid(version_id)?;
            }
            InputRef::Task {
                task_id,
                expected_revision,
            } => {
                uuid(task_id)?;
                revision(*expected_revision)?;
            }
        }
    }
    Ok(())
}
pub fn submit(input: &SubmitInput) -> Result<(), OperationReason> {
    uuid(&input.operation_id)?;
    uuid(&input.task_id)?;
    revision(input.expected_task_revision)?;
    text(&input.body, 0, 20_000)?;
    if input.versions.is_empty() || input.versions.len() > 16 {
        return Err(OperationReason::Invalid);
    }
    let mut selected = std::collections::HashSet::new();
    for v in &input.versions {
        uuid(&v.asset_id)?;
        uuid(&v.version_id)?;
        if !selected.insert((&v.asset_id, &v.version_id)) {
            return Err(OperationReason::Invalid);
        }
    }
    Ok(())
}
pub fn import(input: &ImportOutputInput) -> Result<(), OperationReason> {
    uuid(&input.operation_id)?;
    uuid(&input.session_id)?;
    uuid(&input.output_id)?;
    revision(input.expected_output_revision)?;
    text(&input.title, 1, 240)?;
    if input.title.trim().is_empty() {
        return Err(OperationReason::Invalid);
    }
    match (&input.asset_id, input.expected_asset_revision) {
        (None, None) => Ok(()),
        (Some(id), Some(rev)) => {
            uuid(id)?;
            revision(rev)
        }
        _ => Err(OperationReason::Invalid),
    }
}
pub fn event(event: &SessionEvent) -> Result<(), OperationReason> {
    match event {
        SessionEvent::Message { message } if message.text.len() > MAX_PART_BYTES => {
            Err(OperationReason::Invalid)
        }
        SessionEvent::Terminal { data } if data.len() > MAX_PART_BYTES => {
            Err(OperationReason::Invalid)
        }
        SessionEvent::Tool { tool } => text(&tool.name, 1, 120),
        _ => Ok(()),
    }
}
