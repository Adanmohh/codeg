//! Manual CLI fixture only; absent from every shipped binary. No inference,
//! production credentials, provider origins or task engine execution.
use super::*;
use axum::{
    http::{HeaderMap, StatusCode},
    routing::post,
};
use std::{path::PathBuf, sync::Arc};

#[tokio::test]
#[ignore = "manual Playwright CLI fixture, own loopback port 4322"]
async fn intake_host_browser_fixture() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../.build/intake-host")
        .join(format!("browser-{}", uuid::Uuid::new_v4()));
    let project = dir.join("synthetic-project");
    std::fs::create_dir_all(&project).unwrap();
    let db = crate::db::test_helpers::fresh_disk_db(&dir).await;
    let (db, source) = seed_into(db, project.to_str().unwrap()).await;
    let provider = fixture::Provider::start().await;
    let empty = std::env::var("OPS_INTAKE_FIXTURE_EMPTY").as_deref() == Ok("1");
    if empty {
        db.conn
            .execute_unprepared("DELETE FROM ops_intake_host_product")
            .await
            .unwrap();
    } else {
        provider.configure(&db, &source).await;
        let p = product(&db.conn, 1, &source.product_id).await.unwrap();
        start_task(&db, p.binding.folder_id).await;
        let mut missing = p.binding;
        missing.product_id = "synthetic-unconfigured".into();
        operator::configure(
            &db.conn,
            &human(),
            &provider.runtime,
            ConfigureInput {
                binding: missing,
                origin: "https://synthetic.invalid".into(),
                intake_bearer: None,
                app_private_key: None,
            },
        )
        .await
        .unwrap();
    }
    let state = Arc::new(crate::app_state::AppState::new_for_test(db, dir.clone()));
    let actor_state = state.clone();
    let runtime = provider.runtime.clone();
    // A clearly synthetic parent-owned run exercises the actual pi seam when
    // the human has prepared a draft. It never attaches, confirms or approves.
    let proposer = tokio::spawn(async move {
        loop {
            let rows=actor_state.db.conn.query_all(sql(
                "SELECT d.id,d.revision,json_extract(d.draft_json,'$.prepared.draft.task_id') AS task_id,json_extract(d.draft_json,'$.prepared.draft.run_seq') AS run_seq FROM ops_intake_host_draft d WHERE json_type(d.draft_json,'$.prepared')='object'",
                vec![])).await.unwrap();
            for row in rows {
                let ctx = RunContext {
                    account_id: 1,
                    task_id: row.try_get("", "task_id").unwrap(),
                    run_seq: row.try_get("", "run_seq").unwrap(),
                    agent_id: "synthetic-pi".into(),
                    connection_id: "synthetic-connection".into(),
                };
                let _ = review::propose(
                    &actor_state.db.conn,
                    &ctx,
                    &row.try_get::<String>("", "id").unwrap(),
                    row.try_get("", "revision").unwrap(),
                    &runtime,
                )
                .await;
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });
    let seen = provider.seen.clone();
    let controls = post(
        move |headers: HeaderMap, axum::Json(input): axum::Json<serde_json::Value>| {
            let seen = seen.clone();
            async move {
                if !headers
                    .get("authorization")
                    .is_some_and(|h| h == format!("Bearer {}", fixture::TOKEN).as_str())
                {
                    return StatusCode::UNAUTHORIZED;
                }
                let mut state = seen.lock().unwrap();
                match input["mode"].as_str() {
                    Some("denied") => state.denied = true,
                    Some("ok") => state.denied = false,
                    Some("changed") => state.changed = true,
                    _ => return StatusCode::BAD_REQUEST,
                }
                StatusCode::NO_CONTENT
            }
        },
    );
    let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../out");
    let router = fixture::router(state, provider.runtime.clone(), static_dir)
        .route("/_fixture/source", controls);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4322")
        .await
        .unwrap();
    println!(
        "Synthetic intake fixture: http://127.0.0.1:4322 ; token={} ; data={}",
        fixture::TOKEN,
        dir.display()
    );
    axum::serve(listener, router).await.unwrap();
    proposer.abort();
}
