# Business intake implementation

Owner: tickets, sole writer in `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`,
branch `feat/business-intake`, base `086eee485e6f40a8b02ea77c9dbeefe315c81596`.
Implementation is in progress; no B01–B18 or BI1–10 pass is claimed yet.

The accepted wire contract is `670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5`
(`docs/contracts/business-intake.md`, final report handoff `7f4d4dbc`), with canonical
access `18be55edc276713fc6d46d075baec363245ba285`. PR25 merged as `aa16a9b9`.
Independent review `2a765db2` found no open contract blockers. Those are contract
reviews, not implementation evidence.

## Work and integration contract

- Single `business_intake` module/access, sole migration
  `m20260908_000011_business_intake.rs`; existing Principal, writer and credential store.
- Fireflies protected setup → explicit grants → bounded read/import → exact human
  preparation → atomic shared task is first. Email/Hafidh protected projections
  and monotonic configuration fencing follow within this assignment.
- Rebrand owns the frontend. The accepted camelCase POST `{input:...}` DTOs and
  slash-to-underscore native command names remain authoritative. Binding capabilities
  do not imply connectivity, and task-domain access does not grant private sources.
- The first compiling checkpoint contains strict credential mutation reads and
  task-owned transaction extraction. Further checkpoints will publish concrete module
  and route types for UI wiring and immutable independent review.

## Provenance and docs first

Complete current FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS, implementation scope,
both intake contracts and existing identity/task contracts read before edits.
React19.2.4 package and Cargo manifest read in separate commands first. Applied
code-context using existing rag-skills `.venv/bin/python`, `HF_HUB_OFFLINE=1`:
guide exit0; docs exit3 because `data/code/tickets.db` is absent. No corpus-backed
API claim is made; local pinned source is the authority. Guide's reuse rule is
applied without subagents, per owner instruction.
Live hook metadata confirms PreToolUse and PostToolUse at1788892494, exit0,
session `01a07c1c-d82f-7022-84db-778a438632f1`, this worktree. Hooks remain enabled.

Existing Apache Codeg credential adapter at base086eee48, `keyring_store.rs`
blob `29fc3fb38280338aa26939c45f80ef9aefc2a394`, supplies the existing process-local
lock and private atomic rename path. The narrow correction refuses mutation on
malformed/unreadable storage. Native keyring and SQLite are still separate resources.
Task extraction retains the existing Apache/approved IntroMail task authorization
and attribution; no copyleft task code is imported. Full intended Fireflies source
ledger remains in `reports/business-intake-contract.md`; actual ports and NOTICE
will be recorded as implementation lands.

## Commands and limits

- Tracked clean check, fetch and new branch from accepted main: exit0. The paused
  untracked `reports/visual-correspondence.md` is preserved (SHA256
  `a2120d9cc87dbb4823b1223e02d377660fff370d846f2fd102fe1eb36599a50d`).
- Existing fixtures, browser sessions, exports, targets and bundled executables are
  preserved. New compilation uses an isolated target; no fixture is started yet.
- No provider/model/engine/email/issue action, dependency or configuration change.
- `CARGO_TARGET_DIR=$WORKTREE/.docs/business-intake-target cargo check --locked
  --offline --no-default-features --bin codeg-server`: exit0 (initial64s,
  updated helper source29.8s). Helper checkpoint has two temporary unused-function
  warnings until intake consumes prepare/link; no lint waiver added.
- Same isolated target, `cargo test --locked --offline --no-default-features --lib
  intake_`: exit0, **21 passed/1 manual ignored**, including four new strict-store
  and task-transaction regressions; execution2.35s. Existing host cases matched
  the selector. Logs in `.docs/business-intake-logs/`; linker unwind-size and
  proc-macro-error2 future-compatibility warnings are disclosed.
- New focused names: `intake_store_mutation_preserves_malformed_and_unreadable_stores`,
  `intake_store_missing_file_and_staged_cleanup_preserve_other_entries`,
  `intake_task_transaction_rolls_back_creation_and_preserves_explicit_editor_owner`,
  `intake_task_source_link_is_atomic_text_free_and_invalidates_review`.
- Planned validation: strict-store failures, current identity/grant/source fences,
  source refresh races, durable recovery, exact atomic decisions, all relevant existing
  business/Ops regressions, desktop/server/companion checks and Clippy. Guarded
  synthetic fixture/port coordination and actual Playwright CLI follow implementation.

First compiling prerequisite SHA: `60daf42e79fa7dc10f8118b9cdb8a73b2c07e80d`.
Draft PR: https://github.com/Adanmohh/codeg/pull/28 (main; do not merge yet).

## Closed DTO/schema checkpoint

`business_intake/types.rs` transcribes the accepted inputs/results, including
write-only keys, explicit nullable fields, non-null optional credential replacement,
page defaults, publication capabilities and restricted decision targets. The module
and sole migration000011 register additively; endpoints are not exposed yet.
Migration uses the existing explicit transaction/foreign-key/retained-history pattern.
Reader/access/claim implementation is in progress separately; no readiness control
is enabled by a schema checkpoint.

Locked offline server check: exit0,36.87s; six temporary unused-consumer warnings
remain (task prepare/link, three intake error helpers, write-only secret field).
`cargo test --locked --offline --no-default-features --lib business_intake::tests`
with the same new isolated target: exit0,2 passed/0 ignored,0.04s execution.
These cover closed/spoof/secret-header/null input rejection and fresh migration
with nonexistent-owner FK rejection and five table registrations. The latter is
not an actual existing cross-org-member or populated-row-retention test, despite
the initial test name; neither claim is made. Populated upgrade and real cross-org
references remain planned, as does complete B authorization.

Root reserved UI4350, own synthetic backend4351/upstream4352. Availability will
be checked immediately before a coordinated launch. No new listener exists yet.

## P2 strict-store permission correction

Independent review `473b3b316abdac67bacf09d4fef011ba3ec24ff4` found that
`read_tokens_for_write` at `60daf42e` skipped the inherited pre-read Unix0600
hardening. Read the full store and independent synthetic probe patch. The bounded
fix shares the existing hardening helper between both readers and calls it before
strict read under `TOKEN_WRITE_LOCK`. Only files are hardened; the directory
read-failure probe keeps its contents and traversal permissions. True NotFound
alone initializes an empty map; malformed/read failures still preserve all bytes.
No native keyring behavior or authorization changed. This reuses the existing
Apache source and test pattern already attributed above and in NOTICE.

Same isolated target: `cargo test --locked --offline --no-default-features --lib
keyring_store::tests` exited0: **10 passed/0 failed/0 ignored**, execution0.23s,
compile44.89s. Log: `.docs/business-intake-logs/tests-strict-hardening.log`.
New regression `intake_store_strict_read_and_rejected_mutations_retain_0600_hardening`
checks valid0644 strict reads plus rejected malformed set/delete, byte preservation,
0600 mode and no temporary residue. Existing directory-failure, concurrent mutation
and legacy hardening cases also passed. Independent probe rerun remains reviewer-owned;
no actual credentials, fixture, target outside the owned new target or A bundle touched.

Correction SHA: `74bde8b6f1aeee135122cf52f78746eec36c6602`. Independent review
`435b1c046ed0dc0d889e2b47c1527196ffa823e0` closes R1: original strict-store3,
task2, protected-router1 and identical permission probes2 all pass, eight total,
exit0. Those reviewer runs certify the narrow prerequisite, not the later B core.

## Protected setup / reader checkpoint

Implemented the eight binding/grant operations over the existing business HTTP
middleware and matching native command names. Zero initial grants; member-owner
credentials and agents cannot configure sources. Current grants intersect current
source/destination identity permissions. Owner revision drift pauses use until
protected revalidation; setup authority alone gives no source read.

Fireflies uses the fixed MIT adapter user/list/detail queries, an immutable staged
credential reference,12-second provider budget and15-second core budget. Setup
activation checks current Principal, owner revision, binding revision/epoch and
attempt expiry in the writer. Same-provider rotation preserves an active key on
failure. A private ledger supports bounded later cleanup of only proven unreferenced
keys. It does not claim atomicity between SQLite and the existing credential store.

Pure legacy projections now narrow the accepted account/inbox/public-message and
account/product/TestFlight stores. Email type8 ingested normalized content and
type0 authored public text use the identical public-note/activity predicate;
addresses/headers/attributes are excluded. Hafidh includes only title/description,
revision and existing expiry, never reporter, proof, config or a freshness write.
Retained configuration counters fence away-and-back changes, including host-folder
and repository binding changes. In-place email key changes bracket store I/O with
a persisted changing flag and counter increments; a crash remains fail-closed.

Actual Fireflies port and complete MIT licence are now in NOTICE:
`firefliesai/n8n-nodes-fireflies@fbd24607bc784a2294ce402426aefe2cb8c00f50`,
`helpers/queries.ts`, `credentials/FirefliesApi.credentials.ts`,
`transport/index.ts` and `LICENSE.md` (full paths/blobs in the accepted source ledger).
Pure legacy/transport fixtures adapt the exact existing Codeg files at086eee48
listed in NOTICE. No new dependency, provider mutation or copyleft source.

Tests so far: the first setup/reader selector passed10/10,0ignored,exit0,0.39s
(compile97s). Added populated migration/foreign-installation-owner and legacy cases
then gave12pass/2fail: a projection initially rejected inherited email type8;
source-read correction admits exactly types0/8. The Hafidh test kept a single-pool
read transaction open before a separate count; the test now closes it first.
Focused `business_intake::tests::legacy_cases` recheck exited0:2passed/0ignored,
0.10s (compile37.29s). The other12 cases passed before that correction, including
populated task/ticket retention and atomic migration-DDL rollback. The actual
foreign-org owner case uses a real second organization's member in a separate
temporary database; accepted v1 permits one organization per backend. It does
not bypass the singleton constraint to fabricate a second organization in one DB.
Initial HTTP compilation also caught Arc coercion and test-module path errors;
both were corrected before the10-pass run. Logs retain these failed iterations.

Server check with registered setup routes exited0,16.30s, before the narrow email
projection correction; updated locked/offline check exited0,9.01s. Existing
`ops::tests` selector also exited0:22passed/3 manual ignored,1.68s, including
scope/privacy, exact review, cancellation, unknown/no-resend and receipt recovery.
Logs: `check-setup-checkpoint.log`, `tests-setup-legacy.log`,
`tests-legacy-corrected.log`, `tests-ops-setup-checkpoint.log` in the owned log directory.
Temporary unused source/import/candidate consumers remain until the next slice
(32 server warnings,17 test warnings including the inherited linker warning).
Native runtime/checks, strict
Clippy and all full B fences/atomic decisions remain pending. No manual fixture,
browser, old target/export or accepted native bundle was changed.

Compiling setup/reader source SHA: `67708b0768cae3cacecd0dd1989c57bb565ab5f0`
(PR28). Shared with the frontend owner and independent reviewer.

## Source/import checkpoint and new tenancy steering

Source/import core now compiles: immutable normalized versions, explicit source-wide
refresh fences, durable human-claimed imports, current-Principal revalidation after
one read, and read-only source projections. Eight source/import HTTP operations use
the existing authenticated Principal. Locked offline server check exited0,11.67s,
log `check-imports.log`; runtime cases for this slice are pending. No complete B
workflow or race acceptance is claimed. New native registrations are held for the
published tenancy selection seam; existing setup registrations are preserved.

Owner now requires true multitenancy and tenant-managed UI. Root/identity own that
contract; this worker continues only tenant-neutral core taking the existing private
Principal with explicit organization filtering. No new bootstrap/identity constructor,
fallback or broad auth migration is being introduced. Existing B work is preserved.
Precise seams to close, read at base086eee48/current67708b07:

- `business_identity/store.rs:104,175` and migration000009: organization selection
  and bootstrap use the singleton=1 row; `context` also loads that one organization.
  This must become explicit authorized tenant selection/provisioning. Current
  two-database foreign-member tests do not certify two tenants in one backend.
- `business_identity/mod.rs` (`operator_principal`, `current_human`) and
  `business_identity/http.rs`: protected operator bearer currently selects that
  singleton owner. Native `commands/business_identity.rs`, `business_tasks.rs` and
  `business_intake.rs` call this global owner seam. A server operator and a tenant
  administrator need the new contract's explicit distinction; do not infer one
  tenant or authorization from UI choice alone.
- Existing member credentials already store organization/member lineage; all intake
  writer calls use `begin_write(db, principal.organization_id())` and recheck that
  Principal. Binding/grant/source/import/candidate/decision/link/receipt keys and
  foreign keys carry organization IDs. Preserve those scoped checks and original
  credential revocation when tenant selection changes.
- `ops/mod.rs` still resolves a process-level CODEG_OPS_ACCOUNT_ID/default1 for
  legacy operator transport. B member use does not call that constructor, but B
  setup's verified-operator resolver currently consumes it. Tenant→legacy account/
  inbox/product resolution must replace that global selection before multitenant
  enablement. Existing `ops_intake_host_product` and repository bindings are also
  global product IDs/account tuples; B's monotonic fence detects changes but does
  not create tenant ownership for those legacy records.
- The existing credential adapter is one OS-keyring/file store and process-local
  writer lock. B keys have backend-generated unique references in an org-scoped
  staged ledger, never caller refs. Tenant admin setup authority, provider resource
  associations and shared-backend secret custody must be specified independently
  of desktop local-owner credentials; UI theme/settings are not authorization.
- `business_tasks/agent.rs` and `work_task/desk.rs:72` preserve reviewed separate
  source entrustment, exact task/run/root/agent and original human delegation.
  Engineering work_task/folder/engine indices remain legacy local resources without
  native tenant ownership. A central tenant task must not acquire an arbitrary local
  executor just from matching IDs, generic CLI type or a reconstructed operator.
  B adds no agent source grant, task execution, privileged CLI or MCP authority.

Per-tenant UI preferences/navigation are frontend/identity contract work, not B
source permission. No global store, local filesystem or engine is claimed to be
an OS/multi-tenant sandbox. No tenant migration or new framework is implemented here.

Source/import compiling SHA: `4a194500b76b97aa5caaf9434ce5c1f16e54ea48`, pushed to
PR28. Its new focused selector `business_intake::tests::import_cases` passed
**5/5,0 ignored**, exit0,0.43s execution/58.02s compile, same isolated locked/offline
server-library target. Log `tests-imports.log` covers deduplicated discovery and
replay without another HTTP read, nullable pre-detail revision, identical/A→B→A
versions, retained candidate identity, explicit cancellation and expired claims,
cross-import source fencing, grant revocation, dropped caller future, three-attempt
retry budget and capped five-page duplicate discovery. These synthetic loopback
tests create no business task or persistent fixture; publication is still pending.

Independent review `d92d1b9534025f994a85f070d3bb78174c876e62` at67708b07 reports
setup5/reader3/legacy2/migration2 passing and749/749 source blobs matched, with no
new concrete blocker in that scope. Late/uncertain credential cleanup and complete
publication remain gaps, not certified by those12 passes or the newer five tests.

Owner clarified the tenant experience retains the rich chat/terminal workspace,
tabs and split panes alongside task/table/board/calendar views. This checkpoint
adds no member access to legacy terminal/session/host APIs. Tenant/session/host
isolation and explicit tenant selection must be reviewed before those capabilities
are exposed; a simplified separate dashboard is not the assumed product contract.
