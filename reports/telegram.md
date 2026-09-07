# Step 3 — Telegram Ops review notifications

Email Telegram checkpoint ready for independent acceptance, 2026-09-08. Sole writer in
`/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`, branch
`feat/step3-telegram`, created clean from accepted origin/main
**f76503792e18445c867e565534c1f0a11f6aa786** after PR7 merge
**f9ae7f1ec91fb0a9569f06fa9dddbde2884db1c6**. No extra workers.
Draft PR: **https://github.com/Adanmohh/codeg/pull/10**. Initial contract checkpoint
**84c44414** was pushed before source implementation. Product source is frozen at
**932419e5b47cc1f84440baa05797c7a1212fe782**; final code/test/fixture head is
**c7a46acf30296a9e7c41a5a5c57d47afaeaa00e4**, committed and pushed. This handoff
is email-only, not final Phase 1/design acceptance.

## Ownership and fixture coordination

New owned modules: `src-tauri/src/ops_telegram/`, corresponding commands/handlers,
`src/components/ops-telegram/`, `src/lib/ops-telegram/`, static review/settings
pages and minimal accepted Ops/channel entry points. Reserve
**m20260908_000007_ops_telegram** for notification configuration, snapshot binding
and delivery dedupe; bug host owns 000006. Preserve all accepted migrations and
NOTICE entries. Root planning documents are read-only.

Read the rebrand worker's `reports/bug-workflow.md` directly. Its Bug intake
destination uses the workbench/sidebar and separate host/frontend modules. This
task uses a separate `/ops-review` static page for phone links and scoped
Telegram settings inside the existing Ops view (Telegram notifications button).
This preserves the current remote backend transport and in-memory leave guard.
No new workbench route or changes to the bug host's live modules.
The accepted email review remains authoritative; a typed GitHub review extension
will be coordinated with the separate host rather than copying uncommitted code.

Root's fixture **PID 30815 / port 4320 remains running and unchanged** for final
design. Its static `out/` must also remain unchanged while in use. New Telegram
fixture port is **4323**, with a distinct temporary DB, synthetic credentials,
loopback provider and Playwright CLI session. Isolated build/export setup is
documented below. Root's 4318 and bug host's 4322 are also untouched.

Include the requested **BC-14 receipt_recorded** fixture: a controlled provider
receipt already stored with local recording pending. The real UI's Finish
recording action must create the public message without a second provider call.
This is database-only recovery coverage, not unknown-outcome reconciliation.

## Notification and authorization contract

- Reuse Codeg's existing Telegram backend, channel configuration and keyring
  token lookup. Never enable a channel, start polling, test an existing live bot
  or send a real notification during implementation/tests. New Ops notification
  configuration defaults off and requires explicit operator setup.
- Notification recipients are explicitly authorized **private** recipients,
  bound to a selected Telegram channel and canonical numeric user/chat identity.
  Groups/topics/usernames cannot silently become private destinations. Revalidate
  the target and configuration before dispatch; no caller-supplied arbitrary URL,
  payload, sender or execution action enters the send path.
- Store explicit account, proposal, task, run and complete-payload snapshot hash
  with the recipient/configuration binding. The external message contains only
  fixed review wording and an opaque review URL. No mail subject/body/recipients,
  diagnostic logs, operator credential, snapshot payload or execute capability.
- The review URL is a locator, not authorization. Mobile login uses the existing
  operator boundary, preserves only a validated internal review locator, loads
  the full payload through the protected API and enforces the existing human
  floor and exact edited-payload approval. No open redirect or token in a URL.
- Existing ACP `/approve`, `/deny`, and `approve always` remain ACP-only. No Ops
  approval callback or direct phone approval command is being invented. Unrelated
  recipient/topic/sender inputs must never resolve an Ops proposal.
- Reuse the existing channel scheduling seam for optional bounded queue scans,
  not a new task/notification engine. Durable uniqueness and writer-first claims
  dedupe repeated scans and concurrent attempts. Ambiguous outbound outcomes do
  not trigger blind retries or rich-text fallback sends. A stale link cannot
  revive a denied/canceled/edited proposal or an old task run.

Source finding: Telegram's inherited rich/interactive methods retry **any**
failure as plain text. Ops notification dispatch must use one plain-send attempt
and persist an unconfirmed outcome instead. The inherited `send_message` currently
accepts an empty returned message ID; Ops must require positive provider receipt
evidence before marking notification delivery. These are scoped adapter needs,
not authorization to rewrite the general channel engine.

## Docs-first and borrowing evidence

Read complete ORCHESTRATOR, FOUNDING, STATUS, DECISIONS, AGENTS and
`reports/telegram-integration-contract.md`; also the design acceptance checklist
and bug-workflow coordination report. Applied code-context, frontend-design,
design-checklist, mobile-ui and the already-read Playwright CLI skill. Existing
neutral light/dark surfaces, Hafidh accent pairs and user typography stay intact.

Separate React 19.2.4 package and Cargo manifest reads exited 0. Offline
code-context guide used the existing rag-skills `.venv/bin/python` with
`HF_HUB_OFFLINE=1`, exit 0. Relevant rules: actual Playwright verification;
atomic explicit-file staging; one route per workflow step; human approval.
Dependency query exited 3 because `data/code/approvals.db` is missing. Use local
installed source/types; no invented corpus coverage or model download.

Live docs-first audit records for session
**01a07c1c-d3a3-7c22-a5b6-cedce2970d8d**, this worktree cwd:
PreToolUse **4568 / 1788820633 / exit 0**, PostToolUse
**4550 / 1788820542 / exit 0**. These read commands recorded context_emitted=false;
the source-write hook subsequently emitted at **1788820989**, PreToolUse exit 0,
same session/worktree, before implementation. Further apply_patch calls emitted
live framework reminders, including the final ES2020 test correction. Separate
React package and Cargo manifest reads were repeated before that correction;
the audit still contains live PreToolUse and PostToolUse records at
**1788824206**, same session/worktree, exit 0. Hooks remain enabled and were not
bypassed.

Borrowed Codeg source remains **v0.30.4**, Apache-2.0, immutable commit
**6f6bd648b206412644842a98d9ffeebf57292bed**. Local source reads preceded gh api
tree verification. Exact upstream blobs:

| File | Blob SHA | Use |
| --- | --- | --- |
| src-tauri/src/chat_channel/backends/telegram.rs | 1a55a50f181cdbe220fcf7299951171b7132ca04 | Existing send/config/recipient primitives |
| src-tauri/src/chat_channel/manager.rs | 6c49b49af5ecf4580e915bf8d08896d738e36a31 | Shared lifecycle; do not start polling for Ops setup |
| src-tauri/src/chat_channel/traits.rs | 670b25efa150ce2c236b9be16e5dcb49a0ab230a | Backend interface |
| src-tauri/src/chat_channel/types.rs | 6c1d342b39d159f5df35d3513ff9aae9c8015c2d | Typed channel/target/receipt |
| src-tauri/src/chat_channel/scheduler.rs | 779439d929773708c9dc3ed591a40d925be61fa4 | Existing bounded recurring host seam |
| src-tauri/src/chat_channel/session_bridge.rs | 980920f906d0efb040a671a0daeb78dd67b2e7ec | ACP permission ownership audit |
| src-tauri/src/chat_channel/session_commands.rs | 58d94126ca966199260445b4f35d54eee7698844 | Existing ACP-only approval path |
| src-tauri/src/chat_channel/session_event_subscriber.rs | 6fed9ec465dffdb7fd618d0953e6ecb38595e681 | Standing ACP approval boundary audit |
| src-tauri/src/db/entities/chat_channel.rs | 1162d74ee913d04801157d212dd4d7cbe399cd1c | Existing channel config storage |
| src-tauri/src/db/service/chat_channel_service.rs | 55a364063b6ec1f3338e534a16406d4a59a643ff | Existing channel lookup |
| src/app/login/page.tsx | a02ef36fe3eeaeee1364c97ca6bff82e0e18edb2 | Existing operator login |
| src/lib/transport/web-auth.ts | dde9cc4a2a0639e2775a25d12d795d820761f377 | Closed login return locator |
| src-tauri/src/db/service/canvas_service.rs | bbb67591739afe7c36457cbe044dffd91c597587 | Writer-first transaction |
| src-tauri/src/keyring_store.rs | d3e9041b95ebf2b36db66f5d15ba15df2ec494cd | Existing credentials, no copied store |
| src-tauri/src/commands/canvas.rs | d9258d6d5c80c10efca2f24cc1dc1c68b869ced7 | Tauri wrappers |
| src-tauri/src/web/handlers/canvas.rs | 001091c40c6d7ba66e75a78d78e1bd3e58ab0e87 | Protected HTTP wrappers |
| LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | Original Apache-2.0 retained |

Official Telegram API source resolved once through gh api to
`tdlib/telegram-bot-api` **e3e9dd8e5b3d7ab8537cd5a10dc31d5ffa8f82d1**,
Client.cpp blob **3beb03068fbd79deb13f94359c7192f221849fae**. Relevant API contract
reads cover JsonUser.is_bot, JsonChat.private/id, JsonMessage receipt/chat/topic,
getChat/sendMessage and link_preview_options.is_disabled. LICENSE_1_0.txt blob
**36b7cd93cdfbac762f5be4c6ce276df2ea6305c2** was read in full (Boost-1.0).
No Telegram C++ code is ported and Codeg is not repinned. NOTICE now maps the
Codeg adaptations and accepted local Ops reuse. No AGPL/enterprise source.

Installed primitives read: reqwest **0.12.28** ClientBuilder retry/redirect/
no_proxy/timeouts and Response.chunk; Tokio **1.49.0** timeout cancellation and
spawn_blocking limits; SeaORM **1.1.19** ActiveModel.reset_all and transaction
begin/commit; URL **2.5.8** origin/credentials/path/query/fragment; React **19.2.4**
and installed React types for state/effect/ref/callback; Next **16.1.6** navigation
source/types and build/export custom distDir handling. Local Next source confirms
`CODEG_EXPORT_DIR=out-telegram pnpm build` exports to **out-telegram/** while
preserving root's running **out/**. The build uses this worktree's .next cache.

## Implemented contract and source checkpoint

- Protected POST APIs: `ops_telegram_status`, `ops_telegram_configure`,
  `ops_telegram_disable`, `ops_telegram_notify`, `ops_telegram_resolve`, with
  matching Tauri commands. Configuration input is closed and includes channelId,
  privateUserId, reviewOrigin, enabled and expectedRevision. Resolve accepts only
  `notice`, a canonical opaque UUID. Transport supplies Operator; no actor,
  sender, account, action name or target override is accepted.
- `/ops-review?notice=<uuid>` preserves only this locator through existing login.
  It reuses the real complete-payload ReviewCard and original-thread view. Login's
  touched input now has a persistent label, error association and 44px controls.
  All existing exact authorization/receipt-only email behavior remains in Ops.
- Default-off configuration explicitly binds the operator account, channel,
  private positive numeric user/chat, full channel config hash and configuration
  revision. No existing channel is connected/enabled, no token is copied into a
  new store, and no polling starts. State shows absent/disabled/changed recipient/
  unavailable token; saving configuration does no provider I/O.
- Named migration **m20260908_000007_ops_telegram** stores proposal/task/run,
  typed email action, full snapshot SHA-256, recipient/configuration and claim
  ownership. It stores no mail/private payload. Unique proposal notice and a
  writer-first claim dedupe concurrent scans/restarts. Resolution verifies the
  current binding and complete displayed snapshot again.
- Root's early review correctly identified unbounded scan time and nonrecoverable
  preflight errors. The existing scheduler now runs this scan after its legacy
  scheduled channel work. **Whole scan: 8s, cleanup: at most 1s**. Credential
  reads run off the executor with a 2s wait and at most one blocked lookup per
  runtime; a late lookup cannot send. Network awaits never hold a SQLite writer.
- Pre-send getChat failure/cancellation becomes `preflight_failed`, safely
  retryable on the next queue check. A 30s checking lease handles abrupt restart;
  its replacement claim ID prevents an old resumed checker from sending. After
  POST begins, cancellation becomes durable `unknown`; sending/unknown/failed/
  sent are never reclaimed or blindly resent. If shutdown/database contention
  prevents cleanup, persisted sending still renders unconfirmed and cannot retry.
- Scoped Telegram client disables proxies, redirects, protocol retries and rich
  fallback. It bounds decoded responses to 64KiB, validates the private chat
  before send and requires a positive message ID, same private receipt chat,
  bot sender and no topic. Outbound text is fixed wording plus locator; link
  previews are disabled. Provider bodies/errors/tokens are never surfaced.

The unavoidable post-preflight race can deliver a harmless stale notification
if a proposal changes after its last writer check. The link cannot authorize it;
the protected resolver and existing human approval CAS both recheck current state.
No direct Telegram approval/callback route exists. Link possession does not prove
Telegram sender identity and grants no extra operator permission.

Email-only typed projection is intentional for this checkpoint, as root confirmed.
After GitHub host PR9 acceptance, coordinate a typed issue projection/locator with
its owner. Do not extend this schema through an arbitrary action dispatcher or
copy its uncommitted source. Phase 1 P1 Telegram and final design remain follow-ons.

Root also reproduced locale-change unsaved-edit loss on its unchanged 4320 fixture.
Read full i18n-provider.tsx: appReady replaces the mounted subtree while locale
messages load. Root has reassigned this required P2 fix, Ops RTL and internal-copy
design fixes to the freed rebrand worker after PR9 handoff. No locale product
edits belong on this Telegram branch. That follow-on must preserve initial boot
semantics and in-memory edits, with reply/note/review regression checks across
language changes from a separate settings tab. Root owns the integrated Design
rechecks. After PR10/PR9 acceptance, this worker's next priority is typed P1 issue
phone review; it is not implemented or claimed here.

## Completed validation and limits

Meaningful cases: wrong private recipient/channel/topic/sender; no actor spoofing;
stale/denied/canceled/edited/run-changed snapshots; account scope; concurrent claims
and restart dedupe; ambiguous outcome/no retry; sanitized notifications/errors;
configured-disabled/missing-key states; and absence of an Ops command approval
route. Actual CLI mobile link → login → full payload review, desktop/mobile light/
dark, and BC-14 recovery-state visibility are verified by the worker. Root then
completed the reserved full approval and BC-14 recovery against the same protected
API/provider-loopback fixture, as recorded below. No real provider or paid inference.

Rust commands ran from `src-tauri/` with **CARGO_TARGET_DIR=target-approvals**;
frontend commands ran from this worktree root. No other target/output was used.

| Command | Result | Local ignored log |
| --- | --- | --- |
| `cargo check --locked` | exit 0, default desktop | `reports/telegram-desktop-check.log` |
| `cargo check --locked --no-default-features --bin codeg-server` | exit 0 | `reports/telegram-server-check.log` |
| `cargo clippy --locked --all-targets --features test-utils -- -D warnings` | exit 0 | `reports/telegram-desktop-clippy.log` |
| `cargo clippy --locked --no-default-features --bin codeg-server --lib -- -D warnings` | exit 0 | `reports/telegram-server-clippy.log` |
| `cargo test --locked --no-default-features --lib ops_telegram -- --nocapture` | 13 passed, exit 0, 0.77s | `reports/telegram-tests.log` |
| `cargo test --locked --no-default-features --lib chat_channel::backends::telegram::tests -- --nocapture` | 12 passed, exit 0, 0.02s | `reports/telegram-channel-tests.log` |
| `pnpm exec vitest run src/lib/ops-telegram/locator.test.ts src/components/ops-telegram/settings.test.tsx src/components/ops/ops-flows.test.tsx src/components/ops/session.test.tsx` | 11 passed, exit 0 | `reports/telegram-frontend-tests.log` |
| `pnpm exec vitest run src/lib/ops-telegram/locator.test.ts` | 2 passed after ES2020 test correction, exit 0 | `reports/telegram-locator-recheck.log` |
| `pnpm exec tsc --noEmit` | exit 0 after final test correction | `reports/telegram-typecheck.log` |
| `CODEG_EXPORT_DIR=out-telegram pnpm build` | exit 0; static review page exported; root `out/` untouched | `reports/telegram-build.log` |

Root independently reports **13 passed, 1 manual fixture ignored, exit 0, 0.86s**
in `/tmp/ops-telegram-independent-rust.log`, plus **11 frontend tests passed** and
an independent Playwright CLI verification of the login correction. That is
root-provided evidence, separate from the worker runs above. The unchanged Rust
suite was not repeated just for root. Runtime/Clippy gates include the new manual
fixture; the subsequent Rust change only restores an inherited fixture username
to its original upstream value. No production frontend change occurred after the
isolated build.

The first Rust run was 12/13: an assertion assumed 404, while the inherited
missing-API fallback is 501. Read the fallback and corrected the assertion;
all 13 then passed. This did not add an approval route. Inherited Telegram tests
initially passed 11/12 because rebranding had replaced the synthetic username
`CodegTopics` with `Hafidh Ops DeskTopics`, while assertions still required
`@codegtopics`. Verified Codeg v0.30.4's exact original fixture and restored only
that test literal; all 12 pass. The initial frontend test had a missing `const`
loop declaration (10/11); fixed and reran. Typecheck subsequently caught a new
test's ES2021 `replaceAll` against this repo's ES2020 target. Read tsconfig and
installed TypeScript `lib.es5.d.ts` String.replace overload, used the supported
global RegExp form, and reran the locator tests and full typecheck successfully.

Frontend component tests explicitly mock the facade; they are not claimed as
provider E2E. Actual browser checks below use the protected API and injected
loopback providers without request interception. Missing/disabled configuration,
retryable preflight failures and unconfirmed sends are covered by backend and
component tests; the shared browser fixture stays configured to preserve root's
stored notification links. No real Telegram account/device or remote phone
connectivity was tested. The inherited single-tenant operator boundary is not an
OS sandbox against a malicious same-user process.

Build-only limitations: existing proc-macro-error2 future-incompatibility notice,
large macOS test-link unwind warning, and the inherited development sidecar
placeholder are not release-package validation. The isolated fixture's synthetic
workspace path produces inherited get_git_head/start_workspace_state_stream 404
console entries; no Telegram review API error was hidden. Wrong-login 401 is
intentional validation. Root owns final Design Studio acceptance and inherited
shell contrast/unnamed controls, plus the separately assigned locale/RTL/copy work.

Commands so far: clean status, fetch, branch creation, planning/source reads and
immutable gh api lookups exited 0; code-context exits are above. Two exploratory
searches used absent auth filenames/globs, then actual web-auth.ts was discovered.
One additional exploratory read used an absent ops/auth.rs filename before
reading the actual Operator boundary in ops/mod.rs. All product source writes
are confined to this worktree. No real channel configuration/credential reads,
sends or polling have occurred. Test provider calls are loopback-only.

## Live isolated fixture — released for root mutations

Source checkpoint **932419e5b47cc1f84440baa05797c7a1212fe782** is pushed to PR10.
The fixture is now listening at **http://127.0.0.1:4323/__telegram_fixture**,
owned **PID 51913**, log `reports/telegram-browser-server.log` (ignored).
Worker browser session **ops-telegram-check is closed**. Synthetic operator token:
**ops-telegram-synthetic-operator**. This is fixture-only and authenticates no
real server. Credentials and all provider implementations are injected locally.

Reproduce from this worktree (both commands use only owned outputs):

```sh
CODEG_EXPORT_DIR=out-telegram pnpm build
cd src-tauri
CARGO_TARGET_DIR=target-approvals cargo test --locked --no-default-features --lib ops_telegram_browser_fixture -- --ignored --nocapture
```

The ignored test creates a new temporary DB and loopback Telegram/Resend servers;
it never reads real channel tokens, connects/polls a bot or starts the scheduler.
The fixture landing page has real stored notice links for pending/stale/denied/
canceled proposals. Opening pending while signed out goes to the existing login
with only `opsNotice=<UUID>` and returns to the protected full review after login.
Actual CLI at 390×844 reached this login with its locator intact.

On fresh seed, BC-14 **proposal #2** has a real synthetic Resend receipt and
`receipt_recorded` state, produced through the actual approve/client path with a temporary SQLite
trigger blocking only local public-message recording. The trigger is removed
before serving. Open Ops → Approvals → Reply review #2 → Finish recording receipt.
Protected GET **/api/ops_telegram_fixture_stats** reports scalar synthetic provider
request counts and receiptProposalId; initial emailProviderRequests is **1** and
telegramSendRequests is **4**. Finishing the stored receipt must leave the email
request count unchanged. The stats/landing routes are compiled only in this test.

**Worker released the fixture before root's successful mutation pass below.**
Pending proposal **#1** at notice **a4c39cdf-5586-40db-9c31-9fd12639ed66** and BC-14
receipt proposal **#2** were untouched after seeding. Final worker browser stats
were **emailProviderRequests=1**,
**telegramSendRequests=4**, receiptProposalId=2. The worker performed no save/
approve/deny/configure/notify/finish calls after seed; its unsaved field edits and
theme choice were browser-local. PID 51913 remains listening unchanged. No reseed
is needed. After root finishes its session, it may stop/reseed only this fixture
with the command above (new locators appear on its landing page). Never stop root
PID 30815 / 4320, whose listener/static out remain unchanged.
Remote phone access is untested and requires an operator-configured reachable
protected HTTPS origin; 4323's loopback link is synthetic browser validation only.

Actual Playwright CLI **0.1.18** worker evidence (no MCP, no package upgrade):

- At 390×844, pending link while signed out preserved `opsNotice` through login.
  Wrong synthetic token showed the associated invalid-token error. Correct token
  plus keyboard Enter opened the actual protected complete review.
- Full From/To/Cc/Bcc/subject/body/thread-header fields were available. Client-only
  Cc/Bcc/body sentinels survived 390→1280→390 and selecting Appearance → Dark in
  another settings tab. The native leave confirmation appeared; dismiss retained
  the unsaved payload. This theme check does not claim the known locale bug fixed.
- Stale, denied, canceled and invalid links rendered the unavailable heading with
  no editable payload. Configured Telegram settings showed the explicit recipient,
  origin guidance and notification history without initiating a queue check.
- BC-14's actual queue entry opened to `Provider accepted · recording pending`
  with the enabled Finish recording receipt control. Left it unclicked for root.
  Final mobile document width was exactly 390px for the 390px viewport.

Committed screenshots: [mobile light](telegram-mobile-light.png),
[desktop light](telegram-desktop-light.png), [mobile dark](telegram-mobile-dark.png),
[desktop dark](telegram-desktop-dark.png), [stale mobile](telegram-stale-mobile.png),
[scoped settings mobile](telegram-settings-mobile.png),
[BC-14 reserved recovery](telegram-bc14-reserved.png). CLI snapshots and console
logs remain local in ignored `.playwright-cli/`. These are synthetic browser
validation artifacts, not final Design acceptance or real-phone screenshots.

**Root independent mutation pass completed after worker handoff:** root reports
proposal #1's Bcc/body edited and approved through the real `/ops-review` card.
Email provider requests went **1→2**, Telegram remained **4**, a sent receipt was
shown and exactly one public outgoing copy was recorded. For proposal #2, Finish
recording changed **receipt_recorded→sent**, removed the recovery button and
recorded exactly one public copy while counts stayed **email 2 / Telegram 4**.
No second provider request occurred for BC-14. Root reports no new backend finding;
its artifacts are in root's `reports/browser-phone-independent`. These are
root-provided independent results, not worker assertions of having repeated them.
The generic no-receipt terminal copy remains the known Design follow-on. The live
fixture now contains those completed decisions; no reseed/repeat is needed.

## Final commit and PR handoff

- Initial contract: **84c44414**, pushed before implementation.
- Production implementation: **932419e5b47cc1f84440baa05797c7a1212fe782**,
  `feat: add bounded private Telegram review notifications`.
- Final code/test/fixture head: **c7a46acf30296a9e7c41a5a5c57d47afaeaa00e4**,
  `chore: add isolated Telegram review validation`. Commit/push exited 0.
- `git diff --check` and staged diff check exited 0. Only named owned files were
  staged; NOTICE and accepted migrations are preserved. Lockfiles and root
  planning documents were not changed. Later Pi/bug-host integration is outside
  this frozen validation tree rooted at **f7650379**.
- Draft **PR10** to `Adanmohh/codeg:main` has its final title and validation body;
  `gh pr edit 10 --repo Adanmohh/codeg --body-file ...` exited 0 after installed
  help was read. No merge, deployment or messages/comments to people.

This report-only finalization follows the validated code/test head. The fixture
listener remains available to root; no more worker browser/provider mutations
or broad tests are pending for this email Telegram checkpoint.
