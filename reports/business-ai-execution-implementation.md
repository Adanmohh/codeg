# Business AI execution — implementation checkpoint

## Scope and current state

Owner: tickets, sole writer on `feat/business-ai-execution` in
`/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`.
Base: accepted main `9e61fe67261219921e0a6a860c45f6e08ea99c21`.
Contract: `3f164c2a989cd08a523e51f1e47559c15a48c0ef`, SHA256
`e2cf8304fb5eababce79961da57061e244f2567eae11b75b88dbfcbccc6a114d`.
Independent contract verdict618a3d96 and merged PR31 are in this base.

This is an early implementation checkpoint, not a runtime acceptance claim.
The owner now authorizes complete protected E1 HTTP/native-core integration,
existing-engine sessions, scoped events and managed-version publication. E1 is
the original operator slice; tenant execution/native tenant windows and account
snapshots remain unavailable. No provider login, live inference, new dependency
or runtime is authorized. Only synthetic ACP/PTY children in isolated test
directories may run during validation. Any persistent fixture requires a prior
published port/guard/lifetime recipe.

## Implementation sequence and fixed boundaries

1. Closed DTOs and additive migration `m20260909_000014_business_execution`:
   sessions, captured authority and generations, durable operation receipts,
   managed assets/immutable versions, selected public references.
2. Task-owned transaction helpers retain current authorization, human review,
   task CAS and immutable activity. File publication explicitly requires a human;
   existing agent-capable text submission is unchanged.
3. Backend profile discovery and existing chat/ACP/PTY integration with durable
   launch/prompt/stop/import uncertainty; no inferred credentials or fabricated
   model readiness. Every private operation checks original operator authority.
4. Authenticated bounded NDJSON snapshot/replay/history and protected content
   delivery, revalidation and safe public metadata. No token-bearing URLs.

Source constraint found before editing: migration000010 requires nonempty
deliverable text. Contract file-only submission permits an empty body with selected
versions. The forward000014 migration will preserve existing rows and immutable
triggers while allowing that case; existing text input validation remains intact.
The new task-owned writer must require and atomically publish selected references.
Applied000010/000012 will not be edited. Base is A+000012; incoming B000011/000013
must retain their order before000014, including already-recorded migration paths.

## Docs-first and intended source mapping

Code-context offline guide completed exit0 using the existing rag-skills Python,
`HF_HUB_OFFLINE=1`. Applied “Atomic per-task staging”; unrelated per-tenant database
and delegation guidance does not override this project's accepted shared database
and no-extra-worker instructions. Installed docs retrieval exits3 because
`data/code/tickets.db` is absent; no corpus coverage is claimed or install performed.
Direct local pinned source is used: React19.2.4, SeaORM1.1.19, SQLx0.8.6,
Tokio1.49.0, portable-pty0.8.1, codeg0.30.4. React package and Cargo manifest were
read in separate commands. Earlier own-session hook records include PreToolUse26778–82
and PostToolUse26783–84 in `ops-docs-first-audit.jsonl`, exact tickets cwd/session
`01a07c1c-d82f-7022-84db-778a438632f1`; no hook bypass.

All remote documentation uses immutable `gh api` reads. Accepted contract/report
records the bounded official MCP specification reference and existing ACP/Pi
source pins; those are reference evidence, not a new protocol or package upgrade.
New product code is integration glue over Apache-licensed Codeg. Intended source
files at base9e61 (exact Git blobs):

| Source | Blob / purpose |
| --- | --- |
| `db/migration/m20260908_000012_business_tenancy.rs` | `a305a682bbf55e750a5d5156ac0aa4695aac08c1`; pinned SQLite connection, cancellation-safe FK restoration, atomic marker/receipt retry |
| `db/migration/m20260908_000010_business_tasks.rs` | `3cb107d5bc0ab375e1b82fc64c7fefc82396b318`; retained task/deliverable schema |
| `business_tasks/store.rs` | `d48fdf1e85feb4410dbff51e8a671d6c9d525691`; task-owned authorization/CAS/activity |
| `business_identity/mod.rs` | `f7205760715d2516b11b6e2cb79fd133218672bb`; existing Principal, captured epoch, original authority and opaque delegation lineage |

Exact per-port NOTICE additions will accompany implementation; all inherited
sections/LICENSE remain. No AGPL/GPL code is used.

## Planned acceptance and preservation

Planned, not yet run: retained populated migration/FK/cancellation/receipt retry;
two-tenant and operator-versus-owner denial; task assignment/domain/cancel/reopen
fencing; same-operation replay versus changed payload; concurrent task CAS;
launch/prompt/stop uncertain outcomes with synthetic children; immutable import
path/symlink/byte-change rejection; safe selected-version human review discovery;
ordered bounded stream replay/revalidation; default/server/companion checks,
focused tests, typecheck and Clippy. No passing evidence is invented here.

Builds will reuse only `.docs/business-intake-tenancy-target`; latest space
observation7.6Gi free. No new large target/copy. The copied stable B fixture binary,
ports4351/4352, root native bundle, other exports and every old fixture remain
unchanged. User-inspected `intake-worker4354` and its records are frozen.
Paused `reports/visual-correspondence.md` remains untracked and untouched. Preserved
branches include business-intake462604b00 and docs/business-ai-executioncab3b27e.

At the initial79d19547 checkpoint, implementation was not yet written.
Commands so far: separate local docs/status/source reads, offline retrieval and
`git fetch origin` / `git merge --ff-only origin/main` (exit0), `gh pr create --help`
(exit0). Draft PR: https://github.com/Adanmohh/codeg/pull/32 (stays draft).

## First compiling product checkpoint

Immutable product checkpoint: `6195d9daf` (full SHA in validation digests).
The initial exact-file staging excluded ignored `.log` files; the immediately
following evidence commit force-adds only those two owned logs and their digests.

Implemented internal closed DTOs/validation and migration000014, registered only
the module/migration. No HTTP/native handlers or runner actions exist at this
checkpoint. Profiles remain backend discoveries, with no wire credential/command
input. Captured authority, task-scope epoch, profile revision, session generation,
receipts, output claims, immutable versions and selected publication references
are separate retained records. Task assignment/domain/cancel/reopen changes fence
old session generations, including change-away-and-back. Runtime consumption of
these fences remains to implement/test; schema alone is not authority acceptance.

Migration ports000012's one pinned SQLx connection, close-on-cancel FK restoration
and atomic completion marker. Deliverable rebuild keeps every column/ID/FK and
immutable trigger, changing only the lower text-length bound. A real migration
receipt deletion/retry was tested after populated human text/history/current
deliverable retention. Mid-DDL table collision rolls back the rebuild/receipt and
restores FK enforcement. Blank legacy text submission remains rejected after14.
Actual execution-linked retention, cancellation while awaiting SQLite, B11/13
combined ordering, file-only writer/CAS and runtime races are still pending.

The rebrand wire questions are closed in an additive contract section and Rust
types: assets/list is a stable page; private assets/versions adds authorized older
version pagination; assets/get has explicit current capability/preview fields;
successful prompt has durable messageId/inputHash. Public task readers still get
only selected safe references/metadata. No route availability is implied by DTOs.

Actual commands, own existing target only:

| Command | Result |
| --- | --- |
| `cargo check --locked --offline --manifest-path src-tauri/Cargo.toml --no-default-features --bin codeg-server` | exit0;36.66s; initial schema/DTO source |
| `cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --no-default-features --lib business_execution::tests` | exit0;3 passed,0 failed/ignored;0.13s runtime,1m56s compile; final checkpoint DTO source |
| `rustfmt --edition 2021` on the five new Rust files; `git diff --check` | exit0 |

`CARGO_TARGET_DIR` was the existing absolute
`.docs/business-intake-tenancy-target` described above. Committed logs are under
`reports/business-ai-execution-validation/checkpoint/`. Existing linker unwind and
proc-macro-error2 future-compatibility notices are preserved in logs. This is not
final default/server/companion/Clippy/typecheck or complete E1 acceptance.

## Integrated B checkpoint and historical global hook block

Accepted B mainc8453a48d was merged locally as b73940560. NOTICE retains the
complete accepted-main prefix plus the E1 addition. Applied migrations10/11/12/13
and LICENSE were byte-compared against c8453a48d and are exact. Registration order
is11,12,13,14. Existing B task-owned transaction helpers arrived intact.

The next owned test run completed successfully before any further build:
four business_execution::tests passed,0 failed/ignored,0.30s runtime,2m02s compile,
exit0. It uses the actual integrated B migrations. The additional retained-schema
case preserves every column of the task, agent-authored deliverable, original
execution link/opaque grant/authority epoch, activity and member rows through14
and real receipt loss/retry. This is schema retention, not a live engine ownership
proof. The DDL rollback case now also preserves a populated deliverable.
Actual log remains at the owned absolute path
/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-ai-execution-logs/tests-integrated-schema.log.
The four unchanged B epoch_migration_cases and final integrated check have NOT run.

While that previously authorized test process was compiling, the global launcher
changed to docs-first-v2. Its current source and full documentation were read:
[/Users/mohamedadan/.codex/hooks/docs-first.md](/Users/mohamedadan/.codex/hooks/docs-first.md)
states: “Other languages, unknown configuration, SQL, notebooks, MDX, arbitrary
JSON and ambiguous syntax stop.” It also states: “Arbitrary interpreters, scripts,
builds, tests, formatters, shell patches and shell write helpers are deliberately
unsupported even after a documentation read.” Rust/Cargo and git commit/push are
therefore unavailable through this gate. Package resolution has no Cargo/Rust
source implementation; installed Rust reads cannot satisfy that missing path.

Actual denials: the coordination message returned shell_unresolved, and even
herdr agent discovery / rg with a directory option returned
shell_unproven_use_patch_or_reader. Safe plain cat and herdr agent list still work.
The engine accepts only numeric workspace IDs in its Herdr target grammar;
the actual existing Ops IDs are wR:p1 through wR:p4 and have no agent aliases.
Thus the bounded reviewer build-window message was NOT delivered. No alternate
execution route, hook edit/disable, hidden acknowledgement or process reuse was
attempted. Existing permitted test polling observed completion only.

At that pause, the hook owner needed to add reviewed Rust/SQL, build/test/Git and actual Herdr-ID
support before implementation could continue. This was an environment gate, not an
E1 product failure or request to weaken authority. Latest pushed checkpoint is
fdfe34294 over product6195d9daf; local mergeb73940560 and the later test/report
edits remain preserved but cannot be pushed under the new gate. No source/fixture/
provider/native action follows this block. Prior stable fixtures and the owner's
inspected browser/records remain unchanged. Full E1 remains incomplete.

## Resumed combined checkpoint

The owner installed Claude-style admission f5554c92. This session reread the full
`~/.codex/AGENTS.md` and current `docs-first.md` preface: ordinary Git, gh api,
build/test and authorized Herdr commands are admitted; Rust/SQL still require
manual source grounding, and supported patches retain signed documentation checks.
Actual `git fetch origin` and immutable `gh api` read of accepted B migration13 at
`c8453a48d441f40eb47f9c9af856235b4f9986e5` both exit0. That source confirms nullable
historical epochs, per-column idempotence and writer/FK checks. No denied edit was
rerouted. The historical blockage above is resolved, not deleted from evidence.

This checkpoint publishes local merge `b73940560a69748f6774f41eb1a8de44e7c4c11f`
plus the actual four-test changes. Committed integrated log:
`reports/business-ai-execution-validation/integrated-schema/tests.log`, SHA256
`adab474c63542e411377f1681c952ac2dc2a92819111144ffb93126700a096dd`.
Exact tested `business_execution/tests.rs` SHA256:
`3885fa6dfcaac60be118bbccf1b449f7f0ac0274c56548e68b7058068935dd14`.
All other product source matches that merge. Four passes remain schema/closed-wire
evidence; the four accepted B epoch cases and integrated server check are next.

Current free space is 5.7GiB, a changed observation rather than a reservation.
The reviewer build-window message now succeeds; only the existing tickets target
will be used, with no concurrent reviewer compilation requested. Herdr currently
reports `gpt-6-astra`; this does not establish historical reasoning effort or
retroactively certify earlier turns. The owner's restored Astra/max selection is
retained. No fixture, user browser, paused visual report, target or native bundle
was removed, restarted or replaced during recovery.

At exact integrated source `177d0f3e2b426d65de9573ba534ac8c11f9250f7`:

| Additional command, same existing target and locked/offline flags | Actual result |
| --- | --- |
| `cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib business_intake::tests::epoch_migration_cases` | exit0; four passed,0 failed/ignored;0.48s runtime,0.75s cached compile |
| `cargo check --manifest-path src-tauri/Cargo.toml --no-default-features --bin codeg-server` | exit0;31.10s |

The B tests use the accepted real migrations and cover both installation orders,
retained populated rows, nullable historical epochs, rollback and receipt retry
with14 registered. Logs `integrated-schema/b-epochs.log` and `check.log` have
SHA256 `0d3b0edc8b1e7a856213a7c7d9a4db3763bd7bff6860d102603093f709c79df8` and
`9dc1e031c298bdf9f8a734e747bac68a63fbf6a63708c412df629d658f8a8698` respectively.
Both processes completed before releasing the reviewer build window. No final
runtime/asset/session gate is inferred from these migration checks.

The legacy audit file was read in this session without command payloads: session
`01a07c1c-d82f-7022-84db-778a438632f1`, tickets cwd, PreToolUse/PostToolUse exit0
records at Unix timestamps1788951206–1788951207. These are historical records;
reading them now does not make them fresh launcher evidence. The owner clarified
that the current launcher does not write a replacement generic Pre/Post journal.
No fresh generic hook-event metadata or universal hook coverage is claimed.
Actual current Git/gh results and normally admitted edits establish the usable
workflow only. The owner's separate signed-reader deny/read/retry verification
is owner evidence, not a test independently rerun here.

## Admission and file prerequisites — 3773a027a

Exact product `3773a027a8d580bd9bac1808efdb718ae6f9e135` adds private SQL records,
current original-operator/task/profile checks, durable exact-input receipts and
one-time backend launch admissions. A replay returns no new admission. Original
operator authority is stored through the existing opaque DelegationGrant, with
captured tenant/task/profile generations; an ordinary owner credential is denied.
Late launch binding is rejected after profile/task revocation. Sole unaccepted14
adds member/tenant lifecycle triggers; applied10–13 remain untouched. No Principal
constructor, engineering work_task or replacement engine was added.

The file prerequisite ports Codeg upload_jail's Unix descriptor/no-follow pattern:
bounded depth4/512-entry discovery, hidden/profile/config exclusions, regular-file
and single-link checks,50MiB ceiling, identity/size/time checks, streamed hash/copy,
exclusive0600 staging outside scratch and completed0400 retained objects. Existing
completed objects are hash-checked for same-operation recovery, including missing
scratch. Corrupt/partial objects are never silently overwritten. Public assets,
publication and recovery dispatch still require the later DB/runtime consumer;
these filesystem methods are not exposed by a route. The trusted data-directory
ancestor is not an OS sandbox. Non-Unix managed file support fails closed pending
equivalent hardening; the profile consumer must expose that readiness limit.

Manual grounding used SeaORM1.1.19 Statement/transaction commit/drop and existing
Codeg task/intake/identity source; immutable gh api task writer read atc8453a48d.
File methods used the installed libc0.2.180, sha2 0.10.9/digest0.10.7 plus official
Rust1.98.0 `library/std/src/{fs,os/unix/fs}.rs` at
`88d9e12ae178fab0fb5cc050a94da85685d449ea` via gh api: descriptor metadata, link/time
fields, sync and directory-name enumeration behavior. Relevant Codeg upload_jail
source was also re-read locally and through immutable gh api. Exact source-to-port
files/blobs are appended in NOTICE; no lockfile, dependency or licence changed.
One native patch returned `patch_context_unresolved`; actual source and relevant
pinned APIs were reread, then the same native patch tool accepted corrected exact
context. There was no shell/interpreter edit fallback or hook modification.

Actual gates, unchanged existing target, locked/offline flags:

| Command | Result at3773a027a |
| --- | --- |
| `cargo test --manifest-path src-tauri/Cargo.toml --no-default-features --lib business_execution::` | exit0;12 passed,0 failed/ignored;0.80s runtime,1m57s compile |
| `cargo check --manifest-path src-tauri/Cargo.toml --no-default-features --bin codeg-server` | exit0;11.50s |
| `rustfmt --edition 2021 src-tauri/src/business_execution/mod.rs`; `git diff --check` | exit0 |

The12 are four earlier migration/wire cases, four new admission/authority cases
and four real temporary-file cases. Authority cases cover a human-only task with
zero work_task rows, receipt replay/changed input, same-org owner credential and
real separately provisioned tenant denial, profile change-away-and-back, task
cancel/reopen and fresh operator authentication after tenant suspend/resume.
They bind a synthetic linkage record without starting a subprocess: not live
engine ownership, scoped event delivery or protected HTTP/native acceptance.
Filesystem cases cover retained bytes/scratch deletion, symlink/hardlink/traversal/
profile exclusion, changed source, corrupt retained object, oversize and replaced
storage parent. Mid-copy mutation and integrated import/publication/receipt races
are still pending. Warning logs are retained:58 unused-consumer warnings in server
mode,9 warnings in test mode including the inherited linker message. Clippy is
not passing or claimed at this prerequisite checkpoint.

Committed `admission-files/tests.log` SHA256
`1b2acae5bc2197c6c664b140436e2f75d022744964124d99371242699f031043`;
`admission-files/check.log` SHA256
`352204e540f5962f6ebab8a7833bcae1c1f89e115b9e8df5d6a6bec6bec76c55`.
Latest free-space observation2.9GiB; both own Cargo processes finished. Root/reviewer
are notified before another build. No old target/artifact/fixture/browser changed.
Next: managed DB import and task-owned selected-reference publication, existing
ACP/PTY lifecycle and narrowed companion scope, protected transport/events/content,
then full runtime/CAS/receipt/byte tests and desktop/server/companion/Clippy gates.

## Three internal review corrections — source checkpoint9949c8a9a

Accepted main4043efa0e integrated and pushed as
`54daac22eba8aaab1e7b2e6da9e39a03e80e6be1`: complete main NOTICE retained with
the existing E1 attribution appended; report conflict retained this newer report.
Backend/locks/LICENSE were unchanged by that merge. Three bounded fixes follow:

- R2 `836fbdcf451b5d9181f8b6d13e14ba962f57f151`: admission fields are private;
  completion validates the receipt's kind/target task/session/generation/resource
  before any linkage/status write and checks the same binding again at receipt
  completion. Two genuine pending admissions, substituted operation ID, stale
  generation and profile revocation tests compare all retained session/generation/
  receipt values on rejection and preserve correct A/B completion controls.
- R1 `5a231349edafa0a57d3fad8792ea1ac7b95f0787`: current stored authority/profile/
  task eligibility is applied before LIMIT. Cursors are immutable positions under
  the current org/member/task, not session access grants. Revocation between pages
  cannot strand a returned cursor; no revoked row is emitted as an item/cursor.
  Three tests cover leading/interleaved stale rows, stable complete traversal,
  revocation after page one and real member/tenant plus wrong-task cursor controls.
- R3 `9949c8a9ad99ffb70232c05bf1c2ad243dcaf194`: same-object recovery verifies the
  descriptor/hash, then repeats file metadata and parent-directory sync before
  returning Retained. Test-only injection exercises each post-seal failure,
  repeated failed retries and successful ordered real sync with the original
  inode/bytes retained after scratch removal. Production has no injectable sync
  callback. Existing corruption/partial-object/no-overwrite boundaries remain.

This checkpoint is committed source, not yet a test or runtime acceptance claim.
Changed files are the execution module's files/file_tests, receipts, session_store,
session_tests, mod, new pagination_tests and session_store/completion_tests. No
schema, public transport, dependency, lockfile, identity constructor or provider
runtime changed. Exact3773 source-only findings remain reviewer evidence at
`dc0e9cb06ded25122c373029c3bc9f9bbe40b42f` until independent correction review.

Grounding: existing3773 session/scope/receipt and temporary-file tests, accepted
Codeg task/identity/SQLite fixture sources already attributed in NOTICE, installed
SeaORM1.1.19 Statement/QueryResult and Rust1.98.0 File::metadata/try_clone/sync_all.
Installed std HTML source was located under the toolchain's share/doc/rust/html/
src/std/fs.rs.html; prior unavailable Rust source coverage did not include this
location. Official Rust source was also read via gh api at
88d9e12ae178fab0fb5cc050a94da85685d449ea. No third-party source or licence added.

Owner removed enforcement hooks and restarted this same session. Updated global
AGENTS was read: do not install/restore hooks without an explicit request. Normal
git status and full immutable gh api lookup of54daac22 succeeded, as did native
patching. The initial abbreviated Git-commit lookup returned404; the full40-char
lookup succeeded. No hook maintenance or fresh legacy Pre/Post claim is made.
Earlier durability patch context denial applied nothing; exact source was reread
and the normal patch tool accepted corrected context before the removal override.

Storage interruption: writes/builds paused when reviewer reported117MiB. Read-only
inventory found old incremental caches, but this worker removed nothing. Root's
separately authorized cache recovery and same-session restart preserved fixture
66200, previews and all worker targets. Current preflight reports17,659,348KiB
available (about16.84GiB), an observation rather than reserved capacity.
Next authorized bounded gates: execution_pagination_, execution_admission_,
execution_authority_, execution_assets_ selectors and server check, all locked/
offline, existing .docs/business-intake-tenancy-target, -j2. Start only with at
least10GiB free; stop/report near5GiB. No new target/export/native build or cleanup.

## Executed correction gates — product9949c8a9a

The bounded gates ran after the source/report checkpoint
`e56affc361ba7901e624dcc7e4d8e5a2776aee5b`, with all eight changed product files
byte-identical to `9949c8a9ad99ffb70232c05bf1c2ad243dcaf194`. Every command used
the existing `.docs/business-intake-tenancy-target`, locked/offline and `-j2`.
No source changed during the run. Logs and exact command/source hashes are in
`reports/business-ai-execution-validation/review-corrections/digests.json`.

| Selector / check | Actual result | Test runtime / Cargo finish |
| --- | --- | --- |
| `execution_pagination_` | 3 passed, exit0 | 0.27s / 1m44s |
| `execution_admission_` | 3 passed, exit0 | 0.30s / 0.51s |
| `execution_authority_` | 3 passed, exit0 | 0.25s / 0.22s |
| `execution_assets_` | 5 passed, exit0 | 0.09s / 0.21s |
| server `cargo check --no-default-features --bin codeg-server` | exit0 | 27.46s |

These are 14 owner-run focused tests, including the six new cases. The frozen177
schema suites were not repeated. Independent9949 source review and any later
independent execution remain separate evidence. Test build7/server check62
warnings are retained: unused consumers are still being implemented, plus the
inherited test linker and proc-macro-error2 future-compatibility messages. This is
not a Clippy or complete E1 runtime acceptance claim.

Initial disk observation17,653,552KiB; final16,600,328KiB (about15.83GiB).
All own Cargo commands finished and the window was explicitly released to root
and approvals. Farha now owns the production build window: no further Ops Cargo
or large allocation until coordinated release. No cleanup, target copy, fixture,
browser, provider or engine action occurred. Original paused visual report remains
untracked and unchanged. PR32 is still draft, verified using the explicit fork
REST endpoint; an earlier implicit-repository `gh pr view` lookup failed and made
no change.

Next source work remains managed asset records/import and task-owned selected
reference publication, followed by existing-engine lifecycle and protected
transports. The runner must consume an admission once; current
`complete_launch(&Admission)` only proves internal DB/receipt binding, not that
an actual process launch is owned or one-time. No remaining runtime prerequisite
is waived by these correction tests.

## Managed publication source follow-on — gates pending

While approvals owns the separate9949 correction build, source work adds the
accepted task-owned deliverable references and publication helper. Every task
Detail now reads only its exact selected PublishedAssetRef list; text-only history
has an empty list. Private E1 submit requires the original operator and an actual
human, validates retained bytes outside the writer, then repeats current scope /
selection and commits task CAS, immutable references/activity and exact receipt
together. Previously published versions use current task Read; private versions
still require current producer-session authority. The task public projection
contains no private session/turn/profile/object/latest-version fields.

Code is in business_execution/publication.rs and
business_tasks/store/managed.rs, with minimal parent/type registration. Six new
execution_publication_ cases use actual temporary retained bytes, real SQLite /
identity/task writers, injected SQL failures and explicit synthetic version rows.
They are not an import or process-launch test. Two pre14 migration cases now seed
historical text rows directly and compare retained columns, because today's Detail
reader requires the real publication table. No migration DDL or applied history
changed; no production table-existence fallback was added.

Actual source checks so far: rustfmt and git diff --check exit0. New code/tests
have NOT been compiled or executed while the reviewer owns capacity. Planned
next bounded gates are execution_publication_, the two adapted migration cases,
relevant existing task tests and server check, after explicit capacity release.
Correction product9949/owner14-pass and independent42d5978b source evidence remain
separate from this unvalidated follow-on. No new route/native registration, client,
provider, fixture, browser, target or large allocation occurred.

Grounding: code-context retrieval used the existing rag-skills Python offline.
Applied Atomic per-task staging and installed-version guidance; the current
tickets dependency corpus remains missing (docs lookup exit3), without ingest or
installation. Read accepted Codeg store/policy/types and migration14 SQL locally;
read the exact c8453a48d441f40eb47f9c9af856235b4f9986e5 task submit/CAS helper via
gh api. Installed SeaORM1.1.19 Statement/QueryResult/transaction source and
Tokio1.49.0 spawn_blocking/Semaphore source ground the writer and bounded read.
Blocking jobs retain their semaphore permit until completion; cancelling the
request cannot make a detached verifier commit a receipt. Mid-I/O cancellation
and actual transport/engine consumption remain later integration tests.

The publication source checkpoint is
`d0d56a36399d81be23787891177752022c91edf6`. A source-only follow-on adds the
task-scoped content core: bounded retained-file verification, format-specific
preview refusal and a second current reader/reference check after I/O. It returns
bytes/metadata inside Rust, without HTTP/native framing or a new registered route.
Two additional cases cover scratch-independent reads with reader revocation after
I/O, and abort after verified bytes before publication. The prior late-authority
case now crosses the actual verification path before task cancel/reopen.
The execution_publication_ selector now has eight planned cases, not executed.
Rustfmt/diff checks pass; all new Rust compile/runtime gates remain held.

Independent reviewer supplied closure
`de3a6b8fdc2602ef743ec2e1da72f488707980af`: four9949 selectors,14 passes,
all exits0 and802/802 before/after correspondence, distinct from owner evidence.
Root imported that bounded R1–R3 closure. It does not certify this publication
follow-on, import, runner or transport. Reviewer released its window; root
reassigned it to Farha. No own Cargo/test/check started after the prior9949
release, including the read-only preflight before that reassignment message.
Wait for explicit root allocation and fresh≥10GiB before these queued gates.

The content/abort follow-on is committed at
`3841e659bec3d0161d73eaca16058b170fe751be`. Reviewer publication-source verdict
`d1fba7e4a5367b05bd93c7e58d3e3d8f78b98c89` covers frozen d0d56a only: no additional
blocking source finding, no compilation or execution. It does not certify the
later content/abort cases. Farha's browser reservation remains active; no own
Cargo/test/check or large allocation has started.

A further source-only file helper gives import reconciliation an explicit
recovery-only operation: validate and re-sync the reserved sealed object, never
fall through to copying the mutable workspace again. The existing9949 retained
descriptor/hash/mode/sync checks are shared unchanged. A new planned
`execution_assets_reconciliation_never_recopies_missing_or_partial_object` case
covers missing and non-sealed objects, changed scratch, same-inode reuse and
deleted-object refusal without recreation. The assets selector now has six
planned cases; its five prior passes remain evidence of the earlier source only.
Rustfmt and diff checks exit0; no runtime result is claimed for this addition.
No schema, transport, dependency, licence source or fixture changes accompany it.

## Fixed output discovery — next bounded validation window

Recovery-only source is `bdfa7ca69` (full immutable head in Git). The next source
checkpoint adds outputs.rs and five execution_outputs_ cases. The fixed workspace
is derived from the authorized session's bound generation; no caller path enters
the scanner. The writer repeats the same principal/task/profile/generation checks
and compares the pre-scan observation revisions, so an older scan cannot replace
a newer committed observation. IDs remain stable across edits/removal; revisions
advance, removed candidates are labelled changed, and source fingerprints stay
private. Ended runs return only previously captured observations, never refresh
late workspace bytes after stop. A new generation cannot consume the old cursor.
Each generation retains at most512 candidate identities, with explicit rate-limit
refusal at the bound. No asset or publication is created by output discovery.

Exact new planned selector names:
- execution_outputs_revisioned_discovery_and_pagination_do_not_publish
- execution_outputs_foreign_cursor_and_member_transport_fail_before_scan
- execution_outputs_late_profile_revocation_retains_no_observation
- execution_outputs_reordered_scan_cannot_replace_newer_observation
- execution_outputs_stop_fence_and_new_generation_cannot_rebind_old_scan

The tests use real temporary files/SQLite but synthetic admitted engine links and
a synthetic DB stop/new-generation boundary. No process ownership/teardown or
HTTP/native delivery acceptance is implied. Current native rustfmt and diff
checks pass; all output/publication additions are still uncompiled at this freeze.
Applied offline code-context Atomic per-task staging / installed-version rules;
dependency corpus absence remains disclosed. Exact accepted B imports.rs source
was read via gh api; NOTICE records blob c0338191beff26817d2bf85b943fd0c2602a47c0.
Read installed chrono0.4.43 from_timestamp implementation for UTC file metadata.

Root has now explicitly returned the next serialized existing-target window.
Queued gates: publication8, assets6, the two adapted historical schema cases,
relevant existing task core controls, outputs5 and server check. Recheck≥10GiB
before each start and stop/report near5GiB. No new target/copy/export, fixture or
browser action. Initial failures, any bounded correction and exact final source
will be retained separately; nothing in this paragraph claims a test has run.

## Executed publication gate — capacity stop

Frozen compiling source: `4251987071e8e7213a678be94039cd3fad1328f1` (PR32 draft).
The owner-run `execution_publication_` command passed all8 cases, exit0:
Cargo1m57s, test runtime1.05s. Command used only the existing intake-tenancy target,
locked/offline/-j2; actual Cargo PID19985. No product correction was needed.
All12 changed product/NOTICE file pairs match that immutable head. Exact command,
blobs/SHA256 and log digest are in
`reports/business-ai-execution-validation/managed-checkpoint/digests.json`.
The unedited first log is `publication-initial.log`, SHA256
`778247d077532fc92e97519a83d101614dd8d784929a0b200d231189e54ebc78`.
Seven warnings remain (unused runtime consumers and inherited linker/future
compatibility notices); this is not Clippy or final E1 acceptance.

Immediate pre-start disk10,796,940KiB exceeded the required10GiB. After the gate,
disk9,744,388KiB (about9.29GiB) was below the threshold for a new command. No next
gate started. All-command window was explicitly released to root/approvals, and
the preceding progress message saying PID19985 was active was immediately
corrected when completion was collected. No own Cargo remains; no cleanup or new
target/copy/export occurred. The existing fixture/user browser is unchanged.

Pending for a later explicit window with fresh≥10GiB: assets6, outputs5, the two
exact adapted historical migration cases, existing task core controls and server
check. All their Rust bodies compiled in this lib-test build; their assertions
were not executed by the publication selector. Independent source follow-on for
3841/bdfa reported no added blocker, but is not execution evidence. The8 passes
prove the tested post-verification cancellation / current reader checks, real
file verification and task/receipt writer paths. They do not prove mid-blocking
abort, import lineage, actual runner ownership, HTTP/native byte delivery or
scoped event streaming. These remain implementation/acceptance work.

## Source-only correction: accepted empty/no-op scans

Reviewer/root identified a P2 in frozen425198707 outputs.rs: an older scan could
commit after a newer empty/unchanged scan because the revision vector did not
change. Example: A sees brief.md, it is removed, B accepts an empty scan, then A
inserts the vanished file as Available. The five original output tests did not
cover that no-row-change ordering case. This is source analysis, not a live
exploit/import authorization finding. Frozen425 and its8 publication passes are
preserved; those tests did not exercise this output race.

Root reserved `m20260909_000015_business_execution_scans`. The bounded correction
adds a separate org/session/generation scan-counter table and forward migration;
both000014 files remain byte-identical to425. Existing generation/output/authority
rows are retained; initial counter0 means no accepted scan under this mechanism,
not a grant/freshness reset. Future generations receive a counter in their own
writer transaction. Composite FK, immutable keys, monotonic increment and retained
rows prevent rebinding/reset. Session visible revision/timestamps are unchanged.

An active scan captures its counter before I/O. After existing current principal /
task/profile/generation validation, the final writer compares/advances that exact
counter and retains observations together. Every accepted scan advances it,
including empty/no-op results. A stale scan conflicts before retaining anything;
writer/read/commit failure rolls back counter and observations together. Ended
sessions still return only cached observations and do not advance the counter.
There is no missing-state fallback or reconstructed Principal.

Migration15 uses an explicit writer transaction with FK enforcement left on.
Schema/seed/triggers/marker commit together; the SeaORM receipt remains a separate
write, as verified in installed1.1.19 migrator source. Marker retry validates
coverage/FKs and preserves counters rather than re-seeding them. Two historical
14 test receipt totals now account for the additional15 receipt;14 DDL/rebuild
code is untouched. No existing fixture/database/target is migrated by this work.

The focused execution_outputs_ selector now has11 planned cases (previous5 plus6):
- execution_outputs_newer_empty_scan_cannot_resurrect_older_candidate
- execution_outputs_newer_unchanged_scan_still_fences_older_candidate
- execution_outputs_writer_and_commit_failures_preserve_fence_and_rows
- execution_outputs_migration_retains_generation_and_real_receipt_retry
- execution_outputs_migration_failure_rolls_back_schema_seed_and_receipt
- execution_outputs_migration_generation_fk_and_rollback_keep_exact_binding

The new runtime tests use the actual list_with interleaving, a fresh subsequent
scan positive control, unchanged visible session state, writer failure and a
deferred FK COMMIT failure. Migration tests start with actual14 receipts, retained
admission/grant/engine/output rows and temporary file metadata; they cover real
receipt-insert failure/retry, late DDL failure/rollback, future-generation seed
rollback, invalid composite parent references and refused counter reset/downgrade.
They do not claim a live process, real second-tenant execution or file transport.

Actual checks at this checkpoint: native rustfmt and git diff --check exit0;
git diff --quiet425 over both000014 files exit0. No new test, Cargo/server check,
fixture/import, provider or engine action has run. Tests remain held for explicit
root allocation and fresh≥10GiB; this correction is not independently executed or
closed yet. Exact accepted B source blobs and pinned SeaORM references are in
NOTICE; offline code-context staging/version rules and earlier missing corpus
coverage still apply. Only the bounded source/tests/NOTICE/report are changed.

Immutable correction source: `9e58f4e6a41569a96d79fc2af963acec6e5c5aa6`, pushed
to draft PR32. Seven changed product/NOTICE pairs and both preserved000014 hashes
are recorded in `reports/business-ai-execution-validation/scan-fence/source.json`.
This is a source checkpoint, not a compiling/test result. Reviewer/root receive
this exact pin for bounded re-review; no build window is claimed or consumed.

The bounded source review at `9e58f4e6a41569a96d79fc2af963acec6e5c5aa6` found no
additional blocking issue. It independently matched seven changed and two
preserved source pairs, including both migration000014 files and the NOTICE
attributions. The eleven scan-fence cases remain planned and unrun. The reviewer
kept the two prior14 receipt-count expressions as a documented migration-ledger
detail, and distinguished nonexistent-parent FK probes from real two-tenant
runtime evidence. Existing stopped-session coverage confirms cached observations
do not advance the counter. R4 remains open until an authorized compile/test
window produces execution evidence; no reviewer Cargo, fixture, target or
provider action occurred.
