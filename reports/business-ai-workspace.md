# Business AI workspace — implementation checkpoint

E1 frontend implementation is active on `feat/business-ai-workspace`,
[draft PR33](https://github.com/Adanmohh/codeg/pull/33). First isolated type/report
checkpoint `2bade5c4` is committed and pushed, based on
accepted main `9e61fe672`. Root accepted PR31 and lifted the internal-only gate;
this report does not claim an implemented backend or runtime acceptance.

Current resume: root accepted PR29 and published main
`f988700db6975364d35b80125af12bfe2d5baac3`, including the exact narrow worker
tsconfig correction `91c98d469`. Root reports 109/109 scoped frontend tests and
normal `tsc --noEmit --incremental false` passing on that integrated source,
without this E1 WIP. These are independent results, not this worker's E1 gates.
Root explicitly authorized preserving/committing the existing E1 WIP before
additive main integration. That checkpoint retains the known three source type
errors and 40 formatting diagnostics; 18 stream tests pass. The alias-correcting
hook candidate is under independent review, so the denied native patch remains
pending and imports remain unchanged.

The wire authority is
`docs/contracts/business-ai-execution.md` at
`3f164c2a989cd08a523e51f1e47559c15a48c0ef`, SHA256
`e2cf8304fb5eababce79961da57061e244f2567eae11b75b88dbfcbccc6a114d`.
GitHub contents API independently returned blob
`ec22db669331e03c8d1d6fb4d312896924955853`. The report-only successor is
`cab3b27eaf241062a56532b5cdfe885f7e648fc9`; merge is `3ddb8f819964fdba006d96ad8221a0436fd1a2cb`.
The independent `618a3d96d` verdict closes contract questions, not runtime gates.
Compiling backend DTOs at `6195d9daf`, `src-tauri/src/business_execution/types.rs`,
and their additive contract change were read completely. These close private
asset list/get/versions and prompt result shapes. Local types reflect them;
the known response-only previewReason contract remains null/unavailable.

## Shared shell dependency and preserved handoff

PR29 is frozen at `96e0bcc174b000abdbb4f6d31f1c9c7c43f90518`, product
`60d600db0f224d44ff191490ab79bc25530ba959`. Its final report contains actual
4355 empty-editor recovery, retained prepared draft, two-user revision conflict /
explicit adoption and the 12-case EN/AR, light/dark, 390/768/1280 measured check.
The report records the failed locator/result-extraction attempts and bounded
measurement limits. Independent corrected-browser review now reports pass at
`c0c7e522600a94bcf33674f640d5f3665cffd39b`, with no further own fixture mutation.

PR29 is not in this branch's accepted base. Prepare isolated typed client and
components first; integrate accepted main after PR29 merges before wiring its
`WorkSurface` tabs/panes. Do not reproduce that shell or import unaccepted WIP.
Existing human task review remains the publication/review destination.

That dependency is now accepted as noted above; the next owned Git operation is
to preserve the WIP checkpoint and integrate that exact accepted main. No shell
source has been recreated or imported from an unaccepted branch.

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
the specified result envelopes. Tickets' compiling `6195d9daf` DTO handoff closes
prompt and private asset list/get/versions, including rediscovery of older private
version IDs. The concrete stop/terminal-write/resize response wrappers remain an
implementation handoff item. No guessed wrapper, moving latest version in review,
SSE, EventSource or global WS fallback.
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

Tickets' additive private versions/list/get and successful prompt envelopes are
grounded in the compiling `6195d9daf` transcription. Public selected-version
projection remains distinct. Stop/write/resize reply wrappers will likewise use
the actual backend transcription, not guesses.

No actual backend operation, browser flow or provider/native runtime pass is
claimed. Next: framing/byte reader, client/component glue and meaningful synthetic
tests with small checkpoints. Actual
same-workspace CLI/Design Studio and E1 ACP/PTY fixtures wait for the shared shell
acceptance plus coordinated compiling backend/port handoff. Root owns integration
and final acceptance.

## Current local checkpoint and execution block

Resume after the owner's global-hook checkpoint `f5554c92`: updated global
AGENTS and the complete docs-first.md preface were read. This worker keeps the
assigned Astra/max configuration and never/full-access policy; no model or hook
configuration was changed. Actual Git status and `gh api` now exit0. Git HEAD is
`2bade5c42b435afb9a8640e5bb319cb55cef2aab`. The GitHub commit/contents responses
resolve the backend DTO pin to `6195d9daf049b7cd16615e35b87d08c7f2870bf0`; the
returned Rust types were read completely. The live signed Vitest2.1.9 reader also
returns successful correlated evidence under docs-first-v2 for this worktree.
The old JSONL audit still contains the earlier Pre/Post records cited above;
it is not asserted to contain new workflow-admission records.

The supported test edit remains denied as `package_unresolved`. Root requested
read-only resolver diagnosis. Exact cause from `docs_first_gate.py`: the JS
import traversal at lines519–528 recognizes only dot-prefixed local imports;
otherwise `record` calls `package_name` (lines458–459), whose regex at224–226
rejects `@/lib/business/tasks`. The path is `stream.test.ts → frames.ts → types.ts`,
with that type-only import at `types.ts:1`. The real tsconfig explicitly maps
`@/*` to `./src/*`, but the hook contains no tsconfig handling. The resolver does
not recurse into Vitest's external package source at this step; Node built-ins
in its declaration imports are not the cause. No import rewrite, other editing
tool, hook/state edit or denied-patch reroute was used. A compound regex source
search received `reader_arguments_invalid`; bounded sed and simple rg reads
succeeded and supplied this diagnosis.

Allowed validation of the preserved, unchanged local code now ran:

- `pnpm exec vitest run src/lib/business-execution/stream.test.ts`: exit0,
  18/18; `resumed-baseline-stream.txt`.
- Source-only typecheck: exit2, three concrete errors in
  `resumed-baseline-source-typecheck.txt`: `client.ts:165` generic return inference
  and `stream.test.ts:92,164` JSON.stringify callback signatures. These remain
  unfixed pending supported edit admission; no compiling-source claim.
- Scoped ESLint of `src/lib/business-execution`: exit1, 40 formatting diagnostics
  in `resumed-baseline-lint.txt`. No autofix or formatter write was used to
  reroute the denied native patch.

The archive-discovery correction below is separate from those source errors. Offline guide
retrieval was repeated successfully; installed-doc corpus retrieval still exits3
because rebrand.db is absent. No corpus/dependency/global mutation was made.

The following paragraph records the earlier blanket-denial period, now superseded
for Git/tests by the successful commands above; the alias edit denial is current.

After the pushed type checkpoint, isolated `client.ts`, `content.ts`, `frames.ts`,
`protocol.ts`, `reader.ts` and `stream.test.ts` were added under
`src/lib/business-execution/`. They are uncommitted and unvalidated. They implement
closed injected HTTP operations, byte-bounded UTF-8 NDJSON validation, abort/
detach ownership, bounded immutable size/hash checks and disposable inert Blob
URLs. No existing client/shell registration or runtime fixture is wired.
The focused tests are synthetic byte streams, not backend or browser acceptance.

During this work the active global hook changed to docs-first-v2. Supported
individual cat/sed reads work, and its signed installed-doc reader successfully
read Vitest2.1.9 `dist/index.d.ts` ranges1–80,180–198,550–575. Normal source/type
reads also covered the mock interfaces at the installed @vitest/spy2.1.9.
Nevertheless, `pnpm exec vitest run src/lib/business-execution/stream.test.ts`
is rejected before execution as `shell_unproven_use_patch_or_reader`.
`git rev-parse HEAD` receives the same rejection. A narrow correction of the test's
JSON.stringify callback receives `package_unresolved` despite the signed reads.
The focused retry after an additional successful signed TypeScript5.8.3
`lib.es5.d.ts` JSON/stringify declaration read (1150–1187) receives the same
rejection. Separate `git status --short --branch` is also rejected as
`shell_unproven_use_patch_or_reader`; current Git state cannot be freshly asserted.
That correction has not applied, and the new test must not be called compiling.
Earlier multi-command/regex reads were rejected as `shell_unresolved`; separate
supported reads succeeded. One report patch used stale context and was rejected
without changes before this corrected patch.

Read-only inspection of the active launcher and its documented reader entry
identified the supported signed-reader command; no hook/config/state was modified
or bypassed. The supported validation/Git runner is requested from root while
safe source reading continues. No alternate execution path, new target, install,
fixture change or passing-test claim is used to work around these denials.
Initial source-only typecheck/lint results above apply only to pushed `2bade5c4`;
the normal full typecheck limitation remains separate. New local logs using .log
are ignored by the inherited rule and still need explicit owned-artifact staging
when commit execution is available. PR29 stays frozen at96e0bcc1/product60d.

## Bounded presentation follow-up

The existing RichComposer, its editor configuration and reference node/view/badge
were read completely. Its restored reference badges are inert spans; that alone
does not claim or grant file access. No global composer change is justified from
badge hydration. E1 can omit reference search and supply only explicit task/asset
input selection alongside the existing text editor.

`ai-elements/message.tsx` and `markdown-link.tsx` were read completely. The
Message/MessageContent wrappers can provide presentation without the inherited
conversation container. MessageResponse always installs the shared MarkdownLink
after caller component overrides. That renderer attaches file-reference actions
and the shared link-safety opener; it must not be adopted unchanged as the
version-authorized document or E1 message reader. Lower presentation with explicit
scoped content actions is the integration seam. This is source mapping, not a new
runtime security finding or a claim that a rendered browser was tested.

The complete inherited TerminalView and `lib/terminal/write-queue.ts` were also
read. The queue already serializes sends and never retries a failed batch, but
continues subsequent queued input; the E1 adapter must dispose that queue when a
receipt is uncertain and expose explicit reconciliation. TerminalView's spawn,
global output subscription and canceled-spawn kill remain outside the scoped
view adapter. Existing fit/font/keyboard behavior can be retained without those
host operations. Installed @xterm/xterm6.0.0 and @xterm/addon-fit0.11.0 both identify
source commit `f447274f430fd22513f6adbf9862d19524471c04`; MIT license and the fit
declaration were read. No terminal component or third-party source port has yet
been written.

## Separate compiler discovery correction

Root explicitly assigned a bounded normal-tsc fix while the supported source
patch remains pending. `tsconfig.json` now excludes only
`reports/business-integration-review`, the retained immutable reviewer evidence
directory. Its six TS/TSX artifacts were meant to be copied into separate source
archives: their relative `./workspace`, `./test-fixtures` and `./vitest.config`
imports do not resolve beside the committed report copies. The artifacts and
their bytes remain untouched. This is narrower than excluding all reports or
replacing the normal include list with a source-only configuration.

Grounding: installed TypeScript5.8.3 `lib/typescript.js` getConfigFileSpecs at
42965–43044 and getFileNamesFromConfigSpecs at43545–43621 were read. Official
`gh api` resolves v5.8.3 to `68cead182cc24afdc3f1ce7c8ff5853aba14b65a`; returned
`src/compiler/commandLineParser.ts` at3067–3180 and3886–4002 was read. Exclusions
filter wildcard discovery; they do not change compiler options or remove
explicit files. Installed `tsc --help --all` confirms --listFilesOnly exposes
the real program inputs and --noEmit still performs checking. The signed reader
declined the large compiled source as source_unreadable; ordinary bounded source
reads and the immutable official response provide the manual config grounding.
The config patch was admitted with the documented manual-grounding reminder,
not rerouted from a denied supported edit. No TypeScript compiler code is copied.
The existing Codeg-derived config base is blob
`f651e558c3708e7f939535afbb9a30e87c8c3988` at the branch's accepted base.

Actual normal `pnpm exec tsc --noEmit` before/after logs are retained as
`normal-typecheck-{before,after}-archive-exclusion.txt`. Before: exit2, nine
archive import errors plus three E1 source errors. After: exit2, exactly the
three E1 source errors. **No normal typecheck pass is claimed.**

The real compiler --listFilesOnly runs both exit0. A file-set comparison exits0
and records3789→3783 files, exactly the six report copies removed, no additions,
all1211 src inputs unchanged, and both `next.config.ts` and `vitest.config.ts`
retained. See `tsconfig-file-set-check.txt`; raw lists stay in the owned ignored
`.build/e1-tsconfig-{before,after}-files.txt`. No test include, TypeScript strict
option, source import, lockfile, framework config or archive source changed.

## Presentation implementation checkpoint

The next component glue uses the accepted existing workbench's task/session/file
surfaces. There is no additional navigation system or client-selected tenant.
Private session reads require the actual original-operator capability before
loading; a member sees only selected files in the existing task deliverable.

| Surface | Existing implementation and E1-specific handoff |
| --- | --- |
| Prompt beside the task | RichComposer's once-only defaultText, onChange/onSubmit, IME-safe shortcut and onReady callbacks. Omit referenceSearch; exact task/asset references are separate explicit selections. Preserve unsent text across locale, viewport and visible-pane changes. Pending/uncertain submission retains the exact accepted draft for receipt recovery; a prompt receipt is not a completion. |
| Persistent conversation | Message/MessageContent presentation with closed messageId/part and tool state from the authenticated stream. Snapshot/reset replaces stream state without resetting the unsent composer. History stays paged; reconnect never repeats a start/prompt. Scope/generation loss detaches and clears inaccessible private content. |
| Interactive terminal | Adapt TerminalView's existing xterm fit/font/keyboard and single-flight write queue to injected session-bound operations. No spawn/kill/global subscribe on mount or unmount. On ambiguous write, dispose the queue before subsequent buffered input can send, then require receipt reconciliation. Closing a surface detaches only. |
| Documents and versions | Existing file/artifact framing receives immutable asset/version IDs and authorized content handles. Private versions are listed only through assets/versions; task reviewers use the distinct selected-version get/content projection. Explicit import and submit remain separate actions with exact task/version CAS and audience confirmation. |

The complete MarkdownDocumentPreview and HtmlPreview were read to resolve the
managed-reader seam: the former has filesystem-relative image reads and link
opening; the latter loads local resources and permits an opt-in script mode.
Neither complete container fits the selected-version reader. Reuse framing and
lower presentation with authorized bytes; never pass a fake fileDir/rootPath or
expose legacy file APIs. The initial escaped text reader and bounded download
remain honest while an office/deck preview capability is unavailable.

Existing `lib/terminal/write-queue.test.ts` was read and run for this reuse seam:
9/9 pass, exit0, `borrowed-terminal-queue-baseline.txt`. It verifies at-most-one
send, dropped failed batches and disposal discarding buffered work. This is an
unchanged inherited-helper baseline, **not** an E1 receipt/PTY/runtime pass.
The 18 E1 stream baseline tests were not redundantly rerun.

Checkpoint source/NOTICE/report diff whitespace check exits0. Including the raw
new Vitest log makes diff --check exit2 for its final blank line; the runner's
raw output is retained, rather than silently rewritten as validation evidence.

Focused implementation tests still required after admitted edits: exact pending
prompt preservation/reconciliation without replay; stale client/session response
discard; abort and immutable byte/hash checks before Blob delivery; URL disposal
on revocation/close; explicit terminal queue freeze after uncertain receipt;
member selected-file reads without private sibling/session calls; same-scope
EN/AR and narrow-pane draft retention. Actual CLI ACP/PTY/generated-file flows
wait for a coordinated backend fixture. No current browser, export, process or
fixture was mutated for this planning/checkpoint work.
