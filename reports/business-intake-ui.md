# Business Sources implementation

Status: implementation in progress on `feat/business-intake-ui`, based on accepted
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

Checkpoint commit and draft PR URL will be recorded after publication.
