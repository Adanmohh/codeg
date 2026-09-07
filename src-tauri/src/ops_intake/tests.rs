use super::github::tests::{Fixture, Mode};
use super::*;
use crate::db::{
    entities::work_task::WorkTaskStatus,
    service::{ops_approvals as approvals, work_task_service as tasks},
    test_helpers::{fresh_in_memory_db, seed_conversation, seed_folder},
    AppDatabase,
};
use crate::models::{agent::AgentType, WorkTaskDraft};
use chrono::Utc;
use sea_orm::{ConnectionTrait, DatabaseConnection};
use sea_orm_migration::{MigratorTrait, SchemaManager};
use serde_json::json;

async fn seeded() -> (AppDatabase, IssueDraftV1, RepositoryBinding) {
    seeded_with(fresh_in_memory_db().await).await
}

async fn seeded_with(db: AppDatabase) -> (AppDatabase, IssueDraftV1, RepositoryBinding) {
    let folder = seed_folder(&db, "/synthetic/intake-project").await;
    let conversation = seed_conversation(&db, folder, AgentType::Codex).await;
    let task = tasks::create(&db.conn, WorkTaskDraft { folder_id: folder, title:"triage".into(),
        config:json!({"display_text":"triage", "prompt_blocks":[{"type":"text","text":"triage"}]}) }).await.unwrap();
    let run_seq = tasks::claim_for_run(&db.conn, task.id, WorkTaskStatus::Todo, "user")
        .await
        .unwrap()
        .unwrap();
    assert!(tasks::begin_setup(&db.conn, task.id, run_seq)
        .await
        .unwrap());
    assert!(
        tasks::mark_running(&db.conn, task.id, run_seq, conversation, "fixture")
            .await
            .unwrap()
    );
    let mut b = github::tests::binding();
    b.folder_id = folder;
    configure_repository(&db.conn, &b).await.unwrap();
    let source_ref = SourceRef {
        product_id: "hafidh".into(),
        source: SourceKind::Testflight,
        ulid: "01ARZ3NDEKTSV4RRFFQ69G5FAV".into(),
    };
    let source_revision = types::digest("synthetic source revision");
    record_source(
        &db.conn,
        &source_ref,
        &source_revision,
        Utc::now().timestamp(),
    )
    .await
    .unwrap();
    let mut evidence = vec![];
    for (field, value, provenance) in [
        (EvidenceField::Build, "42", EvidenceProvenance::AscBuild),
        (
            EvidenceField::Screen,
            "Mushaf reader",
            EvidenceProvenance::HumanReport,
        ),
        (
            EvidenceField::Reciter,
            "Abdul Basit",
            EvidenceProvenance::HumanReport,
        ),
        (
            EvidenceField::Log,
            "audio stream stopped at verse 4",
            EvidenceProvenance::LocalDiagnostic,
        ),
    ] {
        evidence.push(
            attach_evidence(
                &db.conn,
                EvidenceAttachment {
                    source_ref: source_ref.clone(),
                    source_revision: source_revision.clone(),
                    field,
                    value: value.into(),
                    content: value.as_bytes().to_vec(),
                    captured_at: None,
                    session_ulid: None,
                    provenance,
                    expires_at: None,
                },
                "human-1",
            )
            .await
            .unwrap(),
        );
    }
    let draft = IssueDraftV1 {
        schema_version: 1,
        template_version: TEMPLATE_VERSION.into(),
        task_id: task.id,
        run_seq,
        source_ref,
        source_revision,
        title: "Audio stopped in Mushaf".into(),
        summary: "Play the selected verse; playback stops early.".into(),
        labels: vec!["audio".into(), "severity:medium".into()],
        evidence: EvidenceSet {
            build: evidence[0].clone(),
            screen: evidence[1].clone(),
            reciter: evidence[2].clone(),
            log: evidence[3].clone(),
        },
    };
    set_scope(&db.conn, "act_low_risk").await;
    (db, draft, b)
}

async fn set_scope(conn: &DatabaseConnection, mode: &str) {
    conn.execute(store::sql(
        "DELETE FROM ops_agent_scope WHERE agent_id='intake-agent'",
        vec![],
    ))
    .await
    .unwrap();
    conn.execute(store::sql("INSERT INTO ops_agent_scope(agent_id,domain,resource,mode) VALUES('intake-agent','github','owner/repo',?)", vec![mode.into()])).await.unwrap();
}
async fn pending(conn: &DatabaseConnection, p: &PreparedIssue) -> i32 {
    match approvals::propose(
        conn,
        p.draft.task_id,
        p.draft.run_seq,
        "intake-agent",
        &GithubIssueAction,
        p.payload().unwrap(),
    )
    .await
    .unwrap()
    {
        approvals::ProposalOutcome::Pending(row) => row.id,
        _ => panic!("GitHub creates always need human approval"),
    }
}
async fn authorized(conn: &DatabaseConnection, p: &PreparedIssue) -> AuthorizedAction {
    let id = pending(conn, p).await;
    approvals::approve(
        conn,
        p.draft.task_id,
        p.draft.run_seq,
        id,
        "human-1",
        &GithubIssueAction,
        approvals::Review {
            expected_payload: &p.payload().unwrap(),
            approved_payload: p.payload().unwrap(),
        },
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn all_four_evidence_fields_and_renderer_are_mandatory() {
    let (db, draft, _) = seeded().await;
    let p = prepare(&db.conn, draft).await.unwrap();
    for field in ["build", "screen", "reciter", "log"] {
        let mut value = p.payload().unwrap();
        value["draft"]["evidence"]
            .as_object_mut()
            .unwrap()
            .remove(field);
        assert!(GithubIssueAction.validate(&value).is_err());
        for placeholder in [
            "",
            "unknown",
            "N/A",
            "TBD",
            "dummy",
            "https://private.invalid/log",
        ] {
            let mut value = p.payload().unwrap();
            value["draft"]["evidence"][field]["value"] = json!(placeholder);
            assert!(GithubIssueAction.validate(&value).is_err());
        }
    }
    let mut tampered = p.payload().unwrap();
    tampered["outgoing"]["body"] = json!("changed after render");
    assert!(GithubIssueAction.validate(&tampered).is_err());
    let mut extra = p.payload().unwrap();
    extra["approved"] = json!(true);
    assert!(GithubIssueAction.validate(&extra).is_err());
    assert!(
        p.outgoing.body.contains("## build")
            && p.outgoing.body.contains("## screen")
            && p.outgoing.body.contains("## reciter")
            && p.outgoing.body.contains("## log")
    );
}

#[tokio::test]
async fn evidence_bytes_revision_expiry_and_product_binding_are_rechecked() {
    for mutation in [
        "content", "revision", "expiry", "disabled", "folder", "stale",
    ] {
        let (db, draft, mut b) = seeded().await;
        let p = prepare(&db.conn, draft).await.unwrap();
        match mutation {
            "content" => {
                db.conn
                    .execute(store::sql(
                        "UPDATE ops_intake_evidence SET content=? WHERE artifact_id=?",
                        vec![
                            b"tampered".to_vec().into(),
                            p.draft.evidence.log.artifact_id.clone().into(),
                        ],
                    ))
                    .await
                    .unwrap();
            }
            "revision" => {
                record_source(
                    &db.conn,
                    &p.draft.source_ref,
                    &types::digest("changed"),
                    Utc::now().timestamp(),
                )
                .await
                .unwrap();
            }
            "expiry" => {
                db.conn
                    .execute(store::sql(
                        "UPDATE ops_intake_evidence SET expires_at=0",
                        vec![],
                    ))
                    .await
                    .unwrap();
            }
            "disabled" => {
                b.enabled = false;
                configure_repository(&db.conn, &b).await.unwrap();
            }
            "folder" => {
                b.folder_id = seed_folder(&db, "/synthetic/other-project").await;
                configure_repository(&db.conn, &b).await.unwrap();
            }
            _ => {
                db.conn
                    .execute(store::sql(
                        "UPDATE ops_intake_source SET fetched_at=0",
                        vec![],
                    ))
                    .await
                    .unwrap();
            }
        }
        assert!(prepare(&db.conn, p.draft).await.is_err(), "{mutation}");
    }
}

#[tokio::test]
async fn read_scope_denied_and_allow_rule_cannot_skip_human_floor() {
    let (db, draft, _) = seeded().await;
    let p = prepare(&db.conn, draft).await.unwrap();
    set_scope(&db.conn, "read").await;
    assert!(matches!(
        approvals::propose(
            &db.conn,
            p.draft.task_id,
            p.draft.run_seq,
            "intake-agent",
            &GithubIssueAction,
            p.payload().unwrap()
        )
        .await
        .unwrap(),
        approvals::ProposalOutcome::Denied(_)
    ));
    set_scope(&db.conn, "act_low_risk").await;
    db.conn.execute(store::sql("INSERT INTO ops_agent_rule(agent_id,domain,behavior) VALUES('intake-agent','github','allow')",vec![])).await.unwrap();
    let id = pending(&db.conn, &p).await;
    assert!(approvals::approve(
        &db.conn,
        p.draft.task_id,
        p.draft.run_seq,
        id,
        "intake-agent",
        &GithubIssueAction,
        approvals::Review {
            expected_payload: &p.payload().unwrap(),
            approved_payload: p.payload().unwrap()
        }
    )
    .await
    .is_err());
    set_scope(&db.conn, "read").await;
    assert!(approvals::approve(
        &db.conn,
        p.draft.task_id,
        p.draft.run_seq,
        id,
        "human-1",
        &GithubIssueAction,
        approvals::Review {
            expected_payload: &p.payload().unwrap(),
            approved_payload: p.payload().unwrap()
        }
    )
    .await
    .is_err());
}

#[tokio::test]
async fn stale_review_and_stale_run_never_produce_dispatch() {
    let (db, draft, _) = seeded().await;
    let p = prepare(&db.conn, draft).await.unwrap();
    let id = pending(&db.conn, &p).await;
    let mut stale = p.payload().unwrap();
    stale["draft"]["title"] = json!("stale card");
    assert!(approvals::approve(
        &db.conn,
        p.draft.task_id,
        p.draft.run_seq,
        id,
        "human-1",
        &GithubIssueAction,
        approvals::Review {
            expected_payload: &stale,
            approved_payload: p.payload().unwrap()
        }
    )
    .await
    .is_err());
    db.conn
        .execute(store::sql(
            "UPDATE work_task SET run_seq=run_seq+1 WHERE id=?",
            vec![p.draft.task_id.into()],
        ))
        .await
        .unwrap();
    assert!(approvals::approve(
        &db.conn,
        p.draft.task_id,
        p.draft.run_seq,
        id,
        "human-1",
        &GithubIssueAction,
        approvals::Review {
            expected_payload: &p.payload().unwrap(),
            approved_payload: p.payload().unwrap()
        }
    )
    .await
    .is_err());
}

#[tokio::test]
async fn exact_edited_approval_payload_is_sent_and_labels_mismatch_is_created() {
    let (db, draft, _) = seeded().await;
    let original = prepare(&db.conn, draft).await.unwrap();
    let id = pending(&db.conn, &original).await;
    let mut edited = original.draft.clone();
    edited.title = "Human edited title".into();
    let p = prepare(&db.conn, edited).await.unwrap();
    let auth = approvals::approve(
        &db.conn,
        p.draft.task_id,
        p.draft.run_seq,
        id,
        "human-1",
        &GithubIssueAction,
        approvals::Review {
            expected_payload: &original.payload().unwrap(),
            approved_payload: p.payload().unwrap(),
        },
    )
    .await
    .unwrap();
    let fixture = Fixture::new(Mode::LabelsDropped).await;
    let result = dispatch(&db.conn, &fixture.client, auth).await.unwrap();
    assert_eq!(result.state, FilingState::Created);
    assert!(!result.issue.as_ref().unwrap().labels_match);
    assert_eq!(
        fixture.observed.lock().unwrap().bodies,
        vec![serde_json::to_value(&p.outgoing).unwrap()]
    );
    let status = filing_status(&db.conn, &p.draft.source_ref, p.repository_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(status.state, FilingState::Created);
    assert_eq!(fixture.observed.lock().unwrap().issue_posts, 1);
}

#[tokio::test]
async fn response_lost_is_durable_unknown_blocks_resend_and_get_reconciles() {
    let (db, draft, _) = seeded().await;
    let p = prepare(&db.conn, draft).await.unwrap();
    let fixture = Fixture::new(Mode::Lost).await;
    let auth = authorized(&db.conn, &p).await;
    let result = dispatch(&db.conn, &fixture.client, auth).await.unwrap();
    assert_eq!(result.state, FilingState::Unknown);
    let again = authorized(&db.conn, &p).await;
    assert_eq!(
        dispatch(&db.conn, &fixture.client, again)
            .await
            .unwrap_err(),
        IntakeError::Conflict
    );
    assert_eq!(fixture.observed.lock().unwrap().issue_posts, 1);
    let reconciled = reconcile_filing(&db.conn, &fixture.client, result.attempt_id)
        .await
        .unwrap();
    assert_eq!(reconciled.state, FilingState::Created);
    assert_eq!(fixture.observed.lock().unwrap().issue_posts, 1);
}

#[tokio::test]
async fn ambiguous_http_outcomes_never_automatically_retry() {
    for mode in [Mode::ServerError, Mode::Malformed, Mode::Redirect] {
        let (db, draft, _) = seeded().await;
        let p = prepare(&db.conn, draft).await.unwrap();
        let fixture = Fixture::new(mode).await;
        let auth = authorized(&db.conn, &p).await;
        assert_eq!(
            dispatch(&db.conn, &fixture.client, auth)
                .await
                .unwrap()
                .state,
            FilingState::Unknown
        );
        assert_eq!(fixture.observed.lock().unwrap().issue_posts, 1);
    }
}

#[tokio::test]
async fn definite_rejection_requires_new_approval_and_preserves_attempt_history() {
    let (db, draft, _) = seeded().await;
    let p = prepare(&db.conn, draft).await.unwrap();
    let fixture = Fixture::new(Mode::Reject).await;
    let auth = authorized(&db.conn, &p).await;
    let receipt = dispatch(&db.conn, &fixture.client, auth).await.unwrap();
    assert_eq!(receipt.state, FilingState::Failed);
    assert_eq!(receipt.error_code.as_deref(), Some("rejected_payload"));
    assert!(!serde_json::to_string(&receipt).unwrap().contains("PRIVATE"));
    let auth = authorized(&db.conn, &p).await;
    let next = dispatch(&db.conn, &fixture.client, auth).await.unwrap();
    assert_ne!(receipt.attempt_id, next.attempt_id);
    assert_eq!(fixture.observed.lock().unwrap().issue_posts, 2);
}

#[tokio::test]
async fn missing_config_or_revoked_evidence_cannot_send() {
    let (db, draft, _) = seeded().await;
    let p = prepare(&db.conn, draft).await.unwrap();
    let auth = authorized(&db.conn, &p).await;
    assert_eq!(
        dispatch(&db.conn, &GithubAppClient::new(None).unwrap(), auth)
            .await
            .unwrap_err(),
        IntakeError::NotConfigured
    );
    let auth = authorized(&db.conn, &p).await;
    revoke_evidence(&db.conn, &p.draft.evidence.log.artifact_id)
        .await
        .unwrap();
    let fixture = Fixture::new(Mode::Success).await;
    assert_eq!(
        dispatch(&db.conn, &fixture.client, auth).await.unwrap_err(),
        IntakeError::InvalidEvidence
    );
    assert_eq!(fixture.observed.lock().unwrap().token_requests, 0);
    assert_eq!(fixture.observed.lock().unwrap().issue_posts, 0);
}

#[tokio::test]
async fn concurrent_authorized_filings_reserve_only_one_post() {
    let (db, draft, _) = seeded().await;
    let p = prepare(&db.conn, draft).await.unwrap();
    let first = authorized(&db.conn, &p).await;
    let second = authorized(&db.conn, &p).await;
    let fixture1 = Fixture::new(Mode::Success).await;
    let fixture2 = Fixture::new(Mode::Success).await;
    let (a, b) = tokio::join!(
        dispatch(&db.conn, &fixture1.client, first),
        dispatch(&db.conn, &fixture2.client, second)
    );
    assert_ne!(a.is_ok(), b.is_ok());
    assert_eq!(
        fixture1.observed.lock().unwrap().issue_posts
            + fixture2.observed.lock().unwrap().issue_posts,
        1
    );
}

#[tokio::test]
async fn reconciliation_with_no_match_multiple_matches_or_pr_stays_unknown() {
    for count in [0, 2, 3] {
        let (db, draft, _) = seeded().await;
        let p = prepare(&db.conn, draft).await.unwrap();
        let fixture = Fixture::new(Mode::Malformed).await;
        let auth = authorized(&db.conn, &p).await;
        let receipt = dispatch(&db.conn, &fixture.client, auth).await.unwrap();
        {
            let mut seen = fixture.observed.lock().unwrap();
            let issue = seen.issues[0].clone();
            seen.issues.clear();
            for n in 0..count {
                let mut item = issue.clone();
                item["id"] = json!(n + 1);
                if count == 3 {
                    item["pull_request"] = json!({});
                }
                seen.issues.push(item);
            }
        }
        assert_eq!(
            reconcile_filing(&db.conn, &fixture.client, receipt.attempt_id)
                .await
                .unwrap()
                .state,
            FilingState::Unknown
        );
        assert_eq!(fixture.observed.lock().unwrap().issue_posts, 1);
    }
}

#[tokio::test]
async fn reserved_migration_round_trip_preserves_other_modules() {
    let db = fresh_in_memory_db().await;
    db.conn.execute_unprepared("CREATE TABLE intake_test_follower(marker TEXT); INSERT INTO intake_test_follower VALUES('preserved')").await.unwrap();
    let migration = crate::db::migration::Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260907_000005_ops_intake")
        .unwrap();
    let manager = SchemaManager::new(&db.conn);
    migration.down(&manager).await.unwrap();
    migration.up(&manager).await.unwrap();
    assert_eq!(
        db.conn
            .query_one(store::sql(
                "SELECT marker FROM intake_test_follower",
                vec![]
            ))
            .await
            .unwrap()
            .unwrap()
            .try_get::<String>("", "marker")
            .unwrap(),
        "preserved"
    );
    assert!(db
        .conn
        .query_one(store::sql(
            "SELECT name FROM sqlite_master WHERE name='ops_proposal'",
            vec![]
        ))
        .await
        .unwrap()
        .is_some());
}

#[tokio::test]
async fn evidence_requires_actual_bytes_explicit_session_and_build_provenance() {
    let (db, draft, _) = seeded().await;
    for case in ["empty", "uri", "session", "marketing", "private", "non_log"] {
        let mut a = EvidenceAttachment {
            source_ref: draft.source_ref.clone(),
            source_revision: draft.source_revision.clone(),
            field: EvidenceField::Log,
            value: "audio stopped".into(),
            content: b"audio stopped".to_vec(),
            captured_at: None,
            session_ulid: None,
            provenance: EvidenceProvenance::LocalDiagnostic,
            expires_at: None,
        };
        match case {
            "empty" => a.content.clear(),
            "uri" => {
                a.value = "https://private.invalid/log".into();
                a.content = a.value.as_bytes().to_vec();
            }
            "session" => a.provenance = EvidenceProvenance::SessionDiagnostic,
            "marketing" => {
                a.field = EvidenceField::Build;
                a.provenance = EvidenceProvenance::HumanReport;
                a.value = "1.2.3".into();
                a.content = a.value.as_bytes().to_vec();
            }
            "private" => a.content = b"audio stopped Bearer PRIVATE".to_vec(),
            _ => a.provenance = EvidenceProvenance::HumanReport,
        }
        assert_eq!(
            attach_evidence(&db.conn, a, "human-1").await.unwrap_err(),
            IntakeError::InvalidEvidence,
            "{case}"
        );
    }
}

#[tokio::test]
async fn reservation_survives_database_reopen_without_capability_replay() {
    // Test output belongs to this worktree's target directory.
    let dir = tempfile::tempdir_in("target").unwrap();
    let (db, draft, _) =
        seeded_with(crate::db::test_helpers::fresh_disk_db(dir.path()).await).await;
    let p = prepare(&db.conn, draft).await.unwrap();
    let auth = authorized(&db.conn, &p).await;
    let proposal_id = auth.proposal_id();
    let exact = prepared(&auth.into_payload()).unwrap();
    let id = store::reserve(&db.conn, proposal_id, &exact).await.unwrap();
    db.conn.close().await.unwrap(); // Simulate process loss after reservation.
    let reopened = sea_orm::Database::connect(format!(
        "sqlite:{}?mode=rw",
        dir.path().join("source.db").display()
    ))
    .await
    .unwrap();
    let receipt = filing_status(&reopened, &p.draft.source_ref, p.repository_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(receipt.attempt_id, id);
    assert_eq!(receipt.state, FilingState::Unknown);
    assert_eq!(
        store::reserve(&reopened, proposal_id, &p)
            .await
            .unwrap_err(),
        IntakeError::Conflict
    );
    reopened.close().await.unwrap();
}

#[tokio::test]
async fn missing_label_and_policy_change_after_approval_do_not_send() {
    let (db, mut draft, _) = seeded().await;
    draft.labels = vec!["not-an-existing-label".into()];
    let p = prepare(&db.conn, draft).await.unwrap();
    let fixture = Fixture::new(Mode::Success).await;
    let auth = authorized(&db.conn, &p).await;
    assert_eq!(
        dispatch(&db.conn, &fixture.client, auth).await.unwrap_err(),
        IntakeError::InvalidPayload
    );
    assert_eq!(fixture.observed.lock().unwrap().issue_posts, 0);
    let mut draft = p.draft.clone();
    draft.labels.clear();
    let p = prepare(&db.conn, draft).await.unwrap();
    let auth = authorized(&db.conn, &p).await;
    db.conn.execute(store::sql("INSERT INTO ops_agent_rule(agent_id,domain,behavior) VALUES('intake-agent','github','deny')",vec![])).await.unwrap();
    assert_eq!(
        dispatch(&db.conn, &fixture.client, auth).await.unwrap_err(),
        IntakeError::AccessDenied
    );
    assert_eq!(fixture.observed.lock().unwrap().issue_posts, 0);
}

#[tokio::test]
async fn authentication_failures_and_durable_rate_deadline_are_safe() {
    for mode in [Mode::TokenDenied, Mode::WriteDenied, Mode::RateLimit] {
        let (db, draft, _) = seeded().await;
        let p = prepare(&db.conn, draft).await.unwrap();
        let fixture = Fixture::new(mode).await;
        let auth = authorized(&db.conn, &p).await;
        let result = dispatch(&db.conn, &fixture.client, auth).await;
        if mode == Mode::TokenDenied {
            assert_eq!(result.unwrap_err(), IntakeError::AccessDenied);
            assert_eq!(fixture.observed.lock().unwrap().issue_posts, 0);
        } else {
            let receipt = result.unwrap();
            assert_eq!(receipt.state, FilingState::Failed);
            assert_eq!(fixture.observed.lock().unwrap().issue_posts, 1);
            if mode == Mode::RateLimit {
                assert!(receipt.retry_after.unwrap() > Utc::now().timestamp());
                let fresh = Fixture::new(Mode::Success).await;
                let auth = authorized(&db.conn, &p).await;
                assert_eq!(
                    dispatch(&db.conn, &fresh.client, auth).await.unwrap_err(),
                    IntakeError::UpstreamUnavailable
                );
                assert!(fresh.observed.lock().unwrap().requests.is_empty());
            }
        }
    }
}

#[tokio::test]
async fn ask_rule_cannot_queue_stale_bound_evidence() {
    for mutation in ["deleted", "expired", "disabled", "revision"] {
        let (db, draft, mut b) = seeded().await;
        let p = prepare(&db.conn, draft).await.unwrap();
        db.conn.execute(store::sql("INSERT INTO ops_agent_rule(agent_id,domain,behavior) VALUES('intake-agent','github','ask')",vec![])).await.unwrap();
        match mutation {
            "deleted" => revoke_evidence(&db.conn, &p.draft.evidence.log.artifact_id)
                .await
                .unwrap(),
            "expired" => {
                db.conn
                    .execute_unprepared("UPDATE ops_intake_evidence SET expires_at=0")
                    .await
                    .unwrap();
            }
            "disabled" => {
                b.enabled = false;
                configure_repository(&db.conn, &b).await.unwrap();
            }
            _ => record_source(
                &db.conn,
                &p.draft.source_ref,
                &types::digest("new source"),
                Utc::now().timestamp(),
            )
            .await
            .unwrap(),
        }
        assert!(
            approvals::propose(
                &db.conn,
                p.draft.task_id,
                p.draft.run_seq,
                "intake-agent",
                &GithubIssueAction,
                p.payload().unwrap()
            )
            .await
            .is_err(),
            "{mutation}"
        );
        assert_eq!(
            db.conn
                .query_one(store::sql("SELECT COUNT(*) AS n FROM ops_proposal", vec![]))
                .await
                .unwrap()
                .unwrap()
                .try_get::<i64>("", "n")
                .unwrap(),
            0
        );
    }
}

#[tokio::test]
async fn missing_scope_defaults_to_propose_while_read_and_deny_stay_restricted() {
    let (db, draft, _) = seeded().await;
    let p = prepare(&db.conn, draft).await.unwrap();
    db.conn
        .execute_unprepared("DELETE FROM ops_agent_scope")
        .await
        .unwrap();
    let auth = authorized(&db.conn, &p).await; // no scope row still requires human
    let fixture = Fixture::new(Mode::Success).await;
    assert_eq!(
        dispatch(&db.conn, &fixture.client, auth)
            .await
            .unwrap()
            .state,
        FilingState::Created
    );
    for mode in ["read", "deny"] {
        let (db, draft, _) = seeded().await;
        let p = prepare(&db.conn, draft).await.unwrap();
        db.conn
            .execute_unprepared("DELETE FROM ops_agent_scope")
            .await
            .unwrap();
        if mode == "read" {
            set_scope(&db.conn, "read").await;
        } else {
            db.conn.execute_unprepared("INSERT INTO ops_agent_rule(agent_id,domain,behavior) VALUES('intake-agent','github','ask'),('intake-agent','github','deny')").await.unwrap();
        }
        assert!(
            matches!(
                approvals::propose(
                    &db.conn,
                    p.draft.task_id,
                    p.draft.run_seq,
                    "intake-agent",
                    &GithubIssueAction,
                    p.payload().unwrap()
                )
                .await
                .unwrap(),
                approvals::ProposalOutcome::Denied(_)
            ),
            "{mode}"
        );
    }
}

#[tokio::test]
async fn attachments_accept_canonical_backend_operator_labels_without_aliases() {
    let (db, draft, _) = seeded().await;
    for actor in ["operator:http", "operator:desktop"] {
        let proof = attach_evidence(
            &db.conn,
            EvidenceAttachment {
                source_ref: draft.source_ref.clone(),
                source_revision: draft.source_revision.clone(),
                field: EvidenceField::Screen,
                value: "Mushaf reader".into(),
                content: b"Mushaf reader".to_vec(),
                captured_at: None,
                session_ulid: None,
                provenance: EvidenceProvenance::HumanReport,
                expires_at: None,
            },
            actor,
        )
        .await
        .unwrap();
        let row = db
            .conn
            .query_one(store::sql(
                "SELECT reviewed_by FROM ops_intake_evidence WHERE artifact_id=?",
                vec![proof.artifact_id.into()],
            ))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(row.try_get::<String>("", "reviewed_by").unwrap(), actor);
    }
}
