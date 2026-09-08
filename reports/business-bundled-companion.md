# Final N2 bundled companion acceptance — passed

**The final N2 artifact passed the unchanged harness: exit0, 275 assertions,
zero failures.** This is a new execution against the changed final digest, not
carried-forward acceptance of the earlier package. Evidence:
[result-1788889157262-15575.json](business-bundled-companion-evidence/result-1788889157262-15575.json),
SHA256 `d970effe076fe8b16dd2a49b4eb5a2c4455c22a91e834ddc65d65e491d87a986`.

Root supplied stable window `business-native-n2-f4757d8d` after its normal
package build exited0: immutable source
`f4757d8dcd9e62824bfde577a5595a5a2a5e5c7c`, product `3d000874`, package
`0.30.4`. The actual before/after bundle identity was exactly **21,711,944 bytes**,
SHA256 **`9c53291347445a6ca80f057b11c59955013998f30bc3a65fa31c3e74bae7b5f5`**.
Root's separate native app remained outside this probe.

Before execution, read the complete build-source `Cargo.toml` with `git show`,
exit0. `git diff --exit-code` between pinned recipe source `294fb634` and this
build source, restricted to all nine paths in the ledger below, exited0.
The committed `464ae97a` harness remained byte-identical, SHA256
`9e9fdc9bb2df58e6c6d9f15e3c12dafc74027690f197a01dabf35afe78a16796`;
no syntax rerun or source modification was needed. Executed exactly once, exit0:

```text
node reports/business-bundled-companion-evidence/probe.mjs \
  --binary '/Users/mohamedadan/projects/ops-desk/src-tauri/target/debug/bundle/macos/Hafidh Ops Desk.app/Contents/MacOS/codeg-mcp' \
  --sha256 9c53291347445a6ca80f057b11c59955013998f30bc3a65fa31c3e74bae7b5f5 \
  --source f4757d8dcd9e62824bfde577a5595a5a2a5e5c7c \
  --package-version 0.30.4 \
  --stable-window business-native-n2-f4757d8d
```

| Final N2 boundary | Actual result |
| --- | --- |
| Help and initialization | Help exit0; both groups returned protocol `2024-11-05`, server `codeg-mcp`, version `0.30.4`; zero stderr bytes. |
| Discovery and refusal | Exact three intake/ten Desk tools; closed schemas, cached intake descriptions and nonterminal business progress enum. All 26 disabled-tool calls and four nonobject inputs refused locally with `-32602`, zero forwarding. |
| Scoped synthetic relay | Four reads per group: exact public success, `denied`, `stale` and parked cancellation. Exact UDS envelope; CLI parent label never became forwarded identity. |
| Cancellation / listener removal | Cancelled reads closed their socket and emitted no result; later discovery worked. Missing listener returned `-32603`, no successful fallback. |
| Processes / cleanup | Harness PID `15575`; help `15576`, intake `15577`, Desk `15578` all exit0, null signal, no IO error or forced termination. Own `.bpc-KgXqT9` removed, `scratchRemoved:true`. |
| Artifact | Before/after absolute path, 21,711,944-byte size and complete `9c532913…` digest equal. |

These are **275 assertions, not 275 independent tests**. The unchanged scope and
limits below apply: a synthetic UDS peer checks transport/error relay, not real
backend authorization, database scope or the native UI. No Pi, model, provider,
engine, server or TCP listener was launched by this harness; zero-action counters
are not an OS-wide monitor. No target, existing fixture, dependency, configuration,
credential, product source or root NOTICE/LICENSE was changed. Live own-session
docs-first PreToolUse/PostToolUse records at `1788889191`/`1788889192` show exit0
in the tickets worktree; session `01a07c1c-d82f-7022-84db-778a438632f1`.

The B docs draft was checkpointed/pushed as
`79a922945668a633ea6b5f7e68f9bdc08a0725a3` before switching to the owned
`review/business-integration-regressions` branch for this report/result-only
handoff. Prior result and paused visual report hashes remain unchanged. Root was
notified of complete execution/cleanup and release of the stable window. B
contract preparation resumes separately; no B implementation has run.

## Previous package execution retained — superseded artifact

The following original record describes only digest `90f9fd8e…` at source
`6cfff7d6`. Its result remains unchanged for provenance; final N2 acceptance is
the distinct execution above.

**The exact supplied bundled companion passed: command exit0, 275 harness checks, no failures.** Both MCP feature groups completed their synthetic read/refusal/stale/cancellation/missing-listener cases; all children exited0, owned socket state was removed, and before/after artifact hashes matched. Evidence: [result-1788886066203-29071.json](business-bundled-companion-evidence/result-1788886066203-29071.json). This is packaged MCP transport evidence with a synthetic peer. Root owns the separate final app/all-three executable correlation and native AX review. N1 source resolution is recorded in [business-native-review.md](business-native-review.md), pushed as `169015cf3568a1b136b8ebca3d2a1b95cd630057`.

Owner/worktree: tickets, `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`; branch `review/business-integration-regressions`. Only the supplied companion and new owned synthetic socket state were used. No shared target build/mutation, existing fixture restart, other-worktree edit, Pi/model/provider/engine launch, installation or credential access. Root's concurrent isolated app launch was outside this probe.

## Pinned recipe and execution boundary

Read the existing real-process recipe `integrations/pi-desk/process.test.ts` completely, its config/package, `protocol.ts`, actual companion CLI parser/dispatch/schema/UDS transport, listener token boundary and extracted-assets fixture. Source pin: accepted **Adanmohh/codeg `294fb634b1833ebb13c4223484624e595598235b`**. The nine paths below remain unchanged at root's immutable build source **`6cfff7d64e4d45f84d8cd5a26325b6ad4e1f311f`**: `git diff --exit-code` of the two commits, restricted to those paths, exited0 before execution. Read that build commit's complete `src-tauri/Cargo.toml` via local `git show`, exit0; package is `0.30.4`. Exact blobs:

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

The existing Vitest and extracted-assets tests hardcode `target/debug/codeg-mcp`. Running them unchanged, or copying the bundle executable over that output, would not meet this assignment. The new [probe.mjs](business-bundled-companion-evidence/probe.mjs) adapts their standalone Node stdio/UDS orchestration and takes the exact absolute artifact path and expected digest. No product/test import, development companion substitution or dependency addition. Read installed `@types/node@25.2.2` child_process/net/fs-promises/crypto/readline/path/url/util signatures and Node24.19.0 help: argument-array spawn, explicit environment, stdio pipes, socket lifecycle, unique directories, SHA256, filesystem identity and deep comparison.

The companion has a handwritten CLI/JSON-RPC dispatcher, not an rmcp server wrapper. `--features` must be explicit: omitted flags retain legacy delegation defaults. The harness must omit `CODEG_PI_DESK_LAUNCH`, `NODE_OPTIONS`, operator tokens, provider credentials and unrelated inherited configuration. Spawn only the supplied binary with `shell:false`, an explicit minimal environment and new owned working directory. `logging/init.rs:562–579` uses stderr-only logging for this mode. No Pi, adapter, model catalogue or server engine is required for this bounded MCP wire check.

## Materialized harness and syntax evidence

Root accepted recipe commit `6af123bad49d5d25027fa8af9fbc3ef1310a1a79` and materialized harness **`464ae97a1bf243d9a22f5ec35fa6c45d7c674c66`**, then supplied the completed-build execution handoff. Ran that unchanged committed harness, SHA256 **`9e9fdc9bb2df58e6c6d9f15e3c12dafc74027690f197a01dabf35afe78a16796`**. The adjacent [NOTICE](business-bundled-companion-evidence/NOTICE) maps the exact borrowed fixture and reproduces all inherited Pi-package MIT notices; the original Apache licence remains at the repository root. No root NOTICE/LICENSE edit.

Previously ran **`node --check reports/business-bundled-companion-evidence/probe.mjs`**, Node **v24.19.0**, **exit0** on the final harness. Installed help explicitly says this checks syntax without executing the script. These preparatory checks did not execute a candidate. Runtime evidence below was obtained only after the explicit completed-build handoff; no syntax/test rerun was needed for unchanged harness source.

The harness requires all five named arguments, validates the SHA/source syntax and owned worktree, checks the actual bundle-path suffix/nonempty executable/digest, then launches only that absolute binary. Its child environment is exactly `NODE_ENV=test` and `PATH=/usr/bin:/bin`; it does not inherit HOME or credentials. New scratch is owned `.bpc-*`, mode0700. A unique `result-<timestamp>-<pid>.json` in the evidence directory uses exclusive create/mode0600. Failure summaries use fixed check labels; raw request tokens, arbitrary response bodies and stderr text are never serialized. The public outcome is explicitly synthetic transport data, not a claimed real task DTO.

Executed once from the tickets worktree, **exit0**, in root's explicit stable window **`business-native-6cfff7d6`**:

```text
node reports/business-bundled-companion-evidence/probe.mjs \
  --binary '/Users/mohamedadan/projects/ops-desk/src-tauri/target/debug/bundle/macos/Hafidh Ops Desk.app/Contents/MacOS/codeg-mcp' \
  --sha256 90f9fd8ed430ce1710919fc030ac87c34ba0c8e0a6e23cc4706f2c2b9515846d \
  --source 6cfff7d64e4d45f84d8cd5a26325b6ad4e1f311f \
  --package-version '0.30.4' \
  --stable-window business-native-6cfff7d6
```

Root supplied the stable window after its normal Tauri build exited0; this worker did not infer stability or rebuild anything. The before/after executable was **21,711,944 bytes**, SHA256 **`90f9fd8ed430ce1710919fc030ac87c34ba0c8e0a6e23cc4706f2c2b9515846d`**, at the exact resolved path above. The result JSON SHA256 is **`9f7608abcd50f0427dd64b195a40f3e94a34367aaeace90a95c6cf6efe24c690`**. Source identity is the root handoff plus local source comparison; the digest independently identifies the binary tested.

## Observed runtime results

| Boundary | Actual evidence |
| --- | --- |
| Help and initialization | Bundled `--help` exited0; both groups returned protocol `2024-11-05`, `codeg-mcp` version `0.30.4`, tools capability and correlated JSON-RPC. No stdout contamination or stderr bytes. |
| Discovery | Exactly three intake and ten Desk tools listed below; all input schemas closed. Three intake descriptions explicitly say cached; business progress schema contains only `todo`, `in_progress`, `review`. No mutation invoked. |
| Local refusals | 26 disabled/forbidden tool calls and four nonobject-input calls returned `-32602` with no success result and zero forwarded requests before the read cases. |
| Synthetic read/error relay | Each group forwarded exactly four reads: fixed public success, wrong-record `denied`, retired-state `stale`, and parked cancellation. Exact UDS token/tool/input envelope matched; no CLI parent label was forwarded. Success/error JSON and `isError` matched exactly. |
| Cancellation and missing listener | Both parked reads closed their UDS connection after cancellation, emitted no cancelled result, and subsequent discovery responded. After listener close, safe reads returned `-32603` without success/fallback. |
| Child lifecycle | Help PID `29072`, intake `29073`, Desk `29074`: each exit0, signal null, no spawn/stdin error, zero stderr bytes, no forced termination. Harness PID `29071`, exit0. |
| Cleanup and identity | Owned `.bpc-Z6HysH` and sockets removed (`scratchRemoved:true`); before/after path, size and SHA256 equal. No other fixture touched. |

The **275 checks are harness assertions, including repeated RPC health checks, not 275 independent test cases**. Both feature groups passed; `failures` is empty. The harness issued zero model/Pi launches, provider requests or TCP listeners. Those counters describe this harness's actions, not an OS-wide network monitor. Root was notified that artifact execution/cleanup had completed before this report commit.

## Approved recipe retained for reproduction

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

## Limits and docs-first evidence

The companion deliberately forwards object-shaped input to the trusted backend, whose closed DTOs and live authorization reject extra identity/scope fields. It is not a second Ops/task validator. Schema inspection and synthetic refusals must not be misreported as executing that real backend validation. Existing accepted listener/ownership/CAS/revocation tests provide separate source evidence; this task will not redundantly rerun those suites or start another task engine. Full bundled parent-listener/real task binding, extracted Pi/adapter loading and native AX controls are separate gates if root requires them after this artifact wire check.

Code-context guide with the existing venv/`HF_HUB_OFFLINE=1` exited0 and returned mostly other-project guidance; applied its installed-version rule. Docs query exited3 because `tickets.db` coverage is absent; no ingestion/install or invented coverage. Installed Node24.19.0 help and `@types/node@25.2.2` APIs were read before harness authoring. Live own-session PreToolUse/PostToolUse records at `1788886098`/`1788886099` have exit0 and the tickets worktree; hooks remain enabled. All APIs were read locally; no new remote research. The reused fixture attribution is preserved in the evidence NOTICE. Root NOTICE/LICENSE, product source, target outputs and paused visual work remain unchanged.

**Bounded acceptance complete for the exact digest above.** No malformed/truncated response extension, wrong-token backend authorization test, real database/parent-listener binding, Pi/installed-adapter launch or native AX interaction was run here. Root's reported all-three executable/profile and 1,016 web-file matches remain root evidence, not worker reruns. This result does not certify release signing, distribution, other binaries or a changed artifact.
