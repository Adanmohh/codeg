# Independent PR22 task review

**R1 is resolved at `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5`. No blocking finding remains in this bounded task authorization review.** Independently reviewed the committed correction and ran **13 focused tests: 5 ownership, 6 existing business bridge, and 2 migration/HTTP tests**, all passing. An unrelated live run cannot become a business agent through human linking alone; actual operator source entrustment is now required, while the subsequent human link retains its original individual credential lineage.

This verdict covers the frozen source and tests below. It does not replace the task worker's default-native, Clippy, Pi CLI or UI acceptance gates, and does not claim malicious same-user process isolation. No fixture state or product source was changed by this reviewer.

## Frozen target and isolation

- PR: https://github.com/Adanmohh/codeg/pull/22, draft, resolved with `gh api`.
- Final reviewed head: `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5` (`feat/business-tasks`), independently resolved with `gh api`; the review did not follow subsequent worker changes.
- Correction comparison: `f83bf6c82731230adbb1aa5c6c13be4fe4fd795a..1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5`; read the full product, migration, test, NOTICE and contract changes.
- Original reviewed head: `76bb6909511016a11e864abe19d4ffd493aafa0c`. Task-only comparison: `861fb0ef4394d4980a19ba375bb4c1f3f218d39b..76bb6909511016a11e864abe19d4ffd493aafa0c`; 17 files, 1,482 added lines. The base is the accepted identity implementation already incorporated by the task worker.
- Report branch: `review/business-tasks`, created from accepted main `d9882b68b268b516b1d1a24031862a022a797899`; identity merge `ab46c9d9` is an ancestor.
- Initial report checkpoint `592618ec40977348964f3b882775a66f6425c60f` and validation/provenance checkpoint `6138e10e` were committed and pushed early. Findings and the final exact-head verdict were relayed to existing tickets/root panes through the authorized Herdr interface, exit 0; no new worker was created.
- Tracked work was clean before switching. Existing `.build/`, `out-design-final/`, and `reports/visual-refresh-baseline.md` were preserved. No product edits, other-worktree writes, fixture mutations, live providers, or new agents.

## R1 closure and current review

All file/line references in this section are at final head `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5`, relative to `src-tauri/src/`. Unqualified `store.rs`/`agent.rs` references refer to `business_tasks/`.

| Boundary | Independently verified correction |
|---|---|
| Actual operator source authority | `business_tasks/mod.rs:29` rejects any principal without `is_operator()` and any agent context. `work_task/desk.rs:72` holds the existing engine request mutex while deriving the live root/run and committing entrustment. `business_tasks/store.rs:674` repeats current authorization after acquiring SQLite writer ownership. An owner-role member credential is not this transport authority. The real authenticated HTTP test also returns 403 for that credential. |
| Exact source scope and revision | `business_tasks/agent.rs:29` requires the stored organization, business task, domain, assigned agent UUID, connection and engine key to match the current task and live generation. `store.rs:723` then requires the source's post-entrustment task revision to equal the current pre-link revision. Changing assignment/domain/cancellation away and back cannot restore that source authorization. |
| Separate human grant | `store.rs:738` serializes the linking human's opaque `DelegationGrant`, not the entrusting operator's authority. The execution row retains its exact `authority_id`. `agent.rs:84`, `:132` and `:187` validate that source on restoration and on each read/write transaction. Accepted `business_identity/mod.rs:163` and `:190` reread the original credential, current human grants and active agent membership; a replacement credential for the same human does not revive the original link. |
| Root and lifecycle fencing | `work_task/desk.rs:178` holds the same request mutex through each Desk operation. Its `:192` business branch rejects a live delegated child even if it is another Pi. Existing cancellation/run checks repeat inside the SQLite transaction. Tests cover actual indexed roots, a live same-type child, queued retirement, assignment races, changed generation, current grant changes and aborted requests. |
| Migration and immutable history | `db/migration/m20260908_000010_business_tasks.rs:58` adds scoped source authority, including positive source revision and one source per run generation; `:93` binds each execution to that same task's authority. Unique generation bindings and the `:129` onward triggers prevent source rewriting/deletion and execution lineage rewriting/reactivation. DDL is explicitly transactional; the exact-name migration test passes empty rollback, induced creation failure/retry, foreign-org rejection and populated rollback refusal. |

The new ownership tests in `work_task/desk/business_ownership.rs` were read completely before execution:

- `:51`: a real live/indexed unrelated run fails linking; owner-role member and manager credentials cannot entrust it. Rejected operations leave the task detail and source/link/activity/deliverable counts unchanged. Actual operator entrustment alone grants no agent access; the subsequent manager link succeeds with distinct operator/linker activity attribution. Source and grant mutation attempts fail.
- `:125` and `:173`: wrong assigned agent, assignment away/back, domain away/back and cancel/reopen use fresh request revisions but still reject the stale source. The recorded task detail/count snapshot remains unchanged by each rejected link.
- `:259`: retirement queues before linking under the real lifecycle mutex and wins; the rejected link changes no business records. A separately valid linked root works, while its live same-Pi child cannot submit and changes no records.
- `:317`: an independent SQLite connection holds the writer and changes assignment/revision while linking waits. After release, linking returns conflict. Only the competing writer's revision is present: one source, zero links, two existing activities, zero deliverables. This is an explicit DB-writer race, not a claim that the synthetic SQL update exercised the public assignment endpoint.

The six existing bridge regressions also passed on the fixed head, including original credential revocation, current human/agent grants, reassignment/cancellation/generation fencing, delayed payload retarget denial, queued revocation/cancellation and client abort, plus successful scoped contribution and human-only review.

**Operational limit retained by the accepted contract:** any intervening business task edit before linking invalidates the source revision. One immutable source/binding per engineering generation means a stale or revoked generation cannot be re-entrusted/relinked; a new generation is required. This strict behavior is intentional, not an automatic model-launch or recovery feature. No native/browser or live provider action was executed in this review. Simultaneous accept-versus-submit/assignment is not a separately executed test here; its shared writer/CAS path was source-reviewed, with earlier core race and rollback evidence retained below.

## Independently executed validation at the fixed head

Extracted unchanged tracked source with `git archive` into this worktree's `.build/review-business-tasks/1ba73e3c`; no registration patch or product edit was applied. The post-test Git-blob comparison found **726/726 tracked `src-tauri` and `integrations/pi-desk` files identical** to `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5` (exit 0). All commands below ran from that archive's `src-tauri`, using the owned existing target `src-tauri/target-approvals`, separate from the running identity fixture's target.

Common command prefix: `cargo test --locked --offline --no-default-features --lib`. Each command also passed `--target-dir /Users/mohamedadan/projects/_worktrees/ops-desk/approvals/src-tauri/target-approvals`.

| Exact selector / test arguments | Result | Log |
|---|---|---|
| Cargo filter `desk::tests::business::ownership` | exit 0; **5 passed**, 0 failed/ignored; tests 0.36s, compile 2m18s | `/tmp/review-business-tasks-1ba73e3c-ownership.log` |
| Cargo filter `desk::tests::business::`, then `-- --skip ownership` | exit 0; **6 passed**, 0 failed/ignored; tests 1.10s | `/tmp/review-business-tasks-1ba73e3c-bridge.log` |
| No Cargo filter; test arguments `-- task_migration_is_atomic_scoped_and_refuses_to_drop_durable_history real_business_router_derives_actor_rejects_spoofs_and_returns_revision_conflicts` | exit 0; **2 passed**, 0 failed/ignored; tests 0.25s | `/tmp/review-business-tasks-1ba73e3c-migration-http.log` |

The additional bridge runs are justified because source authorization is now required on their restoration and contribution paths. The two targeted tests cover the changed migration and new actual-operator HTTP gate. No broad suite was repeated. The installed test binary's `--help` was read before using multiple filters/`--skip`. Tokio 1.49.0's installed `sync/mutex.rs:20`, `:112` and `:415` document the FIFO/cancellation semantics used by the queued-retirement test.

Warnings remain the inherited macOS linker compact-unwind size warning and `proc-macro-error2` 2.0.1 future incompatibility notice. These fresh SQLite/in-process transport tests launched no model or external provider. Worker-reported broader counts are not substituted for these independent results.

## Historical finding — resolved at the fixed head

### R1 — P1 at original head: live execution existence accepted as business execution ownership

**Locations at the reviewed head:** `src-tauri/src/work_task/desk.rs:78–85`; `src-tauri/src/business_tasks/store.rs:309–325`; `src-tauri/src/business_tasks/agent.rs:19–30`.

`link_business_execution` looks up the request's numeric `workTaskId`, takes that row's root connection, and asks `desk_scope(root)` for a live generation. That proves engine liveness and ancestry. `store::link` then verifies permissions on the destination business task and active assigned agent, but never verifies that this engine run was entrusted to this organization/domain/business task or to that business agent member. It stores the independently chosen assignee ID beside the live connection's engine-type key. Neither check establishes a relationship between those two identities.

**Reproducible trigger to cover in the completed transport:** create a Feedback manager and an active Feedback agent member, create a Feedback task assigned to that agent, and use an unrelated, unbound live engineering run's integer ID in `POST /api/business/tasks/link-execution` with the correct task revision. The current branch of checks accepts the unrelated live run and records a delegation under that manager. The requester need not be the legacy operator or own the source execution. This defeats the required business-scope/assigned-agent ownership condition; once agent dispatch is connected, the unrelated run can resolve the grant and act as the selected business member.

**Required fix:** require a backend-owned authorization binding for the source run, matching the current organization, business scope/task, and assigned business agent; validate it under the same engine lifecycle lock and database writer transaction before recording the delegation. Do not infer ownership from the numeric ID, connection liveness, or a generic agent-type wire key. If an existing legacy run has no trusted business ownership provenance, fail closed until the trusted bridge establishes it. Keep link execution available to an Assign-authorized human when that proof exists. Add a negative test with a real unrelated live/indexed connection, plus a valid binding control, wrong assigned-agent case, and retirement/reassignment race assertions that leave revision/activity/link rows unchanged on rejection.

**Evidence classification at original head:** independently read source/control flow, not a fabricated executed exploit. That checkpoint was not yet fully registered, as described below.

**Historical follow-up:** R1 still existed at registered checkpoint `1e8b525076efe1aa3c4563c954d52c0affd5d3c7` (`desk.rs:78–86`, `store.rs:672–695`). Its new `check_link` preflight checked destination authority, not source ownership. The executed positive bridge fixture first created a generic engineering run and then linked it as a new Feedback business agent (`work_task/desk/business.rs:19–101`); it contained no preceding trusted source entrustment. Its successful tests therefore did not exercise the missing boundary. Later `f83bf6c82731230adbb1aa5c6c13be4fe4fd795a` prevented reuse of a previously bound generation but did not add source entrustment. These are separate immutable historical observations; final closure is supported by the subsequent source and tests above.

The proposed correction matched the accepted identity seam: only `Principal::is_operator()` plus current authorization inside the writer transaction may entrust a source run; an owner-role member credential does not have that authority. The subsequently linking Assign-authorized human must retain their own original credential lineage. Requested assignment/domain change-away-and-back, cancellation/reopen, exact root/generation and retirement coverage is now implemented and independently executed above.

## Historical checkpoint review and limits

The original `76bb6909` head added `work_task/desk.rs` references to `crate::business_tasks` while `src-tauri/src/lib.rs` did not declare that module; migration 000010, task HTTP routing, and the twelve native commands were also not registered there. Later committed checkpoints completed those registrations. No Rust/task API test pass is claimed for `76bb6909`; the final tests above execute the registered fixed source, including the additional entrustment route.

Read the complete original 17-file task-only change (including both contracts/report, NOTICE, all new core/schema/DTO/transport/test files), the accepted identity helpers, and relevant engine cancellation/retirement/ancestry code. Also inspected `1e8b5250` registrations, bridge, added tests and common-core changes, and the generation guard correction at `f83bf6c8`. The following table records that pre-fix review; its then-pending R1 coverage is superseded by the final review above.

| Review area | Verified behavior and limits |
|---|---|
| Identity and visibility | Human HTTP derives `Principal` from the existing business middleware; native commands derive original operator identity. No request actor/org/credential constructor. Task queries restrict organization plus current domain; detail/activity/deliverables share one authorized snapshot. Agent list/search is denied. |
| Delegation storage | `store::link` uses only identity's opaque `delegation_grant`; response DTOs omit `delegation_json`, credential IDs and root connection. Migration rejects lineage changes/deletion/reactivation. The missing source-run proof is R1, not a claim that lineage is accepted from JSON. |
| Original credential revocation | Restore rereads the original delegator credential, and `visible`/`authorize` repeat that check after SQLite writer acquisition. A different new credential for the same member cannot revive the old grant. The real listener/engine regression and queued-writer regression passed independently at `1e8b5250`. |
| Cancellation and generations | Business cancellation, reassignment, domain changes, archive and reopen revoke the existing execution. Agent writes recheck task/run/root/status/deletion inside the transaction. Engine `cancel` commits generation-fenced cancellation before teardown; `retire_connection` and `forget_delegation_child` share `request_lock` with Desk calls. The new Desk business dispatch keeps this lock through its core call. Existing bridge regressions pass for reassignment, engine cancellation and replaced generation; R1's new source-record races still require the fix's own tests. |
| Generation retargeting | `f83bf6c8` now rejects any previously bound `(work_task_id, run_seq)`, including a revoked binding, under the writer lock. That prevents a delayed revision-only agent payload from being redirected to a different task. This worker-originated correction was independently read, not yet independently executed at that head. |
| Review/CAS | State, next revision, immutable deliverable and activity commit together; the revision predicate and writer lock serialize concurrent writes. Edited metadata/assignments invalidate review. Only human Review capability plus the designated reviewer condition can reach done; the progress input enum cannot express done/cancelled. The tests exercise stale review after edit, racing metadata revisions, and audit-failure rollback; a simultaneous accept-versus-submit/assignment interleaving is not independently exercised yet. |
| References and migration | Source/destination domains and active owner/assignee/reviewer references are checked on writes; historical actors remain historical. Migration targets its exact name, wraps DDL in a transaction and refuses a populated rollback. Scoped foreign keys and immutable history are retained; `1e8b5250` additionally makes current-deliverable/execution pointers scoped foreign keys. The real migration failure/rollback test passed. |

No additional blocking schema, task visibility, credential restoration or review/CAS defect was established in this bounded review. This is not a UI/native acceptance verdict or same-user OS isolation claim.

## Historical independently executed validation at `1e8b5250`

To avoid changing the review branch's product files or testing uncommitted worker code, extracted an unchanged `git archive` of `1e8b525076efe1aa3c4563c954d52c0affd5d3c7` into this worktree's `.build/review-business-tasks/1e8b5250`. A post-test Git-blob comparison of all **723** tracked `src-tauri`/`integrations/pi-desk` files found **zero differences**. No registration patch or product fix was applied. Own existing Cargo output `src-tauri/target-approvals` was used; fixture4341 uses a different target.

Commands run from that archive's `src-tauri`:

| Command | Result | Log |
|---|---|---|
| `cargo test --locked --offline --no-default-features --lib business_tasks:: --target-dir /Users/mohamedadan/projects/_worktrees/ops-desk/approvals/src-tauri/target-approvals` | exit 0; **10 passed**, 0 failed/ignored; tests 0.62s, compilation 1m48s | `/tmp/review-business-tasks-1e8b5250-core.log` |
| Same Cargo flags, filter `desk_business_` | exit 0; **4 passed**, 0 failed/ignored; tests 0.47s | `/tmp/review-business-tasks-1e8b5250-bridge.log` |
| Post-test blob comparison | exit 0; 723 files match the immutable target | Result captured in review session |
| `git diff --check` | exit 0 | Report worktree |

Warnings: inherited linker compact-unwind size warning and `proc-macro-error2` future incompatibility notice. Tests use fresh in-memory/on-disk SQLite, synthetic identities and in-process framed listener transports; no real agent, provider, email or external action. No default-native/Clippy/frontend/browser rerun is claimed by this reviewer. The worker's earlier test counts are explicitly not substituted for these results.

## Independently verified source provenance

Local source/objects were read first; remote verification used only `gh api` with immutable refs. Codeg tag resolution independently returned `v0.30.4` → `6f6bd648b206412644842a98d9ffeebf57292bed`. Read its Apache-2.0 LICENSE (blob `261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64`), typed entities, task CAS/event transaction and migration source. Original LICENSE remains unchanged and NOTICE additions preserve earlier entries.

| Source commit | Exact file | Blob / reviewed use |
|---|---|---|
| Codeg `6f6bd648b206412644842a98d9ffeebf57292bed` | `src-tauri/src/db/entities/work_task.rs` | `a81a2f18e6dd9fd218474d2b9c4378f0b510ab54`; typed task states/entities |
| Same | `src-tauri/src/db/entities/work_task_event.rs` | `d34c80355efd6e401c58f3bb4e7ac48b8d11ddea`; task-linked activity |
| Same | `src-tauri/src/db/service/work_task_service.rs` | `8042ecea7d083a6246fe3726cad2ad9b5223449d`; conditional update + event transaction |
| Same | `src-tauri/src/db/migration/m20260801_000001_work_task.rs` | `2cecae7a30b58c694bda99de499effde4bc32c9e`; task/event schema convention |
| Accepted Codeg `4ec04d7282a50529335d724438d42b99a53385a2` | `src-tauri/src/ops/agent.rs` | `76a05103c318940bcdf131ac5c9ab2d4d5eadd0c`; live-run check, whose limits R1 identifies |
| Same | `src-tauri/src/ops/types.rs` | `3cdc2120cc341764e133a3ea7ca59c446de82110`; closed request DTOs |
| Same | `src-tauri/src/db/migration/m20260908_000008_ops_telegram_issues.rs` | `abcf6ce3d2bd95a4017c2ee98c6d99da30dbd6f0`; explicit SQLite migration transaction |
| Accepted Codeg `05e1ab7043eeadab753dbb2c6c6922661fff5c42` | `src-tauri/src/business_identity/tests/fixture.rs` | `9def1b639cc07a80389e18aa73d0797aaea51b17`; independently read original fixture and verified the new NOTICE attribution for the task-only manual fixture; Apache-2.0 |
| Owner-authorized IntroMail `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | `backend/app/models.py` (`Task` definition) | `ed783c0e2520b25b9e72c2c1f7ab439333fd28fc`; basic title/notes/due/creator vocabulary |
| Same | `backend/app/services/task_ops.py` | `3b1014af27a505c96633c767a9a82de6dc2d4b13`; common human/agent operation boundary; create's list existence check is not source authorization |
| Same | `backend/app/services/agent/tasks_pack.py` | `af500fbd419ece67787c84ad4e805424c48fec12`; current human/agent visibility intersection |
| Same | `docs/BORROW-PLANE-OPENPROJECT.md` | `637881c17aaef59d98bf67e06b5304e915a3902a`; read completely to identify mixed provenance, not a permitted source-code port |

IntroMail's tree response was not truncated and contained no LICENSE/NOTICE match. Its permission here is the owner's explicit authorization, **not** a fabricated permissive licence for the whole private repository. Its provenance document recommends Plane/OpenProject implementations for favorites, relations, watchers, source keys and field journals; those code paths are absent from the reviewed Rust port. The activity implementation instead uses the attributed Codeg transaction/event pattern. No GPL/AGPL/enterprise hunk was identified; this review does not certify every unrelated line of IntroMail history. `task_ops.py` history at the pin confirms introduction/refactor `d46b2b885671acb00a7ffc693040264154ca2f3a` and merge `da645b4c79c407445c4c9d4276920f730ebde30f`.

The later bridge extends the already attributed Pi adapter rather than copying a new framework. Read the actual installed `@earendil-works/pi-coding-agent` **0.85.1** extension types/runner and typebox **1.3.7** in the tickets worktree **read-only** after finding this worktree has no Pi installation. Immutable official Pi source remains `badlogic/pi-mono@d981de1229ef899957bbe968bc8dcda02a21f477`: `packages/coding-agent/src/core/extensions/types.ts` blob `eaff3a86ebf50ec543faf936699f5e6c84dfdf65`; `runner.ts` blob `3346645a10456c36f5fc3051fda1540c4e7d1f8b`; LICENSE blob `b0a8e9b81083294360c69b4ec45d3d39a2b28197` independently read and matches the retained Mario Zechner MIT notice. No dependency/lockfile update is part of the reviewed task change.

## Docs-first evidence

Applied the local code-context skill with the existing RAG environment and `HF_HUB_OFFLINE=1`; guide query exited 0. It returned general SQLite/local validation conventions, not a task-specific authorization corpus. Prior installed-doc lookup did not contain this worktree's dependency index; direct installed source is authoritative and no corpus coverage is invented.

Read `node_modules/react/package.json` and `src-tauri/Cargo.toml` separately. Relevant pins: React 19.2.4, SeaORM/sea-orm-migration 1.1.19, Axum 0.8.8, Tokio 1.49.0, Serde 1.0.228, UUID 1.20.0, Chrono 0.4.43. Read SeaORM's installed transaction begin/commit/drop rollback implementation and the accepted identity writer-lock/revalidation helpers before assessing transaction behavior.

Live hook audit evidence from `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`, this worktree and session `01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`: initial `PreToolUse` time `1788857631`, `PostToolUse` time `1788857603`, both exit 0. Final review also verified own live `PreToolUse` at `1788859439` and `PostToolUse` at `1788859419`, before updating this report. Hooks remained enabled.

Fixture 4341 remains PID **29883**, independently observed listening on **127.0.0.1:4341** again at handoff; this review has not used or changed its state or earlier fixture outputs. Corrected exploratory missing reads (this worktree's Pi package paths, a nonexistent `work_task/tests.rs`, and a `business_identity/auth.rs` search) by file discovery and reading the actual installed package, inline engine tests and identity `mod.rs`; no API/path existence was inferred from those failed reads.
