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

Next: finish parser/HTTP/ownership contracts; implement the internal client and
pull adapter with local mock HTTP tests; run desktop/server checks, focused tests,
typecheck and Clippy; record exact mappings, limitations, commit and draft PR.
