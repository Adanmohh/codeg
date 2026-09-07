//! Direct REST patterns ported from IntroMail resend_client.py; see NOTICE.
use std::{collections::BTreeMap, time::Duration};

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, RequestBuilder,
};
use sea_orm::DatabaseConnection;
use serde::{de::DeserializeOwned, Serialize};

use super::{
    normalize::{mailbox, single_line, wire_id, MAX_CONTENT},
    Cursor, EmailId, Error, ListOptions, ReceivedEmail, ReceivedPage, SendEmail, SentEmail,
};
use crate::db::service::ticket_service::{require_inbox, Scope};

pub(super) const MAX_RESPONSE: usize = 8 * 1024 * 1024;

pub struct ResendClient {
    http: Client,
    base: String,
    pub(super) scope: Scope,
    pub(super) inbox_email: String,
}

impl ResendClient {
    /// Supply the selected inbox's secret from the existing credential store.
    /// The client neither persists it nor discovers credentials from environment.
    pub async fn new(
        conn: &DatabaseConnection,
        scope: Scope,
        api_key: &str,
    ) -> Result<Self, Error> {
        Self::build(
            conn,
            scope,
            api_key,
            "https://api.resend.com".into(),
            Duration::from_secs(30),
            false,
        )
        .await
    }

    async fn build(
        conn: &DatabaseConnection,
        scope: Scope,
        api_key: &str,
        base: String,
        timeout: Duration,
        local_mock: bool,
    ) -> Result<Self, Error> {
        let inbox = require_inbox(conn, scope).await.map_err(|_| Error::Inbox)?;
        if inbox.channel_type != "Channel::Email" {
            return Err(Error::Inbox);
        }
        let inbox_email = mailbox(&inbox.email_address).map_err(|_| Error::Inbox)?;
        if api_key.is_empty()
            || api_key.len() > 4096
            || !api_key.bytes().all(|b| b.is_ascii_graphic())
        {
            return Err(Error::InvalidInput("credential"));
        }
        let mut auth = HeaderValue::from_str(&format!("Bearer {api_key}"))
            .map_err(|_| Error::InvalidInput("credential"))?;
        auth.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth);
        let mut builder = Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never());
        if local_mock {
            builder = builder.no_proxy();
        }
        let http = builder.build().map_err(transport_error)?;
        Ok(Self {
            http,
            base,
            scope,
            inbox_email,
        })
    }

    /// The only endpoint injection seam is test-only and loopback-only. Production
    /// callers cannot redirect a credential to an arbitrary endpoint.
    #[cfg(test)]
    pub(super) async fn local_mock(
        conn: &DatabaseConnection,
        scope: Scope,
        address: std::net::SocketAddr,
        timeout: Duration,
    ) -> Result<Self, Error> {
        if !address.ip().is_loopback() {
            return Err(Error::InvalidInput("mock loopback address"));
        }
        Self::build(
            conn,
            scope,
            "local-mock-token",
            format!("http://{address}"),
            timeout,
            true,
        )
        .await
    }

    pub(super) async fn check_inbox(&self, conn: &DatabaseConnection) -> Result<(), Error> {
        let inbox = require_inbox(conn, self.scope)
            .await
            .map_err(|_| Error::Inbox)?;
        if inbox.channel_type != "Channel::Email" || inbox.email_address != self.inbox_email {
            return Err(Error::Inbox);
        }
        Ok(())
    }

    /// Credential-wide metadata, not an inbox/user authorization boundary.
    pub async fn list_received(&self, options: ListOptions) -> Result<ReceivedPage, Error> {
        if !(1..=100).contains(&options.limit) {
            return Err(Error::InvalidInput("page limit 1..100"));
        }
        let mut query = vec![("limit", options.limit.to_string())];
        match options.cursor {
            Some(Cursor::After(id)) => query.push(("after", id.0)),
            Some(Cursor::Before(id)) => query.push(("before", id.0)),
            None => {}
        }
        let page: ReceivedPage = self
            .json(
                self.http
                    .get(format!("{}/emails/receiving", self.base))
                    .query(&query),
            )
            .await?;
        if page.object != "list" || page.data.len() > usize::from(options.limit) {
            return Err(Error::Response("received list shape"));
        }
        if page.has_more && page.data.is_empty() {
            return Err(Error::Pagination);
        }
        Ok(page)
    }

    /// Raw provider detail, reserved for trusted transport code. `into_ticket`
    /// checks recipients before scoped persistence. Signed URLs are never fetched.
    pub async fn get_received(&self, id: &EmailId) -> Result<ReceivedEmail, Error> {
        let email: ReceivedEmail = self
            .json(
                self.http
                    .get(format!("{}/emails/receiving/{}", self.base, id.as_str()))
                    .query(&[("html_format", "cid")]),
            )
            .await?;
        if email.object != "email" || email.envelope.id != *id {
            return Err(Error::Response("received detail identity"));
        }
        Ok(email)
    }

    /// Trusted low-level network operation, NOT an approval check. Only a future
    /// approval dispatcher may call this; no route/command registers this method.
    /// Do not generate a new key or Message-ID after a timeout: persist and reuse
    /// the same key AND payload. Provider idempotency is not an eternal send ledger.
    pub async fn send(
        &self,
        conn: &DatabaseConnection,
        email: &SendEmail,
    ) -> Result<SentEmail, Error> {
        let payload = SendPayload::validated(&self.inbox_email, email)?;
        self.check_inbox(conn).await?;
        self.json(
            self.http
                .post(format!("{}/emails", self.base))
                .header("Idempotency-Key", &email.idempotency_key)
                .json(&payload),
        )
        .await
    }

    async fn json<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T, Error> {
        let mut response = request.send().await.map_err(transport_error)?;
        if !response.status().is_success() {
            return Err(Error::Http {
                status: response.status().as_u16(),
                retry_after_seconds: response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse().ok()),
            });
        }
        if response
            .content_length()
            .is_some_and(|size| size > MAX_RESPONSE as u64)
        {
            return Err(Error::Response("body size"));
        }
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(transport_error)? {
            if body.len() + chunk.len() > MAX_RESPONSE {
                return Err(Error::Response("body size"));
            }
            body.extend_from_slice(&chunk);
        }
        serde_json::from_slice(&body).map_err(|_| Error::Response("JSON schema"))
    }
}

fn transport_error(error: reqwest::Error) -> Error {
    Error::Transport {
        timeout: error.is_timeout(),
    }
}

#[derive(Serialize)]
struct SendPayload<'a> {
    from: &'a str,
    to: &'a [String],
    #[serde(skip_serializing_if = "<[String]>::is_empty")]
    cc: &'a [String],
    #[serde(skip_serializing_if = "<[String]>::is_empty")]
    bcc: &'a [String],
    #[serde(skip_serializing_if = "<[String]>::is_empty")]
    reply_to: &'a [String],
    subject: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<&'a str>,
    headers: BTreeMap<&'static str, String>,
}

impl<'a> SendPayload<'a> {
    fn validated(from: &'a str, email: &'a SendEmail) -> Result<Self, Error> {
        let count = email.to.len() + email.cc.len() + email.bcc.len();
        if email.to.is_empty() || count > 50 || email.reply_to.len() > 50 {
            return Err(Error::InvalidInput("1..50 recipients"));
        }
        for addr in email
            .to
            .iter()
            .chain(&email.cc)
            .chain(&email.bcc)
            .chain(&email.reply_to)
        {
            mailbox(addr)?;
        }
        single_line(&email.subject)?;
        if email.text.as_deref().is_none_or(|s| s.trim().is_empty())
            && email.html.as_deref().is_none_or(|s| s.trim().is_empty())
        {
            return Err(Error::InvalidInput("email body required"));
        }
        if email
            .text
            .iter()
            .chain(&email.html)
            .any(|body| body.chars().count() > MAX_CONTENT)
        {
            return Err(Error::InvalidInput("email body size"));
        }
        if email.idempotency_key.is_empty()
            || email.idempotency_key.len() > 256
            || !email.idempotency_key.bytes().all(|b| b.is_ascii_graphic())
        {
            return Err(Error::InvalidInput("idempotency key"));
        }
        if email.references.len() > 100 {
            return Err(Error::InvalidInput("reference count"));
        }
        let mut headers = BTreeMap::from([("Message-ID", wire_id(&email.message_id)?)]);
        if let Some(id) = &email.in_reply_to {
            headers.insert("In-Reply-To", wire_id(id)?);
        }
        if !email.references.is_empty() {
            let references = email
                .references
                .iter()
                .map(|s| wire_id(s))
                .collect::<Result<Vec<_>, _>>()?
                .join(" ");
            single_line(&references)?;
            headers.insert("References", references);
        }
        Ok(Self {
            from,
            to: &email.to,
            cc: &email.cc,
            bcc: &email.bcc,
            reply_to: &email.reply_to,
            subject: &email.subject,
            text: email.text.as_deref(),
            html: email.html.as_deref(),
            headers,
        })
    }
}
