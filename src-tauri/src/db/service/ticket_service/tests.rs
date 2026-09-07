//! MIT cases adapted from Chatwoot v4.17.1 mailbox finder specs (see NOTICE).
//! SQLite rollback, replay and restart cases test the persistence glue.
use super::*;
use crate::db::{migration::Migrator, test_helpers::fresh_in_memory_db};
use sea_orm::{ConnectOptions, Database};
use sea_orm_migration::{MigratorTrait, SchemaManager};

async fn scope(conn: &DatabaseConnection, account_id: i32, email: &str) -> Scope {
    let inbox = create_inbox(conn, account_id, "Support", email)
        .await
        .unwrap();
    Scope {
        account_id,
        inbox_id: inbox.id,
    }
}

fn mail(id: &str) -> IncomingEmail {
    IncomingEmail {
        headers: ThreadHeaders::default(),
        message_id: id.into(),
        sender_email: "tester@example.com".into(),
        sender_name: None,
        subject: "Same subject".into(),
        content: "Which build?".into(),
        auto_reply: false,
    }
}

async fn seed(conn: &DatabaseConnection, scope: Scope, id: &str) -> IngestedEmail {
    ingest_email(conn, scope, mail(id)).await.unwrap()
}

fn pattern(ticket: &conversation::Model) -> String {
    format!("conversation/{}/messages/123@example.com", ticket.uuid)
}

async fn found(
    conn: &DatabaseConnection,
    scope: Scope,
    headers: &ThreadHeaders,
) -> (Option<i32>, Strategy) {
    let (ticket, strategy) = threading::find(conn, scope, headers).await.unwrap();
    (ticket.map(|t| t.id), strategy)
}

#[tokio::test]
async fn strategy_precedence_uses_first_match_even_when_each_targets_a_different_thread() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    let a = seed(&db.conn, s, "a@example.com").await.conversation;
    let b = seed(&db.conn, s, "b@example.com").await.conversation;
    let c = seed(&db.conn, s, "c@example.com").await.conversation;
    let mut h = ThreadHeaders {
        receivers: vec![format!("reply+{}@example.com", a.uuid)],
        in_reply_to: vec![pattern(&b)],
        references: vec![pattern(&c)],
        ..Default::default()
    };
    assert_eq!(
        found(&db.conn, s, &h).await,
        (Some(a.id), Strategy::ReceiverUuid)
    );
    h.receivers.clear();
    assert_eq!(
        found(&db.conn, s, &h).await,
        (Some(b.id), Strategy::InReplyTo)
    );
    h.in_reply_to.clear();
    assert_eq!(
        found(&db.conn, s, &h).await,
        (Some(c.id), Strategy::References)
    );
    h.references.clear();
    assert_eq!(
        found(&db.conn, s, &h).await,
        (None, Strategy::NewConversation)
    );
    assert_eq!(
        list_conversations(&db.conn, s).await.unwrap().len(),
        3,
        "finder does not persist"
    );
}

#[tokio::test]
async fn receiver_uuid_requires_exact_local_part_and_uses_first_syntactic_match() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    let a = seed(&db.conn, s, "a@example.com").await.conversation;
    for receiver in [
        format!("reply+{}@example.com", a.uuid),
        format!("REPLY+{}@EXAMPLE.COM", a.uuid.to_uppercase()),
    ] {
        let h = ThreadHeaders {
            receivers: vec!["other@example.com".into(), receiver],
            ..Default::default()
        };
        assert_eq!(
            found(&db.conn, s, &h).await,
            (Some(a.id), Strategy::ReceiverUuid)
        );
    }
    for receiver in [
        "reply+not-a-valid-uuid@example.com".into(),
        format!("test+{}@example.com", a.uuid),
        format!("reply+{}-extra@example.com", a.uuid),
    ] {
        let h = ThreadHeaders {
            receivers: vec![receiver],
            ..Default::default()
        };
        assert_eq!(
            found(&db.conn, s, &h).await,
            (None, Strategy::NewConversation)
        );
    }
    let h = ThreadHeaders {
        receivers: vec![
            "reply+99999999-9999-9999-9999-999999999999@example.com".into(),
            format!("reply+{}@example.com", a.uuid),
        ],
        ..Default::default()
    };
    assert_eq!(
        found(&db.conn, s, &h).await,
        (None, Strategy::NewConversation)
    );
}

#[tokio::test]
async fn header_patterns_source_ids_and_values_keep_upstream_order() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    let a = seed(&db.conn, s, "a@example.com").await.conversation;
    let b = seed(&db.conn, s, "b@example.com").await.conversation;
    for value in [
        pattern(&a),
        format!("account/1/conversation/{}@example.com", a.uuid),
        " <a@example.com> ".into(),
    ] {
        for references in [false, true] {
            let values = vec!["missing@example.com".into(), value.clone(), pattern(&b)];
            let mut h = ThreadHeaders::default();
            let strategy = if references {
                h.references = values;
                Strategy::References
            } else {
                h.in_reply_to = values;
                Strategy::InReplyTo
            };
            assert_eq!(found(&db.conn, s, &h).await, (Some(a.id), strategy));
        }
    }
    // Within a single token, UUID pattern takes precedence over a source-id hit.
    let mut b_message = message::Entity::find()
        .filter(message::Column::ConversationId.eq(b.id))
        .one(&db.conn)
        .await
        .unwrap()
        .unwrap()
        .into_active_model();
    b_message.source_id = Set(Some(pattern(&a)));
    b_message.update(&db.conn).await.unwrap();
    let h = ThreadHeaders {
        in_reply_to: vec![pattern(&a)],
        ..Default::default()
    };
    assert_eq!(
        found(&db.conn, s, &h).await,
        (Some(a.id), Strategy::InReplyTo)
    );
    // A syntactically valid but missing UUID still falls through to source_id.
    let missing_pattern =
        "conversation/99999999-9999-9999-9999-999999999999/messages/123@example.com";
    let c = seed(&db.conn, s, missing_pattern).await;
    let h = ThreadHeaders {
        references: vec![missing_pattern.into()],
        ..Default::default()
    };
    assert_eq!(
        found(&db.conn, s, &h).await,
        (Some(c.conversation.id), Strategy::References)
    );
}

#[tokio::test]
async fn every_strategy_is_isolated_by_account_and_inbox_and_skips_foreign_references() {
    let db = fresh_in_memory_db().await;
    let own = scope(&db.conn, 1, "own@example.com").await;
    let other_inbox = scope(&db.conn, 1, "other@example.com").await;
    let other_account = scope(&db.conn, 2, "other@example.com").await;
    let local = seed(&db.conn, own, "local@example.com").await.conversation;
    for foreign in [other_inbox, other_account] {
        let ticket = seed(&db.conn, foreign, "foreign@example.com")
            .await
            .conversation;
        for headers in [
            ThreadHeaders {
                receivers: vec![format!("reply+{}@example.com", ticket.uuid)],
                ..Default::default()
            },
            ThreadHeaders {
                in_reply_to: vec![pattern(&ticket)],
                ..Default::default()
            },
            ThreadHeaders {
                in_reply_to: vec!["foreign@example.com".into()],
                ..Default::default()
            },
            ThreadHeaders {
                references: vec![pattern(&ticket)],
                ..Default::default()
            },
            ThreadHeaders {
                references: vec!["foreign@example.com".into()],
                ..Default::default()
            },
            ThreadHeaders {
                in_reply_to: vec![format!(
                    "account/{}/conversation/{}@example.com",
                    foreign.account_id, ticket.uuid
                )],
                ..Default::default()
            },
        ] {
            assert_eq!(
                found(&db.conn, own, &headers).await,
                (None, Strategy::NewConversation)
            );
        }
        let h = ThreadHeaders {
            references: vec![
                pattern(&ticket),
                "foreign@example.com".into(),
                pattern(&local),
            ],
            ..Default::default()
        };
        assert_eq!(
            found(&db.conn, own, &h).await,
            (Some(local.id), Strategy::References)
        );
        assert!(get_conversation(&db.conn, own, ticket.id).await.is_err());
        assert!(get_contact(&db.conn, own, ticket.id).await.is_err());
        assert!(
            list_messages(&db.conn, own, ticket.id, MessageView::Internal)
                .await
                .is_err()
        );
        assert!(
            add_private_note(&db.conn, own, ticket.id, "reviewer", "secret")
                .await
                .is_err()
        );
        assert!(assign(
            &db.conn,
            own,
            ticket.id,
            Assignment::User("reviewer".into()),
            None
        )
        .await
        .is_err());
        assert!(
            set_status(&db.conn, own, ticket.id, ConversationStatus::Resolved)
                .await
                .is_err()
        );
    }
    let invalid = Scope {
        account_id: 2,
        inbox_id: own.inbox_id,
    };
    assert!(
        threading::find(&db.conn, invalid, &ThreadHeaders::default())
            .await
            .is_err()
    );
    assert!(ingest_email(&db.conn, invalid, mail("bad@example.com"))
        .await
        .is_err());
    assert!(threading::find(
        &db.conn,
        Scope {
            inbox_id: 99999,
            ..own
        },
        &ThreadHeaders::default()
    )
    .await
    .is_err());
}

#[tokio::test]
async fn new_conversation_reuses_contact_but_never_merges_on_sender_or_subject() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    let mut initial = mail("a@example.com");
    initial.sender_email = "TESTER@example.com".into();
    initial.sender_name = Some("Tester Name".into());
    initial.auto_reply = true;
    let a = ingest_email(&db.conn, s, initial).await.unwrap();
    let b = seed(&db.conn, s, "b@example.com").await;
    assert_ne!(a.conversation.id, b.conversation.id);
    assert_eq!(a.conversation.contact_id, b.conversation.contact_id);
    assert_eq!(
        a.conversation.contact_inbox_id,
        b.conversation.contact_inbox_id
    );
    let contact = get_contact(&db.conn, s, a.conversation.id).await.unwrap();
    assert_eq!(contact.name, "Tester Name");
    assert_eq!(contact.email, "tester@example.com");
    let meta: serde_json::Value =
        serde_json::from_str(&a.conversation.additional_attributes).unwrap();
    assert_eq!(meta["source"], "email");
    assert_eq!(meta["mail_subject"], "Same subject");
    assert_eq!(meta["auto_reply"], true);
    assert!(meta["initiated_at"]["timestamp"].is_string());
    let second_inbox = scope(&db.conn, 1, "second@example.com").await;
    let c = seed(&db.conn, second_inbox, "c@example.com").await;
    assert_eq!(c.conversation.contact_id, a.conversation.contact_id);
    assert_ne!(
        c.conversation.contact_inbox_id,
        a.conversation.contact_inbox_id
    );
    let other_account = scope(&db.conn, 2, "support@example.com").await;
    let d = seed(&db.conn, other_account, "d@example.com").await;
    assert_ne!(d.conversation.contact_id, a.conversation.contact_id);
}

#[tokio::test]
async fn stored_in_reply_to_fallback_is_exact_and_scoped() {
    let db = fresh_in_memory_db().await;
    let own = scope(&db.conn, 1, "own@example.com").await;
    let foreign = scope(&db.conn, 1, "foreign@example.com").await;
    let headers = ThreadHeaders {
        raw_in_reply_to: Some("<missing-parent@example.com>".into()),
        ..Default::default()
    };
    let mut a_mail = mail("a@example.com");
    a_mail.headers = headers.clone();
    let a = ingest_email(&db.conn, foreign, a_mail).await.unwrap();
    assert_eq!(
        found(&db.conn, own, &headers).await,
        (None, Strategy::NewConversation)
    );
    let mut b_mail = mail("b@example.com");
    b_mail.headers = headers.clone();
    let b = ingest_email(&db.conn, own, b_mail).await.unwrap();
    let mut c_mail = mail("c@example.com");
    c_mail.headers = headers.clone();
    let c = ingest_email(&db.conn, own, c_mail).await.unwrap();
    assert_eq!(c.conversation.id, b.conversation.id);
    assert_ne!(c.conversation.id, a.conversation.id);
    assert_eq!(c.strategy, Some(Strategy::NewConversation));
    assert_eq!(
        list_messages(&db.conn, own, b.conversation.id, MessageView::Public)
            .await
            .unwrap()
            .len(),
        2
    );
    let h = ThreadHeaders {
        raw_in_reply_to: Some("missing-parent@example.com".into()),
        ..Default::default()
    };
    assert_eq!(
        found(&db.conn, own, &h).await,
        (None, Strategy::NewConversation)
    );
}

#[tokio::test]
async fn failed_first_message_rolls_back_contact_join_and_conversation_and_retry_succeeds() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    let mut too_long = mail("retry@example.com");
    too_long.content = "a".repeat(150_001);
    assert!(ingest_email(&db.conn, s, too_long).await.is_err());
    assert!(contact::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
    assert!(contact_inbox::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
    assert!(list_conversations(&db.conn, s).await.unwrap().is_empty());
    assert!(message::Entity::find()
        .all(&db.conn)
        .await
        .unwrap()
        .is_empty());
    assert!(!seed(&db.conn, s, "retry@example.com").await.duplicate);
}

#[tokio::test]
async fn replay_uses_source_id_before_strategy_and_preserves_content() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    let a = seed(&db.conn, s, "a@example.com").await;
    let b = seed(&db.conn, s, "b@example.com").await;
    let mut replay = mail("<a@example.com>");
    replay.content = "changed content".into();
    replay.headers.in_reply_to = vec![pattern(&b.conversation)];
    let duplicate = ingest_email(&db.conn, s, replay).await.unwrap();
    assert!(duplicate.duplicate);
    assert_eq!(duplicate.strategy, None);
    assert_eq!(duplicate.message.id, a.message.id);
    assert_eq!(duplicate.message.content, a.message.content);
    assert_eq!(duplicate.conversation.id, a.conversation.id);
    let foreign = scope(&db.conn, 1, "foreign@example.com").await;
    let separate = seed(&db.conn, foreign, "a@example.com").await;
    assert!(!separate.duplicate);
    assert_ne!(separate.message.id, a.message.id);
}

#[tokio::test]
async fn notes_assignment_and_lifecycle_survive_disk_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("tickets.db");
    let url = format!("sqlite:{}?mode=rwc", path.to_string_lossy());
    let mut options = ConnectOptions::new(url.clone());
    options
        .max_connections(1)
        .min_connections(1)
        .sqlx_logging(false);
    let conn = Database::connect(options.clone()).await.unwrap();
    conn.execute_unprepared("PRAGMA foreign_keys=ON; PRAGMA journal_mode=WAL;")
        .await
        .unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let s = scope(&conn, 1, "support@example.com").await;
    let a = seed(&conn, s, "a@example.com").await;
    assign(
        &conn,
        s,
        a.conversation.id,
        Assignment::AgentBot("pi".into()),
        None,
    )
    .await
    .unwrap();
    let assigned = assign(
        &conn,
        s,
        a.conversation.id,
        Assignment::User("reviewer".into()),
        Some("support".into()),
    )
    .await
    .unwrap();
    assert_eq!(assigned.assignee_agent_bot_id, None);
    assert_eq!(assigned.assignee_id.as_deref(), Some("reviewer"));
    let note = add_private_note(
        &conn,
        s,
        a.conversation.id,
        "reviewer",
        "Internal account note",
    )
    .await
    .unwrap();
    assert!(note.private);
    assert_eq!(note.message_type, 1);
    assert_eq!(note.source_id, None);
    assert!(get_conversation(&conn, s, a.conversation.id)
        .await
        .unwrap()
        .waiting_since
        .is_some());
    let resolved = set_status(&conn, s, a.conversation.id, ConversationStatus::Resolved)
        .await
        .unwrap();
    assert_eq!(resolved.waiting_since, None);
    conn.close().await.unwrap();

    let conn = Database::connect(options).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let stored = get_conversation(&conn, s, a.conversation.id).await.unwrap();
    assert_eq!(stored, resolved);
    assert_eq!(list_inboxes(&conn, 1).await.unwrap().len(), 1);
    assert_eq!(
        get_contact(&conn, s, stored.id).await.unwrap().email,
        "tester@example.com"
    );
    let internal = list_messages(&conn, s, stored.id, MessageView::Internal)
        .await
        .unwrap();
    let public = list_messages(&conn, s, stored.id, MessageView::Public)
        .await
        .unwrap();
    assert_eq!(internal.len(), 2);
    assert_eq!(public, vec![a.message]);
    let mut reply = mail("reply@example.com");
    reply.headers.in_reply_to = vec!["a@example.com".into()];
    let reopened = ingest_email(&conn, s, reply).await.unwrap();
    assert_eq!(reopened.conversation.id, stored.id);
    assert_eq!(reopened.conversation.status, 0);
    assert!(reopened.conversation.waiting_since.is_some());
    let snoozed = set_status(&conn, s, stored.id, ConversationStatus::Snoozed(Utc::now()))
        .await
        .unwrap();
    assert!(snoozed.snoozed_until.is_some());
    let mut reply = mail("reply2@example.com");
    reply.headers.references = vec!["a@example.com".into()];
    let reopened = ingest_email(&conn, s, reply).await.unwrap();
    assert_eq!(reopened.conversation.snoozed_until, None);
    assert_eq!(reopened.conversation.status, 0);
    assert_eq!(
        assign(&conn, s, stored.id, Assignment::Unassigned, None)
            .await
            .unwrap()
            .assignee_id,
        None
    );
    conn.close().await.unwrap();
}

#[tokio::test]
async fn database_constraints_prevent_cross_inbox_links_and_private_source_ids() {
    let db = fresh_in_memory_db().await;
    let own = scope(&db.conn, 1, "own@example.com").await;
    let foreign = scope(&db.conn, 2, "foreign@example.com").await;
    let a = seed(&db.conn, own, "a@example.com").await;
    let b = seed(&db.conn, foreign, "b@example.com").await;
    let mut bad_message = a.message.clone().into_active_model();
    bad_message.conversation_id = Set(b.conversation.id);
    assert!(bad_message.update(&db.conn).await.is_err());
    let mut bad_ticket = a.conversation.clone().into_active_model();
    bad_ticket.contact_inbox_id = Set(b.conversation.contact_inbox_id);
    assert!(bad_ticket.update(&db.conn).await.is_err());
    let mut bad_assignment = a.conversation.clone().into_active_model();
    bad_assignment.assignee_id = Set(Some("user".into()));
    bad_assignment.assignee_agent_bot_id = Set(Some("bot".into()));
    assert!(bad_assignment.update(&db.conn).await.is_err());
    let note = add_private_note(&db.conn, own, a.conversation.id, "reviewer", "secret")
        .await
        .unwrap();
    let mut bad_note = note.into_active_model();
    bad_note.source_id = Set(Some("note@example.com".into()));
    assert!(bad_note.update(&db.conn).await.is_err());
    let mut activity = a.message.clone().into_active_model();
    activity.id = sea_orm::ActiveValue::NotSet;
    activity.source_id = Set(None);
    activity.message_type = Set(2);
    activity.insert(&db.conn).await.unwrap();
    assert_eq!(
        list_messages(&db.conn, own, a.conversation.id, MessageView::Public)
            .await
            .unwrap(),
        vec![a.message]
    );
}

#[tokio::test]
async fn migration_down_up_only_removes_ticket_tables() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    seed(&db.conn, s, "a@example.com").await;
    let migration = Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260907_000002_ops_tickets")
        .unwrap();
    let manager = SchemaManager::new(&db.conn);
    migration.down(&manager).await.unwrap();
    assert!(!manager.has_table("ops_ticket_inbox").await.unwrap());
    assert!(manager.has_table("conversation").await.unwrap());
    assert!(manager.has_table("work_task").await.unwrap());
    migration.up(&manager).await.unwrap();
    let s = scope(&db.conn, 1, "support@example.com").await;
    assert!(list_conversations(&db.conn, s).await.unwrap().is_empty());
    seed(&db.conn, s, "a@example.com").await;
}

#[tokio::test]
async fn malformed_identity_is_rejected_without_creating_records() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    for sender in [
        "",
        "not-an-address",
        "bad@@example.com",
        "Display <user@example.com>",
        "bad\0@example.com",
    ] {
        let mut input = mail("a@example.com");
        input.sender_email = sender.into();
        assert!(ingest_email(&db.conn, s, input).await.is_err());
    }
    for source in [
        "",
        " ",
        "<>",
        "bad\nid@example.com",
        "<unclosed@example.com",
    ] {
        assert!(ingest_email(&db.conn, s, mail(source)).await.is_err());
    }
    assert!(list_conversations(&db.conn, s).await.unwrap().is_empty());
}

#[tokio::test]
async fn public_reply_is_a_receipt_separate_from_notes_and_threads_future_incoming_mail() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    let a = seed(&db.conn, s, "a@example.com").await.conversation;
    add_private_note(&db.conn, s, a.id, "reviewer", "Private reasoning")
        .await
        .unwrap();
    assert_eq!(
        get_conversation(&db.conn, s, a.id)
            .await
            .unwrap()
            .first_reply_created_at,
        None
    );
    let public = record_public_reply(
        &db.conn,
        s,
        a.id,
        "reviewer",
        "Please try build 42",
        "<sent@example.com>",
    )
    .await
    .unwrap();
    assert!(!public.private);
    assert_eq!(public.message_type, 1);
    let ticket = get_conversation(&db.conn, s, a.id).await.unwrap();
    assert_eq!(ticket.first_reply_created_at, Some(public.created_at));
    assert_eq!(ticket.waiting_since, None);
    let duplicate = record_public_reply(
        &db.conn,
        s,
        a.id,
        "reviewer",
        "Please try build 42",
        "sent@example.com",
    )
    .await
    .unwrap();
    assert_eq!(duplicate, public);
    assert!(record_public_reply(
        &db.conn,
        s,
        a.id,
        "reviewer",
        "Changed after send",
        "sent@example.com"
    )
    .await
    .is_err());
    let b = seed(&db.conn, s, "b@example.com").await.conversation;
    assert!(record_public_reply(
        &db.conn,
        s,
        b.id,
        "reviewer",
        "Please try build 42",
        "sent@example.com"
    )
    .await
    .is_err());
    let mut input = mail("response@example.com");
    input.headers.in_reply_to = vec!["sent@example.com".into()];
    let reply = ingest_email(&db.conn, s, input).await.unwrap();
    assert_eq!(reply.conversation.id, a.id);
    assert_eq!(reply.strategy, Some(Strategy::InReplyTo));
    let public_view = list_messages(&db.conn, s, a.id, MessageView::Public)
        .await
        .unwrap();
    assert_eq!(public_view.len(), 3);
    assert!(public_view.iter().all(|m| !m.private));
}

#[tokio::test]
async fn independent_sqlite_connections_serialize_replay_and_parent_fallback() {
    let dir = tempfile::tempdir().unwrap();
    let url = format!(
        "sqlite:{}?mode=rwc",
        dir.path().join("concurrent.db").to_string_lossy()
    );
    let mut options = ConnectOptions::new(url);
    options
        .max_connections(1)
        .min_connections(1)
        .sqlx_logging(false);
    let conn1 = Database::connect(options.clone()).await.unwrap();
    conn1
        .execute_unprepared(
            "PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON; PRAGMA busy_timeout=5000;",
        )
        .await
        .unwrap();
    Migrator::up(&conn1, None).await.unwrap();
    let conn2 = Database::connect(options).await.unwrap();
    conn2
        .execute_unprepared("PRAGMA foreign_keys=ON; PRAGMA busy_timeout=5000;")
        .await
        .unwrap();
    let s = scope(&conn1, 1, "support@example.com").await;
    let (first, second) = tokio::join!(
        ingest_email(&conn1, s, mail("same@example.com")),
        ingest_email(&conn2, s, mail("same@example.com"))
    );
    let (first, second) = (first.unwrap(), second.unwrap());
    assert_ne!(first.duplicate, second.duplicate);
    assert_eq!(first.message.id, second.message.id);
    assert_eq!(first.conversation.id, second.conversation.id);
    assert_eq!(list_conversations(&conn1, s).await.unwrap().len(), 1);
    let mut a = mail("a@example.com");
    a.headers.raw_in_reply_to = Some("<external-parent@example.com>".into());
    let mut b = mail("b@example.com");
    b.headers = a.headers.clone();
    let (first, second) = tokio::join!(ingest_email(&conn1, s, a), ingest_email(&conn2, s, b));
    let (first, second) = (first.unwrap(), second.unwrap());
    assert_eq!(first.conversation.id, second.conversation.id);
    assert_ne!(first.message.id, second.message.id);
    assert_eq!(list_conversations(&conn1, s).await.unwrap().len(), 2);
    conn1.close().await.unwrap();
    conn2.close().await.unwrap();
}

#[tokio::test]
async fn migration_ddl_failure_rolls_back_preceding_tables() {
    let conn = Database::connect("sqlite::memory:").await.unwrap();
    conn.execute_unprepared("CREATE TABLE ops_ticket_message (id INTEGER PRIMARY KEY);")
        .await
        .unwrap();
    let migration = Migrator::migrations()
        .into_iter()
        .find(|m| m.name() == "m20260907_000002_ops_tickets")
        .unwrap();
    let manager = SchemaManager::new(&conn);
    assert!(migration.up(&manager).await.is_err());
    assert!(!manager.has_table("ops_ticket_inbox").await.unwrap());
    assert!(!manager.has_table("ops_ticket_contact").await.unwrap());
    assert!(!manager.has_table("ops_ticket_conversation").await.unwrap());
    assert!(manager.has_table("ops_ticket_message").await.unwrap());
}
