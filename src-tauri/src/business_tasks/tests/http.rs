use super::*;
use std::sync::Arc;

#[tokio::test]
async fn real_business_router_derives_actor_rejects_spoofs_and_returns_revision_conflicts() {
    let db = fresh_in_memory_db().await;
    let op = initialize(&db.conn).await;
    let (_, issued, _) = human(&db.conn, &op, Role::Member, vec![Domain::Feedback]).await;
    let (_, viewer_issued, _) = human(&db.conn, &op, Role::Viewer, vec![Domain::Feedback]).await;
    let (_, outside_issued, _) = human(&db.conn, &op, Role::Member, vec![Domain::Marketing]).await;
    let root = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        root.path().into(),
    ));
    let server = axum_test::TestServer::new(crate::web::router::build_router(
        state,
        "synthetic-operator-only".into(),
        root.path().into(),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    ))
    .unwrap();
    let bearer = format!("Bearer {}", issued.token);
    let viewer = format!("Bearer {}", viewer_issued.token);
    let outside = format!("Bearer {}", outside_issued.token);
    server
        .post("/api/business/tasks/list")
        .json(&json!({"input":{}}))
        .await
        .assert_status_unauthorized();
    let created = server.post("/api/business/tasks/create").add_header("authorization", bearer.as_str()).json(&json!({"input":{"title":"Real router task", "domain":"feedback", "dueDate":"2028-02-29"}})).await;
    created.assert_status_ok();
    assert_eq!(created.header("cache-control"), "no-store");
    let value = created.json::<serde_json::Value>();
    assert_eq!(value["task"]["ownerId"], issued.credential.member_id);
    assert_eq!(
        value["activity"][0]["actor"]["id"],
        issued.credential.member_id
    );
    assert_eq!(value["task"]["dueDate"], "2028-02-29");
    let id = &value["task"]["id"];
    for body in [
        json!({"input":{"title":"forged", "domain":"feedback", "actor":"operator"}}),
        json!({"input":{"title":"forged", "domain":"feedback"}, "organizationId":op.organization_id()}),
    ] {
        let rejected = server
            .post("/api/business/tasks/create")
            .add_header("authorization", bearer.as_str())
            .json(&body)
            .await;
        rejected.assert_status_bad_request();
        assert_eq!(
            rejected.json::<serde_json::Value>()["code"],
            "invalid_input"
        );
    }
    server
        .post("/api/business/tasks/get")
        .add_header("authorization", outside.as_str())
        .json(&json!({"input":{"taskId":id}}))
        .await
        .assert_status_not_found();
    server
        .post("/api/business/tasks/note")
        .add_header("authorization", viewer.as_str())
        .json(&json!({"input":{"taskId":id,"expectedRevision":1,"body":"forbidden"}}))
        .await
        .assert_status_forbidden();
    let write = json!({"input":{"taskId":id,"expectedRevision":1,"body":"Human update"}});
    server
        .post("/api/business/tasks/note")
        .add_header("authorization", bearer.as_str())
        .json(&write)
        .await
        .assert_status_ok();
    let stale = server
        .post("/api/business/tasks/note")
        .add_header("authorization", bearer.as_str())
        .json(&write)
        .await;
    stale.assert_status(axum::http::StatusCode::CONFLICT);
    assert_eq!(stale.json::<serde_json::Value>()["code"], "already_exists");
    for status in ["done", "cancelled"] {
        server
            .post("/api/business/tasks/progress")
            .add_header("authorization", bearer.as_str())
            .json(&json!({"input":{"taskId":id,"expectedRevision":2,"status":status}}))
            .await
            .assert_status_bad_request();
    }
    let current = server
        .post("/api/business/tasks/get")
        .add_header("authorization", bearer.as_str())
        .json(&json!({"input":{"taskId":id}}))
        .await
        .json::<serde_json::Value>();
    assert_eq!(current["task"]["revision"], 2);
    assert_eq!(current["activity"].as_array().unwrap().len(), 2);
    assert_eq!(
        current["activity"][1]["actor"]["id"],
        issued.credential.member_id
    );
    assert!(!current.to_string().contains(&issued.token));
}
