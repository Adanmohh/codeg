//! Bounded plain text and calendar-date inputs shared by all transports.
use chrono::NaiveDate;

pub(super) const MAX_TITLE: usize = 240;
pub(super) const MAX_TEXT: usize = 20_000;

pub(super) fn valid_text(value: &str, limit: usize, required: bool) -> bool {
    (!required || !value.trim().is_empty())
        && value.chars().count() <= limit
        && !value
            .chars()
            .any(|ch| ch.is_control() && !matches!(ch, '\n' | '\r' | '\t'))
}

/// v1 deadlines are calendar dates, never instants. Keep the original date
/// unchanged in storage/JSON, so viewing it in another zone cannot move it.
pub(super) fn valid_due_date(value: Option<&str>) -> bool {
    let Some(value) = value else { return true };
    value.len() == 10
        && value.bytes().enumerate().all(|(index, byte)| {
            if matches!(index, 4 | 7) {
                byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        })
        && !value.starts_with("0000")
        && value.parse::<NaiveDate>().is_ok()
}
