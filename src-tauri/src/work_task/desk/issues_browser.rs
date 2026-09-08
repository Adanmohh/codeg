//! Manual test-only protected UI over the real token/listener/task/host path.
//! Reuses the accepted host's synthetic Python/provider fixture. No engine pump,
//! inference, global configuration, credentials or external provider request.
use super::*;
use crate::ops_intake_host::{operator, tests::fixture::Provider, types::ListInput};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

const OPERATOR_TOKEN: &str = "pi-issues-synthetic-operator";

async fn rpc(
    stdin: &mut tokio::process::ChildStdin,
    stdout: &mut tokio::io::Lines<BufReader<tokio::process::ChildStdout>>,
    id: usize,
    method: &str,
    params: Value,
) -> Value {
    let line = json!({"jsonrpc":"2.0","id":id,"method":method,"params":params});
    stdin
        .write_all(format!("{line}\n").as_bytes())
        .await
        .unwrap();
    let line = tokio::time::timeout(Duration::from_secs(5), stdout.next_line())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let reply: Value = serde_json::from_str(&line).unwrap();
    assert_eq!(reply["id"], id);
    reply
}

#[tokio::test]
#[ignore = "manual real companion and Playwright CLI fixture on owned 4324"]
async fn pi_issues_browser_fixture() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest.join("target/pi-issues-browser");
    std::fs::create_dir_all(&root).unwrap();
    let stop = root.join("stop");
    assert!(
        !stop.exists(),
        "remove the previous owned fixture stop file first"
    );
    let dir = tempfile::tempdir_in(&root).unwrap();
    let db = crate::db::init_database(dir.path(), "pi-issues-browser")
        .await
        .unwrap();
    let (engine, task) = running_task_in(db).await;
    let ctx = context(&engine, task).await;
    let (source, _) = intake::tests::human_prepared(&engine.db.conn, &ctx).await;
    let provider = Provider::start().await;
    provider.configure(&engine.db, &source).await;
    let human = crate::ops::Operator::server().unwrap();
    operator::list(
        &engine.db.conn,
        &human,
        &provider.runtime,
        ListInput {
            product_id: source.product_id.clone(),
            cursor: None,
        },
    )
    .await
    .unwrap();
    operator::refresh(&engine.db.conn, &human, &provider.runtime, source.clone())
        .await
        .unwrap();
    // Fixture setup models the existing human evidence/confirmation/preparation
    // path. The agent below receives no proof objects or operator capability.
    let draft = intake::tests::prepare_source(&engine.db.conn, &ctx, &source).await;
    let (listener, _, token) = bridge(&engine, PARENT_CONN).await;
    let socket = manifest
        .parent()
        .unwrap()
        .join(format!(".pi-issues-{}.log", std::process::id()));
    let bound = DelegationListener::bind(&socket).await.unwrap();
    let serve = tokio::spawn(listener.clone().accept_loop(bound, socket.clone()));
    let before = cache_bytes(&engine).await;
    let reads_before = provider.seen.lock().unwrap().reads;

    let mut child = tokio::process::Command::new(manifest.join("target/debug/codeg-mcp"))
        .args([
            "--features",
            "intake",
            "--parent-connection-id",
            "not-a-trusted-identity",
            "--socket-path",
        ])
        .arg(&socket)
        .args(["--token", &token])
        .env_clear()
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap()).lines();
    assert!(rpc(&mut stdin, &mut stdout, 1, "initialize", json!({})).await["error"].is_null());
    let discovered = rpc(&mut stdin, &mut stdout, 2, "tools/list", json!({})).await;
    let mut names: Vec<_> = discovered["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap().to_owned())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "hafidh_feedback_get",
            "hafidh_feedback_list",
            "hafidh_intake_status"
        ]
    );
    let mut agent_results = vec![];
    for (id, name, input) in [
        (3, "hafidh_intake_status", json!({})),
        (4, "hafidh_feedback_list", json!({})),
        (5, "hafidh_feedback_get", json!({"ulid":source.ulid})),
    ] {
        let reply = rpc(
            &mut stdin,
            &mut stdout,
            id,
            "tools/call",
            json!({"name":name,"arguments":input}),
        )
        .await;
        assert_eq!(reply["result"]["isError"], false);
        let result: DeskResponse =
            serde_json::from_str(reply["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
        agent_results.push(succeeded(result));
    }
    assert_eq!(agent_results[0]["cachedRecords"], 3);
    assert_eq!(
        agent_results[2]["item"]["draft"]["revision"],
        draft.revision
    );
    assert_eq!(agent_results[2]["item"]["cache"]["state"], "fresh");
    for private in [
        "PRIVATE",
        "proofs",
        "sha256",
        "artifact_id",
        "bearer_ref",
        "testerEmail",
    ] {
        assert!(!json!(agent_results).to_string().contains(private));
    }
    assert!(rpc(
        &mut stdin,
        &mut stdout,
        6,
        "tools/call",
        json!({"name":"desk_propose_issue","arguments":{}})
    )
    .await["error"]
        .is_object());
    drop(stdin);
    assert!(tokio::time::timeout(Duration::from_secs(5), child.wait())
        .await
        .unwrap()
        .unwrap()
        .success());
    assert_eq!(cache_bytes(&engine).await, before);
    assert_eq!(provider.seen.lock().unwrap().reads, reads_before);

    // Native extension transport uses this same framed one-shot UDS request.
    // No scope/allow rule is installed: accepted default remains propose.
    let mut client = tokio::net::UnixStream::connect(&socket).await.unwrap();
    let input = json!({"draftId":draft.id,"expectedRevision":draft.revision});
    write_frame(
        &mut client,
        &BrokerMessage::Desk(BrokerDeskRequest {
            token,
            request: DeskCall {
                tool: DeskTool::DeskProposeIssue,
                input: input.clone(),
            },
        }),
    )
    .await
    .unwrap();
    let response: BrokerResponse = read_frame(&mut client).await.unwrap();
    let proposed = succeeded(serde_json::from_value(response.outcome).unwrap());
    assert_eq!(proposed["status"], "pending");
    let expected = serde_json::to_value(draft.prepared.as_ref().unwrap()).unwrap();
    let rows = ops_proposal::Entity::find()
        .all(&engine.db.conn)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(
        serde_json::from_str::<Value>(&rows[0].payload_json).unwrap(),
        expected
    );
    let evidence = json!({"companionTools":names,"cachedCalls":agent_results,"nativeInput":input,
        "nativeResult":proposed,"cachedReadsMintedFreshness":false,"cachedReadsRequestedUpstream":false});
    std::fs::write(
        root.join("bridge.json"),
        serde_json::to_vec_pretty(&evidence).unwrap(),
    )
    .unwrap();

    let state = Arc::new(crate::app_state::AppState::new_for_test(
        crate::db::AppDatabase {
            conn: engine.db.conn.clone(),
        },
        dir.path().into(),
    ));
    let seen = provider.seen.clone();
    let database = engine.db.conn.clone();
    let counts = axum::routing::get(move || {
        let seen = seen.clone();
        let database = database.clone();
        let expected = expected.clone();
        async move {
            let rows = ops_proposal::Entity::find().all(&database).await.unwrap();
            let seen = seen.lock().unwrap();
            axum::Json(json!({"upstreamReads":seen.reads,"githubPosts":seen.posts,
                "githubTokenRequests":seen.tokens,"proposals":rows.iter().map(|r| json!({"id":r.id,"status":r.status})).collect::<Vec<_>>(),
                "expectedIssue":expected["outgoing"]}))
        }
    });
    let router = crate::web::router::build_router(
        state,
        OPERATOR_TOKEN.into(),
        manifest.join("target/pi-issues-export"),
        Arc::new(crate::web::shutdown::ShutdownSignal::new()),
    )
    .layer(axum::Extension(provider.runtime.clone()))
    .route("/_fixture/counts", counts);
    let http = tokio::net::TcpListener::bind("127.0.0.1:4324")
        .await
        .unwrap();
    println!("Pi issue fixture ready on 127.0.0.1:4324; public synthetic operator token in fixture source; real bridge passed");
    axum::serve(http, router)
        .with_graceful_shutdown(async move {
            while !stop.exists() {
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        })
        .await
        .unwrap();
    let seen = provider.seen.lock().unwrap();
    assert_eq!(
        seen.posts, 0,
        "the proposal-review fixture must never file an issue"
    );
    assert_eq!(seen.tokens, 0);
    serve.abort();
    std::fs::remove_file(socket).unwrap();
}
