//! Port of services/agent/redaction.py at the IntroMail SHA in NOTICE.
use regex::Regex;
use serde_json::Value;
use std::sync::LazyLock;

static CREDENTIAL_KEY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(^|_)(password|passwd|secret|token|api[_-]?key|apikey|access[_-]?key|private[_-]?key|credential|authorization|auth[_-]?header|session[_-]?id)($|_)")
        .expect("constant credential regex")
});

pub fn scrub(value: &Value, private_fields: &[&str]) -> Value {
    match value {
        Value::Object(map) => Value::Object(map.iter().map(|(key, value)| {
            let value = if CREDENTIAL_KEY.is_match(key) || private_fields.contains(&key.as_str()) {
                Value::String("[redacted]".into())
            } else { scrub(value, private_fields) };
            (key.clone(), value)
        }).collect()),
        Value::Array(items) => Value::Array(items.iter().map(|v| scrub(v, private_fields)).collect()),
        _ => value.clone(),
    }
}
