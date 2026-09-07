# Step 2 — direct Resend transport (piece 2c)

In progress. Sole writer in `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`,
branch `feat/step2-email-transport`, created directly from accepted main `5bd0b1e1`.
No UI/routes, approval/work-task changes, other-worktree writes, additional agents,
live sends or deployment. Draft PR and implementation validation are pending.

Read current FOUNDING, ORCHESTRATOR, STATUS, DECISIONS and AGENTS completely.
Applied code-context using the existing rag-skills Python and HF_HUB_OFFLINE=1.
Guide exited 0: reuse/port sibling code and feature-branch isolation apply; the
HTML-rendering rule is reserved for later UI work. Owner's no-agent/no-merge and
gh-api-only instructions take precedence over generic retrieved workflow rules.
Dependency docs exited 3: `rag-skills/data/code/tickets.db` is absent. Installed
source is the API authority; no missing corpus coverage is claimed.

Separate React/Cargo manifest reads exited 0. Live hook audit pairs at lines
930/931 and 932/933 are PreToolUse/PostToolUse for this worktree and session
`01a07c1c-d82f-7022-84db-778a438632f1`. Hooks remain enabled.

Source contracts resolved through `gh api`:

- IntroInnovation/intromail: required commit
  `0bd24dfe284b888aa9f602fa1fd00e337ea38874`; read `backend/app/services/resend_client.py`,
  adjacent ingestion/mailbox ownership code and send authorization tests; router
  and further relevant tests are under review. Owner-authorized private source.
- Official resend/resend-node release `v6.26.0`:
  `c61cccae2999d50d2aca9ce5fd1064f3bb855219`, MIT. Receiving get/list methods,
  response interfaces, pagination helpers and email send method read.
- Official resend/resend-openapi:
  `8dca27c284bd377e48a6db131b52f6689d53195e`, MIT. Read `/emails` POST,
  `/emails/receiving` GET and `/emails/receiving/{email_id}` GET plus referenced
  request/response schemas from `resend.json` (spec version 1.5.1).

Verified HTTP shape: bearer auth, 30-second timeout pattern, POST /emails with
Idempotency-Key and explicit Message-ID/In-Reply-To/References; received listing
uses exclusive before/after UUID cursors, limit 1–100 (default 20), and
`{object: "list", has_more, data}`. Detail is a direct email object; the SDK
supports `html_format=cid`. IntroMail lacks a list/poll helper, so that portion
uses the official SDK/OpenAPI contract without changing the IntroMail pin.

Current tickets require decoded envelope addresses and RFC message-ID tokens
(folding/comments parsed), scope every lookup to account/inbox, and deduplicate
ingestion by RFC source ID within an inbox. Their inputs are trusted internal
scope selections, not authentication credentials. No existing RFC mail parser
dependency was found. Evaluating a minimal pinned parser dependency instead of
writing one from scratch; additions and licenses will be recorded before delivery.

Installed versions read: reqwest 0.12.28 (the direct dependency), http 1.4.0,
Tokio 1.49.0, Axum 0.8.8, axum-test 17.3.0, SeaORM 1.1.19, serde 1.0.228,
serde_json 1.0.149, sha2 0.10.9, URL 2.5.8. Current lockfiles are unchanged.
Research responses and verified blob metadata stay in ignored
`reports/email-transport-source.log/`. No credential values are read or logged.

## Internal contract for coordinated UI/API work

Module `codeg_lib::email_transport` (no route/command registration):

- `ResendClient::new(&DatabaseConnection, ticket_service::Scope, api_key: &str)`:
  trusted caller supplies the selected inbox's existing-store credential; verifies
  account/inbox membership of the record and binds From to its email address.
  Scope remains a routing selection, not user authentication or send approval.
- `list_received(ListOptions { limit, cursor: Option<Cursor::After/Before(EmailId)> })`
  and `get_received(&EmailId)` mirror the provider API. These raw responses are
  credential-wide; never expose them as authorized inbox/user views.
- `ReceivedEmail::into_ticket(inbox_email)` returns `Option<IncomingEmail>`;
  only exact decoded To/Cc/Bcc inbox recipients pass. `received_for` is excluded
  from authorization because it comes from Received header clauses. Alias-only
  delivery needs explicit future inbox alias configuration.
- `pull_received(&DatabaseConnection, PullOptions { page_size, max_pages })`
  performs a bounded full-history pass, normalizes before writes, orders ancestors
  before replies, and uses existing per-inbox RFC source-ID deduplication. Returns
  counts in `PullSummary`. No scheduler or durable checkpoint is introduced.
- `send(&DatabaseConnection, &SendEmail)` is a trusted low-level operation for a
  later approval dispatcher ONLY. No public route or agent command may expose it.
  Inputs have plain addr-spec recipient arrays, subject/text/html, Message-ID,
  optional In-Reply-To, ordered References, and a mandatory persisted idempotency
  key. There is no caller-selected From or arbitrary custom-header map. Retrying
  requires the same key and payload, including Message-ID. No live send is run.

No migration is required; reserved `m20260907_000004_ops_email` remains unused.
The caller must serialize scheduled pulls for an inbox. Database failures may
follow successful per-message transactions; replay the full pass to deduplicate.
The pass fails before writes on incomplete pagination, malformed normalization,
reference cycles, >100 pages of <=100 items, or >32 MiB normalized data. Raw
response limit is 8 MiB, request timeout 30 seconds, redirects and automatic
retries disabled. Attachment/raw download, scheduler/cursor storage, user auth,
approval dispatch and routes remain later integration work.

The sole new direct dependency is `mail-parser = "=0.11.1"` (MIT selected),
reviewed official tag/installed crate SHA
`6c5d33028530e0ec957f3440ec2ffe65722ecc8c`. It adds only `hashify 0.2.9` transitively;
no existing package version changes. Both are MIT/Apache alternatives, MIT selected.
The library supplies RFC address/encoded-text parsing and HTML-to-text conversion;
strict ID validation and its nested-comment transitions adapt the permissive parser
to the existing ticket contract. Unsupported: quoted/obsolete Message-IDs and
quoted-local-part/domain-literal/SMTPUTF8 mailbox addresses. HTML-only messages
become plain text; this is not an HTML sanitization/rendering API.

Implementation is now present with 15 passing local HTTP/normalization tests.
The final desktop/server checks, desktop tests and Clippy are in progress.
Initial server check passed. Typecheck passed after the research-cache correction
described below. Source attribution and full MIT license texts are in NOTICE.

## Source-to-port mapping

All remote source reads used `gh api repos/OWNER/REPO/contents/PATH?ref=SHA`;
content was base64-decoded and its Git blob SHA verified locally. No web, MCP or
Resend CLI research/transport was used. Installed sources were read before API use.

| Immutable source | Exact files / reviewed portions | Port or verification |
| --- | --- | --- |
| IntroInnovation/intromail `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | `backend/app/services/resend_client.py` (whole) | `client.rs`: shared bearer client, 30s timeout, POST /emails body and threading headers, Idempotency-Key, GET received detail. This source has no list helper. |
| Same IntroMail pin | `backend/app/services/ingest.py` header normalization/Message-ID priority/RFC dedup/envelope fields; `backend/app/services/mailboxes.py` (whole); `backend/app/routers/mail.py` SendIn and `_resolve_sender_mailbox`/send_mail (lines 1080–1422) | `normalize.rs`: object/array headers, decoded sender/recipients, real RFC header before provider fallback, text-first body; `client.rs`: inbox-bound From. Existing ticket store owns threading and atomic dedup; IntroMail's task routing/webhook/auth models are not ported. |
| Same IntroMail pin | `backend/tests/test_send_authorization.py` (whole); `backend/tests/test_mail_privacy.py` visibility/multi-recipient/hidden-thread/taint cases; `backend/tests/test_task_reply_email.py` `_ingest`, on-wire header and ordinary reply cases | `tests.rs`: wrong account/changed inbox, From binding, foreign recipient isolation, array headers, normal reply persistence and mock wire assertions. |
| Same IntroMail pin | `backend/app/services/scheduler.py` dispatch_send | Research only: dispatch forwards persisted message fields but generates a random key each attempt. This adapter instead REQUIRES a caller-persisted key, matching the owner's idempotency requirement. No scheduler code is ported. |
| resend/resend-node v6.26.0 `c61cccae2999d50d2aca9ce5fd1064f3bb855219` | `src/emails/emails.ts`; `src/emails/receiving/receiving.ts` get/list; `src/emails/receiving/receiving.spec.ts` get/list/cursor fixtures; `src/emails/receiving/interfaces/{get-receiving-email,list-receiving-emails}.interface.ts`; `src/common/interfaces/pagination-options.interface.ts`; `src/common/utils/{build-pagination-query,get-pagination-query-properties}.ts`; `LICENSE` | `mod.rs` DTOs/cursor union; `client.rs` direct list/detail responses, cid option and cursor query; `pull.rs` accepts SDK fixture timestamps; `tests.rs` HTTP shape/nullability/pagination. MIT ©2023 Plus Five Five, Inc., reproduced verbatim. |
| resend/resend-openapi `8dca27c284bd377e48a6db131b52f6689d53195e` | `resend.json` spec 1.5.1: POST /emails, GET /emails/receiving, GET /emails/receiving/{email_id} and recursively referenced schemas; `LICENSE` | Provider UUIDs, page limit 1–100, Idempotency-Key <=256, recipients/body/response fields. MIT ©2026 Plus Five Five, Inc., reproduced verbatim. |
| stalwartlabs/mail-parser v0.11.1 `6c5d33028530e0ec957f3440ec2ffe65722ecc8c` | `Cargo.toml`, `LICENSES/MIT.txt`, `src/lib.rs`, `src/core/{header,address}.rs`, `src/parsers/mod.rs`, `src/parsers/fields/{address,id,unstructured}.rs`, `src/decoders/html.rs` | Installed parser primitives and HTML-to-text; `normalize.rs` adapts nested-comment/escape/delimiter transitions. Strict adapter checks reject ignored junk, unfinished delimiters and ambiguous routing headers. MIT alternative, ©2020 Stalwart Labs LLC. Installed `.cargo_vcs_info.json` exactly matches the tag. |
| chatwoot/chatwoot v4.17.1 `b354a9550e1fb59fa537a9c384232cb076213e72` | `app/presenters/mail_presenter.rb` auto_reply?/auto_submitted?/x_auto_reply?; `spec/presenters/mail_presenter_spec.rb` matching cases | `normalize.rs` auto_reply predicate and tests. Existing Chatwoot copyright/MIT text in NOTICE retained, including enterprise exclusion. No enterprise source. |

IntroMail metadata was independently rechecked: private repository, license null,
complete immutable tree (`truncated:false`), no LICENSE/LICENCE/NOTICE/COPYING files.
Its reuse is the owner's explicit authorization in FOUNDING, not an MIT claim.
Evaluated but did not add/copy `staktrace/mailparse v0.17.0`: its message-ID parser
explicitly does not strip comments. No restricted source was borrowed.
`hashify 0.2.9` installed VCS SHA is `74bd8a251dff8a514039e035a6437eabf1b9fd57`;
MIT alternative and ©2025 Stalwart Labs LLC are included in NOTICE.

## Validation progress and corrections

All commands run in this worktree; Rust uses its own `src-tauri/target` output.

| Command | Exit / result |
| --- | --- |
| `cargo fetch --target aarch64-apple-darwin` | 0; only mail-parser 0.11.1 and hashify 0.2.9 added. No existing dependency upgrades; pnpm lock untouched. |
| `cargo check --locked --no-default-features --bin codeg-server` | 0 initial implementation check; final repeat pending parser refinements. |
| `cargo test --locked --no-default-features --lib email_transport::` | Initial 101: missing `.into()` in one test String literal, corrected. Then 13/13 passed; after malformed-address/auto-reply regressions, 15/15 passed, exit 0. |
| `pnpm exec tsc --noEmit` | Initial 2: broad tsconfig included downloaded SDK research `.ts` files. Renamed only ignored cache files to `.ts.log`; rerun 0. No tsconfig/product TS changes. |
| `rustfmt --edition 2021 src/email_transport/{mod,client,normalize,pull,tests}.rs` | 0; only new module files formatted. |
| `git diff --check` | 0. |

Hook audit reconfirmed live at lines 1639/1640 (PreToolUse/PostToolUse), same
session/worktree as above. Protected root planning docs and AGENTS are byte-identical
to the immutable branch base. Initial implementation and final validation commits
and draft PR are recorded below when pushed.
