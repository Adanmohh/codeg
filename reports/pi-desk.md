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

Checkpoint commit and draft PR: pending first push.
