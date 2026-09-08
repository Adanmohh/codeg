//! Real host/approval/provider paths; no fabricated authorization or actor wire.
use super::*;
use crate::ops_intake_host::{self as host, tests as fixtures, types as h};

pub(super) async fn opted(db: &AppDatabase, provider: &Provider) {
    let op = operator(1);
    let cfg = settings(db, &op, &provider.runtime, true)
        .await
        .configuration
        .unwrap();
    configure(
        &db.conn,
        &op,
        &provider.runtime,
        ConfigureInput {
            channel_id: cfg.channel_id,
            private_user_id: cfg.private_user_id,
            review_origin: cfg.review_origin,
            enabled: true,
            github_issues_enabled: true,
            expected_revision: Some(cfg.revision),
        },
    )
    .await
    .unwrap();
}

pub(super) async fn proposed(
    db: &AppDatabase,
    ctx: &crate::ops::agent::RunContext,
    d: &h::Draft,
    runtime: &host::HostRuntime,
) -> host::agent::Proposed {
    fixtures::propose_notice(db, ctx, d, runtime).await
}

#[tokio::test]
async fn issue_preflight_can_recover_but_ambiguous_delivery_cannot_repeat() {
    let github = fixtures::fixture::Provider::start().await;
    let (db, _, ctx, d) = fixtures::ready(&github).await;
    proposed(&db, &ctx, &d, &github.runtime).await;
    let provider = Provider::new().await;
    opted(&db, &provider).await;
    provider.reply(
        StatusCode::SERVICE_UNAVAILABLE,
        json!({"ok":false}),
        Duration::ZERO,
    );
    notify_now(&db.conn, &operator(1), &provider.runtime)
        .await
        .unwrap();
    let before = first_notice(&db).await;
    assert_eq!(before.status, "preflight_failed");
    assert_eq!(provider.sends(), 0);
    provider.reply(
        StatusCode::OK,
        json!({"ok":true,"result":{"id":123,"type":"private"}}),
        Duration::ZERO,
    );
    // A wrong sender in a successful response is ambiguous, never accepted.
    provider.reply(StatusCode::OK,json!({"ok":true,"result":{"message_id":42,"chat":{"id":123,"type":"private"},"from":{"is_bot":false}}}),Duration::ZERO);
    notify_now(&db.conn, &operator(1), &provider.runtime)
        .await
        .unwrap();
    let after = first_notice(&db).await;
    assert_eq!(after.id, before.id);
    assert_ne!(after.claim_id, before.claim_id);
    assert_eq!(after.status, "unknown");
    assert!(after.provider_message_id.is_none());
    notify_now(&db.conn, &operator(1), &provider.runtime)
        .await
        .unwrap();
    assert_eq!(provider.sends(), 1);
    assert_eq!(github.seen.lock().unwrap().posts, 0);
}

#[tokio::test]
async fn email_and_issue_share_one_scan_deadline_and_keep_cancelled_attempts() {
    let github = fixtures::fixture::Provider::start().await;
    let (db, _, ctx, d) = fixtures::ready(&github).await;
    let issue = proposed(&db, &ctx, &d, &github.runtime).await;
    let (op, key) = seed(&db, 1).await;
    let email = saved(&db, &op, key).await;
    let reply = pending(&db, &op, &email).await;
    let provider = Provider::new().await;
    opted(&db, &provider).await;
    provider.reply(
        StatusCode::OK,
        json!({"ok":true,"result":{"id":123,"type":"private"}}),
        Duration::ZERO,
    );
    provider.reply(StatusCode::OK, json!({"ok":true}), Duration::from_secs(5));
    let start = std::time::Instant::now();
    bounded_scan(
        &db.conn,
        &provider.runtime,
        None,
        Duration::from_millis(150),
    )
    .await
    .unwrap();
    assert!(start.elapsed() < Duration::from_secs(2));
    let first = first_notice(&db).await;
    assert_eq!(first.proposal_id, issue.proposal_id.unwrap());
    assert_eq!(first.status, "unknown");
    assert_eq!(provider.sends(), 1);
    tick(&db.conn, &provider.runtime).await.unwrap();
    assert_eq!(provider.sends(), 2);
    let email = notice::Entity::find()
        .filter(notice::Column::ProposalId.eq(reply.id))
        .one(&db.conn)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(email.status, "sent");
    assert!(resolve(&db.conn, &op, ResolveInput { notice: email.id })
        .await
        .unwrap()
        .proposal
        .is_some());
}

#[tokio::test]
async fn more_than_twenty_stale_issue_candidates_do_not_starve_the_current_one() {
    let github = fixtures::fixture::Provider::start().await;
    let (db, source, ctx, mut d) = fixtures::ready(&github).await;
    let mut latest = proposed(&db, &ctx, &d, &github.runtime)
        .await
        .proposal_id
        .unwrap();
    let folder = crate::db::service::work_task_service::get(&db.conn, ctx.task_id)
        .await
        .unwrap()
        .folder_id;
    for _ in 0..21 {
        let ctx = fixtures::start_task(&db, folder).await;
        latest = proposed(&db, &ctx, &d, &github.runtime)
            .await
            .proposal_id
            .unwrap();
        d = host::operator::detail(&db.conn, &operator(1), source.clone())
            .await
            .unwrap()
            .draft;
    }
    let provider = Provider::new().await;
    opted(&db, &provider).await;
    notify_now(&db.conn, &operator(1), &provider.runtime)
        .await
        .unwrap();
    assert_eq!(provider.sends(), 1);
    assert_eq!(first_notice(&db).await.proposal_id, latest);
    assert_eq!(github.seen.lock().unwrap().posts, 0);
}

#[test]
fn old_configuration_payload_does_not_enable_new_issue_family() {
    let input: ConfigureInput = serde_json::from_value(json!({"channelId":1,"privateUserId":"123","reviewOrigin":"https://desk.example","enabled":true})).unwrap();
    assert!(!input.github_issues_enabled);
    assert!(serde_json::from_value::<ActionKind>(json!("arbitrary_executor")).is_err());
}

#[tokio::test]
async fn issues_require_separate_opt_in_and_email_remains_reviewable() {
    let github = fixtures::fixture::Provider::start().await;
    let (db, _, ctx, d) = fixtures::ready(&github).await;
    let issue = proposed(&db, &ctx, &d, &github.runtime).await;
    let (op, key) = seed(&db, 1).await;
    let email = saved(&db, &op, key).await;
    let reply = pending(&db, &op, &email).await;
    let provider = Provider::new().await;
    let cfg = settings(&db, &op, &provider.runtime, true)
        .await
        .configuration
        .unwrap();
    notify_now(&db.conn, &op, &provider.runtime).await.unwrap();
    assert_eq!(provider.sends(), 1);
    let email_notice = first_notice(&db).await;
    assert_eq!(email_notice.proposal_id, reply.id);
    assert_eq!(email_notice.action_kind, ActionKind::EmailReply);
    configure(
        &db.conn,
        &op,
        &provider.runtime,
        ConfigureInput {
            channel_id: cfg.channel_id,
            private_user_id: cfg.private_user_id,
            review_origin: cfg.review_origin,
            enabled: true,
            github_issues_enabled: true,
            expected_revision: Some(cfg.revision),
        },
    )
    .await
    .unwrap();
    assert_eq!(provider.sends(), 1); // Saving cannot send.
    let (a, b) = tokio::join!(
        notify_now(&db.conn, &op, &provider.runtime),
        notify_now(&db.conn, &op, &provider.runtime)
    );
    a.unwrap();
    b.unwrap();
    assert_eq!(provider.sends(), 2);
    let notice = notice::Entity::find()
        .filter(notice::Column::ProposalId.eq(issue.proposal_id.unwrap()))
        .one(&db.conn)
        .await
        .unwrap()
        .unwrap();
    let projection = resolve(
        &db.conn,
        &op,
        ResolveInput {
            notice: notice.id.clone(),
        },
    )
    .await
    .unwrap();
    assert!(projection.proposal.is_none());
    assert_eq!(
        projection.issue.unwrap().detail.proposals[0]
            .payload
            .as_ref(),
        Some(&issue.prepared)
    );
    assert!(
        resolve(&db.conn, &operator(2), ResolveInput { notice: notice.id })
            .await
            .unwrap()
            .issue
            .is_none()
    );
    // No contents, repository, private tester data or execution authority leaves
    // through the existing fixed Telegram message builder.
    let wire = serde_json::to_string(&*provider.state.seen.lock().unwrap()).unwrap();
    for private in [
        "Original approved body",
        "owner/repo",
        "verse four",
        "PRIVATE",
        "synthetic-pi",
        "Bearer",
    ] {
        assert!(!wire.contains(private), "{private}");
    }
    assert_eq!(github.seen.lock().unwrap().posts, 0);
}

#[tokio::test]
async fn issue_locator_and_loaded_decision_reject_every_changed_binding() {
    for mutation in [
        "deny",
        "cancel",
        "run",
        "connection",
        "account",
        "folder",
        "repository",
        "installation",
        "app",
        "source",
        "expired",
        "proof",
        "draft_revision",
        "edited",
        "rebind_restore",
        "recipient",
        "topic",
        "opt_out",
    ] {
        let github = fixtures::fixture::Provider::start().await;
        let (db, source, ctx, d) = fixtures::ready(&github).await;
        let p = proposed(&db, &ctx, &d, &github.runtime).await;
        let provider = Provider::new().await;
        opted(&db, &provider).await;
        notify_now(&db.conn, &operator(1), &provider.runtime)
            .await
            .unwrap();
        let row = first_notice(&db).await;
        assert!(resolve(
            &db.conn,
            &operator(1),
            ResolveInput {
                notice: row.id.clone()
            }
        )
        .await
        .unwrap()
        .issue
        .is_some());
        let sql = match mutation {
            "deny" => "UPDATE ops_proposal SET status='denied'",
            "cancel" => "UPDATE work_task SET status='canceled'",
            "run" => "UPDATE work_task SET run_seq=run_seq+1",
            "connection" => "UPDATE work_task SET connection_id='another-live-connection'",
            "account" => "UPDATE ops_intake_host_product SET account_id=2",
            "folder" => "UPDATE folder SET deleted_at=CURRENT_TIMESTAMP",
            "repository" => "UPDATE ops_intake_binding SET config_json=json_set(config_json,'$.repository_id',12)",
            "installation" => "UPDATE ops_intake_binding SET config_json=json_set(config_json,'$.installation_id',8)",
            "app" => "UPDATE ops_intake_binding SET config_json=json_set(config_json,'$.app_id','other-app')",
            "source" => "UPDATE ops_intake_source SET fetched_at=0",
            "expired" => "UPDATE ops_intake_evidence SET expires_at=0",
            "proof" => "UPDATE ops_intake_evidence SET content=CAST('changed proof bytes' AS BLOB)",
            "draft_revision" => "UPDATE ops_intake_host_draft SET revision=revision+1,draft_json=json_set(draft_json,'$.revision',revision+1)",
            "edited" => "UPDATE ops_intake_host_draft SET draft_json=json_set(draft_json,'$.prepared',NULL)",
            "rebind_restore" => "UPDATE ops_intake_host_product SET config_json=json_set(config_json,'$.binding.full_name','other/repo'); UPDATE ops_intake_host_product SET config_json=json_set(config_json,'$.binding.full_name','owner/repo')",
            "recipient" => "UPDATE ops_telegram_config SET private_user_id='456'",
            "topic" => "UPDATE chat_channel SET config_json=json_set(config_json,'$.topic_mode',json('true'))",
            _ => "UPDATE ops_telegram_config SET github_issues_enabled=0",
        };
        db.conn.execute_unprepared(sql).await.unwrap();
        let unavailable = resolve(
            &db.conn,
            &operator(1),
            ResolveInput {
                notice: row.id.clone(),
            },
        )
        .await
        .unwrap();
        assert_eq!(unavailable.state, "unavailable", "{mutation}");
        assert!(unavailable.issue.is_none() && unavailable.proposal.is_none());
        assert!(
            host::review::approve(
                &db.conn,
                &operator(1),
                &github.runtime,
                h::ReviewInput {
                    source: source.clone(),
                    proposal_id: p.proposal_id.unwrap(),
                    expected_payload: p.prepared.clone(),
                    approved_payload: p.prepared.clone(),
                    review_notice: Some(row.id.clone()),
                }
            )
            .await
            .is_err(),
            "{mutation}"
        );
        assert!(
            host::review::deny(
                &db.conn,
                &operator(1),
                &github.runtime,
                h::DenyInput {
                    source,
                    proposal_id: p.proposal_id.unwrap(),
                    expected_payload: p.prepared,
                    review_notice: Some(row.id),
                }
            )
            .await
            .is_err(),
            "{mutation}"
        );
        assert_eq!(github.seen.lock().unwrap().posts, 0, "{mutation}");
        assert_eq!(provider.sends(), 1, "{mutation}");
    }
}

#[tokio::test]
async fn issue_edit_while_recipient_get_is_pending_prevents_notification() {
    let github = fixtures::fixture::Provider::start().await;
    let (db, _, ctx, d) = fixtures::ready(&github).await;
    proposed(&db, &ctx, &d, &github.runtime).await;
    let provider = Provider::new().await;
    opted(&db, &provider).await;
    provider
        .state
        .pause_check
        .store(true, std::sync::atomic::Ordering::SeqCst);
    let op = operator(1);
    let (scan, ()) = tokio::join!(notify_now(&db.conn, &op, &provider.runtime), async {
        provider.state.started.notified().await;
        db.conn.execute_unprepared("UPDATE ops_intake_host_draft SET revision=revision+1,draft_json=json_set(draft_json,'$.revision',revision+1)").await.unwrap();
        provider.state.release.notify_one();
    });
    scan.unwrap();
    assert_eq!(first_notice(&db).await.status, "obsolete");
    assert_eq!(provider.sends(), 0);
    assert_eq!(github.seen.lock().unwrap().posts, 0);
}

#[tokio::test]
async fn phone_decisions_use_the_core_once_and_preserve_an_overlapping_acp_wait() {
    for approve in [true, false] {
        let github = fixtures::fixture::Provider::start().await;
        let (db, source, ctx, d) = fixtures::ready(&github).await;
        let p = proposed(&db, &ctx, &d, &github.runtime).await;
        let provider = Provider::new().await;
        opted(&db, &provider).await;
        notify_now(&db.conn, &operator(1), &provider.runtime)
            .await
            .unwrap();
        let row = first_notice(&db).await;
        crate::db::service::work_task_wait_service::track_request(
            &db.conn,
            ctx.task_id,
            ctx.run_seq,
            &ctx.connection_id,
            "p:phone-overlap",
            true,
        )
        .await
        .unwrap();
        if approve {
            let input = || h::ReviewInput {
                source: source.clone(),
                proposal_id: p.proposal_id.unwrap(),
                expected_payload: p.prepared.clone(),
                approved_payload: p.prepared.clone(),
                review_notice: Some(row.id.clone()),
            };
            let op = operator(1);
            let (a, b) = tokio::join!(
                host::review::approve(&db.conn, &op, &github.runtime, input()),
                host::review::approve(&db.conn, &op, &github.runtime, input())
            );
            assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
            let result = a.or(b).unwrap();
            assert_eq!(
                result.receipt.unwrap().state,
                crate::ops_intake::FilingState::Created
            );
            assert!(
                host::review::approve(&db.conn, &op, &github.runtime, input())
                    .await
                    .is_err()
            );
        } else {
            host::review::deny(
                &db.conn,
                &operator(1),
                &github.runtime,
                h::DenyInput {
                    source,
                    proposal_id: p.proposal_id.unwrap(),
                    expected_payload: p.prepared,
                    review_notice: Some(row.id.clone()),
                },
            )
            .await
            .unwrap();
        }
        assert_eq!(github.seen.lock().unwrap().posts, usize::from(approve));
        assert_eq!(
            crate::db::service::work_task_service::get(&db.conn, ctx.task_id)
                .await
                .unwrap()
                .status,
            crate::models::WorkTaskStatus::AwaitingInput
        );
        assert!(
            resolve(&db.conn, &operator(1), ResolveInput { notice: row.id })
                .await
                .unwrap()
                .issue
                .is_none()
        );
        assert_eq!(provider.sends(), 1);
    }
}
