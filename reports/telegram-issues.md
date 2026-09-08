# Step 3 — typed GitHub issue phone review

Implementation checkpoint, 2026-09-08. Sole writer in
`/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`, new branch
**feat/step3-telegram-issues**, created clean from accepted origin/main
**77c88d9c615072a11b6e24224ec954c398b33518**. No additional agents.
Contract commit **a7e2e2f3eee3be3e34b0f7006bf8d96bc5f004bb** is pushed.
Draft PR: **https://github.com/Adanmohh/codeg/pull/12**.

**Current delivery is in progress, not final acceptance.** Product checkpoint
**8ff15d9d** is pushed. Accepted PR9 is integrated. Combined worker Ops suite:
**178 passed, 4 manual fixtures ignored, exit 0, 10.32s**. Root independently
reported **175 passed, 3 manual ignored, exit 0, 8.34s** at 8ff15d9d using its own
target (`/tmp/ops-phone-issues-independent.log`); that is separate evidence.
The isolated production export, server check, frontend typecheck and 21 focused
frontend tests passed. Desktop/Clippy and actual protected phone flows continue.

**Active fixture handoff:** 4323 now runs the owned issue fixture, **PID 14341**,
static **out-telegram-issues**, in-memory SQLite. Open
`http://127.0.0.1:4323/__issue_fixture`; its three opaque links use the real login
and protected API. Test-only operator token: **ops-issue-phone-synthetic-operator**.
The worker is currently mutating these cases; read-only inspection is safe. A
fresh fixture will be supplied for independent decisions after worker scenarios.
No scheduler or agent loop runs. Both providers are loopback only. 4320/PID30815
and its `out/` remain untouched.

Reproduce from this worktree: build with
`CODEG_EXPORT_DIR=out-telegram-issues pnpm build`, then from `src-tauri/` run
`CARGO_TARGET_DIR=target-approvals cargo test --locked --no-default-features --lib
ops_telegram_issue_browser_fixture -- --ignored --nocapture`. The protected
`GET /__issue_fixture/stats` reports GitHub POST/token/issue counts and Telegram
sends. Protected `POST /__issue_fixture/refresh` explicitly renews only synthetic
source/snapshot freshness timestamps, never claims, payloads, receipts or sends.
Both fixture controls require the synthetic bearer above. They are test code,
absent from production routes. Remote phone use still requires a configured,
reachable protected origin; these loopback links validate only local browsing.

## Implemented checkpoint and exact host contract

Preparation was pushed as **4fefd024** (three expected red migration tests).
Accepted main **61738519** was merged cleanly as
**81716c7ab952c4fa655b70c7d7cedae4b01bdc1f** after owner confirmation of PR9.
NOTICE, all planning documents and migration order 000005→000006→000007 were
preserved. No other worktree was changed.

- `ops_intake_host::notice` is the closed read-only snapshot/projection seam.
  It verifies pending `github.create_issue`, account/product/folder, source
  freshness/revision, host draft/revision/prepared payload, task/run/connection,
  all stored proof bytes/hashes/revocation and repository/App/installation.
  It returns only identifiers/hash to scanning; the authenticated resolver gets
  the host `Detail` projection. No human Operator is constructed for scanning.
- Forward migration **m20260908_000008_ops_telegram_issues** preserves every
  existing email notice/claim/receipt and adds `github_issues_enabled=0`. The
  closed action-kind column accepts only `email_reply` / `github_issue`.
  A separate `ops_telegram_issue_binding` epoch with local host-config triggers
  invalidates rebind/restore locators without altering host product columns.
  Rollback refuses any issue-notice rows; mid-DDL rollback restores original
  email rows/schema. An upgrade never mints a replacement delivery attempt.
- Existing `ops_telegram_resolve` returns `proposal` for email, `issue` for the
  typed `{source,binding,detail}` issue projection, or neither when unavailable.
  Issue projection reads occur in one local SQLite writer transaction, with no
  provider or credential call. Both families share the existing 8-second scan
  and one-second cleanup budget. Keyset progression skips stale candidates;
  at most 20 notices per account are attempted within the shared deadline.
- Existing host approve/deny inputs gain optional **`review_notice`**. The
  phone supplies the opaque notice plus the complete existing expected/approved
  payload. The host checks the locator again under the approval writer lock;
  denial uses an extracted internal `deny_in_transaction` over the unchanged
  core task/proposal CAS, redaction and wait reconciliation. The workspace keeps
  its ordinary authenticated review contract. No actor/action executor input,
  Telegram callback, operator token or mutable payload dispatch is added.
- The phone shares the accepted `IssueReviewCard`; the workspace wrapper keeps
  Tasks navigation. A phone locator does not offer draft mutation/task creation.
  Complete issue body, repository, labels and four evidence summaries/hashes
  remain visible. Its button is now accurately **Open workspace** at `/workspace`.
  No locale/RTL/global copy or shell design changes are included.

Read complete accepted host source/NOTICE, including store, review, operator,
types, runtime, process, shared review/evidence/UI/API and host fixture tests.
Verified via gh api at accepted head **8703e00fae2e1c92e045936b140b83012cfe0f57**:

| Source file | Immutable blob |
| --- | --- |
| ops_intake_host/review.rs | 2d37cc41a9df58c5598ce2e5b128a9b01a5fae81 |
| ops_intake_host/store.rs | bb016b603412ebb22a0f5ef44252b7397b9eb0e0 |
| ops_intake_host/operator.rs | d2178860743352b9cf5b37d7b5d71ff278af2065 |
| ops_intake_host/types.rs | 3a4f2514e09b05bc98263087d5aa266d97db9586 |
| ops_intake_host/runtime.rs | c7057b15ce7c21dc493cd72b6ea21017e0866b79 |
| ops_intake_host/tests.rs | 86efca2eca48227e8fe5ccbc31ebdab1114f9048 |
| ops_intake_host/tests/fixture.rs | e449ffa4677b400410e48c126647bf54e82df343 |
| src/components/ops-intake/issue-review.tsx | 595672b0b85ee659b36f1f510aeca71779963d5d |
| src/lib/ops-intake/api.ts | d43a4eb3c320d54e6d3c7e8ce9e034b454bc7805 |

Rust paths in that table are under `src-tauri/src/`. NOTICE appends the exact
source-to-glue mapping; original Apache LICENSE and prior pinned ports remain.
Live docs-first **apply_patch** PreToolUse records emitted context at
**1788826785**, **1788827184**, **1788827398**, same session/worktree, exit 0.
Read installed React **19.2.4** and `@types/react` useState/useRef/useEffect/
useSyncExternalStore, testing-library/react **16.3.2** render/rerender/query
types, SQLite **3.46.0** trigger grammar and SeaORM **1.1.19** before using them.

Checkpoint validation: initial three migration tests failed at the missing
registration (exit101), then all **16** migration/email Telegram tests passed.
The new host cases initially stopped at `AdapterMissing`, then caught a fixture
runtime-lock collision between independent test DBs. Installed the accepted
`requirements.lock` and editable host package **offline** in this worktree's
`integrations/hafidh-intake/.venv` with CPython **3.13.14**, after installed uv
help/README reads. The fixture seam now reuses its own HostRuntime, preserving
the production per-product lock. Latest `CARGO_TARGET_DIR=target-approvals cargo
test --locked --no-default-features --lib ops_telegram::tests -- --nocapture`:
**20 passed, 0 failed, exit0, 3.67s** (`telegram-issues-rust-progress.log`).
Matrix includes 18 changed bindings and loaded approve/deny rejection with
zero GitHub POSTs. Initial server check found a private re-export; visibility
was corrected to crate-only. Final server/desktop gates have not yet run.
Frontend typecheck **exit0**; prior four-file frontend run **17 passed, exit0**,
and the expanded five-file frontend run is now **21 passed, exit0**, including
two phone-card tests and the existing two session-preservation regressions.
These mocked component results are separate from the still-pending real CLI flow.

Additional focused happy-path gate `phone_decisions_use_the_core` passed
**1 test, exit0, 0.49s**: concurrent phone approvals create exactly one synthetic
GitHub issue; retry fails, denial creates none, both retain a separately pending
ACP wait and resolve no payload through the used locator. Log:
`telegram-issues-decisions-progress.log`. This uses the real loopback provider,
host and core rather than intercepted frontend responses.

PR10 was accepted at **c7a46acf30296a9e7c41a5a5c57d47afaeaa00e4**, merged as
**c3cef09a896308b2501947e5aaafa533e36d9053**. Its last report-only commit
**886e4bcfcdc323d89b87cf257d750be76edd8af1** remains pushed on the old branch;
no further product work will go there. This task adds P1 issue review only.

## Acceptance dependency and ownership

Read the full published `reports/bug-workflow.md` directly from the rebrand
worktree as coordination evidence. PR9's inspected metadata is still open/draft,
head **1e8dbc904c8fadf3ddabcb011fd7d3febbb6571f**. Its report describes the host
and review boundaries, but is not permission to copy unaccepted implementation.
**Integrate accepted origin/main containing PR9 before product wiring**, then
read the actual host types/source/tests and record that immutable merge/head.
This initial commit contains documentation only; no issue notification is live.

Owned changes will stay in Telegram modules, the phone page, focused tests and
minimal read-only host adapter/registrations needed to reuse the accepted host.
The host retains proposal approval, exact payload validation and GitHub dispatch.
Rebrand owns locale preservation, Ops RTL and internal-copy design fixes; tickets
owns shell/Pi follow-ons. No edits to those surfaces or root planning documents.

Narrow phone-page correction requested by root: its current **Open Ops workspace**
button navigates to `/workspace`, which opens the generic workspace. Relabel it
**Open workspace** using the existing route; do not add a routing/controller
system. Include that destination/label in the scoped phone CLI regression.

## Closed typed contract

1. Keep the existing opaque `/ops-review?notice=<uuid>` locator and authenticated
   `ops_telegram_resolve` boundary. Input remains only the locator. No caller actor,
   product/folder/repository override, action name, callback approval or execution
   capability enters this boundary. Login continues preserving only the locator.
2. Resolve to a closed typed email-reply or GitHub-issue projection, or unavailable.
   GitHub projection must come from the accepted host's current source/detail and
   exact prepared payload. The phone page uses the host's real issue-review UI
   and protected approve/deny/reconcile functions. It never reconstructs a body
   from mutable data for dispatch or substitutes an arbitrary action executor.
   Exact wire field/type names will be fixed after reading the accepted host.
3. Add a small host-owned read-only snapshot/projection seam if needed. A queue
   scan supplies the stored account/proposal IDs, not a fabricated human Operator.
   The seam verifies current product/account/folder, enabled repository/App/
   installation binding, source revision/freshness, all evidence hashes/content,
   exact host draft/prepared payload and live task/run. It returns identifiers and
   a deterministic complete-binding hash for notification storage. Only the
   authenticated resolver receives the full sanitized human review projection.
4. Recheck that snapshot in the writer-first claim, after recipient GET preflight,
   and on link resolution. The host's approve path must independently enforce
   its existing current-binding and exact-expected-payload checks at decision
   time. Changed source/proof/title/body/labels/repository/folder/run, revocation,
   stale/denied/canceled proposals and wrong recipient configuration cannot
   resolve or execute an issue through an old locator.
5. Keep notification contents fixed wording plus a locator. No issue title/body,
   evidence/log text, private tester identity, credentials, source access URL or
   executable capability leaves the desk in a notification. No Telegram command
   can authorize an Ops action; the human floor remains authoritative.
6. Preserve the single existing scheduler, **8s whole-scan budget + at most 1s
   cleanup**, checking leases and safely recoverable preflight failures. Attempted
   or ambiguous sends remain durable and cannot be blindly replayed. Email and
   issue candidates share that budget; no second scheduler/timer is introduced.
7. Existing enabled email configuration must not silently authorize the new
   action family. Add an explicit GitHub-issue inclusion flag defaulting **off**;
   the existing default-off master configuration and private recipient/origin
   binding still apply. Saving configuration performs no provider I/O. Existing
   email-only setups continue working until an operator opts into issue notices.

Reserve **m20260908_000008_ops_telegram_issues**, following verified local dated
names; 000006 belongs to the accepted-host dependency and 000007 is the merged
Telegram migration. 000007's current `CHECK(action_kind = 'email_reply')` requires
a forward migration, not a historical edit. Extend the closed set to
`email_reply` / `github_issue`, preserve old notice IDs/claims/status/receipts and
uniqueness, and add the default-off issue setting atomically. Migration tests must
target this migration by name and preserve accepted tables and unknown attempts.

## Grounding and source ledger

Read complete current FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS and NOTICE,
the full Telegram integration contract and PR9 coordination report. Retain the
previously applied frontend-design, design-checklist, mobile-ui and Playwright
CLI skills; no new visual system or final integrated design acceptance is claimed.

Applied code-context with the existing rag-skills `.venv/bin/python` and
**HF_HUB_OFFLINE=1**. Guide exited **0**; relevant returned rules are **Human-in-
the-loop: nothing is filed/sent without explicit human approval**, **Strict
TypeScript**, and **Each page/route maps to one job-to-be-done step**. Guidance
includes unrelated projects; it does not override this assignment. Docs query
exited **3**, missing `data/code/approvals.db`; no invented corpus coverage,
download, global ingestion or environment changes.

Separate React package/Cargo manifest reads already confirmed installed React
**19.2.4**, Next **16.1.6**, TypeScript **5.8.3**, SeaORM **1.1.19**, Tokio
**1.49.0** and reqwest **0.12.28** for the accepted implementation. Before new
primitive use, re-read its exact installed source/types. Hook stays enabled;
live records for this session **01a07c1c-d3a3-7c22-a5b6-cedce2970d8d** and this
worktree include PreToolUse **1788825040** and prior Pre/Post **1788824206**.
Source-write hook evidence will be recorded before implementation. No bypass.

Read local accepted Telegram types/entity/migration, phone component/API, and
accepted `ops_intake/{types,mod}.rs`. `PreparedIssue` already binds the exact
outgoing title/body/labels, source/task/run, evidence set and repository digest;
`dispatch` requires the consumed, non-Clone/non-Serialize `AuthorizedAction`.
This task reuses those contracts and never adds a token/action executor.

Verified accepted-source blobs via **gh api** at PR10 merge
**c3cef09a896308b2501947e5aaafa533e36d9053**:

| Exact file | Blob SHA | Planned reuse |
| --- | --- | --- |
| src-tauri/src/ops_telegram/mod.rs | e5e6eb6927b55026774b29880c5d6b96e573eae5 | Existing scoped claims, lifecycle and resolver |
| src-tauri/src/ops_telegram/types.rs | 3f9ef91a6eb57725acd1bfab350891286d5871e3 | Closed projection/configuration extension |
| src-tauri/src/ops_telegram/entity.rs | 3abc3b3d25aa12b94698059370a6062ef1c83591 | Durable binding/attempt preservation |
| src-tauri/src/db/migration/m20260908_000007_ops_telegram.rs | be6c56915d3e78e2aefab9556e328254aa82fbd1 | Forward migration over existing email rows |
| src/components/ops-telegram/review-link-page.tsx | ced19560c628f0cc92f8099ab5a06385b4486ed5 | Protected responsive review and leave guard |
| src/lib/ops-telegram/api.ts | 1dac2ccc6f1d3d1a64645bdfa98b71d6180e73c6 | Existing dual-runtime transport |
| src-tauri/src/ops_intake/mod.rs | 5e7a75f17156cb1fbe5dd83265d73a8071ddd209 | Existing approved dispatch boundary |
| src-tauri/src/ops_intake/types.rs | 3a0c3c9cd12d17afae5364574b3164c63ef6c8c1 | PreparedIssue/RepositoryBinding contracts |
| LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | Original Apache-2.0 retained |

Original borrowing remains Codeg **v0.30.4 /
6f6bd648b206412644842a98d9ffeebf57292bed** (Apache-2.0), owner-authorized
IntroInnovation/intromail **0bd24dfe284b888aa9f602fa1fd00e337ea38874** and the
already attributed accepted intake/approval ports. NOTICE was read in full;
append exact accepted host/source-to-glue mapping before product writes without
replacing any original terms. Telegram API reference remains
`tdlib/telegram-bot-api@e3e9dd8e5b3d7ab8537cd5a10dc31d5ffa8f82d1`; no client
API change or C++ port is planned. No AGPL, enterprise or PolyForm source.

## Validation and fixture plan

- New focused regressions must prove an issue notification resolves only the
  exact current host payload and rejects stale proof/source/draft, denial,
  cancellation, run/connection changes, product/folder/repository/App rebinding,
  forged account/identity, and concurrent arrival/edit. Existing email cases
  remain regression gates. Include explicit issue opt-in and upgrade preservation.
- Reuse loopback-only Telegram/GitHub providers and real host fixtures after
  acceptance. Actual Playwright CLI: locator → login → full issue/evidence review
  → confirmed created or explicit rejected/unknown states, and the corrected
  Open workspace label/destination. Distinguish backend fixture flows from mocked
  component tests; never claim intercepted approve JSON as full E2E.
- Locked desktop/server checks, both Clippy gates, focused Telegram/host/approval
  regressions, frontend typecheck and isolated production build after code changes.
  No broad tests have been run for this documentation-only checkpoint.

Root closed **ops-phone-independent** after its successful email/BC14 pass and
authorized ownership/reseeding of **4323** for this task. Existing **PID 51913**
is still the completed email fixture until replaced intentionally. Use a fresh
owned DB and separate static export for issue validation. **4320 / PID 30815 and
its `out/` remain untouched**; do not touch 4318/4322 or other worker outputs.
Remote phone access still requires an explicitly configured reachable protected
origin; loopback is synthetic browser validation only. No live bot, issue, App
installation, paid inference, deployment or messages to people.

## Progress and next step

Owner accepted the complete contract, then explicitly confirmed PR9 accepted at
**8703e00fae2e1c92e045936b140b83012cfe0f57**, merged as
**02e3f5d8a15a3fee792cd0625966dee6c500d184**. Checkpoint this preparation before
integrating accepted main. The earlier unmerged metadata below is historical.

Added three focused, named-forward-migration regressions covering preservation
of sent/unknown/checking/preflight_failed email notices, failure after table
replacement, a closed action-kind constraint and refusal of lossy rollback.
The production migration is intentionally not implemented in this checkpoint;
the worker's own `target-approvals` red run completed: `cargo test --locked
--no-default-features --lib ops_telegram::tests::migration -- --nocapture`,
exit **101**, all three tests fail specifically because the named migration is
not registered. Output: `reports/telegram-issues-migration-red.log`.
This establishes the missing implementation; no passing result is claimed.
Appended the exact accepted PR10 and original Codeg active-enum attribution to
NOTICE before these source writes. No accepted historical migration was edited.

Installed-source grounding also includes SeaORM active-enum derivation,
SchemaManager column/index queries, serde **1.0.228** defaulted fields, and
libsqlite3-sys **0.30.1** bundled SQLite **3.46.0** ALTER TABLE/DROP COLUMN.
Live hook records for this same session/worktree include PreToolUse and
PostToolUse **1788826240**, both exit 0; the docs-first source reminder was
emitted during the pinned-source reads before the test write. No bypass.

Next: complete the red test observation, commit/push this resumable preparation,
merge the now-authorized accepted main, read the actual host source and NOTICE,
then implement the forward migration and complete typed host phone flow.

Clean-status check, `git fetch origin`, `git switch -c feat/step3-telegram-issues
origin/main`, full planning/NOTICE/source reads and immutable gh api metadata
checks exited **0**. Guide/docs query results are above. Initial branch base is
fixed at **77c88d9c615072a11b6e24224ec954c398b33518**. Commit/push and PR creation
exited **0**, draft PR12 created after reading installed `gh pr create --help`.
PR9's later report checkpoint is **7a3e88d1510134b0010df22a41b617b22bd3a224**,
still unmerged at the latest metadata check. Product wiring remains gated by the
owner's explicit accepted-host requirement. No product behavior is claimed yet.

Read the full accepted Telegram claim/live/dispatch/scan/resolve implementation
and its named-migration/authentication tests. Read installed SeaORM **1.1.19**
`database/transaction.rs` begin/commit/rollback/drop and sea-orm-migration
**1.1.19** `migrator.rs` up/down/SQLite dispatch. SQLite migrations do not receive
an automatic transaction wrapper there, so the forward migration must explicitly
own its DDL transaction. Test upgrade from real email sent/unknown/checking rows,
injected mid-DDL failure with all original rows intact, strict two-kind rejection
and explicit issue opt-in. Preserve claim IDs when upgrading; no retry is minted
by a schema change. These are planned regressions, not executed test claims.

Latest live hook observations for this session/worktree include PreToolUse
**1788825480**, PostToolUse **1788825422**, exit 0. This remains a report-only
checkpoint while PR9 acceptance is pending. After acceptance: fetch/merge accepted
main, verify full host source/attribution, add NOTICE mapping, implement the closed
projection and forward migration, then run the reported gates and isolated CLI
fixture. No extra authorization is needed beyond that already specified sequence.
