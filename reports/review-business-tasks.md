# Independent PR22 task review

Review in progress. One source-established authorization blocker is recorded below; this is not an acceptance verdict or a claim that the unfinished checkpoint is deployed.

## Frozen target and isolation

- PR: https://github.com/Adanmohh/codeg/pull/22, draft, resolved with `gh api`.
- Reviewed head: `76bb6909511016a11e864abe19d4ffd493aafa0c` (`feat/business-tasks`).
- Task-only comparison: `861fb0ef4394d4980a19ba375bb4c1f3f218d39b..76bb6909511016a11e864abe19d4ffd493aafa0c`; 17 files, 1,482 added lines. The base is the accepted identity implementation already incorporated by the task worker.
- Report branch: `review/business-tasks`, created from accepted main `d9882b68b268b516b1d1a24031862a022a797899`; identity merge `ab46c9d9` is an ancestor.
- Tracked work was clean before switching. Existing `.build/`, `out-design-final/`, and `reports/visual-refresh-baseline.md` were preserved. No product edits, other-worktree writes, fixture mutations, live providers, or new agents.

## Findings

### R1 — P1: live execution existence is accepted as business execution ownership

**Locations at the reviewed head:** `src-tauri/src/work_task/desk.rs:78–85`; `src-tauri/src/business_tasks/store.rs:309–325`; `src-tauri/src/business_tasks/agent.rs:19–30`.

`link_business_execution` looks up the request's numeric `workTaskId`, takes that row's root connection, and asks `desk_scope(root)` for a live generation. That proves engine liveness and ancestry. `store::link` then verifies permissions on the destination business task and active assigned agent, but never verifies that this engine run was entrusted to this organization/domain/business task or to that business agent member. It stores the independently chosen assignee ID beside the live connection's engine-type key. Neither check establishes a relationship between those two identities.

**Reproducible trigger to cover in the completed transport:** create a Feedback manager and an active Feedback agent member, create a Feedback task assigned to that agent, and use an unrelated, unbound live engineering run's integer ID in `POST /api/business/tasks/link-execution` with the correct task revision. The current branch of checks accepts the unrelated live run and records a delegation under that manager. The requester need not be the legacy operator or own the source execution. This defeats the required business-scope/assigned-agent ownership condition; once agent dispatch is connected, the unrelated run can resolve the grant and act as the selected business member.

**Required fix:** require a backend-owned authorization binding for the source run, matching the current organization, business scope/task, and assigned business agent; validate it under the same engine lifecycle lock and database writer transaction before recording the delegation. Do not infer ownership from the numeric ID, connection liveness, or a generic agent-type wire key. If an existing legacy run has no trusted business ownership provenance, fail closed until the trusted bridge establishes it. Keep link execution available to an Assign-authorized human when that proof exists. Add a negative test with a real unrelated live/indexed connection, plus a valid binding control, wrong assigned-agent case, and retirement/reassignment race assertions that leave revision/activity/link rows unchanged on rejection.

**Evidence classification:** independently read source/control flow, not a fabricated executed exploit. The reviewed checkpoint is not yet fully registered, as described below.

## Checkpoint limits and remaining review

The frozen head adds `work_task/desk.rs` references to `crate::business_tasks` while `src-tauri/src/lib.rs` does not declare that module; migration 000010, task HTTP routing, and the twelve native commands are also not registered there. This is the known integration work for which the task worker has applied the separately committed registration patch locally. That later uncommitted state is outside this immutable review. No Rust/task API test pass is claimed for `76bb6909`.

Read so far: both business contracts, the complete task checkpoint report, task NOTICE diff, all task store/policy/agent code, the new engine-link method and existing `desk_scope`, and the committed test bodies. Remaining: finish schema/DTO/transport and lifecycle review, independently verify exact borrowed source provenance, finalize findings and validation limits.

## Docs-first evidence

Applied the local code-context skill with the existing RAG environment and `HF_HUB_OFFLINE=1`; guide query exited 0. It returned general SQLite/local validation conventions, not a task-specific authorization corpus. Prior installed-doc lookup did not contain this worktree's dependency index; direct installed source is authoritative and no corpus coverage is invented.

Read `node_modules/react/package.json` and `src-tauri/Cargo.toml` separately. Relevant pins: React 19.2.4, SeaORM/sea-orm-migration 1.1.19, Axum 0.8.8, Tokio 1.49.0, Serde 1.0.228, UUID 1.20.0, Chrono 0.4.43. Read SeaORM's installed transaction begin/commit/drop rollback implementation and the accepted identity writer-lock/revalidation helpers before assessing transaction behavior.

Live hook audit evidence from `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`, this worktree and session `01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`: `PreToolUse` time `1788857631`, `PostToolUse` time `1788857603`, both exit 0. Hooks remained enabled.

Fixture 4341 (previously recorded PID 29883) and all earlier outputs/listeners are preserved; this review has not used or changed their state.
