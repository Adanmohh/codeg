use super::{store::*, types::*, *};
use crate::db::test_helpers::{fresh_in_memory_db, seed_folder};
use crate::ops_intake::{self, RepositoryBinding};
use sea_orm::ConnectionTrait;
use serde_json::json;

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
    let db = fresh_in_memory_db().await;
    let folder = seed_folder(&db, "/synthetic/host").await;
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
