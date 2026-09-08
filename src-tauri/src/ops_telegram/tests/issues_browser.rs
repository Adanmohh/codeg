//! Manual protected phone fixture, isolated 4323/out-telegram-issues only.
//! Reuses accepted host/provider setup, no scheduler/agent loop or live secrets.
use super::*;
use crate::ops_intake_host::{tests as fixtures, types as h};
use axum::{
    http::HeaderMap,
    response::Html,
    routing::{get, post},
    Json,
};

const TOKEN: &str = "ops-issue-phone-synthetic-operator";

#[tokio::test]
#[ignore = "manual synthetic issue phone fixture on 4323; requires own out-telegram-issues export"]
async fn ops_telegram_issue_browser_fixture() {
    let port = std::env::var("CODEG_DESIGN_FIXTURE_PORT")
        .map(|value| value.parse::<u16>().expect("fixture port must be u16"))
        .unwrap_or(4323);
    let export = std::env::var("CODEG_DESIGN_FIXTURE_EXPORT")
        .unwrap_or_else(|_| "out-telegram-issues".into());
    let github = fixtures::fixture::Provider::start().await;
    let (db, source, ctx, d) = fixtures::ready(&github).await;
    let first = issues::proposed(&db, &ctx, &d, &github.runtime)
        .await
        .proposal_id
        .unwrap();
    let mut cases = vec![("Created path", first)];
    for (name, ulid) in [
        ("Unknown path", "01ARZ3NDEKTSV4RRFFQ69G5FAW"),
        ("Rejected path", "01ARZ3NDEKTSV4RRFFQ69G5FAX"),
    ] {
        let source = h::SourceInput {
            product_id: source.product_id.clone(),
            ulid: ulid.into(),
        };
        let (ctx, d) = fixtures::ready_source(&db, &github, &source).await;
        let p = issues::proposed(&db, &ctx, &d, &github.runtime).await;
        cases.push((name, p.proposal_id.unwrap()));
    }
    let telegram = Provider::new().await;
    issues::opted(&db, &telegram).await;
    let cfg = status(&db.conn, &operator(1), &telegram.runtime)
        .await
        .unwrap()
        .configuration
        .unwrap();
    configure(
        &db.conn,
        &operator(1),
        &telegram.runtime,
        ConfigureInput {
            channel_id: cfg.channel_id,
            private_user_id: cfg.private_user_id,
            review_origin: format!("http://127.0.0.1:{port}"),
            enabled: true,
            github_issues_enabled: true,
            expected_revision: Some(cfg.revision),
        },
    )
    .await
    .unwrap();
    notify_now(&db.conn, &operator(1), &telegram.runtime)
        .await
        .unwrap();
    assert_eq!(telegram.sends(), 3);
    assert_eq!(github.seen.lock().unwrap().posts, 0);
    let mut links = String::new();
    for (name, id) in cases {
        let row = notice::Entity::find()
            .filter(notice::Column::ProposalId.eq(id))
            .one(&db.conn)
            .await
            .unwrap()
            .unwrap();
        links.push_str(&format!(
            "<li><a href='/ops-review?notice={}'>{name}</a> · proposal {id}</li>",
            row.id
        ));
        println!(
            "{name}: http://127.0.0.1:{port}/ops-review?notice={}",
            row.id
        );
    }
    let dir = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let static_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(export);
    assert!(static_dir.join("ops-review.html").is_file());
    let landing=format!("<!doctype html><html lang='en'><meta name='viewport' content='width=device-width, initial-scale=1'><title>Synthetic issue phone fixture</title><main><h1>Synthetic issue phone fixture</h1><p>Loopback providers only. Sign in through the actual protected review page.</p><ul>{links}</ul><p>Operator token for this test-only server: {TOKEN}</p><p>Freshness expires after 15 minutes. The protected fixture refresh endpoint renews only synthetic timestamps.</p></main></html>");
    let seen = github.seen.clone();
    let messages = telegram.state.seen.clone();
    let refresh_state = state.clone();
    let router=crate::web::router::build_router(state,TOKEN.into(),static_dir,Arc::new(crate::web::shutdown::ShutdownSignal::new()))
        .layer(axum::Extension(github.runtime.clone()))
        .layer(axum::Extension(telegram.runtime.clone()))
        .route("/__issue_fixture",get(move || {let landing=landing.clone();async move {Html(landing)}}))
        .route("/__issue_fixture/stats",get(move |headers:HeaderMap| {
            let seen=seen.clone();let messages=messages.clone();
            async move {
                if !headers.get("authorization").is_some_and(|h|h==format!("Bearer {TOKEN}").as_str()) { return Err(StatusCode::UNAUTHORIZED); }
                let seen=seen.lock().unwrap();
                let telegram_sends=messages.lock().unwrap().iter().filter(|(m,_)|m=="/sendMessage").count();
                Ok(Json(json!({"githubPosts":seen.posts,"githubTokens":seen.tokens,"githubIssues":seen.issues.len(),"telegramSends":telegram_sends})))
            }
        }))
        .route("/__issue_fixture/refresh",post(move |headers:HeaderMap| {
            let state=refresh_state.clone();
            async move {
                if !headers.get("authorization").is_some_and(|h|h==format!("Bearer {TOKEN}").as_str()) {return StatusCode::UNAUTHORIZED;}
                // Explicit test control, not production reconciliation: no
                // payload/status/config/claim/receipt or provider request changes.
                let at=Utc::now().timestamp();
                state.db.conn.execute(Statement::from_sql_and_values(state.db.conn.get_database_backend(),"UPDATE ops_intake_source SET fetched_at=?",[at.into()])).await.unwrap();
                state.db.conn.execute(Statement::from_sql_and_values(state.db.conn.get_database_backend(),"UPDATE ops_intake_host_snapshot SET verified_at=? WHERE error IS NULL",[at.into()])).await.unwrap();
                StatusCode::NO_CONTENT
            }
        }));
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    println!("Synthetic issue phone fixture: http://127.0.0.1:{port}/__issue_fixture ; PID={} ; token={TOKEN} ; SQLite=in-memory ; auxiliary data={}",std::process::id(),dir.path().display());
    axum::serve(listener, router).await.unwrap();
}
