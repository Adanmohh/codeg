# Business Sources implementation

Status: compiling Sources plus shared-workbench frontend checkpoint on `feat/business-intake-ui`, based on accepted
`a40b03393a466672060066ae6e0e8c9054a2349d`. No integrated B runtime or browser pass is claimed.
Current product: **9f16a4292eeec1d81ab376f1f35055beabf85600**, pushed to
[draft PR29](https://github.com/Adanmohh/codeg/pull/29). Its own 34-route export
passes and is served on loopback4350, PID21549. Tests41/41, typecheck and scoped
lint pass. Actual protected B/two-user and settings/native-tenant runtime
validation remains pending backend handoff. This is a source checkpoint,
not product or design acceptance.

The accepted A workspace, native chrome, paused visual checkpoint and existing
fixtures/exports are preserved. This worker owns frontend glue only; tickets
owns Rust, protected registrations, migration 000011 and all provider access.

## Contract and interaction

The closed authority is `docs/contracts/business-intake.md` at
`670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5` (SHA256
`6d4be7f3be2c41264ab3bb9c27311e879306a75af0aaae6c0e240f434c874dbd`) plus
`docs/contracts/business-intake-access.md` at
`18be55edc276713fc6d46d075baec363245ba285` (SHA256
`2d3312aa9b85e3f2af5763785b248a6e55ccc06a71c39be8c5962bb505fd4675`). Both were
read fully locally and verified through `gh api` at those immutable refs.
The accepted interaction plan is `reports/business-intake-ui-plan.md`, final
`1a876afc2ae81c7ea2066a14cdbb268fb363e37c`. Q1–Q4 are closed; no new survey.

Sources sits beside My work, Shared work and Review. A person chooses a source,
reads and selects exact passages, prepares a private task, then reviews its
complete destination audience and content before publishing. A text-free link
uses an existing authorized task snapshot with its exact revision/domain; it
does not publish a task draft. Discard and terminal decision rediscovery remain
available according to current capabilities. Task Review remains primary human
work, and source membership never implies destination task access.

Setup consumes only canonical `bindings/list/status` and operator-only grant
operations. New bindings start disabled with zero grants. Grant confirmation
explicitly covers retained history plus current/future imports. Unavailable
binding copy asks the operator to review access and setup without revealing a
private owner revision. Freshness comes only from successful upstream validation;
cached reads, client timers and source selection cannot restore it.

Drafts and credentials stay in the existing per-connection, per-member-revision
memory lifetime. No member token enters legacy transport/storage. Locale and
viewport changes preserve that lifetime. Metadata-only disclosure withholds
private content, including a prepared draft whose destination is inaccessible.
Lost mutation responses preserve their operation identity and use authorized
decision/status recovery; they never fall through to ordinary task creation.

## Composition and borrowed source

Art direction: **Source-to-work reading desk**. The real passage is the visual
anchor: a comfortable reading column alongside a compact preparation/review
column on wide screens, sequential sections on narrow screens. Selected text,
private draft and shared task audience have distinct labels and boundaries.
Source connection and import state are useful secondary controls. There are no
invented metrics, marketing modules, provider success states or engineering hero.

The existing Hafidh mark, user fonts, semantic themes, field sizes, rounded
surfaces and native titlebar remain. Typography uses the current user font with
24–32px section headings, 16px readable passages and restrained metadata. No
new font, icon package, global palette or motion system. Design Studio
art-direction/frontend-design/checklist methods were read; generic new-font,
palette, storage and delegation suggestions yield to these project constraints.

Source mapping (Apache-2.0, accepted Codeg
`a40b03393a466672060066ae6e0e8c9054a2349d`, itself retaining original Codeg
v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed`):

| Source read | Intended reuse |
| --- | --- |
| `src/lib/business/client.ts` | Closed typed intake calls on the existing isolated HTTP/native client; safe finite intake reasons only |
| `src/lib/business/{tasks,identity,presentation}.ts` | Canonical task fields, UUID members, calendar dates and current role/domain labels |
| `src/components/business/task-form.tsx` | Existing metadata and assignment fields; acceptance never calls ordinary create |
| `src/components/business/{ui,task-detail,workspace}.tsx` | Existing controls, modal focus, explicit revision comparison and shell navigation |
| `src/app/business/{page,layout}.tsx` | Preserve private session lifetime and accepted native chrome unchanged |
| `src/components/ui/{input,button}.tsx` | Inherited controls with scoped business contrast/focus treatment |

No IntroMail/provider implementation is copied into the UI. Its approved
behavioral foundation is represented by the accepted backend contract; tickets
owns those precise source/license mappings. NOTICE additions will name each
adapted source without replacing any existing attribution.

## Grounding and coordination

Complete current FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS,
BUSINESS-IMPLEMENTATION and design BRIEF were read before product edits.
Separate reads of installed `node_modules/react/package.json` and
`src-tauri/Cargo.toml` ran first. Live audit records in
`/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl` contain both
PreToolUse and PostToolUse for this worktree/session
`01a07c1c-cf2e-73e1-bbe3-e758c8363042`; hooks remain enabled.

Installed authority: React 19.2.4, @types/react 19.2.13, Next 16.1.6,
TypeScript 5.8.3 and Tailwind 4.1.18. Existing client/field/component source and
installed hook/input/button types were read before their adaptation. Local
code-context ran using the existing RAG `.venv/bin/python` and
`HF_HUB_OFFLINE=1`: guide exit 0 (strict canonical types and source-first rules);
docs exit 3 because the rebrand installed-doc corpus is absent. This is a
coverage limitation, not a successful dependency-doc retrieval; local installed
source supplies the API grounding. No corpus/dependency install or upgrade.

Tickets' immutable DTO checkpoint `9a4c8c882cec938665bc233b4d658d8de019ccfd`
matches the frontend transcription. Its later setup checkpoint `67708b07`
reports eight bindings/grants HTTP/native operations exposed through the existing
Principal boundary. Source/import/candidate routes are not exposed at that
checkpoint. This report does not claim an integrated endpoint or fixture pass.

Reviewer preparation: `599aa1e6b74c0841ab2753dd58c806cae0c83dcd`,
`reports/review-business-intake.md`. The owned public cold-entry export is now on 4350; protected B flow validation still awaits backend handoff.
Root reserved owned UI **4350**, tickets' protected synthetic backend **4351**
and synthetic upstream **4352**. The 4350 export listener is owned here; exact registered backend source and synthetic access must be coordinated before protected flow checks.
No old listener, export or browser is changed.

## Checks and remaining work

- Read-only initial status: only preserved untracked `reports/visual-workspace.md`.
- `git fetch origin`, fresh branch creation: exit 0.
- Contract local/immutable GH byte checks: both hashes match, exit 0.
- Installed-source reads and live hook audit inspection: exit 0.
- Frontend focused tests, typecheck, scoped lint and first isolated export pass;
  exact commands and scope appear below. Real B browser/API checks remain pending.

Remaining: reconcile executable backend checkpoint; guarded real protected API
fixtures with two synthetic users; actual Playwright CLI and measured Design
Studio checks, including the expanded main-workspace design scope below.
Fireflies is the first complete slice;
email and Hafidh remain obligations through the backend's safe projections.
No live configuration, provider/model/engine action, send or deployment is
authorized by these fixture checks.

Early report checkpoint: `556956d4`; draft PR
https://github.com/Adanmohh/codeg/pull/29. Root reserved UI **4350**, tickets
backend **4351** and synthetic upstream **4352**; only the owned UI4350 listener is started here.

Typed client checkpoint: exact closed intake operations, native names and safe
reason allowlist extend the existing client. Existing identity/task errors and
20-second timeout remain. Initial focused Vitest2.1.9 run passed **26/26**
(client13, source freshness/selection3, provider isolation8, session2), exit 0.
The tests use explicitly synthetic unit responses; no production mock or live
backend acceptance is claimed. Source fields preserve normalized nulls, exact
passage revisions and date-only task values. Client-checkpoint typecheck
`tsc --noEmit --incremental false` passed, exit 0, after correcting an ES2022
`Object.hasOwn` use to the installed ES2020 `hasOwnProperty.call` API and adding
the new client method to the native test fixture. The first typecheck exited 2
for those two implementation mistakes; no target or dependency was upgraded.

## Wired composition checkpoint

Client source is pushed at `13e6b516`; wired product checkpoint is
`f6a326c986f80ac53ae478c7e76a243d60d9610e`. The composition is reachable from
**Business → Sources**, with My work, Shared work and Review retained in their
existing order and role boundaries. Source calls are not preloaded on My work.
`SourcesWorkspace` stays in the existing private session lifetime after first
visit; leaving its view hides private content while locale changes keep drafts.
Returning rechecks source/candidate access before disclosure. Source references
on ordinary tasks are fetched only when their disclosure control is opened.

| New target | Reused source / final responsibility |
| --- | --- |
| `src/components/business-intake/{workspace,source-review}.tsx` | Existing business workspace list, paging, generation guards and safe private scope; canonical bindings/status plus cached sources/candidates |
| `candidate.tsx`, `task-target.tsx` | Existing task-detail explicit revision comparison; existing metadata/assignment fields; saved `PreparedTask` audience review or authorized target TaskDetail + CAS, never ordinary create |
| `setup.tsx` | Existing business Field/Modal/Action controls; operator-projected setup, masked write-only key, disabled zero-grant creation, explicit all-history/current/future grant confirmation |
| `imports.tsx` | Existing isolated client and safe request lifetime; explicit bounded steps, full coverage/failure/status, exact operation replay after uncertainty |
| `task-sources.tsx`, `ui.tsx` | Existing disclosure controls, Person and semantic styles; inaccessible link redaction, bounded literal passages, calendar date display and expiry-only clock |
| `src/lib/business/intake-copy.ts` | Full EN/AR source/setup/import/audience/recovery copy using the existing locale provider and fonts |
| Existing `business/{workspace,task-detail}.tsx` | Additive Sources navigation and lazy authorized source-reference seam; native layout, authentication and task mutations unchanged |

The immutable Rust DTO checkpoint now read is
`9a4c8c882cec938665bc233b4d658d8de019ccfd`,
`src-tauri/src/business_intake/types.rs`, SHA256
`e5fd8008c589be894ff0315aba07c3fe7d8292778d1c58fad71af0820be72e9c`.
Its closed input/output fields match the frontend transcription, including the
resource union, write-only credential variants, nullable revision, grant nulls,
`hasPreparedDraft`, candidate flags and DecisionTask redaction. HTTP/native
operation registration is **not yet exposed at that backend head**; method names
remain grounded in the accepted contract until tickets publishes registrations.

Focused checkpoint checks:

- `vitest run src/components/business-intake/workflow.test.tsx src/components/business src/lib/business`: **73 passed / 8 files**, exit 0. Sixteen new workflow cases cover exact private save/accept payload, date/null preservation, full audience confirmation, locale/view draft continuity, source expiry/revocation, destination draft withholding, lost-response receipt recovery, conflict comparison/adoption, passage-only rebase/text-free link, stale discard through the real parent, late target rejection, lazy reference redaction, setup consent and exact import replay. Existing session/provider/native/task tests also pass.
- `tsc --noEmit --incremental false`: exit 0 after fixing the new test helper's locale union and removing unsupported Testing Library ByRole `exact` options. The prior test-harness typecheck exited 2; expectations were retained.
- Scoped ESLint across changed frontend source and tests: exit 0, zero warnings. Initial composition checks caught a missing existing Person fallback prop and an impure render/effect clock update; both were corrected using the inherited control contract and expiry callback.
- No Rust/backend, dependency, lockfile, theme, session or native chrome edit.
- First isolated export passed; guarded backend/real browser/Design Studio
  acceptance remains pending.

Reproducible logs are under `reports/business-intake-ui-evidence/`. Unit fixture
records and responses are explicitly synthetic and excluded from product
imports. The owned cold-entry listener/browser is recorded below; no old fixture is changed.
The next independent-review target is this committed composition, followed by
an isolated export and real protected API4351 validation when tickets publishes
its executable checkpoint. Fireflies, email and Hafidh controls use only their
accepted operations; no enabled invented provider endpoint or synthetic product
response was introduced.

## Navigation and preview preparation checkpoint

Source-reference navigation now uses the existing task editor's unsaved-change
confirmation. Keep editing retains the exact private brief; only explicit
discard opens the source, and a pending task mutation disables that navigation.
A focused regression verifies no task write occurs. Candidate mutation controls
also stay disabled while their source parent is checking current access. Source
errors use source-specific safe copy, and an unprepared candidate with no
permitted destination has an explicit state instead of a blank preparation area.

`vitest run src/components/business-intake/workflow.test.tsx
src/components/business/workflow.test.tsx`: **32/32 pass**, exit 0, including
17 intake cases and 15 existing task cases. The first added-test run had one
test-harness failure from an incorrect expected button label; it was corrected
against existing `copy.ts` without weakening the behavior assertions.
`tsc --noEmit --incremental false`: exit 0. Scoped ESLint for the changed intake
components, task detail and intake copy: exit 0, zero warnings. `git diff --check`
and the fixture's `node --check`: exit 0. These checks do not replace the pending
actual API, rendered contrast/focus or two-user recovery validation.

First export of unchanged product `f6a326c9`:
`CODEG_EXPORT_DIR=.build/business-intake-ui-export NEXT_TELEMETRY_DISABLED=1 pnpm build`,
exit 0, 34 static routes. `business.html` SHA256:
`8b94b44e1efc663354c48d8461ab216b8eff96cf44ce5085cf0ec80993a463ba`.
It predates the navigation correction and is **not** advertised as a corrected
or accepted preview. No browser or backend was used for that build.

Owned fixture: `reports/business-intake-ui-evidence/serve.mjs` serves
only an owned export on loopback4350 and proxies the closed identity, human-task
and intake operations to the separately guarded backend4351. It rejects legacy
APIs, engine operations and WebSockets, and records only method/path/status.
No response mocking, credentials/body logging or synthetic production state.
Node syntax check passed; this does not establish backend readiness.

## Expanded design workorder

Compiling intake/navigation checkpoint before reshaping:
`b9a7607f` (pushed to draft PR29). The source remains reviewable independently
of the following presentation pass.

Owner steering now includes the existing business sidebar/navigation, tabs,
list, board and task detail as well as Sources. Human/agent shared work and
business decisions lead; engineering remains accessible in its accepted
authorized context. This intake checkpoint is published before visual reshaping
so its behavior remains independently reviewable.

Next design pass will use Design Studio art-direction, checklist and measured
audit methods across one coherent workspace, with real Playwright CLI before/
after captures. Unit tests alone are not design acceptance. Root is publishing
the concrete direction and reconciling tokens; existing user fonts/themes and
native chrome remain authoritative meanwhile. No fake metrics, unsupported
modules, new dependencies or global palette replacement.

Tenant customization is a future validated presentation input: tenant brand,
theme, permitted navigation/default view and role presets must remain separate
from personal preferences and transport authority. Business-scoped presentation
primitives should accept that contract without importing it into authentication
or storing member credentials. There is no fake tenant switcher, arbitrary
tenant JS/CSS, client-only isolation or invented config response. Root/approvals
own the tenant boundary contract; this frontend will consume the accepted shape.

The source-lint outputs under evidence are preliminary heuristic checks. The
candidate scan's list/hover findings require inspecting the actual state and
inherited Action styles; they are not a measured contrast or interaction verdict.
`design-checklist.md` records pending observable gates. The accepted preview and
all previous outputs stay unchanged until a corrected, explicitly identified
synthetic preview is ready.

### Owner correction: one rich shared workspace

The owner explicitly supersedes a separate lightweight business dashboard.
The existing engineering workbench is the common foundation: rich tabs and split
panes, chat and AI terminal as first-class business tools, alongside task
list/board/detail and structured data. Tenant/role tailor tools and data; they
must not remove the workspace's power by definition. The earlier compact
dashboard composition is superseded before product edits. Product remains
b9a7607f7d2d3651501672c6020ff167af8fc70e.

Direction: **One workbench, many kinds of work**. World: conversation, terminal,
source passage, work brief, deliverable and human sign-off. Reuse the engineering
shell's compact chrome, tab strip, stable pane geometry and contextual tools;
business tasks and Sources become real working surfaces in that vocabulary.
Existing semantic colors, configured UI font and native caption geometry remain.
No calendar, visualization data, execution authority or provider readiness is
claimed merely because its eventual surface has been discussed.

    organization / navigation       work tabs / focused pane controls
    work / sources / people         +------------------+------------------+
    permitted tools                 | tasks / sources  | detail / AI work |
                                    | list or board    | authorized pane  |
                                    +------------------+------------------+
    personal preferences            terminal / contextual tools when authorized

This diagram is a composition plan, not an assertion that member AI execution
or a terminal route already exists. Calendar/content planning remains future
capability with no enabled placeholder navigation. Structured lists/boards use
real task data; future charts must disclose their real loaded scope.

### Exact shared-shell source assessment

All paths below are existing Apache Codeg source read at accepted
a40b03393a466672060066ae6e0e8c9054a2349d; no proprietary or AGPL implementation
is copied. The original v0.30.4 attribution remains. The table distinguishes
reusable presentation from runtime that cannot be mounted for a member.

| Exact source / observed implementation | Shared-shell plan |
| --- | --- |
| src/app/workspace/layout.tsx: sidebar/center/aux horizontal panels, workspace/terminal vertical panels; KeptMountedSurface combines inert/hidden state with OverlayHostHiddenProvider | Extract/reuse the stable presentation slots and visibility rule. Keep original operator provider composition intact until an authorized scoped adapter exists; do not wrap a member in that provider tree. |
| src/components/ui/resizable.tsx; installed react-resizable-panels@2.1.9 README and PanelGroup/PanelResizeHandle types | Reuse the existing panel controls and keyboard resize contract. Give conditional panels stable IDs/order; keep B arrangements in the existing private session lifetime. Do not enable default localStorage autosave for source/task drafts. |
| src/lib/tab-group-layout.ts (pure immutable split/remove/resize/rect functions); conversation-detail-panel.tsx stable sibling group rendering around lines2591/2754 | Reuse the split-tree model and stable group keys. Resizing/orientation changes must not reparent a live editor or lose its draft; adapting a pane is not recreating a task/agent engine. |
| src/components/tabs/{tab-bar,tab-item}.tsx | Reuse the tab chrome and callback-driven interaction. Separate its conversation-specific descriptor/status and global store reads from generic task/source panes; never force a task UUID into a conversation/folder DTO. Preserve close, pin, selection, split and narrow focus semantics through the shared presentation boundary. |
| src/contexts/workbench-route-context.tsx, src/components/workbench/workbench-content.tsx | Reuse explicit registered surfaces and leave guards. That existing provider also mounts OpsSessionBoundary, so direct import is not an isolated member route. Keep B's private audience/receipt and dirty guards when adapting content slots. |
| src/contexts/tab-context.tsx, src/stores/tab-store.ts | Current lifecycle hydrates/saves legacy opened_tabs, subscribes global events and injects ACP. Layout state uses global workspace:tab-groups:v1; these are not tenant/member-scoped authority. Reuse pure operations/presentation, not this singleton's persistence or ambient connection for members. |
| src/contexts/terminal-context.tsx, src/components/terminal/terminal-view.tsx, src/lib/api.ts:terminalSpawn | Current terminal provider loads global terminal settings/subscriptions, and the view calls legacy spawn/write/resize/kill. An adapted terminal needs a typed scoped transport supplied by the backend contract before it can be enabled for an ordinary business member. |
| src-tauri/src/web/handlers/terminal.rs, web/router.rs | Existing terminal spawn accepts working directory/shell/initial command and injects operator credential environment; routes/events are protected by the legacy operator boundary. This is intentional operator behavior, not a tenant terminal API. Link hiding or a role label cannot turn it into one. |

Concrete implementation order:

1. Retain compiling B content/client checkpoint.
2. Introduce controlled common chrome/pane slots from the approved sources,
   keeping legacy provider/runtime behavior unchanged.
3. Place real task and Sources work in those slots with stable in-memory editors
   and explicit close guards.
4. Wire first-class conversation/terminal slots only to the accepted tenant
   execution contract.
5. Run actual two-session and measured390/768/1280 EN/AR light/dark/pane/focus/
   resize loops. No dummy chat, terminal success output, fake tenant selector or
   provider launch for a screenshot.

Backend needs to settle: server-derived tenant selection and principal context;
tenant-owned execution roots and tools; per-terminal/conversation create/read/
write/subscribe/close authority; secret/provider environment isolation; output
ownership and revocation; tenant-admin versus platform-operator setup. These are
backend prerequisites, not proposed URLs or a second frontend auth system.
Tenant presentation can later consume only validated brand/theme/navigation/
default-view presets, separately from personal preferences. Arbitrary tenant
JS/CSS and client-side isolation are excluded.

Root's architecture research pins are edu-blend
735e7695a44ab6e5dbda521c822f3a3809f289c8 (proprietary inspiration only; no source
republication) and Payload 54a0e3d24015b2e9c565bd7e695be1ec7184662e (MIT).
Approvals owns the architecture report/authority contract; neither source has
been copied here, and this UI report does not independently certify its isolation.

Local code-context's workflow and custom-UI rules support meaningful pane jobs
and actual design iteration. Its cross-project DB/delegation advice does not
override assigned ownership. W3C ARIA Practices main was resolved via gh api
to 7e4034b262bc0d25332e330d8a582aaf34113829; exact
content/patterns/{tabs,table}/*-pattern.html was read. Full tab focus/panel
semantics are required, and a visual table does not justify incomplete grid
roles. These are documentation references, not copied implementation or a
founding source upgrade. Self-critique: preserve workspace capability and the
auth boundary together; a simplified dashboard or an unrestricted legacy shell
would each fail the owner's corrected requirement.

### Latest backend integration status

Tickets' PR28 handoff is 5de1176beb4778604b26df65f2ec669b0ab7987b, production
4a194500b76b97aa5caaf9434ce5c1f16e54ea48. The exact committed http.rs was read:
eight source/import operations now join the eight setup operations under the
existing Principal middleware, and their names match the closed TS client.
Candidate decisions and task-source references are not exposed in this router
yet. Native source/import additions are explicitly held for reviewed tenant
selection. No native readiness or independent backend acceptance is inferred;
no manual 4351/4352 listener handoff has occurred here.


## Shared workbench source checkpoint

The first composition now opens real authorized tasks in separate working tabs,
with the same task editor and its pending-write, revision, review and explicit
unsaved-close rules. My work/Shared work/Review retain their filters and real
loaded task page; Table is a semantic table over that same page. Sources and
People are separate lazily visited surfaces. No fixture response is imported
by production UI.

The existing Codeg sibling-pane geometry and hidden-surface context keep every
editor mounted in the same React parent/slot while switching tabs, resizing,
changing locale or moving between narrow and wide windows. A wide pane can keep
the collection/reference alongside the active task. The installed resizable
control retains its pointer and keyboard implementation; pane sizes and private
contents remain in memory with no global tab store or localStorage persistence.
The business-only modal now respects the hidden-surface context so a background
pane's portal cannot paint over another pane. Original engineering providers,
authentication, native chrome and backend remain unchanged.

Source mapping is appended in NOTICE at exact accepted Codeg a40b0339. Local
React19.2.4/@types-react19.2.13 hook/DOM types and
react-resizable-panels2.1.9 PanelGroup/Panel/PanelResizeHandle definitions were
read before adaptation. The pure existing computeRects implementation and
KeptMountedSurface/flat sibling examples were read completely at their relevant
functions. No new package, runtime or lockfile. Code-context guide again returned
workflow/type/source-first guidance; dependency docs exit3 remains the missing
rebrand.db corpus, not a library upgrade. Live hook audit has this session's
PreToolUse line20657 and PostToolUse line20658; hooks stay enabled.

New focused evidence: **41/41 pass in four files**, Vitest2.1.9, exit0:
`vitest run src/components/business/workbench.test.tsx
src/components/business/workflow.test.tsx src/components/business/ui.test.tsx
src/components/business-intake/workflow.test.tsx`.
Five new shell cases cover independent task drafts and DOM identity across
split/tab/locale/narrow changes, guarded mouse/Delete close, manual EN/AR tab
focus/linked panels, and exact calendar-date task table navigation. One new
modal case covers hidden portal privacy and retained parent draft state. The
existing 17 intake and 15 task workflow cases remain passing. The first run was
39/40: jsdom does not load the Tailwind visibility class in this suite; explicit
hidden style now accompanies the inherited hiding class and role/inert guards.
The assertion was retained. Async test reads now await the actual loaded heading;
final output contains no React act warning. Raw outputs are preserved in the own
ignored evidence directory; this is not rendered or backend acceptance.

### Tenant contract reconciliation

Read complete architecture af00c956787142f900384f7ba6b34ebbc005eb88 and the fixed
settings/native contract 7516461633c163c2ac683930487e33231e630b0b,
`docs/contracts/business-tenancy.md`. Approvals owns its compiling helpers and
all identity/native ACL implementation. UI settings will use exact closed
organizationId/revision/settings responses and expectedRevision updates with
no caller actor/organization selector. Neutral/blue/violet presets and
split/stacked layout remain scoped presentation, not authority; personal locale
and theme mode remain separate. A conversations default grants no execution.

The pending explicit personal-credential switch authenticates the destination
scope and offers cancel/discard while old edits remain intact. Only confirmed
switch closes/aborts the old client, invalidates delayed results/events and clears
private draft/cache/tab/pane content; same-scope locale/viewport preserves it.
Compiling identity/settings core f3b408dae5c724f354763961d79a17a7ae5c86f8 now implements lifecycle fields and manageTenantSettings; transport registration/native bridge remain pending. Native kind=native remains original-operator compatibility and will not be used
as tenant authority. Restricted native access must consume approvals' actual
window-bound bridge/ACL. No guessed selector, settings runtime success or member
terminal endpoint has been added. Execution isolation is a separate backend gate
and does not block this real shared-shell work.

### Owned preview inventory

Public cold-entry preview only: http://127.0.0.1:4350/business and /business.html.
Node PID50065, owned exec session64827, script
`reports/business-intake-ui-evidence/serve.mjs`, fixed protected proxy target4351.
Two own CLI sessions are intake-ui-a4350 (browser PID50564) and
intake-ui-b4350 (browser PID50617). Both remain unauthenticated; no production
credential, broad Settings snapshot or provider/engine action. Tickets has not
handed off a protected4351/upstream4352 fixture, so source/import/candidate flow
acceptance is still pending and no mocked native readiness is claimed.

Served export `.build/business-intake-ui-b9a7607f` is **pre-shell** product
b9a7607f7d2d3651501672c6020ff167af8fc70e. Build command
`CODEG_EXPORT_DIR=.build/business-intake-ui-b9a7607f NEXT_TELEMETRY_DISABLED=1 pnpm build`
passed exit0 with34 static routes. business.html SHA256:
3f46f53e3227793684c1d81d4acd3f6b2f865fa9396a1623b70f5b23b8d51d86.
It is not the new shell export or accepted design. A fresh export will be given
an explicit source/hash before replacing this owned preview. All accepted old
fixtures, exports, browsers and the paused visual checkpoint are preserved.


Actual public baseline on the **pre-shell** 4350 export: Playwright CLI0.1.18,
intake-ui-a4350 at1280×900/light/English, document overflow=false,
nativeChrome=false and fixture calls=[] (zero protected API calls). This checks
only the unauthenticated connection surface. No full Sources, task pane or
settings browser pass is claimed from it. All commands exited0; no CLI update.

Commit-source `tsc --noEmit --incremental false` passed exit0. Scoped ESLint
and final41-test rerun passed exit0 after focus recovery was added to guarded
tab closure. No passing checks were weakened. Public baseline B session at
390×844/dark/Arabic returned direction=rtl, overflow=false and fixture calls=[];
PNG evidence is public-before-1280-light.png and public-before-390-dark-ar.png.
These are cold-entry captures of b9, not the new shell's rendered acceptance.


## Current corrected export and next native seam

Product9f16a4292eeec1d81ab376f1f35055beabf85600 export:
`.build/business-intake-ui-9f16a429`, business.html SHA256
`fc4809a927c5024388eeba94aec39cff245e2e5b35aaa3f70af7de0d68b8e610`.
`CODEG_EXPORT_DIR=.build/business-intake-ui-9f16a429 NEXT_TELEMETRY_DISABLED=1 pnpm build`
passed exit0 (34 static routes). This owned 4350 listener is now PID21549,
exec55235, using the same closed synthetic guard/proxy target4351. Previous
owned Node50065 was verified by its exact command/port and gracefully stopped;
both old exports are retained. No other fixture/process/browser was changed.

The two own CLI sessions reloaded this exact export: /business and
/business.html returned fixture export=business-intake-ui-9f16a429 and calls=[];
both had no document overflow, the 390px Arabic view remained RTL, and the web
route had no native drag chrome. Protected4351/upstream4352 still had no listener
at the last read. These checks establish cold-entry isolation only, not the new
shell's authenticated visual review or full B behavior. PNG before evidence is
explicitly the retained b9 public connection surface.

Approvals' native refinement is recorded: generic Tauri window plugin controls
accept a target label, so the restricted tenant window must not receive those
plugins/global events. Future custom tenant chrome will use its closed
business_window_control seam with actual invoking window and only
close/minimize/toggle_maximize/is_maximized/start_dragging, no caller label.
Window-context/session-handle DTOs and real bridge registration remain pending;
trusted host chrome is untouched and native system decorations remain the safe
fallback. This worker made no identity/backend/native change or readiness claim.

Remaining immediate dependencies are tickets' exact protected synthetic4351
handoff (including candidate decisions/email/Hafidh projections) and approvals'
closed settings/platform/native transport. UI can consume their committed DTOs;
no guessed endpoint, ambient native owner fallback or live provider/config action.
The shared-shell/source checkpoint is already pushed for independent review.

## Native availability and settings preparation checkpoint

Approvals' published `29774b50aafc29658a2f48fab1f44d366ed2c8a0` contract and
`business_identity/{http,settings,types,store}.rs` were read at that immutable
commit. Its native context explicitly reports `tenantWindowAvailable:false`.
The native host now offers only the original local operator path; its personal
HTTP form is absent, and both the submit handler and client constructor reject
HTTP sessions before retaining a credential or making a request. Browser
personal sign-in remains unchanged. Existing native chrome/window controls are
untouched. This frontend prevention does not claim native tenant isolation.

The formerly accepted native personal-draft test is superseded by the new
product boundary: it now verifies local-only EN/AR entry and stable chrome.
A separate browser test retains the same private-token DOM/locale assertion.
The client regression verifies no HTTP/invoke, no member-token storage and no
replacement of the ambient operator slot. Original native command and browser
session rejection/member-revision tests still pass.

Standalone `settings-editor.tsx`, `settings.ts` and `settings-copy.ts` prepare
the exact closed settings DTO and EN/AR editor. It is not yet imported by the
workspace and makes no production request. It accepts a supplied protected
get/update interface, sends only expectedRevision plus the four settings
fields, retains drafts across locale changes, compares current revision after
409/response loss, requires explicit adoption, ignores an unmounted response,
and refuses to apply another organization's response. Backend authority remains
required; no role inference, CSS injection, global token write or mocked
production response. The newly published actual settings transport is the next
wiring step.

Checks: **36/36 focused tests, exit0** (client14, native chrome14, session2,
settings6); `pnpm exec tsc --noEmit --incremental false` exit0; scoped ESLint
exit0/no warnings. The initial settings-only typecheck found an overly broad
test locale type, and lint identified a cleanup ref warning; both were fixed
without relaxing assertions. Full sanitized commands/output are in
[business-intake-ui-evidence/native-settings-checkpoint.txt](business-intake-ui-evidence/native-settings-checkpoint.txt).
The live hook audit again includes this session/worktree's PreToolUse and
PostToolUse (lines21441 and21440). Local installed @tauri-apps/api2.10.1
core.js provides the actual isTauri/invoke contract. No build or browser
acceptance of these new changes is claimed yet; 4350 still serves the preserved
9f16a429 export/PID21549 and both owned public-entry browser sessions remain.
