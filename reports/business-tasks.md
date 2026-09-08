# Shared business tasks — implementation handoff

Draft [PR22](https://github.com/Adanmohh/codeg/pull/22), branch `feat/business-tasks`, sole writer in the tickets worktree. Product source **`1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5`** includes the R1 ownership correction. Independent reviewer resolves R1 at that exact source: report commit `a8d1dbbebf5cfef57fb489f5417f303fa8dd422f`, five ownership +six bridge +two migration/HTTP tests passed in its own archive. Full review report read; this is bounded authorization acceptance, not UI acceptance. **Implementation and required gates are complete; ready for orchestrator integration review.** Later changes are this report/evidence, attribution clarification and the extracted test's bounded readiness wait only. Fixed DTO/policy contract: [business-tasks.md](../docs/contracts/business-tasks.md).

## Delivered behavior

Business tasks are organization/domain records independent of engineering runs: title, public notes, priority, calendar-only deadline, accountable human owner, optional human/agent assignee, immutable creator and optional human reviewer. Humans create, progress, submit and complete work through permitted human review without a folder, chat or agent. Null reviewer means any currently permitted human reviewer, not an invented unavailable identity. `dueDate` is null or valid YYYY-MM-DD, years0001–9999; no instant/timezone conversion.

HTTP, native and scoped agents call the same core. HTTP uses identity's existing authenticated POST `{input}` boundary; native uses the actual operator principal. Unknown actor/org/role fields are rejected. Each write obtains the identity SQLite writer before current credential/member/domain/reference checks, revision CAS and one immutable activity event in the same transaction. Edited metadata/assignment invalidates review; stale acceptance cannot approve edited content. Agents can only read their linked task, progress among todo/in_progress/review, add notes and submit exact public deliverables. No done/cancel/review/assign/admin/provider capability is available to them.

Migration `m20260908_000010_business_tasks` registers scoped task/activity/deliverable/execution/source-authority tables and immutable history/lineage triggers in one explicit transaction. Historical engineering IDs remain soft references so retained business history cannot block existing engineering deletion. Current live row/folder/root/generation is mandatory before use. No dependency, manifest or lockfile change; no framework, runtime or task engine added.

## R1 — source-run ownership, corrected

The independent review at frozen `76bb6909511016a11e864abe19d4ffd493aafa0c` correctly identified that a live/indexed engineering run was being accepted as business ownership. Earlier passing tests did not establish this missing condition.

The fix adds prior **protected-operator source entrustment**, independently of human linking:

- `POST /api/business/tasks/entrust-execution` / `business_tasks_entrust_execution` uses the existing closed LinkExecutionInput. Actual `Principal::is_operator()` is required; an owner-role member credential remains false. Current identity/reference authorization repeats inside the writer transaction. This operation entrusts an existing local run and never launches/prompts one.
- Under the engine lifecycle mutex and DB writer, store immutable organization/task/domain/assigned business-agent UUID/exact run/root/key and the resulting task revision. A generic `pi` key alone establishes no business member identity. Entrustment has its own attributed activity/revision and delegates nothing.
- An Assign-authorized human links only that source and current exact revision. Its **own original DelegationGrant** is persisted separately; the operator source record never replaces the linker's individual credential lineage. Unbound legacy runs, wrong tasks/agents and retired indexed roots fail closed.
- Any intervening task edit invalidates pre-link source revision, including assignment/domain changes away and back, cancel/reopen and reassignment. Once linked, irreversible binding revocation fences those changes. One immutable entrustment and binding per generation prevent retargeting delayed payloads. V1 deliberately requires a new engineering generation when that source revision becomes stale.
- Only the entrusted root receives that business member identity. Even a live same-Pi delegated child cannot impersonate it. Existing non-business delegation, stable Pi policy identity, Astra/max guard and human destructive floor are unchanged.

Actual full bridge provenance: individual manager credential → authorized task assignment → separately entrusted engine CAS-minted generation/root → per-launch token parent → current private engine index → stored original human/credential grant → common transaction checks. New credentials for the same human do not revive a revoked original grant. Tests cover credential/member/domain changes, assignment, cancellation, new generations and peer abort.

Focused R1 tests: `work_task::engine::desk::tests::business::ownership::desk_business_` plus these suffixes:
`unrelated_live_run_and_owner_role_cannot_mint_source_ownership`,
`wrong_agent_and_assignment_away_back_do_not_resurrect_entrustment`,
`scope_and_cancel_reopen_away_back_invalidate_source_revision`,
`retirement_precedes_queued_link_and_child_pi_cannot_impersonate_member`,
`source_revalidated_after_database_writer_changes_assignment`.
Negative cases verify unchanged revisions/activity/deliverables/link counts. The writer race uses an independent SQLite connection; its competing assignment row is deliberately staged by test SQL and disclosed as such.

## Exact source-to-implementation mapping

All applicable original NOTICE sections and Apache/MIT licenses are preserved. No AGPL/GPL source/tests are ported.

| Immutable source | Read files → adaptation |
|---|---|
| Codeg v0.30.4, Apache-2.0, `6f6bd648b206412644842a98d9ffeebf57292bed` | `src-tauri/src/db/entities/{work_task,work_task_event}.rs`, `db/service/work_task_service.rs`, `db/migration/m20260801_000001_work_task.rs` → typed states/entities and CAS/activity/migration patterns in business_tasks; organization fields/authorization are new glue. |
| Accepted Codeg `4ec04d7282a50529335d724438d42b99a53385a2`, Apache-2.0 | `src-tauri/src/ops/{agent,types}.rs`, migration000008; `work_task/{desk,engine}.rs`, `acp/desk.rs`, `acp/delegation/{listener,companion,transport}.rs` and cancellation tests → transaction/live context, protected wrappers and existing private token/run bridge. |
| Owner-authorized IntroMail `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | `backend/app/models.py` blob `ed783c0e2520b25b9e72c2c1f7ab439333fd28fc`; `backend/app/services/task_ops.py` blob `3b1014af27a505c96633c767a9a82de6dc2d4b13`; `backend/app/services/agent/tasks_pack.py` blob `af500fbd419ece67787c84ad4e805424c48fec12` → basic title/notes/due/creator and common human-agent operation concepts only. No private-repo public MIT claim. Source create/PATCH destination-authorization omissions are corrected by current domain/reference checks. |
| IntroMail same pin, provenance boundary | Full `docs/BORROW-PLANE-OPENPROJECT.md`, blob `637881c17aaef59d98bf67e06b5304e915a3902a`, read via gh api. Favorites/relations/watchers/source-key/field-journal and uncertain copyleft-derived sections are excluded. |
| Accepted identity `861fb0ef4394d4980a19ba375bb4c1f3f218d39b` | `business_identity/{mod,store,http,types}.rs` → exact private Principal/authorize/active_reference/begin_write/delegation_grant/agent_principal_from_binding APIs. Committed registration patch `c435d1531eaf0e4496494ecd56f59761590433b2` applied (check/apply0); required extra entrust native registration is additive. No duplicate identity constructor. |
| Pi v0.85.1 `d981de1229ef899957bbe968bc8dcda02a21f477`, MIT Mario Zechner | Existing attributed examples plus actual extension types/runner → four closed Desk business tools, frozen input and post-hook validation in `integrations/pi-desk/{index,protocol}.ts`. Installed runner does not revalidate mutable hook arguments; this bridge does. |
| pi-mcp-adapter v2.32.1 `10a45367e033a32026987a75d6f401e37340c86f`, MIT Nico Bailon | Existing broker/explicit installed adapter retained; native schemas flow through the fixed companion. Hafidh remains exactly three cached read tools. No provider writes or additional MCP permissions. |
| Accepted Codeg fixture `05e1ab7043eeadab753dbb2c6c6922661fff5c42`, Apache-2.0 | `src-tauri/src/business_identity/tests/fixture.rs`, blob `9def1b639cc07a80389e18aa73d0797aaea51b17` → own guarded4342 manual task fixture. Browser scripts reuse `reports/business-identity-evidence/{browser-api,browser-second-session}.js` at accepted `d9882b68b268b516b1d1a24031862a022a797899`; no production response mocks. |

## Executed gates at the R1 product source

Commands use this worktree, its own `src-tauri/target`, and installed dependencies. Rust commands run in src-tauri unless the direct test-binary path is shown.

| Command / evidence | Observed result |
|---|---|
| `cargo test --locked --no-default-features --lib business_` | **34pass, 2 manual ignored, 0fail**, exit0; 2.05s runtime, 57.43s build. Includes13 accepted identity,10 task and11 full business bridge/ownership tests. |
| `cargo test --locked --features test-utils --lib business_` | **34pass, 2 manual ignored, 0fail**, exit0; 2.11s runtime, 2m25s build. |
| `src-tauri/target/debug/deps/codeg_lib-b542e2757758a7bb desk_` | **30pass, 2 explicit ignored, 0fail**, exit0, 2.01s. Existing email/P1/live ancestry, stable Pi deny across two launch UUIDs, cancellation/abort and new business paths. |
| `cargo check --locked` | exit0, 51.45s, default desktop. |
| `cargo check --locked --no-default-features --bin codeg-server --bin codeg-mcp` | exit0, 41.30s. |
| `cargo clippy --locked --all-targets --features test-utils -- -D warnings` | exit0, 1m35s; test-only readiness correction recheck also exit0,19.29s. |
| `cargo clippy --locked --no-default-features --bin codeg-server --bin codeg-mcp --lib -- -D warnings` | exit0, 38.27s. |
| `pnpm exec tsc --noEmit`; `pnpm exec tsc --noEmit -p integrations/pi-desk/tsconfig.json` | both exit0; root strict TS and actual Node extension TS. |
| `pnpm exec eslint integrations/pi-desk/index.ts integrations/pi-desk/protocol.ts integrations/pi-desk/desk.test.ts` | exit0, no warnings. |
| Actual Playwright CLI, [evidence and commands](business-tasks-evidence/README.md) | **55/55 assertions**, every CLI operation exit0, two independent browser contexts and real protected Rust endpoints. Human create/progress/concurrent-CAS/submit, exact manager review, shared completed state, date, visibility, spoof/entrust denial and revocation. |
| `cargo build --locked --no-default-features --bin codeg-mcp` | exit0, 1m28s. Real own companion SHA256 `36424d165643e8a7ab4d1401d7e808ce40d3f467aa1757721ddd40b901c2a08c`. |
| `pnpm exec vitest run --config integrations/pi-desk/vitest.config.ts` | **20/20 pass**, exit0, 821ms:17 unit +three actual Pi/companion process tests. |
| `cargo test --locked --no-default-features --lib pi_desk_extracted_assets_real_rpc_discovery -- --ignored --exact acp::pi_desk::tests::pi_desk_extracted_assets_real_rpc_discovery` | **1/1 pass**, exit0, 1.05s (47.04s build), after the test-only readiness correction. Real extracted assets, installed Pi/adapter and fixed companion discover all three cached reads plus four business tools. |
| `git diff --check`; lockfile/root planning-doc comparisons | exit0; no lock changes and no task diff to root planning documents. |

After the final test-only desktop Clippy check, rebuilt the real companion again (same build command, exit0,20.00s) because the inherited default-feature build replaces its own sidecar output with a placeholder. Final stable own binary SHA256 **`c1a6ba22c959c5b0ed1bae8e61980c3dd043f9e6fd74641e7626ad678ea4f7f5`**. Rechecked only affected real processes against that artifact: `pnpm exec vitest run --config integrations/pi-desk/vitest.config.ts integrations/pi-desk/process.test.ts` **3/3 pass**, exit0,708ms; direct compiled fixture `src-tauri/target/debug/deps/codeg_lib-b542e2757758a7bb acp::pi_desk::tests::pi_desk_extracted_assets_real_rpc_discovery --exact --ignored` **1/1 pass**, exit0,1.45s. No subsequent build overwrites this restored binary.

The first R1 test attempt was33pass/1fail: its queued-writer assertion shared a connection pool and could be denied at preflight instead of the intended writer boundary. Switching the test to an independent SQLite connection exposed the intended current-row Conflict and passed; no product guard was weakened. Earlier pre-R1 passes are checkpoint evidence only. Inherited linker large-unwind-table/proc-macro-error2 future-compat warnings remain. Default desktop builds write their documented own sidecar placeholder; the real companion was rebuilt after those gates before process testing. No packaged/native release artifact claim.

The extracted fixture initially failed twice (one parallel, one serial, exit101): `/desk-status` observed the four native business tools before the three cached MCP tools had registered. Installed and immutable adapter2.32.1 `index.ts:642–680` at `10a45367e033a32026987a75d6f401e37340c86f` explicitly starts eager initialization asynchronously in session_start for programmatic directTools. The correction is **test-only**: bounded polling of the actual seven names through the known extension command, preserving the zero-model-message and zero-provider-connection assertions. No production runtime or R1 code changes after `1ba73e3c`.

The corrected extracted fixture also verifies actual RPC refusal of cheap models, reduced reasoning and session replacement; abort/state remain allowed. Final model is synthetic gpt-6-astra/max, messageCount0; the loopback provider listener receives no connection. Removing the isolated synthetic catalogue produces the required setup failure. This does not claim the owner's real catalogue or credentials are configured. Processes terminate on RPC EOF and per-launch temporary assets are removed by the existing owner guards.

Browser fixture PID794 was deliberately stopped by Ctrl-C through its own PTY (tool exit1, not a passing ignored test). Named browsers4549/5528 both reported closed. `lsof -nP -iTCP:4342 -sTCP:LISTEN` then returned exit1/no output. Later PID21431 reused4342 from the rebrand worker's `.build/business-workspace/task-fixture-1ba73e3c/src-tauri`; that separately owned fixture is untouched and its owner notified. No own extracted-fixture node/pi/companion processes remained in the final sanitized process check. The test landing is not the business workspace UI; frontend composition/accessibility/dirty-state review belongs to PR21. No broad Settings snapshots, real credentials, engine startup or model/provider calls were used.

## Docs-first, integration and preservation

Read complete FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS, docs/BUSINESS-IMPLEMENTATION.md, all three research reports and consolidated research, identity/UI contracts and reports, and root's18-item acceptance checklist at `ff4f6c6c`. Applied code-context with the existing rag-skills .venv Python and HF_HUB_OFFLINE=1: guide0 (general reuse/deterministic guidance); installed-doc retrieval3 (no tickets.db coverage). Direct pinned source is authoritative; no corpus coverage was invented.

Local reads: React19.2.4; SeaORM/migration1.1.19 transactions/Statement/FromQueryResult; Axum0.8.8 and axum-core0.5.6 extractors; Serde1.0.228; Chrono0.4.43 NaiveDate; Tokio1.49.0 FIFO mutex/try_lock and cancellation; UUID1.20.0 new_v4; pi0.85.1 extension runner/types; TypeBox1.3.7 builders/check; adapter2.32.1; Playwright CLI0.1.18 and Playwright1.63.0-alpha-2026-08-05 installed types/help. No install/upgrade or global configuration changes. Remote source research used gh api at immutable refs; no web fallback.

Separate React/Cargo reads were the first resumed commands. Live hook session `01a07c1c-d82f-7022-84db-778a438632f1`; own-worktree PreToolUse1788859227/PostToolUse1788859188 both exit0, sanitized [hooks.json](business-tasks-evidence/hooks.json). Hook stayed enabled.

Initial contract `cb2e184f`, core `bf4309f5`, transport checkpoint `76bb6909`, integrated bridge `1e8b5250`, generation fencing `f83bf6c8`, R1 fix `1ba73e3c` were committed/pushed early. Accepted identity/main `d9882b68` (including PR23 merge `ab46c9d9`) integrated as `94a643ca`; both NOTICE sections and registries retained. Registration-missing limitations of the frozen early review head do not apply to the integrated checked source. No uncommitted other-worker product code copied.

Paused files preserved: `reports/visual-correspondence.md` stays untracked/unstaged, SHA256 `a2120d9cc87dbb4823b1223e02d377660fff370d846f2fd102fe1eb36599a50d`. Own colliding research report was byte-identical to main and retained under ignored `.docs/checkpoints/business-tasks-20260908/research-role-work-platform.md`, SHA256 `c874c2cc8ff52bbf90cfecbfcc7824057f668067f595fd50cf1e8598c39450ff`.

Other command results: installed CLI/source/help and git/gh operations succeeded; one unquoted gh API query hit zsh glob expansion (exit1), then exact quoted queries returned source blob SHAs exit0. Corrected missing source-path probes are not API existence claims. Browser evidence scripts were formatted after execution; their lint exits0 with three expected unused-function-expression warnings required by CLI run-code syntax.

## Limits

One authoritative server/organization; explicit org checks throughout. No offline synchronization, hosted multi-tenant or OS-isolation claim. Existing local operator/agent full-filesystem trust remains. Human task CRUD/review is usable immediately; source entrustment/linking needs an already running operator-controlled executor, and no launch UI/automatic inference is added. Current Astra credentials/catalogue are not claimed configured; existing fail-closed setup/model guard remains required.

Task list is paginated50; detail returns retained public history in full. Large-history pagination and the inherited IPC frame ceiling remain scaling limits. All task content is public only within current authorized domains; private attachments/imports, recurrence, provider actions and a new workflow engine are outside this increment. Migration000010 is a fresh unaccepted increment: no shipped business-task database upgrade from intermediate worker checkpoints is claimed.
