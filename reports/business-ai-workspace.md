# Business AI workspace — implementation checkpoint

Current branch: `feat/business-ai-workspace`; draft
[PR33](https://github.com/Adanmohh/codeg/pull/33). Accepted shell base is
`f988700db6975364d35b80125af12bfe2d5baac3`. Guarded session entry is pushed as
`45d7fb2f5`; this checkpoint adds the private task file workflow described below.
Conversation product checkpoint
`b125aec9e5703508ac7f6948d0d2db134534c90d` is pushed. This next checkpoint adds
the original-operator browser entry, explicit start/continue and guarded session
tabs to that existing workbench; it does not enable a backend or prove a runner.

Task detail opens AI workspace beside the retained human draft. Profiles and
saved sessions load only after that explicit entry and only for actual
`legacyOperator`, never member/owner role or native fallthrough. Starting requires
a deliberate ready original-operator profile selection and captured task/profile
revisions. Continue uses the selected saved session revision. Both requests retain
their original operation ID during uncertainty; missing receipt never relaunches.
The pending pane cannot close before receipt resolution. Existing sessions attach
through the committed conversation component without spawn on mount. Closing a
resolved pane detaches its child scope and does not stop a process. Native and
terminal entry remain explicitly unavailable, with no enabled fallback control.

The new glue reuses accepted `business/workspace.tsx`, `workbench.tsx`,
`task-detail.tsx` TaskFrame close/discard registration and `business/ui.tsx`,
at the exact source blobs listed in the ledger below. Private result parsing now
also validates the closed ProfilesResult and task-bound session page. The tests
use the real BusinessClient/Workbench/TaskDetail/conversation with an in-memory
synthetic fetch boundary; no listening fixture or production response is changed.

| Session-entry gate | Result; evidence under business-ai-workspace-evidence |
| --- | --- |
| New session entry/receipt/authority/retained-task-draft component tests | 12/12, exit0; session-entry-tests-first.txt |
| Existing workbench/workflow/session/provider-isolation tests | 33/33, exit0; session-entry-shell-regressions.txt |
| Normal `pnpm exec tsc --noEmit --incremental false` | exit0; session-entry-typecheck-final.txt |
| Scoped ESLint including new tests and affected task/workspace files | exit0; session-entry-lint-final.txt |

The initial render-prop guard implementation triggered React's render-time ref
lint. The final implementation reads the guard only during a close event, using
the existing registered-close seam; no lint suppression or hook maintenance.
Earlier diagnostic logs remain. TypeScript, React and test-library installed
references are unchanged from the grounding recorded below.

The private Documents & assets work area is now mounted through the same guarded
tabs from the task AI workspace or a saved conversation. Search and opaque pages
load actual typed retained-file metadata; checking session output is explicit and
does not import it. Exact output ID/revision and a proposed title define the import.
Adding a version also rechecks current addVersion capability and the captured
asset revision. The title bound is240 Unicode characters from validation.rs at
`d0d56a36399d81be23787891177752022c91edf6`; no caller path/hash/provenance is sent.

An import retains a private immutable version and does not select it for review.
Current assets/get metadata gates escaped Markdown/text preview and verified inert
Blob download; exact-version pagination does not silently follow a moving latest
pointer. Unsupported/large text and office/deck previews show an explicit gap.
Selecting1–16 versions starts a separate submission draft. Review fetches current
task and every exact version, compares the retained hash/size/type, then shows
current domain audience, owner/assignee/reviewer, status and task revision. The
human checkbox confirms that exact selection/note/audience. Changing the note or
selection invalidates it. The existing task writer remains the only publication,
review and status authority; this UI does not call legacy submit/progress/review
to manufacture a completed result.

Import uncertainty keeps the captured request and permits receipt lookup only.
Once that receipt is confirmed, it recovers metadata through the contract's exact
same-operation import replay; it does not guess which ID the opaque receipt target
represents. Pending/missing/uncertain receipts never reimport. Submit uncertainty
uses the original receipt plus tasks/get to locate its exact historical deliverable;
it does not repeat assets/submit. This target mapping is read from publication.rs
atd0d56a36. Known failed receipts require explicit selection review. Behavior for
backend refusals that do not retain a failed receipt still needs the operational
handoff: the UI conservatively keeps unresolved intent, without inferring a retry
from a missing receipt. Private access denial clears file content/actions; tab
closure aborts reads and disposes verified handles. Same-scope locale/tab changes
retain task, prompt and submission drafts in memory.

| Managed-file gate | Result; evidence under business-ai-workspace-evidence |
| --- | --- |
| Closed private asset parser | 11/11, exit0; private-assets-parser-first.txt |
| File/import/submission components plus integrated task/session tabs and existing conversation | 49/49, exit0; managed-assets-final.txt (17 managed,12 entry,9 conversation,11 parser) |
| Existing public selected-file reader and cold provider isolation | 15/15, exit0; private-assets-public-boundary-regressions.txt |
| Normal `pnpm exec tsc --noEmit --incremental false` | exit0; private-assets-typecheck-final.txt |
| Scoped ESLint over E1 library/components and affected BusinessWorkspace | exit0; private-assets-lint-final.txt |

The first managed component run was13/14: Testing Library's default whitespace
normalizer did not match the exact multiline preview string. Installed DOM10.4.1
matches.js was read; the existing public-reader test's exact PRE.textContent
assertion now verifies every byte of displayed text. The isolated locator recheck
passes1/1, then all17 final cases pass; initial raw evidence is retained. Initial
type errors were the one-value previewReason tuple and test locale inferred as
string; both now use the actual closed types. No compiler/lint options or behavior
assertions were weakened. All data is synthetic in-memory HTTP; the PPTX case is
version/selection metadata, not generated slide rendering or a runtime fixture.

Sources: accepted Codeg business/ui.tsx and WorkSurface/TaskFrame seams atf988;
owned fee6a939e7f693d95896a44db02147a2c34383e3 selected reader; owned b931b0f57
frozen prompt receipt and45d7fb2f5 pane lifetime; accepted contract3f164c2a and
Rust6195d9daf private DTOs, plus d0d56a36 publication.rs/validation.rs. The current
PR32 head855cd3b0cf9147e6fc89d3c4a5e1ec8e59169d80 still has the exact previously
read types blob47ca21b002c9c1efb898e38e0c920e9ca3456c70 (11,207 bytes), verified via
gh api. No stop/write/resize result wrapper has appeared. Its scan-fence report
is separate backend evidence, not this worker's compilation/runtime claim.

Remaining implementation: the existing xterm presentation after exact
write/resize/stop result wrappers are published. Actual E1 API/ACP/PTY/browser,
Design Studio and export gates await a coordinated backend and build window.
The backend `d0d56a36399d81be23787891177752022c91edf6` asset handoff is source only,
explicitly uncompiled by its owner; no runtime or native acceptance is claimed.
The manual offline code-context guide was rerun with the existing cached model;
installed-version grounding and safe untrusted-content guidance apply. The
cross-project content-factory/delegation rules do not apply to this task. No new
dependency corpus was ingested; the previously missing rebrand docs corpus remains
an explicit coverage limit. React19.2.4, TS5.8.3, Testing Library16.3.2/DOM10.4.1,
Vitest2.1.9 and the previously read Blob/WebCrypto types are the installed API
grounding. Owner-removed enforcement remains removed.

Frozen4354/4355, all old exports/browsers/targets and the paused visual report
remain untouched. No dependency, lockfile, Rust, auth or engine change is made.

## Earlier checkpoints and source evidence

Latest completed public-file checkpoint:
`fee6a939e7f693d95896a44db02147a2c34383e3`, pushed to draft PR33. The next
compiling component adds persistent chat presentation over the existing E1
attachment client. It reuses Codeg Message/MessageContent and RichComposer,
without mounting the legacy conversation provider or Markdown file opener.
At that earlier checkpoint the private profile/session entry and guarded tab
registration were still in progress; their current source gate is recorded above.

Its authenticated attachment uses the exact session/generation and opaque cursor.
Unexpected EOF requires explicit session revalidation/reconnect, never prompt
replay. Snapshot reset updates the bounded recent-message cache without remounting
the actual prompt editor. History requests40 parts per page through the accepted
opaque cursor; a160-part live window reports trimming and exposes saved history.
Duplicate cursor/part delivery is deduplicated, including snapshot/history rows.
Tool state is a closed projection; terminal bytes are not rendered as chat/task
text. Revoked status, authority detach or denied current binding aborts the owned
view, clears private content/draft and prevents buffered delivery. Ordinary tab
detachment does not stop, continue or launch the process.

| Conversation component gate | Result; evidence under business-ai-workspace-evidence |
| --- | --- |
| Conversation state + real message/composer + existing prompt tests | 22/22, exit0; conversation-component-final.txt |
| Normal `pnpm exec tsc --noEmit --incremental false` | exit0; conversation-typecheck-final.txt |
| Scoped ESLint | exit0, no diagnostics; conversation-lint-final.txt |

The first state negative control is retained in conversation-negative-control.txt:
duplicate snapshot/history parts initially failed1/4; all four now pass. Earlier
type/lint diagnostics are preserved: tuple inference, a test-only Array.at outside
the unchanged ES2020 lib, an effect dependency and synchronous history loading
resets. The latter now derive loading/error from the exact cursor-associated
response; no lint rule or compiler library was disabled/upgraded. These remain
in-memory synthetic tests, without a provider, exported UI, browser or native pass.
No existing fixture/preview/target was changed and no build window was taken.

The prompt checkpoint `b931b0f57` adds the inherited RichComposer with
explicit E1 prompt receipts, EN/AR copy and in-memory context selection. Its nine
focused tests pass, as do normal TypeScript and scoped ESLint. At that checkpoint
the component was not mounted in the workbench; the current entry now mounts it.
No operational AI or browser acceptance is claimed. The preceding parent/client integration is pushed as
`ca3454c4bb995b48f813b024eceba2ee31a3d6ac` on draft PR33. No export, native/Rust
build, provider, credential or existing fixture/browser was touched.

The common frontend Task Detail now has required `Deliverable.assets` and the
six-field `PublishedAssetRef`, matching the complete Rust task types read through
gh api at `d0d56a36399d81be23787891177752022c91edf6`. Historical text submissions
use an empty array. The execution types reuse this common type instead of an
isolated substitute. This source handoff has no routes or fixture, and the
backend owner explicitly reports it uncompiled while Farha owns the build window.
There is no claim that the running B previews return this new field.

Prompt glue reuses RichComposer at accepted
`f988700db6975364d35b80125af12bfe2d5baac3`, blob
`7d1a646bc1bf0ad0692dd36974f4799f431341f8`, and the explicit frozen-operation
pattern in `business-intake/candidate.tsx`, blob
`daef5d5f8e8efdb07ba04cf979afa89d3b3f917f`. It sends once on explicit Enter/button
intent, retains exact text and selected version labels while pending/uncertain,
and looks up only the original operation. A missing receipt does not unlock a
resend. A confirmed receipt is acceptance, not agent completion or task review.
Unsent reference revision drift requires a new explicit selection. Locale and
pane visibility keep the actual editor mounted; unmount aborts local delivery.
Absent prompt capability permits no send. No host mention/file API is mounted.

| Current component gate | Result; evidence under business-ai-workspace-evidence |
| --- | --- |
| `pnpm exec vitest run src/components/business-execution/session-prompt.test.tsx` | 9/9, exit0; session-prompt-final.txt |
| Prompt plus affected frame parser checkpoint | 26/26, exit0 (8 prompt + 18 existing stream); session-prompt-after.txt |
| `pnpm exec tsc --noEmit --incremental false` | exit0; session-prompt-typecheck-final.txt |
| Scoped ESLint, component/copy/frame/task types | exit0; session-prompt-lint-final.txt |

The pending-reference negative control fails against the pre-fix component
because a newer parent reference replaces the submitted label; its exact failure
is retained in prompt-reference-negative-control.txt. The nine final tests use
the real RichComposer with an injected in-memory transport, not intercepted
production responses. React19.2.4, @types/react19.2.13, Testing Library16.3.2,
Vitest2.1.9 and TypeScript5.8.3 installed source/types were read before edits.
Normal Git status and immutable gh-api reads both pass in this resumed session;
owner-removed enforcement hooks remain absent and are not restored.

The next compiling checkpoint mounts the task-selected file reader in the
accepted workbench. Existing Task Detail cards open exact task/deliverable/asset/
version selections as stable WorkSurface tabs. Returning to a human editor retains
its unsaved draft. The selected route rechecks current access before metadata or
content; it never calls private assets/list, versions, sessions or legacy paths.
The public parser refuses extra private fields and foreign asset/version/
deliverable identities, and captures caller selection before asynchronous reads.
The task manifest's hash/size/type must match returned metadata before a read can
be offered. This extends source only; no preview export was replaced or started.

The reader uses escaped text for plain/Markdown content up to1MiB. Other formats
or larger text show an explicit preview gap and a separately authorized download
up to the contract's50MiB bound. SHA-256 and exact byte/header validation precede
content delivery. Download uses an inert binary Blob and an explicit Save file
link, never a credential URL or filesystem opener. Close/version change aborts
pending reads and disposes handles; a denied read clears displayed content.
The slide-format unit case is a synthetic MIME/byte handling test, not a generated
PPTX/Office-renderer or real browser proof. Native E1 remains unavailable.

| Selected-file gate | Result; evidence under business-ai-workspace-evidence |
| --- | --- |
| Published projection plus business parent client | 18/18, exit0; published-after.txt |
| Real reader/Task Detail/Workbench components with injected synthetic HTTP | 7/7, exit0; published-component-final.txt |
| Affected workbench/workflow plus execution library/parent tests | 88/88, exit0; published-integrated-regressions.txt |
| Normal `tsc --noEmit --incremental false` | exit0; published-integrated-typecheck-final.txt |
| Scoped ESLint | exit0, no diagnostics; published-integrated-lint-final.txt |

The unchanged b931 public client fails11/12 new projection cases; exact negative
evidence is published-negative-control.txt. The first component typecheck found a
test fixture using public SubmittedBy fields for the different TaskActor type.
It now uses the actual task `id/displayName/kind`, without a production or assertion
change. Its initial diagnostic and an unused test parameter lint warning remain
in published-integrated-typecheck.txt and published-integrated-lint.txt. The
corrected seven component tests reran. Installed @types/node25.2.2 Blob/WebCrypto
types ground the test-only Node implementations replacing missing jsdom APIs;
this is not browser proof. Lucide0.563.0 exports and existing Button/Action props
were read. No dependency, lockfile or global configuration changed.

The earlier transport checkpoint passes the normal TypeScript check and scoped lint, with
48/48 client, content and stream tests passing. After the owner confirmed complete enforcement-hook
removal and restarted this saved session, the first normal Git status and gh-api
read both exited0. The preserved type fixes then applied through the normal
native patch tool: the result generic is explicit and JSON.stringify receives
only the array item, not the map callback's index. Existing assertions remain.
Prettier3.8.1, using the unchanged repository configuration, corrected the earlier
formatting diagnostics in the seven-file E1 directory only.

The initial correction is pushed as c781c9d5a29dcf29c4fbe64ca1f510aa8f0e004d.
Its raw evidence is `business-ai-workspace-evidence/resumed-fixed-stream.txt`
(18 tests, exit0), `resumed-fixed-typecheck.txt` (normal --noEmit --incremental
false, exit0) and `resumed-fixed-lint.txt` (scoped ESLint, exit0). Source diff
whitespace check exits0. No runtime, browser, provider, native or full-app pass
is implied. The subsequent ownership regressions and current gates follow below.
No static export, Rust compile or existing browser/fixture action was taken.

Documentation grounding continues manually: installed TypeScript5.8.3 JSON
overloads, Vitest2.1.9 types, Prettier3.8.1 help/configuration and relevant Codeg
source were read. Official gh-api resolves Vitest v2.1.9 to
`c9e59a089d94642eea29a43f2ee1986a5afb99c6`. Owner replacement AGENTS forbids
restoring enforcement hooks without an explicit request. All prior alias
candidate/activation work is canceled; the historical denials below are not a
current implementation block. This worker changed no global hook/config/state.

Current response-ownership changes capture stream binding and content metadata
before asynchronous reads. A changed caller selection cannot authorize a different
returned session/version. Scope closure cancels readers and prevents late delivery;
401 also revokes all content handles. Each handle's disposal is idempotent. Local
abort, closure and deadline are explicitly `ExecutionInterrupted` with the existing
safe transport_unavailable reason, distinct from a server-reported cancelled
operation. No write is replayed automatically. Unsupported executable preview is
refused before transport; verified download remains an inert binary Blob.

| Current bounded gate | Result and committed evidence |
| --- | --- |
| `pnpm exec vitest run src/lib/business-execution` | 48/48, exit0; client-content-stream-after.txt |
| `pnpm exec tsc --noEmit --incremental false` | exit0; client-content-typecheck.txt |
| `pnpm exec eslint src/lib/business-execution` | exit0; client-content-lint.txt |
| Source diff whitespace | exit0 |

All paths above are under reports/business-ai-workspace-evidence. The unchanged
c781 transport fails10 of the30 new tests after correcting the test harness;
client-content-negative-control.txt retains that exit1. The first attempt also
contained a Vitest2-incompatible assertion and an asynchronously handled failed
assertion; client-content-before.txt is retained, not counted as clean product
evidence. Installed assertion types resolved the harness; no product assertion
was weakened. These are synthetic in-memory tests, not backend/browser acceptance.
The new tests cover held selections, actual Unicode/hash bytes, malformed headers,
truncation/overflow, partial-read abort, URL cleanup, error privacy, unavailable
account references, stream deadline lifetime and zero automatic prompt replay.

The latest read-only gh-api check resolves PR32 to
b15ded12cd24f557d0de9bb55b43db30b9f8bc89; its complete types.rs was reread and
has no new stop/write/resize result wrapper. Those operations are not guessed.
The admitted-body UI and parent-client lifecycle wiring are the next independent
work; no existing fixture, export, browser, Rust source/target or credential was
changed. No static/native build was started during tickets' Rust build window.

The response-ownership checkpoint is pushed as776e4bf5. The next coherent change
connects E1 to the existing BusinessClient's private credential closure. Creating
a child scope makes no request. Closing a pane scope leaves sibling work and the
parent usable; closing the business connection or receiving401 in either client
closes all child readers/handles. The factory can create a fresh scope for a new
pane without reusing a disposed client. There is no bearer getter or generic API
method. The fixed business origin, POST envelope, no-store, omitted cookies,
redirect refusal and no-referrer policy apply. Native E1 is explicitly unavailable
until dedicated backend commands are handed off; it never falls through to a
host API or browser credential. The factory does not confer execution authority:
UI private reads will additionally require the actual original-operator capability,
and the backend remains decisive on every request.

Parent-scope gates:69/69 client/transport tests in parent-scope-tests.txt;69/69
affected existing workspace/intake tests in parent-scope-existing-workflows.txt;
normal TypeScript exit0 in parent-scope-typecheck-final.txt; scoped ESLint exit0
in parent-scope-lint-final.txt. TypeScript first identified six existing typed
test doubles missing the new method. Each now refuses unexpected execution use;
no behavioral assertion was changed. One insertion initially matched the native
window mock instead of the business mock; that test-only location was corrected,
then its14 OS-mocked tests reran successfully (parent-scope-native-fixture.txt).
The intermediate typecheck diagnostics are retained. This is no actual native
or browser claim, and no execution surface is mounted yet.

E1 frontend implementation is active on `feat/business-ai-workspace`,
[draft PR33](https://github.com/Adanmohh/codeg/pull/33). First isolated type/report
checkpoint `2bade5c4` is committed and pushed, based on
accepted main `9e61fe672`. Root accepted PR31 and lifted the internal-only gate;
this report does not claim an implemented backend or runtime acceptance.

Accepted integration: root accepted PR29 and published main
`f988700db6975364d35b80125af12bfe2d5baac3`, including the exact narrow worker
tsconfig correction `91c98d469`. Root reports 109/109 scoped frontend tests and
normal `tsc --noEmit --incremental false` passing on that integrated source,
without this E1 WIP. These are independent results, not this worker's E1 gates.
The E1 WIP was committed and pushed as
`f9186504022c28f33e16e2601b6d768fa577bc3b` before merging that exact accepted
main. The sole conflict was NOTICE: the complete accepted-main file is preserved
byte for byte, followed by both exact E1 sections. At that merge all seven E1 files
were identical to f9186504; LICENSE, both dependency locks, package/configuration and
protected documents match f988700d. Verification exits0; the integration evidence
is `business-ai-workspace-evidence/accepted-main-integration.txt`.
The completed merge is pushed as `1c44d5a86d7dfa59558eb933ec1558214b61ae27`.
The original post-merge `pnpm exec tsc --noEmit --incremental false` exited2 with exactly
the three known E1 errors, no archive or additional integration errors; raw output
is `business-ai-workspace-evidence/accepted-main-typecheck.txt`. No broad suite,
export, Rust target or native build was repeated.

That historical checkpoint retained three source type errors and 40 formatting
diagnostics, now corrected above. Root had reported canonical alias candidate
`a918234a97b6786686140f833bdcfa6af53372ad694906192a24ee4dc56f702e`
passing 46 new and 56 retained regressions, but it was never installed and is now
canceled. Those were root-reported maintenance results, not this worker's product
gates. The patch was held until confirmed removal/session restart; there was no
import rewrite or alternate-route bypass.

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
The latest owner checkpoint is `3773a027a8d580bd9bac1808efdb718ae6f9e135`,
reported through `79703eeb`. Actual gh-api contents responses at both 6195d9daf
and 3773a027a identify the same types.rs blob
`47ca21b002c9c1efb898e38e0c920e9ca3456c70`, 11207 bytes. Its agreed DTOs are
unchanged. The owner reports durable admission/private bytes ready internally;
production routes, runner and managed publication remain in progress. This is
not an enabled-controls or fixture handoff. No unaccepted backend is merged here.

## Shared shell dependency and preserved handoff

PR29 was accepted at `96e0bcc174b000abdbb4f6d31f1c9c7c43f90518`, product
`60d600db0f224d44ff191490ab79bc25530ba959`. Its final report contains actual
4355 empty-editor recovery, retained prepared draft, two-user revision conflict /
explicit adoption and the 12-case EN/AR, light/dark, 390/768/1280 measured check.
The report records the failed locator/result-extraction attempts and bounded
measurement limits. Independent corrected-browser review now reports pass at
`c0c7e522600a94bcf33674f640d5f3665cffd39b`, with no further own fixture mutation.

The accepted PR29 WorkSurface tabs/panes are now integrated. No shell source was
recreated or imported from an unaccepted branch. Existing human task review
remains the publication/review destination. E1 is still isolated and not mounted;
the compiling transport and next component work use the actual merged client and
workspace. No duplicate shell is introduced.

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
| `src/components/business/workbench.tsx`, blob `8d46bb7a9ada8d0e25e89da7dc00a03eff8b9f0b` | Accepted keyed mounted `WorkSurface` content and split/stack geometry. Append task/session/library surfaces here; closing a session tab detaches, never stops a process. |
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
Historical audit `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl` records
PreToolUse and PostToolUse for this worktree/session
`01a084ee-12a8-7833-ace9-f3f4985ba926`, timestamps 1788950051/1788950059,
exit0. These timestamps do not establish a current generic Pre/Post journal.
Those records were captured while enforcement was enabled; the owner has since
removed it. The current normal-tool verification is recorded at the top.

Offline code-context guide exits0 using the existing rag-skills venv and
HF_HUB_OFFLINE=1. Relevant retrieved rules: strict TypeScript and gating data
loading as well as visible controls; locked sandbox for untrusted previews.
Cross-project delegation/context7 defaults do not override the owner's no-worker
and gh-api rules. Installed-doc retrieval exits3: `rebrand.db` is absent. No
ingest/install/global change; direct installed source/types provide grounding.
React19.2.4 and TypeScript5.8.3 `lib.dom.d.ts` ReadableStream/TextDecoder/SubtleCrypto/
URL declarations are read. Existing business client/session tests and the Codeg
event stream source are read before adapting their seams. No new dependency.

The first product checkpoint, 2bade5c4, added only isolated closed E1 wire types.
It changed no existing client, session, shell, task type, backend or UI route.
Its full `pnpm exec tsc --noEmit` exited2 on nine reviewer-archive relative imports
under `reports/business-integration-review/ui-{191f69b3,a58c004c,dc14e83b}`; exact
diagnostics are retained in `business-ai-workspace-evidence/initial-typecheck.txt`.
At that point the normal project configuration and imported evidence were untouched.

The separate source-only config checks every `src` TS/TSX file and its tests:
`pnpm exec tsc --noEmit --project reports/business-ai-workspace-evidence/tsconfig.source.json`
exited0 at 2bade5c4; scoped E1 types ESLint also exited0 then. These historical
results do not cover the later seven-file WIP. Pure type transcription has no
behavioral test claim; parser/client/component regressions follow implementation.

Tickets' additive private versions/list/get and successful prompt envelopes are
grounded in the compiling `6195d9daf` transcription. Public selected-version
projection remains distinct. Stop/write/resize reply wrappers will likewise use
the actual backend transcription, not guesses.

No actual backend operation, browser flow or provider/native runtime pass is
claimed. Framing/byte-reader WIP is preserved; corrected client/component glue and
meaningful synthetic tests continue through normal grounded edits. Actual same-workspace
CLI/Design Studio and E1 ACP/PTY checks wait for coordinated compiling backend and
fixture handoff. Root owns integration and final acceptance.

## Historical local checkpoint and resolved execution block

Resume after the owner's global-hook checkpoint `f5554c92`: updated global
AGENTS and the complete docs-first.md preface were read. This worker keeps the
assigned Astra/max configuration and never/full-access policy; no model or hook
configuration was changed. Actual Git status and `gh api` exit0; the pushed
integration head is recorded above. The GitHub commit/contents responses
resolve the backend DTO pin to `6195d9daf049b7cd16615e35b87d08c7f2870bf0`; the
returned Rust types were read completely. The live signed Vitest2.1.9 reader also
returns successful correlated evidence under docs-first-v2 for this worktree.
The old JSONL audit still contains the earlier Pre/Post records cited above;
it is not asserted to contain new workflow-admission records.

The supported test edit was denied as `package_unresolved`. Root requested
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

Earlier validation of the preserved, unchanged code ran:

- `pnpm exec vitest run src/lib/business-execution/stream.test.ts`: exit0,
  18/18; `resumed-baseline-stream.txt`.
- Source-only typecheck: exit2, three concrete errors in
  `resumed-baseline-source-typecheck.txt`: `client.ts:165` generic return inference
  and `stream.test.ts:92,164` JSON.stringify callback signatures. These remain
  unfixed at f9186504; the current corrected gate is recorded above.
- Scoped ESLint of `src/lib/business-execution`: exit1, 40 formatting diagnostics
  in `resumed-baseline-lint.txt`. No autofix or formatter write was used to
  reroute the denied native patch.

The archive-discovery correction below is separate from those source errors. Offline guide
retrieval was repeated successfully; installed-doc corpus retrieval still exits3
because rebrand.db is absent. No corpus/dependency/global mutation was made.

The preserved seven-file WIP includes the closed injected HTTP client, bounded
UTF-8 NDJSON decoder, abort/detach reader, inert content handles and synthetic
stream tests. It is committed at f9186504, unchanged by integration. No existing
client/shell registration or runtime fixture is wired. The stream tests are not
backend or browser acceptance.

Historical blanket command denials, including shell_unproven_use_patch_or_reader
for Git/tests, are superseded by the admitted commands above. At that historical
checkpoint the supported alias patch had not applied. Signed Vitest2.1.9 declaration reads and
TypeScript5.8.3 lib.es5.d.ts JSON.stringify at1150–1187 did not repair the installed
alias resolver. The patch waited for the owner-confirmed removal and restart;
it is now applied and passing, with no activation work pending.
Initial .log files were ignored by the inherited rule; only the explicitly named
committed .txt evidence is treated as committed raw validation output.

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
three E1 source errors. No normal typecheck pass was claimed for that historical
checkpoint; the corrected normal gate is recorded at the top.

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

## Accepted-main wiring checkpoint

The merged page, client, full BusinessWorkspace and full BusinessWorkbench were
read at accepted `f988700db6975364d35b80125af12bfe2d5baac3`. This resolves the
earlier shell dependency without importing an unaccepted implementation.

| Accepted source and Git blob | Concrete E1 glue point |
| --- | --- |
| src/app/business/page.tsx — b260d42ab9ac3d59b4c8efa48ac6d8ad01ad7b94 | The current connection generation owns the private subtree. Organization/epoch/member identity drift closes the client; membership revision keys the workspace. E1 cleanup must also run on that keyed remount, not only when the bearer closes. Locale and pane visibility remain outside this lifetime key. |
| src/lib/business/client.ts — db86ebdd13fec0a3284c8f85b49c22a60d0b31df | Instantiate the closed E1 adapter inside this credential closure. Parent close/401 must abort child attachments and revoke content handles immediately; a disposed workspace must not reuse its old child adapter. Expose no bearer getter or generic fetch. Preserve the browser-only personal connection restriction. Native E1 requires actual dedicated commands after backend handoff. |
| src/components/business/workspace.tsx — 4dbaa30b43037f96cf6cf5114d8712d8763dce2e | Append task-scoped session/file surfaces to the existing surfaces collection. The original-operator capability gates private profile/session loading, independently from public selected-deliverable reads. Owner role and engineering domain do not imply execution authority. Existing task list/board/table, Sources and tenant appearance remain in the same shell. |
| src/components/business/workbench.tsx — 8d46bb7a9ada8d0e25e89da7dc00a03eff8b9f0b | Reuse stable keyed surfaces, render(visible), close callbacks and existing split/stack layout. A visibility change keeps the prompt mounted. The E1 editor owns its pending/dirty close guard; removing a surface detaches transport without stop/kill. No second tab store or layout implementation is needed. |
| src/components/business/task-detail.tsx — e912904115c0ec6f3697cb735e74276ee1a51af5 | Inspected entry/editor/close/revision and SavedTask render seams. Append selected version cards to the existing deliverable presentation when task-owned assets arrive. Keep current CAS adoption and explicit human review; an imported private asset or confirmed prompt is not a reviewed deliverable. |

The original task type base was blob751813ba7d7abea191885e95d3690c4a4db5f9e2.
The current additive Deliverable.assets source handoff is recorded at the top.
Selected task versions remain distinct from private versions/list.

Source review of the preserved E1 WIP identified two bounded checks before
wiring: capture event binding and expected content metadata before asynchronous
reads, so caller mutation cannot change the object being verified; and distinguish
local transport abort from a confirmed durable cancellation. A lost mutation
response requires its original operation receipt, never blind replay. These are
now covered by the current48-test transport checkpoint, not accepted-main regressions. JSON result bodies
are byte-bounded and statically typed. Event frames, prompt receipts, task-selected
public asset metadata and content headers/bytes have explicit runtime validators;
other private result envelopes still require component-specific validation.

With normal tools restored, the next focused gate covers parent/client closure,
keyed scope disposal and no late delivery; unchanged draft through locale and
visible-pane changes; exact receipt recovery without reissuing a prompt;
immutable event/content selection during a held response; member selected-file
reads with zero private session/asset-list calls; and terminal queue disposal
before later buffered input can be sent after uncertainty. Native controls and
real ACP/PTY/content transport remain separately coordinated runtime checks.

The offline guide was rerun successfully for these seams. Applicable rules were
"Gate optional module data loading as well as visible controls" and "Atomic
per-task staging"; cross-project delegation recommendations remain overridden
by the owner's no-worker instruction. Current admitted Git, gh-api, normal-tsc
and native prose patches were observed; there is no new generic Pre/Post journal
claim. A context-mismatched no-op report patch was rejected without changes;
the actual report tail was reread before this distinct prose amendment. The
TypeScript patch stayed untouched until the owner-confirmed removal and restart.

Root subsequently recovered storage to17.46GiB by removing only root-owned idle
compiler cache. This worker deleted nothing. There is no active E1 build/export
command. Preserved Cargo99580 is the launcher for the existing4353 fixture
binary99613, not a new compilation;4354/4355 listeners81173/85232 remain intact.
No static/native export or new target is authorized now; tickets retains the
single Rust build window. Existing user sessions, exports and the paused report
remain untouched. Operational UI still depends on the backend's enabled-route
handoff. PR33 remains draft and is not ready for runtime acceptance.
