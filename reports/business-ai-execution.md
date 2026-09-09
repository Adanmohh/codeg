# Business AI execution contract — tickets

Docs-only handoff on `docs/business-ai-execution`, based on accepted
`f30a27da45c41ed7593c5da433759af8314a08ef` after a documentation-only fast-forward
from `cba079c756efe058b4b9cd27d640c022568148df`.
Deliverable: [contract](../docs/contracts/business-ai-execution.md).
Draft [PR31](https://github.com/Adanmohh/codeg/pull/31); early checkpoint
`d1f22f85a22d844660df99a770daee39df3e371f` published before final DTOs.
No product implementation or execution acceptance claimed. Frozen contract head:
`3f164c2a989cd08a523e51f1e47559c15a48c0ef` (final event/content and public
metadata clarification after `f454db7554531feb6fd8e9fd5c7949e7b7fa777e` and full
DTO/recovery head `f2af30f73e38cd929b993026df8600fde32715d6`).
Contract SHA256: `e2cf8304fb5eababce79961da57061e244f2567eae11b75b88dbfcbccc6a114d`.

## Progress and preserved work

- B closeout committed/pushed as `462604b00ad74a7f72c36d0bea3714a5912bf640`
  on `feat/business-intake`, draft PR28. Production remains
  `e55f3bfd1f069d6d6111370223993596b19ecb9b`; fixture source `7ed0dd0c`.
- Stable backend/upstream PID66200, ports4351/4352, data and binary unchanged.
  `intake-worker4354` is USER-INSPECTED; no further automation, navigation,
  closing, refresh or worker-record writes. The human draft is preserved.
- Paused untracked `reports/visual-correspondence.md` remains untouched; old
  targets, exports, other fixtures and native bundle remain untouched.
- Source investigation found existing hidden non-Git chat folders, actual ACP
  reuse and text-only business submit. Original host credentials/terminal and the
  broad companion session read are not tenant authority. Managed asset persistence
  is necessary; existing tool-output file cards do not provide it.
- Owner's persistent sessions, managed documents/slides and current connected
  marketing-account state are included. First handoff is the smallest real
  task→existing AI→managed version→human review slice, not a general survey.

## Source and verification method

Governing docs/contracts and accepted tenancy/native/companion findings were read;
current root product amendment read at `f30a27da4`. Code-context skill applied with
existing `/Users/mohamedadan/projects/rag-skills/.venv/bin/python` and
`HF_HUB_OFFLINE=1`. Guide command exit0: “Atomic per-task staging” applies.
Other-project per-tenant-database/subagent guidance does not replace the accepted
shared database/three-existing-worker assignment. Installed dependency corpus for
this worktree was previously missing (exit3); direct installed source is used,
not a claim of complete RAG coverage. Separate local reads confirm React19.2.4
and Codeg0.30.4 Cargo manifest; hook evidence and exact source ledger follow.

Read local Codeg Apache-2.0 seams listed in the contract; no new third-party hunk
or license change. Original Codeg v0.30.4 pin remains
`6f6bd648b206412644842a98d9ffeebf57292bed`. Prior af00/tenancy and Pi/companion
provenance is retained; no AGPL/GPL port. No remote survey or dependency upgrade.

Live audit evidence read from `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`:
own session `01a07c1c-d82f-7022-84db-778a438632f1`, exact tickets cwd, PreToolUse
lines25893–25894 and25913–25915; PostToolUse lines25895–25897. Hook reminders also
appeared during the immutable gh-api source reads. No disabling/bypass.
Whitelisted local turn_context metadata only: lines14796 (09:33:18.662Z) and15334
(09:54:43.206Z),2026-09-09, both `gpt-6-astra`/`max`, tickets cwd. This establishes
the current authored turns, not a claim that the entire historic session never
used another model; prior model-audit limits remain unchanged. No message/secret
payloads were inspected for this metadata check.

Installed locks/source: React19.2.4; portable-pty0.8.1 `src/cmdbuilder.rs`72–79,
209–214,309–320 (host environment inherited by default, explicit clearing exists);
Tokio1.49.0, SACP/SACP-Tokio11.0.0 (local vendor patch retained),
agent-client-protocol-schema0.11.7 `src/agent.rs`3977–3981/4562–4572 (new/prompt/
cancel baseline, load capability); SeaORM1.1.19, Axum0.8.8, Tauri2.10.2.
These do not prove process/tenant isolation. Offline docs query rerun exit3:
`data/code/tickets.db` missing; no corpus install/ingest or fabricated coverage.

## Exact remote official reads and licensing

After local installed/source reads, used only `gh api` for the bounded official
protocol check, at root's unchanged pin
`modelcontextprotocol/modelcontextprotocol@aa8ce049f089f92618340190d4ece141f663310d`,
specification2026-07-28. Commit API resolved tree
`b31887a5eafa44808ca3ba8cca6e8b3e1e36c5bb`; exact tree entries then raw contents read:

| Official file | Blob / consequence |
| --- | --- |
| [server/tools.mdx](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/aa8ce049f089f92618340190d4ece141f663310d/docs/specification/2026-07-28/server/tools.mdx) | `449020f54a6582122607b4869129bec5f1035f37`; explicit tool input/result schema and errors; annotations alone are untrusted; state handles still need caller authorization. Supports closed dispatch, not a universal provider capability claim. |
| [server/resources.mdx](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/aa8ce049f089f92618340190d4ece141f663310d/docs/specification/2026-07-28/server/resources.mdx) | `f49dd8e6be3fd8f13911788ae5f5d4c87d2c53cd`; resource URIs/list/read/notifications supply context; file paths and permissions still require validation. A resource link is not our durable asset library. |
| [LICENSE](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/aa8ce049f089f92618340190d4ece141f663310d/LICENSE) | `4a93985763241755401a10678395303de4e720ba`; Apache-2.0 transition with unrelicensed contributions retaining MIT, non-spec documentation CC-BY-4.0. Read as design reference only, no source hunk/examples copied. |

Commands: `gh api repos/modelcontextprotocol/modelcontextprotocol/commits/<pin>`,
`gh api 'repos/.../git/trees/<pin>?recursive=1'` with selected fields, and
`gh api -H 'Accept: application/vnd.github.raw+json' 'repos/.../contents/<path>?ref=<pin>'`.
All exited0. Long output was bounded; remaining security/license-header sections
were explicitly read with tail/head, not assumed from truncated output. No MCP
wire/dependency upgrade: current companion/Pi protocol remains pinned. Actual
social account adapter is unselected/unverified; E3 cannot show fake accounts.

Inherited reuse: Codeg Apache v0.30.4 pin above; owner-owned IntroMail
`0bd24dfe284b888aa9f602fa1fd00e337ea38874` existing authorization/audit adaptation;
pi-mono0.85.1 `d981de1229ef899957bbe968bc8dcda02a21f477` and pi-mcp-adapter2.32.1
`10a45367e033a32026987a75d6f401e37340c86f` (MIT) remain in NOTICE. New product
ports must attribute exact selected files; this docs-only PR preserves NOTICE
blob `78c75a1ef015459833835c3412b19d7314a5f5d8` and LICENSE
`261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64` byte-for-byte.

Read/coordination commands: `git status`, installed `gh help pr create`/`gh help
api`, docs-only `git merge --ff-only origin/main`, offline guide, exact-file
staging/commit/push and `gh pr create --draft --body-file` exited0. Authorized
Herdr prompts relayed the checkpoint to existing root/UI/reviewer panes, exit0;
no new agents. Some exploratory searches used wrong paths/version guesses and
returned rg/shell errors; subsequent `rg --files` located actual files. No claim
relies on nonexistent paths or a truncated unread section. Context-mismatch patch
attempts applied nothing and were retried against the actual document. No product
tests/builds, candidate binary, browser, provider, Pi or engine process ran here.

## Local source-to-contract ledger

Git blobs at accepted `f30a27da45c41ed7593c5da433759af8314a08ef` below. Relevant
definitions/callers/tests were read, not a claim to have read every line of the
very large ACP files. Exact ranges/behaviors are in the contract's reuse table.
The product source is unchanged from `cba079c75`; intervening files are root docs
and imported B browser evidence, not new backend code.

| Source | Immutable blob |
| --- | --- |
| `src-tauri/src/commands/conversations.rs` | `01d46d718ce08cce32a5636f5c90ab0ca352b6bb` |
| `src-tauri/src/db/service/folder_service.rs` | `9b8d7a7db24c751c129360401b616214bd72ab50` |
| `src-tauri/src/acp/manager.rs` | `67ca08b74f765833f7216f59d13035cfe3738406` |
| `src-tauri/src/acp/connection.rs` | `7ba5bd65dbe91346aaef5b3e649353ba1e710796` |
| `src-tauri/src/commands/acp.rs` | `1ed2d91ca23fb17bdbfaf921bb52e47ae76145f6` |
| `src-tauri/src/acp/delegation/listener.rs` | `74898d66293f6eae19363d4bf79ade58493455aa` |
| `src-tauri/src/acp/pi_desk.rs` | `e2d27ecc393ac4c7e2e024d410158e5aa47cb6ea` |
| `src-tauri/src/web/handlers/terminal.rs` | `3d61e09b75ab91164904a50e5bcf9a9daae59e33` |
| `src-tauri/src/terminal/manager.rs` | `8f32b0e35d293dbced452965b60fa8a2c7f45a68` |
| `src-tauri/src/web/ws_attach.rs` | `a847caa00cb29f6711979061cf9443a5e39bc0de` |
| `src-tauri/src/web/ws.rs` | `3fae93f2638b74206bbc289f7ed9c5ed1e07c980` |
| `src-tauri/src/business_identity/mod.rs` | `f7205760715d2516b11b6e2cb79fd133218672bb` |
| `src-tauri/src/business_tasks/store.rs` | `d48fdf1e85feb4410dbff51e8a671d6c9d525691` |
| `src-tauri/src/business_tasks/policy.rs` | `59c0fe10a27c52a75679bcd4f28779e1e0984055` |
| `src-tauri/src/business_tasks/agent.rs` | `616317e0e47956bf8a35da78f6054f516b71684f` |
| `src-tauri/src/web/handlers/upload_jail.rs` | `66621d1bccf9fe1a8080eea4a38864e90635e3df` |
| `src/components/message/reply-artifacts.tsx` | `65474f8c8a5af35644c6ed9a3adf9e0341890dd6` |
| `src/components/files/office-preview.tsx` | `d13764842eb2ef17f75f3566899f9c49f2109d53` |
| `src/components/conversations/conversation-detail-panel.tsx` | `0e138f15401229526a7fa9a158d8ab3b13d21a39` |
| `src/lib/business/client.ts` | `7b90fb8a8b9e7907571f6dbf212fd7dd610a2492` |
| `src/lib/office-actions.ts` | `d5cf2059f21b72637a9ffd6572b7cc9dee50017d` |
| `integrations/pi-desk/launch.mjs` | `fa90e89b3b6202d83ca5f7ffae91e793171f1d97` |
| `integrations/pi-desk/process.test.ts` | `e0d98025cb98719aaa0027b64022717c55483c2e` |

## Closed handoff and limits

E1 is ready for bounded contract review: every private route requires the real
original operator; stored Operator/Credential lineage remains explicit; start,
attach, prompt, stop, file import and submission have durable receipts/unknown
states; managed versions use service-owned bytes and atomic task-owned CAS/audit;
only explicitly selected versions reach named human review. Existing human task
policy is unchanged. No tenant Principal minted from host access or session IDs.
Read the full initial independent review through immutable `gh api` at
`3eb8f971df67386f7fd0dd3bf3bcbf920a01390d`, exit0. E1-1..4 are addressed in
Admission/Wire, receipt/recovery table, managed storage protocol and explicit
human-only `assets/submit`. Operator audit identifies the mapped original owner
and operator authority, not an individually proven user of a shared host token;
separate member review keeps the credential-derived human identity. This is owner
reconciliation, not the independent reviewer's final verdict.
E2 process/native isolation and E3 a verified official account adapter remain
explicit prerequisites, not reasons to delay E1 implementation dispatch. All new
DTO paths and synthetic selectors are proposed, not existing/passing APIs/tests.

Own B evidence remains the partial user-inspected closeout. Approvals separately
reports `6895bedf4`:62 API requests/278 assertions,11 actual IUI assertions and3
read-persistence checks, its own browser closed. Those are attributed independent
B results, not AI execution proof. No new fixture health call/restart, source or
record mutation was needed for this contract. Await exact authority/UI review;
no runtime exposure before root dispatch/acceptance.

Final closeout: explicit `git diff --check`, commit/push and `gh api` immutable
commit/PR verification exit0; PR31 is open/draft against main. Only the two owned
documents differ from the accepted branch point. Final report commit follows the
frozen contract; it does not change that hash. Original paused report and all
runtime/fixture resources remain preserved.

Root has now dispatched **internal E1 prerequisites only**, separately on
`feat/business-ai-execution`, migration `m20260909_000014_business_execution`.
E1 must reject unimplemented `account_snapshot` input references; future manager
stop applies to E2, never relaxes E1's every-operation operator gate. Published
provenance remains selected-version facts only. Final `3f164c2a` explicitly pins
the safe public metadata projection (no private session/turn/profile references),
authenticated POST NDJSON framing/reset/replay/history/revalidation, bounded
content/error/disposition/no-range responses and fetch-to-Blob rendering without
bearer URLs. E2 manager stop is explicitly separate; unavailable E1 account refs
fail before mutation. These close the five requested wire clarifications for
independent review; runtime implementation/testing is still required. No routes,
native registration, profile launch or fixture/provider work until final authority
verdict. That implementation gets `reports/business-ai-execution-implementation.md`;
this document and PR31 remain the docs-only contract handoff.
