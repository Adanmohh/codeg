//! Ignored manual loopback fixture. Production identity core/router, synthetic
//! in-memory DB, no engine/scheduler/provider and no old static export mutation.
use super::*;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::get,
};

const OPERATOR: &str = "business-identity-synthetic-operator";

#[tokio::test]
#[ignore = "manual guarded identity API fixture on loopback4341; no task/UI acceptance"]
async fn business_identity_browser_fixture() {
    let db = fresh_in_memory_db().await;
    let directory = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        directory.path().into(),
    ));
    let router = crate::web::router::build_router(state, OPERATOR.into(), directory.path().into(), Arc::new(crate::web::shutdown::ShutdownSignal::new()))
        .route("/__business_fixture", get(|| async { Html("<!doctype html><html lang='en'><meta name='viewport' content='width=device-width,initial-scale=1'><title>Synthetic identity API fixture</title><main><h1>Synthetic identity API fixture</h1><p>Production protected identity endpoints; in-memory data only. This page is test scaffolding, not the business workspace UI.</p><p>Only the business API and harmless health route are reachable. Engineering, configuration, provider and agent operations are blocked.</p></main></html>") }))
        .layer(middleware::from_fn(|request: Request, next: Next| async move {
            // Defense in depth around this manual fixture. Legacy member denial
            // through the unmodified full router is tested separately above.
            let path = request.uri().path();
            if path.starts_with("/api/business/") || path == "/api/health" || path == "/__business_fixture" {
                next.run(request).await
            } else {
                let response: Response = (StatusCode::FORBIDDEN, "Synthetic fixture guard").into_response();
                response
            }
        }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4341")
        .await
        .unwrap();
    println!("Identity fixture: http://127.0.0.1:4341/__business_fixture ; PID={} ; SQLite=in-memory ; test-only operator constant in fixture source ; no old export touched", std::process::id());
    axum::serve(listener, router).await.unwrap();
}
