Independent ticket acceptance review, 2026-09-07.

Acceptance: **changes requested for one P2 defect** below. No P0/P1 findings.
The finding is established by tracing the Rust implementation against the pinned
upstream behavior; its proposed Rust regression was not executed in this review.

Reviewed immutable commit: `c2dc3be58b0e98999360fa23565aac8a3fe25fb6`.
Baseline: `2f74992e6b133a3ae3ffe3c1e4d0c14444b29137`.
Read the complete committed `reports/tickets.md`, the entire diff, all five new
Rust files including all 15 tests, the three registry changes, and NOTICE.
Independently confirmed that the difference from the author's validated product
commit `427108f87f965073d235089212cba0e34f5197f3` is only the ticket report.
The later rebrand merge to main did not change this review target.

**P2 — Check the conversation's blocked contact before reopening it.**

Location: [src-tauri/src/db/service/ticket_service/mod.rs:362](https://github.com/Adanmohh/codeg/blob/c2dc3be58b0e98999360fa23565aac8a3fe25fb6/src-tauri/src/db/service/ticket_service/mod.rs#L362),
specifically the reopening branch at lines 357–367. `sender` was selected using
the incoming message's email at lines 248–275; it need not be the conversation's
primary contact. The header strategies intentionally allow another participant
to reply to an existing thread. Consequently, `if !sender.blocked` permits an
unblocked participant to reopen a resolved or snoozed conversation whose primary
contact remains blocked. This breaks the borrowed conversation mute behavior.

Reproducible trigger, using the existing test helpers and an ordinary database
fixture for the already-supported `blocked` field:

1. Ingest an initial message, creating a conversation for `tester@example.com`.
2. Set that contact's persisted `blocked` field to `true`, and resolve the
   conversation through `set_status`.
3. Ingest a new message from `cc@example.com` in the same account/inbox, with
   In-Reply-To pointing to the first message.
4. The finder selects the original conversation, but ingestion creates an
   unblocked sender for `cc@example.com` and changes the conversation to Open.
   The primary contact is still blocked. Expected: it stays Resolved.

Suggested regression for the existing ticket test module; **not run here**:

```rust
#[tokio::test]
async fn another_participant_cannot_reopen_a_muted_conversation() {
    let db = fresh_in_memory_db().await;
    let s = scope(&db.conn, 1, "support@example.com").await;
    let first = seed(&db.conn, s, "original@example.com").await;
    let mut primary = get_contact(&db.conn, s, first.conversation.id)
        .await.unwrap().into_active_model();
    primary.blocked = Set(true);
    primary.update(&db.conn).await.unwrap();
    set_status(&db.conn, s, first.conversation.id, ConversationStatus::Resolved)
        .await.unwrap();
    let mut reply = mail("reply@example.com");
    reply.sender_email = "cc@example.com".into();
    reply.headers.in_reply_to = vec!["original@example.com".into()];
    let result = ingest_email(&db.conn, s, reply).await.unwrap();
    assert_eq!(result.conversation.id, first.conversation.id);
    assert_eq!(result.conversation.status, 1);
}
```

Upstream evidence at Chatwoot
`b354a9550e1fb59fa537a9c384232cb076213e72`:
[Message#reopen_conversation, lines 404–410](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/models/message.rb#L404)
returns when `conversation.muted?`;
[ConversationMuteHelpers#muted?, lines 19–21](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/models/concerns/conversation_mute_helpers.rb#L19)
uses the conversation's contact. This concern is included by the borrowed
Conversation model at line 65. Neither another participant nor sender verification
changes which contact owns that mute state. The current 15 tests never set or
assert `blocked`, so their successful run does not cover this case.

Required fix: load/check the conversation's primary contact in the existing
scoped writer transaction and use its blocked state to guard reopening. Keep
sender attribution separate from conversation mute policy. Add the regression
above, its snoozed equivalent, and an unblocked-primary control that still reopens.
This requires no transport, UI, or new blocking API.

**Source and licence verification.**

All remote research used `gh api`; no latest source replaced a founding pin.
Independently resolved `chatwoot/chatwoot` tag `v4.17.1` through
`gh api repos/chatwoot/chatwoot/commits/v4.17.1 --jq .sha`: exit 0, yielding
`b354a9550e1fb59fa537a9c384232cb076213e72`.
Read the immutable Git tree (not truncated) and fetched all 17 files below at
that commit. For each raw response, `git hash-object --stdin` produced the same
blob SHA as both the remote tree and the committed report's integrity table:
**17/17 matched**, all hash pipelines exited 0. No objects were written.

| Exact borrowed paths | Independently inspected port correspondence |
| --- | --- |
| `app/services/mailbox/conversation_finder.rb` and `app/services/mailbox/conversation_finder_strategies/{base_strategy,receiver_uuid_strategy,in_reply_to_strategy,references_strategy,new_conversation_strategy}.rb` | `ticket_service/threading.rs`: ordered short-circuit strategies, first syntactic receiver, UUID/source-id matching, original-header fallback; persistence in `ticket_service/mod.rs` |
| `app/models/{conversation,message,contact,inbox,contact_inbox}.rb` | `entities/ops_ticket.rs`, migration `m20260907_000002_ops_tickets.rs`, and store lifecycle/privacy behavior; email-ticket subset and documented omissions checked |
| `spec/services/mailbox/conversation_finder_spec.rb` and `spec/services/mailbox/conversation_finder_strategies/{receiver_uuid_strategy,in_reply_to_strategy,references_strategy,new_conversation_strategy}_spec.rb` | `ticket_service/tests.rs`: source strategy cases plus stronger isolation, rollback and replay cases |
| `LICENSE` | NOTICE preserves the complete original text byte-for-byte as its suffix, including copyright, permission, warranty and enterprise exclusion |

The exact per-file hashes are independently confirmed against the table in the
[immutable ticket report](https://github.com/Adanmohh/codeg/blob/c2dc3be58b0e98999360fa23565aac8a3fe25fb6/reports/tickets.md).
NOTICE names all 16 source/spec files and the exact source commit. All listed
files are outside `enterprise/` and covered by the pinned repository's
[MIT Expat licence](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/LICENSE).
No restricted enterprise implementation or other prohibited source was found in
the diff. Ruby extension declarations were not treated as licences to copy their
enterprise implementations.

Also read `app/mailboxes/{mailbox_sanitizer,reply_mailbox,incoming_email_validity_helper}.rb`
and `app/models/concerns/conversation_mute_helpers.rb` at that same commit to
check inherited behavior. These were review-only dependency reads, not new ports.
Codeg `v0.30.4` independently resolved through `gh api` to
`6f6bd648b206412644842a98d9ffeebf57292bed` (exit 0). Its local immutable
`canvas_service.rs` documents the write-before-read transaction pattern used here.
The existing Apache LICENSE is unchanged; NOTICE was absent at the baseline, so
this branch did not overwrite an existing attribution entry.

**Concurrency, ownership, migration and task integration.**

The mutations claim SQLite's writer before their first SELECT. Ingestion's
deduplication, contact/join/conversation/message creation and lifecycle update
share one transaction. Receipt replay checks conversation, author, direction,
privacy and exact content. The unique inbox/source-id constraint backs replay
serialization. The two-connection test uses separate database connections and
tests duplicate delivery and simultaneous replies sharing the original-header
fallback; it is more meaningful than an in-process mutex-only test. No additional
concurrency defect was identified in the reviewed paths. Assignment and explicit
status edits are serialized last-write-wins operations, not approval decisions.

All finder paths and conversation/message reads constrain account and inbox.
Public message reads exclude both private notes and activity. Scoped foreign keys
cover contact/inbox/conversation/message ownership; notes cannot obtain an external
source-id through the API, and the database rejects a private message carrying one.
Logical account scopes and opaque author/assignee strings are explicitly trusted
internal inputs. This store does not provide caller authentication or recipient
authorization, and it is not exposed as a new HTTP or desktop command in this diff.

The five-table up/down DDL is explicitly transactional and uses distinct
`ops_ticket_*` names. The reserved migration is registered once. The migration
history row is still inserted afterward by the inherited SeaORM migrator: the
verified atomicity is the ticket DDL, not crash atomicity with migration history.
The author's report already discloses this framework limitation.

The diff changes no `work_task` entity, transition, event, or CAS predicate, and
the ticket store never writes those tables. Ticket resolution therefore does not
mark a work task Done or bypass run_seq/review guards. The four registry additions
are the only shared Rust changes. Combined integration with the approvals branch
and preservation of both NOTICE sections remain the orchestrator's work; no main
changes were pulled or integrated during this review.

**Validation evidence and limits.**

| Evidence | Result and attribution |
| --- | --- |
| `git diff --check 2f74992e6b133a3ae3ffe3c1e4d0c14444b29137 c2dc3be58b0e98999360fa23565aac8a3fe25fb6` | Independently run, exit 0 |
| In-memory DDL checks using `HF_HUB_OFFLINE=1 /Users/mohamedadan/projects/rag-skills/.venv/bin/python -B -` | Independently run, exit 0; nine checks passed on Python's SQLite 3.53.1; no database files written |
| Raw source blob hashes, immutable tag resolution, NOTICE text comparison | Independently run; all 17 hashes, both tag SHAs and complete licence comparison matched |
| Focused Rust ticket suite, 15 tests | Orchestrator explicitly reported an independent successful rerun at the reviewed head during this review; not rerun by this reviewer |
| Desktop/server cargo check, server clippy, TypeScript and formatting gates | Recorded successful by the author in the immutable report; not independently rerun by this reviewer |

The nine DDL checks extracted the unchanged raw SQL from the reviewed migration
using `git show` and executed it in `:memory:` with foreign keys enabled. They
checked all five tables, rejected cross-account joins, rejected a mismatched
conversation/inbox link, rejected simultaneous human/bot assignment, rejected
cross-inbox messages, rejected private messages with source IDs, rejected duplicate
source IDs within an inbox, allowed the same ID in a different inbox with a clean
foreign-key check, and verified rollback after a deliberately preexisting final
message table caused late DDL failure. This validates SQL constraints on that
SQLite engine; it is not a Rust/SeaORM execution or a rerun of the author's tests.

Applied the code-context skill and used installed pinned sources before library
research: SeaORM/sea-orm-migration 1.1.19 and SQLx SQLite 0.8.6 transaction,
migration and connection defaults. The local code-context corpus has no
ops-desk-specific dependency coverage; no corpus result is claimed for this port.
Installed Python/sqlite3 and Git help were read before the extra checks. Live hook
evidence in `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl` includes
PostToolUse lines 499–500 at Unix time 1788809481 and PreToolUse lines 509–512 at
1788809521, all for session `01a07c1c-d3a3-7c22-a5b6-cedce2970d8d` and this
approvals worktree. Hooks remained enabled.

No Cargo build or Rust regression was run because this review authorizes writes
only to this report. No transport, browser, HTML rendering, external receipt
ordering, runtime UI or combined-branch test was performed. The later direct
Resend REST and GitHub App integrations, including their approval/authentication
boundaries, remain outside this Step 1 review; no MCP transport is proposed.
Only this report was written. No product edits, other-worktree writes, additional
agents, commits, pushes, merges, deployments or messages to people were made.
