# Step 1 — approvals

Implemented piece 1 on `feat/step1-approvals`, solely in `/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`. Implementation complete; all requested local validation gates passed. Draft PR publication is the remaining delivery step. No workers, merge, deployment, external sends or main integration.

## Behavior delivered

- IntroMail gate order: **deny → ask → payload-aware check → destructive floor → allow → mode**. Both explicit action allow and standing allow remain below the destructive floor. Missing scope defaults to propose; resource scopes override domain defaults; rule specificity and oldest-ID tie-breaking match the source.
- SeaORM Proposal, AuditLog, AgentRule and AgentScope equivalents use separate `ops_*` tables. Reserved migration is `m20260907_000001_ops_approvals`. It applies atomically on SQLite, enforces one pending proposal per task generation, prevents duplicate NULL-resource default scopes, validates modes/behaviors/statuses, and rejects audit UPDATE/DELETE.
- `propose` validates and gates a complete payload. Gate denial creates only an audit decision. Pending proposals move the existing task from running to awaiting_input; auto authorization returns an owned payload after commit.
- `approve` takes a `Review` containing the original snapshot and complete approved payload, allowing edits without filling fields from a newer draft. It checks proposal ID/action/task/run_seq/status, original payload, live task/folder, human identity boundary, and current policy for the edited resource. Successful CAS resumes running; it does not finish the task. An ask rule cannot conceal a payload deny when a human resolves it.
- Trusted async Action callbacks receive the same transaction and principal for live resource/permission reads. New action packs default to destructive=true. Callback/schema errors fail closed with non-payload error messages.
- `AuthorizedAction` has private fields, no Clone/Serialize/Debug, and contains the exact validated approved Value. It is returned only after proposal, audit and task timeline commit. A sorted-key compact JSON SHA-256 records the complete unredacted command, including recipients/attachment descriptors/text; audit contains no raw payload.
- Credential-shaped values are scrubbed recursively from previews/audit; payloads that would be changed by that sweep are rejected before execution binding. Declared private fields remain reviewable while pending and are overwritten on approval, denial or auto authorization, including original, edited and preview columns.
- The existing `flip_awaiting` CAS now refuses a generic ACP resume while an Ops proposal owns that task/run wait. This avoids stale cards attaching to another wait in the same generation. Ordinary ACP waits still resume after proposal resolution. Cancellation/new generations and review reject late approvals. Existing review-before-done transitions remain intact.

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

Base codeg source is `xintaofei/codeg@v0.30.4`, **`6f6bd648b206412644842a98d9ffeebf57292bed`**. CAS/timeline glue adapts `src-tauri/src/db/service/work_task_service.rs` (`flip_awaiting`, `record_event`, `status_str`). Entity/migration conventions come from `db/entities/work_task.rs`, `work_task_event.rs`, and `db/migration/m20260801_000003_work_task_template.rs`. The only existing state-machine edit is the 16-line proposal-wait guard; shared entity/service/migration registries have additive module entries. Tickets' tables are untouched.

`NOTICE` records exact files and immutable source SHAs, owner-authorized private source, and codeg Apache-2.0 attribution. There was no NOTICE at this branch's baseline, so none was overwritten. Original LICENSE is unchanged. No Plane/Twenty/Postiz, restricted Chatwoot enterprise, or Kun source was imported.

## Docs-first / hook evidence

Read FOUNDING.md, ORCHESTRATOR.md, STATUS.md, DECISIONS.md, AGENTS.md and reports/step0.md completely before implementation; the four protected planning documents remain unchanged.

The resumed session's first two commands were **separate** `cat node_modules/react/package.json` (React 19.2.4) and `cat src-tauri/Cargo.toml`, both exit 0. Immediately inspected live `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`. It contains paired PreToolUse/PostToolUse records at timestamps **1788789370** and **1788789374**, session **`01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`**, cwd **`/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`**, tool Bash, exit 0, context_emitted false. These are this worker's live reads, not another worktree's smoke test. Subsequent library edits also received the live docs-first reminder. Hooks were not disabled or bypassed.

Applied code-context using the existing `/Users/mohamedadan/projects/rag-skills/.venv/bin/python` with `HF_HUB_OFFLINE=1`. `guide 'Rust SeaORM approval CAS transactions audit redaction' --project ops-desk` exited 0. Relevant retrieved principle: **“Human-in-the-loop: nothing is filed/sent to authorities without explicit human approval”**; retrieved guidance came from other projects, not ops-desk-specific coverage. `docs 'SeaORM transaction conditional update' --repo <this worktree>` exited **3** because `data/code/approvals.db` is absent. No dependency-corpus coverage is claimed and no shared corpus was mutated.

Used installed source as the fallback: **SeaORM/sea-orm-migration 1.1.19** query/update.rs, database/transaction.rs, database/mod.rs, migration connection.rs and migrator.rs; **sha2 0.10.9** lib.rs; **serde_json 1.0.149** map.rs and Value::sort_all_objects; **async-trait 0.1.89** lib.rs; **Tokio 1.49.0** join macro docs; **regex 1.12.3** Regex construction/matching source. Pinned versions were read from Cargo.lock. Installed gh api/pr-create/pr-edit, git add/commit/push, cargo check/test/clippy, pnpm/install/TypeScript, and rustfmt help were read. No latest remote dependency documentation or dependency upgrades were needed.

## Validation

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

## Limits and Step 2 contract

- This is the shared backend approval core. Transport routes, queue UI, trusted action registry/domain packs, ACP/pi wiring and human authentication are subsequent adapters, not shipped by this piece. Actor strings must come from authenticated human identity; the service rejects empty/self-agent identities but is not itself an authentication system. Action implementations must be trusted, validate complete schemas, read through ActionContext, and never perform sends in callbacks.
- `approved`/`auto` mean **authorization committed**, not “sent” or “task done.” The exact owned handoff is the only dispatch input; never read a mutable draft or replay a resolved row. No executor/network call is provided. Crash after authorization yields no automatic replay. External delivery/idempotency/recovery remains an adapter responsibility; no exactly-once delivery claim.
- Source-specific user/thread/event/chat foreign keys, notifications, free-text feedback, HTTP IP attribution and one-click standing-rule creation were not transplanted into unrelated codeg schemas. Agent IDs are canonical strings; task_id/run_seq are mandatory. Rule/scope tables support the source gate; policy-management UI is later work.
- Canceled/retired pending rows cannot authorize another generation and remain pending with review content; retention/expiry cleanup is not implemented here. Private-field redaction is declaration-based and credential matching intentionally preserves the source's exact key-name regex, not arbitrary secrets embedded in prose or every spelling variation. Domain packs must keep credentials in credential stores and declare content fields.
- Known upstream desktop warnings: zero-byte MCP sidecar placeholder and proc-macro-error2 2.0.1 future compatibility. No packaged/running desktop, real sidecar, frontend production build, browser flow or end-to-end external process was validated by this Rust-only piece.

## Checkpoint and delivery

Initial source report pushed as `9ed749d1`. Owner-requested hook-reload product checkpoint: **`29fe00c85160ccb0de35787a515d060c021e6b32`**, report follow-up `c8f86674`. Stopped exactly for reload and resumed only when instructed. All local gates passed. Final implementation SHA and draft PR URL are recorded in the delivery follow-up below.
