//! Bounded pull-first glue over official Resend pagination and the ticket store.
use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use sea_orm::DatabaseConnection;

use super::{Cursor, Error, ListOptions, ResendClient};
use crate::db::service::ticket_service::{ingest_email, IncomingEmail};

const MAX_BATCH_BYTES: usize = 32 * 1024 * 1024;

pub struct PullOptions {
    pub page_size: u8,
    pub max_pages: u16,
}

impl Default for PullOptions {
    fn default() -> Self {
        Self {
            page_size: 100,
            max_pages: 100,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PullSummary {
    pub listed: usize,
    pub skipped_other_inbox: usize,
    pub inserted: usize,
    pub duplicates: usize,
}

impl ResendClient {
    /// Complete, bounded replay of received history. No durable cursor/scheduler
    /// is invented here. Fetch and normalize the whole selected pass before any
    /// writes, then ingest ancestors before replies. Exceeding the page/memory
    /// budget fails explicitly; it never silently truncates history/checkpoints.
    ///
    /// A database error can follow successful per-message transactions. Retry the
    /// same pass: ticket source-ID uniqueness and writer claims deduplicate it.
    /// Only one pull per inbox should be scheduled by the later coordination layer.
    pub async fn pull_received(
        &self,
        conn: &DatabaseConnection,
        options: PullOptions,
    ) -> Result<PullSummary, Error> {
        if !(1..=100).contains(&options.page_size) || !(1..=100).contains(&options.max_pages) {
            return Err(Error::InvalidInput("pull page limits"));
        }
        self.check_inbox(conn).await?;
        let mut cursor = None;
        let mut seen = HashSet::new();
        let mut cursors = HashSet::new();
        let mut ids = Vec::new();
        for page_number in 0..options.max_pages {
            let page = self
                .list_received(ListOptions {
                    limit: options.page_size,
                    cursor,
                })
                .await?;
            let next = page.data.last().map(|item| item.id.clone());
            let previous_count = seen.len();
            for item in page.data {
                if seen.insert(item.id.clone()) {
                    ids.push(item.id);
                }
            }
            if !page.has_more {
                break;
            }
            if seen.len() == previous_count {
                return Err(Error::Pagination);
            }
            let next = next.ok_or(Error::Pagination)?;
            if !cursors.insert(next.clone()) {
                return Err(Error::Pagination);
            }
            if page_number + 1 == options.max_pages {
                return Err(Error::PullLimit);
            }
            cursor = Some(Cursor::After(next));
        }
        let mut summary = PullSummary {
            listed: ids.len(),
            ..Default::default()
        };
        let mut pending = Vec::new();
        let mut bytes = 0;
        for id in ids {
            let detail = self.get_received(&id).await?;
            let created_at = received_at(&detail.envelope.created_at)?;
            if let Some(mail) = detail.into_ticket(&self.inbox_email)? {
                bytes += mail.content.len()
                    + mail.subject.len()
                    + mail.sender_email.len()
                    + mail.sender_name.as_ref().map_or(0, String::len)
                    + mail
                        .headers
                        .receivers
                        .iter()
                        .map(String::len)
                        .sum::<usize>()
                    + mail
                        .headers
                        .references
                        .iter()
                        .map(String::len)
                        .sum::<usize>()
                    + mail
                        .headers
                        .in_reply_to
                        .iter()
                        .map(String::len)
                        .sum::<usize>()
                    + mail.headers.raw_in_reply_to.as_ref().map_or(0, String::len);
                if bytes > MAX_BATCH_BYTES {
                    return Err(Error::PullLimit);
                }
                pending.push((created_at, id, mail));
            } else {
                summary.skipped_other_inbox += 1;
            }
        }
        pending.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.as_str().cmp(b.1.as_str())));
        let mut mails: Vec<_> = pending.into_iter().map(|(_, _, mail)| mail).collect();
        // A reply can have the same receipt timestamp as its parent. Honor local
        // reference dependencies as well as chronology, without changing the
        // ticket finder's upstream strategy precedence or order of header tokens.
        let mut ordered = Vec::with_capacity(mails.len());
        while !mails.is_empty() {
            let remaining: HashSet<_> = mails.iter().map(|m| m.message_id.as_str()).collect();
            let index = mails
                .iter()
                .position(|mail| {
                    !dependencies(mail).any(|id| id != mail.message_id && remaining.contains(id))
                })
                .ok_or(Error::Response("cyclic received references"))?;
            ordered.push(mails.remove(index));
        }
        self.check_inbox(conn).await?;
        for mail in ordered {
            let result = ingest_email(conn, self.scope, mail)
                .await
                .map_err(|_| Error::Persistence)?;
            if result.duplicate {
                summary.duplicates += 1;
            } else {
                summary.inserted += 1;
            }
        }
        Ok(summary)
    }
}

fn dependencies(mail: &IncomingEmail) -> impl Iterator<Item = &str> {
    mail.headers
        .in_reply_to
        .iter()
        .chain(&mail.headers.references)
        .map(String::as_str)
}

fn received_at(value: &str) -> Result<DateTime<FixedOffset>, Error> {
    // OpenAPI uses RFC3339; the official SDK's receiving fixtures also use
    // PostgreSQL timestamps with a space and an hour-only UTC offset.
    DateTime::parse_from_rfc3339(value)
        .or_else(|_| DateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S%.f%#z"))
        .map_err(|_| Error::Response("received timestamp"))
}
