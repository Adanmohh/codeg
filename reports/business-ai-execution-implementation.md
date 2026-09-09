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
read in separate commands. Live own-session hook records include PreToolUse26778–82
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

Current live hook metadata was independently read in this session without command
payloads: own session `01a07c1c-d82f-7022-84db-778a438632f1`, exact tickets cwd,
PreToolUse/PostToolUse exit0 records at Unix timestamps1788951206–1788951207.
