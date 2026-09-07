# P1 bug workflow — host and operator UI

Implementation in progress. Sole writer in
`/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand`, branch
`feat/step2-bug-workflow`, created after a clean check and fetch from accepted
main `b15ba2d40e1fafe7d272ec5910a5b690164fff9b`. Intake PR #5 is accepted at
`493df48ca92699c5818fde39d5819596712b9fdb`, merged `4988f46b`; root's independent
22 Rust and 14 Python tests passed. This task adds the host/UI layer only.

## Ownership and immediate coordination needs

Owned additions: `src-tauri/src/ops_intake_host/`,
`src-tauri/src/commands/ops_intake.rs`,
`src-tauri/src/web/handlers/ops_intake.rs`,
`src/components/ops-intake/`, `src/lib/ops-intake/`, plus minimal registrations.
Reserve `m20260908_000006_ops_intake_host` for product configuration references,
normalized snapshots, revisioned drafts/evidence associations and fix-task links
if persistence needs it. Preserve every other migration and NOTICE entry.
No changes to the core approvals engine or the email owner's live `ops/` files.

Read PR #7's report and source at
`dce45beb59a09be5285cf3b26eb22f97c4d4bf30` as interface guidance only. It remains
unmerged at this checkpoint. Shared auth/navigation edits will use accepted
main after that PR is accepted, not copies of its work in progress.

Exact small seam needed from the Ops owner:

- Reuse `web::auth::AuthenticatedOperator`, which its authenticated middleware
  inserts, and `ops::Operator::server()` / `desktop()`. Current `Operator`
  fields are private and there are no accessors. Need crate-visible read-only
  `account_id() -> i32` and `actor() -> &'static str` accessors. Keep fields and
  construction private to the existing boundaries; no JSON actor or second alias.
- P1 handlers use the same marker and canonical `operator:http` /
  `operator:desktop`. They remain separate from email routes and registry.
- After accepted-main integration, add a Bug intake navigation destination
  using existing sidebar/workbench routing. Do not modify the live Ops page.
- Existing credential-store operations will hold dedicated random intake
  references; no Git account token is selected, replaced or used for runtime
  GitHub filing. Read the accepted store changes before adding any shared seam.

Proposed narrow bridge contract for the pi owner (names will be updated here
when implemented): `ops_intake_host::agent::prepare_and_propose` takes a
backend-created run context and a revisioned local bug draft ID, not a raw
`IssueDraftV1` or arbitrary action name. Context contains the authenticated
account/task/run/connection/agent attribution; product/folder/repository come
from the stored host binding and the live task. The helper rechecks that chain
and loads only previously human-attached proof IDs, then calls accepted
`prepare` and `ops_approvals::propose`. Result is pending/denied metadata plus
the frozen review projection, never a capability or token. No agent evidence
minting, config, approve/deny/dispatch or receipt-forging method will exist.

The accepted gate requires a real live task/run. The host will not manufacture
a running task to simulate an agent proposal. Missing eligible task context
must be visible, with an action to the existing task machinery. Local fixtures
can establish synthetic tasks through the existing service transitions.

## Intended implemented flow

1. Operator connects Hafidh intake and a repository using stored nonsecret
   product/folder/App/installation/repository configuration and credential-store
   references. Secret entry is write-only. Missing adapter, credential, binding
   or permission remains an actionable unavailable state.
2. A closed owned Python process bridge calls the accepted `IntakeClient`
   read operations; no duplicated triage, invented Hafidh endpoint or direct
   Rust backend HTTP adapter. Keep its list/get state within the owned process.
   Only successful authorized upstream revalidation can update source revision
   and freshness. Agent/UI JSON cannot assert either value.
3. Operator attaches sanitized bounded UTF-8 build/screen/reciter/log proof by
   content, with explicit source/session/provenance association. No arbitrary
   server path, URL, bucket read or screenshot-as-log. All four fields remain
   mandatory. Revision changes and stale sources require refresh/review.
4. Show exact repository, title, body and labels from accepted preparation.
   Show suggested severity as unconfirmed until the operator confirms it;
   retain actual available device/app/session metadata without guesses. Edited
   content is prepared and validated again before existing human approval.
5. Consume the existing committed handoff through `ops_intake::dispatch`.
   Created/failed/unknown receipts stay distinct; unknown permits read
   reconciliation only. No mutable draft reload or blind retry.
6. A created issue can seed a linked proposed fix in existing work-task
   creation/review machinery. No auto-launch, fix merge or release engine.
   Build-note and tester-email bridge gaps will be recorded explicitly.

## Source and design grounding

Read complete FOUNDING, ORCHESTRATOR, STATUS, DECISIONS and AGENTS from the
branch base, the accepted intake report, `docs/design/BRIEF.html` and
`reports/design-acceptance-checklist.md`. Read code-context, frontend-design,
Playwright CLI, Design Studio checklist/audit source instructions and relevant
input/form/error checklists. The brief overrides generic marketing/motion
suggestions. No additional worker or paid inference is authorized/used.

Offline code-context guide exit 0: end-to-end Playwright verification, human
approval and actual sibling source reuse apply. Owner local-first / gh-api /
no-worker instructions override generic context7/delegation advice. Dependency
docs query exit 3: `data/code/rebrand.db` is absent. Use actual pinned installed
source; no corpus hit or new global index is claimed.

Separate React 19.2.4 metadata and Cargo reads succeeded. Live docs-first audit
at `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl` records this
worktree/session `01a07c1c-cf2e-73e1-bbe3-e758c8363042`: PreToolUse line 3085,
PostToolUse line 3067. Hooks remain enabled.

Borrowing remains codeg v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed`
(Apache-2.0), accepted intake modules and the exact SDK/Hafidh/intromail pins in
`reports/intake-github.md`. Initial local source reads include
`src-tauri/src/keyring_store.rs`, `web/auth.rs`,
`db/service/work_task_service.rs` create/forge-source/CAS paths, and accepted
`integrations/hafidh-intake/src/hafidh_intake/{server,schemas}.py`. Exact
source-to-destination mapping and added NOTICE entries will follow the ports.
All remote research uses `gh api` at recorded immutable refs; latest docs do
not change the founding source versions.

Design plan: retain inherited Inter/user typography, light primary #245e58,
dark primary #9bd4c5, neutral backgrounds, existing radii/spacing. The actual
source report, proof checklist and rendered issue are the main content.
Desktop uses a compact source list beside evidence/review; mobile shows one
readable pane with Back. Forms keep visible labels, focus and saved/error
feedback. Synthetic fixture records are explicitly labelled. No decorative
metrics, new UI system or production sample data.

## Validation and delivery plan

Own browser fixture port **4322**; do not touch root 4318 or email 4320. Publish
the owned process command, isolated data/output paths and a nonsecret fixture
token for root reproduction. Runtime provider traffic must be test-only loopback;
no real issue/email/message, App install, backend write or deployment.

Required gates: locked default desktop/server checks, strict Clippy, typecheck,
focused host evidence/auth/privacy/stale/receipt tests, Python bridge tests,
and actual Playwright CLI desktop/mobile light/dark missing/populated/error/
success flows with screenshots. Apply Design Studio measured checks and audit
fix/recheck loops against BC-1/2/3/11/12/16/17/18 as applicable. Existing email
flow checks remain its owner's responsibility. No implementation gate has run
for this new task at this checkpoint.

Commands so far: clean `git status`, `git fetch origin`, and new branch switch
all exit 0; installed git usage help exits 129 normally; document/source reads
and immutable PR metadata resolution exit 0; code-context exits recorded above.
Early commit/push and draft PR are next. This is resumable progress, not a
finished workflow claim.
