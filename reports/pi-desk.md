# Phase 1 piece 6 — Pi Desk email integration

**Email bridge complete; draft PR #8 ready for orchestrator review.**

- Branch: `feat/step2-pi-desk`, sole writer in
  `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`.
- Final product/test commit: **`eb6f7113543f6219d8aba1cbedac4f1cfd2e6e29`**, pushed.
  Subsequent handoff changes are report/PR documentation only.
- Rust/extension checkpoint: **`7d376edbe0e9a05116e9326499e3c2988dc55f58`**.
  The corrective commit changes frontend error conversion, tests and evidence;
  Rust runtime and Pi extension bytes remain identical to that checkpoint.
- Draft PR to `Adanmohh/codeg:main`:
  **https://github.com/Adanmohh/codeg/pull/8**. No merge or deployment.
- Accepted UI merge `f9ae7f1ec91fb0a9569f06fa9dddbde2884db1c6` is integrated;
  exact helper source `756d064f1cc391ed1da32ba90429adef225f080d`. Merge base
  with subsequently updated main is `f76503792e18445c867e565534c1f0a11f6aa786`.
- P1 cached reads and issue proposal are the agreed **follow-on after host
  acceptance**, followed by the report-only Design Studio judge. They do not
  hold this email bridge handoff. Phase 1 as a whole is not declared complete.

## Delivered contract

The actual framed companion listener resolves the existing per-launch
TokenRegistry entry to its parent connection. The task engine's existing
private `task_for_connection` generation mapping resolves the live task/root
run and validates every connected ancestor. Account, task, run and agent
identity come from trusted backend state, never operator bearer or caller
identity fields. There is no alternate task engine or approval policy.

| Native tool | Agent input | Accepted backend helper / result |
| --- | --- | --- |
| `desk_context` | `{}` | `ops::agent::context`; `{taskId,runSeq,accountId,inboxes}` |
| `desk_tickets` | `inboxId`, optional `status`, `page` | `ops::agent::tickets`; bounded public ticket DTOs |
| `desk_thread` | `inboxId`, `conversationId` | `ops::agent::thread`; public thread, reply target and versioned draft |
| `desk_save_reply` | `inboxId`, `conversationId`, `expectedRevision`, existing validated `reply` DTO | `ops::agent::save_draft`; exact saved payload and revision |
| `desk_propose_reply` | `draftId`, `expectedRevision` | `ops::review::propose_reply`; only `{proposalId,status:"pending",draftId,revision}` on success |

The reply DTO contains existing `inboxId`, `conversationId`, `from`, `to`,
`cc`, `bcc`, `subject`, `text`, `inReplyTo`, `references` fields. No request
accepts task/run/account/actor identities. Closed schemas reject extra fields;
execution revalidates inputs after mutable Pi hooks and uses a detached,
immutable JSON snapshot. Wire messages are bounded to 1 MiB with an absolute
10-second deadline and abort handling.

`ops::agent::RunContext` remains backend-only: `account_id`, `task_id`,
`run_seq`, `connection_id` (current stored root), and `agent_id` (actual token
parent's stable agent wire key). The helper performs its own live run/folder
check, and draft CAS/cancellation checks share the writer transaction. ACP
calls those helpers directly, without `Operator::server`, copied SQL or
reimplemented validators. Valid scope no longer returns an Unavailable stub.

Errors reduce to closed sanitized categories. A denied gate returns `denied`;
its accepted audit retains matched-rule evidence. An unexpected authorized
execution capability is refused and dropped, never serialized or dispatched.
No tool exposes approve/deny/execute, private notes, credentials, arbitrary
URL/file fetching or generic dispatch. Direct Resend/GitHub APIs stay inside
the trusted backend approval/dispatcher path.

The listener holds the existing token registry read lease through the bounded
operation. Peer closure drops pending work and releases the lease. Completed
revocation cannot be followed by a new mutation. The engine checks binding
under its existing request lock; Ops checks current generation again at the
transaction boundary. Tests cover stale generation, disconnected ancestry,
cancellation writers, revocation and aborted queued writes.

## Stable policy identity

Every Pi launch uses **`agent_id = "pi"`**, from the actual token parent's
`AgentType::as_wire()`. Other agents retain their existing stable wire/custom
registry keys. Neither a connection UUID suffix nor display label `Pi` is a
policy identity. This follows IntroMail's persisted system-agent identity and
exact agent equality for scopes/rules. Standing operator deny/scope rules
therefore apply across launches. Gating defaults, deny-first order and the
email action's destructive floor are unchanged.

Root connection, current task/run and checked ancestry remain separate
liveness coordinates. Existing Ops audit attribution is stable principal plus
task/run; this does not claim a new persisted full child-ancestry audit trail.
The full-path test configures `pi` deny + allow + `act_low_risk` scope, launches
two random connection UUIDs/runs, and checks that both audit rows matched the
same deny rule. Removing only that deny yields pending human review, never
execution.

## Launcher, broker and default

`pi_desk::prepare` embeds/extracts the exact extension, launcher and licenses
into a unique per-launch asset directory. `PI_ACP_PI_COMMAND` selects the
existing codeg-mcp executable as a Node launcher trampoline. Generated token
and socket values override caller plumbing, remain unsaved, and revoke on
teardown/drop/failure. Accepted operator `CODEG_TOKEN` stripping is preserved.
No project/global extension settings or credentials are copied.

The launcher verifies installed versions, explicitly loads Desk and the
adapter, checks actual RPC discovery and Astra/max state before forwarding
commands, and refuses model/reasoning downgrades, cycling and session
replacement. Abort remains permitted. Prompt and compaction hooks fail closed
if the bridge is missing. Existing Ping proves availability only; each Ops
request still requires a token-bound live task/run. This also permits ordinary
coding/pre-run compaction without prematurely granting Ops access.

The adapter broker uses the exact synchronous claim contract and only
`allow_once`/`deny`. It rechecks server/tool/input after asynchronous context
validation, binds an immutable payload, and handles abort. Its allowlist is
only `hafidh_feedback_list`, `hafidh_feedback_get`, `hafidh_intake_status` on
trusted `hafidh`. **The adapter currently loads with an explicit empty server
set.** There is no live Hafidh access claim or generic command/args environment
configuration. Future credentials belong exclusively to the trusted host
intake process.

Pi is first for a fresh unsaved frontend/work-task default. Explicit saved
folder, conversation, task and user ordering choices retain precedence. The
browser found an inherited structured-error conversion bug: the actual Pi
setup failure rendered as `[object Object]`. The final correction reuses
Codeg's existing `toErrorMessage` in the ACP context/lifecycle hook, preserving
both native string/Error and structured HTTP errors.

## Docs-first and exact source mapping

Read complete FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS,
`reports/pi-integration-contract.md`, `reports/telegram-integration-contract.md`,
and the UI/host reports read-only before their seams were used. Read accepted
Ops source before integration. Initial and resumed separate reads included
React's installed package manifest and `src-tauri/Cargo.toml`.

Live hook evidence: audit file
`/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`, session
`01a07c1c-d82f-7022-84db-778a438632f1`, exact tickets cwd. Latest verified
PreToolUse/PostToolUse entries **5537/5536**, timestamp **1788822607**; earlier
paired evidence includes 2218/2219 and 4120/4121. Hooks stayed enabled; no
bypass. These are observed session records, not a universal interception claim.

Applied code-context with existing rag-skills `.venv/bin/python` and
`HF_HUB_OFFLINE=1`. Guide exit 0, including explicit staging/source reuse and
“Verify every user-facing change end-to-end with the playwright-cli skill
(--headed).” Docs exit **3**: `data/code/tickets.db` absent. No invented corpus
coverage. Local installed source/types supplied the missing package grounding.

Read installed Pi 0.85.1 extension runner/loader/types, models/RPC docs;
adapter 2.32.1 broker/config sources; TypeBox 1.3.7 declarations; Node 24.19.0
and `@types/node` 25.2.2; React 19.2.4, Next 16.1.6, TypeScript 5.8.3,
Vitest 2.1.9, Testing Library React 16.3.2; Tokio 1.49.0, SeaORM 1.1.19,
serde 1.0.228, serde_json 1.0.149, tempfile 3.24.0 and applicable existing
fixtures. CLI help was read before use. All remote source research used
**gh api at immutable refs**, without upgrading mandated pins. Cached source
blobs were verified against Git object SHA-1 IDs in ignored
`reports/pi-desk-source.log/`.

| Borrowed source | Immutable revision and exact files | Destination / license |
| --- | --- | --- |
| `badlogic/pi-mono` v0.85.1 | `d981de1229ef899957bbe968bc8dcda02a21f477`; `packages/coding-agent/examples/extensions/{confirm-destructive,bash-spawn-hook,commands,permission-gate}.ts`; `src/core/extensions/{types,runner,loader}.ts`, `src/modes/rpc/rpc-mode.ts`, `src/core/agent-session.ts`, `docs/{extensions,models,rpc}.md` under that package; `LICENSE` | Extension lifecycle/tools/blocking, argument snapshots and launch RPC glue; MIT, Mario Zechner 2025 |
| `nicobailon/pi-mcp-adapter` v2.32.1 | `10a45367e033a32026987a75d6f401e37340c86f`; `types.ts`, `tool-approval.ts`, `__tests__/tool-approval.test.ts`, `index.ts`, `config.ts`, `README.md`, `package.json`, `LICENSE` | Broker, isolated adapter configuration and borrowed abort/claim regression patterns; MIT, Nico Bailon 2026 |
| `svkozak/pi-acp` v0.0.33 | `1bfcb394088ed879db8fd936b570bb626017f878`; `src/pi-rpc/{command,process}.ts`, `src/acp/{agent,pi-settings}.ts`, `test/unit/pi-command.test.ts`, `README.md`, `package.json`, `LICENSE` | Existing executable override/process protocol into per-launch glue; MIT, Sergii Kozak 2025 |
| `xintaofei/codeg` v0.30.4 | `6f6bd648b206412644842a98d9ffeebf57292bed`; `src-tauri/src/acp/delegation/{listener,transport,companion}.rs`, `acp/work_task_tools.rs`, `acp/connection.rs`, `work_task/engine.rs`, `bin/codeg_mcp.rs`, `models/agent.rs`; `src/lib/{resolve-default-agent,app-error,types}.ts`, `LICENSE` | Existing token/framing/task map/defaults/error formatter reused with minimal glue; Apache-2.0 |
| Owner IntroMail | `0bd24dfe284b888aa9f602fa1fd00e337ea38874`; `backend/app/services/agent/{identity,gating,proposals}.py`, `backend/app/models.py` | Stable policy identity semantics; owner-owned per FOUNDING, unchanged accepted gate |
| Accepted Ops UI | `756d064f1cc391ed1da32ba90429adef225f080d`; `src-tauri/src/ops/{agent,review,types,mod}.rs`, `tests/{agent.rs,integration.rs,integration/browser.rs}` | Real helper calls and public/CAS/browser fixture reuse; existing Apache notice |

Additional verified blobs: Pi RPC `fc8083bedc67824dd7ff1a5a154f1a08b28c4098`,
agent-session `ac2bd4b18dbe4d888e48309ad7bb63f1166179b5`; Codeg agent identity
`bc38571361cc3cea3770970f7251cd29811e0bda`, app-error
`9ddaab9e53c9c9c6971f8ad1a32938f364722457`; IntroMail identity
`297be5540f3409a9599a4f961989456e8ebeedb9`, gating
`4c7c6b88973b48d65d223fce21eb55c3707728cc`, proposals
`ee47b9e25f7f93794136e5aa2d94f4f18559cabd`, models
`ed783c0e2520b25b9e72c2c1f7ab439333fd28fc`.

Verified all three exact original MIT license texts in both NOTICE and the
extension LICENSE. Original Apache LICENSE and accepted NOTICE prefix are
preserved. No AGPL/restricted source. No dependency installs or lock changes
for this piece; exact peer versions and ignored local links reuse installed
Pi/adapter types. Accepted intake dependencies remain unchanged. Root planning
docs, Ops source and migration registry are unchanged relative to accepted
main; shared ACP registration is additive only.

## Validation and commands

All exit codes below are observed, not CI claims. Rust commands ran in this
worktree's `src-tauri`, using its own target/output.

| Worker-run command / check | Result |
| --- | --- |
| `cargo check --locked` (default desktop) | 0, integrated checkpoint 7d376edb |
| `cargo check --locked --no-default-features --bin codeg-server --bin codeg-mcp` | 0, integrated checkpoint |
| `cargo clippy --locked --all-targets --features test-utils -- -D warnings` | 0, integrated checkpoint |
| `cargo clippy --locked --no-default-features --bin codeg-server --bin codeg-mcp --lib -- -D warnings` | 0, integrated checkpoint |
| `cargo test --locked --no-default-features --lib desk_bridge` | 0; four actual listener → engine → Ops/database tests |
| Earlier `cargo test --locked --no-default-features --lib desk_` | 0; nine passed, one explicit external-client fixture ignored before four Ops tests were added |
| `cargo test --locked --no-default-features --lib pi_desk_extracted_assets_real_rpc_discovery -- --ignored --nocapture` | 0; actual extracted assets, installed Pi/adapter discovery, no model messages/provider connections; expanded model/session/abort guard fixture |
| `pnpm exec vitest run --config integrations/pi-desk/vitest.config.ts` | 0; 16 tests, including actual codeg-mcp tools/list and real installed Pi RPC discovery |
| `pnpm exec tsc --noEmit` | 0, including frontend corrective imports |
| `pnpm exec tsc --noEmit -p integrations/pi-desk/tsconfig.json` | 0 against actual pinned Node/Pi/adapter types; isolated package excluded from web project |
| `pnpm exec vitest run src/hooks/use-connection-lifecycle.send-failure.test.ts src/hooks/use-connection-lifecycle.test.ts src/lib/resolve-default-agent.test.ts src/contexts/acp-connections-context.test.tsx` | 0; **125/125**, including three no-prompt setup-error regressions; `reports/pi-desk-final-frontend-tests.log` |
| Affected frontend ESLint | 0; context, hook, new setup regressions |
| `pnpm build` | 0; rebuilt 32 static pages after error correction, `reports/pi-desk-final-build.log` |
| `cargo clippy --locked --no-default-features --features test-utils --lib --tests -- -D warnings` | 0 for added browser fixture; `reports/pi-desk-browser-clippy-with-test-utils.log` |
| `cargo test --locked --no-default-features --lib pi_desk_browser_fixture -- --ignored --nocapture` with documented isolated env | 0; manual protected-router fixture closed cleanly after actual Playwright CLI checks; `reports/pi-desk-browser.log` |
| `cargo build --locked --no-default-features --bin codeg-mcp --bin codeg-server` | 0; restored real owned companion after desktop gates |
| Exact licenses, protected-file diff, `git diff --check`, commits/pushes | 0 |

Full-path regressions cover standing Pi deny/scopes across two launch UUIDs,
stale old token/run, deny precedence/destructive floor, public-note exclusion,
foreign account/inbox, identity/header injection, draft reload/edit/CAS/exact
proposal revision, cancellation writer and peer abort/revocation before a
queued commit. Low-level/extension tests cover closed allowlists, argument
mutation, fragmentation/limits/errors, blocked/allowed/deny/stale/abort/headless
cases. No provider execution occurs.

Root independently reported at 7d376edb: **13 Rust Desk tests passed**, one
extracted fixture explicitly ignored in that selector
(`/tmp/ops-pi-independent-rust-bridge.log`); the extracted fixture separately
**1/1 passed**, zero model messages/provider connections
(`/tmp/ops-pi-independent-extracted-assets.log`); **14 Desk unit +2 actual
process tests passed**, restored process run
`/tmp/ops-pi-independent-process-restored.log`. Root also independently passed
all **8** setup/send-failure hook tests after the frontend correction. These
are attributed root results, not extra worker reruns.

The stable companion was 14,698,456 bytes, SHA-256
`58211aff505f32828595239370be3d3890ea59e862d8b0351b12c05f7d891972`, verified
again at handoff and matching root's restored process run. Desktop Tauri
checks previously copied the documented zero-byte placeholder over this own
artifact, causing ENOEXEC. Rebuilding the real companion after desktop gates
resolved it. No root/other worktree sidecar was touched.

Actual headed browser evidence and reproducible fixture commands:
[reports/pi-desk-evidence/README.md](pi-desk-evidence/README.md). Fresh default
Pi, real Astra setup error, explicit folder Codex choice for a new conversation,
and Codex preservation after reload passed. Workspace counters: **3 Pi
connect-only attempts, 0 prompt, 0 other-agent launch, 0 off-origin requests**.
Banner is one line and visually truncated; clicking opens verified
`/settings/agents?agent=pi`, not expanded guidance. Final mobile/full-guidance
assessment belongs to the later Design Studio loop. No inference or settings
mutation outside the fixture.

## Corrected attempts and remaining limits

Earlier corrected attempts: initial wrong upstream repo lookup 404; ES2022
`Object.hasOwn` under the web ES2020 target; stable wire `pi` versus display
`Pi`; explicit nested Rust test-module path after E0583; own ENOEXEC artifact
race described above. Initial browser CLI guard used unavailable VM global
`URL`, causing navigation timeout; a direct `typeof URL` check confirmed it
was undefined and a local string-prefix guard corrected the fixture. New hook
regressions initially mocked a connected state (which correctly suppresses
old errors); changing that fixture to disconnected yielded all 125 passes.
An extra Clippy `--tests` invocation omitted required `test-utils`, producing
missing test-helper errors; corrected command above passed. Formatting checks
were corrected with installed Prettier. No product checks were waived.

Baseline proc-macro-error2 2.0.1 future-compatibility and large debug linker
unwind-section warnings remain. This is not packaged desktop, CI or live
inference validation. The ordinary installed Pi catalogue has no Astra entry;
production fails explicitly until the existing client has Astra/max configured.
Actual `pi-acp` is unavailable on the resolved path. The browser uses an
availability-only version fixture to reach the real model preflight, not a
fake successful ACP session. The extracted fixture uses an isolated synthetic
Astra catalogue and loopback provider that receives no connection. It validates
the shipped extraction/loading path, not the owner's missing model credentials.
No cheaper fallback, login email, global config change or paid prompt occurred.

The bridge follows the existing same-user full-filesystem trust model; it is
not an OS sandbox. No additional agents, other-worktree writes, live sends,
GitHub App installs, comments/messages to people, or deployments occurred.

## P1 follow-on contract

After host acceptance, add native `desk_propose_issue` with only
`{draftId:string,expectedRevision:positive integer}`, calling the host-owned
`ops_intake_host::agent::prepare_and_propose(db, &ops::agent::RunContext,
draft_id, expected_revision)`. The backend supplies trusted task/run context;
the host derives product/folder/repository and loads existing human-reviewed
proof. Do not accept raw issue payload/provenance, mint evidence, import or
refresh trusted source data, choose configuration, approve or execute.

Wire the host's three bounded cached public reads through the fixed per-launch
companion and existing broker allowlist. Keep future Hafidh credentials only
in the trusted intake process. No host validator or unaccepted implementation
was copied into this PR. Owner sequencing explicitly assigns this integration
to a follow-on branch after PR #8 acceptance, then a report-only Design Studio
judge.
