//! Local HTTP cases adapt IntroMail send authorization/privacy tests and the
//! official Resend receiving SDK fixtures; exact sources/licenses are in NOTICE.
use std::{
    collections::VecDeque,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    body::{to_bytes, Bytes},
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Router,
};
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};
use serde_json::{json, Value};

use super::*;
use crate::db::{
    service::ticket_service::{self as tickets, Scope},
    test_helpers::fresh_in_memory_db,
};

const A: &str = "67d9bcdb-5a02-42d7-8da9-0d6feea18cff";
const B: &str = "97d9bcdb-5a02-42d7-8da9-0d6feea18cff";
const C: &str = "a7d9bcdb-5a02-42d7-8da9-0d6feea18cff";

struct Recorded {
    method: String,
    uri: String,
    headers: HeaderMap,
    body: Bytes,
}
struct Reply {
    status: u16,
    body: String,
    headers: HeaderMap,
    delay: Duration,
}
impl Reply {
    fn json(body: Value) -> Self {
        Self {
            status: 200,
            body: body.to_string(),
            headers: HeaderMap::new(),
            delay: Duration::ZERO,
        }
    }
}

#[derive(Clone, Default)]
struct MockState {
    replies: Arc<Mutex<VecDeque<Reply>>>,
    seen: Arc<Mutex<Vec<Recorded>>>,
}
struct Mock {
    address: SocketAddr,
    state: MockState,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Mock {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl Mock {
    async fn start() -> Self {
        async fn handler(State(state): State<MockState>, request: Request) -> impl IntoResponse {
            let (parts, body) = request.into_parts();
            let body = to_bytes(body, 2 * 1024 * 1024).await.unwrap();
            state.seen.lock().unwrap().push(Recorded {
                method: parts.method.to_string(),
                uri: parts.uri.to_string(),
                headers: parts.headers,
                body,
            });
            let reply = state
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected mock request");
            tokio::time::sleep(reply.delay).await;
            (
                StatusCode::from_u16(reply.status).unwrap(),
                reply.headers,
                reply.body,
            )
        }
        let state = MockState::default();
        let router = Router::new().fallback(handler).with_state(state.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        Self {
            address,
            state,
            task,
        }
    }
    fn push(&self, reply: Reply) {
        self.state.replies.lock().unwrap().push_back(reply);
    }
    fn json(&self, body: Value) {
        self.push(Reply::json(body));
    }
    fn count(&self) -> usize {
        self.state.seen.lock().unwrap().len()
    }
}

async fn scope(conn: &sea_orm::DatabaseConnection, account: i32, address: &str) -> Scope {
    let inbox = tickets::create_inbox(conn, account, "Support", address)
        .await
        .unwrap();
    Scope {
        account_id: account,
        inbox_id: inbox.id,
    }
}

async fn client(conn: &sea_orm::DatabaseConnection, scope: Scope, mock: &Mock) -> ResendClient {
    ResendClient::local_mock(conn, scope, mock.address, Duration::from_secs(5))
        .await
        .unwrap()
}

fn outgoing() -> SendEmail {
    SendEmail {
        to: vec!["customer@example.com".into()],
        cc: vec![],
        bcc: vec![],
        reply_to: vec![],
        subject: "Re: invoice".into(),
        text: Some("Received, thanks.".into()),
        html: None,
        message_id: "reply-1@example.com".into(),
        in_reply_to: Some("<original@example.com>".into()),
        references: vec!["first@example.com".into(), "original@example.com".into()],
        idempotency_key: "ops-email/approved-operation-1".into(),
    }
}

fn received(id: &str, source_id: &str) -> Value {
    json!({"object":"email", "id":id, "from":"Customer <customer@example.com>",
        "to":["support@example.com"], "cc":null, "bcc":null, "reply_to":null,
        "received_for":["support@example.com"], "subject":"Invoice", "message_id":source_id,
        "created_at":"2023-04-07 23:13:52.669661+00", "text":"Question", "html":null,
        "headers":{"Message-ID":format!("<{source_id}>")}, "attachments":[]})
}

fn page(items: Vec<Value>, more: bool) -> Value {
    json!({"object":"list","has_more":more,"data":items})
}

#[tokio::test]
async fn send_binds_from_and_reuses_exact_headers_payload_and_idempotency_key() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    let client = client(&db.conn, scope, &mock).await;
    let mut email = outgoing();
    email.cc.push("cc@example.com".into());
    email.bcc.push("bcc@example.com".into());
    email.reply_to.push("reply@example.com".into());
    for _ in 0..2 {
        mock.json(json!({"id":A}));
        assert_eq!(client.send(&db.conn, &email).await.unwrap().id.as_str(), A);
    }
    let seen = mock.state.seen.lock().unwrap();
    assert_eq!(seen.len(), 2);
    assert_eq!(seen[0].body, seen[1].body);
    assert_eq!(seen[0].method, "POST");
    assert_eq!(seen[0].uri, "/emails");
    for request in seen.iter() {
        assert_eq!(request.headers["authorization"], "Bearer local-mock-token");
        assert_eq!(request.headers["idempotency-key"], email.idempotency_key);
    }
    let body: Value = serde_json::from_slice(&seen[0].body).unwrap();
    assert_eq!(
        body,
        json!({"from":"support@example.com", "to":["customer@example.com"],
        "cc":["cc@example.com"],"bcc":["bcc@example.com"],"reply_to":["reply@example.com"],
        "subject":"Re: invoice", "text":"Received, thanks.", "headers":{
            "Message-ID":"<reply-1@example.com>","In-Reply-To":"<original@example.com>",
            "References":"<first@example.com> <original@example.com>"}})
    );
}

#[tokio::test]
async fn recipient_and_header_injection_and_invalid_send_inputs_never_reach_http() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    let client = client(&db.conn, scope, &mock).await;
    for address in [
        "ok@example.com\r\nBcc: secret@example.com",
        "a@example.com,b@example.com",
        "Group:a@example.com;",
        "Name <a@example.com>",
        "a@b@c",
        "a@-bad.test",
        "a..b@example.com",
        "a@example.com\0",
        "\"a b\"@example.com",
        "a@[127.0.0.1]",
    ] {
        for field in 0..4 {
            let mut email = outgoing();
            match field {
                0 => email.to = vec![address.into()],
                1 => email.cc = vec![address.into()],
                2 => email.bcc = vec![address.into()],
                _ => email.reply_to = vec![address.into()],
            }
            assert!(
                client.send(&db.conn, &email).await.is_err(),
                "field {field}"
            );
        }
    }
    let cases: Vec<SendEmail> = vec![
        SendEmail {
            to: vec![],
            ..outgoing()
        },
        SendEmail {
            bcc: vec!["a@example.com".into(); 50],
            ..outgoing()
        },
        SendEmail {
            subject: "hi\nBcc: secret@example.com".into(),
            ..outgoing()
        },
        SendEmail {
            message_id: "<x@y>\r\nTo: secret@example.com".into(),
            ..outgoing()
        },
        SendEmail {
            in_reply_to: Some("<x@y> <z@y>".into()),
            ..outgoing()
        },
        SendEmail {
            references: vec!["x@y\nBcc:z@y".into()],
            ..outgoing()
        },
        SendEmail {
            idempotency_key: "".into(),
            ..outgoing()
        },
        SendEmail {
            idempotency_key: "two keys".into(),
            ..outgoing()
        },
        SendEmail {
            idempotency_key: "x".repeat(257),
            ..outgoing()
        },
        SendEmail {
            idempotency_key: "x\r\nAuthorization: other".into(),
            ..outgoing()
        },
        SendEmail {
            text: None,
            html: None,
            ..outgoing()
        },
        SendEmail {
            text: Some("x".repeat(150_001)),
            ..outgoing()
        },
    ];
    for email in cases {
        assert!(client.send(&db.conn, &email).await.is_err());
    }
    assert_eq!(mock.count(), 0);
}

#[tokio::test]
async fn wrong_account_and_changed_inbox_fail_without_network() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    assert!(matches!(
        ResendClient::new(
            &db.conn,
            Scope {
                account_id: 2,
                ..scope
            },
            "fake"
        )
        .await,
        Err(Error::Inbox)
    ));
    assert!(ResendClient::new(&db.conn, scope, "fake\r\ninjected")
        .await
        .is_err());
    let client = client(&db.conn, scope, &mock).await;
    let mut inbox = tickets::require_inbox(&db.conn, scope)
        .await
        .unwrap()
        .into_active_model();
    inbox.email_address = Set("different@example.com".into());
    inbox.update(&db.conn).await.unwrap();
    assert!(matches!(
        client.send(&db.conn, &outgoing()).await,
        Err(Error::Inbox)
    ));
    assert!(matches!(
        client.pull_received(&db.conn, PullOptions::default()).await,
        Err(Error::Inbox)
    ));
    assert_eq!(mock.count(), 0);
}

#[tokio::test]
async fn received_list_and_detail_match_official_queries_and_nullable_shapes() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    let client = client(&db.conn, scope, &mock).await;
    let id = EmailId::parse(A).unwrap();
    for cursor in [
        None,
        Some(Cursor::After(id.clone())),
        Some(Cursor::Before(id.clone())),
    ] {
        mock.json(page(vec![received(A, "first@example.com")], false));
        let result = client
            .list_received(ListOptions { limit: 10, cursor })
            .await
            .unwrap();
        assert_eq!(result.data[0].id, id);
    }
    let mut detail = received(A, "first@example.com");
    detail["headers"] = Value::Null;
    detail["subject"] = Value::Null;
    detail.as_object_mut().unwrap().remove("received_for");
    mock.json(detail);
    let result = client
        .get_received(&id)
        .await
        .unwrap()
        .into_ticket("support@example.com")
        .unwrap()
        .unwrap();
    assert_eq!(result.message_id, "first@example.com");
    assert_eq!(result.subject, "");
    let seen = mock.state.seen.lock().unwrap();
    assert_eq!(seen[0].uri, "/emails/receiving?limit=10");
    assert_eq!(seen[1].uri, format!("/emails/receiving?limit=10&after={A}"));
    assert_eq!(
        seen[2].uri,
        format!("/emails/receiving?limit=10&before={A}")
    );
    assert_eq!(
        seen[3].uri,
        format!("/emails/receiving/{A}?html_format=cid")
    );
    assert!(seen.iter().all(|r| r.method == "GET"));
}

#[test]
fn normalized_headers_decode_comments_folding_names_and_preserve_reference_order() {
    let mut detail = received(A, "opaque-provider-value");
    detail["from"] = json!("=?UTF-8?Q?Doe=2C_Jane?= <Customer@example.com> (supporter)");
    detail["to"] = json!(["Support (nested (comment)) <SUPPORT@example.com>"]);
    detail["subject"] = json!("=?UTF-8?Q?Maksu_=E2=82=AC?=");
    detail["headers"] = json!([
        {"name":"mEsSaGe-Id","value":"(ignore <fake@example.com>) <real@example.com>"},
        {"name":"In-Reply-To","value":"(outer (nested) \\) comment) <parent@example.com>\r\n \t<other@example.com>"},
        {"name":"References","value":"<oldest@example.com>\r\n <parent@example.com>"},
        {"name":"Auto-Submitted","value":"auto-replied"}
    ]);
    let mail = serde_json::from_value::<ReceivedEmail>(detail)
        .unwrap()
        .into_ticket("support@example.com")
        .unwrap()
        .unwrap();
    assert_eq!(mail.message_id, "real@example.com");
    assert_eq!(mail.sender_email, "customer@example.com");
    assert!(mail.sender_name.unwrap().contains("Doe, Jane"));
    assert_eq!(mail.subject, "Maksu €");
    assert_eq!(mail.headers.receivers, ["support@example.com"]);
    assert_eq!(
        mail.headers.in_reply_to,
        ["parent@example.com", "other@example.com"]
    );
    assert_eq!(
        mail.headers.references,
        ["oldest@example.com", "parent@example.com"]
    );
    assert!(mail
        .headers
        .raw_in_reply_to
        .unwrap()
        .contains("<parent@example.com> \t<other@example.com>"));
    assert!(mail.auto_reply);
}

#[test]
fn foreign_to_and_forged_received_for_or_raw_to_cannot_select_an_inbox() {
    let mut detail = received(A, "mail@example.com");
    detail["to"] = json!(["other@example.com"]);
    detail["headers"]["To"] = json!("support@example.com");
    detail["received_for"] = json!(["support@example.com"]);
    assert!(serde_json::from_value::<ReceivedEmail>(detail.clone())
        .unwrap()
        .into_ticket("support@example.com")
        .unwrap()
        .is_none());
    detail["bcc"] = json!(["Support <support@example.com>"]);
    assert!(serde_json::from_value::<ReceivedEmail>(detail)
        .unwrap()
        .into_ticket("support@example.com")
        .unwrap()
        .is_some());
}

#[test]
fn malformed_and_duplicate_headers_and_missing_rfc_identity_are_rejected() {
    for headers in [
        json!({"Message-ID":"<a@b>\r\nBcc: secret@b"}),
        json!({"Message-ID":"<a@b>","message-id":"<c@d>"}),
        json!([{"name":"References","value":"<a@b>"},{"name":"references","value":"<c@d>"}]),
        json!({"Bad\nName":"value"}),
        json!({"Message-ID":17}),
    ] {
        let mut detail = received(A, "x@example.com");
        detail["headers"] = headers;
        assert!(serde_json::from_value::<ReceivedEmail>(detail).is_err());
    }
    let same_key = "{\"message-id\":\"<a@b>\",\"message-id\":\"<c@d>\"}";
    assert!(serde_json::from_str::<ReceivedHeaders>(same_key).is_err());
    for id in [
        "",
        "opaque-id",
        "<a@b> <c@d>",
        "<a@b> junk",
        "(unclosed <a@b>",
        "<a b@c>",
    ] {
        let mut detail = received(A, id);
        detail["headers"] = json!({"Message-ID":id});
        assert!(
            serde_json::from_value::<ReceivedEmail>(detail)
                .unwrap()
                .into_ticket("support@example.com")
                .is_err(),
            "{id}"
        );
    }
    let mut detail = received(A, "unused@example.com");
    detail["headers"] = Value::Null;
    detail["message_id"] = Value::Null;
    assert!(serde_json::from_value::<ReceivedEmail>(detail)
        .unwrap()
        .into_ticket("support@example.com")
        .is_err());
}

#[test]
fn html_only_becomes_plain_text_and_explicit_text_wins() {
    let mut detail = received(A, "body@example.com");
    detail["html"] = json!("<p>Hello &amp; thanks</p><img src='https://example.com/track'>");
    let mail = serde_json::from_value::<ReceivedEmail>(detail.clone())
        .unwrap()
        .into_ticket("support@example.com")
        .unwrap()
        .unwrap();
    assert_eq!(mail.content, "Question");
    detail["text"] = Value::Null;
    let mail = serde_json::from_value::<ReceivedEmail>(detail)
        .unwrap()
        .into_ticket("support@example.com")
        .unwrap()
        .unwrap();
    assert_eq!(mail.content.trim(), "Hello & thanks");
    assert!(!mail.content.contains("<img"));
}

#[tokio::test]
async fn malformed_pagination_and_response_identity_fail_explicitly() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    let client = client(&db.conn, scope, &mock).await;
    for limit in [0, 101] {
        assert!(client
            .list_received(ListOptions {
                limit,
                cursor: None
            })
            .await
            .is_err());
    }
    for id in ["", "../emails", "x?after=secret", "not-a-uuid"] {
        assert!(EmailId::parse(id).is_err());
    }
    assert_eq!(mock.count(), 0);
    mock.json(page(vec![], true));
    assert!(matches!(
        client.list_received(ListOptions::default()).await,
        Err(Error::Pagination)
    ));
    mock.json(json!({"object":"other","has_more":false,"data":[]}));
    assert!(client.list_received(ListOptions::default()).await.is_err());
    mock.json(received(B, "body@example.com"));
    assert!(matches!(
        client.get_received(&EmailId::parse(A).unwrap()).await,
        Err(Error::Response("received detail identity"))
    ));
}

#[tokio::test]
async fn transport_errors_are_redacted_and_never_retry_or_follow_redirects() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    let client = client(&db.conn, scope, &mock).await;
    for status in [401, 403, 404, 409, 422, 429, 500, 503, 302, 307] {
        let mut reply = Reply::json(json!({"message":"secret-provider-body customer@example.com"}));
        reply.status = status;
        reply.headers.insert("retry-after", "7".parse().unwrap());
        reply.headers.insert(
            "location",
            format!("http://{}/must-not-follow", mock.address)
                .parse()
                .unwrap(),
        );
        mock.push(reply);
        let error = client.send(&db.conn, &outgoing()).await.err().unwrap();
        assert_eq!(
            error,
            Error::Http {
                status,
                retry_after_seconds: Some(7)
            }
        );
        assert!(!format!("{error:?} {error}").contains("secret"));
    }
    assert_eq!(mock.count(), 10);
    let mut reply = Reply::json(json!({}));
    reply.body = "not JSON secret".into();
    mock.push(reply);
    assert!(matches!(
        client.list_received(ListOptions::default()).await,
        Err(Error::Response("JSON schema"))
    ));
}

#[tokio::test]
async fn response_size_and_timeout_are_bounded_and_connection_errors_are_safe() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    let client = client(&db.conn, scope, &mock).await;
    let mut reply = Reply::json(json!({}));
    reply.body = "x".repeat(super::client::MAX_RESPONSE + 1);
    mock.push(reply);
    assert!(matches!(
        client.list_received(ListOptions::default()).await,
        Err(Error::Response("body size"))
    ));
    let mut reply = Reply::json(page(vec![], false));
    reply.delay = Duration::from_millis(250);
    mock.push(reply);
    let client = ResendClient::local_mock(&db.conn, scope, mock.address, Duration::from_millis(50))
        .await
        .unwrap();
    assert!(matches!(
        client.list_received(ListOptions::default()).await,
        Err(Error::Transport { timeout: true })
    ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let disconnected = ResendClient::local_mock(&db.conn, scope, address, Duration::from_secs(1))
        .await
        .unwrap();
    assert!(matches!(
        disconnected.list_received(ListOptions::default()).await,
        Err(Error::Transport { .. })
    ));
}

#[test]
fn malformed_senders_and_decoded_control_characters_are_rejected() {
    for from in [
        "Name <sender@example.com",
        "Name <sender@example.com>>",
        "(unclosed sender@example.com",
        "\"unclosed name <sender@example.com>",
        "first@example.com, second@example.com",
        "Group:sender@example.com;",
        "sender without an address",
        "sender@example.com\r\nBcc: secret@example.com",
        "=?UTF-8?Q?Name=0ABcc=3Asecret?= <sender@example.com>",
    ] {
        let mut detail = received(A, "sender@example.com");
        detail["from"] = json!(from);
        assert!(serde_json::from_value::<ReceivedEmail>(detail)
            .unwrap()
            .into_ticket("support@example.com")
            .is_err());
    }
}

#[test]
fn auto_reply_matches_pinned_chatwoot_presenter_predicates() {
    for (headers, expected) in [
        (json!({"Auto-Submitted":"auto-replied"}), true),
        (json!({"Auto-Submitted":"no"}), false),
        (json!({"X-Autoreply":"yes"}), true),
        (json!({"X-Autoreply":"no"}), false),
        (json!({"Auto-Submitted":"no", "X-Autoreply":"yes"}), true),
        (json!({}), false),
    ] {
        let mut detail = received(A, "reply@example.com");
        detail["headers"] = headers;
        let mail = serde_json::from_value::<ReceivedEmail>(detail)
            .unwrap()
            .into_ticket("support@example.com")
            .unwrap()
            .unwrap();
        assert_eq!(mail.auto_reply, expected);
    }
}

#[test]
fn reply_to_precedes_from_and_preserves_encoded_display_name() {
    for (envelope, header) in [
        (json!(["Reporter <reporter@example.test>"]), Value::Null),
        (
            Value::Null,
            json!("=?UTF-8?Q?Doe=2C_Jane?= <reporter@example.test>"),
        ),
        (
            json!(["REPORTER@example.test"]),
            json!("=?UTF-8?Q?Doe=2C_Jane?= <reporter@example.test>"),
        ),
        (json!([]), json!("Reporter <reporter@example.test>")),
    ] {
        let mut detail = received(A, "reply-target@example.test");
        detail["from"] = json!("Service Robot <noreply@service.test>");
        detail["reply_to"] = envelope;
        if !header.is_null() {
            detail["headers"]["Reply-To"] = header.clone();
        }
        let mail = serde_json::from_value::<ReceivedEmail>(detail)
            .unwrap()
            .into_ticket("support@example.com")
            .unwrap()
            .unwrap();
        assert_eq!(mail.sender_email, "reporter@example.test");
        assert_eq!(
            mail.sender_name.as_deref(),
            Some(if header.as_str().is_some_and(|s| s.starts_with("=?")) {
                "Doe, Jane"
            } else {
                "Reporter"
            })
        );
    }
}

#[test]
fn absent_or_empty_reply_to_falls_back_to_from_but_ambiguity_fails_closed() {
    for value in [Value::Null, json!([]), json!([""]), json!([" \t "])] {
        let mut detail = received(A, "empty@example.test");
        detail["reply_to"] = value;
        detail["headers"]["Reply-To"] = json!(" \t ");
        let mail = serde_json::from_value::<ReceivedEmail>(detail)
            .unwrap()
            .into_ticket("support@example.com")
            .unwrap()
            .unwrap();
        assert_eq!(mail.sender_email, "customer@example.com");
        assert_eq!(mail.sender_name.as_deref(), Some("Customer"));
    }
    for (envelope, header) in [
        (json!(["Reporter <reporter@example.test"]), Value::Null),
        (json!(["a@example.test", "b@example.test"]), Value::Null),
        (json!(["a@example.test,b@example.test"]), Value::Null),
        (Value::Null, json!("a@example.test,b@example.test")),
        (
            json!(["reporter@example.test"]),
            json!("Reporter <reporter@example.test"),
        ),
        (
            json!(["reporter@example.test"]),
            json!("different@example.test"),
        ),
        (
            json!(["reporter@example.test"]),
            json!("Group:reporter@example.test;"),
        ),
    ] {
        let mut detail = received(A, "ambiguous@example.test");
        detail["reply_to"] = envelope;
        if !header.is_null() {
            detail["headers"]["Reply-To"] = header;
        }
        assert!(serde_json::from_value::<ReceivedEmail>(detail)
            .unwrap()
            .into_ticket("support@example.com")
            .is_err());
    }
}

#[tokio::test]
async fn pulled_reply_to_is_the_persisted_ticket_contact_and_cannot_route_an_inbox() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    let client = client(&db.conn, scope, &mock).await;
    let mut detail = received(A, "actual-reporter@example.test");
    detail["from"] = json!("Service Robot <noreply@service.test>");
    detail["reply_to"] = json!(["reporter@example.test"]);
    detail["headers"]["Reply-To"] = json!("=?UTF-8?Q?Doe=2C_Jane?= <reporter@example.test>");
    mock.json(page(vec![detail.clone()], false));
    mock.json(detail.clone());
    let summary = client
        .pull_received(&db.conn, PullOptions::default())
        .await
        .unwrap();
    assert_eq!(summary.inserted, 1);
    let conversation = tickets::list_conversations(&db.conn, scope)
        .await
        .unwrap()
        .remove(0);
    let contact = tickets::get_contact(&db.conn, scope, conversation.id)
        .await
        .unwrap();
    assert_eq!(contact.email, "reporter@example.test");
    assert_eq!(contact.name, "Doe, Jane");
    detail["to"] = json!(["another-inbox@example.com"]);
    detail["reply_to"] = json!(["support@example.com"]);
    detail["headers"]["Reply-To"] = json!("support@example.com");
    assert!(serde_json::from_value::<ReceivedEmail>(detail)
        .unwrap()
        .into_ticket("support@example.com")
        .unwrap()
        .is_none());
}

#[tokio::test]
async fn pull_paginates_orders_parent_before_reply_and_replays_without_duplicates() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    let client = client(&db.conn, scope, &mock).await;
    let parent = received(B, "parent@example.com");
    let mut child = received(A, "child@example.com"); // Sorts BEFORE its parent at the same timestamp.
    child["headers"]["In-Reply-To"] = json!("<parent@example.com>");
    let mut foreign = received(C, "foreign@example.com");
    foreign["to"] = json!(["private@example.com"]);
    for pass in 0..2 {
        mock.json(page(vec![child.clone(), parent.clone()], true));
        mock.json(page(vec![parent.clone(), foreign.clone()], false)); // Overlapping page.
        mock.json(child.clone());
        mock.json(parent.clone());
        mock.json(foreign.clone());
        let result = client
            .pull_received(
                &db.conn,
                PullOptions {
                    page_size: 2,
                    max_pages: 3,
                },
            )
            .await
            .unwrap();
        assert_eq!(
            result,
            PullSummary {
                listed: 3,
                skipped_other_inbox: 1,
                inserted: if pass == 0 { 2 } else { 0 },
                duplicates: if pass == 0 { 0 } else { 2 }
            }
        );
    }
    let conversations = tickets::list_conversations(&db.conn, scope).await.unwrap();
    assert_eq!(conversations.len(), 1);
    let messages = tickets::list_messages(
        &db.conn,
        scope,
        conversations[0].id,
        tickets::MessageView::Internal,
    )
    .await
    .unwrap();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].source_id.as_deref(), Some("parent@example.com"));
    assert_eq!(messages[1].source_id.as_deref(), Some("child@example.com"));
    assert_eq!(
        mock.state.seen.lock().unwrap()[1].uri,
        format!("/emails/receiving?limit=2&after={B}")
    );
}

#[tokio::test]
async fn incomplete_stalled_cyclic_or_malformed_pull_does_not_write_partial_history() {
    let db = fresh_in_memory_db().await;
    let scope = scope(&db.conn, 1, "support@example.com").await;
    let mock = Mock::start().await;
    let client = client(&db.conn, scope, &mock).await;
    let first = received(A, "first@example.com");
    mock.json(page(vec![first.clone()], true));
    assert!(matches!(
        client
            .pull_received(
                &db.conn,
                PullOptions {
                    page_size: 1,
                    max_pages: 1
                }
            )
            .await,
        Err(Error::PullLimit)
    ));
    for _ in 0..2 {
        mock.json(page(vec![first.clone()], true));
    }
    assert!(matches!(
        client.pull_received(&db.conn, PullOptions::default()).await,
        Err(Error::Pagination)
    ));
    let mut bad = received(B, "bad@example.com");
    bad["headers"]["References"] = json!("not an ID");
    mock.json(page(vec![first.clone(), bad.clone()], false));
    mock.json(first.clone());
    mock.json(bad);
    assert!(client
        .pull_received(&db.conn, PullOptions::default())
        .await
        .is_err());
    let mut first_cycle = first.clone();
    first_cycle["headers"]["References"] = json!("<second@example.com>");
    let mut second = received(B, "second@example.com");
    second["headers"]["References"] = json!("<first@example.com>");
    mock.json(page(vec![first_cycle.clone(), second.clone()], false));
    mock.json(first_cycle);
    mock.json(second);
    assert!(matches!(
        client.pull_received(&db.conn, PullOptions::default()).await,
        Err(Error::Response("cyclic received references"))
    ));
    assert!(tickets::list_conversations(&db.conn, scope)
        .await
        .unwrap()
        .is_empty());
}
