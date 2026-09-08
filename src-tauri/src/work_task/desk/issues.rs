//! P1 tests traverse production token listener -> live engine -> accepted host.
use super::*;
use crate::ops_intake_host::{agent as intake, types::SourceInput};

#[path = "issues_browser.rs"]
#[cfg(unix)]
mod browser;

fn sql(text: &str, values: Vec<sea_orm::Value>) -> Statement {
    Statement::from_sql_and_values(sea_orm::DbBackend::Sqlite, text, values)
}
async fn context(engine: &TaskEngine, id: i32) -> agent::RunContext {
    let task = work_task_service::get_model(&engine.db.conn, id)
        .await
        .unwrap();
    agent::RunContext {
        account_id: agent::account_id().unwrap(),
        task_id: id,
        run_seq: task.run_seq,
        connection_id: task.connection_id.unwrap(),
        agent_id: "pi".into(),
    }
}
async fn get(listener: &Arc<DelegationListener>, token: &str, source: &SourceInput) -> Value {
    succeeded(
        call(
            listener,
            token,
            DeskTool::HafidhFeedbackGet,
            json!({"ulid":source.ulid}),
        )
        .await,
    )
}
async fn no_proposal(engine: &TaskEngine) {
    assert!(ops_proposal::Entity::find()
        .all(&engine.db.conn)
        .await
        .unwrap()
        .is_empty());
    assert_eq!(
        engine
            .db
            .conn
            .query_one(sql("SELECT COUNT(*) AS n FROM ops_intake_filing", vec![]))
            .await
            .unwrap()
            .unwrap()
            .try_get::<i64>("", "n")
            .unwrap(),
        0
    );
}
async fn cache_bytes(engine: &TaskEngine) -> Vec<String> {
    let mut result = vec![];
    for query in [
        "SELECT json_array(product_id,ulid,record_json,verified_at,error) AS data FROM ops_intake_host_snapshot ORDER BY product_id,ulid",
        "SELECT json_array(source_key,revision,fetched_at) AS data FROM ops_intake_source ORDER BY source_key",
        "SELECT json_array(id,revision,draft_json) AS data FROM ops_intake_host_draft ORDER BY id",
        "SELECT json_array(artifact_id,sha256,reviewed_by) AS data FROM ops_intake_evidence ORDER BY artifact_id",
    ] { for row in engine.db.conn.query_all(sql(query, vec![])).await.unwrap() {
        result.push(row.try_get("", "data").unwrap());
    }}
    result
}

#[tokio::test]
async fn desk_issues_cached_pagination_and_changed_source_preserve_human_draft() {
    let (engine, task) = running_task().await;
    let (listener, _, token) = bridge(&engine, PARENT_CONN).await;
    let ctx = context(&engine, task).await;
    let (source, draft) = intake::tests::human_prepared(&engine.db.conn, &ctx).await;
    for n in 0..14 {
        let ulid = format!("01ARZ3NDEKTSV4RRFFQ69G5F{n:02}");
        engine.db.conn.execute(sql(
            "INSERT INTO ops_intake_host_snapshot(product_id,ulid,record_json,verified_at,error) SELECT product_id,?,json_set(record_json,'$.source_ref.ulid',?),NULL,NULL FROM ops_intake_host_snapshot WHERE product_id=? AND ulid=?",
            vec![ulid.clone().into(), ulid.into(), source.product_id.clone().into(), source.ulid.clone().into()],
        )).await.unwrap();
    }
    let before = cache_bytes(&engine).await;
    let first = succeeded(call(&listener, &token, DeskTool::HafidhFeedbackList, json!({})).await);
    assert_eq!(first["nextPage"], 1);
    assert_eq!(first["items"].as_array().unwrap().len(), 10);
    assert!(first["items"]
        .as_array()
        .unwrap()
        .iter()
        .all(|i| i["draft"].is_null()));
    let second = succeeded(
        call(
            &listener,
            &token,
            DeskTool::HafidhFeedbackList,
            json!({"page":first["nextPage"]}),
        )
        .await,
    );
    assert_eq!(second["items"].as_array().unwrap().len(), 5);
    assert!(second["nextPage"].is_null());
    let mut ulids: Vec<_> = first["items"]
        .as_array()
        .unwrap()
        .iter()
        .chain(second["items"].as_array().unwrap())
        .map(|i| i["ulid"].as_str().unwrap())
        .collect();
    ulids.sort();
    ulids.dedup();
    assert_eq!(ulids.len(), 15);
    assert_eq!(cache_bytes(&engine).await, before);
    engine.db.conn.execute(sql("UPDATE ops_intake_host_snapshot SET record_json=json_set(record_json,'$.source_revision',?),verified_at=NULL WHERE product_id=? AND ulid=?", vec!["b".repeat(64).into(), source.product_id.clone().into(), source.ulid.clone().into()])).await.unwrap();
    let before = cache_bytes(&engine).await;
    let detail = get(&listener, &token, &source).await;
    assert_eq!(detail["item"]["draft"]["revision"], draft.revision);
    assert_eq!(detail["item"]["draft"]["matchesSourceRevision"], false);
    assert_eq!(detail["item"]["draft"]["preparedForCurrentRun"], false);
    assert_eq!(detail["item"]["cache"]["state"], "unverified");
    assert_eq!(cache_bytes(&engine).await, before);
    no_proposal(&engine).await;
}

#[tokio::test]
async fn desk_issues_human_edit_exactness_and_revoked_evidence() {
    use crate::ops_intake_host::{
        operator,
        types::{SaveInput, Severity},
        HostRuntime,
    };
    let (engine, task) = running_task().await;
    let (listener, _, token) = bridge(&engine, PARENT_CONN).await;
    let ctx = context(&engine, task).await;
    let (source, initial) = intake::tests::human_prepared(&engine.db.conn, &ctx).await;
    let human = crate::ops::Operator::server().unwrap();
    let runtime = HostRuntime::production();
    let mut d = operator::save(
        &engine.db.conn,
        &human,
        &runtime,
        SaveInput {
            source: source.clone(),
            expected_revision: initial.revision,
            title: "Human corrected title".into(),
            summary: "Human corrected reproduction".into(),
            labels: vec![],
            confirmed_severity: None,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeIssue,
            json!({"draftId":d.id,"expectedRevision":initial.revision})
        )
        .await
        .code,
        Some(DeskError::Stale)
    );
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeIssue,
            json!({"draftId":d.id,"expectedRevision":d.revision})
        )
        .await
        .code,
        Some(DeskError::SeverityRequired)
    );
    no_proposal(&engine).await;
    d = operator::save(
        &engine.db.conn,
        &human,
        &runtime,
        SaveInput {
            source: source.clone(),
            expected_revision: d.revision,
            title: d.title,
            summary: d.summary,
            labels: d.labels,
            confirmed_severity: Some(Severity::Medium),
        },
    )
    .await
    .unwrap();
    crate::ops_intake::revoke_evidence(&engine.db.conn, &d.proofs["log"].proof.artifact_id)
        .await
        .unwrap();
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeIssue,
            json!({"draftId":d.id,"expectedRevision":d.revision})
        )
        .await
        .code,
        Some(DeskError::EvidenceRequired)
    );
    no_proposal(&engine).await;
    // Human re-attaches reviewed evidence through the accepted helper. The
    // native proposal must carry the human's new title/body, never the old one.
    d = intake::tests::prepare_source(&engine.db.conn, &ctx, &source).await;
    let result = succeeded(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeIssue,
            json!({"draftId":d.id,"expectedRevision":d.revision}),
        )
        .await,
    );
    assert_eq!(result["status"], "pending");
    assert_eq!(result["title"], "Human corrected title");
    let rows = ops_proposal::Entity::find()
        .all(&engine.db.conn)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    let payload: Value = serde_json::from_str(&rows[0].payload_json).unwrap();
    assert_eq!(payload, serde_json::to_value(d.prepared.unwrap()).unwrap());
    assert!(payload["outgoing"]["body"]
        .as_str()
        .unwrap()
        .contains("Human corrected reproduction"));
}

#[tokio::test]
async fn desk_issues_cached_metadata_and_stale_reads_never_mint_freshness() {
    let (engine, task) = running_task().await;
    let (listener, _, token) = bridge(&engine, PARENT_CONN).await;
    assert_eq!(
        call(&listener, &token, DeskTool::HafidhIntakeStatus, json!({}))
            .await
            .code,
        Some(DeskError::ProductMissing)
    );
    let ctx = context(&engine, task).await;
    let (source, draft) = intake::tests::human_prepared(&engine.db.conn, &ctx).await;
    let before = cache_bytes(&engine).await;
    let status = succeeded(call(&listener, &token, DeskTool::HafidhIntakeStatus, json!({})).await);
    assert_eq!(status["cachedRecords"], 1);
    assert_eq!(status["inAppAvailable"], false);
    let list = succeeded(call(&listener, &token, DeskTool::HafidhFeedbackList, json!({})).await);
    assert_eq!(list["items"].as_array().unwrap().len(), 1);
    assert!(list["nextPage"].is_null());
    let detail = get(&listener, &token, &source).await;
    assert_eq!(detail["item"]["cache"]["state"], "fresh");
    assert_eq!(detail["item"]["draft"]["draftId"], draft.id);
    assert_eq!(detail["item"]["draft"]["revision"], draft.revision);
    assert_eq!(detail["item"]["draft"]["preparedForCurrentRun"], true);
    for data in [status, list, detail] {
        let text = data.to_string();
        for private in [
            "PRIVATE-",
            "artifact_id",
            "sha256",
            "proofs",
            "binding_digest",
            "external_id",
            "bearer_ref",
            "reviewed_by",
        ] {
            assert!(!text.contains(private), "private projection {private}");
        }
    }
    assert_eq!(cache_bytes(&engine).await, before);

    for (verified, error, state) in [
        (None, None, "unverified"),
        (Some(chrono::Utc::now().timestamp() - 901), None, "expired"),
        (
            Some(chrono::Utc::now().timestamp()),
            Some("PRIVATE RAW ERROR"),
            "unavailable",
        ),
    ] {
        engine
            .db
            .conn
            .execute(sql(
                "UPDATE ops_intake_host_snapshot SET verified_at=?,error=? WHERE product_id=?",
                vec![
                    verified.into(),
                    error.into(),
                    source.product_id.clone().into(),
                ],
            ))
            .await
            .unwrap();
        let before = cache_bytes(&engine).await;
        let detail = get(&listener, &token, &source).await;
        assert_eq!(detail["item"]["cache"]["state"], state);
        assert!(detail["item"]["cache"]["operatorAction"]
            .as_str()
            .unwrap()
            .contains("operator"));
        assert!(!detail.to_string().contains("PRIVATE"));
        assert_eq!(detail["item"]["title"], "Synthetic audio report");
        assert_eq!(cache_bytes(&engine).await, before);
        assert_eq!(
            call(
                &listener,
                &token,
                DeskTool::DeskProposeIssue,
                json!({"draftId":draft.id,"expectedRevision":draft.revision})
            )
            .await
            .code,
            Some(DeskError::CacheExpired)
        );
        no_proposal(&engine).await;
    }
    let missing = succeeded(
        call(
            &listener,
            &token,
            DeskTool::HafidhFeedbackGet,
            json!({"ulid":"01ARZ3NDEKTSV4RRFFQ69G5FAW"}),
        )
        .await,
    );
    assert_eq!(missing["item"]["cache"]["state"], "missing");
    assert!(missing["feedback"].is_null());
}

#[tokio::test]
async fn desk_issues_closed_inputs_foreign_and_ambiguous_scope() {
    let (engine, task) = running_task().await;
    let (listener, _, token) = bridge(&engine, PARENT_CONN).await;
    let ctx = context(&engine, task).await;
    let (source, draft) = intake::tests::human_prepared(&engine.db.conn, &ctx).await;
    for (tool, input) in [
        (DeskTool::HafidhFeedbackList, json!({"page":-1})),
        (DeskTool::HafidhFeedbackList, json!({"page":10001})),
        (DeskTool::HafidhFeedbackList, json!({"cursor":"upstream"})),
        (DeskTool::HafidhIntakeStatus, json!({"accountId":1})),
        (
            DeskTool::HafidhFeedbackGet,
            json!({"ulid":source.ulid,"productId":source.product_id}),
        ),
        (DeskTool::HafidhFeedbackGet, json!({"ulid":"../../file"})),
        (
            DeskTool::DeskProposeIssue,
            json!({"draftId":draft.id,"expectedRevision":0}),
        ),
        (
            DeskTool::DeskProposeIssue,
            json!({"draftId":draft.id,"expectedRevision":1.5}),
        ),
        (
            DeskTool::DeskProposeIssue,
            json!({"draftId":"","expectedRevision":1}),
        ),
        (
            DeskTool::DeskProposeIssue,
            json!({"draftId":draft.id,"expectedRevision":draft.revision,"actor":"operator"}),
        ),
        (
            DeskTool::DeskProposeIssue,
            json!({"draftId":draft.id,"expectedRevision":draft.revision,"proofs":{}}),
        ),
    ] {
        assert_eq!(
            call(&listener, &token, tool, input).await.code,
            Some(DeskError::InvalidInput)
        );
    }
    let before = cache_bytes(&engine).await;
    let foreign = crate::db::test_helpers::seed_folder(&engine.db, "/fixture/foreign").await;
    engine.db.conn.execute(sql("UPDATE ops_intake_host_product SET config_json=json_set(config_json,'$.binding.folder_id',?) WHERE product_id=?", vec![foreign.into(), source.product_id.clone().into()])).await.unwrap();
    assert_eq!(
        call(&listener, &token, DeskTool::HafidhIntakeStatus, json!({}))
            .await
            .code,
        Some(DeskError::ProductMissing)
    );
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeIssue,
            json!({"draftId":draft.id,"expectedRevision":draft.revision})
        )
        .await
        .code,
        Some(DeskError::ProductMissing)
    );
    let folder = work_task_service::get_model(&engine.db.conn, task)
        .await
        .unwrap()
        .folder_id;
    engine.db.conn.execute(sql("UPDATE ops_intake_host_product SET account_id=999,config_json=json_set(config_json,'$.binding.folder_id',?) WHERE product_id=?", vec![folder.into(), source.product_id.clone().into()])).await.unwrap();
    assert_eq!(
        call(&listener, &token, DeskTool::HafidhFeedbackList, json!({}))
            .await
            .code,
        Some(DeskError::ProductMissing)
    );
    engine
        .db
        .conn
        .execute(sql(
            "UPDATE ops_intake_host_product SET account_id=?",
            vec![ctx.account_id.into()],
        ))
        .await
        .unwrap();
    intake::tests::human_prepared(&engine.db.conn, &ctx).await;
    for tool in [DeskTool::HafidhFeedbackList, DeskTool::HafidhIntakeStatus] {
        assert_eq!(
            call(&listener, &token, tool, json!({})).await.code,
            Some(DeskError::AmbiguousProduct)
        );
    }
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeIssue,
            json!({"draftId":draft.id,"expectedRevision":draft.revision})
        )
        .await
        .code,
        Some(DeskError::AmbiguousProduct)
    );
    assert!(!before.is_empty());
    no_proposal(&engine).await;
}

#[tokio::test]
async fn desk_issues_exact_human_revision_pending_floor_and_stable_deny() {
    let (engine, task) = running_task().await;
    let ctx = context(&engine, task).await;
    let (source, _) = intake::tests::human_prepared(&engine.db.conn, &ctx).await;
    scope::ActiveModel {
        agent_id: Set("pi".into()),
        domain: Set("github".into()),
        resource: Set(Some("owner/repo".into())),
        mode: Set("act_low_risk".into()),
        ..Default::default()
    }
    .insert(&engine.db.conn)
    .await
    .unwrap();
    rule::ActiveModel {
        agent_id: Set("pi".into()),
        domain: Set("github".into()),
        resource: Set(None),
        action_name: Set(Some("github.create_issue".into())),
        behavior: Set("allow".into()),
        ..Default::default()
    }
    .insert(&engine.db.conn)
    .await
    .unwrap();
    let deny = rule::ActiveModel {
        agent_id: Set("pi".into()),
        domain: Set("github".into()),
        resource: Set(None),
        action_name: Set(None),
        behavior: Set("deny".into()),
        ..Default::default()
    }
    .insert(&engine.db.conn)
    .await
    .unwrap();
    let mut previous: Option<(Arc<DelegationListener>, String)> = None;
    for _ in 0..2 {
        let connection = uuid::Uuid::new_v4().to_string();
        relaunch_on(&engine, task, &connection).await;
        let (listener, _, token) = bridge(&engine, &connection).await;
        if let Some((old, token)) = previous.take() {
            assert_eq!(
                call(&old, &token, DeskTool::HafidhIntakeStatus, json!({}))
                    .await
                    .code,
                Some(DeskError::Stale)
            );
        }
        let d = get(&listener, &token, &source).await["item"]["draft"].clone();
        assert_eq!(d["preparedForCurrentRun"], false);
        assert_eq!(
            call(
                &listener,
                &token,
                DeskTool::DeskProposeIssue,
                json!({"draftId":d["draftId"],"expectedRevision":d["revision"]})
            )
            .await
            .code,
            Some(DeskError::Denied)
        );
        previous = Some((listener, token));
    }
    no_proposal(&engine).await;
    let gates = audit::Entity::find()
        .filter(audit::Column::Action.eq("agent.gate"))
        .all(&engine.db.conn)
        .await
        .unwrap();
    assert_eq!(gates.len(), 2);
    for row in gates {
        assert_eq!(row.actor, "pi");
        assert!(row
            .detail
            .contains(&format!("\"matched_rule_id\":{}", deny.id)));
    }
    rule::Entity::delete_by_id(deny.id)
        .exec(&engine.db.conn)
        .await
        .unwrap();
    let (listener, token) = previous.unwrap();
    let d = get(&listener, &token, &source).await["item"]["draft"].clone();
    let request = json!({"draftId":d["draftId"],"expectedRevision":d["revision"]});
    let result = succeeded(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeIssue,
            request.clone(),
        )
        .await,
    );
    assert_eq!(result["status"], "pending");
    assert_eq!(result["submittedRevision"], d["revision"]);
    assert_eq!(result["title"], "Synthetic audio report");
    assert!(!result.to_string().contains("proof"));
    let rows = ops_proposal::Entity::find()
        .all(&engine.db.conn)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    let stored: Value = serde_json::from_str(&rows[0].payload_json).unwrap();
    let host = engine
        .db
        .conn
        .query_one(sql(
            "SELECT draft_json FROM ops_intake_host_draft WHERE id=?",
            vec![d["draftId"].as_str().unwrap().into()],
        ))
        .await
        .unwrap()
        .unwrap();
    let host: Value =
        serde_json::from_str(&host.try_get::<String>("", "draft_json").unwrap()).unwrap();
    assert_eq!(stored, host["prepared"]);
    assert_eq!(result["proposalId"], rows[0].id);
    assert_eq!(
        succeeded(call(&listener, &token, DeskTool::DeskProposeIssue, request).await),
        result
    );
    assert_eq!(
        call(
            &listener,
            &token,
            DeskTool::DeskProposeIssue,
            json!({"draftId":d["draftId"],"expectedRevision":1})
        )
        .await
        .code,
        Some(DeskError::Stale)
    );
    assert_eq!(
        engine
            .db
            .conn
            .query_one(sql("SELECT COUNT(*) AS n FROM ops_intake_filing", vec![]))
            .await
            .unwrap()
            .unwrap()
            .try_get::<i64>("", "n")
            .unwrap(),
        0
    );
}

#[tokio::test]
async fn desk_issues_cancel_writer_and_peer_abort_prevent_queued_proposal() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/pi-issues-fixtures");
    std::fs::create_dir_all(&root).unwrap();
    for cancel in [false, true] {
        let dir = tempfile::tempdir_in(&root).unwrap();
        let db = crate::db::init_database(dir.path(), "issues-race")
            .await
            .unwrap();
        let (engine, task) = running_task_in(db).await;
        let ctx = context(&engine, task).await;
        let (_, d) = intake::tests::human_prepared(&engine.db.conn, &ctx).await;
        let (listener, access, token) = bridge(&engine, PARENT_CONN).await;
        let before = cache_bytes(&engine).await;
        let writer = engine.db.conn.begin().await.unwrap();
        writer
            .execute(sql(
                if cancel {
                    "UPDATE work_task SET status='canceled',connection_id=NULL WHERE id=?"
                } else {
                    "UPDATE work_task SET run_seq=run_seq WHERE id=?"
                },
                vec![task.into()],
            ))
            .await
            .unwrap();
        let (mut client, served) = start_call(
            listener.clone(),
            &token,
            DeskTool::DeskProposeIssue,
            json!({"draftId":d.id,"expectedRevision":d.revision}),
        )
        .await;
        access.entered.notified().await;
        tokio::time::sleep(Duration::from_millis(80)).await;
        assert!(!served.is_finished());
        if cancel {
            writer.commit().await.unwrap();
            let response: BrokerResponse = read_frame(&mut client).await.unwrap();
            assert_ne!(response.outcome["ok"], true);
            served.await.unwrap();
        } else {
            drop(client);
            tokio::time::timeout(Duration::from_secs(2), served)
                .await
                .unwrap()
                .unwrap();
            listener.tokens.revoke(&token).await;
            writer.commit().await.unwrap();
            assert_eq!(
                call(
                    &listener,
                    &token,
                    DeskTool::DeskProposeIssue,
                    json!({"draftId":d.id,"expectedRevision":d.revision})
                )
                .await
                .code,
                Some(DeskError::Denied)
            );
        }
        no_proposal(&engine).await;
        assert_eq!(cache_bytes(&engine).await, before);
    }
}
