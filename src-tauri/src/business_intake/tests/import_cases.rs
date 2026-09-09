use super::{super::*, support::*};
use sea_orm::{ConnectionTrait, FromQueryResult, TransactionTrait};
use serde_json::json;
use std::sync::Arc;

fn next(row: &Import) -> ImportRevisionInput {
    ImportRevisionInput {
        operation_id: common::id(),
        import_id: row.id.clone(),
        expected_revision: row.revision,
    }
}
async fn start(f: &Fixture, b: &BindingAdmin) -> Import {
    imports_start(
        &f.db.conn,
        &f.op,
        &f.services,
        StartImportInput {
            operation_id: common::id(),
            binding_id: b.id.clone(),
            selection: Selection::Window {
                from_date: "2026-09-01T00:00:00Z".into(),
                to_date: "2026-09-08T00:00:00Z".into(),
            },
        },
    )
    .await
    .unwrap()
}
async fn record(f: &Fixture, b: &BindingAdmin, source_id: &str) -> Import {
    imports_start(
        &f.db.conn,
        &f.op,
        &f.services,
        StartImportInput {
            operation_id: common::id(),
            binding_id: b.id.clone(),
            selection: Selection::Record {
                source_id: source_id.into(),
            },
        },
    )
    .await
    .unwrap()
}
async fn discover(f: &Fixture, b: &BindingAdmin) -> (Import, String) {
    let import = start(f, b).await;
    f.mock
        .json(json!({"data":{"transcripts":[{"id":"record-1","title":"Feedback planning"}]}}));
    let import = imports_advance(&f.db.conn, &f.op, &f.services, next(&import))
        .await
        .unwrap();
    let sources = sources_list(
        &f.db.conn,
        &f.op,
        &f.services,
        BindingPageInput {
            binding_id: b.id.clone(),
            page: 0,
        },
    )
    .await
    .unwrap();
    (import, sources.items[0].id.clone())
}
async fn detail(f: &Fixture, source_id: &str) -> SourceDetail {
    sources_get(
        &f.db.conn,
        &f.op,
        &f.services,
        SourceInput {
            source_id: source_id.into(),
        },
    )
    .await
    .unwrap()
}

#[tokio::test]
async fn intake_import_discovery_replay_and_aba_versions_preserve_candidate_and_source_identity() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let first = start(&f, &b).await;
    let input = next(&first);
    let replay_op = input.operation_id.clone();
    f.mock
        .json(json!({"data":{"transcripts":[{"id":"record-1","title":"Feedback planning"}]}}));
    let listed = imports_advance(&f.db.conn, &f.op, &f.services, input)
        .await
        .unwrap();
    let replay = imports_advance(
        &f.db.conn,
        &f.op,
        &f.services,
        ImportRevisionInput {
            operation_id: replay_op,
            import_id: first.id.clone(),
            expected_revision: first.revision,
        },
    )
    .await
    .unwrap();
    assert_eq!(replay.revision, listed.revision);
    assert_eq!(f.mock.count(), 2);
    let source = sources_list(
        &f.db.conn,
        &f.op,
        &f.services,
        BindingPageInput {
            binding_id: b.id.clone(),
            page: 0,
        },
    )
    .await
    .unwrap()
    .items
    .remove(0);
    assert_eq!(source.revision, None);
    assert_eq!(source.access, AccessState::Unverified);
    assert!(detail(&f, &source.id).await.passages.is_empty());
    f.mock.json(transcript("record-1", "A exact text"));
    let completed = imports_advance(&f.db.conn, &f.op, &f.services, next(&listed))
        .await
        .unwrap();
    assert_eq!(completed.state, ImportState::Complete);
    assert_eq!(completed.completed, 1);
    let fresh = detail(&f, &source.id).await;
    assert_eq!(fresh.source.revision, Some(1));
    assert_eq!(fresh.disclosure, Disclosure::Fresh);
    assert_eq!(fresh.candidate_count, 1);
    let tx = f.db.conn.begin().await.unwrap();
    let candidate = records::Candidate::find_by_statement(common::sql(
        "SELECT * FROM business_intake_candidate WHERE source_id=?",
        vec![source.id.clone().into()],
    ))
    .one(&tx)
    .await
    .unwrap()
    .unwrap();
    assert!(candidate.draft_json.is_none());
    tx.rollback().await.unwrap();
    for (text, revision) in [
        ("A exact text", 1),
        ("B changed text", 2),
        ("A exact text", 3),
    ] {
        let import = record(&f, &b, &source.id).await;
        f.mock.json(transcript("record-1", text));
        imports_advance(&f.db.conn, &f.op, &f.services, next(&import))
            .await
            .unwrap();
        let read = detail(&f, &source.id).await;
        assert_eq!(read.source.id, source.id);
        assert_eq!(read.source.revision, Some(revision));
        assert_eq!(read.candidate_count, 1);
    }
    assert_eq!(count(&f.db.conn, "business_intake_version").await, 3);
    let row = f
        .db
        .conn
        .query_one(common::sql(
            "SELECT id,revision,source_revision FROM business_intake_candidate WHERE source_id=?",
            vec![source.id.clone().into()],
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(row.try_get::<String>("", "id").unwrap(), candidate.id);
    assert_eq!(row.try_get::<i64>("", "source_revision").unwrap(), 1);
    assert_eq!(row.try_get::<i64>("", "revision").unwrap(), 3);
    let history = imports_list(
        &f.db.conn,
        &f.op,
        &f.services,
        ImportsInput {
            binding_id: b.id.clone(),
            view: ImportView::All,
            page: 0,
        },
    )
    .await
    .unwrap();
    assert_eq!(history.items.len(), 4);
    assert!(imports_list(
        &f.db.conn,
        &f.op,
        &f.services,
        ImportsInput {
            binding_id: b.id,
            view: ImportView::Unfinished,
            page: 0
        }
    )
    .await
    .unwrap()
    .items
    .is_empty());
    assert_eq!(count(&f.db.conn, "business_task").await, 0);
}

#[tokio::test]
async fn intake_import_cancel_and_expired_claim_discard_late_detail_without_a_replacement() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let (import, source_id) = discover(&f, &b).await;
    let gate = Arc::new(tokio::sync::Barrier::new(2));
    let mut reply = Reply::json(transcript("record-1", "Late cancelled"));
    reply.gate = Some(gate.clone());
    f.mock.reply(reply);
    let future = imports_advance(&f.db.conn, &f.op, &f.services, next(&import));
    let cancel = async {
        f.mock.seen(3).await;
        let live = imports_get(
            &f.db.conn,
            &f.op,
            &f.services,
            ImportInput {
                import_id: import.id.clone(),
            },
        )
        .await
        .unwrap();
        assert!(!live.capabilities.advance);
        let cancelled = imports_cancel(&f.db.conn, &f.op, &f.services, next(&live))
            .await
            .unwrap();
        assert_eq!(cancelled.state, ImportState::Cancelled);
        gate.wait().await;
    };
    let (result, _) = tokio::join!(future, cancel);
    assert!(result.is_err());
    assert_eq!(count(&f.db.conn, "business_intake_version").await, 0);
    assert_eq!(
        detail(&f, &source_id).await.disclosure,
        Disclosure::MetadataOnly
    );
    let retry = record(&f, &b, &source_id).await;
    let gate = Arc::new(tokio::sync::Barrier::new(2));
    let mut reply = Reply::json(transcript("record-1", "Late expired"));
    reply.gate = Some(gate.clone());
    f.mock.reply(reply);
    let future = imports_advance(&f.db.conn, &f.op, &f.services, next(&retry));
    let expire = async {
        f.mock.seen(4).await;
        f.db.conn
            .execute(common::sql(
                "UPDATE business_intake_import SET lease_until=? WHERE id=?",
                vec!["2020-01-01T00:00:00Z".into(), retry.id.clone().into()],
            ))
            .await
            .unwrap();
        gate.wait().await;
    };
    let (result, _) = tokio::join!(future, expire);
    assert!(result.is_err());
    assert_eq!(count(&f.db.conn, "business_intake_version").await, 0);
    let expired = imports_get(
        &f.db.conn,
        &f.op,
        &f.services,
        ImportInput {
            import_id: retry.id,
        },
    )
    .await
    .unwrap();
    assert_eq!(expired.state, ImportState::Waiting);
    assert!(expired.capabilities.advance);
}

#[tokio::test]
async fn intake_source_fence_across_distinct_imports_rejects_older_response() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let (old, source_id) = discover(&f, &b).await;
    let new = record(&f, &b, &source_id).await;
    let gate = Arc::new(tokio::sync::Barrier::new(2));
    let mut reply = Reply::json(transcript("record-1", "Obsolete response"));
    reply.gate = Some(gate.clone());
    f.mock.reply(reply);
    f.mock.json(transcript("record-1", "Current response"));
    let slow = imports_advance(&f.db.conn, &f.op, &f.services, next(&old));
    let fast = async {
        f.mock.seen(3).await;
        let result = imports_advance(&f.db.conn, &f.op, &f.services, next(&new))
            .await
            .unwrap();
        gate.wait().await;
        result
    };
    let (slow, fast) = tokio::join!(slow, fast);
    assert!(slow.is_err());
    assert_eq!(fast.state, ImportState::Complete);
    let current = detail(&f, &source_id).await;
    assert_eq!(current.source.revision, Some(1));
    assert_eq!(current.passages[0].text, "Current response");
    assert_eq!(count(&f.db.conn, "business_intake_version").await, 1);
}

#[tokio::test]
async fn intake_import_grant_revocation_and_caller_abort_cannot_commit_private_content() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let (import, source_id) = discover(&f, &b).await;
    let g = grants_list(
        &f.db.conn,
        &f.op,
        &f.services,
        BindingPageInput {
            binding_id: b.id.clone(),
            page: 0,
        },
    )
    .await
    .unwrap()
    .items
    .remove(0);
    let gate = Arc::new(tokio::sync::Barrier::new(2));
    let mut reply = Reply::json(transcript("record-1", "Revoked response"));
    reply.gate = Some(gate.clone());
    f.mock.reply(reply);
    let future = imports_advance(&f.db.conn, &f.op, &f.services, next(&import));
    let revoke = async {
        f.mock.seen(3).await;
        let result = grants_revoke(
            &f.db.conn,
            &f.op,
            &f.services,
            RevokeGrantInput {
                operation_id: common::id(),
                binding_id: b.id.clone(),
                expected_binding_revision: b.revision,
                grant_id: g.id.clone(),
                expected_grant_revision: g.revision,
            },
        )
        .await
        .unwrap();
        gate.wait().await;
        result
    };
    let (result, revoke) = tokio::join!(future, revoke);
    assert!(result.is_err());
    assert_eq!(count(&f.db.conn, "business_intake_version").await, 0);
    assert!(sources_get(
        &f.db.conn,
        &f.op,
        &f.services,
        SourceInput {
            source_id: source_id.clone()
        }
    )
    .await
    .is_err());
    let b = grants_upsert(
        &f.db.conn,
        &f.op,
        &f.services,
        grant(
            &revoke.binding,
            f.op.member_id(),
            Some(revoke.grant.revision),
        ),
    )
    .await
    .unwrap()
    .binding;
    let import = record(&f, &b, &source_id).await;
    let mut reply = Reply::json(transcript("record-1", "Aborted request"));
    reply.gate = Some(Arc::new(tokio::sync::Barrier::new(2)));
    f.mock.reply(reply);
    {
        let future = imports_advance(&f.db.conn, &f.op, &f.services, next(&import));
        tokio::pin!(future);
        tokio::select! {_=f.mock.seen(4)=>{},_=&mut future=>panic!("gated read unexpectedly completed")}
    }
    assert_eq!(count(&f.db.conn, "business_intake_version").await, 0);
    assert_eq!(
        detail(&f, &source_id).await.disclosure,
        Disclosure::MetadataOnly
    );
    let row = imports_get(
        &f.db.conn,
        &f.op,
        &f.services,
        ImportInput {
            import_id: import.id,
        },
    )
    .await
    .unwrap();
    assert_eq!(row.state, ImportState::Running);
    assert!(!row.capabilities.advance);
}

#[tokio::test]
async fn intake_import_retry_budget_and_duplicate_page_cap_are_explicit() {
    let f = Fixture::new().await;
    let b = f.allowed().await;
    let mut import = start(&f, &b).await;
    for attempt in 1..=3 {
        let mut reply = Reply::json(json!({"private":"provider failure"}));
        reply.status = 503;
        f.mock.reply(reply);
        import = imports_advance(&f.db.conn, &f.op, &f.services, next(&import))
            .await
            .unwrap();
        assert_eq!(
            import.state,
            if attempt < 3 {
                ImportState::Waiting
            } else {
                ImportState::Failed
            }
        );
        assert_eq!(import.error_code, Some(error::Reason::ProviderUnavailable));
        if attempt < 3 {
            assert!(
                imports_advance(&f.db.conn, &f.op, &f.services, next(&import))
                    .await
                    .is_err()
            );
            f.db.conn
                .execute(common::sql(
                    "UPDATE business_intake_import SET next_attempt_at=? WHERE id=?",
                    vec!["2020-01-01T00:00:00Z".into(), import.id.clone().into()],
                ))
                .await
                .unwrap();
        }
    }
    assert!(
        imports_advance(&f.db.conn, &f.op, &f.services, next(&import))
            .await
            .is_err()
    );
    assert_eq!(f.mock.count(), 4);
    let mut import = start(&f, &b).await;
    for page in 0..5 {
        f.mock.json(json!({"data":{"transcripts":vec![json!({"id":"overlapping-record","title":"Repeated provider result"});50]}}));
        import = imports_advance(&f.db.conn, &f.op, &f.services, next(&import))
            .await
            .unwrap();
        if page == 0 {
            f.mock
                .json(transcript("overlapping-record", "Deduplicated source"));
            import = imports_advance(&f.db.conn, &f.op, &f.services, next(&import))
                .await
                .unwrap();
        }
    }
    assert_eq!(import.state, ImportState::Complete);
    assert_eq!(import.coverage, Coverage::Capped);
    assert_eq!(import.discovered, 1);
    assert_eq!(import.completed, 1);
    let requests = f.mock.requests();
    let skips: Vec<_> = requests
        .iter()
        .filter_map(|r| r["variables"]["skip"].as_i64())
        .collect();
    assert_eq!(skips, vec![0, 0, 0, 0, 50, 100, 150, 200]);
    assert_eq!(count(&f.db.conn, "business_intake_candidate").await, 1);
    assert_eq!(count(&f.db.conn, "business_task").await, 0);
}
