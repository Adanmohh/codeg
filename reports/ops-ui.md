# Step 2 — Ops API, email, approvals and morning UI

Early coordination contract, 2026-09-07. Implementation and validation are pending.
Sole writer in `/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`, new branch
`feat/step2-ops-ui` from accepted main **`56f874a9dcef0faa46aa012c0967a42d31ebdd4b`**.
The prior approvals branch/worktree was clean before switching. No other worker,
worktree, root planning document, transport module or intake module is being edited.

Reserved migration: **`m20260907_000003_ops_ui`** for draft/UI/action-adapter storage.
Email owns 000004 if needed; GitHub/intake owns 000005. No migration renumbering.

## Authentication and scope contract

- Existing server `web/auth.rs::require_token` verifies the instance operator's
  bearer; existing Tauri commands execute in the trusted local desktop boundary.
  New Ops handlers derive a typed operator principal only from those boundaries.
  No request accepts actor, agent identity, arbitrary action name or execution URL.
  Unknown request fields will fail validation rather than silently accepting actor
  spoofing. The existing shared server token is an instance-operator credential,
  not individual employee identity or per-account RBAC.
- Ops account scope is selected by trusted process configuration
  `CODEG_OPS_ACCOUNT_ID` (default 1; invalid/nonpositive values fail closed).
  Callers choose an inbox/conversation within that scope, never an account ID.
  Every thread, note, draft and proposal lookup validates the whole scoped chain.
  Morning tasks retain existing instance-wide work-task visibility.
- Do not pass the operator bearer into an agent environment, prompt, tool result
  or proposal capability. Audit inherited process environments before delivery.
  Per-run agent proposal/read capability is a separate adapter gap: no public
  agent proposal endpoint will pretend the operator bearer is a restricted token.
  The trusted Rust proposal helper will require an existing live task/run binding.

## Proposed API contracts for transport-worker alignment

Use the inherited command transport: HTTP POST `/api/<command>` and equivalent
Tauri command names. Mutating/read parameters use an `input` object with camelCase
fields; responses use explicit Ops DTOs, not unrestricted database mutation.

| Command | Input / result |
| --- | --- |
| `ops_context` | Selected account, boundary-derived operator label, scoped inboxes and honest transport availability. No credential values. |
| `ops_inbox_create` | Local inbox name/email only. Creates local storage, not a provider mailbox or send authority. |
| `ops_tickets` | inboxId, optional status and bounded page; scoped summaries. |
| `ops_thread` | inboxId/conversationId; scoped contact, messages and draft. Private notes are explicitly marked in this operator-only view. |
| `ops_note_add` | inboxId/conversationId/content; author comes from the principal. Always private; never a public-reply receipt. |
| `ops_draft_save` | inboxId/conversationId, expectedRevision and complete reply payload; revision CAS rejects another tab's newer draft. |
| `ops_proposals` / `ops_proposal_get` | Scoped pending/recent proposal queue and complete supported review payload with live task/run/stale status. Unknown actions show unsupported state. |
| `ops_proposal_approve` | proposal ID, expected complete original payload, complete edited payload. Registry and actor are server-selected. |
| `ops_proposal_deny` | proposal ID and original review snapshot; no caller-selected actor. |
| `ops_morning` | Existing active/attention task queues plus scoped ticket/proposal work, without decorative metrics. |

Closed initial action: **`ops.email.reply`**. The payload binds draftId and
draftRevision to a complete reply containing inboxId, conversationId, from, to,
cc, bcc, subject, text, inReplyTo and references. The sender must equal the scoped
inbox identity; recipients/content are visible together and revalidated after
editing. Attachments are not silently accepted: this initial action has no
attachment-storage capability. A later attachment contract must bind immutable
content descriptors. A draft save after proposal creation makes that proposal
stale; approval must not reread the mutable draft as execution input.

All email actions remain destructive/human-required in every gate mode. A trusted
internal registry selects a concrete action pack; there is no generic action-name
executor. Dispatch consumes the committed **AuthorizedAction** by value once,
extracts the exact approved reply, and passes that owned envelope to a trusted
transport adapter. It never reconstructs content from a draft/proposal row.

Default transport is explicitly unconfigured. Approval must preserve the pending
proposal and explain missing configuration instead of committing authorization
that has nowhere to go. A future adapter returns a typed provider receipt before
the UI can say sent; `record_public_reply` is called only from the trusted receipt
path. Ambiguous delivery/receipt failures must not offer blind replay. No HTTP
endpoint can forge a receipt or mark a message sent. Tests may inject a local
fixture dispatcher; no live email or issue is authorized by this task.

Draft preparation is independently usable without running an agent. Turning a
draft into an agent proposal requires the trusted live task/run adapter; that
capability is not being simulated by minting a fake running work-task or exposing
the operator token. The review UI handles real proposals once supplied through
that internal seam. GitHub actions will join the closed registry only after their
own typed pack is integrated; this worker does not create or edit that module.

## UI direction and staged delivery

Follow `docs/design/BRIEF.html`: existing Inter/user font choices, white/neutral
light and paired dark surfaces, Hafidh primary #245e58 / dark #9bd4c5, inherited
6–14px radii and 4/8/12/16/24/32 spacing. Real ticket correspondence is the central
artifact. Compact inbox list and thread on desktop; one readable pane with back
navigation on phones. A distinct private-note composer and complete recipient
review keep internal notes separate from reply drafts. No mock customers in
production, new decorative counters, unrelated font changes or marketing motion.

First stage: scoped inbox/thread, private notes, revisioned draft persistence,
readable proposal/morning queues, and an early draft PR. Continue through trusted
review/dispatch seam, error/stale/privacy tests and actual browser flows.
Loading, empty, failed, missing configuration, denied, stale and saved states need
concrete next actions. Visible focus, labelled controls, 44px mobile targets,
16px mobile inputs, safe areas and RTL-aware layout follow the applied skills.

Own browser session/server/database will be separate from root's
`ops-desk-check` / localhost:4318. Planned own port: 4320 (verify before binding).
Use Playwright CLI only. Root owns final Design Studio loops; this worker will
test actual empty/populated/mobile/keyboard/error flows and capture evidence.

## Docs-first and sources read so far

Read complete FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS, approvals/tickets
reports, browser-step1, pi-integration-contract and design brief before product
edits. Applied code-context, frontend-design, design-checklist and mobile-ui;
read Playwright CLI workflow. The brief overrides generic cinematic/marketing
styling suggestions in the checklist's broad playbook.

Offline code-context guide with the existing rag-skills `.venv/bin/python` and
`HF_HUB_OFFLINE=1` exited 0. Relevant rules: **Render untrusted email HTML in a
locked sandboxed iframe, not dangerouslySetInnerHTML**, **Each page/route maps to
one job-to-be-done step**, and human approval before sends. Initial UI will render
plain content as text, without HTML execution or remote images. Dependency docs
query exited 3 because `data/code/approvals.db` is absent; installed source/types
remain the authority and no corpus coverage is invented.

Separate React 19.2.4 package and Cargo manifest reads exited 0. Live docs-first
PreToolUse/PostToolUse records at lines **1469/1468**, timestamp **1788812692**,
session **`01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`**, cwd this worktree, both exit 0.
Hooks are enabled. Two preliminary guessed filenames (work_tasks.rs/http.ts)
were absent; actual discovered work_task.rs/web-transport.ts are used instead.

Borrowing remains pinned to Codeg v0.30.4,
**`6f6bd648b206412644842a98d9ffeebf57292bed`**, Apache-2.0. Initial local reads:
`src-tauri/src/web/auth.rs`, `web/handlers/work_task.rs`, `canvas.rs`, `error.rs`,
`src/lib/transport/index.ts` and `src/components/tasks/board-columns.ts`.
Exact chat/message/ai-elements/task/transport source-to-port mapping and immutable
`gh api` blob verification will be recorded before those ports. NOTICE additions
will preserve all current attribution. No AGPL/restricted source or dependency
upgrade is authorized by latest-doc research.

Read the email worker's current report for alignment; its direct Resend client
and parser remain its responsibility. No transport-worker source was edited.
Implementation checks, screenshots, commit SHA and draft PR URL follow as work
lands. This early report is a contract/progress checkpoint, not a completion claim.
