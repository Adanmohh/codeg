# Independent business tenancy review

**Final bounded follow-on verdict: no additional blocking findings at29774b50.**
Ten independent focused tests passed, including the actual indexed-run epoch
probe. Native tenant isolation and B integration remain outside this acceptance.
The completed first-head evidence below stays frozen at f3b408da; the
[separate follow-on review](#follow-on-review-29774b50) records the later result.

Review target: `f3b408dae5c724f354763961d79a17a7ae5c86f8`, PR30, tree
`d3f4cc354ceb7034c0b685b76289c1d3de4513af`, parent contract
`7516461633c163c2ac683930487e33231e630b0b`. Source base is
`f3813e3f1edb521f1d1b20d0b372643acc4123a5`; own report branch
`review/business-tenancy` starts at accepted main `0084fd3ef41983eca6cf52fbdf2dc2b8e9936cd2`.
B is preserved/pushed at `15bb402b`, including four passing recovery tests at
`6513dc15`; its production remains `e9c63237`. Paused visual report is preserved.

## Bounded verdict and scope

**No additional blocking findings in the reviewed f3b408da migration/identity/
settings core.** Sixteen unchanged tests and four reviewer probes passed
independently. This completes the bounded first-checkpoint review, not complete
tenancy acceptance: the unlinked-run epoch correction and the other unfinished
boundaries below remain explicit. Read the full implementation report, contract, product diff,
identity/settings/migration source and relevant unchanged transaction/auth code.
Scope is the pinned-connection parent rebuild, foreign-key restoration and retry,
captured/legacy delegation epochs, original organization mapping, and settings
authorization/CAS/audit. No product source, other worktree or existing fixture was
edited. All runtime databases and credentials are synthetic.

## Independently executed evidence

An own ignored `git archive` of the frozen head is under
`.docs/business-tenancy-review/f3b408da/source`. Every **731/731** tracked blob in
`src-tauri`, `integrations/pi-desk`, NOTICE and LICENSE matches the Git object;
`source-blobs-before.json` records each comparison. No uncommitted owner source
was copied. Only the new isolated `.docs/business-tenancy-review-target` is used.

From that archive's `src-tauri` directory:

```sh
env CARGO_TARGET_DIR=/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-tenancy-review-target cargo test --locked --offline --no-default-features --lib business_identity::tests > /Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-tenancy-review/f3b408da/evidence/identity-unchanged.log 2>&1
```

**Exit0: 16 passed, 0 failed, 1 manual fixture ignored; 0.99s execution,
2m23s compile.** This includes the three new owner tenancy tests and thirteen
existing identity/auth tests. The real protected router remains covered; the
ignored4341 browser fixture was not started. Linker unwind-size and existing
proc-macro-error2 future-compatibility notices remain. This is not Clippy,
desktop/native, browser, provider or full B acceptance.

Four reviewer-only probes also passed, **exit0, 4 passed/0 failed/0 ignored,
0.34s execution/1m01s compile**, using the same command/target with selector
`business_identity::tests::independent_review` and log `independent-probes.log`.

| Probe | Independently observed boundary |
| --- | --- |
| `review_tenancy_retained_task_lineage_and_actual_migration_receipt_retry` | Actual SeaORM migration receipt INSERT fails after the schema COMMIT. Every column in eight populated member/credential/history/task/authority/execution/deliverable tables is retained; all three separately checked-out connections enforce FKs. Retry records exactly one migration receipt, preserves a post-commit settings change, original mapping and immutable task/delegation history. |
| `review_tenancy_cancel_closes_pinned_connection_and_retry_preserves_data` | Hold an independent SQLite writer, observe the migration still pending with its single pool connection occupied, abort before releasing the writer, then confirm the replacement connection lacks the old TEMP marker and has FKs on. No partial tenancy marker; retained task lineage survives and retry succeeds. |
| `review_tenancy_settings_roles_closed_payload_and_audit_atomicity` | Viewer/member/manager writes and scoped-agent reads/writes are denied. A real admin credential remains non-operator and can save. Extra actor/org/URL/CSS fields, invalid names and nonpositive revisions fail. An injected audit INSERT failure rolls back settings/revision; valid save adds one event without renaming the organization. |
| `review_tenancy_settings_real_cas_and_queued_epoch_and_credential_fences` | Two SQLite connections race one settings revision: one save, one conflict, one event. A queued save loses authority after suspend/resume commits; explicit fresh authentication can save. A later queued credential revocation denies the stale save without settings/audit mutation. |

Committed [result and artifact hashes](business-tenancy-review/f3b408da/results.json),
[unchanged log](business-tenancy-review/f3b408da/identity-unchanged.log),
[probe log](business-tenancy-review/f3b408da/independent-probes.log),
[test-only patch](business-tenancy-review/f3b408da/independent-probes.patch),
[readable tests](business-tenancy-review/f3b408da/independent_cases.rs), and
[source correspondence](business-tenancy-review/f3b408da/source-correspondence.json).
The clean baseline matches 731/731 blobs. For the additional run, 730 remain exact;
the only changed original file is `business_identity/tests.rs`, with one test
module registration. The new test file exactly matches the report copy; no
production blob differs. `git apply --check --reverse` against the owned archive
exited0 for the committed patch. No owner source correction was needed.

Probe SHA256 `f406841d2eaa31bb834e47efee7bf67a4ac08526036b173343f29758f247f602`;
patch `6edbcbe5ddbee64e2ca3f59133ab2eb7f141508a52552a597a2f6685598f2817`;
unchanged log `52e677dfb9d8db8c64f01dae2bdc7efff555e83051a78cfe4a52935d4d82bd45`;
probe log `bd75a10e76d785dd9aa03bd9cfeb081cf6770271b9ed554594aa7afa1ea744c8`.
The exact tested reviewer file retains an unused `TransactionTrait` import;
that warning is disclosed rather than changing the tested artifact. Rustfmt and
source/report-only diff checks exited0. The complete staged artifact diff check
exits2 only for the two preserved logs' final blank lines and a blank context
line in the unified patch. Raw evidence hashes are preserved. No broad suite or
passing selector was repeated.

## Grounding and attribution

Full current FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS and implementation scope
read. Code-context used the existing rag-skills Python with `HF_HUB_OFFLINE=1`:
guide exit0; docs exit3 because `data/code/tickets.db` is absent. Cross-project
per-tenant-database advice does not override this accepted shared-database
contract. Installed SeaORM1.1.19, SQLx0.8.6 and SQLite3.46.0 source is the API
authority. SQLx pool `close_on_drop`, SQLite worker cancellation/acknowledgement
handling and SeaORM's SQLite migration/receipt ordering were read directly.
Live hook audit for this worktree/session `01a07c1c-d82f-7022-84db-778a438632f1`
contains PostToolUse21203–21206 and PreToolUse21211–21214. Hooks remain enabled.

Verified NOTICE's six Codeg source blobs against the immutable GitHub tree at
f3813e3f using gh api: identity `mod.rs`22a99cae, `store.rs`bb11e604,
`types.rs`145f1d0e, `tests/transactions.rs`e9cac228, migration000009 ad2db145,
and `src/lib/theme-presets.ts`9181e38c. Full original Apache-2.0 LICENSE read;
blob `261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64`. All earlier notices remain.
SQLite official `version-3.46.0` resolves through gh api to
`bebe2d8be8acfd02592c4972f4ba32c3b4e4a33f`; its public-domain LICENSE was read.
Its LICENSE blob `f68a6c175f0b72086c313a9dbd9f2ec8d87525dd` matches NOTICE.
Dependency primitives are API references, not copied implementations. No
Edublend, GPL or AGPL implementation is introduced by this diff or review.
Reviewer fixture glue additionally follows the same pinned Codeg transaction
tests and `db/test_helpers.rs`; attribution is included in the test file. The
production NOTICE/LICENSE and dependency manifests/locks are unchanged by review.

## Explicit pending boundaries

- Actual platform provisioning and same-database two-tenant transport tests are
  pending the owner's compiling implementation. This head exposes no new
  settings/platform route and does not implement restricted native sessions.
- B's persisted setup/attempt/source/preview epoch fields and000011→000012
  integrated retention remain B-owner work. No stale Principal refresh/default
  epoch is permitted. B is not included in the frozen A-only migration prefix.
- Owner has identified the entrusted-but-unlinked A run epoch seam and is
  preparing a separate immutable correction; it is not claimed fixed here.
- Root's newly supplied native prerequisite requires cross-window large/raw
  channel denial, ordered positive host delivery and no fallback queue on send
  error. A custom app-command ACL or channel interceptor alone is not native
  isolation acceptance; platform-specific responder conditions must be retained.
  Caller-window controls must exclude arbitrary target labels. These are future
  native gates, not an executed exploit or a defect claim against accepted A.
- Cancellation was measured while waiting behind an independent writer, not at
  every SQLite/driver await point. Receipt failure was deliberately injected
  after the real schema commit; there was no forced OS process/power loss. The
  populated retention fixture is A-only and launches no actual execution run.
- No old targets, fixtures, exports, A bundle, real stores or credentials changed;
  no live provider/model/engine/Pi process, server launch or new dependency.

Early report checkpoint `8d935c88d061ca5179fb74ad7f9f2a337aae7a36` was pushed
and relayed before these probes completed. Both test processes have exited0.
The final reviewed implementation remains exactly f3b408da; later owner product
commits require their own source correlation and focused review.

## Follow-on review: 29774b50

Frozen target `29774b50aafc29658a2f48fab1f44d366ed2c8a0`, tree
`af9265e34f37022cd91142a8ef48ef5801846f9b`, direct parent f3b408da,
verified through gh api. Own fresh archive is
`.docs/business-tenancy-review/29774b50/source`; **739/739** tracked Rust/Pi/
NOTICE/LICENSE blobs match before testing. No uncommitted owner source copied.
The same owned isolated review target is reused, not B's target or a fixture.

Read the complete follow-on report, contract, diff and changed core/HTTP/native
source and tests. **No additional blocking findings in this bounded review.**
The source authority lookup now requires the immutable sidecar epoch to equal
the active organization's epoch before initial link and later agent checks.
Original human DelegationGrant storage remains separate. Platform middleware
requires the real original operator marker; tenant owner roles cannot construct
that context. Create/reissue/status take the SQLite writer before receipt replay,
lineage/current-revision validation and atomic changes. Replays omit tokens.
Last-owner demotion/revocation checks share the member mutation's writer.

Independent unchanged selectors passed, all exit0: platform **5/5**
(0.42s,2m14s compile), exact unlinked-source epoch test **1/1** (0.18s), and
changed populated-migration test **1/1** (0.15s). Commands use the preceding
locked/offline/no-default-features/lib pattern and own target. Logs are in this
head's committed evidence directory. The platform suite exercises actual protected router
creation of two tenants in one DB, cross-tenant denial, owner/member versus
platform separation, settings race, lost-response replay/reissue and concurrent
last-owner demotion. Earlier unchanged f3 settings/cancellation tests were not
repeated without a changed concern.

Three reviewer-only probes also passed **exit0,3 passed/0 failed/0 ignored,
0.19s execution/47.77s compile**. Same command/target, selector `review_297_`;
only two test module registrations and two new reviewer files in the archive.

| Reviewer probe | Independently observed result |
| --- | --- |
| `review_297_indexed_run_rejects_old_entrustment_and_preserves_exact_human_grant` | Use the real backend CAS to mint a running generation and seed its private live index/connection fixture. Entrust through TaskEngine, suspend/resume through platform core, then authenticate the original human credential afresh: initial link is Forbidden with no task revision/history/authority/sidecar/binding/deliverable mutation. A newly CAS-minted generation can be explicitly entrusted, linked by that human and contribute through the token listener. Its stored grant retains the exact original human credential, member and epoch3, not operator authority. Another lifecycle cycle denies contribution and leaves the grant bytes unchanged even though that credential can authenticate again. |
| `review_297_sidecar_retains_authority_and_grant_across_receipt_retry_and_owner_fks` | Fail the real SeaORM receipt INSERT after schema COMMIT. Every column of the existing authority and legacy grant is retained; backfill captures epoch1. Insert a new synthetic authority at epoch3 before retry: retry preserves1/3 and writes one migration receipt. Sidecar orphan INSERT/update/delete fail. Actual provisioned tenant credentials cannot be substituted across tenants, nor from another existing member in the same tenant. Original mapping and FK checks remain valid. |
| `review_297_protected_recovery_receipt_rollback_and_lifecycle_replay` | Actual protected HTTP create/reissue/status/list handlers. Inject reissue receipt failure:500/no-store, identical credential rows, same recorded lineage, old bearer still valid, no receipt. Successful retry rotates once; replay omits secret; stale ancestor conflicts. Suspension rejects member authentication and recovery; resume permits fresh member authentication. Historical status replay returns its original metadata without changing the current epoch or receipt count. List excludes synthetic bearer/hash fields; platform receipts remain immutable and attributed to legacy_operator. |

The previously pending unlinked-run epoch seam is resolved at this head:
[authority lookup](https://github.com/Adanmohh/codeg/blob/29774b50aafc29658a2f48fab1f44d366ed2c8a0/src-tauri/src/business_tasks/agent.rs#L29),
[immutable sidecar](https://github.com/Adanmohh/codeg/blob/29774b50aafc29658a2f48fab1f44d366ed2c8a0/src-tauri/src/db/migration/m20260908_000012_business_tenancy.rs#L157).
Protected [reissue](https://github.com/Adanmohh/codeg/blob/29774b50aafc29658a2f48fab1f44d366ed2c8a0/src-tauri/src/business_identity/platform.rs#L279),
[last-owner guard](https://github.com/Adanmohh/codeg/blob/29774b50aafc29658a2f48fab1f44d366ed2c8a0/src-tauri/src/business_identity/store.rs#L552)
and the migration's composite credential FK were read against the accepted contract.

Committed [exact commands/results/hashes](business-tenancy-review/29774b50/results.json),
[probe log](business-tenancy-review/29774b50/independent-probes.log),
[test-only patch](business-tenancy-review/29774b50/independent-probes.patch),
[platform/migration probes](business-tenancy-review/29774b50/independent_platform_review.rs),
[bridge probe](business-tenancy-review/29774b50/independent_tenancy.rs), and
[source correspondence](business-tenancy-review/29774b50/source-correspondence.json).
Baseline739/739 match; after probes737/739 original blobs remain exact. The other
two are test registrations only; both added files match their report copies.
All production blobs, NOTICE/LICENSE and lockfiles are unchanged by review.
Zero-context patch reverse check (`git apply --check --reverse --unidiff-zero`)
and Rustfmt check exited0. The three original logs and all command exits are in
results.json; both linker unwind-size and existing proc-macro-error2 notices
remain. No Clippy or full-suite pass is claimed from this review.
Report/source diff checks exited0. Full staged artifact diff check exits2 only
for one final blank line in each of the four preserved raw logs; their bytes and
hashes are retained. All nine artifact hashes and the seven prior f3 files were
checked; the paused visual report's hash also remains unchanged.

Result SHA256 `f6b0f8228604cd3fefcc5b6c0a062cc366b13ca3536487a6cf6d587d5c13f970`;
probe log `13280d177d33b22c59be27cf0817cb6f2671a2814d19e0f2e6661e220824feeb`;
patch `6c368c3403e4fbcaa56a8cc2e3d868f1f55d7d0a5d409575f4954268ca202f1c`.
All earlier f3 artifact hashes remain unchanged.

Native preparation is explicitly **unavailable for production tenant windows**:
`business_window_context` reports `tenantWindowAvailable:false`; no production
tenant creator is registered. Read the installed Tauri2.10.2 ACL dispatch,
public channel producer, private callback producer and fetch source. The ACL
fetch exemption/global queue and non-macOS/non-iOS private producer conditions
match the reported gap. The prepared MockRuntime tests were read, not independently
executed by this reviewer at297. They do not prove cross-window channel isolation,
physical webview behavior or packaged acceptance. No native build was run. B's
000011 tables are absent from this A-only archive; combined migration retention
and B persisted authorization epochs still require integration. The actual-run
probe uses the existing in-process test engine/index/duplex listener, with no
external execution process. The migration authority fixture is synthetic and
does not claim liveness. No power-loss fault or real provider was exercised.

All seven additional Codeg NOTICE source blobs match the official immutable
f3813e3f GitHub tree, including task authority, command/auth glue and000010 test.
Tauri official06374a9 channel blob0b3eb677 and Apache LICENSE blobf433b1a5 match;
full licence read. No dependency implementation port or lockfile change by review.
Live own-session hook audit records PostToolUse21535–21537 and
PreToolUse21542–21544 in this worktree. No fixture, actual store, provider,
model, engine process, root artifact or product source was changed.

Early follow-on report pushed at `62080dbdeef5637d34345820aeb3afd1b573fe74`.
Owner handoff `d3a176d2287c23b649cd1d266cb1a9187bbcc0bb` was verified through
gh api compare: one later commit, only contract ancestry wording and reports/
validation artifacts changed; production remains29774b50. The corrected owner
branch point is `a4cda9d1ab0753b9941e5810f083e20422990395`; reused product source
is still f3813e3f. Owner desktop/
server/Clippy results remain separately attributed, not repeated or counted as
these ten independent passes. Every reviewer test process has exited0. B work,
paused report, old fixtures/targets and root's accepted bundle remain preserved.
