//! Synthetic human preparation, shared with full token/companion fixtures.
//! No source/provider process is invoked. Source timestamps here are test setup,
//! never an agent-facing operation or a replacement for human refresh.
use super::*;
use crate::ops_intake::{self, EvidenceField, RepositoryBinding};

pub(crate) async fn human_prepared(
    db: &DatabaseConnection,
    ctx: &RunContext,
) -> (SourceInput, Draft) {
    let folder = db
        .query_one(store::sql(
            "SELECT folder_id FROM work_task WHERE id=?",
            vec![ctx.task_id.into()],
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get::<i32>("", "folder_id")
        .unwrap();
    let product_id = format!("synthetic-{}", uuid::Uuid::new_v4());
    let binding = RepositoryBinding {
        product_id: product_id.clone(),
        folder_id: folder,
        app_id: "synthetic-app".into(),
        installation_id: 7,
        repository_id: 11,
        full_name: "owner/repo".into(),
        enabled: true,
    };
    ops_intake::configure_repository(db, &binding)
        .await
        .unwrap();
    let p = StoredProduct {
        binding,
        origin: "https://PRIVATE-ORIGIN.invalid".into(),
        bearer_ref: Some("PRIVATE-BEARER-REF".into()),
        key_ref: Some("PRIVATE-KEY-REF".into()),
    };
    db.execute(store::sql(
        "INSERT INTO ops_intake_host_product(product_id,account_id,config_json) VALUES(?,?,?)",
        vec![
            product_id.clone().into(),
            ctx.account_id.into(),
            store::encode(&p).unwrap().into(),
        ],
    ))
    .await
    .unwrap();
    let mut r = super::super::tests::record();
    r.source_ref.product_id = product_id.clone();
    r.source_ref.external_id = Some("PRIVATE-REPORTER-REF".into());
    r.evidence_candidates[0].provenance = "PRIVATE-CANDIDATE".into();
    let source = SourceInput {
        product_id,
        ulid: r.source_ref.ulid.clone(),
    };
    store::listed(db, &r).await.unwrap();
    let now = chrono::Utc::now().timestamp();
    ops_intake::record_source(db, &r.source_ref.source(), &r.source_revision, now)
        .await
        .unwrap();
    db.execute(store::sql(
        "UPDATE ops_intake_host_snapshot SET verified_at=? WHERE product_id=?",
        vec![now.into(), source.product_id.clone().into()],
    ))
    .await
    .unwrap();
    let draft = prepare_source(db, ctx, &source).await;
    (source, draft)
}

pub(crate) async fn prepare_source(
    db: &DatabaseConnection,
    ctx: &RunContext,
    source: &SourceInput,
) -> Draft {
    let human = crate::ops::Operator::server().unwrap();
    assert_eq!(human.account_id(), ctx.account_id);
    let runtime = HostRuntime::production();
    let mut draft = store::ensure_draft(db, source).await.unwrap();
    for (field, value) in [
        (EvidenceField::Build, "42"),
        (EvidenceField::Screen, "Mushaf reader"),
        (EvidenceField::Reciter, "Abdul Basit"),
        (EvidenceField::Log, "Playback stopped at verse four"),
    ] {
        draft = super::super::operator::attach(
            db,
            &human,
            &runtime,
            AttachInput {
                source: source.clone(),
                expected_revision: draft.revision,
                field,
                value: value.into(),
                content: format!("Synthetic human-reviewed evidence: {value}"),
                captured_at: None,
                session_ulid: None,
            },
        )
        .await
        .unwrap();
    }
    draft = super::super::operator::save(
        db,
        &human,
        &runtime,
        SaveInput {
            source: source.clone(),
            expected_revision: draft.revision,
            title: draft.title,
            summary: draft.summary,
            labels: vec!["audio".into()],
            confirmed_severity: Some(Severity::Medium),
        },
    )
    .await
    .unwrap();
    draft = super::super::review::prepare(
        db,
        &human,
        &runtime,
        PrepareInput {
            source: source.clone(),
            expected_revision: draft.revision,
            task_id: ctx.task_id,
        },
    )
    .await
    .unwrap();
    draft
}
