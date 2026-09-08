# Final bundled companion acceptance — prepared, not executed

**Waiting for root's final absolute `Contents/MacOS/codeg-mcp` path, SHA256 and source/build identity. No candidate executable has been run.** This recipe tests that exact packaged companion; prior source-process passes do not certify it. Root owns the final app build, all-three profile/bundle hashes and native AX startup review. N1 source resolution is recorded separately in [business-native-review.md](business-native-review.md), pushed as `169015cf3568a1b136b8ebca3d2a1b95cd630057`.

Owner/worktree: tickets, `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`; branch `review/business-integration-regressions`. No shared target build, existing fixture restart, other-worktree edit, provider/agent launch, installation or credential access is planned. Only new owned synthetic socket/child-process state will be created after the artifact handoff.

## Pinned recipe and execution boundary

Read the existing real-process recipe `integrations/pi-desk/process.test.ts` completely, its config/package, `protocol.ts`, actual companion CLI parser/dispatch/schema/UDS transport, listener token boundary and extracted-assets fixture. Source pin: accepted **Adanmohh/codeg `294fb634b1833ebb13c4223484624e595598235b`**; these paths are unchanged at N1 fix **`3a189d1822f9fd335cc58ecb440f75ab9cbe937e`**. Local source comparison exits 0. Exact blobs:

| File | Blob |
| --- | --- |
| `integrations/pi-desk/process.test.ts` | `e0d98025cb98719aaa0027b64022717c55483c2e` |
| `integrations/pi-desk/protocol.ts` | `e62f02c66d2695010e9f7c3b81f445ef5e6eb0b7` |
| `src-tauri/src/bin/codeg_mcp.rs` | `02f7ed96fade6be35354d3846843fbdc3da6d347` |
| `src-tauri/src/acp/delegation/companion.rs` | `cc0e793af7ee5ced04ab29b4cfa6beca9e023ef2` |
| `src-tauri/src/acp/delegation/transport.rs` | `c267184e772892b5894b606269c1116a9563480e` |
| `src-tauri/src/acp/delegation/listener.rs` | `74898d66293f6eae19363d4bf79ade58493455aa` |
| `src-tauri/src/acp/desk.rs` | `fbf0502ab94b3462638345ea9fe1e0d6338253ea` |
| `src-tauri/src/acp/desk_schema.json` | `d66dafe8d60bf2308c25a5dca3c45f53b73d710a` |
| `src-tauri/src/acp/pi_desk.rs` | `e2d27ecc393ac4c7e2e024d410158e5aa47cb6ea` |

The existing Vitest and extracted-assets tests hardcode `target/debug/codeg-mcp`. Running them unchanged, or copying the bundle executable over that output, would not meet this assignment. Instead adapt only their standalone Node stdio/UDS orchestration into a new owned evidence harness that takes the exact absolute artifact path and expected digest. Do not change product/tests or substitute the development companion. Read installed `@types/node@25.2.2` child_process/net/fs-promises signatures: spawn argument array, explicit environment, stdio pipes, socket listen/close and unique temporary directories. No dependency or runtime addition is needed.

The companion has a handwritten CLI/JSON-RPC dispatcher, not an rmcp server wrapper. `--features` must be explicit: omitted flags retain legacy delegation defaults. The harness must omit `CODEG_PI_DESK_LAUNCH`, `NODE_OPTIONS`, operator tokens, provider credentials and unrelated inherited configuration. Spawn only the supplied binary with `shell:false`, an explicit minimal environment and new owned working directory. `logging/init.rs:562–579` uses stderr-only logging for this mode. No Pi, adapter, model catalogue or server engine is required for this bounded MCP wire check.

## Planned assertions against the supplied executable

1. **Artifact identity:** require the supplied absolute path and expected SHA256 before any execution; verify regular nonempty executable and digest. Record source/build identity, resolved path and byte size. Read its actual `--help` first after handoff. Execute the original bundle path directly, without chmod/copy/staging or concurrent compilation. Rehash after the run; a mismatch invalidates the result and stops acceptance.
2. **Owned setup:** create a unique short socket path inside a new directory in this worktree, with synthetic public records and a synthetic launch token known only to the harness/child. No TCP backend/real database is attached. Use argv `--features intake` or `--features desk`, `--parent-connection-id fixture-untrusted-label`, `--socket-path <owned socket>`, `--token <synthetic token>`, and `--parent-pid <harness PID>`. Never inherit a production socket or parent token. Do not log request tokens or arbitrary stderr payloads.
3. **Initialize/discovery:** send newline JSON-RPC `initialize`, `notifications/initialized`, then `tools/list`. Expect protocol `2024-11-05`, server `codeg-mcp`, package version matching the supplied source, tools capability, valid correlated response IDs and no stdout diagnostic contamination. Run the two feature groups sequentially. Assert full exact name sets below, `additionalProperties:false`, and explicit cached semantics for all three intake descriptions.
4. **Permitted public fixture read:** for each group, invoke a read (`hafidh_intake_status` / `desk_business_task`, with an additional `desk_thread` public-record case as needed). The owned listener accepts only the expected token/tool/input and returns a fixed public outcome. Assert exact round-trip JSON and `isError:false`, with no private reporter, proof, configuration or credential fields. The actual UDS message must be only `{kind:"desk",token,request:{tool,input}}`; the untrusted CLI parent label must not become request identity.
5. **Closed/refused capabilities:** try `approve`, `deny`, `execute`, `send_email`, `fetch`, `private_notes`, `delegate_to_agent` and `task_complete`; expect JSON-RPC `-32602` and zero listener requests. In intake-only mode also refuse Desk draft/proposal/business mutation names. Nonobject Desk arguments must likewise reject before UDS. Read actual schema restrictions for business progress (`todo`/`in_progress`/`review`, never `done`/`cancelled`) without invoking a mutation.
6. **Scope/stale/error relay:** the synthetic listener returns `{ok:false,code:"denied"}` for its wrong-record/token case and `{ok:false,code:"stale"}` for its retired fixture state; require exact error outcome and `isError:true`, with no successful/public value. These are transport refusal checks, not a claim of actual TokenRegistry/DB authorization. A malformed/truncated owned response may check the JSON-RPC `-32603` broker-error path if needed; no arbitrary remote host is contacted.
7. **Cancellation/lifecycle:** park one read in the owned listener, send `notifications/cancelled` with its request ID, observe UDS close, and confirm no result for the cancelled ID while a later `tools/list` still responds. Then close the owned listener and repeat a safe read: expect a bounded JSON-RPC `-32603`, no fallback transport and no success response. Finish by closing stdin, await exit, then kill only the recorded child if the bounded cleanup deadline is exceeded. Close only harness sockets/server and remove only its generated directory; record actual child exit, socket removal and final digest.

Exact `intake` tools: `hafidh_feedback_list`, `hafidh_feedback_get`, `hafidh_intake_status`.

Exact `desk` tools: `desk_context`, `desk_tickets`, `desk_thread`, `desk_save_reply`, `desk_propose_reply`, `desk_propose_issue`, `desk_business_task`, `desk_business_progress`, `desk_business_note`, `desk_business_submit`. Only reads are executed in this acceptance run. Discovery of a draft/proposal tool does not authorize approval, external execution or provider transport.

Each request gets a bounded five-second response deadline, with a separate bounded teardown deadline. The evidence records checks, feature/name sets, request counts, synthetic success/refusal values, cancellation/socket closure, child exits and before/after executable hashes. No prompt, model, provider SDK, live email/GitHub request or freshness-minting operation is issued. This topology creates only the harness, actual companion and owned UDS listener; it is not an OS-wide network trace or sandbox claim.

## Limits and next handoff

The companion deliberately forwards object-shaped input to the trusted backend, whose closed DTOs and live authorization reject extra identity/scope fields. It is not a second Ops/task validator. Schema inspection and synthetic refusals must not be misreported as executing that real backend validation. Existing accepted listener/ownership/CAS/revocation tests provide separate source evidence; this task will not redundantly rerun those suites or start another task engine. Full bundled parent-listener/real task binding, extracted Pi/adapter loading and native AX controls are separate gates if root requires them after this artifact wire check.

No recipe execution/result is recorded yet. Source reads and blob/diff inspection exited 0; there are no new runtime passes. Docs-first hooks remain enabled; existing code-context missing installed corpus coverage remains disclosed in the native report. All APIs were read locally; no new remote research or third-party source port. Existing Codeg Apache-2.0 and the Pi package's MIT attribution apply to the reused fixture patterns; preserve exact attribution if the evidence harness is materialized. Existing NOTICE/LICENSE and all paused work remain unchanged.

**Next input required:** root's final bundle executable absolute path, SHA256, source commit and stable completed-build window. Wait for that handoff before executing even `--help`. Root will independently correlate `codeg`, `codeg-server` and `codeg-mcp` profile/bundle hashes. A protocol pass will be attributed only to the exact tested digest.
