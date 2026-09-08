# Business Sources implementation

Status: wired frontend checkpoint on `feat/business-intake-ui`, based on accepted
`a40b03393a466672060066ae6e0e8c9054a2349d`. No B runtime or browser pass is claimed.
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

Tickets reports contract unchanged and an upcoming immutable DTO/API checkpoint.
Its current uncommitted `business_intake/types.rs` and `error.rs` were read as
provisional transcription, not accepted implementation evidence. They match the
closed route/DTO plan, including explicit `PreparedTask`, nullable initial source
revision, metadata-only disclosure, candidate flags and safe reasons. The
frontend will reconcile against the first committed typed checkpoint before
claiming integrated runtime validation.

Reviewer preparation: `599aa1e6b74c0841ab2753dd58c806cae0c83dcd`,
`reports/review-business-intake.md`. No browser or fixture is active for B.
Proposed new owned UI port: **4350**; backend/reviewer ports within 4351–4353
must be coordinated before launch. No old listener, export or browser is changed.

## Checks and remaining work

- Read-only initial status: only preserved untracked `reports/visual-workspace.md`.
- `git fetch origin`, fresh branch creation: exit 0.
- Contract local/immutable GH byte checks: both hashes match, exit 0.
- Installed-source reads and live hook audit inspection: exit 0.
- Product tests/typecheck/lint/export/browser checks: not yet run for B.

Remaining: implement typed client and real source/task composition; reconcile
backend checkpoint; focused privacy/CAS/receipt/role tests; isolated export;
guarded real protected API fixtures with two synthetic users; actual Playwright
CLI and measured Design Studio checks. Fireflies is the first complete slice;
email and Hafidh remain obligations through the backend's safe projections.
No live configuration, provider/model/engine action, send or deployment is
authorized by these fixture checks.

Early report checkpoint: `556956d4`; draft PR
https://github.com/Adanmohh/codeg/pull/29. Root reserved UI **4350**, tickets
backend **4351** and synthetic upstream **4352**; all remain unstarted here.

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

Client source is pushed at `13e6b516`. The new composition is reachable from
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
- Guarded backend/real browser/export/Design Studio acceptance remains pending.

Reproducible logs are under `reports/business-intake-ui-evidence/`. Unit fixture
records and responses are explicitly synthetic and excluded from product
imports. No new listener/browser is running, and no old fixture is changed.
The next independent-review target is this committed composition, followed by
an isolated export and real protected API4351 validation when tickets publishes
its executable checkpoint. Fireflies, email and Hafidh controls use only their
accepted operations; no enabled invented provider endpoint or synthetic product
response was introduced.
