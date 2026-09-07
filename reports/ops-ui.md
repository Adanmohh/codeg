# Step 2 — Ops inbox, approvals, morning desk and Resend integration

Worker delivery complete, **2026-09-08**. Branch
**`feat/step2-ops-ui`**, sole writer in
`/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`.
Validated product head:
**`0e375c9770505ac708ca97d12d07163c81a03f15`**.
Draft PR: **https://github.com/Adanmohh/codeg/pull/7** (open, draft, base main).
Root owns acceptance and merge; no merge, deployment, live email or live issue
was performed.

## Immediate browser handoff

**Root's independent mutation pass is complete. Leave PID 30815 running unchanged
for final design review.** It listens at **http://127.0.0.1:4320**, confirmed by lsof.
Our Playwright CLI session **ops-ui-check is closed**. Root's independent session
is **ops-ui-independent**. Root's separate server/session on **4318** was untouched.

PID 30815 started from the current product source and rebuilt `out/`, with a
fresh temporary database and memory credentials. At handoff, all four original
proposals were pending (number 4 intentionally stale), no inbox key was configured
and no pull/send/deny/note/draft mutation had occurred. Root has since completed
the independent pass summarized below. Reload the
page when switching fixture instances to clear the prior client view.

Synthetic operator token: **ops-ui-synthetic-operator** through the ordinary login
page. Synthetic inbox key: **fixture-resend-key**. These fixture strings have no
authority outside this test server. The real protected API, SQLite store,
approval gate and accepted Resend HTTP client execute; only credentials and the
upstream provider are replaced by test-only memory storage and an HTTP loopback
provider. No frontend request interception is used for these P2 browser flows.

Fresh seeded proposals:

| Proposal | Thread | Provider behavior |
| --- | --- | --- |
| 1 | A real local thread | Successful synthetic receipt; edit full payload before approving |
| 2 | An uncertain delivery | Exact text `simulate unknown` returns HTTP 500 |
| 3 | A rejected delivery | Exact text `simulate rejection` returns HTTP 422 |
| 4 | An outdated review | Newer draft revision; send disabled, denial available |

Pull now imports one synthetic message; another pull returns a duplicate. The
import contains a literal script string to verify escaped plain-text rendering.
Configuring a key affects only the memory credential store. The fixture client
constructor accepts only loopback endpoints and is compiled under cfg(test).

Reproduce after root relinquishes the fixture, from this worktree:

```sh
pnpm build
```

Then from `src-tauri/` (own build output only):

```sh
env CARGO_TARGET_DIR=target-approvals cargo test --locked --features test-utils --lib ops_ui_browser_fixture -- --ignored --nocapture
```

Manual fixture:
`src-tauri/src/ops/tests/integration/browser.rs::ops_ui_browser_fixture`.
The current launcher is exec session **58152**; local ignored log:
`reports/ops-ui-browser-handoff-server.log`.
To stop after review, verify the listener with
`lsof -nP -iTCP:4320 -sTCP:LISTEN`, then terminate that owned fixture PID.
A new invocation reseeds a fresh database. Prefix the command with
`OPS_UI_FIXTURE_EMPTY=1` for the empty-instance flow. Do not stop root's 4318
process. Previous worker fixtures 54941 and 27901 were deliberately terminated
after their scenarios; these long-running manual tests are not counted as passes.

## Delivered behavior and boundaries

Real scoped inbox/thread views, private notes, revisioned complete reply drafts,
pending/recent review queue, and morning queues work in both HTTP and desktop
command transports. Incoming content is escaped plain text; no HTML execution or
remote images. To/Cc/Bcc, sender, subject, body and threading headers are visible
in the editor. Sender stays bound to the inbox identity. Attachments and outgoing
Reply-To are outside this initial draft contract, rather than silently omitted
from an accepted payload.

The existing instance operator bearer middleware supplies
`web::auth::AuthenticatedOperator`; HTTP handlers construct `Operator::server()`
only behind that boundary. Tauri commands use the existing trusted local desktop
boundary. Read-only `Operator::account_id()` and `Operator::actor()` are available
to the separate intake host; constructors remain non-public outside the crate.
No caller-selected actor/account, arbitrary action name or execution URL is
accepted. DTOs reject unknown request fields. The existing bearer identifies the
instance operator, not individual employees or per-account RBAC.

Trusted `CODEG_OPS_ACCOUNT_ID` selects the account (default 1); invalid,
nonpositive and non-Unicode configuration fails closed. Each inbox/thread/note/
draft/proposal query validates account ownership and the full resource chain.
Morning work tasks retain the inherited instance-wide visibility.

The closed email action is **ops.email.reply**. Proposal preparation and approval
recheck draft revision, account/inbox/thread ownership and live work-task run
inside the relevant SQLite transaction, including resource validation before an
ask rule can short-circuit. All sends have the human/destructive floor in every
gate mode. Missing email configuration returns before reserving/consuming
approval, preserving the pending proposal.

`AuthorizedReply` has private fields, read-only accessors and a consuming
`into_parts`; it is neither Clone nor Serialize. It consumes the Step 1
`AuthorizedAction`, and dispatch consumes it once. The executor never rereads a
mutable draft or reconstructs an alleged authorization from request strings.

## API and adapter contracts

Use existing shared transport: HTTP POST `/api/<command>` with
`{ "input": { ...camelCaseFields } }` where input is needed, and the equivalent
Tauri command. All results are scoped DTOs.

| Command | Contract |
| --- | --- |
| ops_context | Scoped account/operator label/inboxes; honest transport availability, no keys |
| ops_inbox_create | Name/email; creates local storage, not a provider mailbox |
| ops_tickets | inboxId, optional status, bounded page; scoped summaries |
| ops_thread | inboxId/conversationId; operator thread, explicitly marked private notes and draft |
| ops_note_add | Thread key/content; backend author, always private |
| ops_draft_save | Thread key, expectedRevision, complete reply; revision CAS |
| ops_proposals / ops_proposal_get | Scoped queue/detail, complete pending payload, run/stale/delivery state |
| ops_proposal_approve | id, expected complete original payload, complete edited payload; server-selected registry/actor |
| ops_proposal_deny | id and original review snapshot; redacts terminal private payload |
| ops_morning | Existing work-task queues and scoped correspondence/review work |
| ops_email_status | inboxId; configuration/pull status without credential values |
| ops_email_configure | inboxId/apiKey; secret is write-only, not Debug/Serialize |
| ops_email_disconnect | inboxId; removes current credential binding |
| ops_email_pull | inboxId; bounded, serialized pull and scoped inserted/duplicate counts |
| ops_email_reconcile_receipt | id = proposal ID; completes local recording from durable receipt, no provider call |

Complete proposal payload:
`{draftId, draftRevision, reply:{inboxId, conversationId, from, to, cc, bcc,
subject, text, inReplyTo, references}}`. Review edits cannot replace the bound
draft/thread. A newer saved draft makes the old proposal stale. Denial remains
possible for a stale draft while its pending task/run remains live. Terminal
review payload content is redacted.

The facade registry remains email-only. Accepted `ops_intake::github` supplies
`github.create_issue`, frozen PreparedIssue and consuming AuthorizedAction
dispatch; its separate host/queue worker owns that integration. No generic
executor was added to bridge the gap.

### Trusted pi helper seam

Read the pi worker's report directly. The pi bridge owns token/parent/task/run/
agent/account attribution and must construct this context from backend state;
it must never deserialize these fields from agent-selected JSON:

```text
ops::agent::RunContext {
  account_id: i32, task_id: i32, run_seq: i32,
  connection_id: String, agent_id: String
}
agent::account_id() -> Result<i32, AppCommandError>
agent::context(db, &RunContext) -> Result<agent::Context, DbError>
agent::tickets(db, &RunContext, TicketsInput) -> Result<TicketPage, DbError>
agent::thread(db, &RunContext, ThreadInput) -> Result<Thread, DbError>
agent::save_draft(db, &RunContext, SaveDraftInput) -> Result<Draft, DbError>
agent::command_error(DbError) -> AppCommandError
review::propose_reply(db, task_id, run_seq, agent, account_id, draft_id, expected_revision)
```

RunContext has no Deserialize. Helpers validate the current run/connection, live
folder and running/awaiting-input task. Agent save acquires SQLite's writer lock
before checking that context and draft CAS in the **same** transaction. Public
thread/list/context helpers check the live run inside their read transaction.
Private notes, counts and activity timestamps/order are excluded before DTO
construction; an exact before/after projection regression covers private-note
metadata leakage. Errors must pass through `agent::command_error`.

The bridge receives no Operator human identity, private-note interface, approval
interface, keys or send interface. This worker did not implement pi token ingress
or claim an autonomous agent-to-send E2E. Preparing/saving a human draft works
independently; turning it into an agent proposal requires the separate trusted
bridge. Existing ACP/terminal launch-point changes remove CODEG_TOKEN from merged
agent environments. Same-user full-filesystem agents are not an OS security
sandbox; no malicious same-user process isolation is claimed.

## Durable email integration

Uses accepted `codeg_lib::email_transport::{ResendClient, SendEmail, PullOptions}`.
Per-inbox configuration uses the existing keyring/private-file store with random
credential references, rather than database IDs as shared key names. No key is
stored in SQLite/read DTOs/errors/audits/agent environments. The server file-store
lock now covers the complete read/modify/write operation; its concurrency test
uses a temporary explicit path, never the user's keychain.

One in-process operation per inbox serializes configure/disconnect/pull/approve.
Manual pull is bounded to five pages of 100 and a 55-second whole-pass deadline;
no scheduler engine. Status records success/failure and time. Returned counts
are scoped imported/duplicate messages, not provider-wide metadata.

Migration **m20260907_000003_ops_ui** stores revisioned drafts. Reserved
**m20260907_000004_ops_email** stores per-inbox credential references/pull status
and durable attempts. Accepted ticket/approval/intake migrations remain registered
in order 000001–000005. Tests target named migrations and check rollback
atomicity, without assuming which migration is last.

Dispatch sequence:

1. Configuration preflight, then reserve the exact reviewed payload/hash,
   proposal/draft revision/task/run/actor, RFC Message-ID and idempotency key.
2. Consume committed approval; claim reserved → sending only with the owned
   AuthorizedReply and exact stored payload/hash match, with current task/run and
   scope checks in the writer transaction.
3. Commit before the provider call. Persist a verified receipt as receipt_recorded
   before receipt-only `record_public_reply`, using the immutable attempt content.
4. Mark sent only after local thread recording. `reconcile_receipt` can finish
   that database-only step idempotently without network I/O or draft reread.

Unique proposal/draft-revision bindings and an unresolved-thread index prevent a
new delivery key after ambiguous delivery, including after restart or a new draft.
HTTP 5xx, timeout/transport ambiguity and malformed success stay unknown. Definite
rejection stays failed. Neither is shown as sent. A crash can leave reserved or
sending unconfirmed; no automatic resend or capability reconstruction exists.
Provider acceptance is explicitly distinguished from recipient delivery.

The sole `email_transport` source edit is a **cfg(test)** visibility change
`pub(super) → pub(crate)` for the existing loopback-checked `local_mock`
constructor. All integration code lives in new Ops modules. No transport internals,
pi-worker implementation or intake-host source was copied from uncommitted work.

## Independent browser findings fixed

Root's P2 breakpoint finding is fixed at
**0e375c9770505ac708ca97d12d07163c81a03f15**. A backend-keyed in-memory provider
above the responsive shells preserves selection, draft/private-note/review edits,
original draft revision/review payload and busy state across remount. No
localStorage, disk storage or global singleton holds private content. A backend
switch destroys the old provider synchronously. Only explicit confirmed navigation
discards edits; a layout remount is not a discard.

Two regressions render the real OpsPage through different parent component types,
forcing actual subtree remounts; they cover draft, note, review/Bcc/selection,
no automatic writes and colliding IDs after backend switch. Our CLI repeat and
root's independent repeat preserved draft 1280→390, note 390→1280, review edits and
selection; discard dismissal retained edits and accepted discard navigated.
Root's evidence: `/tmp/ops-ui-preserved-review-mobile.png`. Root reported seven
independent Ops frontend tests passing at the immutable fix head.

Root's light Deny contrast finding is also fixed at that head. Ops-only
destructive buttons use the paired foreground token while retaining shared tint
and focus styles. Rendered enabled 14px Deny measured **16.94:1 light** and
**14.69:1 dark**, with no 390px horizontal overflow. Measurement uses computed CSS
composited root-to-control on an sRGB canvas and WCAG luminance; evidence
`ops-ui-evidence/contrast.json`. Root independently reported the light control
passing, then selected actual Appearance → Dark in a separate settings tab and
confirmed workspace theme sync. Its 99-text-sample dark probe found no Ops
failures; only inherited "No chats" contrast 4.06 remained. Evidence:
`/tmp/ops-ui-dark-design-probe.json`. Inherited shell findings and terminal receipt
copy remain for root's Design Studio follow-on.

## Validation evidence

All Rust commands below ran from this worktree's `src-tauri/`, with
**CARGO_TARGET_DIR=target-approvals**. No other worktree/main build output was used.
Final Rust checks/gates include accepted main and the ordered combined migrations.
The memory/contrast fix is frontend-only; backend source stayed stable.

| Command | Exit / result |
| --- | --- |
| cargo check --locked | 0, default desktop |
| cargo check --locked --no-default-features --bin codeg-server | 0 |
| cargo test --locked --features test-utils --lib ops::tests | 0, 22 passed / 1 ignored manual fixture |
| cargo test --locked --no-default-features --bin codeg-server --lib ops | 0, 135 passed / 1 ignored; combined Ops/approval/intake coverage |
| cargo test --locked --no-default-features --lib keyring_store::tests | 0, 7 passed |
| cargo test --locked --no-default-features --lib work_task::engine:: | 0, 128 passed |
| cargo test --locked --no-default-features --lib work_task_service:: | 0, 39 passed |
| cargo test --locked --no-default-features --lib ticket_service:: | 0, 18 passed |
| cargo test --locked --no-default-features --lib email_transport:: | 0, 18 passed |
| cargo clippy --locked --no-default-features --bin codeg-server --lib -- -D warnings | 0 |
| cargo clippy --locked --all-targets --features test-utils -- -D warnings | 0 |
| pnpm exec tsc --noEmit | 0 |
| pnpm build | 0, rebuilt own static out/ |
| pnpm exec vitest run src/components/ops/ops-flows.test.tsx src/components/ops/session.test.tsx src/contexts/workbench-route-context.test.tsx | 0, 9 passed |
| Focused ESLint on Ops/API and workbench context | 0 |
| Prettier/rustfmt on changed source | 0 |

Logs remain locally under ignored `reports/ops-ui-*.log`. Root separately reported
22 Ops Rust tests plus the ignored fixture passing at the integrated checkpoint;
its first run during a merge encountered temporary conflict markers, then passed
after the stable merge. Component tests use facade mocks and are **not E2E**.
No repeated broad suite is claimed beyond the focused gates above.

Meaningful Rust cases include protected bearer/unknown-field rejection and
cross-account privacy; concurrent draft CAS; stale payload/run and deny;
unconfigured preservation; complete recipient/header/payload binding; exact edited
provider request and receipt-only recording; concurrent approvals/pulls; unknown
outcome after database reopen/new draft; receipt-recording failure and DB-only
recovery; mismatched owned payload; cancel after authorization before HTTP; live
agent-run checks in the writer transaction; public-only metadata; and concurrent
temporary credential-file writers.

Actual **Playwright CLI 0.1.18**, session ops-ui-check, rebuilt production frontend
against the real protected fixture:

| Flow | Observed result / committed evidence |
| --- | --- |
| Empty instance; keyboard-created local inbox | Empty form, creation persists, no imported messages or connected key; empty-inbox.yml, created-empty-inbox.yml, empty-mobile.png |
| Populated thread / private note | Correct private marker; save and Refresh retain the new note; desktop-inbox.png, saved-note-draft.yml |
| Draft persistence | Explicit save produces revision 1; Refresh retains body; no outgoing message from save |
| Missing configuration | Pending review preserved; honest disabled send/configuration state; approval-unconfigured.png |
| Configure / Pull now / repeat | Actual API and loopback provider; 1 import, then 0 imports/1 duplicate; pulled.yml, pull-repeat.yml |
| Untrusted incoming text | Literal script visible, window.fixtureUnsafe not true; no horizontal overflow |
| Edit before approval / receipt | Proposal 1 sent exact edited body and Bcc, receipt recorded, thread shows approved body while saved draft remains original; receipt.yml, mobile-receipt.png |
| Unknown and definite rejection | Proposal 2 unconfirmed/do not resend, proposal 3 not sent/needs attention, no approve button on unknown; unknown.yml, rejected.yml, unknown-mobile.png |
| Stale / denial | Send disabled; denial succeeds and redacts payload; stale-dark.png, denied.yml |
| Responsive editing / discard | Draft, note, review/Bcc/selection preserved; dismiss/accept work; mobile-draft-preserved.yml/png, mobile-review-preserved.yml |
| Morning / keyboard / mobile | Tab/Tab/Enter opens Morning with visible focus; actual task and correspondence queues; morning-keyboard.yml, morning-mobile.png |

Root independently repeated the complete real protected-API/loopback pass on
fresh PID 30815: configuration, 1 import then 1 duplicate, escaped script text,
separate draft/private-note saves, edited To/Cc/Bcc/body approval with exactly one
public thread copy, unknown/no resend, rejection/not sent, stale disabled send,
denial/redaction, and Morning showing zero pending/four running tasks. Root's
evidence is `/tmp/ops-ui-independent-{sent,unknown,morning}.png`. These independently
reported results are distinct from the worker's committed screenshots above.

All paths above used synthetic fixtures only, with no request interception.
Provider error paths are real loopback HTTP responses, not fabricated approve
JSON. The fixture's nonexistent synthetic task folder causes inherited
get_git_head/start_workspace_state_stream 404 console entries; no live agent was
started. Two canvas performance warnings came from the contrast probe. Initial
navigation before listener startup refused connection and succeeded after
readiness. Two browser assertions initially used shortened visible labels and
timed out; snapshots/read source confirmed the correct successful denial/pull
states. These are recorded as harness corrections, not silently counted passes.

Build limitations/warnings: own compile-only zero-byte MCP sidecar placeholder
(as documented in Step 0/CI), inherited proc-macro-error2 2.0.1 future
incompatibility, macOS large test-binary unwind warning and Vite CJS deprecation.
This is not validation of a distributable desktop sidecar or live provider account.
Initial compile/test type errors were fixed and the listed checks rerun; an
uncached own-target compile was canceled before using the existing own target.

## Docs-first, provenance and licences

Read complete FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS, the approvals/
tickets/browser-step1/pi-integration-contract reports and design brief before
product work. Applied code-context, frontend-design, design-checklist, mobile-ui
and Playwright CLI skills. Existing neutral light/dark surfaces, Inter/user fonts,
Hafidh accent tokens and mobile/safe-area conventions are retained.

Offline code-context used
`/Users/mohamedadan/projects/rag-skills/.venv/bin/python` and
`HF_HUB_OFFLINE=1`, guide exit 0. Rules used: render untrusted email HTML in a
locked sandbox rather than dangerouslySetInnerHTML (this UI only renders text);
one route per job step; human approval before sends. Dependency corpus query
exited 3 because `data/code/approvals.db` is missing. No missing corpus coverage
or model download is claimed.

Separate initial `cat node_modules/react/package.json` and
`cat src-tauri/Cargo.toml` exited 0. Live hook audit:
`/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`,
session **01a07c1c-d3a3-7c22-a5b6-cedce2970d8d**, this worktree cwd.
Initial Pre/Post records 1469/1468 at 1788812692; latest checked PreToolUse
**line 4025 / 1788819389 / exit 0**, PostToolUse
**line 4008 / 1788819355 / exit 0**. Hooks remained enabled throughout.

Local installed sources/types were read before APIs: React 19.2.4 and its installed
hook/context/useSyncExternalStore types; Testing Library React 16.3.2/query types;
Vitest 2.1.9; TypeScript 5.8.3 DOM types; Axum 0.8.8; SeaORM and migration 1.1.19;
SeaQuery 0.32.7; axum-test 17.3.0; Tokio 1.49.0; reqwest 0.12.28; keyring 3.6.3;
portable-pty 0.8.1; vendored sacp-tokio environment semantics. Installed gh,
formatter and Playwright CLI help preceded their use. All remote source research
used gh api at immutable references; no latest-doc upgrade changed founding pins.
Two final gh requests initially lacked shell quoting around '?ref=' and were
corrected before execution. An unqualified read-only gh pr view resolved upstream;
the final PR check/edit explicitly uses **--repo Adanmohh/codeg**.

Primary borrowed/reused source is **Codeg v0.30.4**, Apache-2.0, commit
**6f6bd648b206412644842a98d9ffeebf57292bed**, tree
**06d0da02a774af27fa4d1cba5f15debcc82c62e4**. Local pinned reads preceded gh api
verification. Exact source-to-port mapping (all paths below are at that commit):

| Source file | Blob SHA | Use in this delivery |
| --- | --- | --- |
| src/components/ai-elements/message.tsx | 2db4c106c1ac32268b0a7c49a5dc7fa4d6836192 | Ops thread Message/MessageContent |
| src/components/chat/plan-approval-card.tsx | ba3bc3f4de7cd3937da0904d1bdb3f0cd8b19451 | Review in-flight/error interaction |
| src/components/tasks/board-columns.ts | df044661034c28291c024a23c16996fd42047eb1 | Morning grouping |
| src/components/tasks/task-row.tsx | 50ab1e72ac33b7ef5cd5f160031292c9bfa76cd6 | Row geometry |
| src/components/tasks/task-card.tsx | b346fc689f1dbf08dddb63295ec0edff62c681d6 | StatusChip reuse |
| src/components/workbench/workbench-content.tsx | e481d04b25c4fdc33fe35b03b8c1630d0089cd65 | Ops route entry |
| src/components/layout/sidebar.tsx | 0e9327027d194236cd9ea027803b03bd849f5dda | Sidebar registration |
| src/contexts/workbench-route-context.tsx | 6f2c3366f022b498a2b53f427e48652e92f22c7b | Lifted memory lifetime and navigation guard |
| src/contexts/automations-view-context.tsx | a12d4e0b89093072489e8f3b47faaafd46175033 | Provider lifetime pattern for Ops session |
| src/lib/transport/index.ts | 813c5acf5d964c6f9bf13f24d675df02100a590a | Shared invoke seam |
| src/lib/transport/web-transport.ts | 828572b9fc665c986259f05c20193feaa0152635 | Existing protected POST |
| src/lib/transport/tauri-transport.ts | 153dae021c1a1691427eedb2e2398a5288dfb4a8 | Desktop command transport |
| src-tauri/src/web/auth.rs | 4e4d9630f1848ae08fbf39e075697a1417d5b80a | Existing bearer boundary marker |
| src-tauri/src/web/handlers/canvas.rs | 001091c40c6d7ba66e75a78d78e1bd3e58ab0e87 | Ops handler glue |
| src-tauri/src/web/handlers/work_task.rs | f4169d2b917cba530cf224845a3545e52c492641 | Shared handler conventions |
| src-tauri/src/commands/canvas.rs | d9258d6d5c80c10efca2f24cc1dc1c68b869ced7 | Dual-runtime wrappers |
| src-tauri/src/db/service/canvas_service.rs | bbb67591739afe7c36457cbe044dffd91c597587 | Writer-first transaction pattern |
| src-tauri/src/keyring_store.rs | d3e9041b95ebf2b36db66f5d15ba15df2ec494cd | Existing keyring/private-file store |
| src-tauri/src/acp/connection.rs | 3beb7c1b69ea4e422829d29dc0e0a12ee0b71ec5 | Merged child environment credential removal |
| src-tauri/src/terminal/manager.rs | 0754d09e243079a519eeb37ded88c17ae4267be6 | PTY child environment boundary |
| LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | Original Apache-2.0 retained |

Destinations are new `src/components/ops/`, `src/lib/ops/`,
`src-tauri/src/ops/`, `commands/ops.rs`, `web/handlers/ops.rs`, migrations
000003/000004 and minimal shared entry points. Existing Step 1 IntroMail gate and
MIT ticket ports are reused at their accepted versions; their immutable source
mapping remains in `reports/approvals.md`, `reports/tickets.md` and NOTICE.
Accepted email transport PR6 / **2fecb1cf** supplies client/parser/types and
`email_transport/tests.rs` loopback fixture pattern; its source/licence mapping
remains in `reports/email-transport.md`. No AGPL, restricted enterprise or PolyForm
source was used. NOTICE appends Ops attribution and preserves all upstream,
ticket, Resend and GitHub sections.

## Commits, integration and remaining limits

| Checkpoint | Result |
| --- | --- |
| 10f2fa9f | API contract pushed before broad implementation |
| 26a4bc8d | Usable inbox/note/draft/queue and early draft PR |
| dce45beb59a09be5285cf3b26eb22f97c4d4bf30 | Durable transport and atomic agent helpers |
| 85a5dd18d76ce0e104ca6aedeb369167da617514 | Public context/privacy tests and isolated browser fixture |
| c60669213d4113d7b78dee7e19325ef4c5af4e32 | Integrated accepted main 5829cfd797afd9088b08611c6c9bbd385be1bd0d, including GH PR5 / 4988f46b |
| 0e375c9770505ac708ca97d12d07163c81a03f15 | P2 responsive edit preservation and destructive contrast fix, committed/pushed |

The new branch began clean from accepted main
56f874a9dcef0faa46aa012c0967a42d31ebdd4b. Accepted Resend main was merged before
integration; the later GH merge preserved NOTICE/modules/dependencies and ordered
migrations. Root planning documents and both lockfiles match the integrated main
baseline; no authored lockfile changes or planning-doc edits. No extra worker was
started and no other worktree was written.

Limits: Ops copy currently English; draft attachments/outgoing Reply-To are not
supported; pi ingress and GitHub host registration belong to separate workers.
No automatic scheduler or resend exists. Unconfirmed reserved/sending/unknown
attempts require provider reconciliation; the UI only supports database completion
when a verified receipt is already durable. Refresh picks up external activity.
No live provider credentials, real-email delivery, distributable desktop build,
malicious same-user process isolation, or full agent-run E2E is claimed. Root's
independent provider-loopback mutation pass and dark Ops review passed. Final
Design Studio follow-on and merge remain root-owned acceptance work; root requested
no further product edits or broad tests before this handoff.
