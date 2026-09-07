# Step 1 — approvals

Acceptance fixes complete and pushed, 2026-09-07. All combined local validation gates passed.
Read the complete independent review at `reports/review-approvals.md`; both P2
findings were confirmed and addressed below. Accepted main `d63f98b9` was merged
and pushed as `e1585541`, preserving both NOTICE sections and main's root
planning documents exactly. Fix commit **`ae1cfbfc2ed8d89125354c2f67ef969976df8edd`**
was pushed before fetching main, as requested. Integrated main
**`0529396eb7d778beb9c8bed8424f292a013370cb`**, which includes the accepted tickets
merge and `5bd0b1e1`, as **`c6ccd16cb87b0b6443124be52e977f9c4c6b018c`**, now pushed.
The only conflict was the migration registry; it now registers approvals before
tickets and retains every prior entry. The tracked ticket
review comes exclusively from main; the earlier local copy remains an ignored
log and will not be staged. Draft PR remains [#3](https://github.com/Adanmohh/codeg/pull/3).

Implemented piece 1 on `feat/step1-approvals`, solely in `/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`. Original delivery passed its local gates; acceptance-fix validation is recorded separately below. No workers, PR merge, deployment or external sends.

## Acceptance corrections

- **P2: overlapping wait owners.** Replaced the engine's task-only in-memory set with `ops_acp_wait`, keyed by task, run_seq, connection and namespaced request ID. SQLite serializes ACP acquisition/retraction and Ops resolution; status remains awaiting_input while either owner exists. Approval/denial resolves only its proposal before reconciling all owners in the same transaction. Generic resume cannot clear another ACP key or an Ops proposal.
- Engine request attribution and connection/subtree detachment share a mutex through the database commit. Audited `track_request`, delegation registration/backfill, `forget_delegation_child`, `retire_connection`, launch-unwind `forget_connection`, cancellation and stale-run retirement. Cancellation clears ACP rows in its winning transaction and returns that transaction's run_seq; delayed teardown only disconnects that generation. A fresh claim purges older keys; stale event arrivals cannot acquire a wait for the current run.
- **P2: migration position assumption.** Both migration tests select `m20260907_000001_ops_approvals` explicitly by `MigrationName`, with a real later test migration and preserved marker data. They call that migration's up/down directly and check atomic failure without removing unrelated schema. The test migrator starts with the full application registry and now includes the accepted tickets migration.
- Added seven engine regressions covering approval and denial with parent plus child ACP requests pending, concurrent decision/arrival for both outcomes and both connection kinds, concurrent subtree/parent cleanup, cancellation, and a controlled new generation becoming live before old cancellation teardown. Added two approval tests for separate-connection SQLite races and rollback of wait ownership, status timeline and approval audit. Existing retirement tests now inspect persisted requests, including stale-generation rejection.
- Borrowed cleanup/source-set patterns remain pinned to Codeg `6f6bd648b206412644842a98d9ffeebf57292bed`, `src-tauri/src/work_task/engine.rs`. `gh api` verified immutable blob `c8674facca32b50cc7e4c5dde04f4f104a8fcba7`, matching the local base file. NOTICE retains the original entries and adds this exact source. No IntroMail gate, redaction or borrowed-source version change.

Pre-integration evidence (common Cargo arguments from Validation below): `cargo test --lib overlapping_acp_waits` exited **101**, reproducing the approval and denial overlap failures (Running instead of AwaitingInput). After the fixes, `cargo test --lib ops_` exited **0**, **88 passed**; this broad substring also matches unrelated existing tests, so it is not an approval-only count. `cargo test --lib work_task::engine::tests` exited **0**, **128 passed**, including the existing delegation, retirement, cancel, recovery and compaction gates. Logs: `approvals-overlap-red.log`, `approvals-fixes-second.log`, `approvals-engine-preintegration.log`. One initial helper-signature compile failure was corrected before the behavioral RED run; it is not counted as a regression reproduction. The combined suite was rerun after formatting and integration, with results below.

## Behavior delivered

- IntroMail gate order: **deny → ask → payload-aware check → destructive floor → allow → mode**. Both explicit action allow and standing allow remain below the destructive floor. Missing scope defaults to propose; resource scopes override domain defaults; rule specificity and oldest-ID tie-breaking match the source.
- SeaORM Proposal, AuditLog, AgentRule and AgentScope equivalents use separate `ops_*` tables. Reserved migration is `m20260907_000001_ops_approvals`. It applies atomically on SQLite, enforces one pending proposal per task generation, prevents duplicate NULL-resource default scopes, validates modes/behaviors/statuses, and rejects audit UPDATE/DELETE.
- `propose` validates and gates a complete payload. Gate denial creates only an audit decision. Pending proposals move the existing task from running to awaiting_input; auto authorization returns an owned payload after commit.
- `approve` takes a `Review` containing the original snapshot and complete approved payload, allowing edits without filling fields from a newer draft. It checks proposal ID/action/task/run_seq/status, original payload, live task/folder, human identity boundary, and current policy for the edited resource. Successful CAS releases the proposal's wait and resumes running only when no ACP request remains; it does not finish the task. An ask rule cannot conceal a payload deny when a human resolves it.
- Trusted async Action callbacks receive the same transaction and principal for live resource/permission reads. New action packs default to destructive=true. Callback/schema errors fail closed with non-payload error messages.
- `AuthorizedAction` has private fields, no Clone/Serialize/Debug, and contains the exact validated approved Value. It is returned only after proposal, audit and task timeline commit. A sorted-key compact JSON SHA-256 records the complete unredacted command, including recipients/attachment descriptors/text; audit contains no raw payload.
- Credential-shaped values are scrubbed recursively from previews/audit; payloads that would be changed by that sweep are rejected before execution binding. Declared private fields remain reviewable while pending and are overwritten on approval, denial or auto authorization, including original, edited and preview columns.
- The existing `flip_awaiting` compatibility entry point owns a distinct aggregate ACP key. It cannot release an Ops proposal or any engine request key. Cancellation/new generations and review reject late approvals. Existing review-before-done transitions remain intact.

## Immutable source-to-port mapping

Resolved **one** IntroInnovation/intromail commit via `gh api repos/IntroInnovation/intromail/commits/HEAD` before reading any source: **`0bd24dfe284b888aa9f602fa1fd00e337ea38874`**. Verified paths through the recursive Git tree, then read each source with `gh api .../contents/PATH?ref=SHA`. All remote research used gh api; mandated borrows were never upgraded.

| Exact source at that commit | Port / use |
| --- | --- |
| `backend/app/services/agent/gating.py` | `src-tauri/src/db/service/ops_approvals/gating.rs`: precedence, mode selection, rule matching |
| `backend/app/services/agent/proposals.py` | `src-tauri/src/db/service/ops_approvals/mod.rs`: propose/approve/deny, revalidation, edit-before-approve, resolved history redaction |
| `backend/app/services/agent/redaction.py` | `src-tauri/src/db/service/ops_approvals/redaction.rs`: exact case-insensitive credential-key expression and recursive two-tier scrub |
| `backend/app/services/agent/actions.py`, Action/ActionContext/Decision only | Trusted Rust Action and transaction/principal context; domain actions were not imported |
| `backend/app/audit.py` | Structured audit writer; upgraded best-effort behavior to transaction-required/fail-closed |
| `backend/app/models.py`, Proposal/AuditLog/AgentRule/AgentScope only | `src-tauri/src/db/entities/ops_{proposal,audit_log,agent_rule,agent_scope}.rs` and reserved migration |

Base codeg source is `xintaofei/codeg@v0.30.4`, **`6f6bd648b206412644842a98d9ffeebf57292bed`**. CAS/timeline glue adapts `src-tauri/src/db/service/work_task_service.rs` (`flip_awaiting`, `record_event`, `status_str`, claim and cancel). Entity/migration conventions come from `db/entities/work_task.rs`, `work_task_event.rs`, and `db/migration/m20260801_000003_work_task_template.rs`. The acceptance fix additionally ports the engine's request set into a fifth glue table, `ops_acp_wait`, in the same reserved approvals migration. Shared entity/service/migration registries have additive module entries. Tickets' tables are not edited by this worker.

`NOTICE` records exact files and immutable source SHAs, owner-authorized private source, and codeg Apache-2.0 attribution. There was no NOTICE at this branch's baseline, so none was overwritten. Original LICENSE is unchanged. No Plane/Twenty/Postiz, restricted Chatwoot enterprise, or Kun source was imported.

## Docs-first / hook evidence

Read FOUNDING.md, ORCHESTRATOR.md, STATUS.md, DECISIONS.md, AGENTS.md and reports/step0.md completely before implementation, and reread the updated root documents after integration. Protected planning documents changed only through the authorized merges and match the integrated main commit exactly.

The resumed session's first two commands were **separate** `cat node_modules/react/package.json` (React 19.2.4) and `cat src-tauri/Cargo.toml`, both exit 0. Immediately inspected live `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`. It contains paired PreToolUse/PostToolUse records at timestamps **1788789370** and **1788789374**, session **`01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`**, cwd **`/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`**, tool Bash, exit 0, context_emitted false. These are this worker's live reads, not another worktree's smoke test. Subsequent library edits also received the live docs-first reminder. Hooks were not disabled or bypassed.

Applied code-context using the existing `/Users/mohamedadan/projects/rag-skills/.venv/bin/python` with `HF_HUB_OFFLINE=1`. `guide 'Rust SeaORM approval CAS transactions audit redaction' --project ops-desk` exited 0. Relevant retrieved principle: **“Human-in-the-loop: nothing is filed/sent to authorities without explicit human approval”**; retrieved guidance came from other projects, not ops-desk-specific coverage. `docs 'SeaORM transaction conditional update' --repo <this worktree>` exited **3** because `data/code/approvals.db` is absent. No dependency-corpus coverage is claimed and no shared corpus was mutated.

Used installed source as the fallback: **SeaORM/sea-orm-migration 1.1.19** query/update.rs, database/transaction.rs, database/mod.rs, migration connection.rs and migrator.rs; **sha2 0.10.9** lib.rs; **serde_json 1.0.149** map.rs and Value::sort_all_objects; **async-trait 0.1.89** lib.rs; **Tokio 1.49.0** join macro docs; **regex 1.12.3** Regex construction/matching source. Pinned versions were read from Cargo.lock. Installed gh api/pr-create/pr-edit, git add/commit/push, cargo check/test/clippy, pnpm/install/TypeScript, and rustfmt help were read. No latest remote dependency documentation or dependency upgrades were needed.

## Original delivery validation

All Cargo commands run from this worktree with `--locked --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/target-approvals`. This is an isolated local output directory (locally ignored inside itself), not another worktree/main's target. `out/index.html` is this worktree's ignored CI placeholder from `.github/workflows/test.yml` / reports/step0.md.

| Command (common Cargo arguments above) | Result | Local ignored log |
| --- | --- | --- |
| `pnpm install --frozen-lockfile` | Exit 0; pnpm 11.9.0 | reports/approvals-install.log |
| `cargo test --lib ops_approvals` | Exit 0; **19 passed**, 0 failed | reports/approvals-tests.log |
| `cargo test --lib work_task_service` | Exit 0; **39 passed**, 0 failed | reports/approvals-work-task-tests.log |
| `cargo check` (default desktop features) | Exit 0 | reports/approvals-desktop.log |
| `cargo check --no-default-features --bin codeg-server` | Exit 0 | reports/approvals-server.log |
| `cargo clippy --no-default-features --bin codeg-server --lib -- -D warnings` | Exit 0 | reports/approvals-clippy.log |
| `pnpm exec tsc --noEmit` | Exit 0 | reports/approvals-typecheck.log |
| `rustfmt --edition 2021 --check` on owned module/entity/migration files | Exit 0 | reports/approvals-format.log |
| `git diff --check` | Exit 0 | Tool result |
| Both lockfiles and protected planning documents compared with checkpoint baseline | No diff | Tool result |

The first runnable test suite exited **101** with two real failures: deleted-folder approval was allowed and the approval digest was missing (11 passed/2 failed). Both were fixed; subsequent 17/18/19-test suites passed as transaction-context, atomic migration and generic-resume coverage was added. The first server clippy run exited 101 on cmp_owned in the audit edited flag; changed it to parsed-JSON equality, added unchanged-payload coverage, and reran the approval suite, both builds and clippy successfully. Initial test scaffolding had two incorrect upstream helper argument lists (compile exit 101), corrected after reading their full signatures; that compile error is not counted as behavioral RED evidence.

Focused coverage includes the gate precedence matrix in all three modes, destructive/action/standing allow combinations, scope defaults/overrides, rule specificity/oldest ID, exact edited payload and digest, private/credential scrubbing, wrong snapshot/action/task/ID, malformed/invalid payload, actor rejection, current resource/principal reads, policy changes and ask-hidden deny, canceled/restarted/deleted/review generations, real racing approvals and approve-vs-deny on separate file-backed SQLite connections, rollback after late audit failures, append-only audit enforcement, migration uniqueness/up/down/failure rollback, and proposal-owned vs ordinary ACP waits. The existing work-task suite covers generation CAS and reviewed completion alongside the integration change.

The acceptance fix also used offline code-context guide `Rust shared wait ownership SQLite transactions approval concurrency cleanup stale generation`, exit 0. Applicable general rules: **SQLite-first persistence** and **Atomic per-task staging**; no Ops-specific corpus coverage was returned. Read installed **Tokio 1.49.0** mutex/join/timeout/yield_now docs, **SeaORM/sea-orm-migration 1.1.19** insertion/conflict, transaction and migration traits, and **sqlx-sqlite 0.8.6** connection defaults before implementing the change. Separate React package and Cargo manifest reads again exited 0. Current live hook evidence includes PreToolUse and PostToolUse at **1788811412**, this same approvals session/worktree, exit 0. Hooks remained enabled throughout.

## Limits and Step 2 contract

- This is the shared backend approval core. Transport routes, queue UI, trusted action registry/domain packs, action dispatch through ACP/pi and human authentication are subsequent adapters, not shipped by this piece. Actor strings must come from authenticated human identity; the service rejects empty/self-agent identities but is not itself an authentication system. Action implementations must be trusted, validate complete schemas, read through ActionContext, and never perform sends in callbacks.
- `approved`/`auto` mean **authorization committed**, not “sent” or “task done.” The exact owned handoff is the only dispatch input; never read a mutable draft or replay a resolved row. No executor/network call is provided. Crash after authorization yields no automatic replay. External delivery/idempotency/recovery remains an adapter responsibility; no exactly-once delivery claim.
- Source-specific user/thread/event/chat foreign keys, notifications, free-text feedback, HTTP IP attribution and one-click standing-rule creation were not transplanted into unrelated codeg schemas. Agent IDs are canonical strings; task_id/run_seq are mandatory. Rule/scope tables support the source gate; policy-management UI is later work.
- Canceled/retired pending rows cannot authorize another generation and remain pending with review content; retention/expiry cleanup is not implemented here. Private-field redaction is declaration-based and credential matching intentionally preserves the source's exact key-name regex, not arbitrary secrets embedded in prose or every spelling variation. Domain packs must keep credentials in credential stores and declare content fields.
- The reserved, still-unmerged approvals migration was extended with the ACP coordination table. Validation uses fresh test databases and the combined registry; no upgrade from a throwaway database created by the earlier unpublished approvals schema was implemented. No production database was opened or migrated by this worker.
- Known upstream desktop warnings: zero-byte MCP sidecar placeholder and proc-macro-error2 2.0.1 future compatibility. No packaged/running desktop, real sidecar, frontend production build, browser flow or end-to-end external process was validated by this Rust-only piece.

## Initial checkpoint and original delivery

Initial source report pushed as `9ed749d1`. Owner-requested hook-reload product checkpoint: **`29fe00c85160ccb0de35787a515d060c021e6b32`**, report follow-up `c8f86674`. Stopped exactly for reload and resumed only when instructed. All local gates passed. Final implementation SHA and draft PR URL are recorded in the delivery follow-up below.


Original implementation commit: **`f1cebd64ba84a69463d28e17b664cbd282cd7f96`** (`feat: bind approvals to reviewed payloads and task generations`), pushed to origin. Draft PR: **https://github.com/Adanmohh/codeg/pull/3**, created with `gh pr create --repo Adanmohh/codeg --base main --head feat/step1-approvals --draft --body-file reports/approvals-pr-body.log` (explicit title supplied), exit 0. Verified through gh api: open, draft=true, base main, head feat/step1-approvals. Original final report commit: `a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba`; the original validation table applies to that initial delivery, not the later acceptance fixes.

## Combined validation and acceptance-fix delivery

Tests use the real application migration registry, including `m20260907_000001_ops_approvals` followed by `m20260907_000002_ops_tickets`. Both approval migration tests additionally install a later fixture migration, explicitly select approvals by name, and verify the follower's marker survives target rollback/recreation and injected partial-DDL failure. No test assumes approvals is last.

All commands below use the common locked Cargo manifest/target arguments described above. Validation target is integrated product commit **`c6ccd16cb87b0b6443124be52e977f9c4c6b018c`**.

| Command | Result | Local ignored log |
| --- | --- | --- |
| `cargo test --lib db::service::ops_approvals` | Exit 0; **21 passed**, 0 failed | reports/approvals-combined-approval-tests.log |
| `cargo test --lib db::service::ticket_service` | Exit 0; **18 passed**, 0 failed | reports/approvals-combined-ticket-tests.log |
| `cargo test --lib db::service::work_task_service` | Exit 0; **39 passed**, 0 failed | reports/approvals-combined-work-task-tests.log |
| `cargo test --lib work_task::engine::tests` | Exit 0; **128 passed**, 0 failed | reports/approvals-combined-engine-tests.log |
| `cargo check` (default desktop) | Exit 0 | reports/approvals-combined-desktop-check.log |
| `cargo check --no-default-features --bin codeg-server` | Exit 0 | reports/approvals-combined-server-check.log |
| `cargo clippy --all-targets --features test-utils -- -D warnings` | Exit 0 | reports/approvals-combined-desktop-clippy.log |
| `cargo clippy --no-default-features --bin codeg-server --lib -- -D warnings` | Exit 0 | reports/approvals-combined-server-clippy.log |
| `pnpm exec tsc --noEmit` | Exit 0 | reports/approvals-combined-typecheck.log |
| `rustfmt --edition 2021 --check` on owned approval/wait entities, modules and migration | Exit 0 | reports/approvals-combined-format.log |

Final hook audit read also verified this same session/worktree's live PreToolUse at line **1231**, timestamp **1788812024**, and PostToolUse at line **1227**, timestamp **1788812004**; both exit 0. No hook was disabled, bypassed or reconfigured.

Preservation checks exited 0: both parents' NOTICE lines remain in order; all entity/service/migration registry entries from both parents remain; protected root documents, both review reports, the ticket report and all ticket implementation files, both lockfiles and LICENSE match integrated main exactly. An earlier comparison against the moving shared `origin/main` ref exited 1 because other worktrees advanced that ref; comparison against the immutable integrated parent passed. No review report was duplicated or edited. `git diff --check` and formatting checks on the changed owned entity/service/migration/test modules passed.

The combined Cargo runner exited 0, with all eight command exits recorded in `reports/approvals-combined-exits.log`. These are local passes, not a CI-pass claim. Default desktop compilation retains the inherited sidecar-placeholder warning; Rust retains the proc-macro-error2 future-compatibility notice.

Updated PR #3 with `gh pr edit 3 --repo Adanmohh/codeg --title <recorded title> --body-file reports/approvals-pr-body.log`, exit 0, after reading installed help. Title: **feat: add audited approvals bound to reviewed payloads**. The description records both P2 fixes and every combined gate. `gh api repos/Adanmohh/codeg/pulls/3` confirmed open, draft=true, base main, head feat/step1-approvals at the validated product SHA. This report follows in a docs-only commit. No PR merge, deployment, external action execution, people messages, other worktree writes or lockfile changes. Only this approvals report is being updated; the orchestrator owns acceptance. Packaged-app/browser/real-agent validation remains outside this backend correction; main's browser report is the orchestrator's separately attributed evidence.
