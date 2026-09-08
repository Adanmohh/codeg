# Step 3 — typed GitHub issue phone review

Implementation complete for independent acceptance, 2026-09-08. Sole writer in
`/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`, branch
**feat/step3-telegram-issues**. Draft PR **https://github.com/Adanmohh/codeg/pull/12**.
No additional agents, root planning edits, live provider requests or deployment.
Final integrated Design Studio acceptance remains root-owned.

**Final product commit: 347e06d390890f1517c029f507c4d8859b59906b.** It includes the
accepted-main integration, complete typed issue flow and final hydration correction.
The following handoff commit contains this report/browser evidence only; its exact
head is recorded in the PR body. All gates and final CLI flows below apply to this
product source; no product edits followed browser validation.

**Fresh independent fixture is released:** **127.0.0.1:4323 / PID45025**, all
three issue proposals pending, GitHub creates0 and Telegram notices3. Worker
browser is closed; no further worker mutations/restarts/export writes are planned.
Open `http://127.0.0.1:4323/__issue_fixture` using synthetic-only operator token
**ops-issue-phone-synthetic-operator**. Setup, fresh locators and controls below.

Product checkpoint **8ff15d9d325ddbd529867c0b0d88e91edadfa30f** and fixture/regression
checkpoint **6898431d** are pushed. Accepted main **f4da70275932fcb527e9bd86946f740b3be515e5**
(PR13 merge **783bfb9cd9caf9546f6ef9effc067f5c904fc6be**) is integrated and pushed as
**49fb8f97**. Its sole additive NOTICE conflict was resolved by retaining both
complete sections. Compared with 6898431d, Rust source and lockfiles are unchanged.
Planning documents exactly match accepted main; no other worktree was written.

## Delivered behavior and contracts

- Forward migration **m20260908_000008_ops_telegram_issues** explicitly owns its
  SQLite DDL transaction. It adds `github_issues_enabled=0`, extends the closed
  notice kind to `email_reply` / `github_issue`, and preserves all existing email
  IDs, proposal uniqueness, claims, states, receipts and dates. Failed mid-DDL
  upgrades roll back completely. Down refuses any issue notices instead of
  discarding delivery history. Tests address the migration by name.
- A separate `ops_telegram_issue_binding` epoch and three host-product triggers
  invalidate configuration rebind/restore locators, including ABA restoration.
  Host product columns and historical migrations 000005/000006/000007 remain.
- Closed host seam **`ops_intake_host::notice::{snapshot,projection}`** derives a
  current pending `github.create_issue` from stored data. It verifies account,
  product, folder, source revision/freshness, draft revision and exact PreparedIssue,
  task/run/connection, all four evidence contents/hashes/revocation/expiry, and
  repository/App/installation configuration plus its epoch. The scanner receives
  identifiers/hash only, never a fabricated human Operator.
- Existing **`ops_telegram_resolve`** accepts only an opaque UUID locator. Its
  closed response contains email `proposal`, issue `{source,binding,detail}` in
  `issue`, or neither when unavailable. Issue projection and live validation share
  one SQLite writer transaction. No provider or credential read occurs inside it.
- Existing host approve/deny inputs gain optional **`review_notice`**. Phone
  review submits that locator with the existing complete expected/approved payload.
  The host repeats its exact current binding checks under the approval writer lock.
  Denial delegates to the extracted core `deny_in_transaction`, preserving CAS,
  redaction and ACP/Ops wait reconciliation. Workspace review retains its original
  authenticated boundary; no actor, arbitrary action executor or callback is added.
- The scanner retains one **8s whole-scan + at most 1s cleanup** lifecycle shared
  by both action kinds. Keyset batches skip stale candidates; at most20 claims per
  account are attempted inside that deadline. GET-only preflight failure remains
  safely reclaimable with the same notice ID. An attempted/ambiguous send is durable
  unknown and cannot be blindly repeated. Configuration and binding are checked
  again after recipient preflight and before recording `sending`.
- Issue inclusion is explicitly **off** for existing and new configurations until
  an operator enables it. It also requires the existing default-off master setting,
  configured channel, private recipient and protected review origin. Saving config
  performs no provider I/O; tests do not run the live scheduler or enable a live bot.
- Notifications contain fixed generic wording and an opaque review locator only:
  no issue/mail body, title, evidence/logs, repository, source URL, operator credential
  or execute capability. Wrong account/private recipient/topic/sender and changed,
  denied, canceled, stale or rebound issue locators cannot resolve or decide.
- Phone review reuses the accepted **IssueReviewCard**, showing complete exact
  repository/title/body/labels plus source and four evidence summaries/hashes.
  Explicit human confirmation is mandatory. Draft changes require a new workspace
  proposal; the phone adds no arbitrary edits or task launcher. Created is shown
  only after a receipt. Rejected and unknown states stay explicit; unknown supports
  only the accepted read-only reconciliation, never another create POST.
- **Open workspace** accurately labels the existing `/workspace` destination.
  Shared responsive session safety is preserved. Locale/RTL/Ops internal copy and
  shell fixes remain their assigned owners' work; no new global design is added.
- Focused lint caught the inherited phone boot guard's synchronous effect state
  update. The page now reuses Codeg's existing AppToaster hydration gate: server
  and hydration snapshots stay closed, then browser token retrieval/redirect runs.
  Protected API enforcement and operator identity are unchanged. Final exported
  browser login rejection/success and deep-link return were repeated after this fix.

## Exact branch and integration history

Initial clean base **77c88d9c615072a11b6e24224ec954c398b33518**. Contract
**a7e2e2f3eee3be3e34b0f7006bf8d96bc5f004bb**, docs checkpoint **3d38a844** and
expected-red migration checkpoint **4fefd024** were pushed before host wiring.
Owner then confirmed PR9 accepted at **8703e00fae2e1c92e045936b140b83012cfe0f57**,
merged as **02e3f5d8a15a3fee792cd0625966dee6c500d184**. Accepted main **61738519**
was integrated as **81716c7ab952c4fa655b70c7d7cedae4b01bdc1f**, preserving all
registrations/NOTICE. No unaccepted host implementation was copied.

Current source and fixture checkpoints are at the top. PR creation/edit/push and
merge commands exited0 after installed gh help reads. PR12 stays draft; no merge,
external comment, live send, App installation or paid inference was performed.

## Docs first and borrowing

Read complete FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS, NOTICE, the
Telegram contract, own report and accepted host coordination/source/tests. Applied
code-context with the existing rag-skills `.venv/bin/python`, **HF_HUB_OFFLINE=1**.
Guide exit0: **Human-in-the-loop: nothing is filed/sent without explicit human
approval**, **Strict TypeScript**, **Each page/route maps to one job-to-be-done step**.
Docs query exit3: missing `data/code/approvals.db`. No corpus coverage, ingest,
download or new retrieval environment is claimed. Frontend-design, design-checklist,
mobile-ui and Playwright CLI skills guide the scoped shared UI and local evidence.

Installed pinned reads: React **19.2.4** package and useState/useRef/useEffect/
useSyncExternalStore types; Next **16.1.6** export config; TypeScript **5.8.3**;
Testing Library React **16.3.2** render/rerender and DOM **10.4.1** role-query types;
SeaORM/sea-orm-migration **1.1.19** active enum, transaction and migrator source;
serde **1.0.228** defaults; libsqlite3-sys **0.30.1** / SQLite **3.46.0** ALTER,
trigger and rollback source. Existing Tokio **1.49.0**, reqwest **0.12.28** unchanged.
SQLite Migrator does not automatically transact DDL, which grounds the explicit
migration transaction. Installed CLI/rustfmt/uv help was read before invocation.

Hooks remain enabled. Audit file:
`/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`, own session
**01a07c1c-d3a3-7c22-a5b6-cedce2970d8d**, exact approvals cwd. Live Pre/Post records
include **1788826240**, and source-write PreToolUse records with
**context_emitted:true** include **1788826785**, **1788827184**, **1788827398**;
all exit0. Subsequent live Pre/Post reads at **1788828465–1788828478** confirm the
same session/worktree. No disabled/bypassed hook or command approval.

All remote research used **gh api** at immutable commits, never web search or a
floating latest source. Original pins remain unchanged:

| Original source | Immutable commit / license |
| --- | --- |
| Codeg v0.30.4 | 6f6bd648b206412644842a98d9ffeebf57292bed / Apache-2.0 |
| IntroInnovation/intromail | 0bd24dfe284b888aa9f602fa1fd00e337ea38874 / owner-authorized private source, not represented as MIT |
| tdlib/telegram-bot-api API reference | e3e9dd8e5b3d7ab8537cd5a10dc31d5ffa8f82d1 / Boost-1.0, no C++ port |

NOTICE appends exact source-to-glue mapping, preserving upstream Apache LICENSE,
all prior MIT texts and owner-source attribution. No AGPL/enterprise/PolyForm code.

Accepted PR10 merge **c3cef09a896308b2501947e5aaafa533e36d9053**, verified blobs:

| Exact source file | Blob | Reuse |
| --- | --- | --- |
| src-tauri/src/ops_telegram/mod.rs | e5e6eb6927b55026774b29880c5d6b96e573eae5 | Claims/lifecycle/resolver |
| src-tauri/src/ops_telegram/types.rs | 3f9ef91a6eb57725acd1bfab350891286d5871e3 | Closed DTO/config |
| src-tauri/src/ops_telegram/entity.rs | 3abc3b3d25aa12b94698059370a6062ef1c83591 | Durable notices |
| src-tauri/src/db/migration/m20260908_000007_ops_telegram.rs | be6c56915d3e78e2aefab9556e328254aa82fbd1 | Forward migration input |
| src/components/ops-telegram/review-link-page.tsx | ced19560c628f0cc92f8099ab5a06385b4486ed5 | Protected phone/leave guard |
| src/lib/ops-telegram/api.ts | 1dac2ccc6f1d3d1a64645bdfa98b71d6180e73c6 | Dual-runtime transport |
| src-tauri/src/ops_intake/mod.rs | 5e7a75f17156cb1fbe5dd83265d73a8071ddd209 | Approved dispatch |
| src-tauri/src/ops_intake/types.rs | 3a0c3c9cd12d17afae5364574b3164c63ef6c8c1 | PreparedIssue/repository |
| LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | Retained original license |

Original Codeg `src-tauri/src/db/entities/automation_run.rs` active enum blob:
**1c55bcde715f5ab6d62c7b7ea91d6c49e76fd3c8**. Core denial transaction extraction
reuses `src-tauri/src/db/service/ops_approvals/mod.rs` at accepted PR10 merge.
Client-only boot glue also reuses original Codeg `src/components/ui/app-toaster.tsx`,
blob **44c4e1e4553bb6209fa9a3a38e71d7909975aec1**, at the unchanged v0.30.4 commit.
Read its complete local source, original `git show`, installed React useSyncExternalStore
signature and immutable gh-api blob before appending NOTICE and adapting it.

Accepted PR9 head **8703e00fae2e1c92e045936b140b83012cfe0f57**, verified via gh api:

| Source file (Rust paths under src-tauri/src/) | Immutable blob |
| --- | --- |
| ops_intake_host/review.rs | 2d37cc41a9df58c5598ce2e5b128a9b01a5fae81 |
| ops_intake_host/store.rs | bb016b603412ebb22a0f5ef44252b7397b9eb0e0 |
| ops_intake_host/operator.rs | d2178860743352b9cf5b37d7b5d71ff278af2065 |
| ops_intake_host/types.rs | 3a4f2514e09b05bc98263087d5aa266d97db9586 |
| ops_intake_host/runtime.rs | c7057b15ce7c21dc493cd72b6ea21017e0866b79 |
| ops_intake_host/tests.rs | 86efca2eca48227e8fe5ccbc31ebdab1114f9048 |
| ops_intake_host/tests/fixture.rs | e449ffa4677b400410e48c126647bf54e82df343 |
| ops_intake_host/tests/browser.rs | d8a11abf800455336c5ed2d200332d3592ebdb92 |
| src/components/ops-intake/issue-review.tsx | 595672b0b85ee659b36f1f510aeca71779963d5d |
| src/components/ops-intake/evidence.tsx | 4641c44ed9a44575eb11ae59dbba607eb848314a |
| src/lib/ops-intake/api.ts | d43a4eb3c320d54e6d3c7e8ce9e034b454bc7805 |

Read the full actual host store/review/operator/types/runtime/process, review UI,
evidence/UI/API and fixtures before adaptation. New source is binding/glue plus
focused tests. The fixture shares accepted host/provider helpers with crate-only
cfg(test) visibility; no production transport endpoint or credential is replaced.

## Validation evidence

All Rust commands run from this worktree's `src-tauri/` with
**CARGO_TARGET_DIR=target-approvals**, never a root/other worker output. Logs are
local ignored files under `reports/telegram-issues-*.log`.

| Command / evidence | Observed result |
| --- | --- |
| `cargo test --locked --no-default-features --lib ops_telegram::tests::migration -- --nocapture` initial red | exit101, three missing-registration failures |
| Named migration + existing email cases after implementation | exit0,16 passed |
| `cargo test --locked --no-default-features --lib ops_telegram::tests -- --nocapture` checkpoint | exit0,20 passed,3.67s |
| `cargo test --locked --no-default-features --lib phone_decisions_use_the_core -- --nocapture` | exit0,1 passed,0.49s |
| `cargo test --locked --no-default-features --bin codeg-server --lib ops` at fixture checkpoint | exit0,178 passed,4 manual ignored,10.32s; server binary0 tests |
| `cargo check --locked --no-default-features --bin codeg-server` | exit0 |
| `cargo check --locked` default desktop | exit0 |
| `pnpm exec tsc --noEmit` | exit0 |
| Five-file Vitest command below | exit0,21 passed,1.23s |
| `CODEG_EXPORT_DIR=out-telegram-issues pnpm build` before PR13 integration | exit0,33 static pages including /ops-review |
| Same build after accepted PR13 integration and final boot correction | exit0,33 static pages; `telegram-issues-build-final.log` |
| `cargo clippy --locked --no-default-features --bin codeg-server --lib -- -D warnings` | exit0,29.20s |
| `cargo clippy --locked --all-targets --features test-utils -- -D warnings` | exit0,52.90s |
| Same five-file Vitest command after accepted frontend integration | exit0,21 passed,1.25s |
| Final `pnpm exec tsc --noEmit` after boot correction/export | exit0 |
| Focused ESLint over eight changed TSX/API/test files | initial exit1 for existing boot guard; final exit0 after grounded correction |
| `git diff --check`; root planning/lock comparisons | exit0 |

Exact frontend command: `pnpm exec vitest run src/components/ops-telegram/settings.test.tsx
src/lib/ops-telegram/locator.test.ts src/components/ops-intake/bug-workflow.test.tsx
src/components/ops/ops-flows.test.tsx src/components/ops/session.test.tsx`.
These are component fixtures/mocked API tests, distinct from the real protected
browser/provider flow below. They cover explicit opt-in, complete phone payload,
mandatory confirmation, duplicate-click suppression, repository rebinding and
session safety. Existing email cases stay in the suite.

Rust cases include18 changed-binding variants, wrong account/recipient/topic/sender,
loaded approve AND deny rejection with zero GitHub POSTs, config rebind/restore,
source/proof/draft changes, canceled/stale run/connection, recipient preflight edit,
concurrent approval exactly one POST, and approval/denial retaining independent
ACP waits. New cases cover issue GET recovery vs ambiguous no-repeat, one shared
email/issue timeout and cleanup, and >20 stale candidates not starving a live issue.

Initial setup failures are not concealed: absent local host adapter raised
`AdapterMissing`; using production runtime locks across independent test DBs then
raised `Conflict`. Fixtures now use their own HostRuntime, leaving production
serialization intact. Installed accepted requirements offline in own
`integrations/hafidh-intake/.venv` with CPython3.13.14 using installed uv help:
`uv venv --offline --no-python-downloads --python 3.13 .venv`, then
`uv pip install --offline --python .venv/bin/python -r requirements.lock` and
`uv pip install --offline --python .venv/bin/python --no-deps -e .`; all exit0.
No lock change/download or another environment write. An initial private re-export
compile failure was fixed to crate-only; the fixture's unused import was removed.
Existing linker unwind-size and proc-macro-error2 future-incompat notices remain.
Desktop Clippy also reports the existing build script's missing codeg-mcp sidecar
placeholder; these are compile checks, not a packaged native-app claim. Exact
lint files: `src/components/ops-telegram/{issue-review,review-link-page,settings,settings.test}.tsx`,
`src/components/ops-intake/{issue-review,bug-workflow.test}.tsx`,
`src/lib/{ops-intake,ops-telegram}/api.ts`. Final log: `telegram-issues-lint-final.log`.

**Independent evidence, reported by root:** combined library Ops at8ff15d9d,
root-only target, **175 passed,3 manual ignored,exit0,8.34s**;
`/tmp/ops-phone-issues-independent.log`. Root reviewed migration/snapshot/live scan/
decision integration and regressions with no blocker so far. This is separate
from worker tests and does not stand in for final browser flow or acceptance.

## Isolated protected phone fixture

Fresh owned listener **4323 / PID45025**, in-memory SQLite, test auxiliary data,
static **out-telegram-issues**. Existing4320/PID30815 and its `out/` are untouched;
4318/4322/4324/4326 and other workers' outputs are not used. The former completed
PR10 listener51913 was intentionally stopped after root released4323.

Setup from this worktree:

```sh
CODEG_EXPORT_DIR=out-telegram-issues pnpm build
```

Then from `src-tauri/`:

```sh
CARGO_TARGET_DIR=target-approvals cargo test --locked --no-default-features --lib ops_telegram_issue_browser_fixture -- --ignored --nocapture
```

Open **http://127.0.0.1:4323/__issue_fixture** for three opaque links. Synthetic-only
operator token: **ops-issue-phone-synthetic-operator**. This fixture runs the actual
protected router/login and injected loopback Telegram/GitHub/host providers, no
scheduler/agent loop and no production credentials. Initial counters: GitHub
creates0, Telegram sends3. `GET /__issue_fixture/stats` exposes counts;
`POST /__issue_fixture/refresh` renews only source/snapshot synthetic freshness
(15min), without changing payload, status, claims, receipts or provider calls.
Both test controls require the synthetic bearer; neither exists in production.

The first issue fixture **PID14341** hosted the successful worker mutation pass,
then was intentionally stopped and replaced with45025. The manual command stays
running to serve root, so it is not claimed as an exited/passing unit test.
Fresh locator IDs (also listed on the fixture landing):

- Created path: **f4fbe138-fd6f-469c-be25-7946d89caafa**.
- Unknown path: **84861f57-0757-4d9b-a618-0b1442129bd7**.
- Rejected path: **5fc80d1b-2ef8-492f-841b-b90ae10938fc**.

All three remain pending and unused. The worker performed only login/read/scroll
on the fresh fixture; [fresh counts](browser-telegram-issues/fresh-handoff-counts.json)
are GitHub POST/token/issue0, Telegram3. After15min, use the protected fixture
refresh control then reload the page; proof expiry and every product decision
check remain active. To stop/reseed later, verify4323's owned PID, terminate only
that listener and rerun the command above. Never use a global kill or rebuild4320.

## Actual Playwright CLI flow

Ran **playwright-cli0.1.18**, session **ops-issue-phone-check**, on the final
integrated frontend export with the real protected API, accepted Python host
and loopback provider clients. No request/response interception, page content
replacement, live credentials, GitHub navigation or real phone connection.
CLI actions exited0. Screenshots were opened and inspected, not merely captured.

1. Mobile390×844 opaque link redirected to login while preserving only the locator.
   Wrong synthetic token gave a named invalid-token alert and no review payload.
   Correct token plus Enter returned to the exact review. [Login](browser-telegram-issues/login-rejected.yml).
2. Pending review showed source, four evidence summaries/hashes and complete exact
   repository/title/body/labels. Private tester fields were absent. Approval was
   disabled until confirmation. [ARIA](browser-telegram-issues/pending-mobile.yml),
   [light phone](browser-telegram-issues/pending-mobile-light.png),
   [dark phone](browser-telegram-issues/pending-mobile-dark.png).
   A real wheel scroll reached the end of the entire issue body, including reciter,
   log and marker: scrollTop685 + clientHeight576 = scrollHeight1261.
   [Bottom](browser-telegram-issues/full-body-bottom-mobile-light.png),
   [measured full text](browser-telegram-issues/full-body-scroll.json).
3. Confirmation survived390→1280→390. Actual theme classes followed system color
   scheme emulation (`light`/`dark`); no DOM class injection. Document widths were
   exactly390/1280 and action buttons44px high. [Desktop light](browser-telegram-issues/pending-desktop-light.png),
   [desktop dark](browser-telegram-issues/pending-desktop-dark.png),
   [mobile facts](browser-telegram-issues/mobile-dark-facts.json).
4. Tab reached **Approve and file issue**; Enter dispatched the exact proposal.
   Created receipt **#1 in owner/repo** appeared. GitHub create POST0→1, issue0→1;
   Telegram stayed3. [Keyboard](browser-telegram-issues/approve-keyboard-mobile-dark.png),
   [receipt](browser-telegram-issues/created-mobile-dark.png),
   [counts](browser-telegram-issues/created-counts.json).
5. Unknown case committed at the synthetic provider but returned503. UI showed
   unknown with no create retry. **Check existing issue** found issue#2 using the
   accepted reconciliation; creates stayed2 and Telegram3 throughout.
   [Unknown](browser-telegram-issues/unknown-mobile-light.png),
   [before](browser-telegram-issues/unknown-counts.json),
   [after](browser-telegram-issues/reconciled-counts.json),
   [reconciled receipt](browser-telegram-issues/reconciled-mobile-light.png).
6. Rejected case returned422; UI explicitly showed no confirmed issue and no
   filing retry. Final old-fixture counters: create POST3, actual issues2,
   installation-token requests1, Telegram3. [Rejection](browser-telegram-issues/rejected-mobile-light.png),
   [counts](browser-telegram-issues/rejected-counts.json).
7. Used-link reload and malformed locator exposed no payload or decision action;
   provider counts stayed fixed. **Open workspace** navigated to existing `/workspace`.
   [Used link](browser-telegram-issues/used-link-mobile-dark.png),
   [invalid link](browser-telegram-issues/invalid-link-mobile-light.png),
   [final counts/state](browser-telegram-issues/final-used-fixture.json),
   [workspace destination](browser-telegram-issues/workspace-destination.yml).

This browser evidence is distinct from mocked component tests and root's175-test
backend result. Denial, expired proofs, wrong ownership and loaded racing decisions
are covered by the real Rust/provider suite; no separate browser denial/expiry
scenario is claimed. Console401 is the intentional wrong-token test. Navigating
to the synthetic workspace also produced404s for nonexistent fixture folder Git
HEAD/state-stream reads; that checks destination only, not a functioning source
checkout. No Ops-review request failed unexpectedly. Root final Design Studio
loops and native/mobile-webview acceptance remain separate.

## Limits

This is a local beta/configured-host extension, not a public Telegram deployment,
real phone/network certificate, successful live App installation or paid-model
run. Remote phone review requires a configured reachable protected origin and
operator authentication; loopback links validate local browsing only. Ordinary
same-user full-filesystem agents are not an OS security sandbox. No malicious
same-user-process isolation is claimed. P1 Pi discovery/proposal follow-on belongs
to its worker; notification scanning never fabricates agent/human authority.

Issue evidence must remain fresh and valid; absent credentials/configuration or
stale/rebound data fail closed. Unknown delivery/create outcomes do not authorize
a new attempt. A used/stale locator stops resolving a pending payload; the normal
workspace remains the place to inspect later history. Final cross-product Design
Studio loops, native bundle and any unrelated locale/RTL/copy correction remain
root-coordinated follow-ons. No final design acceptance is asserted here.
