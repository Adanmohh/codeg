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

## Orchestrator reload checkpoint

Stopped on owner request to reload the newly trusted Codex docs-first hook. Implementation is INCOMPLETE; do not merge this checkpoint. No further product edits made after that request.

Current files: four SeaORM entities (ops_proposal, ops_audit_log, ops_agent_rule, ops_agent_scope), reserved migration plus minimal registries, ops_approvals/{mod,gating,redaction}.rs core, and new NOTICE (there was no existing NOTICE in this worktree). Exact source mapping remains above and in NOTICE.

Commands/results:
- `gh api` immutable commit resolution, recursive tree verification, and named source reads: exit 0.
- code-context `guide`: exit 0; dependency `docs`: exit 3, missing approvals.db (documented above).
- `pnpm install --frozen-lockfile`: exit 0, pnpm 11.9.0; lockfiles unchanged.
- `cargo check --locked --manifest-path src-tauri/Cargo.toml --target-dir src-tauri/target-approvals`: exit 0, desktop default features, 2m06s. Local log reports/approvals-desktop.log. Used this worktree's out/index.html placeholder per CI. Known upstream zero-byte sidecar and proc-macro-error2 warnings only.
- Installed gh api/pr create, git add/commit/push, cargo check/test, pnpm/install, TypeScript help read. `git ... -h` returns its standard help exit 129.

Resume work:
1. Reload and obey the newly activated docs-first hook before further work. Reuse the immutable source SHA, never switch to current HEAD for the port.
2. Review the unformatted core implementation. The `#[cfg(test)] mod tests;` declaration is present but tests.rs has NOT yet been created, so test compilation is currently incomplete.
3. Fix task_cas live-folder check to reject soft-deleted folders (currently only checks row existence). Review fail-closed policy exceptions and approval-denied audit transaction handling.
4. Add meaningful gate precedence matrix, scope specificity, exact edited-payload handoff, invalid payload, credential/private redaction, stale run/task/review/deletion, competing approve/deny and audit-write rollback tests. Use upstream work_task create/claim_for_run/begin_setup/mark_running/settle_review helpers already read. No tests have run yet; earlier progress commentary described intended tests, not completed validation.
5. Consider recording an immutable approved-payload digest in audit for after-resolution binding evidence; read local sha2 usage/source before using it. Current exact binding is a non-Clone AuthorizedAction carrying the approved Value, original-payload comparison, and transactional pending/run_seq CAS.
6. Run formatting on only owned files, focused tests, default desktop cargo check --locked again after changes, server cargo check --locked --no-default-features --bin codeg-server, pnpm exec tsc --noEmit. Server/typecheck remain unrun.
7. Finish source mapping/limitations, push and open small draft PR to main using the already-read gh pr create help. Draft PR URL: pending; checkpoint is not review-ready.

Build output is isolated in this worktree at src-tauri/target-approvals (untracked, deliberately not committed); preserve for resumed checks. Local logs are ignored. Do not stage that build directory. No other worktree or main outputs were written. No transport/API/UI adapter, external sender or domain action has been introduced; Step 2 will consume the trusted Rust service boundary. Scope unchanged.

Initial report commit: 9ed749d1. Product checkpoint SHA is recorded below after its commit; report-only follow-up will reference it.
