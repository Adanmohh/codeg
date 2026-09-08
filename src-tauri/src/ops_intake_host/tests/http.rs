use super::*;
use serde_json::Value;
use std::sync::Arc;

async fn post(server: &axum_test::TestServer, name: &str, input: Value) -> axum_test::TestResponse {
    server
        .post(&format!("/api/ops_intake_{name}"))
        .add_header("authorization", format!("Bearer {}", fixture::TOKEN))
        .json(&json!({"input":input}))
        .await
}

#[tokio::test]
async fn protected_routes_reject_missing_auth_and_forged_identity_without_echo() {
    let provider = fixture::Provider::start().await;
    let (db, source) = seeded().await;
    let dir = tempfile::tempdir().unwrap();
    let state = Arc::new(crate::app_state::AppState::new_for_test(
        db,
        dir.path().into(),
    ));
    let server = axum_test::TestServer::new(fixture::router(
        state.clone(),
        provider.runtime.clone(),
        dir.path().into(),
    ))
    .unwrap();
    for name in [
        "status",
        "configure",
        "list",
        "detail",
        "refresh",
        "save",
        "attach",
        "prepare",
        "approve",
        "deny",
        "reconcile",
        "fix",
    ] {
        server
            .post(&format!("/api/ops_intake_{name}"))
            .json(&json!({"input":{}}))
            .await
            .assert_status_unauthorized();
        server
            .post(&format!("/api/ops_intake_{name}"))
            .add_header("authorization", "Bearer wrong")
            .json(&json!({"input":{}}))
            .await
            .assert_status_unauthorized();
    }
    for field in [
        "actor",
        "account_id",
        "source_revision",
        "fetched_at",
        "url",
        "path",
        "credential",
    ] {
        let mut input = json!({"product_id":source.product_id,"ulid":source.ulid});
        input[field] = json!("PRIVATE_SENTINEL");
        let response = post(&server, "refresh", input).await;
        assert!(response.status_code().is_client_error());
        assert!(!response.text().contains("PRIVATE_SENTINEL"));
    }
    let foreign = post(
        &server,
        "detail",
        json!({"product_id":"foreign","ulid":source.ulid}),
    )
    .await;
    assert!(foreign.status_code().is_client_error());
    for field in ["build", "PRIVATE_SENTINEL"] {
        let response=post(&server,"attach",json!({"source":{"product_id":source.product_id,"ulid":source.ulid},
            "expected_revision":1,"field":field,"value":"42","content":"42","actor":"PRIVATE_SENTINEL"})).await;
        assert!(response.status_code().is_client_error());
        assert!(!response.text().contains("PRIVATE_SENTINEL"));
    }
    assert_eq!(provider.seen.lock().unwrap().reads, 0);
    assert_eq!(provider.seen.lock().unwrap().posts, 0);
}

#[tokio::test]
async fn configuration_secrets_are_write_only_and_unconfigured_filing_stays_pending() {
    let provider = fixture::Provider::start().await;
    let (db, source, ctx, d) = ready(&provider).await;
    let p = review::propose(&db.conn, &ctx, &d.id, d.revision, &provider.runtime)
        .await
        .unwrap();
    let stored = product(&db.conn, 1, &source.product_id).await.unwrap();
    provider
        .runtime
        .secrets
        .delete(stored.key_ref.as_deref().unwrap())
        .unwrap();
    let status = operator::status(&db.conn, &human(), &provider.runtime)
        .await
        .unwrap();
    let wire = serde_json::to_string(&status).unwrap();
    for forbidden in [
        "PRIVATE KEY",
        "synthetic-intake-only",
        "bearer_ref",
        "key_ref",
    ] {
        assert!(!wire.contains(forbidden));
    }
    assert!(!status.products[0].app_key_present);
    assert!(matches!(
        review::approve(
            &db.conn,
            &human(),
            &provider.runtime,
            ReviewInput {
                source: source.clone(),
                proposal_id: p.proposal_id.unwrap(),
                expected_payload: p.prepared.clone(),
                approved_payload: p.prepared
            }
        )
        .await,
        Err(HostError::NotConfigured)
    ));
    let detail = operator::detail(&db.conn, &human(), source).await.unwrap();
    assert_eq!(detail.proposals[0].status, "pending");
    assert!(!detail.handoff_unknown);
    assert_eq!(provider.seen.lock().unwrap().posts, 0);
}

#[tokio::test]
async fn failed_upstream_refresh_or_list_removes_freshness_without_private_response() {
    for operation in ["refresh", "list"] {
        let provider = fixture::Provider::start().await;
        let (db, source, _, _) = ready(&provider).await;
        provider.seen.lock().unwrap().denied = true;
        let error = if operation == "refresh" {
            operator::refresh(&db.conn, &human(), &provider.runtime, source.clone())
                .await
                .err()
        } else {
            operator::list(
                &db.conn,
                &human(),
                &provider.runtime,
                ListInput {
                    product_id: source.product_id.clone(),
                    cursor: None,
                },
            )
            .await
            .err()
        };
        assert_eq!(error, Some(HostError::AccessDenied));
        assert!(fresh(&snapshot(&db.conn, &source).await.unwrap()).is_err());
        let d = operator::detail(&db.conn, &human(), source).await.unwrap();
        assert!(!serde_json::to_string(&d).unwrap().contains("PRIVATE"));
    }
}

#[tokio::test]
async fn upstream_revision_change_resets_attached_proofs_and_human_severity() {
    let provider = fixture::Provider::start().await;
    let (db, source, ctx, d) = ready(&provider).await;
    review::propose(&db.conn, &ctx, &d.id, d.revision, &provider.runtime)
        .await
        .unwrap();
    provider.seen.lock().unwrap().changed = true;
    let detail = operator::refresh(&db.conn, &human(), &provider.runtime, source)
        .await
        .unwrap();
    assert_ne!(detail.draft.source_revision, d.source_revision);
    assert!(
        detail.draft.proofs.is_empty()
            && detail.draft.confirmed_severity.is_none()
            && detail.draft.prepared.is_none()
    );
    assert!(detail.proposals[0].stale);
}

#[tokio::test]
async fn proof_gates_reject_omissions_private_content_stale_and_old_revision() {
    let provider = fixture::Provider::start().await;
    let (db, source, ctx, mut d) = ready(&provider).await;
    for field in ["build", "screen", "reciter", "log"] {
        let mut missing = d.clone();
        missing.proofs.remove(field);
        missing.prepared = None;
        cas(&db.conn, &mut missing, d.revision).await.unwrap();
        assert!(matches!(
            review::prepare_for(&db.conn, 1, &source, missing.revision, ctx.task_id, None).await,
            Err(HostError::InvalidEvidence)
        ));
        cas(&db.conn, &mut d, missing.revision).await.unwrap();
    }
    for content in [
        "Bearer PRIVATE_SENTINEL",
        "tester@example.invalid",
        "/Users/private/log.txt",
        "https://private.invalid/log",
        "42",
    ] {
        let value = if content == "42" { "43" } else { "42" };
        assert!(operator::attach(
            &db.conn,
            &human(),
            &provider.runtime,
            AttachInput {
                source: source.clone(),
                expected_revision: d.revision,
                field: ops_intake::EvidenceField::Build,
                value: value.into(),
                content: content.into(),
                captured_at: None,
                session_ulid: None
            }
        )
        .await
        .is_err());
    }
    assert!(matches!(
        operator::attach(
            &db.conn,
            &human(),
            &provider.runtime,
            AttachInput {
                source: source.clone(),
                expected_revision: 1,
                field: ops_intake::EvidenceField::Build,
                value: "42".into(),
                content: "42".into(),
                captured_at: None,
                session_ulid: None
            }
        )
        .await,
        Err(HostError::Conflict)
    ));
    invalidate(&db.conn, &source, HostError::SourceUnavailable)
        .await
        .unwrap();
    assert!(matches!(
        review::prepare_for(&db.conn, 1, &source, d.revision, ctx.task_id, None).await,
        Err(HostError::StaleSource)
    ));
}

#[tokio::test]
async fn approval_rejects_tamper_newer_draft_expired_proof_and_read_only_policy() {
    for mutation in ["payload", "revision", "expiry", "read_only", "deny"] {
        let provider = fixture::Provider::start().await;
        let (db, source, ctx, d) = ready(&provider).await;
        let p = review::propose(&db.conn, &ctx, &d.id, d.revision, &provider.runtime)
            .await
            .unwrap();
        let mut approved = p.prepared.clone();
        match mutation {
            "payload" => approved.outgoing.body.push_str(" tampered"),
            "revision" => {
                let mut next = d.clone();
                cas(&db.conn, &mut next, d.revision).await.unwrap();
            }
            "expiry" => {
                db.conn
                    .execute_unprepared("UPDATE ops_intake_evidence SET expires_at=0")
                    .await
                    .unwrap();
            }
            "read_only" => {
                db.conn.execute_unprepared("INSERT INTO ops_agent_scope(agent_id,domain,resource,mode) VALUES('synthetic-pi','github','owner/repo','read')").await.unwrap();
            }
            _ => {
                db.conn.execute_unprepared("INSERT INTO ops_agent_rule(agent_id,domain,action_name,resource,behavior) VALUES('synthetic-pi','github','github.create_issue','owner/repo','deny')").await.unwrap();
            }
        }
        // A revision-only CAS makes the stored prepared snapshot identical;
        // the human handoff intentionally loads and binds the current revision.
        // Invalidate the actual draft contents too, as a saved edit does.
        if mutation == "revision" {
            let mut next = draft(&db.conn, &source).await.unwrap();
            let rev = next.revision;
            next.prepared = None;
            cas(&db.conn, &mut next, rev).await.unwrap();
        }
        assert!(
            review::approve(
                &db.conn,
                &human(),
                &provider.runtime,
                ReviewInput {
                    source,
                    proposal_id: p.proposal_id.unwrap(),
                    expected_payload: p.prepared,
                    approved_payload: approved
                }
            )
            .await
            .is_err(),
            "{mutation}"
        );
        assert_eq!(provider.seen.lock().unwrap().posts, 0);
    }
}

#[tokio::test]
async fn response_lost_stays_unknown_blocks_repeat_and_reconciles_the_same_issue() {
    let provider = fixture::Provider::start().await;
    let (db, source, ctx, d) = ready(&provider).await;
    let edited = operator::save(
        &db.conn,
        &human(),
        &provider.runtime,
        SaveInput {
            source: source.clone(),
            expected_revision: d.revision,
            title: "Synthetic unknown response".into(),
            summary: d.summary,
            labels: d.labels,
            confirmed_severity: Some(Severity::Medium),
        },
    )
    .await
    .unwrap();
    let p = review::propose(
        &db.conn,
        &ctx,
        &edited.id,
        edited.revision,
        &provider.runtime,
    )
    .await
    .unwrap();
    let input = || ReviewInput {
        source: source.clone(),
        proposal_id: p.proposal_id.unwrap(),
        expected_payload: p.prepared.clone(),
        approved_payload: p.prepared.clone(),
    };
    let detail = review::approve(&db.conn, &human(), &provider.runtime, input())
        .await
        .unwrap();
    assert!(matches!(
        detail.receipt.unwrap().state,
        ops_intake::FilingState::Unknown
    ));
    assert!(
        review::approve(&db.conn, &human(), &provider.runtime, input())
            .await
            .is_err()
    );
    let detail = review::reconcile(&db.conn, &human(), &provider.runtime, source)
        .await
        .unwrap();
    assert!(matches!(
        detail.receipt.unwrap().state,
        ops_intake::FilingState::Created
    ));
    let seen = provider.seen.lock().unwrap();
    assert_eq!(seen.posts, 1);
    assert_eq!(seen.tokens, 1);
}
