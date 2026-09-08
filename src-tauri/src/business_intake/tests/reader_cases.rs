use super::{
    super::{error::Reason, fireflies::Reader, types::*},
    support::*,
};
use serde_json::json;
use std::time::Duration;

#[tokio::test]
async fn intake_reader_fixed_queries_scope_headers_and_exact_normalization() {
    let mock = Mock::start().await;
    let reader = Reader::fixture(mock.address, Duration::from_secs(1)).unwrap();
    mock.json(json!({"data":{"user":{"user_id":"source-user"}}}));
    assert_eq!(reader.user("synthetic-key").await.unwrap(), "source-user");
    mock.json(json!({"data":{"transcripts":[{"id":"record-1","title":"Meeting"}]}}));
    let rows = reader
        .list(
            "synthetic-key",
            "source-user",
            "2026-09-01T00:00:00Z",
            "2026-09-08T00:00:00Z",
            0,
        )
        .await
        .unwrap();
    assert_eq!(rows[0].id, "record-1");
    mock.json(transcript("record-1", "Exact quotation\nwith line break"));
    let value = reader
        .detail("synthetic-key", "source-user", "record-1")
        .await
        .unwrap();
    assert_eq!(value.passages[0].text, "Exact quotation\nwith line break");
    assert_eq!(value.passages[0].index, Some(7));
    assert_eq!(value.passages[0].start, Some(2.5));
    assert_eq!(value.summary, SummaryState::Available);
    let requests = mock.requests();
    assert!(requests
        .iter()
        .all(|r| r["query"].as_str().unwrap().starts_with("query ")));
    assert_eq!(requests[1]["variables"]["mine"], true);
    assert_eq!(requests[1]["variables"]["limit"], 50);
    assert_eq!(requests[1]["variables"]["skip"], 0);
    assert_eq!(requests[1]["variables"]["userId"], "source-user");
    assert_eq!(requests[2]["variables"], json!({"transcriptId":"record-1"}));
    assert!(reader.user("bad\r\nX-injected: yes").await.is_err());
    assert_eq!(mock.count(), 3);
}

#[tokio::test]
async fn intake_reader_rejects_partial_errors_scope_mismatch_expiry_and_malformed_metadata() {
    let mock = Mock::start().await;
    let reader = Reader::fixture(mock.address, Duration::from_secs(1)).unwrap();
    for value in [
        json!({"data":{"user":{"user_id":"source-user"}},"errors":[{"message":"private upstream"}]}),
        json!({"data":{"user":{"user_id":"auth_failed"}}}),
        json!({"data":{"user":{"user_id":" "}}}),
    ] {
        mock.json(value);
        assert!(reader.user("synthetic-key").await.is_err());
    }
    for value in [
        json!({"data":{"transcripts":null}}),
        json!({"data":{"transcripts":[{"title":"missing ID"}]}}),
    ] {
        mock.json(value);
        assert!(reader
            .list(
                "synthetic-key",
                "source-user",
                "2026-09-01T00:00:00Z",
                "2026-09-08T00:00:00Z",
                0
            )
            .await
            .is_err());
    }
    for (pointer, value) in [
        ("/data/transcript/id", json!("other-record")),
        ("/data/transcript/user/user_id", json!("other-user")),
        ("/data/transcript/shared_with", json!(["malformed"])),
        (
            "/data/transcript/shared_with",
            json!([{"expires_at":"2020-01-01T00:00:00Z"}]),
        ),
        ("/data/transcript/sentences/0/end_time", json!(1)),
        (
            "/data/transcript/sentences/0/text",
            json!("bad\u{0000}text"),
        ),
    ] {
        let mut input = transcript("record-1", "text");
        *input.pointer_mut(pointer).unwrap() = value;
        mock.json(input);
        assert!(reader
            .detail("synthetic-key", "source-user", "record-1")
            .await
            .is_err());
    }
    let mut denied = Reply::json(json!({"error":"private response"}));
    denied.status = 401;
    mock.reply(denied);
    assert_eq!(
        reader.user("synthetic-key").await.unwrap_err().reason,
        Reason::SourceDenied
    );
}

#[tokio::test]
async fn intake_reader_missing_empty_unsupported_summary_and_transport_caps_are_distinct() {
    let mock = Mock::start().await;
    let reader = Reader::fixture(mock.address, Duration::from_millis(50)).unwrap();
    for (summary, expected) in [
        (json!(null), SummaryState::Missing),
        (json!({"action_items":[]}), SummaryState::Empty),
        (
            json!({"action_items":{"unexpected":true}}),
            SummaryState::Unsupported,
        ),
    ] {
        let mut input = transcript("record-1", "kept");
        input["data"]["transcript"]["summary"] = summary;
        mock.json(input);
        let normalized = reader
            .detail("synthetic-key", "source-user", "record-1")
            .await
            .unwrap();
        assert_eq!(normalized.summary, expected);
        assert_eq!(normalized.passages.len(), 1);
    }
    let mut redirect = Reply::json(json!({}));
    redirect.status = 302;
    redirect.headers.insert(
        "location",
        "http://127.0.0.1:1/must-not-follow".parse().unwrap(),
    );
    mock.reply(redirect);
    assert!(reader.user("synthetic-key").await.is_err());
    assert_eq!(mock.count(), 4);
    let mut limited = Reply::json(json!({}));
    limited.status = 429;
    limited
        .headers
        .insert("retry-after", "120".parse().unwrap());
    mock.reply(limited);
    assert_eq!(
        reader.user("synthetic-key").await.unwrap_err().retry_after,
        Some(60)
    );
    let mut large = Reply::json(json!({}));
    large.body = " ".repeat(2 * 1024 * 1024 + 1);
    mock.reply(large);
    assert_eq!(
        reader.user("synthetic-key").await.unwrap_err().reason,
        Reason::UnsupportedSchema
    );
    let mut blocked = Reply::json(json!({}));
    blocked.gate = Some(std::sync::Arc::new(tokio::sync::Barrier::new(2)));
    mock.reply(blocked);
    assert_eq!(
        reader.user("synthetic-key").await.unwrap_err().reason,
        Reason::RequestTimeout
    );
    assert_eq!(mock.count(), 7);
}
