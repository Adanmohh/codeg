//! Narrow Rust port of intromail's App client; see NOTICE and report. The
//! create-issue call is glue verified against frozen official GitHub OpenAPI.
use super::types::*;
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::{header::HeaderMap, Client, Method, Response};
use serde::Serialize;
use serde_json::{json, Value};
use std::{collections::HashMap, time::Duration};
use tokio::sync::Mutex;

const API: &str = "https://api.github.com";
const API_VERSION: &str = "2022-11-28";
const MAX_RESPONSE: usize = 2 * 1024 * 1024;

/// Host-only configuration. No Debug/Serialize and no agent-provided key/path.
pub struct GithubAppConfig {
    pub app_id: String,
    pub private_key_pem: String,
}
struct SigningIdentity {
    app_id: String,
    key: EncodingKey,
    generation: String,
}
#[derive(Clone, Hash, PartialEq, Eq)]
struct CacheKey {
    app_id: String,
    generation: String,
    installation: i64,
    repo: i64,
    full_name: String,
}
#[derive(Clone)]
pub(super) struct InstallationToken {
    value: String,
    expires: i64,
}

pub struct GithubAppClient {
    http: Client,
    api: String,
    identity: Option<SigningIdentity>,
    cache: Mutex<HashMap<CacheKey, InstallationToken>>,
    cooldown: Mutex<i64>,
    pub(super) operation: Mutex<()>,
}
#[derive(Serialize)]
struct Claims<'a> {
    iat: i64,
    exp: i64,
    iss: &'a str,
}

pub(super) enum SendOutcome {
    Created(CreatedIssue),
    Failed(&'static str, Option<i64>),
    Unknown,
}

impl GithubAppClient {
    /// Test-only host/browser fixtures. No runtime API URL override exists.
    #[cfg(any(test, feature = "test-utils"))]
    pub(crate) fn loopback_fixture(
        config: Option<GithubAppConfig>,
        address: std::net::SocketAddr,
    ) -> Result<Self, IntakeError> {
        if !address.ip().is_loopback() { return Err(IntakeError::AccessDenied); }
        let mut client = Self::new(config)?;
        client.api = format!("http://{address}");
        Ok(client)
    }
    pub fn new(config: Option<GithubAppConfig>) -> Result<Self, IntakeError> {
        let identity = config
            .map(|config| {
                if !identifier(&config.app_id) || config.private_key_pem.trim().is_empty() {
                    return Err(IntakeError::NotConfigured);
                }
                let pem = if config.private_key_pem.contains("\\n")
                    && !config.private_key_pem.contains('\n')
                {
                    config.private_key_pem.replace("\\n", "\n")
                } else {
                    config.private_key_pem
                };
                let key = EncodingKey::from_rsa_pem(pem.as_bytes())
                    .map_err(|_| IntakeError::NotConfigured)?;
                Ok(SigningIdentity {
                    app_id: config.app_id,
                    key,
                    generation: digest(pem),
                })
            })
            .transpose()?;
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .no_proxy()
            .user_agent("Hafidh-Ops-Desk-internal")
            .build()
            .map_err(|_| IntakeError::NotConfigured)?;
        Ok(Self {
            http,
            api: API.into(),
            identity,
            cache: Mutex::new(HashMap::new()),
            cooldown: Mutex::new(0),
            operation: Mutex::new(()),
        })
    }
    pub fn is_configured(&self) -> bool {
        self.identity.is_some()
    }
    /// Key rotation uses a new immutable client; no old-generation cache survives.
    pub async fn clear_token_cache(&self) {
        self.cache.lock().await.clear();
    }

    fn key(&self, binding: &RepositoryBinding) -> Result<CacheKey, IntakeError> {
        binding.validate()?;
        let identity = self.identity.as_ref().ok_or(IntakeError::NotConfigured)?;
        if !binding.enabled || binding.app_id != identity.app_id {
            return Err(IntakeError::AccessDenied);
        }
        Ok(CacheKey {
            app_id: identity.app_id.clone(),
            generation: identity.generation.clone(),
            installation: binding.installation_id,
            repo: binding.repository_id,
            full_name: binding.full_name.clone(),
        })
    }
    fn app_jwt(&self, now: i64) -> Result<String, IntakeError> {
        let identity = self.identity.as_ref().ok_or(IntakeError::NotConfigured)?;
        encode(
            &Header::new(Algorithm::RS256),
            &Claims {
                iat: now - 60,
                exp: now + 540,
                iss: &identity.app_id,
            },
            &identity.key,
        )
        .map_err(|_| IntakeError::NotConfigured)
    }
    async fn request(
        &self,
        method: Method,
        path: &str,
        bearer: &str,
        body: Option<&Value>,
    ) -> Result<Response, IntakeError> {
        if *self.cooldown.lock().await > Utc::now().timestamp() {
            return Err(IntakeError::UpstreamUnavailable);
        }
        let mut request = self
            .http
            .request(method, format!("{}{path}", self.api))
            .bearer_auth(bearer)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", API_VERSION);
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request
            .send()
            .await
            .map_err(|_| IntakeError::UpstreamUnavailable)?;
        if matches!(response.status().as_u16(), 403 | 429) {
            if let Some(until) = retry_after(response.headers()) {
                *self.cooldown.lock().await = until;
            }
        }
        Ok(response)
    }

    pub(super) async fn installation_token(
        &self,
        binding: &RepositoryBinding,
    ) -> Result<InstallationToken, IntakeError> {
        let key = self.key(binding)?;
        let now = Utc::now().timestamp();
        // Single-flight exchange. Token scope is always exactly one repo and
        // issues:write + metadata:read. No caller can expand permissions.
        let mut cache = self.cache.lock().await;
        if let Some(token) = cache.get(&key) {
            if token.expires - 60 > now {
                return Ok(token.clone());
            }
        }
        cache.remove(&key);
        let response = self.request(Method::POST, &format!("/app/installations/{}/access_tokens", binding.installation_id),
            &self.app_jwt(now)?, Some(&json!({"repository_ids":[binding.repository_id],"permissions":{"issues":"write","metadata":"read"}}))).await?;
        if response.status().as_u16() != 201 {
            return Err(http_error(response.status().as_u16()));
        }
        let data = bounded_json(response).await?;
        let expires = data
            .get("expires_at")
            .and_then(Value::as_str)
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.timestamp())
            .ok_or(IntakeError::AccessDenied)?;
        let value = data
            .get("token")
            .and_then(Value::as_str)
            .filter(|s| {
                !s.is_empty() && s.len() <= 16384 && s.bytes().all(|b| b.is_ascii_graphic())
            })
            .ok_or(IntakeError::AccessDenied)?;
        let repos = data
            .get("repositories")
            .and_then(Value::as_array)
            .ok_or(IntakeError::AccessDenied)?;
        if expires - 60 <= now
            || expires > now + 3660
            || data.get("permissions") != Some(&json!({"issues":"write","metadata":"read"}))
            || repos.len() != 1
            || repos[0].get("id").and_then(Value::as_i64) != Some(binding.repository_id)
            || !repos[0]
                .get("full_name")
                .and_then(Value::as_str)
                .is_some_and(|s| s.eq_ignore_ascii_case(&binding.full_name))
        {
            return Err(IntakeError::AccessDenied);
        }
        let token = InstallationToken {
            value: value.into(),
            expires,
        };
        if cache.len() >= 16 {
            cache.clear();
        }
        cache.insert(key, token.clone());
        Ok(token)
    }

    pub(super) async fn check_labels(
        &self,
        binding: &RepositoryBinding,
        token: &InstallationToken,
        labels: &[String],
    ) -> Result<(), IntakeError> {
        if labels.is_empty() {
            return Ok(());
        }
        let mut found = std::collections::HashSet::new();
        for page in 1..=10 {
            let response = self
                .request(
                    Method::GET,
                    &format!(
                        "/repos/{}/labels?per_page=100&page={page}",
                        binding.full_name
                    ),
                    &token.value,
                    None,
                )
                .await?;
            if response.status().as_u16() != 200 {
                return Err(http_error(response.status().as_u16()));
            }
            let data = bounded_json(response).await?;
            let values = data.as_array().ok_or(IntakeError::UpstreamUnavailable)?;
            for label in values {
                if let Some(name) = label.get("name").and_then(Value::as_str) {
                    found.insert(name.to_lowercase());
                }
            }
            if labels.iter().all(|l| found.contains(&l.to_lowercase())) {
                return Ok(());
            }
            if values.len() < 100 {
                break;
            }
        }
        Err(IntakeError::InvalidPayload)
    }

    /// Private to the intake module: callable only after durable authorization
    /// reservation. Never available as a public generic HTTP/action client.
    pub(super) async fn create_issue(
        &self,
        binding: &RepositoryBinding,
        token: &InstallationToken,
        outgoing: &OutgoingIssue,
    ) -> SendOutcome {
        if token.expires <= Utc::now().timestamp() {
            return SendOutcome::Failed("access_denied", None);
        }
        let Ok(body) = serde_json::to_value(outgoing) else {
            return SendOutcome::Failed("invalid_payload", None);
        };
        let response = match self
            .request(
                Method::POST,
                &format!("/repos/{}/issues", binding.full_name),
                &token.value,
                Some(&body),
            )
            .await
        {
            Ok(r) => r,
            Err(_) => return SendOutcome::Unknown,
        };
        let status = response.status().as_u16();
        if status == 401 {
            self.clear_token_cache().await;
        }
        if matches!(status, 400 | 401 | 403 | 404 | 410 | 422 | 429) {
            return SendOutcome::Failed(
                match status {
                    401 | 403 => "access_denied",
                    404 | 410 => "repository_unavailable",
                    429 => "rate_limited",
                    _ => "rejected_payload",
                },
                retry_after(response.headers()),
            );
        }
        if status != 201 {
            return SendOutcome::Unknown;
        }
        match bounded_json(response)
            .await
            .ok()
            .and_then(|value| created(&value, outgoing))
        {
            Some(issue) => SendOutcome::Created(issue),
            None => SendOutcome::Unknown,
        }
    }

    /// Read all bounded pages, including closed issues; omit PRs. Ambiguous or
    /// incomplete searches never authorize a resend and never claim absence.
    pub(super) async fn reconcile(
        &self,
        binding: &RepositoryBinding,
        p: &PreparedIssue,
    ) -> Result<Option<CreatedIssue>, IntakeError> {
        let token = self.installation_token(binding).await?;
        let mut matches = std::collections::BTreeMap::new();
        for page in 1..=10 {
            let path = format!(
                "/repos/{}/issues?state=all&per_page=100&page={page}",
                binding.full_name
            );
            let response = self.request(Method::GET, &path, &token.value, None).await?;
            if response.status().as_u16() != 200 {
                return Err(http_error(response.status().as_u16()));
            }
            // No arbitrary Link URL is followed. Use the trusted endpoint and
            // bounded page number; a short page establishes scan completion.
            let data = bounded_json(response).await?;
            let issues = data.as_array().ok_or(IntakeError::UpstreamUnavailable)?;
            for value in issues {
                if value.get("pull_request").is_some() {
                    continue;
                }
                if value.get("title").and_then(Value::as_str) == Some(&p.outgoing.title)
                    && value.get("body").and_then(Value::as_str) == Some(&p.outgoing.body)
                {
                    if let Some(issue) = created(value, &p.outgoing) {
                        matches.insert(issue.id, issue);
                    }
                }
            }
            if issues.len() < 100 {
                return Ok(if matches.len() == 1 {
                    matches.into_values().next()
                } else {
                    None
                });
            }
        }
        Ok(None)
    }
}

fn http_error(status: u16) -> IntakeError {
    if matches!(status, 401 | 403 | 404) {
        IntakeError::AccessDenied
    } else {
        IntakeError::UpstreamUnavailable
    }
}
fn retry_after(headers: &HeaderMap) -> Option<i64> {
    let now = Utc::now().timestamp();
    headers
        .get("retry-after")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| {
            s.parse::<i64>()
                .ok()
                .map(|n| now.saturating_add(n.max(0)))
                .or_else(|| DateTime::parse_from_rfc2822(s).ok().map(|d| d.timestamp()))
        })
        .or_else(|| {
            (headers
                .get("x-ratelimit-remaining")
                .and_then(|h| h.to_str().ok())
                == Some("0"))
            .then(|| {
                headers
                    .get("x-ratelimit-reset")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse().ok())
            })
            .flatten()
        })
        .map(|t| t.max(now + 1))
}
async fn bounded_json(mut response: Response) -> Result<Value, IntakeError> {
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| IntakeError::UpstreamUnavailable)?
    {
        if bytes.len() + chunk.len() > MAX_RESPONSE {
            return Err(IntakeError::UpstreamUnavailable);
        }
        bytes.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&bytes).map_err(|_| IntakeError::UpstreamUnavailable)
}
fn created(value: &Value, expected: &OutgoingIssue) -> Option<CreatedIssue> {
    let id = value.get("id")?.as_i64()?;
    let number = value.get("number")?.as_i64()?;
    let html_url = value.get("html_url")?.as_str()?;
    let parsed = reqwest::Url::parse(html_url).ok()?;
    if id <= 0
        || number <= 0
        || parsed.scheme() != "https"
        || parsed.host_str() != Some("github.com")
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        return None;
    }
    let labels: Option<Vec<String>> = value
        .get("labels")?
        .as_array()?
        .iter()
        .map(|l| {
            l.as_str()
                .or_else(|| l.get("name").and_then(Value::as_str))
                .filter(|s| public_text(s, 100))
                .map(str::to_owned)
        })
        .collect();
    let labels = labels?;
    let set = |ls: &[String]| {
        ls.iter()
            .map(|s| s.to_lowercase())
            .collect::<std::collections::BTreeSet<_>>()
    };
    Some(CreatedIssue {
        id,
        number,
        html_url: html_url.into(),
        labels_match: set(&labels) == set(&expected.labels),
        labels,
    })
}

#[cfg(test)]
pub(super) mod tests;
