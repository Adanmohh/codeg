# Step 2 — Ops API, email, approvals and morning UI

Coordination contract and implementation progress, 2026-09-07. Final validation is in progress.
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

## Usable UI/API checkpoint

Early contract **10f2fa9f** was committed and pushed before product edits.
Implemented scoped context/inbox creation/tickets/thread/private notes, revisioned
draft persistence, proposal queue/detail/deny and morning queues in both transports.
New Ops desk sidebar route reuses the workbench's in-memory navigation convention
(no dynamic Next route). The UI renders only escaped plain text, never email HTML.
Drafts and review envelopes include visible To/Cc/Bcc and editable thread headers.
The sender is visible but fixed to the selected inbox identity.

Closed `ops.email.reply` pack reuses the Step 1 gate and run CAS. It rechecks draft
revision and full account/inbox/thread membership inside the approval transaction,
including in `resource` before an ask rule can short-circuit. Review cannot change
the bound draft/thread. Default approval returns `configuration_missing` before
consuming authorization. Denial remains available for a changed draft on a live
pending task. Terminal proposal reply content is redacted.

Owner seam review addressed: `AuthorizedReply` has private fields, read-only
accessors and consuming `into_parts`; no Clone/Serialize or struct-literal
construction outside its module. It consumes the Step 1 AuthorizedAction and
never rereads draft content. It is a future dispatcher handoff, not an executor.

Read tickets `reports/email-transport.md` at checkpoint **7a3c2fd5**. Future
Resend integration MUST persist a delivery attempt, RFC message ID and idempotency
key bound to the exact approved payload, then bind a verified provider receipt
before recording a public reply. None of that transport is wired or copied here.
No migration 000004 is needed by that worker. Our reserved 000003 contains only
draft storage. Agent ingress and durable transport dispatch remain explicit
integration gaps; no agent-facing endpoint gets the instance operator credential.

Audited inherited environments. Minimal existing launch-point changes remove
CODEG_TOKEN from merged ACP environments (including caller-configured overrides)
and terminal children. No unrelated auth rewrite. Existing single-tenant agents
have full filesystem access as the same OS user; this is not malicious same-user
process isolation. Delegation TokenRegistry is only a reference for future
per-run Ops capability binding, not current Ops authorization.

Source proof via gh api: Codeg commit above resolves tree
`06d0da02a774af27fa4d1cba5f15debcc82c62e4`. Verified blobs after installed/local reads:

| Exact upstream file | Blob SHA | Use |
| --- | --- | --- |
| src/components/ai-elements/message.tsx | 2db4c106c1ac32268b0a7c49a5dc7fa4d6836192 | Message/MessageContent in local ticket thread |
| src/components/chat/plan-approval-card.tsx | ba3bc3f4de7cd3937da0904d1bdb3f0cd8b19451 | In-flight ref, failure/retry and review controls |
| src/components/tasks/board-columns.ts | df044661034c28291c024a23c16996fd42047eb1 | Morning grouping over real work tasks |
| src/components/tasks/task-row.tsx | 50ab1e72ac33b7ef5cd5f160031292c9bfa76cd6 | Existing row geometry |
| src/components/tasks/task-card.tsx | b346fc689f1dbf08dddb63295ec0edff62c681d6 | StatusChip reuse |
| src/components/workbench/workbench-content.tsx | e481d04b25c4fdc33fe35b03b8c1630d0089cd65 | Ops route entry |
| src/lib/transport/web-transport.ts | 828572b9fc665c986259f05c20193feaa0152635 | Existing protected POST transport |
| src-tauri/src/web/handlers/canvas.rs | 001091c40c6d7ba66e75a78d78e1bd3e58ab0e87 | HTTP/command glue |
| src-tauri/src/db/service/canvas_service.rs | bbb67591739afe7c36457cbe044dffd91c597587 | Writer-first SQLite transaction |
| src-tauri/src/acp/connection.rs | 3beb7c1b69ea4e422829d29dc0e0a12ee0b71ec5 | Credential removal at merged launch environment |
| src-tauri/src/terminal/manager.rs | 0754d09e243079a519eeb37ded88c17ae4267be6 | PTY child environment removal |
| LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | Original Apache-2.0 retained; NOTICE appended |

Pinned primitives read: React 19.2.4 package plus installed @types/react hooks,
effect cleanup/useId/state/ref contracts; Axum 0.8.8 Extension/from_fn;
SeaORM/sea-orm-migration 1.1.19 transaction/query limit/offset/raw SQL; axum-test
17.3.0 request/response/header source; portable-pty 0.8.1 env_remove/get_env;
vendored sacp-tokio spawn_process blank-to-env_remove convention. CLI help read
before gh api, gh pr create, Prettier and rustfmt. No dependency/lockfile changes.

Checkpoint checks (not final validation): server cargo check --locked with
--no-default-features --bin codeg-server **0**, pnpm exec tsc --noEmit **0**,
focused Ops/workbench ESLint **0**, Prettier and rustfmt **0**. Rust output uses
this worktree's `src-tauri/target-approvals`. An initial uncached own-worktree
target check was canceled (130); corrected to the cached own target. Initial
server compile found WorkTaskStatus/string mismatch (101), fixed and rerun 0.
Focused test compile found a private ThreadHeaders import, corrected to its
discovered public threading module; test run is pending. Full checks, frontend
tests and Playwright screenshots remain to be completed after this early push.
