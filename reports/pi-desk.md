# Phase 1 piece 6 — pi Desk extension and scoped bridge

Status: implementation in progress. Sole writer on `feat/step2-pi-desk`, created
from accepted `origin/main` at `5bfdd07872e40ec0deee078fa7c881b9303e4954` after
a clean check. No other worktree writes, agents, live provider calls, sends,
configuration changes outside this worktree, or deployments.

## Early integration contract

- Extend the existing length-prefixed companion socket protocol with a closed
  Desk request. Authentication uses the existing per-launch TokenRegistry;
  resolve its parent connection through the task engine's private
  `task_for_connection` map. Account, task, run and agent identity are derived
  by the backend, never accepted as operator bearer or caller actor strings.
- Agent operations: context, public ticket/thread read, save a versioned reply
  draft, and propose that exact draft revision. No approve, deny, execute,
  private-note access, provider credentials, arbitrary URL/file retrieval or
  generic action dispatch. Scope checks must also run at the database mutation
  boundary, including current live task generation.
- Planned UI seam (owner contract, awaiting accepted main):
  `ops::review::propose_reply(db, task_id, run_seq, agent, account_id, draft_id,
  expected_revision)`. The bridge supplies trusted identities. It needs the
  same live-run check when saving drafts; public thread projections must omit
  private notes before serialization. I will integrate accepted UI code, not
  copy its uncommitted implementation. UI worker owns `ops/*` and the dispatcher.
- Pi loads the Desk extension explicitly for each launch; ACP `mcpServers`
  cannot deliver it through pi-acp. The MCP broker claims synchronously,
  defaults to deny, and never returns `allow_for_session`. Only the three
  verified Hafidh read tools may be allowed. Resend/GitHub writes remain in
  the backend's exact-payload approval/dispatcher path.
- Draft edits require a new revision/proposal. Broker decisions cannot replace
  arguments. Pi `tool_call` mutations receive no later schema validation, so
  tool execution must validate again and bind an immutable request snapshot.
- Missing bridge/model configuration fails closed in RPC/headless mode.
  Cancellation, token revocation and stale runs cannot authorize execution.
  This remains Codeg's same-user, full-filesystem trust model, not an OS sandbox.
- Pi becomes the unsaved default while preserving explicit folder, conversation
  and user ordering choices. Astra must be configured in the current client;
  there will be no silent cheaper fallback or paid model probe.

## Docs-first evidence

Read complete current FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS,
`reports/pi-integration-contract.md`, `reports/telegram-integration-contract.md`
and the UI worker's `reports/ops-ui.md` read-only. Separate first commands read
`node_modules/react/package.json` (19.2.4) and `src-tauri/Cargo.toml`.

Live audit `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl` has
PreToolUse/PostToolUse records 2218/2219 for session
`01a07c1c-d82f-7022-84db-778a438632f1`, cwd this worktree. Hooks remain enabled.

Applied code-context using the existing rag-skills `.venv/bin/python` with
`HF_HUB_OFFLINE=1`. Guide exit 0: explicit per-task staging and source reuse.
Dependency docs exit 3: `data/code/tickets.db` is missing. No claimed corpus
coverage; installed pinned source/types are the authority. Installed pi is
`@earendil-works/pi-coding-agent@0.85.1`; adapter is `pi-mcp-adapter@2.32.1`.

## Immutable sources and source-to-port plan

All remote reads use `gh api`, after installed help/source. Tags resolved and
downloaded Git blobs verified against SHA-1 object IDs. Ignored source cache:
`reports/pi-desk-source.log/`.

| Source | Immutable revision | Files / intended reuse |
| --- | --- | --- |
| badlogic/pi-mono, MIT, Mario Zechner | v0.85.1, `d981de1229ef899957bbe968bc8dcda02a21f477` | `packages/coding-agent/examples/extensions/{confirm-destructive,bash-spawn-hook,commands,permission-gate}.ts`; `src/core/extensions/{types,runner}.ts`; `docs/{extensions,models}.md`; `LICENSE`. Lifecycle, typed tools and block contract into isolated extension glue. |
| nicobailon/pi-mcp-adapter, MIT, Nico Bailon | v2.32.1, `10a45367e033a32026987a75d6f401e37340c86f` | `tool-approval.ts`, `types.ts`, `__tests__/tool-approval.test.ts`, `index.ts`, `config.ts`, `README.md`, `package.json`, `LICENSE`. Exact synchronous-claim/abort/deny contract and regression patterns. |
| xintaofei/codeg, Apache-2.0 | v0.30.4, `6f6bd648b206412644842a98d9ffeebf57292bed` | Existing `acp/delegation/{listener,transport,companion}.rs`, `acp/work_task_tools.rs`, `work_task/engine.rs`, `acp/connection.rs`, `bin/codeg_mcp.rs`, default resolver. Verified remote source and reused token, framing, cancellation and private task map with additive glue. |

Full original MIT notices are reproduced in NOTICE and the extension LICENSE.
No AGPL or restricted source. No lockfile changes so far.

## Commands and results

- Clean status, `git fetch origin`, `git switch --no-track -c
  feat/step2-pi-desk origin/main`: exit 0.
- Required document/manifests reads, `gh api --help`, `gh pr create --help`,
  `gh pr edit --help`: exit 0.
- Exact pi/adapter tag and blob reads: exit 0, verified pins above. One initial
  incorrect Codeg repository lookup returned 404/exit 1; corrected to the
  founding repository `xintaofei/codeg`, resolving the recorded tag (exit 0).
- Focused tests and integrated desktop/server checks, typecheck and Clippy:
  not yet run for this piece. No validation success claimed.

## Current gaps and resumable next steps

Read actual pi-acp loader source and adapter configuration/fixtures; implement
the extension and scoped transport independently, then integrate the accepted
UI seam. Verify real local CLI/RPC/MCP discovery without a model request.
The safe catalogue currently has no Astra entry per the root contract;
verify installed custom-model support without printing credentials or
modifying global config. Final validation and exact commit/PR evidence follow.

Independent implementation checkpoint: the extension, TypeBox input revalidation,
detached IPC payload, synchronous adapter broker, closed companion tools and
token read lease now exist. The listener holds its lease through a bounded Ops
call; completed revocation cannot be followed by a new mutation. Peer close
cancels pending work. Draft/proposal calls never wait on a human and never send.
The Ops trait currently refuses requests until accepted UI helpers are wired.
Launch wrapper and default-agent integration are still being completed.

Verified local results (2026-09-08): `pnpm exec vitest run --config
integrations/pi-desk/vitest.config.ts` exit 0, 11 tests. Includes actual installed
pi 0.85.1 RPC `get_commands` discovering both `desk-status` and `mcp` from the
explicit Desk extension and isolated adapter (no inference or live MCP server).
`cargo test --locked --no-default-features --lib desk_` exit 0, four tests:
closed allowlist, token-to-parent identity and revocation, revocation versus
in-flight operation, peer-abort lease cleanup. Server check exit 0. A temporary
unused scope-struct warning was removed. First typecheck caught ES2022
`Object.hasOwn` under this repo's ES2020 target; replaced with the installed
ES5 `hasOwnProperty.call` primitive. Final gates remain pending.

No dependency install or lock changes. Fixture typechecking links the existing
global pi/adapter and pi's typebox 1.3.7 into this worktree's ignored package
node_modules. The isolated package records exact peer versions. API grounding:
pi 0.85.1 installed `dist/core/extensions/{types.d.ts,runner.js,loader.js}`, RPC
and models docs; adapter 2.32.1 installed sources plus exact downloaded tests;
TypeBox 1.3.7 `build/type/types/*.d.mts`, `value/check/check.d.mts`; vitest 2.1.9
`dist/{config,index}.d.ts`, local test/config patterns; Node 24.19.0 CLI help and
installed `@types/node@25.2.2` net/child_process/fs/readline/crypto declarations.
Original MIT notices are now in NOTICE and the extension LICENSE.

Independent implementation checkpoint: `c0d2542a` (pushed). Draft PR:
https://github.com/Adanmohh/codeg/pull/8

## UI helper needs (2026-09-08 coordination)

The UI owner supplies Ops helpers; ACP will not construct `Operator::server`
or duplicate Ops validation/CAS SQL. Suggested helper module `ops::agent`
(names can follow the UI module's existing conventions):

| Helper | Inputs after `db` | Required result / boundary |
| --- | --- | --- |
| context | trusted task_id, run_seq, agent, account_id | Account ID and inbox ID/name/email list; no credentials. Reject non-live run. |
| tickets | same trusted identity plus inbox_id | Public ticket summaries in that account/inbox, bounded list. Reject non-live run and foreign inbox. |
| thread | same identity plus inbox_id, conversation_id | Public messages and primary contact/reply target, existing draft revision if appropriate. Omit private notes before serialization, including content/metadata. |
| save_reply | same identity plus the UI's existing validated draft-input DTO | Save exact text/recipient/threading payload with expected revision CAS and current running/awaiting_input task + run check in the same writer transaction. No actor/task/account in the agent DTO. Return saved draft ID/revision and validated payload. |
| propose_reply | existing `ops::review::propose_reply(db, task_id, run_seq, agent, account_id, draft_id, expected_revision)` | Reuse exact committed draft revision and live-run guard, returning proposal ID/state/revision. No approval or execution capability. |

The backend obtains the configured account ID from the same trusted process
configuration as Ops, preferably a small `ops::agent::account_id()` helper or
the existing shared account getter. No operator object is needed. It resolves
task/run via `TaskEngine::task_for_connection`; `agent` is the trusted caller's
stable agent wire/registry key, not request text. Unknown/disconnected connection, revoked token,
cancelled task or newer generation rejects before public data or mutations.
Please keep helper error responses sanitized (closed category/message), and
types public enough for ACP to deserialize the allowed draft fields. Tests in
the UI helper should cover cancellation/new-generation interleaving at the
transaction boundary; this worker tests token revocation and connection mapping.

The extension's intended tools are `desk_context`, `desk_tickets`, `desk_thread`,
`desk_save_reply`, `desk_propose_reply`. Only object IDs and draft content/CAS
revision occur in their input schemas. These are backend-native tools; the
adapter broker only permits `hafidh_feedback_list`, `hafidh_feedback_get`,
`hafidh_intake_status` on its trusted `hafidh` server. Public URL or path input
is never an MCP configuration channel.

## Published helper alignment and follow-on

Read the complete updated UI report read-only on 2026-09-08. Its published
`ops::agent::RunContext` is backend-only (no Deserialize): `account_id: i32`,
`task_id: i32`, `run_seq: i32`, `connection_id: String`, `agent_id: String`.
`agent::{thread,tickets,save_draft}` take `&DatabaseConnection`, `&RunContext`
and the respective existing `ThreadInput`, `TicketsInput`, `SaveDraftInput`.
The existing `review::propose_reply` signature is unchanged.

For delegated connections, RunContext.connection_id must be the current root
work-task connection stored in the task row, while agent_id is the stable
wire/registry key of the actual token-parent agent. The backend's private generation/parent index
must resolve both, rejecting disconnected parents and stale generations.
ACP will not construct an Operator or duplicate the UI's CAS SQL/validators.
The later UI report now publishes `agent::context(db, &RunContext)` for the
public account/inbox projection, `agent::account_id()` for shared trusted
configuration and `agent::command_error(DbError)` for sanitized failures.
All requested email helper contracts are available; integration awaits their
acceptance on main. The operator context is not used by the agent bridge.

The UI report records 20 passing Ops tests, including public-note exclusion,
account/connection/run checks, competing draft CAS, and a held cancellation
writer before draft save. These are the UI owner's results, not an independent
run by this worker. Integration and independent tests follow accepted main.
Root independently observed the 11 Pi tests passing at the published extension
checkpoint; this does not establish that the still-unwired Ops bridge works.

After email bridge/default completion, the minimal GitHub follow-on is a closed
native prepare/propose operation backed by the host owner's published contract
(`ops_intake_host`, rebrand worktree `reports/bug-workflow.md`). The bridge must
derive task, run, product and folder from existing backend state and accept only
existing human-reviewed evidence references plus the narrow issue draft fields.
It must not mint evidence, import data, refresh freshness, choose configuration,
approve or execute. The host owns source/evidence validation; no duplicate
validators or copied unaccepted implementation. PR #5 is accepted at `4988f46b`;
the reported main head is `d38c690e`. Follow-on integration waits for the accepted
host seam. Any future Hafidh credential belongs only to the trusted intake
process, never the generic agent runtime environment.

## Launcher/default checkpoint

Pi assets are embedded in Rust and extracted into a unique per-launch cache
directory. `PI_ACP_PI_COMMAND` selects the existing codeg-mcp executable as a
trampoline into the verified Node/Pi launcher; no project/global extension file,
login, dependency installation or model catalogue rewrite is performed.
Generated socket/token values override caller launch plumbing, stay out of
saved configuration, and are revoked on connection teardown, dropped spawn or
driver panic. Saved agent ordering/folder/conversation/task overrides remain;
Pi is the unsaved frontend and work-task fallback.

The wrapper requires actual extension/adapter RPC discovery and Astra/max state
before forwarding queued commands. Model/reasoning cycle commands and in-process
session replacement are refused; changing those requires a new task launch.
Prompt and compaction hooks check Codeg's existing socket Ping, including in
headless RPC mode. Ping proves availability only; every Ops call still requires
a token-bound live task/run. This preserves ordinary coding, merge sessions
and pre-run compaction, where an eligible running Ops task need not exist yet.
Pi's `input` handled result and `session_before_compact` cancel result
are verified blocking contracts; exceptions in provider hooks are not used.

Additional exact gh-api-verified Pi blobs at the same pinned commit:
`src/modes/rpc/rpc-mode.ts` = `fc8083bedc67824dd7ff1a5a154f1a08b28c4098`;
`src/core/agent-session.ts` = `ac2bd4b18dbe4d888e48309ad7bb63f1166179b5`
(both under `packages/coding-agent`). Original MIT notice retained.

Observed checks, before final accepted-seam integration:

- Pi Vitest: 14/14 passed, exit 0 (previous checkpoint 11/11).
- Default resolver Vitest: 4/4 passed, exit 0; explicit folder, conversation
  inheritance and saved ordering are preserved.
- `cargo test --locked --no-default-features --lib pi_desk_`: exit 0,
  3 passed and 1 explicitly ignored external-client fixture. First test compile
  exposed a missing Option unwrap in the new folder fixture (exit 101); read
  the actual service return type, fixed, then passed.
- `cargo test --locked --no-default-features --lib
  pi_desk_extracted_assets_real_rpc_discovery -- --ignored --nocapture`: exit 0,
  1/1 passed. Runs `prepare_at`/`write_assets` into this worktree's own target,
  then the real installed Pi CLI through the extracted launch wrapper. Both
  desk-status and mcp are discovered; EOF terminates wrapper/Pi. A synthetic
  isolated local Astra catalogue permits startup metadata checks only; the
  loopback provider receives no connection. Removing that fixture catalogue
  makes preparation fail closed. This is not a configured owner Astra client.
- Typecheck after the prior fixture type corrections: exit 0. Final gates
  remain pending after live Ops integration.

The owner's actual offline installed catalogue probe found no available Astra
entry. Production launches therefore show a setup error until the existing
Pi client is configured with Astra/max; no fallback or paid probe was attempted.
Live hook evidence refreshed: PreToolUse/PostToolUse lines 3406/3405, same session
and this worktree. No lockfile changes.

Read the host owner's initial bug-workflow report: proposed
`ops_intake_host::agent::prepare_and_propose` accepts backend run context and a
revisioned stored bug draft ID, loads only human-attached proof, and returns
pending/denied review metadata. That is the intended future native seam; no raw
IssueDraftV1/evidence provenance or credential input is needed in the agent tool.

## Stable policy identity and current bridge work

Owner review identified that `agent_type:connection_id` would miss standing
rules whenever a launch UUID changes. That uncommitted choice is replaced by
`AgentType::as_wire()` from the trusted token-parent SessionState: `pi`, `codex`,
or `custom:<registry-id>`. Display labels are unsuitable too (`Pi` differs from
the persisted wire key `pi`). Every Pi launch therefore uses policy `agent_id`
**`pi`**. The task/run/root connection and validated delegation ancestry remain
separate backend liveness coordinates; no caller can choose either identity.
The approved Ops audit stays attributed to the stable principal plus task/run.
No per-connection suffix, newly minted principal, gate default or permissions
ordering change is introduced.

Grounding, all immutable gh-api reads after accepted local sources:

- IntroMail `0bd24dfe284b888aa9f602fa1fd00e337ea38874`,
  `backend/app/services/agent/identity.py` blob
  `297be5540f3409a9599a4f961989456e8ebeedb9`: ensure/reuse the persisted system
  agent User. `models.py` blob `ed783c0e2520b25b9e72c2c1f7ab439333fd28fc`
  binds AgentRule/AgentScope to that stable user ID.
- Same revision `gating.py` blob `4c7c6b88973b48d65d223fce21eb55c3707728cc`
  and `proposals.py` blob `ee47b9e25f7f93794136e5aa2d94f4f18559cabd`: exact
  agent equality selects scopes/rules and proposal audit attribution. The
  accepted Rust gate preserves that equality and deny-first order unchanged.
- Codeg v0.30.4 `src-tauri/src/models/agent.rs` blob
  `bc38571361cc3cea3770970f7251cd29811e0bda`: `as_wire()` is the existing stable
  storage/registry identity; this bridge introduces no second agent registry.

Backend scope glue now verifies the private task/run mapping, the actual stored
root and every connected ancestor. Unknown, disconnected, cancelled or old-run
bindings reject; a pending accepted-Ops integration still returns unavailable
for otherwise valid scopes. The final integration MUST test a persisted deny
for `pi` through the real bridge across two launch UUIDs. That test has not yet
run, because the UI seam is not accepted on main; no successful live Ops wiring
is claimed here.

Latest independent checks: 16 Pi/IPC tests passed, including real codeg-mcp
tools/list (exactly five Desk tools), unknown approval-tool denial, token-only
wire identity, stale response and cancellation closing the parked socket.
Own `cargo build --locked --no-default-features --bin codeg-mcp` exited 0.
The first scope regression exposed the display/wire label mismatch (8 passed,
1 failed, 1 explicit external-client fixture ignored); stable identity fix and
rerun follow. A new Node fixture needed Buffer.from on its declared string-or-
Buffer socket chunk; corrected after typecheck exit 2.

The extension now has its own strict TypeScript project. Root web typechecking
excludes this isolated Node package, so normal CI does not require untracked
global Pi peers. Extension typechecking is a separate explicit final gate with
the actual pinned peer types. No ambient type stubs, dependency/lock upgrades
or global changes are used. Launch/default checkpoint `87b3810d` is pushed.
The corrected scope suite now passes: `cargo test --locked --no-default-features
--lib desk_`, exit 0, 9 passed and 1 explicit external-client fixture ignored.
Root and extension typechecks and focused ESLint all exit 0.

Read the full latest UI report through its responsive-state correction. Its
context/account/error helpers resolve the previous missing API requests.
Published source remains unaccepted at this read; no Ops helper was copied.

## Host read facade coordination

The adapter currently loads with an empty explicit server configuration. Its
three-tool allowlist/broker is tested, but no live Hafidh MCP access is claimed.
Removed the unused generic `CODEG_DESK_HAFIDH_COMMAND/ARGS` configuration path:
future intake credentials must be resolved only inside the trusted host.
The intended per-launch server is the existing codeg-mcp companion exposing
exactly `hafidh_feedback_list`, `hafidh_feedback_get`, `hafidh_intake_status`,
backed by host-owned bounded reads with backend task/run/account/folder/product
scope. It should return only already authorized public intake projections;
agent reads cannot import, refresh trusted evidence/freshness, attach proofs,
configure a provider or mint provenance. The host owns those checks and the
real Python IntakeClient process. This worker owns only token/IPC/adapter glue.

Requested minimal host helpers alongside prepare_and_propose: public cached
feedback list/get and sanitized intake status for a backend-created run context.
Their exact DTO names can follow the host's accepted schemas. No raw provider
credential, command, arbitrary path/URL or caller product/account identity
belongs in MCP inputs. The stable agent policy key is `pi`; root connection
ancestry and current task/run are resolved separately as above.

## Pushed identity checkpoint and independent launch gates

Stable-identity implementation is pushed at
`2166686b33bf7cec149053e5fe30ef8c1e9170b1` in draft PR #8. Desktop
`cargo check --locked`, desktop `cargo clippy --locked --all-targets
--features test-utils -- -D warnings`, and server/companion
`cargo clippy --locked --no-default-features --bin codeg-server --bin codeg-mcp
--lib -- -D warnings` all exited 0 at that checkpoint. Existing
proc-macro-error2 2.0.1 future-compatibility and compile-only sidecar warnings
remain; this does not claim a packaged app test.

After the unused generic Hafidh command configuration was removed, extension
typecheck exited 0. The first process-suite rerun had 15 passing tests and one
`spawn ENOEXEC` failure: this worktree's desktop check had copied its documented
zero-byte sidecar placeholder over `target/debug/codeg-mcp`. Rebuilt the owned
binary with `cargo build --locked --no-default-features --bin codeg-mcp`
(exit 0), then reran the complete Pi suite: **16/16 passed, exit 0**. Run this
build after desktop checks before process fixtures. No root sidecar/output was
read or changed.

Refreshed live hook evidence: this session/worktree PreToolUse/PostToolUse
records 4120/4121 (and subsequent pairs) after separate React/Cargo reads.
Latest fetched main is `58fbbf6f`: UI PR #7 and the host seam are still
unaccepted, so their product sources have not been copied. The remaining
configured-deny-through-Ops regression is explicitly pending those accepted
helpers; the stable scope and low-level token/abort tests above are complete.

Accepted intake/main integration is pushed as `e766ca7c`; root planning docs,
lockfiles and registries match that main, and NOTICE retains main verbatim plus
this branch's Pi sections. After that merge the Desk suite passed again (9 plus
1 explicit ignored fixture). The explicitly run extracted-asset RPC fixture
also passed, now checking actual wrapper refusals of cheaper model/high
reasoning/new-session commands, permitted abort, and retained Astra/max with
zero model messages and zero loopback provider connections.

UI PR #7 is now accepted at `f9ae7f1ec91fb0a9569f06fa9dddbde2884db1c6`
(source `756d064f1cc391ed1da32ba90429adef225f080d`). Read its accepted
`ops/{agent,review,types,mod}.rs` completely via git show before integration.
The next commit wires only those shared helpers; ACP will not construct an
operator or copy draft validation/SQL.
