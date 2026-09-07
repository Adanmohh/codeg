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
| xintaofei/codeg, Apache-2.0 | v0.30.4, `6f6bd648b206412644842a98d9ffeebf57292bed` | Existing `acp/delegation/{listener,transport,companion}.rs`, `acp/work_task_tools.rs`, `work_task/engine.rs`, `acp/connection.rs`, `bin/codeg_mcp.rs`, default resolver. Reuse token, framing, cancellation and private task map; minimal additive glue. Remote source verification continuing. |

Full original MIT notices will be reproduced in NOTICE before port delivery.
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

Checkpoint commit: `5a5561ff` (pushed). Draft PR:
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
task/run via `TaskEngine::task_for_connection`; `agent` is a trusted connection
identity, not request text. Unknown/disconnected connection, revoked token,
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
