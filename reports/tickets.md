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
