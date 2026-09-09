//! Manual synthetic-only unified tenant fixture. Reuses the Apache-2.0 protected
//! task fixture/router, accepted tenant provisioning and local injected reader.
//! Fixed query shapes mirror fireflies.rs (MIT upstream mapping in NOTICE).
//! No engine, external provider, native keyring, legacy settings or static export.
use super::super::{fireflies::Reader, services::Services};
use crate::{app_state::AppState, db::test_helpers::fresh_disk_db, ops::email::SecretStore};
use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio_util::sync::CancellationToken;

const NAMESPACES: [&str; 4] = ["ui", "root", "review", "worker"];
const BROWSER_ORIGINS: [&str; 3] = [
    "http://127.0.0.1:4354",
    "http://localhost:4354",
    "http://127.0.0.1:4351",
];
const RECORDS: [&str; 3] = ["meeting-follow-up", "meeting-empty", "meeting-long"];
const USER: &str = "query ValidateCredentials { user { user_id } }";
const LIST: &str = "query GetTranscriptsList($fromDate: DateTime, $toDate: DateTime, $limit: Int, $skip: Int, $userId: String, $mine: Boolean) { transcripts(fromDate: $fromDate, toDate: $toDate, limit: $limit, skip: $skip, user_id: $userId, mine: $mine) { id title dateString } }";
const DETAIL: &str = "query Transcript($transcriptId: String!) { transcript(id: $transcriptId) { id title dateString user { user_id } privacy shared_with { expires_at } sentences { index text start_time end_time } summary { action_items } meeting_info { summary_status } } }";

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Scenario {
    #[default]
    Fresh,
    Changed,
    Denied,
    Missing,
}
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct Counts {
    user: u64,
    list: u64,
    detail: u64,
    upstream_rejected: u64,
    api_allowed: u64,
    api_blocked: u64,
    secret_rejected: u64,
}
#[derive(Default)]
struct MemoryStore {
    values: Mutex<BTreeMap<String, String>>,
    counts: Arc<Mutex<Counts>>,
}
fn source_key(namespace: &str) -> String {
    format!("synthetic-intake-{namespace}")
}
impl SecretStore for MemoryStore {
    fn get(&self, reference: &str) -> Option<String> {
        self.values.lock().unwrap().get(reference).cloned()
    }
    fn set(&self, reference: &str, value: &str) -> Result<(), ()> {
        // Rejected before any network request; no fallback to ExistingStore.
        if !reference.starts_with("business-intake:")
            || !NAMESPACES.iter().any(|name| value == source_key(name))
        {
            self.counts.lock().unwrap().secret_rejected += 1;
            return Err(());
        }
        self.values
            .lock()
            .unwrap()
            .insert(reference.into(), value.into());
        Ok(())
    }
    fn delete(&self, reference: &str) -> Result<(), ()> {
        self.values.lock().unwrap().remove(reference);
        Ok(())
    }
}
#[derive(Clone)]
struct Harness {
    counts: Arc<Mutex<Counts>>,
    scenarios: Arc<Mutex<BTreeMap<String, Scenario>>>,
    controls: Arc<BTreeMap<String, String>>,
    stop: CancellationToken,
    stop_key: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    query: String,
    variables: Value,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Control {
    namespace: String,
    scenario: Scenario,
}
fn bearer(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}
fn response(status: StatusCode, value: Value) -> Response {
    (status, [(header::CACHE_CONTROL, "no-store")], Json(value)).into_response()
}
fn rejected(h: &Harness) -> Response {
    h.counts.lock().unwrap().upstream_rejected += 1;
    response(
        StatusCode::FORBIDDEN,
        json!({"error":"Synthetic upstream guard"}),
    )
}
async fn graphql(
    State(h): State<Harness>,
    headers: HeaderMap,
    Json(input): Json<Query>,
) -> Response {
    let Some(namespace) = NAMESPACES
        .iter()
        .find(|name| bearer(&headers) == Some(source_key(name).as_str()))
    else {
        return rejected(&h);
    };
    let provider_user = format!("synthetic-owner-{namespace}");
    if input.query == USER && input.variables == json!({}) {
        h.counts.lock().unwrap().user += 1;
        return response(
            StatusCode::OK,
            json!({"data":{"user":{"user_id":provider_user}}}),
        );
    }
    let scenario = *h.scenarios.lock().unwrap().get(*namespace).unwrap();
    if input.query == LIST {
        let v = &input.variables;
        if v.as_object().is_none_or(|v| v.len() != 6)
            || v["userId"] != provider_user
            || v["mine"] != true
            || v["limit"] != 50
            || !v["skip"]
                .as_i64()
                .is_some_and(|n| (0..250).contains(&n) && n % 50 == 0)
            || !v["fromDate"].is_string()
            || !v["toDate"].is_string()
        {
            return rejected(&h);
        }
        h.counts.lock().unwrap().list += 1;
        let items = if v["skip"] == 0 {
            RECORDS.iter().map(|id| json!({"id":id,"title":format!("{namespace}: {id}"),"dateString":"2026-09-08T10:00:00Z"})).collect::<Vec<_>>()
        } else {
            vec![]
        };
        return response(StatusCode::OK, json!({"data":{"transcripts":items}}));
    }
    if input.query != DETAIL || input.variables.as_object().is_none_or(|v| v.len() != 1) {
        return rejected(&h);
    }
    let Some(id) = input.variables["transcriptId"]
        .as_str()
        .filter(|id| RECORDS.contains(id))
    else {
        return rejected(&h);
    };
    h.counts.lock().unwrap().detail += 1;
    if matches!(scenario, Scenario::Denied) {
        return response(
            StatusCode::FORBIDDEN,
            json!({"error":"Synthetic source denied"}),
        );
    }
    if matches!(scenario, Scenario::Missing) {
        return response(StatusCode::OK, json!({"data":{"transcript":null}}));
    }
    let changed = matches!(scenario, Scenario::Changed);
    let text = if id == "meeting-long" {
        format!("{namespace} private evidence. {}", "Review the customer request in its original context; publish only the text you have deliberately reviewed. ".repeat(50))
    } else if changed {
        format!("{namespace} private evidence revision two: the customer now requests a written response before the meeting.")
    } else {
        format!("{namespace} private evidence revision one: the customer requests an onboarding follow-up. This transcript is not automatically public task text.")
    };
    let sentences = if id == "meeting-empty" {
        vec![]
    } else {
        vec![
            json!({"index":1,"text":text,"start_time":1.0,"end_time":8.0}),
            json!({"index":2,"text":format!("{namespace} second private passage: confirm responsibility with the human owner."),"start_time":8.0,"end_time":14.0}),
        ]
    };
    let summary = if id == "meeting-empty" {
        Value::Null
    } else {
        json!({"action_items":[format!("{namespace}: Review a customer follow-up.")]})
    };
    response(
        StatusCode::OK,
        json!({"data":{"transcript":{
            "id":id,"title":format!("{namespace}: {id}{}",if changed { " updated" } else { "" }),
            "dateString":"2026-09-08T10:00:00Z","user":{"user_id":provider_user},"privacy":"private","shared_with":[],
            "sentences":sentences,"summary":summary,"meeting_info":{"summary_status":if id=="meeting-empty" { "processing" } else { "complete" }}
        }}}),
    )
}
async fn control(
    State(h): State<Harness>,
    headers: HeaderMap,
    Json(input): Json<Control>,
) -> Response {
    if h.controls.get(&input.namespace).map(String::as_str) != bearer(&headers)
        || !NAMESPACES.contains(&input.namespace.as_str())
    {
        return response(
            StatusCode::FORBIDDEN,
            json!({"error":"Synthetic control denied"}),
        );
    }
    h.scenarios
        .lock()
        .unwrap()
        .insert(input.namespace.clone(), input.scenario);
    response(
        StatusCode::OK,
        json!({"namespace":input.namespace,"scenario":input.scenario,"effect":"Future synthetic detail responses only; no database or freshness mutation"}),
    )
}
async fn health(State(h): State<Harness>) -> Response {
    response(
        StatusCode::OK,
        json!({
            "synthetic":true,"pid":std::process::id(),"backendPort":4351,"upstreamPort":4352,
        "namespaceNames":NAMESPACES,"browserOrigins":BROWSER_ORIGINS,"counts":*h.counts.lock().unwrap(),
            "scenarios":*h.scenarios.lock().unwrap(),"tenantWindowAvailable":false,
            "engineStarted":false,"secretStore":"injected-memory-only","readerEndpoint":"http://127.0.0.1:4352/graphql"
        }),
    )
}
async fn stop(State(h): State<Harness>, headers: HeaderMap) -> Response {
    if bearer(&headers) != Some(h.stop_key.as_str()) {
        return response(StatusCode::FORBIDDEN, json!({"error":"Fixture owner only"}));
    }
    h.stop.cancel();
    response(StatusCode::OK, json!({"stopping":true}))
}
fn allowed_api(path: &str) -> bool {
    let path = path.strip_prefix("/api/business/").unwrap_or("");
    matches!(
        path,
        "context"
            | "members/list"
            | "members/create"
            | "members/update"
            | "members/revoke"
            | "credentials/issue"
            | "credentials/list"
            | "credentials/revoke"
            | "settings/get"
            | "settings/update"
            | "tasks/list"
            | "tasks/get"
            | "tasks/create"
            | "tasks/update"
            | "tasks/assign"
            | "tasks/progress"
            | "tasks/note"
            | "tasks/submit"
            | "tasks/review"
            | "tasks/cancel"
            | "tasks/archive"
            | "intake/bindings/list"
            | "intake/bindings/status"
            | "intake/bindings/create"
            | "intake/bindings/update"
            | "intake/bindings/disable"
            | "intake/grants/list"
            | "intake/grants/upsert"
            | "intake/grants/revoke"
            | "intake/sources/list"
            | "intake/sources/get"
            | "intake/candidates/list"
            | "intake/candidates/get"
            | "intake/candidates/create"
            | "intake/candidates/select"
            | "intake/candidates/edit"
            | "intake/candidates/accept"
            | "intake/candidates/link"
            | "intake/candidates/discard"
            | "intake/tasks/sources"
            | "intake/imports/start"
            | "intake/imports/capture"
            | "intake/imports/list"
            | "intake/imports/get"
            | "intake/imports/advance"
            | "intake/imports/cancel"
    )
}
async fn guard(h: Harness, request: Request, next: Next) -> Response {
    let path = request.uri().path();
    let permitted = (allowed_api(path)
        || matches!(
            path,
            "/api/platform/business/context"
                | "/api/platform/business/tenants/list"
                | "/api/platform/business/tenants/create"
                | "/api/platform/business/tenants/status"
                | "/api/platform/business/tenants/reissue-owner-credential"
                | "/__business_intake_fixture/control"
                | "/__business_intake_fixture/stop"
        ))
        && matches!(*request.method(), Method::POST | Method::OPTIONS)
        || path == "/__business_intake_fixture/health" && request.method() == Method::GET;
    let origin_allowed = request.headers().get(header::ORIGIN).is_none_or(|value| {
        value
            .to_str()
            .ok()
            .is_some_and(|value| BROWSER_ORIGINS.contains(&value))
    });
    if !permitted || !origin_allowed {
        h.counts.lock().unwrap().api_blocked += 1;
        return response(
            StatusCode::FORBIDDEN,
            json!({"error":"Synthetic fixture guard"}),
        );
    }
    h.counts.lock().unwrap().api_allowed += 1;
    next.run(request).await
}
fn write_private(path: &Path, value: &Value) {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path).unwrap();
    file.write_all(serde_json::to_string_pretty(value).unwrap().as_bytes())
        .unwrap();
    file.sync_all().unwrap();
}
async fn call(server: &axum_test::TestServer, path: &str, token: &str, input: Value) -> Value {
    let result = server
        .post(path)
        .add_header("authorization", format!("Bearer {token}"))
        .json(&json!({"input":input}))
        .await;
    assert_eq!(
        result.status_code(),
        StatusCode::OK,
        "Synthetic seeding failed at {path}; body withheld"
    );
    result.json()
}

#[tokio::test]
#[ignore = "manual unified synthetic B fixture4351/4352; owner coordination required"]
async fn intake_browser_fixture() {
    // Bind both reserved addresses first: occupied ports fail without touching their owners.
    let backend = tokio::net::TcpListener::bind("127.0.0.1:4351")
        .await
        .unwrap();
    let upstream = tokio::net::TcpListener::bind("127.0.0.1:4352")
        .await
        .unwrap();
    let parent = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(".docs/business-intake-fixtures");
    std::fs::create_dir_all(&parent).unwrap();
    let directory = tempfile::Builder::new()
        .prefix("unified-")
        .tempdir_in(&parent)
        .unwrap()
        .keep();
    let db = fresh_disk_db(&directory).await;
    let app = Arc::new(AppState::new_for_test(db, directory.clone()));
    let counts = Arc::new(Mutex::new(Counts::default()));
    let platform_token = format!("synthetic-platform-{}", uuid::Uuid::new_v4());
    let harness = Harness {
        counts: counts.clone(),
        scenarios: Arc::new(Mutex::new(
            NAMESPACES
                .iter()
                .map(|n| (n.to_string(), Scenario::Fresh))
                .collect(),
        )),
        controls: Arc::new(
            NAMESPACES
                .iter()
                .map(|n| {
                    (
                        n.to_string(),
                        format!("synthetic-control-{}", uuid::Uuid::new_v4()),
                    )
                })
                .collect(),
        ),
        stop: CancellationToken::new(),
        stop_key: format!("synthetic-stop-{}", uuid::Uuid::new_v4()),
    };
    let services = Services::fixture(
        Reader::fixture(upstream.local_addr().unwrap(), Duration::from_secs(12)).unwrap(),
        Arc::new(MemoryStore {
            values: Mutex::default(),
            counts,
        }),
    );
    let upstream_router = Router::new()
        .route("/graphql", post(graphql))
        .with_state(harness.clone());
    let upstream_stop = harness.stop.clone();
    let upstream_task = tokio::spawn(async move {
        axum::serve(upstream, upstream_router)
            .with_graceful_shutdown(upstream_stop.cancelled_owned())
            .await
            .unwrap();
    });
    let fixture_routes = Router::new()
        .route("/__business_intake_fixture/health", get(health))
        .route("/__business_intake_fixture/control", post(control))
        .route("/__business_intake_fixture/stop", post(stop))
        .with_state(harness.clone());
    let guard_harness = harness.clone();
    let router = crate::web::router::build_router(
        app.clone(),
        platform_token.clone(),
        directory.clone(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    )
    .merge(fixture_routes)
    .layer(Extension(services))
    .layer(middleware::from_fn(move |req, next| {
        guard(guard_harness.clone(), req, next)
    }));
    let seed_server = axum_test::TestServer::new(router.clone()).unwrap();
    let mut namespace_metadata = BTreeMap::new();
    for namespace in NAMESPACES {
        let owner = call(&seed_server,"/api/platform/business/tenants/create",&platform_token,json!({"operationId":uuid::Uuid::new_v4().to_string(),"organizationName":format!("Intake {namespace} synthetic"),"ownerName":format!("{namespace} owner")})).await;
        let token = owner["token"].as_str().unwrap();
        let mut sessions = BTreeMap::from([(
            "owner",
            json!({"memberId":owner["ownerMemberId"],"token":token,"credential":owner["credential"]}),
        )]);
        for role in ["manager", "viewer"] {
            let member = call(&seed_server,"/api/business/members/create",token,json!({"organizationId":owner["organization"]["id"],"displayName":format!("{namespace} {role}"),"kind":"human","role":role,"domains":["feedback"]})).await;
            let issued = call(&seed_server,"/api/business/credentials/issue",token,json!({"organizationId":owner["organization"]["id"],"memberId":member["id"],"label":"Synthetic fixture only"})).await;
            sessions.insert(role,json!({"memberId":member["id"],"token":issued["token"],"credential":issued["credential"]}));
        }
        let target = call(&seed_server,"/api/business/tasks/create",token,json!({"title":format!("{namespace}: existing reviewed customer follow-up"),"notes":"Deliberately reviewed shared task text; source transcript remains private.","domain":"feedback","dueDate":"2026-11-01"})).await;
        let credentials = json!({"synthetic":true,"namespace":namespace,"organization":owner["organization"],"sessions":sessions,"firefliesApiKey":source_key(namespace),"controlToken":harness.controls[namespace],"linkTarget":target["task"]["id"]});
        write_private(
            &directory.join(format!("{namespace}-credentials.json")),
            &credentials,
        );
        namespace_metadata.insert(namespace,json!({"organizationId":owner["organization"]["id"],"ownerMemberId":owner["ownerMemberId"],"linkTarget":target["task"]["id"],"credentialFile":format!("{namespace}-credentials.json")}));
    }
    drop(seed_server);
    assert_eq!(
        super::support::count(&app.db.conn, "business_intake_binding").await,
        0
    );
    assert_eq!(
        super::support::count(&app.db.conn, "business_intake_grant").await,
        0
    );
    {
        let initial = harness.counts.lock().unwrap();
        assert_eq!(initial.user + initial.list + initial.detail, 0);
    }
    write_private(
        &directory.join("owner-controls.json"),
        &json!({"synthetic":true,"platformToken":platform_token,"stopToken":harness.stop_key}),
    );
    let metadata = json!({"synthetic":true,"pid":std::process::id(),"backend":"http://127.0.0.1:4351","upstream":"http://127.0.0.1:4352/graphql","package":env!("CARGO_PKG_VERSION"),"binary":std::env::current_exe().unwrap(),"fixtureSourceSha256":format!("{:x}",Sha256::digest(include_bytes!("browser.rs"))),"directory":directory,"database":"source.db","namespaces":namespace_metadata,"initialSourceBindings":0,"initialSourceGrants":0,"initialProviderReads":0,"tenantWindowAvailable":false});
    std::fs::write(
        directory.join("metadata.json"),
        serde_json::to_string_pretty(&metadata).unwrap(),
    )
    .unwrap();
    println!("Synthetic unified intake fixture ready; PID={}; metadata={}; backend4351/upstream4352; no engine",std::process::id(),directory.join("metadata.json").display());
    axum::serve(backend, router)
        .with_graceful_shutdown(harness.stop.clone().cancelled_owned())
        .await
        .unwrap();
    upstream_task.await.unwrap();
    std::fs::write(
        directory.join("final-counts.json"),
        serde_json::to_string_pretty(&*harness.counts.lock().unwrap()).unwrap(),
    )
    .unwrap();
    println!(
        "Synthetic unified intake fixture stopped; data retained at {}",
        directory.display()
    );
}
