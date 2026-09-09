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

## Exact candidate publication checkpoint

Nine candidate/task-source HTTP operations now consume the same explicit Principal
and organization-scoped writer. Select preserves null/prepared drafts and rebases
only exact current passage IDs. Edit uses the task-owned preparation helper with
explicit editing-human owner and unchanged calendar dates. Accept has no task body:
it creates the saved reviewed text through the existing task helper. Text-free link
uses the existing target visibility/edit/domain/CAS helper and invalidates review.
Task, candidate terminal state, private decision/link, receipt and audit share one
transaction. Historical decision rediscovery independently checks target Read;
public task/source links without source grants reveal only an opaque link ID.
No agent source access, new grants, run launch or terminal capability is added.

Locked/offline server check exited0,12.92s (`check-candidates.log`); four unused
private persistence fields remain to remove or consume before strict Clippy.
Focused `business_intake::tests::candidate_cases` exited0: **7 passed/0 ignored**,
0.40s execution/68s compile (`tests-candidates.log`). Coverage: exact human draft,
editor/decider identity separation, calendar deadline, no automatic transcript
publication, identical receipt replay, task-only reader restriction, null-draft
text-free link/review invalidation, destination-Read withholding, expired metadata,
source revision and grant epoch fences, invalid old/duplicate/passages, closed
publication input, injected decision-insert rollback and real two-connection CAS
and original-credential revocation. Test import warning removed after that run;
no production correction was needed by these seven tests. Extra-candidate paths,
full HTTP/native parity and late/uncertain staged cleanup remain pending.

Read approvals' exact tenancy proposal `a2be945d9869c9807a5e3f4d7a98dab1fc2cf2cf`
through gh api. It preserves the current private Principal, UUIDs and grant epochs,
separates tenant-owned provider connections from platform legacy-resource entrustment,
and names the companion's legacy unrestricted session-read path as a new-target
prerequisite. This checkpoint changes none of those platform/tenant/native/CLI
boundaries. Edublend/Payload source has not been copied into this implementation.

Candidate production checkpoint: `e9c6323760497e7b294282d2746cfe1d9b2774fa`, pushed
to draft PR28. Earlier import regressions/report: `5de1176beb4778604b26df65f2ec669b0ab7987b`.

## Tenant epoch integration handoff

Read the full accepted identity/settings proposal at
`af00c956787142f900384f7ba6b34ebbc005eb88` through immutable gh api. Root assigned
approvals sole identity/platform/settings/native ownership and forward migration000012;
this worker retains B/migration000011 and will independently review the published
tenancy head on a separate branch. B is committed and tracked clean before that
transition; the paused visual report remains untracked and preserved.

Direct helper coordination requested from identity: a read-only accessor for the
Principal's **captured** authorizationEpoch. Current B clones/passes the same
Principal across provider reads and final writer; it must never reconstruct or
refresh it. Current identity authorization can then reject old in-flight work on
suspension/resumption. A second, distinct persistence requirement remains: B source
observations, candidate previews and staged/attempted work currently pin the binding
access epoch only. A freshly authenticated post-resume Principal must not accept
an older observation/preview solely because that binding epoch did not change.
B needs an explicit captured tenant-epoch fence in retained storage or an agreed
monotonic binding invalidation seam. No epoch=1/default fallback is implemented.
Migration000011→000012 registration/retention must preserve both owners' records;
no existing fixture is migrated. This new-target gap is not a failure of the
separately pinned pre-tenancy runtime tests and is not claimed implemented yet.

New native command registration and tenant-owned connection administration remain
pending the actual identity/session helper checkpoint. Platform entrustment of
legacy account/inbox/product/host resources must remain a separate capability;
no automatic grants, resource reassignment or stored-credential reconstruction.
The rich tenant workspace does not gain legacy session reads, files, global events,
terminal, child delegation or arbitrary CLI authority through this B checkpoint.

## Bounded recovery test checkpoint

Production remains `e9c6323760497e7b294282d2746cfe1d9b2774fa`; this checkpoint
adds only four synthetic recovery tests, their blocking-store fixture gate and
this report. It does not change identity, setup, candidate or publication code.
`business_intake::tests::recovery_cases` passed **4/4, 0 ignored**, exit0,
0.20s execution/57.91s compile. Exact command, from this worktree's `src-tauri`:

```sh
env CARGO_TARGET_DIR=/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-intake-target cargo test --locked --offline --no-default-features --lib business_intake::tests::recovery_cases > ../.docs/business-intake-logs/tests-recovery.log 2>&1
```

- `intake_recovery_extra_candidate_discard_history_and_explicit_new_work`: an
  extra human selection has no prepared draft; identical creation/discard replays
  recover the same records, changed creation input conflicts, expired disclosure
  permits discard but blocks new selection, and reimport preserves terminal
  history. Further work requires explicit creation. No business task is created.
- `intake_recovery_uses_current_credential_grant_and_real_passage_scope`: a real
  passage belonging to a different binding/source is rejected. Revoking the
  original member credential blocks receipt recovery; an explicitly issued fresh
  credential for that member permits recovery through normal authentication.
  Source-grant revocation still denies access, and explicit regrant reveals only
  retained metadata until a new source refresh. Replays create no extra decision.
- `intake_recovery_late_blocking_store_after_abort_and_cleanup_retries_only_orphan`:
  the actual `Services::set` blocking task waits in an injected synthetic adapter
  while its caller is dropped. Cleanup retires its expired staged reference; the
  delayed write then completes. A failed delete retains that orphan for a later
  successful cleanup. The active and unrelated keys remain unchanged, with no
  provider verification after the dropped request and no initial grants.
- `intake_recovery_activation_commit_failure_and_lost_response_preserve_active_reference`:
  a deferred foreign-key fixture first proves INSERT succeeds and COMMIT fails,
  then injects that failure at the real activation transaction. Binding revision,
  active reference, receipt and audit remain unchanged. A later cleanup removes
  the retired staged key. Separately, discarding a successful rotation response
  and replaying the operation recovers its durable receipt without another
  provider read or deletion of the new active key.

Installed Tokio1.49.0 `task/blocking.rs` and `sync/{oneshot,notify}.rs grounded the
non-abortable blocking-task gate. SeaORM1.1.19/SQLx SQLite0.8.6 transaction sources
and libsqlite3-sys0.30.1's bundled SQLite3.46.0 `sqlite3.c` deferred-FK comments
grounded the commit-time fixture. These are API references, not copied dependency
implementation. The existing Apache Codeg086eee48 transaction/test-helper mapping
in NOTICE applies; no new package, lock change or third-party port is introduced.
Rustfmt and `git diff --check` exited0. The run retains four known unused private
model warnings, the linker unwind-size warning and the existing proc-macro-error2
future-compatibility notice; it is not a strict Clippy pass.

Limits: a failed COMMIT and a discarded successful response are exercised
separately; a process crash or durable successful COMMIT followed by an ambiguous
driver acknowledgement is not injected. Ledger expiry is advanced synthetically,
not by waiting through the full request deadline. No OS keyring, real credential,
native fallback, manual fixture, provider/model/engine, existing target, export or
bundle is touched. Earlier passing selectors were not repeated. Full B HTTP/native
parity, tenant lifecycle/epoch persistence, desktop/Clippy and integrated UI
acceptance remain pending.

Read the full immutable tenancy contract
`7516461633c163c2ac683930487e33231e630b0b` through gh api. Its fixed
`Principal::authorization_epoch() -> i64` is a read-only captured value; there is
no refresh helper. B will persist and check it on setup/attempts, observations and
previews so fresh authentication after suspend/resume cannot revive older evidence.
Missing retained epochs require explicit fresh validation, never a default value.
Identity's compiling source has not yet been supplied; migration000011→000012
integration and the separately assigned exact-head tenancy review remain pending.

Recovery log SHA256: `fbb04afad9b24ecb1442aeec8975d0fae6163ecf4bfd2e936a3787f84a17b20e`.
New test file SHA256: `80758fe6708ae1dde4c92ccbf038f10bcf37700b25f2efd98c53c49b1dfac1ce`.
Paused visual report remains untracked with its original SHA256
`a2120d9cc87dbb4823b1223e02d377660fff370d846f2fd102fe1eb36599a50d`.

Recovery checkpoint committed/pushed as
`6513dc1545841af20f4d9d46e8a64ed6aec83cbb`; diff from production e9c63237 contains
only the report and three test/test-support files. Identity has now supplied the
first compiling immutable head `f3b408dae5c724f354763961d79a17a7ae5c86f8` (PR30),
implementing the captured accessor, settings and migration000012. B is preserved
here while this worker switches to the assigned independent exact-head review.
Platform/native runtime is not yet exposed at that checkpoint; no acceptance of
those missing surfaces or of B's future persisted epoch integration is implied.

## Reviewed tenancy integration — September 9 checkpoint

Returned from preserved `review/business-tenancy` at
`08dca9dd04dcec2106078fe53ff8b9e785bbd512` to intake checkpoint
`15bb402b9265006930cc9ce4428332d04c9fb34a`. Integrated immutable PR30 handoff
`b3dfbcb602314cee6eb0039d92cb997762c02dd8` (gh api verified tree
`4271c2bf0c647729276ad6cac1951a5d7b0e35e6`; reviewed product29774b50).
Four additive conflicts retain both NOTICE sections, intake/settings routers,
command modules and migration000011 before000012. No dependency or lock edit by
this integration; PR30's already reviewed test-utils feature is retained.

New isolated target `.docs/business-intake-tenancy-target`, log
`.docs/business-intake-tenancy-logs/check-integrated-server.log`:
`cargo check --manifest-path src-tauri/Cargo.toml --locked --offline
--no-default-features --bin codeg-server`, exit0, 1m09s. Four prior private-model
unused-field warnings and proc-macro-error2 future-compatibility notice remain;
this is a compiling integration checkpoint, not final Clippy or B acceptance.
Existing targets/fixtures/exports/bundle and the untracked visual report are intact.

Next persisted-authority correction: setup reservations, import attempts, source
observations and candidate previews store the captured Principal tenant epoch.
Null retained epochs never default to1/current. They require explicit fresh
validation; a fresh login after resume cannot rescue an old claim/preview.
B-owned nullable-column upgrade will run inside000012's pinned writer transaction
when old000011 tables exist, preserving histories and migration-receipt retries.
Fresh000011 will create those same columns in its own transaction. This plan was
sent to approvals for the migration boundary; no passing epoch test is claimed yet.

The existing25 POST operation/input DTOs are unchanged here. Tenant Fireflies
connection administration will require current owner/admin identity and domain
ceiling, with zero initial source grants. Legacy account/inbox/product entrustment
remains actual protected platform authority; no implicit operator Principal or
host resource ownership is added. Exact additive setup capabilities and native
same-session wrappers will be published before the guarded fixture/UI handoff.
Native restricted windows stay unavailable and no legacy terminal tool is exposed.

Docs-first: separate React19.2.4 package and Cargo manifest reads; complete current
governing docs and intake/access/tenancy contracts; pinned SeaORM1.1.19,
SQLx0.8.6 source read. Offline code-context guide exit0; dependency docs exit3
because tickets.db is absent. The cross-project per-tenant-DB advice does not
replace this accepted shared-DB contract. Live own-session hook audit recorded
PreToolUse22753/22755 and PostToolUse22750–22754 for this worktree/session
`01a07c1c-d82f-7022-84db-778a438632f1`. No hooks were bypassed.

## Captured epoch and tenant Fireflies checkpoint

Compiling merge checkpoint is `a9a610a8b60c0aa2a8e2a4a873f9c278ec28750b`,
pushed to draft PR28. Root subsequently accepted/merged the same PR30 handoff.
The integration mechanism now follows root's reserved forward
`m20260909_000013_business_intake_epochs`; accepted000012 is byte-identical.
Migration000013 follows000011 and000012, so an already-recorded A-only000012
installation still runs newly pending000011 and000013 through SeaORM's actual
pending-name set. Old populated000011 receives nullable columns in000013;
there is no epoch backfill/default. Per-column checks and one writer transaction
support schema-commit/receipt retry. New000011 also has an atomic schema marker
for its own schema-commit/SeaORM-receipt retry; no existing history is rewritten.

Four private nullable `authorization_epoch` columns fence setup reservations,
import attempts, source observations and candidate previews. New writes use only
`Principal::authorization_epoch()`. Final setup/import writers keep the same
Principal and validate captured storage; source disclosure and preview rebase
compare persisted epoch even after a fresh login. A stale running import is
reported waiting and can be explicitly reclaimed under fresh current authority;
its old result cannot commit. New detail observation does not reauthorize an old
preview: explicit candidate select/edit remains required. Old setup reservations
are retired by authorized bounded orphan cleanup, never reactivated.

Tenant Fireflies administration now uses current human owner/admin permission,
current source-domain Contribute and publication Create ceiling. Binding list
adds response-only `setupKinds: SourceKind[]` and `setupDomains: Domain[]`;
`canManageSetup` reflects a nonempty effective setup scope. Tenant credentials
receive only fireflies; the original actual operator may manage legacy kinds.
BindingView.admin and every setup/grant mutation check the specific kind/domain
scope; foreign/hidden bindings remain undisclosed. New bindings still have zero
grants. Setup, source ownership and tenant administration confer no source read.
Legacy resource association remains actual protected operator transport in the
original mapped tenant; the separately scoped platform-to-new-tenant legacy
entrustment is not yet exposed. No fallback is added. Native B command expansion
and final fixture are still pending this checkpoint.

Locked offline server checks in the new isolated target: epoch correction exit0,
9.53s (`check-epochs-server.log`); tenant setup/forward migration exit0,14.21s
(`check-tenant-setup-server.log`). Same four private-model warnings remain pending
Clippy cleanup. Focused two-tenant/epoch and real migration receipt tests are being
written; no passing claim yet. Planned selectors `business_intake::tests::tenancy_cases`
and `business_intake::tests::epoch_migration_cases`. A broad rustfmt check reported
inherited formatting differences (exit1); no files were changed by that check.
Subsequent formatting is restricted to changed owned files.

The recovered approvals/rebrand same panes received the DTO/migration checkpoint
via wR:p3 and wR:p2. Their old name aliases were absent (two earlier prompt errors);
no suspended process or fixture was touched. Runtime4351/upstream4352 remain off.
