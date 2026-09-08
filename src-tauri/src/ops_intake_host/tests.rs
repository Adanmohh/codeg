use super::{fix_task, operator, review, HostRuntime};
use super::{store::*, types::*};
use crate::db::service::work_task_service as tasks;
use crate::db::test_helpers::{fresh_in_memory_db, seed_folder};
use crate::models::{WorkTaskDraft, WorkTaskStatus};
use crate::ops::agent::RunContext;
use crate::ops_intake::{self, RepositoryBinding};
use sea_orm::ConnectionTrait;
use serde_json::json;

mod browser;
pub(crate) mod fixture;
mod http;
fn human() -> crate::ops::Operator {
    crate::ops::Operator::server().unwrap()
}

pub(crate) async fn start_task(db: &crate::db::AppDatabase, folder: i32) -> RunContext {
    let conversation = crate::db::test_helpers::seed_conversation(
        db,
        folder,
        crate::models::agent::AgentType::Codex,
    )
    .await;
    let task = tasks::create(&db.conn, WorkTaskDraft {folder_id:folder,title:"Synthetic triage task".into(),
        config:json!({"display_text":"Synthetic local triage","prompt_blocks":[{"type":"text","text":"Synthetic local triage"}]})}).await.unwrap();
    let seq = tasks::claim_for_run(&db.conn, task.id, WorkTaskStatus::Todo, "fixture")
        .await
        .unwrap()
        .unwrap();
    assert!(tasks::begin_setup(&db.conn, task.id, seq).await.unwrap());
    assert!(
        tasks::mark_running(&db.conn, task.id, seq, conversation, "synthetic-connection")
            .await
            .unwrap()
    );
    RunContext {
        account_id: 1,
        task_id: task.id,
        run_seq: seq,
        connection_id: "synthetic-connection".into(),
        agent_id: "synthetic-pi".into(),
    }
}

pub(crate) async fn ready(
    provider: &fixture::Provider,
) -> (crate::db::AppDatabase, SourceInput, RunContext, Draft) {
    let (db, source) = seeded().await;
    provider.configure(&db, &source).await;
    let page = operator::list(
        &db.conn,
        &human(),
        &provider.runtime,
        ListInput {
            product_id: source.product_id.clone(),
            cursor: None,
        },
    )
    .await
    .unwrap();
    assert_eq!(page.records.len(), 3);
    assert!(page.records.iter().all(|r| r.verified_at.is_none()));
    let (ctx, d) = ready_source(&db, provider, &source).await;
    (db, source, ctx, d)
}

// Reuse a fixture-owned runtime so independent databases never contend on the
// production singleton's per-product lock while testing parallel scenarios.
pub(crate) async fn propose_notice(
    db: &crate::db::AppDatabase,
    ctx: &RunContext,
    draft: &Draft,
    runtime: &HostRuntime,
) -> super::agent::Proposed {
    review::propose(&db.conn, ctx, &draft.id, draft.revision, runtime)
        .await
        .unwrap()
}

pub(crate) async fn ready_source(
    db: &crate::db::AppDatabase,
    provider: &fixture::Provider,
    source: &SourceInput,
) -> (RunContext, Draft) {
    let detail = operator::refresh(&db.conn, &human(), &provider.runtime, source.clone())
        .await
        .unwrap();
    let p = product(&db.conn, 1, &source.product_id).await.unwrap();
    let ctx = start_task(&db, p.binding.folder_id).await;
    let mut d = detail.draft;
    for (field, value) in [
        (ops_intake::EvidenceField::Build, "42"),
        (ops_intake::EvidenceField::Screen, "Mushaf reader"),
        (ops_intake::EvidenceField::Reciter, "Abdul Basit"),
        (
            ops_intake::EvidenceField::Log,
            "audio stopped at verse four",
        ),
    ] {
        d = operator::attach(
            &db.conn,
            &human(),
            &provider.runtime,
            AttachInput {
                source: source.clone(),
                expected_revision: d.revision,
                field,
                value: value.into(),
                content: format!("Synthetic human proof: {value}"),
                captured_at: None,
                session_ulid: None,
            },
        )
        .await
        .unwrap();
    }
    d = operator::save(
        &db.conn,
        &human(),
        &provider.runtime,
        SaveInput {
            source: source.clone(),
            expected_revision: d.revision,
            title: d.title,
            summary: d.summary,
            labels: vec!["audio".into()],
            confirmed_severity: Some(Severity::Medium),
        },
    )
    .await
    .unwrap();
    d = review::prepare(
        &db.conn,
        &human(),
        &provider.runtime,
        PrepareInput {
            source: source.clone(),
            expected_revision: d.revision,
            task_id: ctx.task_id,
        },
    )
    .await
    .unwrap();
    (ctx, d)
}

async fn filed(provider: &fixture::Provider) -> (crate::db::AppDatabase, SourceInput, Detail) {
    let (db, source, ctx, d) = ready(provider).await;
    let p = review::propose(&db.conn, &ctx, &d.id, d.revision, &provider.runtime)
        .await
        .unwrap();
    let detail = review::approve(
        &db.conn,
        &human(),
        &provider.runtime,
        ReviewInput {
            review_notice: None,
            source: source.clone(),
            proposal_id: p.proposal_id.unwrap(),
            expected_payload: p.prepared.clone(),
            approved_payload: p.prepared,
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        detail.receipt.as_ref().unwrap().state,
        ops_intake::FilingState::Created
    ));
    (db, source, detail)
}

fn record() -> Record {
    serde_json::from_value(json!({
        "schema_version":1,"source_ref":{"product_id":"synthetic-hafidh","source":"testflight","ulid":"01ARZ3NDEKTSV4RRFFQ69G5FAV","external_id":"asc-1"},
        "source_revision":"a".repeat(64),"fetched_at":chrono::Utc::now().to_rfc3339(),
        "title":"Synthetic audio report","title_is_draft":true,"description":"Synthetic playback stops at verse four.",
        "feedback_type":null,"source_status":"pending","submitted_at":null,"source_updated_at":chrono::Utc::now().to_rfc3339(),
        "device":"iPhone","os_version":"18","app_version":null,"build_number":"42","platform":"IOS","locale":"en",
        "screenshots":[],"triage":{"seeded_tags":["audio"],"seeded_severity":"medium","source_tags":[],"source_severity":"medium","confirmed_tags":null,"confirmed_severity":null},
        "evidence_candidates":[{"field":"build","value":"42","provenance":"testflight.buildVersion"}],"missing_required":["build","screen","reciter","log"]
    })).unwrap()
}
async fn seeded() -> (crate::db::AppDatabase, SourceInput) {
    seed_into(fresh_in_memory_db().await, "/synthetic/host").await
}
async fn seed_into(
    db: crate::db::AppDatabase,
    folder_path: &str,
) -> (crate::db::AppDatabase, SourceInput) {
    let folder = seed_folder(&db, folder_path).await;
    let binding = RepositoryBinding {
        product_id: "synthetic-hafidh".into(),
        folder_id: folder,
        app_id: "fixture-app".into(),
        installation_id: 7,
        repository_id: 11,
        full_name: "owner/repo".into(),
        enabled: true,
    };
    ops_intake::configure_repository(&db.conn, &binding)
        .await
        .unwrap();
    let p = StoredProduct {
        binding,
        origin: "https://synthetic.invalid".into(),
        bearer_ref: None,
        key_ref: None,
    };
    db.conn
        .execute(sql(
            "INSERT INTO ops_intake_host_product VALUES(?,1,?)",
            vec!["synthetic-hafidh".into(), encode(&p).unwrap().into()],
        ))
        .await
        .unwrap();
    let source = SourceInput {
        product_id: "synthetic-hafidh".into(),
        ulid: record().source_ref.ulid,
    };
    (db, source)
}

#[tokio::test]
async fn listing_never_mints_freshness_and_failed_get_revokes_old_freshness() {
    let (db, source) = seeded().await;
    let r = record();
    listed(&db.conn, &r).await.unwrap();
    assert_eq!(
        fresh(&snapshot(&db.conn, &source).await.unwrap()),
        Err(HostError::StaleSource)
    );
    assert!(db
        .conn
        .query_one(sql("SELECT source_key FROM ops_intake_source", vec![]))
        .await
        .unwrap()
        .is_none());
    ops_intake::record_source(
        &db.conn,
        &r.source_ref.source(),
        &r.source_revision,
        chrono::Utc::now().timestamp(),
    )
    .await
    .unwrap();
    invalidate(&db.conn, &source, HostError::AccessDenied)
        .await
        .unwrap();
    let row = db
        .conn
        .query_one(sql("SELECT fetched_at FROM ops_intake_source", vec![]))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<i64>("", "fetched_at").unwrap(), 0);
    assert_eq!(
        snapshot(&db.conn, &source).await.unwrap().error.as_deref(),
        Some("access_denied")
    );
}
#[tokio::test]
async fn revision_change_clears_human_confirmation_and_old_saved_draft_loses_cas() {
    let (db, source) = seeded().await;
    let mut r = record();
    listed(&db.conn, &r).await.unwrap();
    let mut draft = ensure_draft(&db.conn, &source).await.unwrap();
    draft.confirmed_severity = Some(Severity::Medium);
    let old = draft.clone();
    cas(&db.conn, &mut draft, old.revision).await.unwrap();
    let mut stale = old.clone();
    assert_eq!(
        cas(&db.conn, &mut stale, old.revision).await,
        Err(HostError::Conflict)
    );
    r.source_revision = "b".repeat(64);
    listed(&db.conn, &r).await.unwrap();
    let reset = ensure_draft(&db.conn, &source).await.unwrap();
    assert!(
        reset.confirmed_severity.is_none() && reset.proofs.is_empty() && reset.prepared.is_none()
    );
    assert!(reset.revision > draft.revision);
}
#[tokio::test]
async fn product_account_binding_and_disabled_repository_fail_closed() {
    let (db, _) = seeded().await;
    assert!(matches!(
        enabled(&db.conn, 2, "synthetic-hafidh").await,
        Err(HostError::AccessDenied)
    ));
    let mut p = product(&db.conn, 1, "synthetic-hafidh").await.unwrap();
    p.binding.full_name = "other/repo".into();
    ops_intake::configure_repository(&db.conn, &p.binding)
        .await
        .unwrap();
    assert!(matches!(
        enabled(&db.conn, 1, "synthetic-hafidh").await,
        Err(HostError::NotConfigured)
    ));
}
#[test]
fn raw_fields_and_client_claimed_source_confirmation_are_rejected() {
    let r = record();
    let mut raw = serde_json::to_value(r).unwrap();
    raw["testerEmail"] = json!("private@example.invalid");
    assert!(serde_json::from_value::<Record>(raw).is_err());
    let mut r = record();
    r.triage.confirmed_severity = Some(Severity::High);
    assert_eq!(
        validate_record(&r, "synthetic-hafidh"),
        Err(HostError::SourceUnavailable)
    );
}

#[tokio::test]
async fn pending_proposal_survives_repeated_call_and_overlapping_acp_wait() {
    use crate::db::service::work_task_wait_service as waits;
    let provider = fixture::Provider::start().await;
    let (db, source, ctx, d) = ready(&provider).await;
    let first = review::propose(&db.conn, &ctx, &d.id, d.revision, &provider.runtime)
        .await
        .unwrap();
    waits::track_request(
        &db.conn,
        ctx.task_id,
        ctx.run_seq,
        &ctx.connection_id,
        "p:overlap",
        true,
    )
    .await
    .unwrap();
    let again = review::propose(&db.conn, &ctx, &d.id, d.revision, &provider.runtime)
        .await
        .unwrap();
    assert_eq!(first.proposal_id, again.proposal_id);
    assert_eq!(draft(&db.conn, &source).await.unwrap().revision, d.revision);
    assert_eq!(first.prepared, again.prepared);
    review::deny(
        &db.conn,
        &human(),
        &provider.runtime,
        DenyInput {
            review_notice: None,
            source,
            proposal_id: first.proposal_id.unwrap(),
            expected_payload: first.prepared,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        tasks::get(&db.conn, ctx.task_id).await.unwrap().status,
        WorkTaskStatus::AwaitingInput
    );
    assert!(waits::track_request(
        &db.conn,
        ctx.task_id,
        ctx.run_seq,
        &ctx.connection_id,
        "p:overlap",
        false
    )
    .await
    .unwrap());
    assert_eq!(
        tasks::get(&db.conn, ctx.task_id).await.unwrap().status,
        WorkTaskStatus::Running
    );
    assert_eq!(provider.seen.lock().unwrap().posts, 0);
}

#[tokio::test]
async fn pending_reuse_rechecks_connection_canceled_run_and_evidence() {
    for mutation in ["connection", "canceled", "run", "deleted", "evidence"] {
        let provider = fixture::Provider::start().await;
        let (db, source, ctx, d) = ready(&provider).await;
        review::propose(&db.conn, &ctx, &d.id, d.revision, &provider.runtime)
            .await
            .unwrap();
        let query = match mutation {
            "connection" => "UPDATE work_task SET connection_id='other'",
            "canceled" => "UPDATE work_task SET status='canceled'",
            "run" => "UPDATE work_task SET run_seq=run_seq+1",
            "deleted" => "UPDATE work_task SET deleted_at=CURRENT_TIMESTAMP",
            _ => "DELETE FROM ops_intake_evidence",
        };
        db.conn.execute_unprepared(query).await.unwrap();
        assert!(
            review::propose(&db.conn, &ctx, &d.id, d.revision, &provider.runtime)
                .await
                .is_err(),
            "{mutation}"
        );
        assert_eq!(draft(&db.conn, &source).await.unwrap().revision, d.revision);
    }
}

#[tokio::test]
async fn unrelated_awaiting_input_does_not_revise_a_draft() {
    let provider = fixture::Provider::start().await;
    let (db, source, ctx, d) = ready(&provider).await;
    crate::db::service::work_task_wait_service::track_request(
        &db.conn,
        ctx.task_id,
        ctx.run_seq,
        &ctx.connection_id,
        "p:question",
        true,
    )
    .await
    .unwrap();
    assert!(matches!(
        review::propose(&db.conn, &ctx, &d.id, d.revision, &provider.runtime).await,
        Err(HostError::TaskRequired)
    ));
    assert_eq!(draft(&db.conn, &source).await.unwrap().revision, d.revision);
}

#[tokio::test]
async fn held_fix_is_discoverable_scheduler_safe_and_manually_requeueable() {
    let provider = fixture::Provider::start().await;
    let (db, source, _) = filed(&provider).await;
    let result = fix_task::create(&db.conn, &human(), &provider.runtime, source.clone())
        .await
        .unwrap();
    let id = result.fix_task_id.unwrap();
    let task = tasks::get(&db.conn, id).await.unwrap();
    assert_eq!(task.status, WorkTaskStatus::Canceled);
    assert_eq!(task.run_seq, 0);
    assert!(!tasks::folders_with_pending(&db.conn)
        .await
        .unwrap()
        .contains(&task.folder_id));
    assert_eq!(
        fix_task::create(&db.conn, &human(), &provider.runtime, source.clone())
            .await
            .unwrap()
            .fix_task_id,
        Some(id)
    );
    assert!(tasks::requeue_canceled(&db.conn, id, None, &[], false)
        .await
        .unwrap());
    assert_eq!(
        tasks::get(&db.conn, id).await.unwrap().status,
        WorkTaskStatus::Todo
    );
    // Global source dedup can reuse this active task when its owned link is absent.
    db.conn
        .execute_unprepared("DELETE FROM ops_intake_host_fix")
        .await
        .unwrap();
    assert_eq!(
        fix_task::create(&db.conn, &human(), &provider.runtime, source)
            .await
            .unwrap()
            .fix_task_id,
        Some(id)
    );
}

#[tokio::test]
async fn global_source_duplicate_in_foreign_folder_is_a_conflict() {
    let provider = fixture::Provider::start().await;
    let (db, source, _) = filed(&provider).await;
    let id = fix_task::create(&db.conn, &human(), &provider.runtime, source.clone())
        .await
        .unwrap()
        .fix_task_id
        .unwrap();
    db.conn
        .execute_unprepared("DELETE FROM ops_intake_host_fix")
        .await
        .unwrap();
    let foreign = seed_folder(&db, "/synthetic/foreign").await;
    db.conn
        .execute(sql(
            "UPDATE work_task SET status='todo',folder_id=? WHERE id=?",
            vec![foreign.into(), id.into()],
        ))
        .await
        .unwrap();
    assert!(matches!(
        fix_task::create(&db.conn, &human(), &provider.runtime, source).await,
        Err(HostError::Conflict)
    ));
    assert!(db
        .conn
        .query_one(sql("SELECT * FROM ops_intake_host_fix", vec![]))
        .await
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn existing_fix_link_rechecks_rebinding_source_receipt_and_product_account() {
    for mutation in [
        "folder",
        "repository",
        "installation",
        "task_source",
        "receipt",
        "account",
    ] {
        let provider = fixture::Provider::start().await;
        let (db, source, _) = filed(&provider).await;
        let id = fix_task::create(&db.conn, &human(), &provider.runtime, source.clone())
            .await
            .unwrap()
            .fix_task_id
            .unwrap();
        let mut p = product(&db.conn, 1, &source.product_id).await.unwrap();
        match mutation {
            "folder" => p.binding.folder_id = seed_folder(&db, "/synthetic/rebound").await,
            "repository" => {
                p.binding.repository_id = 12;
                p.binding.full_name = "owner/changed".into();
            }
            "installation" => p.binding.installation_id = 8,
            "task_source" => {
                db.conn
                    .execute(sql(
                        "UPDATE work_task SET source_key='other' WHERE id=?",
                        vec![id.into()],
                    ))
                    .await
                    .unwrap();
            }
            "receipt" => {
                db.conn
                    .execute_unprepared(
                        "UPDATE ops_intake_filing SET state='unknown',issue_json=NULL",
                    )
                    .await
                    .unwrap();
            }
            _ => {
                db.conn
                    .execute_unprepared("UPDATE ops_intake_host_product SET account_id=2")
                    .await
                    .unwrap();
            }
        }
        if matches!(mutation, "folder" | "repository" | "installation") {
            operator::configure(
                &db.conn,
                &human(),
                &provider.runtime,
                ConfigureInput {
                    binding: p.binding,
                    origin: p.origin,
                    intake_bearer: None,
                    app_private_key: None,
                },
            )
            .await
            .unwrap();
        }
        let result = operator::detail(&db.conn, &human(), source.clone()).await;
        if mutation == "account" {
            assert!(matches!(result, Err(HostError::AccessDenied)));
        } else {
            let d = result.unwrap();
            assert!(d.fix_task_id.is_none() && d.fix_task_conflict, "{mutation}");
        }
        assert!(
            fix_task::create(&db.conn, &human(), &provider.runtime, source)
                .await
                .is_err(),
            "{mutation}"
        );
    }
}
