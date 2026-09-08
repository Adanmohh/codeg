//! Fixed read projections of firefliesai/n8n-nodes-fireflies at
//! fbd24607bc784a2294ce402426aefe2cb8c00f50, MIT Copyright2022 n8n. See NOTICE.
use super::{
    common,
    error::Reason,
    types::{ContentState, PassageKind, SummaryState},
};
use serde::Serialize;
use serde_json::{json, Value};
use std::{collections::HashSet, time::Duration};

const ENDPOINT: &str = "https://api.fireflies.ai/graphql";
const MAX_BYTES: usize = 2 * 1024 * 1024;
const USER: &str = "query ValidateCredentials { user { user_id } }";
const LIST: &str = "query GetTranscriptsList($fromDate: DateTime, $toDate: DateTime, $limit: Int, $skip: Int, $userId: String, $mine: Boolean) { transcripts(fromDate: $fromDate, toDate: $toDate, limit: $limit, skip: $skip, user_id: $userId, mine: $mine) { id title dateString } }";
const DETAIL: &str = "query Transcript($transcriptId: String!) { transcript(id: $transcriptId) { id title dateString user { user_id } privacy shared_with { expires_at } sentences { index text start_time end_time } summary { action_items } meeting_info { summary_status } } }";

#[derive(Debug)]
pub(super) struct ReadError {
    pub reason: Reason,
    pub retry_after: Option<i64>,
}
impl From<Reason> for ReadError {
    fn from(reason: Reason) -> Self {
        Self {
            reason,
            retry_after: None,
        }
    }
}
type Result<T> = std::result::Result<T, ReadError>;
pub(super) struct Reader {
    client: reqwest::Client,
    endpoint: String,
}
impl Reader {
    pub fn production() -> Result<Self> {
        Self::build(ENDPOINT.to_owned(), Duration::from_secs(12))
    }
    fn build(endpoint: String, timeout: Duration) -> Result<Self> {
        let client = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .referer(false)
            .retry(reqwest::retry::never())
            .timeout(timeout)
            .build()
            .map_err(|_| Reason::ProviderUnavailable)?;
        Ok(Self { client, endpoint })
    }
    #[cfg(test)]
    pub fn fixture(address: std::net::SocketAddr, timeout: Duration) -> Result<Self> {
        if !address.ip().is_loopback() {
            return Err(Reason::SourceDenied.into());
        }
        Self::build(format!("http://{address}/graphql"), timeout)
    }
    async fn query(&self, secret: &str, query: &str, variables: Value) -> Result<Value> {
        if secret.is_empty() || secret.len() > 4096 || !secret.bytes().all(|b| b.is_ascii_graphic())
        {
            return Err(Reason::CredentialUnavailable.into());
        }
        let mut response = self
            .client
            .post(&self.endpoint)
            .bearer_auth(secret)
            .json(&json!({"query":query,"variables":variables}))
            .send()
            .await
            .map_err(network)?;
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|h| h.to_str().ok())
                .and_then(retry_after);
            return Err(ReadError {
                reason: if matches!(status, 401 | 403 | 404) {
                    Reason::SourceDenied
                } else {
                    Reason::ProviderUnavailable
                },
                retry_after: if status == 429 {
                    retry_after
                } else if status >= 500 {
                    Some(retry_after.unwrap_or(5))
                } else {
                    None
                },
            });
        }
        if response
            .content_length()
            .is_some_and(|n| n > MAX_BYTES as u64)
        {
            return Err(Reason::UnsupportedSchema.into());
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(network)? {
            if bytes.len().saturating_add(chunk.len()) > MAX_BYTES {
                return Err(Reason::UnsupportedSchema.into());
            }
            bytes.extend_from_slice(&chunk);
        }
        let value: Value = serde_json::from_slice(&bytes).map_err(|_| Reason::UnsupportedSchema)?;
        if let Some(errors) = value.get("errors").filter(|v| !v.is_null()) {
            if !errors.as_array().is_some_and(Vec::is_empty) {
                return Err(Reason::ProviderUnavailable.into());
            }
        }
        value
            .get("data")
            .filter(|d| d.is_object())
            .cloned()
            .ok_or_else(|| Reason::UnsupportedSchema.into())
    }
    pub async fn user(&self, secret: &str) -> Result<String> {
        let data = self.query(secret, USER, json!({})).await?;
        let id = data
            .pointer("/user/user_id")
            .and_then(Value::as_str)
            .filter(|id| provider_id(id) && *id != "auth_failed")
            .ok_or(Reason::SourceDenied)?;
        Ok(id.to_owned())
    }
    pub async fn list(
        &self,
        secret: &str,
        provider_user: &str,
        from: &str,
        to: &str,
        offset: i64,
    ) -> Result<Vec<Listed>> {
        if !provider_id(provider_user) || !(0..250).contains(&offset) || offset % 50 != 0 {
            return Err(Reason::SourceDenied.into());
        }
        let data = self.query(secret, LIST, json!({"fromDate":from,"toDate":to,"limit":50,"skip":offset,"userId":provider_user,"mine":true})).await?;
        let rows = data
            .get("transcripts")
            .and_then(Value::as_array)
            .filter(|r| r.len() <= 50)
            .ok_or(Reason::UnsupportedSchema)?;
        rows.iter()
            .map(|row| {
                let id = row
                    .get("id")
                    .and_then(Value::as_str)
                    .filter(|id| provider_id(id))
                    .ok_or(Reason::UnsupportedSchema)?;
                Ok(Listed {
                    id: id.to_owned(),
                    title: title(row)?,
                })
            })
            .collect()
    }
    pub async fn detail(
        &self,
        secret: &str,
        provider_user: &str,
        expected_id: &str,
    ) -> Result<Normalized> {
        if !provider_id(expected_id) || !provider_id(provider_user) {
            return Err(Reason::SourceDenied.into());
        }
        let data = self
            .query(secret, DETAIL, json!({"transcriptId":expected_id}))
            .await?;
        normalize(
            data.get("transcript").ok_or(Reason::UnsupportedSchema)?,
            expected_id,
            provider_user,
        )
    }
}
fn network(error: reqwest::Error) -> ReadError {
    ReadError {
        reason: if error.is_timeout() {
            Reason::RequestTimeout
        } else {
            Reason::ProviderUnavailable
        },
        retry_after: Some(5),
    }
}
fn retry_after(value: &str) -> Option<i64> {
    value
        .parse::<i64>()
        .ok()
        .filter(|n| *n >= 0)
        .or_else(|| {
            chrono::DateTime::parse_from_rfc2822(value).ok().map(|t| {
                (t.with_timezone(&chrono::Utc) - chrono::Utc::now())
                    .num_seconds()
                    .max(0)
            })
        })
        .map(|n| n.clamp(1, 60))
}
pub(super) fn provider_id(id: &str) -> bool {
    !id.trim().is_empty() && id.chars().count() <= 256 && !id.chars().any(char::is_control)
}
fn title(row: &Value) -> Result<String> {
    let value = row
        .get("title")
        .and_then(Value::as_str)
        .filter(|s| common::text(s, 2000, false))
        .ok_or(Reason::UnsupportedSchema)?;
    Ok(value.to_owned())
}
pub(super) struct Listed {
    pub id: String,
    pub title: String,
}
#[derive(Clone, Serialize)]
pub(super) struct TextPassage {
    pub kind: PassageKind,
    pub text: String,
    pub index: Option<i64>,
    pub start: Option<f64>,
    pub end: Option<f64>,
}
pub(super) struct Normalized {
    pub title: String,
    pub passages: Vec<TextPassage>,
    pub content: ContentState,
    pub summary: SummaryState,
    pub provider_status: Option<String>,
    pub expires_at: Option<String>,
    pub provider_revision: Option<String>,
}
impl Normalized {
    pub fn digest(&self) -> common::Result<String> {
        // Fetch/health/status/expiry is not a provider content revision.
        common::digest(
            &json!({"normalization":1,"title":self.title,"passages":self.passages,"content":self.content,"summary":self.summary}),
        )
    }
}
fn normalize(row: &Value, expected_id: &str, provider_user: &str) -> Result<Normalized> {
    if row.get("id").and_then(Value::as_str) != Some(expected_id) {
        return Err(Reason::UnsupportedSchema.into());
    }
    if row.pointer("/user/user_id").and_then(Value::as_str) != Some(provider_user) {
        return Err(Reason::SourceDenied.into());
    }
    if row
        .get("privacy")
        .is_some_and(|v| !v.is_null() && !v.as_str().is_some_and(|s| common::text(s, 256, false)))
    {
        return Err(Reason::UnsupportedSchema.into());
    }
    let mut expires_at = None;
    if let Some(shared) = row.get("shared_with").filter(|v| !v.is_null()) {
        for item in shared.as_array().ok_or(Reason::UnsupportedSchema)? {
            if !item.is_object() {
                return Err(Reason::UnsupportedSchema.into());
            }
            if let Some(value) = item.get("expires_at").filter(|v| !v.is_null()) {
                let value = value.as_str().ok_or(Reason::SourceExpired)?;
                let instant = common::instant(value).map_err(|_| Reason::SourceExpired)?;
                if instant <= chrono::Utc::now() {
                    return Err(Reason::SourceExpired.into());
                }
                if expires_at
                    .as_ref()
                    .is_none_or(|old: &String| instant.to_rfc3339() < *old)
                {
                    expires_at = Some(instant.to_rfc3339());
                }
            }
        }
    }
    let mut passages = Vec::new();
    let mut indices = HashSet::new();
    if let Some(sentences) = row.get("sentences").filter(|v| !v.is_null()) {
        let sentences = sentences
            .as_array()
            .filter(|s| s.len() <= 10_000)
            .ok_or(Reason::UnsupportedSchema)?;
        for sentence in sentences {
            let text = sentence
                .get("text")
                .and_then(Value::as_str)
                .filter(|s| common::text(s, 20_000, false))
                .ok_or(Reason::UnsupportedSchema)?;
            let index = optional_number(sentence, "index", |v| v.as_i64().filter(|n| *n >= 0))?;
            if index.is_some_and(|i| !indices.insert(i)) {
                return Err(Reason::UnsupportedSchema.into());
            }
            let start = optional_number(sentence, "start_time", |v| {
                v.as_f64().filter(|n| n.is_finite() && *n >= 0.0)
            })?;
            let end = optional_number(sentence, "end_time", |v| {
                v.as_f64().filter(|n| n.is_finite() && *n >= 0.0)
            })?;
            if start.is_some() != end.is_some() || start.zip(end).is_some_and(|(s, e)| e < s) {
                return Err(Reason::UnsupportedSchema.into());
            }
            if !text.trim().is_empty() {
                passages.push(TextPassage {
                    kind: PassageKind::Sentence,
                    text: text.to_owned(),
                    index,
                    start,
                    end,
                });
            }
        }
    }
    let summary = match row.get("summary") {
        None | Some(Value::Null) => SummaryState::Missing,
        Some(object) if !object.is_object() => SummaryState::Unsupported,
        Some(object) => match object.get("action_items") {
            None | Some(Value::Null) => SummaryState::Missing,
            Some(Value::String(value)) => add_summary(&mut passages, std::slice::from_ref(value))?,
            Some(Value::Array(values))
                if values.len() <= 1000 && values.iter().all(Value::is_string) =>
            {
                let values: Vec<String> = values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect();
                add_summary(&mut passages, &values)?
            }
            Some(_) => SummaryState::Unsupported,
        },
    };
    let provider_status = match row.pointer("/meeting_info/summary_status") {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if common::text(s, 240, false) => Some(s.clone()),
        _ => return Err(Reason::UnsupportedSchema.into()),
    };
    let content = if passages.is_empty() {
        ContentState::Missing
    } else {
        ContentState::Available
    };
    Ok(Normalized {
        title: title(row)?,
        passages,
        content,
        summary,
        provider_status,
        expires_at,
        provider_revision: None,
    })
}
fn optional_number<T>(
    row: &Value,
    key: &str,
    parse: impl FnOnce(&Value) -> Option<T>,
) -> Result<Option<T>> {
    match row.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => parse(value)
            .map(Some)
            .ok_or_else(|| Reason::UnsupportedSchema.into()),
    }
}
fn add_summary(passages: &mut Vec<TextPassage>, values: &[String]) -> Result<SummaryState> {
    let mut any = false;
    for value in values {
        if !common::text(value, 20_000, false) {
            return Err(Reason::UnsupportedSchema.into());
        }
        if !value.trim().is_empty() {
            any = true;
            passages.push(TextPassage {
                kind: PassageKind::Summary,
                text: value.clone(),
                index: None,
                start: None,
                end: None,
            });
        }
    }
    Ok(if any {
        SummaryState::Available
    } else {
        SummaryState::Empty
    })
}
