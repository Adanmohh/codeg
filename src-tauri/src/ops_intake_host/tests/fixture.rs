//! Synthetic loopback-only upstream. Uses the accepted client/stdio path.
use super::*;
use axum::{
    body::Bytes,
    http::{HeaderMap, Method, StatusCode, Uri},
    Json, Router,
};
use std::sync::{Arc, Mutex};

pub(super) const TOKEN: &str = "ops-intake-synthetic-operator";
const KEY: &str = include_str!("../../ops_intake/fixtures/synthetic-only-private.pem");
#[derive(Default)]
pub(super) struct Observed {
    pub reads: usize,
    pub posts: usize,
    pub tokens: usize,
    pub denied: bool,
    pub changed: bool,
    pub issues: Vec<serde_json::Value>,
}
pub(super) struct Provider {
    pub runtime: Arc<HostRuntime>,
    pub origin: String,
    pub seen: Arc<Mutex<Observed>>,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Provider {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl Provider {
    pub async fn start() -> Self {
        let seen = Arc::new(Mutex::new(Observed::default()));
        let state = seen.clone();
        let router = Router::new().fallback(move |method: Method, uri: Uri, headers: HeaderMap, bytes: Bytes| {
            let state = state.clone();
            async move {
                let mut seen = state.lock().unwrap();
                if uri.path() == "/api/v1/admin/testflight" {
                    assert_eq!(method, Method::GET);
                    assert!(headers.get("authorization").is_some_and(|h| h == "Bearer synthetic-intake-only"));
                    seen.reads += 1;
                    if seen.denied { return (StatusCode::FORBIDDEN, Json(json!({"message":"PRIVATE NEVER ECHO"}))); }
                    let items: Vec<_> = [
                        ("01ARZ3NDEKTSV4RRFFQ69G5FAV", "Synthetic audio stops at verse four"),
                        ("01ARZ3NDEKTSV4RRFFQ69G5FAW", "Synthetic unknown response example"),
                        ("01ARZ3NDEKTSV4RRFFQ69G5FAX", "Synthetic rejected label example"),
                    ].into_iter().map(|(ulid,title)| json!({"ulid":ulid,"ascSubmissionId":format!("asc-{ulid}"),
                        "comment":if seen.changed {format!("{title}. Updated upstream report")} else {title.into()},
                        "testerEmail":"tester@example.invalid","testerName":"PRIVATE TESTER","notes":"PRIVATE NOTES",
                        "deviceModel":"iPhone 16","osVersion":"18.2","locale":"en","buildVersion":"42","appPlatform":"IOS",
                        "submittedAt":"2026-09-07T10:00:00Z","status":"pending","severity":"medium","tags":["audio"],"screenshots":[],
                        "lastModifiedAt":"2026-09-07T10:00:00Z","lastModifiedByName":"PRIVATE OPERATOR"})).collect();
                    return (StatusCode::OK, Json(json!({"succeeded":true,"hasErrors":false,"value":{
                        "items":items,"total":3,"open":3,"resolved":0,"highSeverityOpen":0,"tagCounts":{}}})));
                }
                let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or_default();
                if uri.path() == "/app/installations/7/access_tokens" {
                    assert_eq!(method, Method::POST);
                    assert_eq!(body,json!({"repository_ids":[11],"permissions":{"issues":"write","metadata":"read"}}));
                    seen.tokens += 1;
                    return (StatusCode::CREATED, Json(json!({"token":"synthetic-installation-only", "expires_at":(chrono::Utc::now()+chrono::Duration::hours(1)).to_rfc3339(),
                        "permissions":{"issues":"write","metadata":"read"},"repositories":[{"id":11,"full_name":"owner/repo"}]})));
                }
                assert!(headers.get("authorization").is_some_and(|h| h == "Bearer synthetic-installation-only"));
                if uri.path() == "/repos/owner/repo/labels" {
                    assert_eq!(method, Method::GET);
                    return (StatusCode::OK, Json(json!([{"name":"audio"},{"name":"severity:medium"}])));
                }
                assert_eq!(uri.path(), "/repos/owner/repo/issues");
                if method == Method::GET { return (StatusCode::OK, Json(json!(seen.issues))); }
                assert_eq!(method, Method::POST);
                seen.posts += 1;
                if body["title"].as_str().unwrap().contains("rejected") {
                    return (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({"message":"PRIVATE REJECTION"})));
                }
                let issue = json!({"id":1000+seen.posts,"number":seen.posts,"html_url":format!("https://github.com/owner/repo/issues/{}",seen.posts),
                    "title":body["title"],"body":body["body"],"labels":body["labels"]});
                seen.issues.push(issue.clone());
                if body["title"].as_str().unwrap().contains("unknown") {
                    // Provider commits, then no usable create response reaches the client.
                    return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({"message":"PRIVATE RESPONSE LOST"})));
                }
                (StatusCode::CREATED, Json(issue))
            }
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        Self {
            runtime: HostRuntime::synthetic(Some(address)),
            origin: format!("http://{address}"),
            seen,
            task,
        }
    }
    pub async fn configure(&self, db: &crate::db::AppDatabase, source: &SourceInput) {
        let p = product(&db.conn, 1, &source.product_id).await.unwrap();
        operator::configure(
            &db.conn,
            &human(),
            &self.runtime,
            ConfigureInput {
                binding: p.binding,
                origin: self.origin.clone(),
                intake_bearer: Some("synthetic-intake-only".into()),
                app_private_key: Some(KEY.into()),
            },
        )
        .await
        .unwrap();
    }
}
