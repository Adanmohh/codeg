# Independent B and tenancy integration review

**The bounded B review found no new blocker. The real unified API pass completed
62 requests/278 assertions, and IUI-1 is closed by the actual protected browser
flow at UI1974d97f/backende55f3bfd.** Eight integrated backend tests at949adb02
and the earlier16 B core tests remain separately recorded. This is not live
provider, full business-AI, native tenant-window or final Design acceptance. Own branch
`review/business-integration` starts at accepted main
`be46d0544f5fc5f33d3785ac5a04c5d1fd2c1935`. The completed
`feat/business-tenancy` branch remains pushed at
`b3dfbcb602314cee6eb0039d92cb997762c02dd8`; its product29774b50 is accepted in
main mergeb3f2f6d03. This review makes no product edits or native-window enablement.

## Frozen inputs and preservation

- B checkpoint `15bb402b9265006930cc9ce4428332d04c9fb34a`, tree
  `9a30f085bdc346e08c884f6519e53d73dc3a4357`; product
  `e9c6323760497e7b294282d2746cfe1d9b2774fa`. Full owner report read. Earlier
  independent67708b07 review covered only setup/reader/legacy/migration, not the
  candidate/import/recovery implementation now under review.
- PR29 report `191f69b3c2b827ce2f712bfcad1f995a201981bb`, tree
  `bc756e23665015e53e3565f21368e9a25b85a67e`; product
  `c1e618dede15368d4af39ac74391f18931b317a5`. Full report read. Its4350 preview
  currently uses a separate4353 task/settings DB and unavailable4351 intake;
  that is not integrated B evidence. Existing owner sessions/records are untouched.
- Both source pins were verified through `gh api`; own fresh archives are
  `.build/business-integration-review/{15bb402b,ui-191f69b3}/source`.
  Initial blob correspondence: B756/756 Rust/Pi/NOTICE/LICENSE;
  UI1229/1229 src/NOTICE/LICENSE. Full manifests are in each archive's
  sibling evidence directory. No uncommitted worker code copied.
- Fresh session `01a084ee-129a-7561-9f10-64266115e759` emitted own-worktree
  PreToolUse22873/22875 and PostToolUse22874/22876 in the live docs-first audit.
  Old stopped PID49733 and inherited fixtures were not resumed, killed or polled
  through old exec IDs. All previous targets/exports remain unchanged.
  Paused visual report SHA256 `abc5fd394af657568abe32a5333df27210115204c4e7eb2630ac59b71caa4060` remains in place, untracked.

## Bounded gates and ownership

Root reserved tickets-owned **m20260909_000013_business_intake_epochs**.
Accepted000012 remains unchanged. The planned nullable captured-epoch fields
have no backfill/current-epoch default; missing retained authority requires fresh
validation. A helper solely inside000012 would miss an installation that already
recorded it. Installed SeaORM1.1.19 `migrator.rs` filters pending names against
recorded receipts, iterates pending registry entries, calls each up, then inserts
its receipt; tests must use this real path, not call a helper alone.

| Gate | Required positive and negative evidence |
| --- | --- |
| Migration paths | Fresh combined; populated old000011→000012→000013; already-recorded A-only000012 then add B000011/000013. Verify actual receipt/order/retry, FK enforcement, retained rows/NULL epochs and rollback without relying on last migration index. |
| Captured authority | Same Principal across await/final writer; suspend/resume rejects old setup/import work. Fresh credential permits new work but cannot disclose or accept old observations/previews. Explicit refresh/rebase restores only the intended scope. |
| Two tenants | Same DB, real member credentials: own binding/grant/import/source/candidate positive controls; foreign IDs, receipts, member references/counts and actor/org spoofing denied. Tenant Fireflies admin remains separate from actual platform legacy entrustment. |
| Claims and source ordering | Two imports share a source fence; expired/cancelled/older responses cannot commit. Original credential/member/owner/grant changes fence final writes. Durable rediscovery uses the current human, never reconstructed identity. |
| Publication/recovery | Exact reviewed draft, current destination Read/Create/Assign/edit/CAS; task/decision/link/receipt/audit atomicity; same operation replay creates no duplicate; terminal target redaction and private-source audience remain independent. |
| Storage and projections | Staged late/uncertain secret cleanup deletes only proved orphans. Email/Hafidh projections exclude private notes, reporter/proof/config and do not refresh or send. Resolved strict-store tests are not repeated without a changed concern. |
| Shared tabs/panes | Actual session/tenant switch aborts in-flight requests and clears private caches/drafts/selection/portals. Cancelled disconnect and same-tenant locale/viewport/pane/settings changes preserve unsaved edits. No private persistent browser storage or legacy API/engine fallback. |
| Settings and source UI | Current tenant-scoped palette/layout, role/CAS conflict and lost-response recovery; stale/withheld passages/draft never shown as fresh; explicit confirmation resets on payload/target/revision changes. Protected API/browser evidence uses one integrated DB and separate synthetic records. |

Compiling integrated head and focused selectors requested from tickets. The
preparation-only statement about no test target is historical: the independent
executions below now cover the previously unread B core and one new UI regression.
No persistent fixture or browser was started. Provider-loopback/Playwright CLI
checks follow a coordinated immutable fixture
handoff; no intercepted frontend JSON will count as real B acceptance. No broad
unchanged A suite, live providers/models, new dependencies or workers are planned.
Native shared-channel isolation stays outside this increment and unavailable.

## Grounding for the initial preparation

Current governing documents, accepted intake/access contracts and BI1–10 read.
Separate installed React19.2.4 package/Cargo manifest reads ran. Code-context
used existing rag-skills Python with HF_HUB_OFFLINE=1: guide exit0; docs exit3
because approvals.db is absent. Installed pinned source is used directly;
other-project database advice does not override the accepted shared-DB contract.
Git/archive/Herdr help read; archive and immutable pin checks exited0. No
implementation/runtime test pass is claimed from this preparation checkpoint.
Borrowing remains approved Codeg Apache/IntroMail and exact Fireflies MIT source;
no new source port is introduced by this report. Full changed-source/license
verification and focused execution will be recorded by exact integrated head.

## IUI-1 baseline and component review — historical; closed below

Frozen UI report/source `191f69b3c2b827ce2f712bfcad1f995a201981bb`, product
`c1e618dede15368d4af39ac74391f18931b317a5`:
`src/components/business-intake/workspace.tsx:184` applies a task-originated
`entry` directly with `setBinding`/`setSourceId`. It bypasses the same component's
`navigate`/`dirty` guard. Its keyed `SourceReview` at line222 then unmounts when
the source differs. `src/components/business/workspace.tsx:764` supplies this
entry from the task's source-link callback; the task editor's own discard guard
does not protect the existing destination source editor.

Trigger: open candidate A in Sources, change its Brief without saving, open a
task in another tab/pane, then follow that task's link to source B. Replacing
the Sources entry destroys A's local edit without a discard decision. The
independent component probe renders the actual SourcesWorkspace, SourceReview
and CandidateReview, fills `Synthetic unsaved source A sentinel`, then supplies
the same task-link entry for a different source. It fails at the required
`Leave this source review?` dialog; the failure DOM already shows source B.
No save/publication operation is used. This is executed mocked-client component
evidence, **not an actual protected API/browser result**.

Required fix: route incoming source entries through the destination review's
existing unsaved/pending-operation guard, or preserve a separately keyed draft
without implicitly persisting private content. Dismissing a discard prompt must
keep A and its draft; explicit discard may open B. Same-source entries and an
in-flight/unknown mutation also need bounded regression coverage. Recheck the
actual task-link flow on the integrated protected fixture after the fix.

Evidence: [probe](business-integration-review/ui-191f69b3/reviewer-source-entry.test.tsx),
[failed log](business-integration-review/ui-191f69b3/source-entry-regression.log),
[command/exit](business-integration-review/ui-191f69b3/source-entry-result.json),
[attribution](business-integration-review/ui-191f69b3/NOTICE).
One test, exit1, 1.15s test/2.18s runner time. All1,229 original archived
src/NOTICE/LICENSE blobs remain unchanged; only the reviewer test and cache
config were added. Rebrand and root were notified directly through existing
Herdr panes. Design Studio Saving changes and Tabs methods apply; no full-page
design score or broad A matrix is claimed.

## Independently executed B core at15bb402b

Complete source reads now include imports, sources, candidates, decisions,
access, their candidate/import/recovery tests and synthetic fixture primitives.
`records`/`common` and changed HTTP registrations/digest/NOTICE were also read.
The provider adapter's only change since the prior677 review adds its provider
revision to the normalized fingerprint. Task transaction helpers are unchanged.

In a fresh own archive and target, each command used `cargo test --locked
--offline --no-default-features --lib -j2`, the archived manifest, and the explicit
reviewer-only target `.build/business-integration-review/target-15bb402b`:

| Selector | Independently observed result |
| --- | --- |
| business_intake::tests::candidate_cases | 7 passed, 0 failed/ignored; exit0; 0.59s tests; initial cold build/run230.71s |
| business_intake::tests::import_cases | 5 passed, 0 failed/ignored; exit0; 0.42s tests |
| business_intake::tests::recovery_cases | 4 passed, 0 failed/ignored; exit0; 0.31s tests |

This covers exact stored publication, real two-connection decision CAS and
queued credential revocation, cross-import late-response fencing, cancelled/
expired claims, monotonic A→B→A versions, current-grant recovery, actual late
spawn_blocking secret completion and deferred-FK activation commit failure.
All provider traffic was fixture loopback with in-memory synthetic SecretStore;
no protected browser flow or tenant-epoch behavior is inferred from these tests.
The four existing private-model unused-field warnings and proc-macro-error2
future-compatibility notice remain visible in the raw logs, not waived.

[Exact commands/exits](business-integration-review/15bb402b/unchanged-results.json),
[full source correspondence](business-integration-review/15bb402b/source-before.json),
[post-run preservation](business-integration-review/15bb402b/source-after.json).
All756 original Rust/Pi/NOTICE/LICENSE blobs match after execution. Test processes
completed; no existing target, database, fixture, output or bundle was changed.

## Historical preparation atf51154b9

`gh api` resolved merge `a9a610a8b60c0aa2a8e2a4a873f9c278ec28750b`, then
product `f51154b948ac1ae74fa1bf6a7df495e7e12e05f2`, tree
`af801c25ec8bd33eead2ace30ef42efb04b58122`. Own f511 archive matches768/768
Rust/Pi/NOTICE/LICENSE blobs. Accepted000012 remains exactly blob
`a305a682bbf55e750a5d5156ac0aa4695aac08c1`.

The complete product diff was read. Forward000013 adds four nullable epochs
under one SQLite writer and checks FKs; per-column existence permits receipt
retry without replacing retained epochs.000011's new marker is only for a fresh
000011 schema-commit/receipt retry. Import finalization reuses the original
Principal; disclosure checks source epoch, and candidate decision checks require
explicit new preparation after epoch drift. Setup/grant management intersects
current tenant owner/admin, source domain and publication ceiling; ordinary
tenant credentials cannot administer legacy resources. Zero initial grants and
separate source-read authority remain present. Full current setup/cleanup source
was read, including late/uncertain activation and domain-scoped orphan cleanup.

Owner reports compiling server exit0 and forthcoming seven migration/tenancy
tests. Those tests are not yet committed at this frozen product and are **not
independent evidence here**. Await their immutable checkpoint, then run actual
Migrator upgrade/receipt/NULL-retention and two-tenant authority probes. Native
wrappers, integrated fixture/browser and final gates remain pending. No new
backend blocker established from this bounded source read; this is not approval
of the remaining B implementation or integration.

## Independently executed integration at949adb02

Frozen PR28 head **949adb02c18c72d0804eeb74b420520a7681ee86** was resolved
through `gh api`. Read the full new tenancy addendum, eight new test bodies,
native command module, all registration changes, retained migration fixture and
NOTICE diff. Original archived source correspondence is772/772 before and after
execution. Accepted000012 remains blob
`a305a682bbf55e750a5d5156ac0aa4695aac08c1`; the retained000011 test fixture
matches15bb source exactly, blob `87550f70c2197cd8333316161223599aa5c33886`.

Reused only the existing own `target-15bb402b`, sequentially with `-j2`; no new
large target or copied build output. Each command used locked/offline server-mode
library tests and the immutable949 archive manifest:

| Selector | Independent result |
| --- | --- |
| business_intake::tests::epoch_migration_cases | 4 passed, 0 failed/ignored; exit0;0.41s tests,169.425s first build/run |
| business_intake::tests::tenancy_cases | 3 passed, 0 failed/ignored; exit0;0.26s tests,1.172s command |
| business_intake::tests::tenant_http_cases | 1 passed, 0 failed/ignored; exit0;0.09s tests,0.375s command |

The real Migrator tests cover fresh combined installation, populated old000011
through000012/000013, and already-recorded A-only000012 before B installation.
Nineteen populated tables retain their original columns/values/cardinality;
failed later ALTERs roll back earlier ALTERs. Real SeaORM receipt-insert failures
exercise000011 and000013 reconciliation without replacing NULL or non-NULL
captured epochs. FK checks and downgrade refusal are asserted. Fresh columns
are nullable with no authority default. No current fixture was upgraded.

Runtime positives use actual platform-provisioned tenants and member credentials
in one DB. Tenant Fireflies owner/admin setup intersects source domain and
publication ceiling, starts with zero grants, denies a real foreign member FK
and foreign IDs, and makes source read depend on an explicit grant. Suspended/
resumed setup and provider reads retain their old Principal and fail final writes.
Fresh login cannot disclose an old observation; identical-content refresh still
requires explicit preview rebase before exact reviewed task publication.

The protected HTTP case uses production router/middleware/core and the synthetic
reader/store. It proves positive setup→grant→import→prepare→accept plus deduped
receipt, server-derived task creator and no automatic transcript publication.
Forged actor/org/epoch/credential fields are400, a member bearer is401 on the
original-token platform router, and foreign binding/source/candidate/import/task
references are404. Its three provider reads are local fixtures. These are actual
protected API tests, not a persistent browser fixture or live Fireflies evidence.

Owner report949 accurately retains its earlier36/1 and37/1 exploratory runs:
one superseded Owner-denial expectation and one403-versus401 assertion, corrected
without an auth product change. Those are owner evidence, distinct from the
eight unchanged independent passes above. The linker unwind-size warning and
proc-macro-error2 future-compatibility notice remain in the raw independent logs.

All25 native wrappers now resolve through the accepted invoking-window
`NativeSessions` seam and the same bounded core. Only a real operator principal
may construct the legacy Ops operator. The prepared tenant ACL and native
availability flag are unchanged. **Restricted tenant native windows remain
unavailable; the existing protected original-organization platform/native path
remains available.** This is a source review of the new wrappers, not a native
runtime or shared-channel isolation pass.

[Commands/results](business-integration-review/949adb02/unchanged-results.json),
[before772 blobs](business-integration-review/949adb02/source-before.json),
[post-run preservation](business-integration-review/949adb02/source-after.json).
All three command processes finished. Root was notified before the bounded
recompile; no old process, fixture, installed dependency or other target changed.

## IUI-1 correction and setup scope atdc14/a58

Read correction026c1e46 at committed **dc14e83b93529afab99e973ddbd21e0db79b851b**,
then the complete navigation/pending-operation correction at
**a58c004c7d5187e6455a452195985528ddf14bc7**, tree
`4add54a24195c6597b361cc8ee0c918c81fcc959`. Read both new test files and the
complete setup capability diff. The incoming entry uses the existing discard
guard. Newer intent clears the prior prompt/read; same-source intent preserves
the editor. Busy/unknown state reaches the parent guard, which disables discard
until the exact operation is resolved. Setup kinds/domains come from the protected
response; missing authority hides setup. Same-scope refresh retains an entered
key, while narrowed scope removes the form and releases it from component memory.

The **unchanged original probe** atdc14 reaches the required dialog, then fails
at its previously unreachable background `getByRole("textbox")` assertion.
The real modal correctly excludes that background from accessible queries.
That exit1 log is retained as a probe limitation, not another product finding.
The adjusted probe changes only this post-dialog check: original field must remain
mounted with the sentinel, then explicit **Keep editing** must expose the same
value. It passes1/1 atdc14, exit0,0.129s test time. The original draft-loss boundary
assertion and zero-mutation assertion remain intact.

At a58, the same adjusted reviewer probe plus the owner's unchanged4 navigation
race/recovery and8 setup-capability cases pass **13/13**, exit0,1.00s test time/
3.12s runner time. They cover old success/failure after newer source intent,
cancel/stay preservation, busy→unknown→exact-operation recovery, server-granted
setup kinds/domains and removal of out-of-ceiling destinations. These remain
mocked-client component tests; no actual source/provider write or browser is
involved. Vitest2.1.9/Vite5.4.21's inherited CJS warning is retained.

Original UI blobs remain exact: dc141230/1230 and a581231/1231. Added reviewer
files/cache are only in own archives. [dc14 original and adjusted runs](business-integration-review/ui-dc14e83b/digests.json),
[a58 command/result](business-integration-review/ui-a58c004c/source-entry-and-setup-result.json),
[reviewer probe attribution](business-integration-review/ui-dc14e83b/NOTICE).
At this component checkpoint, IUI-1 still required the root checklist's real
task-source link sequence on the coordinated unified protected fixture. That
closure is recorded below. No full B or final Design Studio acceptance is claimed.

Report/source `git diff --check` is0. Evidence staging reports2 only for final
blank lines in the six raw tool logs; their exact bytes are deliberately retained.

## Bounded source follow-on before the unified fixture

Read the complete949→**e55f3bfd1f069d6d6111370223993596b19ecb9b** product
and report diff; immutable tree `a7a386dea91540bf30c8bc69bef0b2a1782ad86b`
resolved through `gh api`. `Work::Detail` now boxes the same fenced Source value;
the claim, original Principal, provider inputs and final writer checks are unchanged.
`Reservation` groups the same six borrowed arguments, with identical INSERT values
and both call sites. Derived enum defaults preserve the prior `pending` and
`unfinished` choices and snake_case wire representation. The recovery test scopes
the same mutex assertions lexically before its subsequent awaits. No new concern
requires repeating the eight tests; no new independent runtime result is claimed
for this follow-on. Accepted000012 comparison with PR30 exits0, byte-identical.

The owner's full38/38 run, corrected desktop Clippy and focused recovery run are
explicitly owner evidence in its committed report. Root independently verified
the attached gate/source correspondence artifacts; this worker's unchanged949
execution remains separately frozen above. No new backend blocker established
in this source-only follow-on. At that checkpoint the unified fixture was still
compiling, without a listener handoff or persistent API/browser mutation by this
reviewer. Its subsequent source review is recorded below.

PR29 **514b5432b03667dc6a461593a224d80d14050aee** was also resolved through
`gh api`; read its full report/source/test/NOTICE diff from a58. Product
1974d97f narrows the source-owner selector to currently eligible contributing
humans and prevents submitting a no-longer-eligible selected reference. Backend
authority remains decisive. IUI-1's workspace/source-review/candidate code and
four race tests compare byte-identical to the independently tested a58 (exit0).
No extra UI test run is claimed for this small eligibility change; inspect it
through the forthcoming real setup flow. Existing4350 remains its old export;
owner has proposed a separate4354 frontend to protect its retained private draft.
No export, browser, credential file or listener has been changed by this review.

## Unified fixture guard review at7ed0dd0c

**No concrete fixture-guard blocker found in this bounded source review.** Frozen
fixture source `7ed0dd0c28f5065f1c37983f366cc6dcb0727e08`, tree
`4d263ead4226aee5457f70b3e3cb2ab84592cc8f`; handoff
`1042a0b79b869c7bf86aabf91a6b6292600d7732`. Both pins were resolved with
`gh api`. Production remains e55f3bfd. Read the complete507-line fixture,
recipe/runtime JSON, test registration/NOTICE diff and relevant pinned router,
reader, service injection, AppState/manager construction and file helpers.

- `tests/browser.rs:310` puts the exact path/method/origin guard outside the
  composed router. Installed Axum0.8.8 `Router::layer` also wraps the fallback,
  so the credential directory supplied to the production static service is
  guarded. Host/engine/terminal/file/bootstrap/execution-link paths are absent.
  No-Origin CLI requests still require the actual inner Principal/platform
  middleware; this is not an alternate authentication mechanism.
- `browser.rs:222` checks a separate random control token against its exact
  namespace. It only changes future synthetic detail responses; it does not
  alter source freshness, a tenant epoch or any database row. Owner-only stop
  and platform credentials remain in a separate file. Review consumes only its
  own file and namespace; lifecycle belongs to root/fixture owner.
- `browser.rs` injects an in-memory store that accepts only the four synthetic
  provider keys, with no existing-keyring fallback. `Reader::fixture` fixes the
  endpoint to the bound loopback address; its client has no proxy, redirects or
  retries and retains the12-second/2MiB bounds. USER/LIST/DETAIL strings match
  the production reader byte-for-byte; variables/provider principals are closed.
  These synthetic namespace keys and the shared counters are fixture controls,
  not a production provider ACL or same-user process security boundary.
- `browser.rs:340` uses create-new0600/write-all/sync credential files. Independent
  metadata-only `lstat` confirms the supplied review file is a regular0600 file;
  its contents had not been read at this source checkpoint. Both ports bind
  before new data; retained temporary storage is unique. AppState test
  construction does not launch an engine, delegation listener or channel.

All five949 source/blob attribution entries match Git objects. The additive
NOTICE preserves Codeg Apache-2.0 and the existing exact Fireflies MIT notice.
Fixture source blob `a8fd171918b6879b50c1d3cb79c7ec0db85b7b3d`, SHA256
`b37a13cfb68001fc644e5d8ddfdb340c9718c751cf1dce72a57b54a42e1cc52c`,
matches the committed runtime handoff. [Mechanical checks and metadata](business-integration-review/fixture-7ed0dd0c/source-review.json).

The handoff reports PID66200 on4351/4352, binary SHA256
`313d987c2c5eb159bef700e19b9dffc2f942385e76e3b9a9612cf78ad08866ea`,
zero initial bindings/grants/provider reads and six passing startup guards.
Those runtime observations are **fixture-owner evidence**, not independent
executions. This review launched no runtime, built nothing and made no HTTP
request. Root has now authorized own-review-namespace protected API checks;
actual IUI-1 browser closure still awaits rebrand's matching4354 asset handoff.
Production tenancy, native isolation and live Fireflies behavior are not
certified by this fixture guard.

Current session metadata at line1222 records gpt-6-astra/max, never/full access.
Own-worktree live hook records24833–24835 are PreToolUse and24836–24838 are
PostToolUse. No model downgrade, hook bypass, old process action or fixture
replacement occurred.

## Independently executed unified protected API

The review namespace alone was used on the stable4351 fixture. The credential
file was read in memory, with a regular0600 assertion; no value was placed in
an argument, prompt, source file or output. The Python3.12.13 standard-library
client connects directly to fixed IPv4 loopback with a20-second timeout and no
proxy, redirect or retry. Product source remains e55f3bfd; fixture7ed0dd0c.

`protected-api.py` completed **62 requests and278 assertions, exit0,0.4831s**:

- Actual owner/manager/viewer member credentials resolve the same review tenant
  and their own identity. Missing credentials and member-to-platform requests
  are401. Four caller authority/setup-field spoofs are400; manager/viewer setup
  is403. A real foreign tenant task ID is404. No other namespace was mutated.
- Owner Fireflies setup binds the exact synthetic provider principal, starts
  disabled with zero grants, and does not imply source-owner access. Explicit
  owner/manager grants and enablement permit the real bounded three-record
  import. Complete attempts remain discoverable through `view: all`; the empty
  source has no fabricated passages/candidate. Viewer private-source use is404.
- An exact human draft becomes the shared task with the authenticated creator.
  Stale candidate CAS is409 and leaves draft/revision unchanged. Repeating the
  same decision receipt returns the same task/decision with no duplicate. Viewer
  reads the deliberately published task text but receives a withheld source link;
  neither the public task nor activity response contains the private transcript.
- Revoking the manager's explicit grant denies its private read and changes the
  binding epoch. Prior observations become metadata-only. Fresh source validation
  alone does not renew an old candidate preview; explicit passage-only rebase
  retains an absent draft. The final own pending candidate A and linked task B
  were prepared for the actual browser regression, without executing any agent.

[Exact request/status and assertion evidence](business-integration-review/fixture-7ed0dd0c/protected-api-results.json),
[reviewer probe](business-integration-review/fixture-7ed0dd0c/protected-api.py),
[commands/exits and retained failed attempts](business-integration-review/fixture-7ed0dd0c/command-results.json).
The first two attempts stopped after the same three non-mutating requests:
the probe incorrectly assumed a JSON error, then an empty body, while pinned
`web/auth.rs` returns plain-text401. Both exact scripts/results are retained;
the final parser checks the actual status/content-type/body contract. These
were probe errors, not product failures or mutation retries.

Health counters are shared with concurrent root/UI consumers. They are retained
as observations, not attributed as exclusive per-reviewer provider counts.
All setup/import traffic used the injected synthetic loopback provider. These
checks do not establish live account access or production key-store durability.

## IUI-1 closed by actual Playwright CLI

UI handoff **ea8cf9c0e50b4b1c13fbe6c99e2103e819b4550b**, product
**1974d97f44b69db96051ace1c68717cc8c27b6e9**, built
`6739c476c197e6a00f1c50fc0dfaece8ad998869`. All27 manifest entries were
independently hashed on disk and over HTTP4354: every entry matches, including
HTML `42f37747c38d180ef270e59e0a7956bfc48c848ee35782e4a5b35c9c1ae2c6b7`.
Manifest SHA256 `9b53750c2b200d47cd20f5e5a7272005b5fac5a12ea3d5d9a6616c727ca377a1`;
`src` compares byte-identical between product and built head. The inspected
fixture proxy sends both workspace and intake requests to4351.

Own Playwright CLI0.1.18 / Playwright1.63.0-alpha-2026-08-05 session
`review-intake-unified` used the actual personal login and protected context200.
An ephemeral local file input transferred the authorized review file into this
browser's memory; it was removed before filling the actual personal-token field.
The form detached after login. No filled-form snapshot, saved browser state or
credential literal was used. The browser guard only aborted nonfixture/legacy
destinations; it never fulfilled a mocked response. Actual traffic stayed on the
business API. Source freshness had expired during preparation; the visible
withholding state and real **Refresh source access → Read next step** restored it.

The actual candidate A Brief was changed without saving, then a different task
tab's real source B link was followed. **All11 IUI assertions passed, exit0**:
the destination guard asked before discarding, kept A's exact sentinel behind
the modal across1280×900→390×844, and **Keep editing** retained A and its edit.
Following B again asked again; **Discard my draft** then opened exact source B.
The26 requests during this navigation were all reads; no candidate/task/source
write occurred. No private draft/passages appeared in persistent browser storage.
Three subsequent protected reads independently confirmed candidate A remains
Pending/revision3 with its saved baseline, the unsaved sentinel was never stored,
the published task remains exact, and the tenant still has only two tasks.

[Structured closure/traffic](business-integration-review/fixture-7ed0dd0c/iui-closure-results.json),
[desktop dialog](business-integration-review/fixture-7ed0dd0c/iui-dialog-1280.png),
[mobile retained draft](business-integration-review/fixture-7ed0dd0c/iui-kept-390.png),
[post-browser persistence](business-integration-review/fixture-7ed0dd0c/post-browser-persistence.json).
Screenshots were taken after settling; the desktop dialog and unobscured mobile
draft were visually inspected. Initial CLI harness mistakes (unavailable URL
global, old login label and a stale details ref) are recorded separately; none
is a product finding. Installed CLI help/types and actual labels grounded the
corrections. No dependency was installed or updated.

Design Studio Saving changes/Tabs/Modal methods and its sequential flow/reviewer
checks support this narrow closure. This is not a repeated accessibility matrix,
palette/motion audit or full final Design Studio pass. Private email/Hafidh
intake browser paths and restricted native tenant windows are not certified here.

Own browser PID39457 is closed, with the explicitly discarded local edit gone;
review fixture records remain available. Backend PID66200, frontend PID81173,
all other namespace records, user-inspected browser sessions, old fixtures,
targets/exports and native artifact were left unchanged. Fixture lifecycle stays
with root/tickets. [Source/tool attribution](business-integration-review/fixture-7ed0dd0c/NOTICE),
[immutable source ledger](business-integration-review/fixture-7ed0dd0c/probe-source-ledger.json).

The next assigned authority review concerns the usable task→AI session→managed
deliverable→human review contract, including artifact version/publication access
and per-account connector capability/freshness. Existing original-platform
engineering access is distinct from tenant operation/resource/process authority;
this B handoff does not make the missing tenant AI surface available or approved.

Final report/evidence staged `git diff --check`: exit0. All35 evidence digests
and the three API probe/result source hashes were verified; a private-value
scan of the36 evidence/report files found no credential values. The preserved
paused report and original untracked build/export paths remain unstaged.
