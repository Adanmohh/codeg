# Pi P1 bridge — completed handoff

Sole writer on `feat/step3-pi-issues`. Draft [PR15](https://github.com/Adanmohh/codeg/pull/15) targets main.
**Validated integrated product head: `cd2a29fbdbadb3f316e6b6fb74fb2a036732a025`.**
Final handoff adds this report, evidence and an attribution-path clarification only.
No merge, deployment, live inference, App install or live provider action.

## Integration and delivered contract

Contract `70ed832a` and implementation `b15f11a7` were pushed early.
Accepted PR12 (`e9ddab88dfc45393aab68de52db8d3848b0bb891`) was integrated as
`ae131cf8`; expanded fixtures followed at `0d18f665`/`2efbf1e2`.
After their gates, accepted PR14 (`b7186ba6`) and main docs through
`d5b5b53a4d5f550998dd4ce6ac026cf6346ff74e` were merged as `cd2a29fb` and pushed.
Both merges were conflict-free. All main NOTICE sections, `notice.rs`, additive
host registrations and root planning docs are preserved. No unaccepted source copied.

| Tool | Closed input | Result |
| --- | --- | --- |
| `hafidh_intake_status` | `{}` | Bound product, cached count, unavailable in-app reads and operator guidance |
| `hafidh_feedback_list` | `{page?: 0..10000}` | Ten cached items/page; bounded continuation, freshness and existing public draft metadata |
| `hafidh_feedback_get` | `{ulid}` | Valid TestFlight ULID; cached public detail and draft metadata |
| `desk_propose_issue` | `{draftId, expectedRevision}` | Identifier string and positive i32 revision; accepted host validation and pending proposal metadata |

Existing token/listener → private task/run/ancestry mapping supplies live task,
root connection, configured account and stable actual agent identity (`pi`).
Host resolves exactly one enabled product bound to that folder/account in the
same transaction as its reused live-run check. Missing/ambiguous bindings return
`product_missing`/`ambiguous_product`; callers cannot supply identity or scope.

Reads never call a provider, create/reset drafts, attach evidence or mint freshness.
Human import caches data and invalidates verification; successful human refresh
alone grants upstream freshness. Results explicitly label missing, unverified,
unavailable, expired or fresh cache, with timestamps, the accepted900-second
window and operator action. Public drafts expose ID/revision/title/labels/severity,
source-revision match and preparation's task/run match. That match is contextual
metadata, not evidence validity. Proofs, candidates, private reporter refs,
screenshots, credentials/config refs, binding digests, other tasks and the
rendered evidence-bearing issue body stay out of agent output.

Native proposal delegates to accepted `agent::prepare_and_propose` →
`review::propose`. Host owns source/evidence/severity, revision/CAS, binding,
live-task and gating checks; ACP copies no validators/CAS SQL. Cancellation is
rechecked in the final writer transaction. Default missing scope stays propose,
stable Pi deny wins across launch UUIDs, and allow/`act_low_risk` cannot remove
the destructive human floor. Output is only pending proposal ID, draft ID,
submitted revision, title and labels; no approval/execution capability.

Trusted per-launch values configure one fixed `codeg-mcp --features intake`
server and exactly three direct reads. Ambient MCP config, arbitrary commands/
URLs, resources, sampling, elicitation and automatic auth are disabled. Broker
claims synchronously, revalidates across await, freezes arguments and grants once.
Mutations stay native. Token stripping/revocation and Astra/max prompt/compaction
guards remain. Hafidh credentials remain in the host's existing secret store and
isolated Python process; no new provider secret enters agent runtime configuration.

Only validator visibility changes: crate-visible `ops::agent::require_live` and
`ops_intake::SourceRef::validate`. No migration, new task engine, Ops UI,
dispatcher, policy default or host notice changes.

## Immutable source-to-port / license mapping

Installed source/types first; all remote research via `gh api` at immutable refs.
Both requested tags resolve directly to these commits.

| Source | Exact files and use |
| --- | --- |
| Pi v0.85.1 `d981de1229ef899957bbe968bc8dcda02a21f477` | `packages/coding-agent/examples/extensions/confirm-destructive.ts`; `packages/coding-agent/src/core/extensions/{types,runner,loader}.ts`; `packages/coding-agent/docs/rpc.md`; `packages/coding-agent/src/modes/rpc/rpc-mode.ts`; `LICENSE`. Typed registration, mutable tool-call/revalidation, lifecycle and RPC patterns extended in Pi index/protocol/tests and Rust extracted-assets fixture. MIT Mario Zechner2025. |
| Adapter v2.32.1 `10a45367e033a32026987a75d6f401e37340c86f` | `types.ts`, `index.ts`, `config.ts`, `server-manager.ts`, `init.ts`, `metadata-cache.ts`, `tool-approval.ts`, `__tests__/tool-approval.test.ts`, `LICENSE`. Fixed programmatic configuration and claim/one-use/freeze test patterns in index/broker/tests. Eight installed runtime/source/LICENSE files match immutable GitHub blob hashes. MIT Nico Bailon2026. |
| Codeg v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed` | `src-tauri/src/acp/delegation/{companion,listener,transport}.rs`, `src-tauri/src/bin/codeg_mcp.rs`, `LICENSE`. Feature filtering, framing/token and stdio process patterns reused. Apache-2.0 retained. |
| IntroMail `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | `backend/app/services/agent/identity.py`, read through gh API: persistent policy principal retained as stable `pi`; no UUID policy key or gating-order/default change. Owner-owned source; existing attribution retained. |
| Accepted host PR9 `8703e00fae2e1c92e045936b140b83012cfe0f57` | `src-tauri/src/ops_intake_host/{agent,store,review,types,tests}.rs`, `tests/{fixture,browser}.rs`: public projection over existing store, accepted validation and synthetic provider/human evidence helpers. PR12 `71048a5a6f603476fcf29ef3440fa46f48421e20` supplies accepted test-only fixture visibility. |
| Accepted Pi PR8 `44e30733e6a83f9b7e73f65a8487cf1a87075308` | `src-tauri/src/acp/pi_desk.rs`, `src-tauri/src/work_task/desk.rs`, `work_task/desk/tests.rs`, `reports/pi-desk-evidence/guard.js`: launch/live bridge/process/browser glue. Accepted Ops UI `756d064f1cc391ed1da32ba90429adef225f080d` supplies `ops/{agent,review}.rs` transaction contracts. |

Complete existing MIT notices and extracted LICENSE remain; root Apache LICENSE
unchanged. No AGPL, enterprise or PolyForm source. New code is integration glue.

## Docs-first / coverage

Read complete FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS, P1 checklist and
accepted host/Pi source/contracts. Initial separate reads were React package.json
and Cargo.toml. Applied code-context and Playwright CLI skills. Existing rag-skills
Python with `HF_HUB_OFFLINE=1`: guide exit0; dependency docs exit3 because
`data/code/tickets.db` is absent. No corpus coverage invented. Rules applied:
source reuse, strict TS and actual CLI; owner no-agent/gh-api-only rules prevailed.

Read pinned SeaORM1.1.19 transaction traits, Tokio1.49.0 process/line/cancellation,
serde1.0.228 closed structs, serde_json1.0.149, Pi0.85.1 extension/runner/RPC,
adapter2.32.1, TypeBox1.3.7, Node24.19.0/@types-node25.2.2, React19.2.4,
Next16.1.6 export source, Vitest2.1.9 and Playwright CLI0.1.18 help/installed
Playwright1.63.0-alpha-2026-08-05 types and accepted guard patterns.
Git/gh/Cargo/uv help read before use. No package/lockfile change. Own Python3.13.14
venv installed offline from accepted requirements.lock plus local editable package
with `--no-deps`; no other worktree/global configuration writes.

Live hook audit `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`
has PreToolUse/PostToolUse at1788830836 for session
`01a07c1c-d82f-7022-84db-778a438632f1`, exact tickets cwd; initial live event
1788828696. Hooks remained enabled; no bypass/approval rejection.

## Validation

All following commands exited0. Rust cwd `src-tauri/`; owned targets/outputs only.
[Checkpoint gates](pi-issues-gates.json) and [final integrated gates](pi-issues-final-gates.json)
record exact commands; raw `reports/pi-issues-*.log` remain locally ignored.

| Gate | Observed result |
| --- | --- |
| Default desktop `cargo check --locked` and `cargo clippy --locked --all-targets --features test-utils -- -D warnings` | Final integrated passes |
| Server/companion locked check and Clippy, `--no-default-features --bin codeg-server --bin codeg-mcp` (Clippy also `--lib -- -D warnings`) | Final integrated passes |
| `cargo test --locked --no-default-features --lib desk_` | 19 passed,2 explicit fixtures ignored |
| `cargo test --locked --features test-utils --lib desk_` | 19 passed,2 explicit fixtures ignored |
| `cargo test --locked --no-default-features --lib ops_intake_host::` | 17 passed,1 manual fixture ignored; synthetic loopback |
| `cargo build --locked --no-default-features --bin codeg-mcp` | Final real companion restored after desktop gates |
| `pnpm exec vitest run --config integrations/pi-desk/vitest.config.ts` | Final19/19:16 unit +3 actual process cases |
| Explicit ignored `pi_desk_extracted_assets_real_rpc_discovery` Rust test | Final1/1,0.83s; extracted assets/installed adapter/real companion |
| Explicit ignored `pi_issues_browser_fixture` Rust test | Final1/1,384.54s; real bridge/CLI, graceful shutdown |
| `pnpm exec tsc --noEmit`; same with `--project integrations/pi-desk/tsconfig.json`; `pnpm exec eslint integrations/pi-desk/*.ts` | Frontend, strict Node, lint pass |
| `CODEG_EXPORT_DIR=src-tauri/target/pi-issues-export pnpm build` | Final accepted-UI export pass |
| `git diff --check` | Pass |

Rust19/17 selectors ran at `2efbf1e2`; later PR14 changes no Rust product file,
only its accepted manual UI fixture/registration. Final check/Clippy,19 process,
extracted and browser gates use `cd2a29fb`.
Six new full-path P1 tests cover cache/private-field immutability,15-record paging,
changed-source preservation, closed/foreign/missing/ambiguous inputs, human
edit/CAS/severity and revoked proof, exact pending/idempotency, stable deny across
two launch UUIDs, stale generation and cancel-writer/peer-abort races.

Extracted `prepare_at/write_assets` loads actual installed peers and discovers all
three real read tools via `/desk-status` (RPC extension command, no inference).
Cheap model/reasoning/session substitutions are rejected; abort works; empty
catalogue returns explicit setup failure. Message count0, no message-start event
or synthetic provider connection. Companion SHA256 remained
`11f812d420703af433ba4d49d3887990d3b3cf4adfab6d672d7944b5ac377d90`.

## Actual CLI evidence and limits

Owned headed `pi-issues-review`, loopback4324, isolated DB/export. Fixture human
setup uses accepted host/Python import/refresh and four reviewed synthetic proofs.
The real companion then executes all three cached reads through live token/UDS/
TaskEngine/host without changing cache bytes or upstream read count. A native
draft/revision-only request persists one exact pending proposal under default
missing scope. [Public bridge transcript](pi-issues-evidence/00-bridge.json).

Actual CLI login/navigation opened it. Human list invalidated freshness; stale
status/disabled filing verified. Human refresh restored readiness; confirmation
was still required. DOM matches native repository/title/labels and **966-character
body**, desktop1440 and mobile390; mobile body324px, page390px. Full body scrolled
to final proof, human confirmation enabled review, then **human Deny** closed it.
Filing was never invoked. Inspected [desktop](pi-issues-evidence/02-pending-desktop.png),
[mobile](pi-issues-evidence/03-pending-mobile.png),
[confirmation](pi-issues-evidence/04-confirmed-mobile.png) and
[denied](pi-issues-evidence/05-denied-mobile.png); JSON/YAML and reusable guard/measure
scripts are adjacent. Final counts:4 synthetic upstream GETs (2 setup,2 UI),
**0 GitHub tokens/creates**,1 denied proposal. Guard:0 remote,0 model/agent requests,
0 other/config writes; UI1 import/1 refresh/1 denial.
[Counts](pi-issues-evidence/05-denied-counts.json). Browser/listener/socket closed;
manual fixture returned exit0. No fresh reseed or other fixture change.

- Live Astra/provider support is not established by synthetic catalogue tests;
  missing real setup remains explicit. Accepted single-user filesystem trust
  remains, not an OS sandbox. No live credentials copied or model request.
- Offset cache pages are independent snapshots; concurrent human import can
  change contents. Existing bounded source redaction is not universal personal
  data detection; public text remains untrusted.
- Initial fixture Option inference, invalid non-SHA test revision and one needless
  reference failed compilation/test/Clippy (101); corrected, no waived gates.
- Initial mobile resize left the inherited drawer open; checkbox timed out(1).
  Labelled attempt image retained. Escape closed it; successful visible captures
  and confirmation were recaptured. No forced click/DOM-content substitution.
- Synthetic `/tmp/task-deleg` is not a Git checkout:10 repeated Git-head/workspace
  stream404 console errors are recorded. No clean-console/repository-flow claim.
- Inherited warnings: compile-only placeholder sidecar, large test unwind table,
  proc-macro-error2 future compatibility. Real companion rebuilt; no packaging claim.
- Root separately reports4/4 earlier P1 tests at `ae131cf8`, and full frontend
  **6158/6158 across433 files at `cc778461`**. These are owner-reported independent
  results; this worker did not repeat that full frontend suite.
