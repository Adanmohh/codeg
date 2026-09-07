# Pi integration contract — source review, 2026-09-07

Planning evidence for FOUNDING pieces 1b/6, not an implemented integration. All remote reads used gh api.

## Verified source pins

- `badlogic/pi-mono@v0.85.1` resolves to `d981de1229ef899957bbe968bc8dcda02a21f477`. Installed `pi --version` is 0.85.1. Read `packages/coding-agent/examples/extensions/confirm-destructive.ts`, `bash-spawn-hook.ts`, and the relevant tool events, loading and trust sections of `packages/coding-agent/docs/extensions.md`.
- `nicobailon/pi-mcp-adapter@v2.32.1` resolves to `10a45367e033a32026987a75d6f401e37340c86f`. Read `tool-approval.ts` and the approval event/type section in `types.ts`; recursive source tree confirms README, LICENSE and `__tests__/tool-approval.test.ts` for the implementer to read before porting.
- Local codeg registry at `src-tauri/src/acp/registry.rs:1286` pins `pi-acp@0.0.33`, which launches pi RPC. ACP-wire MCP servers are deliberately skipped for Pi. Actual adapter installation/loading must therefore occur in Pi, not in an ineffective ACP mcpServers field.

## Contract details that constrain the implementation

1. `confirm-destructive.ts` protects session switching/forking with UI confirmations; it does **not** itself enforce command or external-send approval. Reuse it as an extension lifecycle pattern only. The actual execution guard must use the documented async `tool_call` seam or the MCP broker and the desk approval service.
2. Pi's documented tool-call input is mutable and mutations reach execution, but Pi does not validate again after mutation. A desk extension must validate the complete approved command, bind it to the reviewed snapshot and prevent later substitution. Returning a boolean permission for a changed draft is insufficient.
3. The adapter emits `pi-mcp-adapter:tool-approval-request` for each uncached call. A listener must synchronously claim the request; the claimed handler can resolve asynchronously. Request fields include requestId, serverName, originalToolName, prefixedToolName, args, origin and optional abort signal. Only the first synchronous claim wins.
4. Decisions are allow_once, allow_for_session, deny or abstain. The broker decision shape does not carry replacement arguments. Do not claim edited-payload execution merely by returning allow_once. If an MCP approval allows edits, require a separately verified execution path or deny the original call and explicitly hand off the edited action through the desk executor.
5. Session grants are cached by server/tool/argument hash and bypass later broker calls. The desk bridge must use allow_once for approved actions; never grant destructive or externally visible actions for an entire session.
6. With no claimed broker, the adapter follows approveTools configuration; it fails closed headlessly only for tools that require approval. Configure the intended MCP set explicitly. An absent/unloaded bridge must not silently turn a required approval into automatic execution.
7. Resend and GitHub App actions are direct desk REST transports under the owner amendment, not MCP providers. Keep their actual external execution in trusted desk action packs consuming the committed exact authorization. Hafidh intake can remain a read-oriented MCP surface.
8. Project-local Pi extensions require project trust. Global installation is not a substitute for explicit per-project wiring; do not change unrelated global agent configuration. The adapter's actual loader and Pi RPC/ACP compatibility still need local validation.
9. Agent proposal/read capabilities must not include human approval authority. Reuse the repository's per-launch delegation token/connection identity patterns after reading them; do not give the agent the unrestricted operator server token and then rely on a user-supplied actor string.
10. Cancellation and abort signals must retract the bridge wait. Late or replayed responses must not execute against another run or changed payload. Request ownership must integrate with the newly reviewed ACP/Ops wait service.

## Required implementation checks

- Actual loaded extension in pinned Pi/ACP, including missing bridge/config failure behavior.
- One approved exact payload produces one handoff; deny, timeout, abort, stale generation and malformed response produce none.
- Broker claim is synchronous; no session-wide grants; changed inputs cannot reuse an approval.
- Direct email/GitHub proposals use the trusted registry and proper human/agent capability separation.
- Pi defaults apply to new Ops tasks without rewriting existing user agent choices. The fresh browser baseline currently shows inherited Claude defaults and a missing ACP adapter; this is not completed Pi setup.
- No live send or issue filing is needed for automated tests: use local provider fixtures and explicitly distinguish them from production configuration.

Before implementation, read complete source/license/tests at the pins above and cite every adapted file in NOTICE. This contract records verified behavior and review requirements; it adds no Pi extension, dependency, runtime registration or new authority.

## Runtime model and scoped proposal handoff follow-up

Read installed pi 0.85.1 provider/model/auth-storage documentation and codeg `src/lib/resolve-default-agent.ts`, Pi configuration API wrappers and `acp/delegation/listener.rs` on 2026-09-07. Safe offline `pi --no-extensions --no-skills --no-prompt-templates --no-themes --no-context-files --list-models gpt-6-astra` returns no matches; the installed generated model catalogue also has no Astra entry. The available offline list currently contains a configured Qwen-plan provider. This is a product-runtime setup gap, not a worker downgrade: all Herdr workers remain GPT-6 Astra max. Do not silently select a cheaper model for the Desk. The later pi worker must verify current catalogue/custom-model support and expose missing configuration honestly; no auth files or global defaults were modified by this inspection.

The existing `TokenRegistry` binds a per-launch token to `parent_connection_id` and working directory and supports revocation by parent. Existing task progress/completion dispatch resolves task ownership from that connection through TaskEngine. Borrow this seam for scoped Ops proposal capabilities; it is not an existing Ops authorization endpoint. `CODEG_TOKEN` belongs to the operator boundary and must not be handed to pi tool configuration. The inherited single-tenant, same-user filesystem model is not an OS sandbox against a malicious local process.

The current default-agent resolver respects folder defaults, active-conversation inheritance, the user-sorted agent list, then `AGENT_DISPLAY_ORDER`. A later default-to-pi change must preserve explicit saved user choices. Existing Pi configuration APIs merge native settings/auth/model files; do not overwrite unrelated global configuration to implement project defaults.
