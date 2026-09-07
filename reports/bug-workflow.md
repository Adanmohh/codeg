# P1 bug workflow — final host/UI handoff

Implemented and locally validated within this assignment. **Draft PR [#9](https://github.com/Adanmohh/codeg/pull/9)** targets `main`; the worker has not merged it.

- Reviewed host/UI behavior head: **`1e8dbc904c8fadf3ddabcb011fd7d3febbb6571f`**, committed and pushed. Report/browser evidence and the non-executable Tokio version correction were committed and pushed first at **`7a3e88d1510134b0010df22a41b617b22bd3a224`**, before the requested main integration.
- Rust behavior checkpoint: **`4fd9e3a1457cf46db834aef98d4669f23a8b6f5b`**. This contains root-reviewed `fd7bcdce92716c8eb2874b6c7acac51b3ca708f1` plus the reviewed pre-list freshness invalidation. No Rust behavior changed during final review; only `process.rs`'s first documentation line now correctly names locked Tokio 1.49.0.
- Final integration target is accepted main **`026fedb1d139ac729499def31b36ce069c69de6c`**, containing Pi PR8 and Telegram PR10 merge **`c3cef09a896308b2501947e5aaafa533e36d9053`**. The two commits after the owner's announced `77c88d9c` change only root documentation. The five registration conflicts are resolved additively; post-integration gates are running. The earlier PR7 integration remains `506dcc305bb11999eba495634088f0d394d34bc5`.
- Sole worktree `/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand`, branch `feat/step2-bug-workflow`. No other worktree or Hafidh/RAG environment writes, new agents, credential disclosure, live provider mutations, messages, deployment or paid inference.

This delivers the operator intake/evidence/issue/held-task portion of P1. The real pi tool registration and its three cached read helpers remain the explicitly assigned **pi-owned follow-on after PR9 acceptance**. Live external configuration, build-note/tester-email bridges and root's final integrated Design Studio loop remain separate gates; no complete live P1 claim is made.

## Delivered behavior

**Bug intake is reachable in the existing Desk navigation** on desktop and mobile. It reuses PR7's authenticated operator boundary and memory-only Ops session state. Product selection, source selection, ordinary draft edits and unsaved evidence content survive responsive shell remounts. Secret entry is write-only and is intentionally not persisted as UI draft state.

Product settings store the authenticated account's product, existing project folder, enabled repository/App/installation binding and Hafidh origin. Dedicated random `ops-intake-<uuid>` references use the existing credential store. Projection returns presence booleans, never stored references or secret values. Configuration changes invalidate source freshness and reset the cached process/client configuration. No Git PAT slot is selected or replaced; runtime issue filing never uses `gh auth`.

The owned Rust process bridge launches the isolated Python package as `python -I -m hafidh_intake.host`. It delegates to the **accepted** `IntakeClient`; there is no second triage implementation or invented Hafidh route. Only existing TestFlight GET list and GET sync/status contracts are used. In-app feedback remains unavailable. Private identity fields, raw attachment data and signed URLs are omitted from the strict normalized projection; arbitrary adapter messages never reach the operator API.

A list read clears the product's previous freshness **before** awaiting upstream access. A refresh clears the selected record's old freshness before reading. Only successful upstream revalidation records the revision and verified time. Failed/canceled reads cannot reuse an earlier success. The freshness gate is 900 seconds. A changed source revision clears proofs, severity confirmation and preparation; old unsaved edits remain available for comparison. A stale pending proposal retains its exact old payload and can be denied, while filing stays disabled.

Humans attach **all four build/screen/reciter/log proofs by content**, through the authenticated UI. Each is bounded sanitized UTF-8 up to 8 KiB, with a matching summary, content hash and explicit provenance. Optional capture time/session ULID is accepted only when actually known. The build is checked against the reported TestFlight build when present. No agent can mint evidence, import freshness, choose a server path/URL, configure a product or act as the human. Proof editing is locked while a proposal is pending or a filing is unknown/created.

The source panel shows actual device, OS, build, platform and submission metadata. Missing app version is shown as **Not provided**; session metadata is shown only when supplied with real proof. Severity/tag suggestions remain labelled triage guesses. The operator confirms severity and saves title, reproduction/expected behavior and repository labels before preparation. Review presents the **exact repository, title, complete body and labels**, followed by an explicit human confirmation and **Approve and file issue**.

The host wraps accepted `GithubIssueAction` and the accepted approvals API, including the always-human destructive floor. Its resource hook validates current bound source/evidence before an ask rule can queue the action. Approval revalidates the exact current draft/binding/evidence and passes the frozen approved payload to the accepted owned dispatcher. A durable host handoff records unknown before consumption; ambiguous outcomes never expose blind retry. Provider `created`, `failed`, `unknown` and human `denied` states remain distinct. Unknown permits only read reconciliation when a filing receipt exists.

A created receipt can create a **real linked fix-plan task**, atomically in existing **Canceled** state with run sequence zero. The existing scheduler cannot claim it until manual Requeue. Its first deliverable is a report/fix plan; writeback is false. **Review in Tasks** refreshes real task data, includes canceled/archived rows, clears the list status filter and opens the existing Tasks list. Browser review verified task #2 and its detail/Requeue control without starting an engine or agent. No release engine was added.

## Interfaces and ownership

Owned modules are `src-tauri/src/ops_intake_host/`, separate `commands/ops_intake.rs` and `web/handlers/ops_intake.rs`, `src/components/ops-intake/`, and `src/lib/ops-intake/`. Shared edits are minimal command/router/migration/navigation registrations. No `ops/`, email transport or approvals-engine product file is changed relative to the accepted integration base.

Migration **`m20260908_000006_ops_intake_host`** adds five local tables: account/product configuration references, normalized snapshots, revisioned drafts, durable handoffs and fix-task links. Existing evidence/filing tables from 000005 and the accepted approval tables remain authoritative. All other migration registrations and NOTICE entries are preserved. Cargo/package manifests, both project lockfiles and Apache LICENSE match the accepted integration base.

The final main integration preserves every command/module/router entry from both parents. All three Ops HTTP routers remain inside the existing token middleware. The 51-entry migration vector ends **000005 intake → 000006 host → 000007 Telegram**, with no duplicates. [Preservation evidence](bug-host-registration-preservation.json) records both immutable parents: NOTICE text from both is retained in order, existing licenses match main, reviewed host/Python/P1 UI bytes are unchanged, and accepted Pi, Telegram, shared Ops, channel, keyring and authentication guards match main. No new behavior or source port was introduced while resolving these registrations.

The existing transport maps the following command suffixes to Tauri `ops_intake_<suffix>` and protected local HTTP `POST /api/ops_intake_<suffix>`. HTTP accepts only `{input: ...}`, rejects unknown fields with fixed errors and has a 128 KiB body limit. Account and actor derive from `AuthenticatedOperator` / `Operator::server()` or `Operator::desktop()`; canonical human labels are `operator:http` and `operator:desktop`, never JSON inputs.

| Command | Input / result | Boundary |
| --- | --- | --- |
| `status` | `{}` → products, existing folders, adapter availability | Nonsecret operator projection; in-app false |
| `configure` | binding, origin, optional write-only bearer/private key → status | Stored account scope; existing credential client; no App registration/install |
| `list` | product ID, optional process cursor → normalized records/cursor | Upstream GET; revokes old freshness; max 100/page, five-page adapter window |
| `detail` | product ID + source ULID → snapshot/draft/tasks/proposals/receipt/fix link | Local operator read; never grants freshness |
| `refresh` | same source → detail | Successful authorized adapter revalidation alone grants freshness |
| `save` | source, expected revision, title/summary/labels/confirmed severity → draft | CAS; bounded outward text; no raw private fields |
| `attach` | source, expected revision, field/value/content, optional capture/session → draft | Human content-only proof; no arbitrary path/URL or JSON provenance |
| `prepare` | source, expected revision, selected task ID → draft | Backend resolves live run/folder/product; all four proofs and human severity required |
| `approve` | source, proposal ID, expected payload, complete approved payload → detail | Exact stale/tamper recheck, accepted human gate and owned dispatch |
| `deny` | source, proposal ID, expected payload → detail | Accepted gate; stale original payload remains deniable |
| `reconcile` | source → detail | Accepted read-only issue reconciliation; never sends again |
| `fix` | source → detail | Current created receipt and live binding required; held task or explicit conflict |

Implemented pi seam:

```rust
ops_intake_host::agent::prepare_and_propose(
    db: &DatabaseConnection,
    ctx: &ops::agent::RunContext,
    draft_id: &str,
    expected_revision: i32,
) -> Result<Proposed, HostError>
// Proposed { status, proposal_id, prepared }
```

The trusted parent supplies account/task/run/agent/connection context. The host derives product/folder/repository from stored bindings and loads existing human proof. Repeated calls return an identical matching pending proposal without another draft revision. `running` and `awaiting_input` are accepted for that live connection; canceled/deleted tasks, changed runs/connections and unrelated waits fail. An overlapping ACP permission wait remains owned by ACP when the Ops proposal is denied.

**Not implemented in PR9:** public cached agent list/get/status helpers, companion registration, or native `desk_propose_issue`. The three existing Python MCP tools (`hafidh_feedback_list`, `hafidh_feedback_get`, `hafidh_intake_status`) are not a claim that the pi host cache bridge exists. The agreed follow-on exposes scoped cached reads only; no agent refresh/import/configuration/approval/dispatch/evidence capability. Root explicitly assigns this to tickets after host acceptance, with its checklist on main `04635c3b`. Do not delay or broaden PR9 for it.

## Review findings closed

- Global source dedup cannot link a task from another folder/product/account. Same-folder authorized reuse still works. Existing saved links are rechecked against the live task's folder and canonical GitHub source, current created receipt and immutable repository/App/installation binding. Rebinding or moved/changed source returns `fix_task_conflict: true` and **no unrelated task ID**, and creation conflicts.
- Repeated pending proposals preserve their draft and an overlapping ACP wait; cancellation, connection/run changes, revoked proof and unrelated waits reject reuse.
- Pre-list invalidation prevents old freshness surviving a failed/canceled new scan.
- Actual-host subprocess tests now use the production `-I` flag.
- Frontend refresh invalidation retains a deniable pending payload. Proof controls lock in pending/unknown/created states. Visible labels are separate from descriptive help for accessible names.
- Preliminary measured selected-row metadata contrast was 4.07:1 in light mode. Reusing the existing foreground token fixes it; final P1 measurements have zero contrast failures. No unrelated design system changes were made.

Root independently reported **17 Rust tests + 1 ignored**, **9 isolated Python host tests**, and **8 frontend tests** passing. Its real protected-route/loopback and browser review also verified missing configuration, source-list invalidation, same-revision refresh, human confirmation plus rejected filing, and actual held task discovery. Root evidence: `/tmp/ops-bug-host-independent-rust.log`, `/tmp/ops-bug-host-independent-python.log`, `/tmp/ops-bug-independent-frontend.log` and root-owned `reports/browser-bug-independent/`. These are independent review results, not worker CI claims.

## Exact source-to-port ledger

All product borrowing remains within FOUNDING §3. Original Apache LICENSE and MIT license texts in NOTICE remain. No Plane/Twenty/Postiz AGPL, Chatwoot enterprise or Kun PolyForm source was used. No dependency version was upgraded for documentation research.

| Authority / immutable revision | Exact source files | Destination / adaptation |
| --- | --- | --- |
| codeg v0.30.4, Apache-2.0, `6f6bd648b206412644842a98d9ffeebf57292bed` | `src-tauri/src/db/service/work_task_service.rs`: `create_from_forge`, `insert_todo_row`, `record_event`, source dedup; `src-tauri/src/forge/{mod.rs,envelope.rs}` | `ops_intake_host/fix_task.rs`: bounded task-row/provenance/event port, initial Canceled insertion, canonical source validation; existing task service used for discovery/manual requeue |
| Same codeg pin | `src-tauri/src/keyring_store.rs`; `src-tauri/src/web/handlers/forge.rs`; existing SQLite transaction/CAS patterns in `work_task_service.rs` | `ops_intake_host/{runtime,store,operator,review}.rs`, separate command/handler adapters and migration 000006: credential references, bounded state and boundary glue |
| Same codeg pin | `src/lib/transport/{index.ts,types.ts}`; `src/components/ui/{button.tsx,input.tsx,textarea.tsx,browser-link.tsx}`; `src/components/workbench/workbench-content.tsx` | `src/lib/ops-intake/{api,types}.ts`, `src/components/ops-intake/{ui,settings,bug-workflow-page,evidence,issue-review}.tsx`; existing primitives/transport/opener and small navigation registrations |
| Accepted intake PR5 `493df48ca92699c5818fde39d5819596712b9fdb` | `integrations/hafidh-intake/src/hafidh_intake/{client,schemas,triage,server}.py`; `src-tauri/src/ops_intake/{mod,types,store,github}.rs` | New `host.py` delegates only to accepted client; Rust host uses existing prepare/action/evidence/dispatch/receipt code. Only small crate-visible validation/binding helpers and test-only loopback client constructor added to accepted pack |
| Accepted Ops PR7 `756d064f1cc391ed1da32ba90429adef225f080d`, merge `f9ae7f1ec91fb0a9569f06fa9dddbde2884db1c6` | `src-tauri/src/web/auth.rs`, `src-tauri/src/ops/{mod,agent}.rs`; `src/components/ops/session.tsx` | Existing authenticated marker, Operator accessors, trusted RunContext and session lifetime reused unchanged |
| Accepted approvals merge `65aca88916b4af398a568c09ece431440d660e3b` | `src-tauri/src/db/service/ops_approvals/{mod,gating,redaction}.rs` | `ops_intake_host/review.rs`: existing Action/propose/review/authorized handoff; no forked approval policy |
| Hafidh read-only source `a83794708193a18611c8e6eb8e88637b8136e471`, owner-authorized | `backend/app/modules/testflight/{models.py,schemas.py,admin_router.py,triage.py}`, `backend/app/modules/feedback/{models.py,feedback_schemas.py,feedback_router.py}`, `backend/app/common/utils/operation_result.py` | Already attributed strict accepted Python DTO/GET/triage adapter reused by `host.py`; no Hafidh source import or environment change |
| intromail `0bd24dfe284b888aa9f602fa1fd00e337ea38874`, owner-authorized | `backend/app/services/github/{client.py,actions.py}`, `backend/tests/test_github_pack.py`, `docs/GITHUB-INTEGRATION.md` | Accepted GitHub App JWT/cache/action client reused; issue creation is an explicit upstream source gap filled by the reviewed narrow REST glue |
| MCP Python SDK v2.0.1, MIT, `8b191a433634d64b1306d7d51be8b16e14cc0893` | `examples/mcpserver/{readme-quickstart.py,weather_structured.py}`, `src/mcp/server/mcpserver/server.py`, `src/mcp/client/{stdio,client}.py` | Accepted `MCPServer` tool scaffold and actual stdio discovery/read tests retained; no obsolete FastMCP import |
| jsonwebtoken 9.3.1, MIT, `87bbe49004de17ac1c64bf25d7751c0e43cff5dc` | `README.md`, `Cargo.toml`, `src/encoding.rs`, LICENSE | Accepted established RS256 signer reused unchanged; no custom cryptography |

Immutable Codeg blobs read through `gh api`: `work_task_service.rs` **8042ecea7d083a6246fe3726cad2ad9b5223449d**; `keyring_store.rs` **d3e9041b95ebf2b36db66f5d15ba15df2ec494cd**; `forge/envelope.rs` **a6cd4cac6418b2148ede03836147261a863a2ad1**; `ui/button.tsx` **652d5d3d00aff9a1c917711b8d148f9f711f96fe**. `browser-link.tsx` was also read from the local exact Codeg commit before adding its reuse to NOTICE.

The accepted official issue/auth contract remains `github/docs@831337b0fed60b90a72e2711a41dfcad72b5f288` and `github/rest-api-description@3cef12e8a02d612ad032473d4fb87266f2befeae`, exact paths/operations in [the accepted intake report](intake-github.md#exact-source-mapping-implementation-ledger). REST version remains `2022-11-28`. All remote research used `gh api`, never a silently newer borrowing version.

## Docs-first evidence

Read complete current FOUNDING, ORCHESTRATOR, STATUS, DECISIONS and AGENTS, the merged intake/contract reports, `docs/design/BRIEF.html` and the stable [BC-1–18 checklist](design-acceptance-checklist.md). Applied code-context, frontend/UI, mobile, Playwright CLI and Design Studio audit/checklist skills. Owner no-worker/no-paid-inference instructions override generic specialist fan-out; no subagent was started.

Offline guide used `/Users/mohamedadan/projects/rag-skills/.venv/bin/python` with `HF_HUB_OFFLINE=1`, exit 0. Applied sibling reuse, human review and end-to-end verification guidance. Dependency corpus query exited **3** because `data/code/rebrand.db` is absent. This is missing corpus coverage, not a successful retrieval; no global ingestion/environment change was made.

Installed/local authority read: React **19.2.4** and installed React hooks/useId types; Next **16.1.6**; TypeScript **5.8.3** DOM types; Tailwind **4.1.18** and existing component/theme source; Vitest **2.1.9**, Testing Library React **16.3.2**; SeaORM **1.1.19** transactions/queries; Axum **0.8.8** extraction/JSON rejection/body limits; axum-test **17.3.0** fixture APIs; url **2.5.8**; existing transport/error and credential APIs. Audit formatter reads local Node fs types (`@types/node` **25.2.2**; installed Node **24.19.0**). Installed CLI help/types were read; Playwright CLI **0.1.18** bundles Playwright **1.63.0-alpha-2026-08-05**. Update banners were not acted on.

**Tokio grounding correction:** the earlier 1.53.1 registry-cache read/citation was not the locked dependency's authority. Root caught this documentation error. `src-tauri/Cargo.lock:7537` pins **1.49.0**, checksum `72a2903cd7736441aac9df9d7688bd0ce48edccaadf181c3b90be801e81d3d86`; all locked builds/tests already used that version. Before handoff, re-read its actual installed `Cargo.toml`, `src/process/mod.rs` (args/env/env_clear/stdio/spawn/kill_on_drop/kill), `src/time/timeout.rs`, and `src/io/util/{async_buf_read_ext,async_read_ext,async_write_ext}.rs` (read_until/take/write_all/flush), under `/Users/mohamedadan/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.49.0/`. They confirm the used APIs and cancellation semantics. `write_all` may leave partial bytes on cancellation: existing runtime code takes ownership before await, discards that child on cancellation/error, and caches only successful protocol sessions. Explicit kill is awaited; destructor-only cleanup/reaping remains Tokio's documented best effort. The earlier version grounding was a limitation, corrected by pinned source review without any dependency upgrade or behavior change. The one incorrect source documentation line is corrected too.

Separate `cat node_modules/react/package.json` and `cat src-tauri/Cargo.toml` reads succeeded. Live `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl` contains this worktree/session **`01a07c1c-cf2e-73e1-bbe3-e758c8363042`**, with final observed **PreToolUse line 6279 / PostToolUse line 6278** (earlier checkpoint 3085/3067). Hooks were neither disabled nor bypassed. These are observed session records, not a claim of universal interception outside the hook's scope.

## Commands and results

Every Rust command used `CARGO_TARGET_DIR=../.build/intake-host` from this worktree's `src-tauri/`; frontend output is this worktree's `.next/` and `out/`. Python uses only `integrations/hafidh-intake/.venv`. Logs below are local ignored build logs; committed screenshots/probes and this table preserve the handoff evidence.

| Command | Result / evidence |
| --- | --- |
| `cargo check --locked` | **Exit 0**, default desktop; `reports/bug-host-desktop-check.log` |
| `cargo check --locked --no-default-features --bin codeg-server` | **Exit 0**; `reports/bug-host-server-check.log` |
| `cargo clippy --locked --all-targets --features test-utils -- -D warnings` | **Exit 0**; `reports/bug-host-desktop-clippy.log` |
| `cargo clippy --locked --no-default-features --bin codeg-server --lib -- -D warnings` | **Exit 0**; `reports/bug-host-server-clippy.log` |
| `cargo test --locked --no-default-features --lib ops_intake_host` | **17 passed, 1 ignored manual fixture, exit 0**; `reports/bug-host-rust-full.log` |
| `cargo test --locked --no-default-features --bin codeg-server --lib ops` | **152 passed, 2 ignored, 0 failed, exit 0**; 5.86 s test execution; `reports/bug-host-integrated-ops-tests.log`. Includes email facade, approvals, intake and host, plus inherited names matching the filter; standalone ticket/transport suites below |
| `cargo test --locked --no-default-features --bin codeg-server --lib ticket_service` | **18 passed, exit 0**, 0.73 s; `reports/bug-host-ticket-regressions.log` |
| `cargo test --locked --no-default-features --bin codeg-server --lib email_transport` | **18 passed, exit 0**, 0.41 s; `reports/bug-host-email-regressions.log` |
| `PYTHONDONTWRITEBYTECODE=1 integrations/hafidh-intake/.venv/bin/python -m pytest -p no:cacheprovider integrations/hafidh-intake/tests/test_host.py -q` | **9 passed, exit 0**, actual `-I` subprocess; `reports/bug-host-python-isolated.log` |
| Initial full isolated adapter + host pytest | **23 passed, exit 0** at the Python host checkpoint, including actual MCP stdio discovery/read. After aligning subprocess flags, reran the changed 9-test host suite; accepted production MCP/client/triage remained unchanged |
| `pnpm exec vitest run src/components/ops-intake src/components/ops/session.test.tsx src/components/ops/ops-flows.test.tsx` | **15 passed, exit 0** (8 P1 + 7 existing Ops); `reports/bug-host-frontend-final.log`. Root independently reran the 8 P1 tests. Final later product change was one foreground class, checked by export/probe |
| `pnpm exec tsc --noEmit` | **Exit 0** after final product changes; `reports/bug-host-typecheck-final.log` |
| `pnpm exec eslint src/components/ops-intake src/lib/ops-intake reports/bug-probe-report.mjs` | **Exit 0, no warnings**; `reports/bug-host-eslint-final.log` |
| `pnpm exec next build` | **Exit 0**, real static export after final foreground fix; `reports/bug-host-build-final.log` |
| `playwright-cli -s=intake4322 …` | Actual protected-host/browser flows below; successful terminal/mutation checks exit 0, loopback provider only |
| `playwright-cli -s=intake4322 --raw run-code --filename=reports/bug-browser-probe.playwright` then `node reports/bug-probe-report.mjs` | **Exit 0**; four final viewport/theme probes, zero contrast/unnamed/overflow/small-target failures |
| `node …/design-studio/scripts/design-lint.mjs src/components/ops-intake/bug-workflow-page.tsx --brief docs/design/BRIEF.html` | **Exit 0**, zero deterministic state/ARIA gaps; `reports/bug-design-lint-page.json` |
| `git diff --check`; protected-doc/manifest/lock/LICENSE comparison to `f7650379` | **Exit 0**, no protected/lock/license differences |

Meaningful Rust tests cover real missing/invalid HTTP authentication, forbidden JSON actor/freshness/path/credential fields, secret projection, missing App key preserving pending, omitted/private/expired/stale proof, changed source and approval tamper, explicit read scope/deny, response-lost reconciliation and no second POST, pending/ACP ownership, foreign-folder dedup and live fix-link rebinding/receipt/source/account checks. New scans and GET failures never mint freshness. All provider requests use synthetic loopback HTTP/RSA/SQLite fixtures.

Resolved intermediate failures are retained honestly: initial Rust inference needed `Vec<String>` (exit 101); fixture policy mode/column names were corrected after reading the actual accepted schema (`read`, and `action_name/resource/behavior`), not by changing policy. Initial frontend lint found a bare external link, replaced with the inherited BrowserLink, plus fixed hook dependencies. CLI retries corrected an accessible-label selector, a pending-status polling race, and a drawer left open by desktop-to-mobile resize; Escape preserved the edit. Probe theme sampling now waits for color transitions. Prettier's leading semicolon was invalid CLI function input; standalone snippets are now `.playwright` inputs with validated function syntax. These were not claimed as passing commands before correction.

Existing build limits: default desktop check created the documented zero-byte codeg-mcp sidecar placeholder; no native packaged-app pass is claimed. Rust reports the inherited proc-macro-error2 2.0.1 future-compatibility notice; macOS libtest linking also reports its large `__eh_frame` warning. No CI result, production provider access, distribution/signing or live model success is claimed.

## Browser evidence and preliminary Design Studio check

Actual CLI session `intake4322` used **390×844** and **1280×900**, paired light/dark tokens, the real exported frontend, protected Rust routes, actual isolated Python GET process and accepted GitHub client against a loopback provider. Source records/proofs/credentials were synthetic and labelled. The browser did not substitute mocked frontend responses.

| Scenario | Committed evidence |
| --- | --- |
| Invalid token; mobile navigation | [Rejected login](bug-mobile-login-rejected.png), [drawer](bug-mobile-nav.png) |
| Missing credentials, read disabled, actionable settings | `bug-{desktop,mobile}-missing-{light,dark}.png` |
| All four mandatory proofs missing; source not yet fresh | [Missing proof gate](bug-desktop-evidence-missing-light.png), [unsaved proof survives resize](bug-mobile-proof-edit-light.png) |
| Exact body/repository/title/labels; human-confirmation floor | `bug-{desktop,mobile}-review-{light,dark}.png`, [review snapshot](bug-flow-current.yaml) |
| Access failure revokes freshness, no private upstream message | `bug-{desktop,mobile}-error-{light,dark}.png`, [error snapshot](bug-current-error.yaml) |
| Source revision changed; old payload remains visible, approval disabled; explicit denial | `bug-{desktop,mobile}-stale-{light,dark}.png`, [denial snapshot](bug-denied.yaml) |
| Successful approved synthetic issue + held fix | `bug-{desktop,mobile}-success-{light,dark}.png`, [real Tasks row](bug-tasks-discovery.yaml), [held detail](bug-held-task-detail.yaml), [mobile task](bug-mobile-held-task-detail-dark.png) |
| Response lost after provider creation → unknown, no retry → read-reconciled same issue | `bug-{desktop,mobile}-unknown-{light,dark}.png`, [unknown snapshot](bug-unknown.yaml), [reconciled issue](bug-desktop-reconciled-dark.png) |
| Explicit provider rejection, no confirmed created issue | `bug-{desktop,mobile}-rejected-{light,dark}.png` |
| Final foreground correction, responsive source view and keyboard focus | `bug-{desktop,mobile}-populated-final-{light,dark}.png`, [mobile focus](bug-mobile-keyboard-focus-dark.png), [measured probe](bug-design-probe.json) |

Applied the Design Studio aesthetic/a11y/flow review methods inline to this P1 evidence against the existing brief (schema 1, scan mode) and BC-1/2/3/11/12/16/17/18 where applicable. This is **preliminary worker evidence**, not root's final combined specialist acceptance. Used local Design Studio `55c8614dcfff33b4caa5a544b4f1f91877214878`, its deterministic linter and pure `lab/tools/probe.mjs` `buildReport` / ARIA helpers. Browser extraction ran through CLI only; no direct browser SDK launcher or paid flow-test agent was run.

Final sampled P1 surface: **65 mobile / 71 desktop text samples per theme, zero contrast failures, zero unnamed interactive nodes, no heading-order breaks, no horizontal overflow, no effective touch target under 44 px and no text input under 16 px**. Disabled fields are excluded from text-contrast grading. RGB measurements composite actual browser backgrounds, including Tailwind OKLCH/alpha colors. Keyboard focus on the product select was visible with a 44 px control. All four mandatory evidence fields and full preview remain available on mobile.

Limits for root's overall loop: inherited button micro-transitions still remain under reduced-motion emulation; no animation/jank or real iOS keyboard/notch test is claimed. The literal brief comparator flags RGB resolutions of existing OKLCH/alpha tokens, the installed Inter Variable family name and the inherited 6 px button gap; these are preserved as review data, not a claim that token conformance is automatically green. Arabic/i18n, internal IDs and terminal-state copy are reserved for the separately assigned design follow-on after acceptance. Long evidence hashes and the desktop empty space beside a deeply scrolled detail remain reviewable copy/layout tradeoffs; they were not replaced with a new UI system. No claim of complete BC-3 Arabic coverage or whole-Desk audit completion.

## Stable fixture for root

**Keep the current listener alive during root review.** URL `http://127.0.0.1:4322`; nonsecret login **`ops-intake-synthetic-operator`**. Observed owned listener PID **22775**. Current SQLite/state directory:

`/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand/.build/intake-host/browser-a0c200ac-05bf-4a7e-a6e9-ad36dfd53993`

Implementation: `src-tauri/src/ops_intake_host/tests/browser.rs`, provider/router in `tests/fixture.rs`. Launch from this worktree's `src-tauri`:

```sh
CODEG_OPS_ACCOUNT_ID=1 CARGO_TARGET_DIR=../.build/intake-host cargo test --locked --no-default-features --lib intake_host_browser_fixture -- --ignored --nocapture
```

It serves this worktree's real `out/`; build it with `pnpm exec next build`. If the isolated Python environment is missing, follow the existing `integrations/hafidh-intake/README.md` against its committed `requirements.lock`, using only that package's own `.venv`. No Hafidh/RAG environment or global configuration is involved. Set an absolute `CODEG_INTAKE_PYTHON` only for an alternative owned installed environment. The fixture's private key is the committed synthetic RSA test key in memory; it never contacts a real provider or the real credential store.

The fixture does not start the task engine or inference. A labelled synthetic parent run polls prepared local drafts and calls the **real** narrow propose seam; it never attaches proof, configures, approves or files. Failed attempts may therefore acquire another pending synthetic proposal, still requiring a fresh explicit human review. This test automation is not a claim that production pi registration is complete.

| Record suffix (prefix `01ARZ3NDEKTSV4RRFFQ69G5`) | Current fixture outcome |
| --- | --- |
| `FAV` | Synthetic successful issue **#1**, held linked task **#2**, inspectable in actual Tasks |
| `FAW` | Synthetic response-lost creation, then read-reconciled issue **#2**; no unresolved unknown remains |
| `FAX` | Earlier stale proposal denied, later explicit rejected attempt; available for an independent new rejected review |

Products: `synthetic-hafidh` configured; `synthetic-unconfigured` lacks credentials. Optional startup `OPS_INTAKE_FIXTURE_EMPTY=1` creates an empty product configuration; that optional fresh-start variant was not used for final browser evidence. Each start creates a new owned database. Do not start a second listener while 4322 is occupied, or touch 4318/4320.

Authenticated test-only `POST /_fixture/source` accepts `{mode:"denied"|"ok"|"changed"}` with the nonsecret fixture bearer above. It is absent from shipped binaries. Current reads are enabled and the source is in the **changed** revision state. The switch affects all three source records. **Read TestFlight invalidates freshness for the whole product**, so coordinate it with another review session. All worker mutations are finished and all records are released for root use. No reseed was performed for root.

Reusable CLI function inputs are [proof preparation](bug-browser-proof.playwright) and [read-only probe](bug-browser-probe.playwright). The proof function is for a selected synthetic record with no pending/created/unknown filing; it attaches four proofs, saves human-confirmed severity, prepares and reloads the real pending proposal, but does not approve it. The probe never calls a provider mutation.

## Remaining real configuration and integration gaps

- No live Hafidh origin/admin bearer, GitHub App private key/App/installation/repository IDs or authorized repository labels were provisioned or verified. Missing values remain actionable not-configured states. The upstream Hafidh bearer is an existing admin credential; the closed GET-only adapter does not reduce its upstream permissions.
- In-app feedback has no accepted read endpoint; diagnostic/bucket download APIs are absent. Human content attachment is the available evidence path. Redaction is bounded pattern matching, not a universal personal-data detector, so outward text always requires human review.
- The adapter's offset scan is bounded at five 100-record pages with process-local cursors; it is not a lossless change stream. Older records can require another overlapping scan. Only the trusted host can refresh source state.
- Production adapter status checks the configured Python executable's presence; actual import/GET execution can still fail and is reported unavailable. This is an installed internal development package, not a bundled Python runtime.
- The public pi cached read helpers and `desk_propose_issue` registration belong to the accepted next pi task. Runtime missing model/ACP configuration is not worked around. No engine/model launch was needed or attempted here.
- A held fix task must be reviewed and manually requeued; its App provenance identity is not a Git PAT account for future forge delivery. Worktree execution, reviewed diff/merge, a build, build-note draft and tester-email draft/send bridge remain separately configured/reviewed workflow steps. Creating the issue or held task performs none of them.
- Root's final combined Design Studio/locale/copy loop and integrated native bundle are pending. No new assignment is started by this report.
