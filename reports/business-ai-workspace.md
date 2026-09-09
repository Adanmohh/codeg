# Business AI workspace — implementation checkpoint

E1 frontend implementation is active on `feat/business-ai-workspace`, based on
accepted main `9e61fe672`. Root accepted PR31 and lifted the internal-only gate;
this report does not claim an implemented backend or runtime acceptance.

The wire authority is
`docs/contracts/business-ai-execution.md` at
`3f164c2a989cd08a523e51f1e47559c15a48c0ef`, SHA256
`e2cf8304fb5eababce79961da57061e244f2567eae11b75b88dbfcbccc6a114d`.
GitHub contents API independently returned blob
`ec22db669331e03c8d1d6fb4d312896924955853`. The report-only successor is
`cab3b27eaf241062a56532b5cdfe885f7e648fc9`; merge is `3ddb8f819964fdba006d96ad8221a0436fd1a2cb`.
The independent `618a3d96d` verdict closes contract questions, not runtime gates.

## Shared shell dependency and preserved handoff

PR29 is frozen at `96e0bcc174b000abdbb4f6d31f1c9c7c43f90518`, product
`60d600db0f224d44ff191490ab79bc25530ba959`. Its final report contains actual
4355 empty-editor recovery, retained prepared draft, two-user revision conflict /
explicit adoption and the 12-case EN/AR, light/dark, 390/768/1280 measured check.
The report records the failed locator/result-extraction attempts and bounded
measurement limits. Independent corrected-browser review is separate.

PR29 is not in this branch's accepted base. Prepare isolated typed client and
components first; integrate accepted main after PR29 merges before wiring its
`WorkSurface` tabs/panes. Do not reproduce that shell or import unaccepted WIP.
Existing human task review remains the publication/review destination.

4355 remains PID85232, export `.build/business-intake-ui-recovery-60d600db0`;
`business.html` SHA256
`6fd453d6e8153f16119f882fe3766644f3dbd2085e5956ebc6dee2391ff1f65f`.
Its 27-asset manifest is
`6d0ee0ca8c48350589dd94a0b70a1eff5f873acb7e8fc60033a64385d66adbc3`.
Own `intake-recovery-a4355` / `intake-recovery-b4355` remain intact. No new
listener/export/browser is created for E1. User-inspected `intake-worker4354`,
all 4354/4350/4353 sessions/exports and earlier fixtures are unchanged.
The untracked paused `reports/visual-workspace.md` is preserved and excluded.

## Source to component mapping

Inherited source license: Codeg v0.30.4, Apache-2.0,
`6f6bd648b206412644842a98d9ffeebf57292bed`; retain the original LICENSE and
all NOTICE additions. The inspected application snapshots below are immutable
Codeg-derived source at `60d600db0`, also recorded in the PR29 final report.
No AGPL/GPL, proprietary Edublend or dependency implementation is copied.

| Existing source | Exact reuse / authority boundary |
| --- | --- |
| `src/components/business/workbench.tsx`, blob `8d46bb7a9ada8d0e25e89da7dc00a03eff8b9f0b` | PR29 dependency: keyed mounted `WorkSurface` content and split/stack geometry. Add task/session/library surfaces after merge; closing a session tab detaches, never stops a process. |
| `src/lib/business/client.ts` and accepted business session | One closure owns the bearer, clears it and aborts pending work on disposal; explicit HTTP and native branches. E1 must extend this lifecycle, never read legacy token storage or infer native operator authority from a personal credential. |
| `src/components/chat/composer/rich-composer.tsx`, blob `7d1a646bc1bf0ad0692dd36974f4799f431341f8` | Reusable editor with explicit text/submit/focus callbacks. Omit host reference search and arbitrary attachments; prompt inputs are exact closed task/asset references. Keep unsent text in memory. |
| `chat/chat-input.tsx`, blob `a8ce461e35edb6bbb5ef74fa4faec3def6bf938f`; `chat/message-input.tsx`, blob `0e228f1e92e297cea0a3b4882c65e5997f578baf` | Existing composition/button behavior is guidance. Do not mount the full wrapper: skills/file/session reference queries and optional localStorage drafts have different authority. `useShortcutSettings` itself is local personal storage, not a host API. |
| `conversations/conversation-detail-panel.tsx`, blob `0e138f15401229526a7fa9a158d8ab3b13d21a39`; `message/message-list-view.tsx`, blob `7b8825ab5b40a83f69dbbebb264db16ba85297e7` | Rich conversation presentation exists. The containers use global numeric conversation/runtime stores and ACP actions; wrapping their context does not isolate authority. Reuse lower presentation with closed E1 message/tool projections. |
| `terminal/terminal-view.tsx`, blob `ffe2fa67e6c9491198438176a62f0c900e44afea`; `terminal/terminal-panel.tsx`, blob `fca89d1036a40b115c397c2dc744c1857ed4ab9b` | Reuse installed xterm fit/font/layout behavior through injected scoped write/resize/event operations. Original TerminalView spawns on mount and subscribes to global terminal IDs; it cannot be mounted unchanged. Terminal remains LTR in RTL. |
| `message/reply-artifacts.tsx`, blob `65474f8c8a5af35644c6ed9a3adf9e0341890dd6`; `files/office-preview.tsx`, blob `d13764842eb2ef17f75f3566899f9c49f2109d53` | Existing cards/file hints and preview framing are presentation references. Hints are not managed files. OfficePreview starts a path-based watch process with a different sandbox; do not expose it to task readers. |
| `src/lib/transport/web-event-stream.ts` at accepted main | Reuse attachment ownership, explicit snapshot/replay and disposal concepts. E1 uses authenticated POST NDJSON, not this global WebSocket transport or its numeric sequence IDs. |
| `src/lib/business/tasks.ts`, `business/task-detail.tsx` | Extend the accepted task-owned deliverable projection with exact selected `assets` and its authorized get/content routes. Keep the current human review/CAS; do not add a second approval endpoint. |

Source ranges/limits remain explicit in PR29: the rich conversation container,
MessageListView and app workspace layout were inspected at their authority and
render seams, not claimed audited line-for-line. New adapted hunks will be read
completely and attributed before use.

## First usable E1 flow and integration needs

An actual original operator opens a human task, chooses a genuinely ready profile
and starts or continues a persistent chat/terminal beside the task. Explicit prompt
inputs disclose only selected permitted task/asset versions. Ordered events,
history, detach/reconnect and durable receipts keep uncertain outcomes visible;
no automatic repeat prompt/start/terminal write. Generated files are imported from
backend-owned output IDs, retained as versions and explicitly submitted to the
current task audience. A named member reviewer reloads only selected published
metadata/bytes and uses the existing human review operation.

The accepted wire closes NDJSON frame limits, reset/history and authenticated
bounded content/Blob handling. The initial client can type all exact inputs and
the specified result envelopes. Tickets' compiling DTO handoff must supply the
remaining concrete JSON wrappers for prompt/stop/terminal-write/resize and private
asset list/get, including rediscovery of older private version IDs. No guessed
wrapper, moving latest version in review, SSE, EventSource or global WS fallback.
This is an implementation transcription dependency, not a request to reopen the
accepted authority contract.

Native uses only confirmed dedicated operator commands after backend handoff;
personal native access stays unavailable. E2 isolated member execution and E3
account snapshots are separate work. No account_snapshot input is enabled in E1;
no calendar, connector, launch or completion placeholder counts as delivered AI.

## Grounding and validation

Complete AGENTS/FOUNDING/ORCHESTRATOR/DECISIONS/STATUS and the accepted E1 contract
were read, with the accepted-main STATUS amendment after integration. Separate
`cat node_modules/react/package.json` and `cat src-tauri/Cargo.toml` preceded work.
Live audit `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl` records
PreToolUse and PostToolUse for this worktree/session
`01a084ee-12a8-7833-ace9-f3f4985ba926`, timestamps 1788950051/1788950059,
exit0. Hooks remain enabled.

Offline code-context guide exits0 using the existing rag-skills venv and
HF_HUB_OFFLINE=1. Relevant retrieved rules: strict TypeScript and gating data
loading as well as visible controls; locked sandbox for untrusted previews.
Cross-project delegation/context7 defaults do not override the owner's no-worker
and gh-api rules. Installed-doc retrieval exits3: `rebrand.db` is absent. No
ingest/install/global change; direct installed source/types provide grounding.
React19.2.4 and TypeScript5.8.3 `lib.dom.d.ts` ReadableStream/TextDecoder/SubtleCrypto/
URL declarations are read. Existing business client/session tests and the Codeg
event stream source are read before adapting their seams. No new dependency.

The first product checkpoint adds only isolated closed E1 wire types. No existing
client, session, shell, task type, backend or UI route is changed. Full
`pnpm exec tsc --noEmit` exits2 on nine unchanged reviewer-archive relative imports
under `reports/business-integration-review/ui-{191f69b3,a58c004c,dc14e83b}`; exact
diagnostics are retained in `business-ai-workspace-evidence/initial-typecheck.txt`.
The normal project configuration and imported evidence remain untouched.

The separate source-only config checks every `src` TS/TSX file and its tests:
`pnpm exec tsc --noEmit --project reports/business-ai-workspace-evidence/tsconfig.source.json`
exits0; scoped E1 types ESLint exits0. Logs are retained beside that config.
This is not a full-command pass. Pure type transcription has no behavioral test
claim; parser/client/component regressions follow their implementation.

Tickets has now specified the additive private versions/list/get and successful
prompt envelopes in coordination; the immutable compiling DTO pin is still
pending. Public selected-version projection remains distinct. Stop/write/resize
reply wrappers will likewise use the actual backend transcription, not guesses.

No actual backend operation, browser flow or provider/native runtime pass is
claimed. Next: framing/byte reader, client/component glue and meaningful synthetic
tests with small checkpoints. Actual
same-workspace CLI/Design Studio and E1 ACP/PTY fixtures wait for the shared shell
acceptance plus coordinated compiling backend/port handoff. Root owns integration
and final acceptance.
