//! IntroMail header/envelope normalization, adapted to the ticket store contract.
//! Comment/escape transitions adapted from mail-parser 0.11.1 address.rs (MIT).
//! See NOTICE for immutable source references and license text.
use std::{collections::HashMap, fmt};

use mail_parser::{decoders::html::html_to_text, parsers::MessageStream, Address};
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use super::{Error, ReceivedEmail};
use crate::db::service::ticket_service::{threading::ThreadHeaders, IncomingEmail};

const MAX_HEADER: usize = 8 * 1024;
const MAX_HEADERS: usize = 64 * 1024;
pub(super) const MAX_CONTENT: usize = 150_000;

/// IntroMail accepts both Resend's header object and [{name,value}] events.
/// Preserve duplicate detection during deserialization, including same-case keys.
#[derive(Default)]
pub struct ReceivedHeaders(HashMap<String, String>);

impl ReceivedHeaders {
    fn insert(&mut self, name: String, value: String, size: &mut usize) -> Result<(), Error> {
        *size += name.len() + value.len();
        if *size > MAX_HEADERS
            || name.is_empty()
            || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err(Error::Response("header names or size"));
        }
        let name = name.to_ascii_lowercase();
        let value = unfolded(&value)?;
        if self.0.contains_key(&name)
            && matches!(
                name.as_str(),
                "message-id"
                    | "in-reply-to"
                    | "references"
                    | "from"
                    | "to"
                    | "cc"
                    | "bcc"
                    | "reply-to"
                    | "subject"
                    | "auto-submitted"
                    | "x-autoreply"
                    | "x-auto-response-suppress"
            )
        {
            return Err(Error::Response("duplicate routing header"));
        }
        self.0.entry(name).or_insert(value);
        Ok(())
    }

    fn get(&self, name: &str) -> Option<&str> {
        self.0.get(name).map(String::as_str)
    }
}

impl<'de> Deserialize<'de> for ReceivedHeaders {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct HeadersVisitor;
        impl<'de> Visitor<'de> for HeadersVisitor {
            type Value = ReceivedHeaders;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a header object, header array, or null")
            }
            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(ReceivedHeaders::default())
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut result = ReceivedHeaders::default();
                let mut size = 0;
                while let Some((name, value)) = map.next_entry::<String, String>()? {
                    result
                        .insert(name, value, &mut size)
                        .map_err(de::Error::custom)?;
                }
                Ok(result)
            }
            fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
                #[derive(Deserialize)]
                struct Header {
                    name: String,
                    value: String,
                }
                let mut result = ReceivedHeaders::default();
                let mut size = 0;
                while let Some(Header { name, value }) = seq.next_element()? {
                    result
                        .insert(name, value, &mut size)
                        .map_err(de::Error::custom)?;
                }
                Ok(result)
            }
        }
        deserializer.deserialize_any(HeadersVisitor)
    }
}

/// A field value may contain CRLF followed by WSP (folding), never a new field.
fn unfolded(value: &str) -> Result<String, Error> {
    if value.len() > MAX_HEADER {
        return Err(Error::Response("header size"));
    }
    let bytes = value.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        match b {
            b'\r'
                if bytes.get(i + 1) == Some(&b'\n')
                    && matches!(bytes.get(i + 2), Some(b' ' | b'\t')) => {}
            b'\n'
                if i > 0
                    && bytes[i - 1] == b'\r'
                    && matches!(bytes.get(i + 1), Some(b' ' | b'\t')) => {}
            b'\t' => {}
            b if b.is_ascii_control() => return Err(Error::Response("header control character")),
            _ => {}
        }
    }
    Ok(value.replace("\r\n", ""))
}

pub(super) fn single_line(value: &str) -> Result<(), Error> {
    if value.len() > MAX_HEADER || value.chars().any(char::is_control) {
        return Err(Error::InvalidInput("single-line header required"));
    }
    Ok(())
}

fn dot_atom(value: &str) -> bool {
    !value.is_empty()
        && value.split('.').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+-/=?^_`{|}~".contains(&b))
        })
}

/// Conservative, unambiguous addr-spec subset accepted by the current ticket
/// store. Quoted local parts, domain literals and SMTPUTF8 are not supported.
pub(super) fn mailbox(value: &str) -> Result<String, Error> {
    single_line(value)?;
    let (local, domain) = value
        .split_once('@')
        .ok_or(Error::InvalidInput("mailbox"))?;
    if value.len() > 254
        || local.len() > 64
        || !dot_atom(local)
        || domain.is_empty()
        || !domain.split('.').all(|part| {
            !part.is_empty()
                && part.len() <= 63
                && !part.starts_with('-')
                && !part.ends_with('-')
                && part.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        })
    {
        return Err(Error::InvalidInput("mailbox"));
    }
    Ok(value.to_ascii_lowercase())
}

fn addresses(value: &str) -> Result<Vec<(String, Option<String>)>, Error> {
    let value = format!("{}\n", address_field(value)?);
    let parsed = MessageStream::new(value.as_bytes())
        .parse_address()
        .into_address()
        .ok_or(Error::Response("mailbox list"))?;
    parsed
        .into_list()
        .into_iter()
        .map(|addr| {
            let email = mailbox(addr.address.as_deref().ok_or(Error::Response("mailbox"))?)?;
            let name = addr.name.map(|s| s.into_owned());
            if let Some(name) = &name {
                single_line(name)?;
            }
            Ok((email, name))
        })
        .collect()
}

fn sender(value: &str) -> Result<(String, Option<String>), Error> {
    let value = format!("{}\n", address_field(value)?);
    let parsed = MessageStream::new(value.as_bytes())
        .parse_address()
        .into_address();
    if !matches!(&parsed, Some(Address::List(list)) if list.len() == 1) {
        return Err(Error::Response("single sender required"));
    }
    addresses(value.trim_end_matches('\n'))?
        .pop()
        .ok_or(Error::Response("sender"))
}

// Reject unfinished delimiters before the deliberately tolerant address parser.
// Quote/comment/escape/angle transitions adapt the same pinned address.rs state
// machine as below; no encoded display name is reparsed as a recipient list.
fn address_field(value: &str) -> Result<String, Error> {
    let value = unfolded(value)?;
    let (mut quoted, mut angle, mut escaped) = (false, false, false);
    let mut comments = 0;
    for c in value.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match c {
            '\\' if quoted || comments > 0 => escaped = true,
            '(' if !quoted => comments += 1,
            ')' if !quoted && comments > 0 => comments -= 1,
            ')' if !quoted => return Err(Error::Response("address comment")),
            _ if comments > 0 => {}
            '"' => quoted = !quoted,
            '<' if !quoted && !angle => angle = true,
            '>' if !quoted && angle => angle = false,
            '<' | '>' if !quoted => return Err(Error::Response("address brackets")),
            _ => {}
        }
    }
    if quoted || angle || escaped || comments > 0 {
        return Err(Error::Response("unfinished address field"));
    }
    Ok(value)
}

/// Nested comment and quoted-pair transitions borrowed from Stalwart's address
/// parser. Removed comments become whitespace so two tokens cannot be joined.
/// This deliberately rejects quoted/obsolete IDs outside the supported subset.
fn without_id_comments(value: &str) -> Result<String, Error> {
    let mut depth = 0;
    let mut escaped = false;
    let mut result = String::with_capacity(value.len());
    for c in value.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match c {
            '\\' if depth > 0 => escaped = true,
            '(' => {
                depth += 1;
                if depth == 1 {
                    result.push(' ');
                }
            }
            ')' if depth > 0 => depth -= 1,
            ')' => return Err(Error::Response("unbalanced message-id comment")),
            _ if depth == 0 => result.push(c),
            _ => {}
        }
    }
    if depth != 0 || escaped {
        return Err(Error::Response("unclosed message-id comment"));
    }
    Ok(result)
}

fn valid_id(value: &str) -> bool {
    value.len() <= 998
        && value
            .split_once('@')
            .is_some_and(|(left, right)| dot_atom(left) && dot_atom(right))
}

pub(super) fn message_ids(value: &str, allow_bare: bool) -> Result<Vec<String>, Error> {
    let stripped = without_id_comments(&unfolded(value)?)?;
    let input = format!("{}\n", stripped.trim());
    let ids: Vec<String> = MessageStream::new(input.as_bytes())
        .parse_id()
        .into_text_list()
        .unwrap_or_default()
        .into_iter()
        .map(|id| id.into_owned())
        .collect();
    if ids.is_empty() || ids.len() > 100 || ids.iter().any(|id| !valid_id(id)) {
        return Err(Error::Response("message-id tokens"));
    }
    // The parser tolerates broken clients. Require a lossless bracketed token
    // sequence, or one bare ID from Resend/our ticket store; never ignore junk.
    let canonical: String = ids.iter().map(|id| format!("<{id}>")).collect();
    let compact: String = stripped
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    if canonical != compact && !(allow_bare && ids.len() == 1 && stripped.trim() == ids[0]) {
        return Err(Error::Response("malformed message-id sequence"));
    }
    Ok(ids)
}

pub(super) fn wire_id(value: &str) -> Result<String, Error> {
    single_line(value)?;
    // Outbound identifiers are canonical inputs, without display text/comments.
    let bare = value
        .strip_prefix('<')
        .and_then(|s| s.strip_suffix('>'))
        .unwrap_or(value);
    if !valid_id(bare) {
        return Err(Error::InvalidInput("message-id"));
    }
    Ok(format!("<{bare}>"))
}

fn decoded_text(value: &str) -> Result<String, Error> {
    let input = format!("{}\n", unfolded(value)?);
    let text = MessageStream::new(input.as_bytes())
        .parse_unstructured()
        .into_text()
        .map(|s| s.into_owned())
        .unwrap_or_default();
    single_line(&text)?;
    Ok(text)
}

impl ReceivedEmail {
    /// None means this email is not addressed to this inbox. Message headers are
    /// untrusted threading hints, never an account/inbox selector. `received_for`
    /// is deliberately excluded from ownership: Resend derives it from Received
    /// header clauses, which can include sender-controlled forwarded history.
    pub fn into_ticket(self, inbox_email: &str) -> Result<Option<IncomingEmail>, Error> {
        let inbox_email = mailbox(inbox_email)?;
        let mut receivers = Vec::new();
        for value in self
            .envelope
            .to
            .iter()
            .chain(self.envelope.cc.iter().flatten())
            .chain(self.envelope.bcc.iter().flatten())
        {
            receivers.extend(addresses(value)?.into_iter().map(|(addr, _)| addr));
        }
        if !receivers.contains(&inbox_email) {
            return Ok(None);
        }
        let (sender_email, sender_name) = sender(&self.envelope.from)?;
        // IntroMail prioritizes the actual Message-ID header; an opaque provider
        // UUID must never substitute for a missing RFC threading identity.
        let source = self
            .headers
            .get("message-id")
            .or(self.message_id.as_deref())
            .ok_or(Error::Response("missing RFC Message-ID"))?;
        let ids = message_ids(source, true)?;
        if ids.len() != 1 {
            return Err(Error::Response("multiple Message-IDs"));
        }
        let raw_in_reply_to = self.headers.get("in-reply-to").map(str::to_owned);
        let in_reply_to = raw_in_reply_to
            .as_deref()
            .map(|s| message_ids(s, true))
            .transpose()?
            .unwrap_or_default();
        let references = self
            .headers
            .get("references")
            .map(|s| message_ids(s, false))
            .transpose()?
            .unwrap_or_default();
        let subject = decoded_text(self.envelope.subject.as_deref().unwrap_or_default())?;
        let content = self
            .text
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| self.html.as_deref().map(html_to_text).unwrap_or_default());
        if content.chars().count() > MAX_CONTENT {
            return Err(Error::Response("ticket body size"));
        }
        // Chatwoot v4.17.1 MailPresenter#auto_reply? (exact value predicates).
        let auto_reply = self
            .headers
            .get("auto-submitted")
            .is_some_and(|s| s != "no")
            || self.headers.get("x-autoreply") == Some("yes");
        Ok(Some(IncomingEmail {
            headers: ThreadHeaders {
                receivers,
                in_reply_to,
                references,
                raw_in_reply_to,
            },
            message_id: ids[0].clone(),
            sender_email,
            sender_name,
            subject,
            content,
            auto_reply,
        }))
    }
}
