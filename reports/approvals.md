# Step 1 approvals

Branch: feat/step1-approvals. Sole writer in assigned approvals worktree.
Read FOUNDING.md, ORCHESTRATOR.md, STATUS.md, DECISIONS.md, AGENTS.md and reports/step0.md completely before implementation. No workers, external sends or deployment.

## Sources and grounding

Immutable IntroInnovation/intromail source: `0bd24dfe284b888aa9f602fa1fd00e337ea38874` (resolved via `gh api repos/IntroInnovation/intromail/commits/HEAD` before source reads).

- backend/app/services/agent/gating.py → ordered Rust gate and scope/rule matching.
- backend/app/services/agent/proposals.py → proposal creation, edit-before-approve validation, resolution and redaction.
- backend/app/audit.py and backend/app/models.py Proposal/AuditLog/AgentScope/AgentRule → SeaORM records.
- backend/app/services/agent/redaction.py and actions.py → recursive credential/private-field scrubbing and trusted action interface.
- codeg v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed`, src-tauri/src/db/service/work_task_service.rs → transactional CAS + timeline events; existing migration/entity conventions.

Read all named source via gh api at the immutable SHA. Local pinned SeaORM 1.1.19 query/update.rs and database/transaction.rs read before implementation. code-context guide exited 0: relevant principle “Human-in-the-loop: nothing is filed/sent to authorities without explicit human approval”; retrieved rules are from other projects, no ops-desk-specific rules. Dependency docs query exited 3: approvals.db is absent. Direct installed source is the fallback; no fabricated corpus coverage. Owner's latest-doc update accepted: use gh api for official GitHub docs, pinned borrows stay pinned.

## Decisions / progress

- Reserve migration `m20260907_000001_ops_approvals`, following local dated module naming. Separate ops tables; ticket tables untouched.
- Keep source gate order verbatim. Tighten source best-effort auditing to transaction-required, fail-closed writes.
- Core service only: trusted action implementations validate and classify payloads; no network executor. Successful resolution returns the exact validated approved payload once, after CAS commit. Step 2 adapters must dispatch that payload, never re-read a mutable draft. No automatic replay after a crash.
- Preserve work_task run_seq and review-before-done. Proposal wait/resume only uses running ⇄ awaiting_input.
- Validation and final commit/PR pending.
