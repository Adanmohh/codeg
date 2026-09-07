//! Synthetic HTTP and RSA only. Never reads environment, keyring or gh auth.
use super::*;
use axum::{
    body::Bytes,
    http::{HeaderMap, Uri},
    response::IntoResponse,
    Router,
};
use serde::Deserialize;
use std::sync::{Arc, Mutex as StdMutex};

pub(crate) const PRIVATE_KEY: &str = include_str!("../fixtures/synthetic-only-private.pem");
const PUBLIC_KEY: &[u8] = include_bytes!("../fixtures/synthetic-only-public.pem");

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    Success,
    Lost,
    Reject,
    LabelsDropped,
    ServerError,
    Malformed,
    Redirect,
    BadExpiry,
    BroadPermission,
    WrongRepo,
    TokenDenied,
    WriteDenied,
    RateLimit,
}
pub(crate) struct Observed {
    pub token_requests: usize,
    pub issue_posts: usize,
    pub requests: Vec<(String, String)>,
    pub bodies: Vec<Value>,
    pub issues: Vec<Value>,
}
pub(crate) struct Fixture {
    pub client: GithubAppClient,
    pub observed: Arc<StdMutex<Observed>>,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.task.abort();
    }
}
#[derive(Deserialize)]
struct JwtClaims {
    iss: String,
    exp: i64,
    iat: i64,
}

impl Fixture {
    pub async fn new(mode: Mode) -> Self {
        let observed = Arc::new(StdMutex::new(Observed {
            token_requests: 0,
            issue_posts: 0,
            requests: vec![],
            bodies: vec![],
            issues: vec![],
        }));
        let state = observed.clone();
        let app = Router::new().fallback(move |method: Method, uri: Uri, headers: HeaderMap, bytes: Bytes| {
            let state = state.clone();
            async move {
                let body: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
                let (status, result) = {
                    let mut s = state.lock().unwrap();
                    s.requests.push((method.to_string(), uri.to_string()));
                    assert_eq!(headers.get("x-github-api-version").unwrap(), API_VERSION);
                    if uri.path().ends_with("/access_tokens") {
                        assert_eq!(method, Method::POST);
                        let bearer = headers.get("authorization").unwrap().to_str().unwrap().strip_prefix("Bearer ").unwrap();
                        let claims = jsonwebtoken::decode::<JwtClaims>(bearer, &jsonwebtoken::DecodingKey::from_rsa_pem(PUBLIC_KEY).unwrap(), &jsonwebtoken::Validation::new(Algorithm::RS256)).unwrap().claims;
                        assert_eq!(claims.iss, "fixture-app");
                        assert!((Utc::now().timestamp() - claims.iat - 60).abs() < 5);
                        assert_eq!(claims.exp - claims.iat, 600);
                        s.token_requests += 1;
                        assert_eq!(body["permissions"], json!({"issues":"write","metadata":"read"}));
                        assert_eq!(body["repository_ids"].as_array().unwrap().len(), 1);
                        let repo = body["repository_ids"][0].as_i64().unwrap();
                        (if mode == Mode::TokenDenied {403} else {201}, json!({"token":format!("synthetic-opaque-installation-token-{}", s.token_requests),
                            "expires_at":if mode == Mode::BadExpiry { "bad".into() } else { (Utc::now() + chrono::Duration::hours(1)).to_rfc3339() },
                            "permissions":if mode == Mode::BroadPermission { json!({"issues":"write","metadata":"read","contents":"write"}) } else { body["permissions"].clone() },
                            "repositories":[{"id":if mode == Mode::WrongRepo { repo + 1 } else { repo },"full_name":"owner/repo"}]}))
                    } else if uri.path().ends_with("/labels") {
                        assert_eq!(method, Method::GET);
                        (200, json!([{"name":"audio"},{"name":"severity:medium"}]))
                    } else if method == Method::POST && uri.path().ends_with("/issues") {
                        assert!(headers.get("authorization").unwrap().to_str().unwrap().starts_with("Bearer synthetic-opaque-"));
                        s.issue_posts += 1;
                        s.bodies.push(body.clone());
                        let issue = json!({"id":1000 + s.issue_posts,"number":s.issue_posts,"html_url":format!("https://github.com/owner/repo/issues/{}",s.issue_posts),
                            "title":body["title"],"body":body["body"],"labels":if mode == Mode::LabelsDropped { json!([]) } else { body["labels"].clone() }});
                        if !matches!(mode, Mode::Reject | Mode::WriteDenied | Mode::RateLimit) { s.issues.push(issue.clone()); }
                        match mode { Mode::Reject => (422,json!({"message":"PRIVATE UPSTREAM DETAIL"})),
                            Mode::ServerError => (503,json!({"message":"PRIVATE"})),
                            Mode::Malformed => (201,json!({"message":"PRIVATE"})),
                            Mode::Redirect => (307,json!({})), _ => (201,issue) }
                    } else {
                        assert_eq!(method, Method::GET);
                        assert!(uri.to_string().contains("state=all"));
                        (200, json!(s.issues))
                    }
                };
                if mode == Mode::Lost && method == Method::POST && uri.path().ends_with("/issues") {
                    // Side effect recorded above, then response arrives after the
                    // fixture-only client deadline. GET reconciliation still sees it.
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
                let status = if method == Method::POST && uri.path().ends_with("/issues") {
                    match mode { Mode::WriteDenied => 401, Mode::RateLimit => 429, _ => status }
                } else { status };
                let mut response = (axum::http::StatusCode::from_u16(status).unwrap(), axum::Json(result)).into_response();
                if status == 429 { response.headers_mut().insert("retry-after", "120".parse().unwrap()); }
                response
            }
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let api = format!("http://{}", listener.local_addr().unwrap());
        let task = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let mut client = GithubAppClient::new(Some(GithubAppConfig {
            app_id: "fixture-app".into(),
            private_key_pem: PRIVATE_KEY.into(),
        }))
        .unwrap();
        client.api = api;
        if mode == Mode::Lost {
            client.http = Client::builder()
                .timeout(Duration::from_millis(200))
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .retry(reqwest::retry::never())
                .build()
                .unwrap();
        }
        Self {
            client,
            observed,
            task,
        }
    }
}

pub(crate) fn binding() -> RepositoryBinding {
    RepositoryBinding {
        product_id: "hafidh".into(),
        folder_id: 1,
        app_id: "fixture-app".into(),
        installation_id: 123,
        repository_id: 456,
        full_name: "owner/repo".into(),
        enabled: true,
    }
}

#[tokio::test]
async fn rs256_and_cache_are_scoped_single_flight_and_expire_early() {
    let fixture = Fixture::new(Mode::Success).await;
    let b = binding();
    let (first, second) = tokio::join!(
        fixture.client.installation_token(&b),
        fixture.client.installation_token(&b)
    );
    assert_eq!(first.unwrap().value, second.unwrap().value);
    assert_eq!(fixture.observed.lock().unwrap().token_requests, 1);
    let mut other = b.clone();
    other.repository_id += 1;
    fixture.client.installation_token(&other).await.unwrap();
    other.installation_id += 1;
    fixture.client.installation_token(&other).await.unwrap();
    assert_eq!(fixture.observed.lock().unwrap().token_requests, 3);
    let key = fixture.client.key(&b).unwrap();
    fixture
        .client
        .cache
        .lock()
        .await
        .get_mut(&key)
        .unwrap()
        .expires = Utc::now().timestamp() + 59;
    fixture.client.installation_token(&b).await.unwrap();
    assert_eq!(fixture.observed.lock().unwrap().token_requests, 4);
    fixture.client.clear_token_cache().await;
    fixture.client.installation_token(&b).await.unwrap();
    assert_eq!(fixture.observed.lock().unwrap().token_requests, 5);
    other.app_id = "other-app".into();
    assert!(matches!(
        fixture.client.installation_token(&other).await,
        Err(IntakeError::AccessDenied)
    ));
    let unconfigured = GithubAppClient::new(None).unwrap();
    assert!(!unconfigured.is_configured());
    assert!(matches!(
        unconfigured.installation_token(&b).await,
        Err(IntakeError::NotConfigured)
    ));
}

#[tokio::test]
async fn malformed_expiry_or_excess_scope_never_enters_cache() {
    for mode in [Mode::BadExpiry, Mode::BroadPermission, Mode::WrongRepo] {
        let fixture = Fixture::new(mode).await;
        assert!(matches!(
            fixture.client.installation_token(&binding()).await,
            Err(IntakeError::AccessDenied)
        ));
        assert!(fixture.client.cache.lock().await.is_empty());
        assert_eq!(fixture.observed.lock().unwrap().issue_posts, 0);
    }
}

#[test]
fn retry_headers_and_invalid_keys_are_safe() {
    let mut headers = HeaderMap::new();
    headers.insert("retry-after", "30".parse().unwrap());
    assert!(retry_after(&headers).unwrap() >= Utc::now().timestamp() + 29);
    headers.clear();
    headers.insert("x-ratelimit-remaining", "0".parse().unwrap());
    headers.insert(
        "x-ratelimit-reset",
        (Utc::now().timestamp() + 40).to_string().parse().unwrap(),
    );
    assert!(retry_after(&headers).unwrap() > Utc::now().timestamp());
    let error = GithubAppClient::new(Some(GithubAppConfig {
        app_id: "fixture-app".into(),
        private_key_pem: "PRIVATE-INVALID-KEY".into(),
    }))
    .err()
    .unwrap();
    assert_eq!(error.to_string(), "not_configured");
}
