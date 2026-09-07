# Step 1 — tickets (pieces 2 + 2b)

Acceptance revision in progress. Draft PR: https://github.com/Adanmohh/codeg/pull/1

Initial validated implementation: `427108f87f965073d235089212cba0e34f5197f3`.

Worktree: `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`
Branch: `feat/step1-tickets`
Sole writer; no additional workers, model changes, messages to people or deployments.
Owner-authorized main/rebrand merge: `31f27050` from main `d63f98b9`.
All root planning documents are inherited unchanged from that main commit; both
NOTICE sections are preserved. Both lockfiles remain unchanged. The independent
approvals review is inherited from main and is not a separate change in this PR.

Acceptance review read completely from the approvals worker's
`reports/review-tickets.md` (also preserved in main). One P2 finding: a different
unblocked sender could reopen a thread whose primary contact is blocked. Pinned
Chatwoot `message.rb:404–410` and `ConversationMuteHelpers#muted?` confirm that
reopening must check the primary contact. All three new regressions failed against
the old guard (exit 101). The fix reads the primary contact in the existing scoped
writer transaction; message sender attribution remains separate. Incoming
`waiting_since` updates follow `Message#set_waiting_since_on_incoming_message`
independently of reopening. All 18 server-mode ticket tests, typecheck, formatting
and whitespace checks now pass. Desktop/server compile and Clippy gates are pending.

Three new regression tests cover blocked-primary resolved and snoozed threads,
plus unblocked-primary controls with both blocked and unblocked secondary senders.
They verify persisted status, snooze preservation/clearing, primary-contact
identity, separate sender attribution, incoming waiting time and stored messages.
The existing same-contact reopen and independent-connection tests remain passing.

Current raw logs: `tickets-review-regression-before.log` (exit 101, all three
regressions fail against the old guard), `tickets-review-tests-server.log`
(exit 0, 18 passed), `tickets-review-typecheck.log` (exit 0), under reports/.

Docs-first on this revision: separate React/Cargo manifest reads exited 0. Audit
lines 545/546 and 547/548 contain live PreToolUse/PostToolUse pairs for session
`01a07c1c-d82f-7022-84db-778a438632f1` and this tickets worktree. The offline
code-context guide exited 0 with generic SQLite-first guidance; task-specific
corpus coverage is still missing. Read installed SeaORM 1.1.19 select/entity/update
implementations and the existing scoped contact lookup before the edit. Read the
cached immutable Chatwoot message source first, then the newly needed mute concern
through `gh api` at the same required commit. No hook disabled or bypassed.

## Outcome

Ported the Chatwoot default finder chain into Rust: receiver UUID → In-Reply-To
→ References → new conversation. Every lookup is scoped to account and inbox.
One SeaORM migration registers five related ticket tables, including the
contact/inbox join, with assignment and private/public message distinctions.
The shared store supports atomic ingestion, replay deduplication, contact reuse,
assignment/status, notes, public-reply receipts and scoped reads.

## Initial validation

Rust commands run from this worktree's `src-tauri/`; frontend commands from the
worktree root. All build outputs belong to this worktree. No lockfile changes.

| Command | Exit / result | Local log under reports/ |
| --- | --- | --- |
| `cargo check --locked` | 0; default desktop features | tickets-desktop-final.log |
| `cargo check --locked --no-default-features --bin codeg-server` | 0 | tickets-server-check.log |
| `cargo test --locked --no-default-features --lib db::service::ticket_service --target-dir /Users/mohamedadan/projects/_worktrees/ops-desk/tickets/src-tauri/target` | 0; 15 passed, 0 failed | tickets-tests-final.log |
| `cargo clippy --locked --no-default-features --bin codeg-server --lib -- -D warnings` | 0 | tickets-clippy.log |
| `pnpm exec tsc --noEmit` | 0 | tickets-typecheck.log |
| `rustfmt --edition 2021 --check` on all five new Rust implementation/test files | 0 | tool output |
| `git diff --check` | 0 | tool output |
| Lockfile comparison to upstream `v0.30.4` and protected-doc comparison to checkpoint | 0, no diff | tool output |
| Verify downloaded source blobs against immutable Git tree | 0, all 17 match | mapping below |

The 15 focused tests cover competing strategy precedence; receiver validity and
first syntactic match; UUID/fallback/source-id precedence and ordered values;
all-strategy account/inbox isolation and skipped foreign references; contact reuse
without sender/subject merges; exact scoped original-header fallback; rollback of
contact/join/conversation when the first message fails; idempotent replay;
assignment/private notes/lifecycle across close/reopen; database ownership and
privacy constraints; migration down/up; malformed identities; public-reply receipt
replay/conflict and subsequent threading; two independent SQLite connections racing
on replay and shared-parent ingestion; and atomic DDL rollback on migration failure.

No frontend source changed, so no new frontend tests were added. Desktop runtime,
real email, attachment or end-to-end process testing remains outside this store task.
Known upstream warnings: zero-byte codeg-mcp compile sidecar; future compatibility
notice for proc-macro-error2 2.0.1. The ignored out/index.html placeholder follows
CI and reports/step0.md; no real sidecar, package or deployment was produced.
Raw logs remain local because upstream ignores *.log.

## Docs-first and hook evidence

Read FOUNDING.md, ORCHESTRATOR.md, STATUS.md, DECISIONS.md, AGENTS.md completely
before implementation, plus reports/step0.md and CI. No nested AGENTS.md exists.
Applied code-context using the existing rag-skills .venv and HF_HUB_OFFLINE=1:
guide exited 0, applying “SQLite-first persistence”; no codeg-specific rule was
returned. Dependency docs exited 3 because no tickets corpus database exists.
Used local installed source rather than claiming missing corpus coverage.

Pinned dependencies read: SeaORM and sea-orm-migration 1.1.19 (entities,
ConnectionTrait, Statement, transactions, ConnectOptions, migration runner and
SchemaManager); regex 1.12.3; uuid 1.20.0; Tokio 1.49.0 join docs; chrono 0.4.43;
serde_json 1.0.149; tempfile 3.24.0. Frontend manifest/help confirmed React 19.2.4,
TypeScript 5.8.3 and pnpm 11.9.0. Installed gh api/pr create/view/edit help,
cargo check/test/clippy help and rustfmt help were read before reliance.

The owner requested a checkpoint for Codex hook reload. Resumed first with two
SEPARATE read commands: `cat node_modules/react/package.json`, then
`cat src-tauri/Cargo.toml`, both exit 0. Verified live audit records in
`/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`:

| Lines | Unix time | Session | Events / result |
| --- | --- | --- | --- |
| 12, 13 | 1788789299 | `01a07c1c-d82f-7022-84db-778a438632f1` | PreToolUse / PostToolUse; Bash; exit 0 |
| 14, 15 | 1788789303 | `01a07c1c-d82f-7022-84db-778a438632f1` | PreToolUse / PostToolUse; Bash; exit 0 |

All four have cwd `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`.
The live docs-first reminder also appeared during subsequent source/patch calls.
No hook disabled or bypassed. All remote source/documentation research used gh api
at the required immutable commit; no latest version replaced the mandated pin.

## Source-to-port mapping

Chatwoot **v4.17.1**, resolved via `gh api repos/chatwoot/chatwoot/commits/v4.17.1`
to **b354a9550e1fb59fa537a9c384232cb076213e72**. Only MIT sources outside enterprise/
were read/ported. Original copyright, complete permission/warranty text and exact
file names are reproduced in NOTICE. No AGPL, restricted enterprise or Kun code.

| Exact source | Target / behavior |
| --- | --- |
| `app/services/mailbox/conversation_finder.rb` | Rust ordered short-circuit finder |
| `app/services/mailbox/conversation_finder_strategies/base_strategy.rb` | Shared normalized mail context |
| `app/services/mailbox/conversation_finder_strategies/receiver_uuid_strategy.rb` | Anchored reply+UUID local-part matcher |
| `app/services/mailbox/conversation_finder_strategies/in_reply_to_strategy.rb` | Message/fallback UUID patterns, then source_id |
| `app/services/mailbox/conversation_finder_strategies/references_strategy.rb` | Ordered reference matching with inbox scope |
| `app/services/mailbox/conversation_finder_strategies/new_conversation_strategy.rb` | Stored In-Reply-To fallback; contact reuse; atomic creation |
| `app/models/conversation.rb` | Conversation, assignment, lifecycle fields |
| `app/models/message.rb` | Message direction, privacy, source_id and public-chat filtering |
| `app/models/concerns/conversation_mute_helpers.rb` (`muted?` only) | Primary-contact blocked guard for reopening; read/ported during acceptance fix |
| `app/models/contact.rb` | Contact identity and normalized scoped email |
| `app/models/inbox.rb` | Inbox ownership, channel, name/email |
| `app/models/contact_inbox.rb` | Contact/inbox join used by new-conversation strategy |
| `spec/services/mailbox/conversation_finder_spec.rb` | Strategy precedence cases |
| `spec/services/mailbox/conversation_finder_strategies/receiver_uuid_strategy_spec.rb` | Receiver validity cases |
| `spec/services/mailbox/conversation_finder_strategies/in_reply_to_strategy_spec.rb` | UUID, fallback, source_id and multiple-value cases |
| `spec/services/mailbox/conversation_finder_strategies/references_strategy_spec.rb` | Wrong inbox and later valid reference cases |
| `spec/services/mailbox/conversation_finder_strategies/new_conversation_strategy_spec.rb` | Contact reuse, metadata and atomic persistence cases |
| `LICENSE` | Original copyright and MIT permission/warranty in NOTICE |


All finder/strategy files map to
`src-tauri/src/db/service/ticket_service/threading.rs`; new-conversation
persistence/model behavior maps to `src-tauri/src/db/service/ticket_service/mod.rs`.
The five model shapes map to `src-tauri/src/db/entities/ops_ticket.rs` and
`src-tauri/src/db/migration/m20260907_000002_ops_tickets.rs`; upstream spec cases
map to `src-tauri/src/db/service/ticket_service/tests.rs`.

Local Apache glue references: xintaofei/codeg v0.30.4 at
`6f6bd648b206412644842a98d9ffeebf57292bed`, specifically
`src-tauri/src/db/service/canvas_service.rs` (writer claim/transaction pattern),
`src-tauri/src/db/entities/canvas_node.rs` (entity conventions),
`src-tauri/src/db/test_helpers.rs` (full migrated test DB), and existing migration
registration in `src-tauri/src/db/migration/mod.rs`. Apache LICENSE is unchanged.

Additionally inspected `app/mailboxes/mailbox_sanitizer.rb` at the same Chatwoot
SHA to resolve the base strategy dependency; no sanitizer source ported. The
adapter must provide decoded UTF-8 input. Malformed source identities are rejected
instead of silently rewritten; header parsing remains Step 2.

### Immutable source integrity

Raw responses are preserved locally in ignored reports/tickets-source.log/.
All 18 following downloaded blobs match the immutable commit's source metadata.
The original 17 were checked against the full Git tree; the additional concern's
Git blob hash matches its exact-commit contents response from `gh api`.

| Source file (immutable commit linked) | Git blob SHA |
| --- | --- |
| [LICENSE](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/LICENSE) | `d0425a707ab6f0cf06c9aa3c08e8edf5c64adc61` |
| [app/models/contact.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/models/contact.rb) | `a63b164bb2669adc9cbbe10a4581cd86ae497267` |
| [app/models/contact_inbox.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/models/contact_inbox.rb) | `893ab87f3c534e9eb78268a7c6577bff959de3e4` |
| [app/models/conversation.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/models/conversation.rb) | `5d9394afe241b5612c9010c21d45a5e77b991836` |
| [app/models/inbox.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/models/inbox.rb) | `cb9870538305a127331875ad9bc675b2e18107cc` |
| [app/models/message.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/models/message.rb) | `913fb5a6469b43bd347ed6ee8992f56deed81433` |
| [app/models/concerns/conversation_mute_helpers.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/models/concerns/conversation_mute_helpers.rb) | `ebc0542d23aa5e8b361ef07e2c0ca1e156e61d69` |
| [app/services/mailbox/conversation_finder.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/services/mailbox/conversation_finder.rb) | `2f8128ecdb333104dc3940ea555b39dfe26585d1` |
| [app/services/mailbox/conversation_finder_strategies/base_strategy.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/services/mailbox/conversation_finder_strategies/base_strategy.rb) | `fd61fd9e090227d1b7d6d653cf96fa41be616b0e` |
| [app/services/mailbox/conversation_finder_strategies/in_reply_to_strategy.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/services/mailbox/conversation_finder_strategies/in_reply_to_strategy.rb) | `e3d8270f1c6265e0958336007f246b3a78804418` |
| [app/services/mailbox/conversation_finder_strategies/new_conversation_strategy.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/services/mailbox/conversation_finder_strategies/new_conversation_strategy.rb) | `f9f5c3cf87c93bd6a1cbcc7487ad7eb503e023f5` |
| [app/services/mailbox/conversation_finder_strategies/receiver_uuid_strategy.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/services/mailbox/conversation_finder_strategies/receiver_uuid_strategy.rb) | `39b155cf8834f0eeaefb7355dd797b2731c1fe04` |
| [app/services/mailbox/conversation_finder_strategies/references_strategy.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/app/services/mailbox/conversation_finder_strategies/references_strategy.rb) | `4420c5c7d31971f9a7379465818e3adda80cfb06` |
| [spec/services/mailbox/conversation_finder_spec.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/spec/services/mailbox/conversation_finder_spec.rb) | `313ca409a4dddb1305a5a5f8da5bfbac1578ccd6` |
| [spec/services/mailbox/conversation_finder_strategies/in_reply_to_strategy_spec.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/spec/services/mailbox/conversation_finder_strategies/in_reply_to_strategy_spec.rb) | `3fca8c3c916a225a17d746be5b8a0cbfeffed5d3` |
| [spec/services/mailbox/conversation_finder_strategies/new_conversation_strategy_spec.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/spec/services/mailbox/conversation_finder_strategies/new_conversation_strategy_spec.rb) | `8609d0133b0f8a4e2d6a35278832e093eb7f77b0` |
| [spec/services/mailbox/conversation_finder_strategies/receiver_uuid_strategy_spec.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/spec/services/mailbox/conversation_finder_strategies/receiver_uuid_strategy_spec.rb) | `d45f6f4febfd98e7d996606d5cf183b151782b47` |
| [spec/services/mailbox/conversation_finder_strategies/references_strategy_spec.rb](https://github.com/chatwoot/chatwoot/blob/b354a9550e1fb59fa537a9c384232cb076213e72/spec/services/mailbox/conversation_finder_strategies/references_strategy_spec.rb) | `6ef61a58f5c10d8f42da39970a16eba29755fc78` |


## API, decisions and limitations

Use `db::service::ticket_service` with the existing `AppDatabase.conn` in either
runtime. The normal database initializer registers the migration automatically.
The module exposes inbox create/list, conversation list/get, contact lookup,
internal/public message views, `ingest_email`, assignment, status,
`add_private_note`, and `record_public_reply`. The latter persists a confirmed
external human reply; it has no network operation and cannot approve a send.
Replay with changed content/author/conversation returns a conflict.

The schema transcribes the email-ticket subset of the MIT model annotations:

- Inbox: account ownership, name, email, channel type, auto-assignment setting,
  timestamps. Auto-assignment is off; no assignment scheduler was ported.
- Contact: name/email/phone/identifier, account, visitor/lead/customer integer
  vocabulary, blocked flag, custom/additional attributes and timestamps. This
  email-only store requires an email; other source contact kinds remain future work.
- ContactInbox: account-scoped contact/inbox/source-id association and timestamps.
- Conversation: UUID, account/inbox/contact/join, status (open/resolved/pending/
  snoozed), priority, human/bot/team assignment, JSON attributes, activity/waiting/
  first-reply/snooze timestamps. Human assignment resets bot ownership.
- Message: account/inbox/conversation, source-id, incoming/outgoing/activity/template
  direction, separate private flag, content/type, sent/delivered/read/failed
  vocabulary, sender type/id, content/additional attributes and timestamps.

Rails-only identity/channel tables, per-account display-id sequences, campaigns,
SLA/CSAT, labels, presence/unread counters, CRM profile fields and attachment/search
infrastructure were not copied. The local ticket id is its display identifier.
Account ids are logical caller scopes; assignment ids are opaque strings, because
codeg has no equivalent Chatwoot User/AgentBot/Team tables. This module is not an
account authorization boundary. The Step 2 API must select an authorized scope.

The fixed default strategy chain preserves upstream order, first syntactic receiver
behavior, UUID-before-source-id matching, ordered In-Reply-To/References values,
and exact original-header fallback. Unlike global upstream receiver/In-Reply-To
queries, every lookup is constrained to account AND inbox. There is no injected
custom-strategy API. Message ids retain case and trim only framing brackets/outer
whitespace; UUID lookup is case insensitive. No subject/contact-only merging.

No MIME/RFC header parser, Resend integration, outbound send, approval UI, email
panel or deployment is included. Input contains decoded envelope addresses and
ordered parsed header ids; Step 2 supplies that adapter, sender verification,
HTML sanitation/rendering and approval enforcement. The module does not interpret
an email body as instructions. The pre-existing zero-byte MCP sidecar and ignored
`out/index.html` are compile placeholders from CI, not a packaged application.


Additional implementation decisions:

- SQLite mutations claim the writer before their first SELECT (the existing canvas
  store pattern), avoiding stale read-snapshot promotion and cross-process races.
  Source-id deduplication and contact/conversation/message writes commit together.
- SeaORM 1.1.19 does not wrap SQLite migrations; explicitly wrap five-table up/down
  in one transaction. The inherited migrator still owns its migration-history row.
- Composite foreign keys enforce ownership, and the database forbids assigning
  both human and bot, or giving a private note an external source-id.
- Source-id uniqueness is per inbox, allowing the same received email in separate
  inboxes. Public receipt replay must also match conversation, author and content.
- Shared registry changes total four inserted lines relative to baseline. Reserved
  migration name is `m20260907_000002_ops_tickets`; keep it alongside the approvals
  migration when orchestrator integrates. The owner-authorized main/rebrand merge
  preserves both NOTICE sections. Approvals branch integration remains separate.

## Checkpoint history and corrected command failures

- `10225544554351d0bd96948d2a53cb952998e9a5`: initial sources/report, pushed exit 0.
- `cc0d862ac4afe6c537e3fc2b595983b1b690db58`: requested incomplete implementation
  checkpoint, pushed exit 0; `69f7336b` report follow-up also pushed exit 0. Stopped
  as requested; resumed only on owner instruction. Tests/formatting were then pending.
- Checkpoint staged whitespace check exited 2 for an extra blank line at EOF;
  fixed by rustfmt on resumption. Final check passes.
- `git push -h` exited 129 (help convention). `pnpm exec --help` exited 1 because
  it attempted to execute --help; dependency auto-install itself succeeded without
  changing the lockfile. Correct `pnpm help exec` exited 0.
- Initial dependency-corpus docs command exited 3 (missing index, above).
- A source probe for a nonexistent tokio-1.50.0 path exited 2; discovered actual
  files and read lockfile-pinned Tokio 1.49.0 before using its API.
- One test invocation at repo root exited 101 (no Cargo.toml there); corrected cwd
  to src-tauri and final suite passed. Early reads of not-yet-created gate logs
  returned 1; final logs exist. No product workaround or hook bypass was used.

## Initial delivery

Validated implementation SHA: `427108f87f965073d235089212cba0e34f5197f3`.
Commit `feat: complete scoped ticket persistence and threading tests` and push to
`origin/feat/step1-tickets` exited 0.

Draft PR: https://github.com/Adanmohh/codeg/pull/1
`gh pr create --repo Adanmohh/codeg --base main --head feat/step1-tickets --draft
--title 'feat: add scoped ticket store and email threading'
--body-file reports/tickets-pr-body.log` exited 0. Verified via `gh pr view`:
OPEN, isDraft=true, base main, head feat/step1-tickets, head SHA matches the
validated implementation. No merge or deployment. CI completion has not been
awaited; all requested local gates passed.

This final report-only follow-up records the published PR and validated code SHA;
its commit comes after the implementation SHA without changing product/test files.
A report-only trailing-space finding during final staging was corrected before
commit; the staged whitespace check then exited 0.
