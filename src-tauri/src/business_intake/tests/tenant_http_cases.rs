//! Actual protected HTTP middleware + production intake/task operations, synthetic reader/store.
use super::{super::*, support::*};
use serde_json::{json, Value};
use std::sync::Arc;

#[tokio::test]
async fn intake_tenant_http_setup_import_review_task_and_foreign_ids_use_same_principal() {
    let f = Fixture::new().await;
    let dir = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        f.db,
        dir.path().into(),
    ));
    let router = crate::web::router::build_router(
        state.clone(),
        "synthetic-intake-platform".into(),
        dir.path().into(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    )
    .layer(axum::Extension(f.services));
    let server = axum_test::TestServer::new(router).unwrap();
    let mut tenants = vec![];
    for name in ["Tenant A", "Tenant B"] {
        let response = server.post("/api/platform/business/tenants/create")
            .add_header("authorization","Bearer synthetic-intake-platform")
            .json(&json!({"input":{"operationId":common::id(),"organizationName":name,"ownerName":format!("{name} owner")}})).await;
        response.assert_status_ok();
        let body = response.json::<Value>();
        tenants.push((format!("Bearer {}", body["token"].as_str().unwrap()), body));
    }
    let (a, meta) = &tenants[0];
    let (b, _) = &tenants[1];
    let setup = server
        .post("/api/business/intake/bindings/list")
        .add_header("authorization", a.as_str())
        .json(&json!({"input":{}}))
        .await;
    setup.assert_status_ok();
    assert_eq!(setup.json::<Value>()["setupKinds"], json!(["fireflies"]));
    server
        .post("/api/platform/business/tenants/list")
        .add_header("authorization", a.as_str())
        .json(&json!({"input":{}}))
        .await
        .assert_status_unauthorized();
    let input = json!({"operationId":common::id(),"label":"Customer meetings","domain":"feedback","sourceOwnerId":meta["ownerMemberId"],"source":{"kind":"fireflies","apiKey":"synthetic-tenant-key"},"publicationDomains":["feedback"],"retainedTaskText":true});
    for field in [
        "organizationId",
        "actorId",
        "authorizationEpoch",
        "credentialRef",
    ] {
        let mut invalid = input.clone();
        invalid[field] = json!("forged");
        server
            .post("/api/business/intake/bindings/create")
            .add_header("authorization", a.as_str())
            .json(&json!({"input":invalid}))
            .await
            .assert_status_bad_request();
    }
    f.mock
        .json(json!({"data":{"user":{"user_id":"source-user"}}}));
    let created = server
        .post("/api/business/intake/bindings/create")
        .add_header("authorization", a.as_str())
        .json(&json!({"input":input}))
        .await;
    created.assert_status_ok();
    assert_eq!(created.header("cache-control"), "no-store");
    let binding = created.json::<Value>();
    assert!(!binding.to_string().contains("synthetic-tenant-key"));
    assert!(!binding.to_string().contains("credentialRef"));
    assert_eq!(count(&state.db.conn, "business_intake_grant").await, 0);
    for path in ["bindings/status", "sources/list", "grants/list"] {
        server
            .post(&format!("/api/business/intake/{path}"))
            .add_header("authorization", b.as_str())
            .json(&json!({"input":{"bindingId":binding["id"]}}))
            .await
            .assert_status_not_found();
    }
    server
        .post("/api/business/intake/sources/list")
        .add_header("authorization", a.as_str())
        .json(&json!({"input":{"bindingId":binding["id"]}}))
        .await
        .assert_status_not_found();
    let granted=server.post("/api/business/intake/grants/upsert").add_header("authorization",a.as_str()).json(&json!({"input":{
        "operationId":common::id(),"bindingId":binding["id"],"expectedBindingRevision":binding["revision"],"memberId":meta["ownerMemberId"],"expectedGrantRevision":null,
        "scope":"binding_current_and_future_sources","read":true,"import":true,"triage":true,"publicationDomains":["feedback"],"expiresAt":null
    }})).await;
    granted.assert_status_ok();
    let enabled=server.post("/api/business/intake/bindings/update").add_header("authorization",a.as_str()).json(&json!({"input":{
        "operationId":common::id(),"bindingId":binding["id"],"expectedRevision":granted.json::<Value>()["binding"]["revision"],"label":"Customer meetings","enabled":true,"publicationDomains":["feedback"],"retainedTaskText":true
    }})).await;
    enabled.assert_status_ok();
    let started=server.post("/api/business/intake/imports/start").add_header("authorization",a.as_str()).json(&json!({"input":{
        "operationId":common::id(),"bindingId":binding["id"],"selection":{"kind":"window","fromDate":"2026-09-01T00:00:00Z","toDate":"2026-09-08T00:00:00Z"}
    }})).await;
    started.assert_status_ok();
    let mut job = started.json::<Value>();
    for response in [
        json!({"data":{"transcripts":[{"id":"meeting","title":"Private meeting"}]}}),
        transcript("meeting", "Private passage not automatically published"),
    ] {
        f.mock.json(response);
        let advanced=server.post("/api/business/intake/imports/advance").add_header("authorization",a.as_str()).json(&json!({"input":{"operationId":common::id(),"importId":job["id"],"expectedRevision":job["revision"]}})).await;
        advanced.assert_status_ok();
        job = advanced.json::<Value>();
    }
    assert_eq!(job["state"], "complete");
    let listed = server
        .post("/api/business/intake/sources/list")
        .add_header("authorization", a.as_str())
        .json(&json!({"input":{"bindingId":binding["id"]}}))
        .await;
    listed.assert_status_ok();
    let source_id = listed.json::<Value>()["items"][0]["id"].clone();
    let detail = server
        .post("/api/business/intake/sources/get")
        .add_header("authorization", a.as_str())
        .json(&json!({"input":{"sourceId":source_id}}))
        .await;
    detail.assert_status_ok();
    let detail = detail.json::<Value>();
    let candidates = server
        .post("/api/business/intake/candidates/list")
        .add_header("authorization", a.as_str())
        .json(&json!({"input":{"sourceId":source_id}}))
        .await;
    candidates.assert_status_ok();
    let row = candidates.json::<Value>()["items"][0].clone();
    let edited=server.post("/api/business/intake/candidates/edit").add_header("authorization",a.as_str()).json(&json!({"input":{
        "operationId":common::id(),"candidateId":row["id"],"expectedRevision":row["revision"],"expectedSourceRevision":detail["source"]["revision"],"passageIds":[detail["passages"][0]["id"]],
        "task":{"title":"Discuss customer feedback","notes":"Exact reviewed business text","domain":"feedback","dueDate":"2026-11-01"}
    }})).await;
    edited.assert_status_ok();
    let edited = edited.json::<Value>();
    let accept = json!({"operationId":common::id(),"candidateId":row["id"],"expectedRevision":edited["candidate"]["revision"],"expectedSourceRevision":detail["source"]["revision"],"publishToDomain":"feedback"});
    let accepted = server
        .post("/api/business/intake/candidates/accept")
        .add_header("authorization", a.as_str())
        .json(&json!({"input":accept}))
        .await;
    accepted.assert_status_ok();
    let accepted = accepted.json::<Value>();
    assert_eq!(
        accepted["task"]["task"]["notes"],
        "Exact reviewed business text"
    );
    assert_eq!(accepted["task"]["task"]["creatorId"], meta["ownerMemberId"]);
    assert_eq!(accepted["task"]["task"]["dueDate"], "2026-11-01");
    assert!(!accepted["task"].to_string().contains("Private passage"));
    for (path, input) in [
        ("sources/get", json!({"sourceId":source_id})),
        ("candidates/get", json!({"candidateId":row["id"]})),
        ("imports/get", json!({"importId":job["id"]})),
        (
            "tasks/sources",
            json!({"taskId":accepted["task"]["task"]["id"]}),
        ),
    ] {
        server
            .post(&format!("/api/business/intake/{path}"))
            .add_header("authorization", b.as_str())
            .json(&json!({"input":input}))
            .await
            .assert_status_not_found();
    }
    let replay = server
        .post("/api/business/intake/candidates/accept")
        .add_header("authorization", a.as_str())
        .json(&json!({"input":accept}))
        .await;
    replay.assert_status_ok();
    assert_eq!(replay.json::<Value>()["replayed"], true);
    assert_eq!(count(&state.db.conn, "business_task").await, 1);
    assert_eq!(count(&state.db.conn, "business_intake_decision").await, 1);
    assert_eq!(count(&state.db.conn, "business_intake_link").await, 1);
    assert_eq!(f.mock.count(), 3);
}
