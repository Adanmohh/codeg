//! Narrow plain-send adapter over Codeg v0.30.4 TelegramBackend (NOTICE).
//! No polling, rich-message fallback, redirects or automatic request retries.
use super::TelegramBackend;
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum OpsSendError {
    Unavailable,
    Rejected,
    Unknown,
}

impl TelegramBackend {
    pub(crate) fn for_ops(
        channel_id: i32,
        token: String,
        private_user_id: i64,
    ) -> Result<Self, OpsSendError> {
        if private_user_id <= 0
            || token.is_empty()
            || token.len() > 4096
            || !token
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b':' | b'_' | b'-'))
        {
            return Err(OpsSendError::Unavailable);
        }
        let mut backend = Self::new(channel_id, token, private_user_id.to_string(), false);
        // reqwest 0.12.28 defaults to protocol-NACK retries. An ambiguous
        // Telegram send has no provider idempotency key, so explicitly disable.
        backend.client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(15))
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .no_proxy()
            .build()
            .map_err(|_| OpsSendError::Unavailable)?;
        Ok(backend)
    }

    #[cfg(test)]
    pub(crate) fn ops_local_mock(
        channel_id: i32,
        private_user_id: i64,
        address: std::net::SocketAddr,
    ) -> Self {
        assert!(address.ip().is_loopback());
        let mut backend = Self::for_ops(channel_id, "synthetic-only".into(), private_user_id)
            .expect("fixture client");
        backend.ops_mock = Some(address);
        backend
    }

    async fn ops_request(&self, method: &str, body: Value) -> Result<Value, OpsSendError> {
        let mut response = self
            .client
            .post(self.api_url(method))
            .json(&body)
            .send()
            .await
            .map_err(|_| OpsSendError::Unknown)?;
        let status = response.status();
        // Bound decompressed response bytes; never expose provider diagnostics,
        // request URLs (which contain the bot token), or response bodies.
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| OpsSendError::Unknown)? {
            if bytes.len() + chunk.len() > 64 * 1024 {
                return Err(OpsSendError::Unknown);
            }
            bytes.extend_from_slice(&chunk);
        }
        let value: Value = serde_json::from_slice(&bytes).map_err(|_| OpsSendError::Unknown)?;
        if status.is_client_error() && value["ok"] == false {
            return Err(OpsSendError::Rejected);
        }
        if !status.is_success() || value["ok"] != true {
            return Err(OpsSendError::Unknown);
        }
        Ok(value)
    }

    pub(crate) async fn verify_ops_private_recipient(&self) -> Result<(), OpsSendError> {
        let value = self
            .ops_request("getChat", json!({"chat_id": self.chat_id}))
            .await?;
        if !self.ops_private_chat(&value["result"]) {
            return Err(OpsSendError::Unavailable);
        }
        Ok(())
    }

    fn ops_private_chat(&self, chat: &Value) -> bool {
        chat["type"] == "private"
            && chat["id"]
                .as_i64()
                .is_some_and(|id| id > 0 && id.to_string() == self.chat_id)
    }

    /// Trusted host supplies a validated opaque locator. This cannot respond to
    /// ACP permission callbacks or accept a caller-selected chat/topic/payload.
    pub(crate) async fn send_ops_review_link(&self, link: &str) -> Result<String, OpsSendError> {
        let mut body = super::telegram_send_message_body(
            &self.chat_id,
            &format!("An Ops proposal is ready for review. Sign in to read the complete payload and decide.\n{link}"),
            None, None, None,
        ).map_err(|_| OpsSendError::Unavailable)?;
        body["link_preview_options"] = json!({"is_disabled": true});
        let value = self.ops_request("sendMessage", body).await?;
        let receipt = &value["result"];
        let id = receipt["message_id"].as_i64().filter(|id| *id > 0);
        if !self.ops_private_chat(&receipt["chat"])
            || receipt["is_topic_message"] == true
            || !receipt["message_thread_id"].is_null()
            || receipt["from"]["is_bot"] != true
        {
            return Err(OpsSendError::Unknown);
        }
        id.map(|id| id.to_string()).ok_or(OpsSendError::Unknown)
    }
}
