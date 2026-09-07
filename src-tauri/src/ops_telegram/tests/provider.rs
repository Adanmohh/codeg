//! Accepted Ops/Resend fixture pattern, HTTP transport real, loopback only.
use super::*;
use axum::{
    body::to_bytes,
    extract::{Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde_json::{json, Value};
use std::{collections::VecDeque, sync::Mutex};

type Reply = (StatusCode, Value, Duration);
#[derive(Clone, Default)]
pub(crate) struct ProviderState {
    pub seen: Arc<Mutex<Vec<(String, Value)>>>,
    pub replies: Arc<Mutex<VecDeque<Reply>>>,
    pub started: Arc<tokio::sync::Notify>,
    pub release: Arc<tokio::sync::Notify>,
    pub pause_check: Arc<std::sync::atomic::AtomicBool>,
}
pub(crate) struct Provider {
    pub state: ProviderState,
    pub runtime: Arc<TelegramRuntime>,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Provider {
    fn drop(&mut self) {
        self.task.abort();
    }
}
impl Provider {
    pub async fn new() -> Self {
        async fn handle(State(state): State<ProviderState>, request: Request) -> impl IntoResponse {
            let (head, body) = request.into_parts();
            let body: Value =
                serde_json::from_slice(&to_bytes(body, 65536).await.unwrap()).unwrap();
            let method = head.uri.path().to_string();
            assert!(
                matches!(method.as_str(), "/getChat" | "/sendMessage"),
                "polling or unrelated API must never start"
            );
            state.seen.lock().unwrap().push((method.clone(), body));
            if method == "/getChat" && state.pause_check.load(std::sync::atomic::Ordering::SeqCst) {
                state.started.notify_one();
                state.release.notified().await;
            }
            let reply = state.replies.lock().unwrap().pop_front();
            let (status, body, delay) = reply.unwrap_or_else(|| (
                StatusCode::OK,
                if method == "/getChat" { json!({"ok":true,"result":{"id":123,"type":"private"}}) }
                else { json!({"ok":true,"result":{"message_id":41,"chat":{"id":123,"type":"private"},"from":{"id":999,"is_bot":true}}}) },
                Duration::ZERO,
            ));
            tokio::time::sleep(delay).await;
            (status, Json(body))
        }
        let state = ProviderState::default();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let runtime = TelegramRuntime::fixture(
            |_| Some("synthetic-only".into()),
            listener.local_addr().unwrap(),
        );
        let router = Router::new().fallback(handle).with_state(state.clone());
        let task = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        Self {
            state,
            runtime,
            task,
        }
    }
    pub fn sends(&self) -> usize {
        self.state
            .seen
            .lock()
            .unwrap()
            .iter()
            .filter(|(m, _)| m == "/sendMessage")
            .count()
    }
    pub fn reply(&self, status: StatusCode, body: Value, delay: Duration) {
        self.state
            .replies
            .lock()
            .unwrap()
            .push_back((status, body, delay));
    }
}
