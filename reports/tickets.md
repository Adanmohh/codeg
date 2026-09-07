# Step 1 — tickets (pieces 2 + 2b)

Worktree: `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`  
Branch: `feat/step1-tickets`. Sole writer; no workers started.

## Progress

Read FOUNDING.md, ORCHESTRATOR.md, STATUS.md, DECISIONS.md, AGENTS.md,
reports/step0.md and CI before implementation. Reserved the locally unused
migration name `m20260907_000002_ops_tickets` (existing names use dated modules).
Source reads complete; implementation and validation in progress.

## Grounding and decisions

- Chatwoot v4.17.1 resolves via `gh api` to immutable commit
  `b354a9550e1fb59fa537a9c384232cb076213e72`. Only MIT files outside enterprise/ read.
- code-context guide exited 0: applied “SQLite-first persistence”. No codeg-specific
  rule returned. Dependency docs exited 3: no corpus database for this worktree;
  fallback is installed SeaORM/sea-orm-migration 1.1.19 source and existing codeg
  migrations/entities/services. No corpus coverage claimed.
- Port chain order: receiver UUID → In-Reply-To → References → new conversation.
  Extend upstream References inbox isolation to ALL strategies and fallback.
  Resolve an explicit account/inbox scope before ingestion; Step 2 owns transport
  routing and MIME/header decoding. No subject-only or sender-only thread merging.
- New conversation and first message persist atomically, as the source requires.
  Reuse codeg canvas store's first-write transaction claim to avoid SQLite stale
  read snapshot promotion; source-id deduplication belongs to the same transaction.
- Keep registry edits to migration registration and one entity/service module.
  No dependency or lockfile changes intended. No UI, outbound send, or Resend.

## Source-to-port mapping (all at the immutable Chatwoot SHA above)

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
| `app/models/contact.rb` | Contact identity and normalized scoped email |
| `app/models/inbox.rb` | Inbox ownership, channel, name/email |
| `app/models/contact_inbox.rb` | Contact/inbox join used by new-conversation strategy |
| `spec/services/mailbox/conversation_finder_spec.rb` | Strategy precedence cases |
| `spec/services/mailbox/conversation_finder_strategies/receiver_uuid_strategy_spec.rb` | Receiver validity cases |
| `spec/services/mailbox/conversation_finder_strategies/in_reply_to_strategy_spec.rb` | UUID, fallback, source_id and multiple-value cases |
| `spec/services/mailbox/conversation_finder_strategies/references_strategy_spec.rb` | Wrong inbox and later valid reference cases |
| `spec/services/mailbox/conversation_finder_strategies/new_conversation_strategy_spec.rb` | Contact reuse, metadata and atomic persistence cases |
| `LICENSE` | Original copyright and MIT permission/warranty in NOTICE |

Original raw source and tree responses are local under ignored
`reports/tickets-source.log/`; immutable URLs can be reconstructed as
`https://github.com/chatwoot/chatwoot/blob/<SHA>/<exact source>`.

## Commands / exits so far

- Required document and local pinned source reads: 0.
- `gh api repos/chatwoot/chatwoot/commits/v4.17.1`, exact-SHA tree and content reads: 0.
- Installed `gh api --help`, `gh pr create --help`, cargo check/test help: 0.
- `git push -h`: 129 (help convention); stopped chained help reads, reran remaining.
- `pnpm exec --help`: 1 (tries to execute --help); pnpm auto-installed this
  worktree's frozen dependencies first successfully. Correct `pnpm help exec`: 0.
- code-context guide: 0; docs: 3 (missing index, described above).

## Delivery

Implementation SHA: pending. Draft PR URL: pending. Required gates: pending.

## Orchestrator checkpoint — stopped for docs-first hook reload

Owner requested a safe checkpoint and STOP before further product edits. This
is an incomplete implementation checkpoint, not a completion claim.

Current files:

- `src-tauri/src/db/migration/m20260907_000002_ops_tickets.rs`: one registered
  migration with five tables (four requested models plus their contact/inbox
  join), composite ownership foreign keys, assignment exclusivity, privacy and
  scoped source-id constraints, and queue/thread lookup indexes.
- `src-tauri/src/db/entities/ops_ticket.rs`: five nested SeaORM entity modules.
- `src-tauri/src/db/service/ticket_service/threading.rs`: ordered finder,
  receiver and both header patterns, source-id lookup, original-header fallback;
  account/inbox scope on every branch.
- `src-tauri/src/db/service/ticket_service/mod.rs`: shared store interface for
  inbox creation/listing, conversations/contact/messages, atomic deduplicated
  ingestion, assignment, status and private notes. Public message filtering
  follows Message.chat. No outbound transport or approval side effects.
- `NOTICE`: original Chatwoot copyright and licence text with exact source paths
  and immutable SHA; original codeg Apache LICENSE untouched.
- Shared registries changed by only four lines total.

Resumption checklist (do not treat any item as already validated):

1. Reload the configured docs-first hook as orchestrator requested. Continue as
   sole writer on this same branch/worktree; no workers, model changes or scope
   changes. Latest documentation uses `gh api`; pinned source remains v4.17.1.
2. Review and format the new Rust files. They have NOT been compiled or rustfmt'd.
   `ticket_service/mod.rs` declares `#[cfg(test)] mod tests;` but **tests.rs has
   not yet been created**, so test compilation is knowingly incomplete.
3. Add meaningful focused tests ported from the listed Chatwoot specs: competing
   strategy precedence, ordered headers, message-specific and fallback patterns,
   receiver validity, skipped foreign inbox references, all-strategy account/
   inbox isolation, no subject/sender-only merge, stored original-header fallback,
   contact reuse, duplicate ingestion, rollback of first-message failure, private
   versus public views, assignment reset, lifecycle, and on-disk close/reopen.
4. Audit the store's narrow schema transcription and record omissions clearly:
   no Rails account/user/team/channel implementations, no enterprise behavior,
   no CRM-only fields, no attachments, no mail parser, no UI. Account ids are
   explicit caller scopes; user/bot/team ids are opaque strings for future identity
   integration. Inbox auto-assignment defaults off because no scheduler is ported.
   Private notes cannot have source_id. Public outgoing rows are representable
   by the entity/schema; the approved-send integration remains Step 2.
5. Create the ignored local `out/index.html` placeholder from CI if needed.
   Build in THIS worktree's own target directory, never another worktree's output.
   Run default desktop `cargo check --locked`, server
   `cargo check --locked --no-default-features --bin codeg-server`, focused Rust
   tests, and `pnpm exec tsc --noEmit`. Record commands/exits and any failures.
6. Recheck both lockfiles remain unchanged, attribution, minimal registry edits,
   `git diff --check`; finish commits/push with lowercase prefixes. Open small
   DRAFT PR to main after required checks, using gh help already read and a body
   file. Do not merge or deploy. Add PR URL and validated implementation SHA here.

Checkpoint evidence:

- Initial docs commit `10225544554351d0bd96948d2a53cb952998e9a5` pushed to origin,
  exit 0.
- `git diff --check` at checkpoint: exit 0 (tracked edits); newly added source
  still requires formatting/review and compilation.
- No default desktop/server compile, typecheck or focused test has been run yet.
- No draft PR exists yet; owner requested checkpoint before continuing.
- Source downloads remain preserved in ignored `reports/tickets-source.log/`.
- No edits to FOUNDING.md, ORCHESTRATOR.md, STATUS.md, DECISIONS.md or lockfiles.
- Staged `git diff --cached --check`: exit 2, extra blank line at EOF in
  `src-tauri/src/db/entities/ops_ticket.rs:135`. Preserved unchanged for this
  requested stop; remove with formatting when resumed.
