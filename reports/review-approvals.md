Independent acceptance review, 2026-09-07. **Two P2 findings require fixes before Step 1 integration.**

Reviewed commit: `a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba`.
Baseline: `2f74992e6b133a3ae3ffe3c1e4d0c14444b29137`.
The review target remains unchanged following the rebrand merge. I read the complete target `reports/approvals.md`, complete diff, new service/entities/migration/tests, and relevant existing task service and engine code through `git show`. No product source, registry, branch, or other worktree was modified.

1. **P2 — Resolving an Ops proposal clears an overlapping ACP permission wait.**

   Locations: [ops_approvals/mod.rs:343–350](https://github.com/Adanmohh/codeg/blob/a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba/src-tauri/src/db/service/ops_approvals/mod.rs#L343-L350) and [378–385](https://github.com/Adanmohh/codeg/blob/a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba/src-tauri/src/db/service/ops_approvals/mod.rs#L378-L385). Related existing behavior: [engine.rs:2886–2908](https://github.com/Adanmohh/codeg/blob/a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba/src-tauri/src/work_task/engine.rs#L2886-L2908).

   Reproducible trigger: start a task, create a pending Ops proposal, then receive an ACP permission request on its connection or a delegated child's connection before resolving that proposal. The engine records the outstanding request, but its `Running → AwaitingInput` CAS does nothing because the proposal already put the task in `AwaitingInput`. Approving **or denying** the proposal then unconditionally changes the task to `Running`, although the ACP request remains unanswered.

   The engine tracks ACP requests separately in `self.awaiting`; proposal resolution neither consults nor reconciles that set. Another outstanding request does not repair the status because the engine flips only on the first request. The task is consequently reported running while it still requires input, and the service's running-state gate can admit another proposal. This is a task-state defect; I am not claiming it bypasses the ACP permission response itself.

   Required fix: release only the Ops wait when resolving the proposal, and derive task status from all outstanding wait owners for the current run. Coordinate the database transition with the engine's ACP request tracking so a simultaneous arrival or resolution cannot lose a wait. Preserve the new protection against generic resume clearing an Ops wait. Add approval and denial tests where an ACP request is outstanding **during** resolution, including a concurrent arrival. The existing test at `ops_approvals/tests.rs:993–1021` starts its ACP wait after approval and misses this overlap.

   Evidence: source-level state-transition counterexample. I did not run a Rust engine reproduction.

2. **P2 — Migration tests roll back whichever migration is last, breaking the reserved tickets integration.**

   Locations: [ops_approvals/tests.rs:926–932](https://github.com/Adanmohh/codeg/blob/a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba/src-tauri/src/db/service/ops_approvals/tests.rs#L926-L932) and [970–976](https://github.com/Adanmohh/codeg/blob/a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba/src-tauri/src/db/service/ops_approvals/tests.rs#L970-L976).

   Reproducible trigger: register the reserved `m20260907_000002_ops_tickets` after `m20260907_000001_ops_approvals`, then run these two approval tests. Installed `sea-orm-migration@1.1.19` implements `Migrator::down(..., Some(1))` by reversing the applied migration list and taking one entry. It therefore rolls back tickets, leaving approvals intact. The roundtrip test retains its scope row and fails the empty-table assertion; the atomicity test fails immediately at `CREATE TABLE ops_agent_rule`, before reaching the intended migration failure.

   Required fix: select the approvals migration by its stable name or use a test migrator whose target is explicit. Exercise its up/down and failure behavior without assuming its position in the application registry or removing unrelated schema. Verify both tests with the tickets migration registered afterward.

   Evidence: independently executed the exact approval and ticket migration SQL in an in-memory SQLite database, applying the rollback selected by SeaORM's documented local implementation. The first scenario retained one scope row; the second returned `table ops_agent_rule already exists`. This was a SQL reproduction of the integration condition, not a Rust test run or an integrated checkout. The 19 passing tests at the isolated reviewed head do not exercise this condition.

Borrowed-source verification:

All remote reads used `gh api` with immutable commit references. No latest-source substitution was made.

| Exact source | Port inspected | Independent verification |
| --- | --- | --- |
| `IntroInnovation/intromail@0bd24dfe284b888aa9f602fa1fd00e337ea38874`, `backend/app/services/agent/gating.py` | `ops_approvals/gating.rs` | Read complete source; checked scope fallback, rule specificity, deny/ask precedence, action permission and destructive-action floor. |
| Same commit, `backend/app/services/agent/redaction.py` | `ops_approvals/redaction.rs` | Read complete source; checked recursive credential-key matching and declared private-field redaction. |
| Same commit, `backend/app/services/agent/proposals.py` | `ops_approvals/mod.rs` | Read complete source; checked proposal lifecycle, edited payload handling and terminal private-data scrubbing. Rust task CAS and post-commit authorization are the documented integration adaptation; this port does not execute a transport. |
| Same commit, `backend/app/services/agent/actions.py` | `Action`, `ActionContext`, `Decision` | Read the corresponding source interfaces. |
| Same commit, `backend/app/models.py` | Four `ops_*` entities and `m20260907_000001_ops_approvals.rs` | Read `Proposal`, `AuditLog`, `AgentRule`, `AgentScope`; checked the retained schema, enum constraints and added task/run binding. |
| Same commit, `backend/app/audit.py` | Transactional `audit_record` | Read complete source; verified credential scrubbing and the port's stronger failure behavior: audit failure aborts authorization. |
| `xintaofei/codeg@6f6bd648b206412644842a98d9ffeebf57292bed`, `src-tauri/src/db/service/work_task_service.rs`, `LICENSE` | Existing task CAS patterns and root license | Retrieved both exact files via API; each is byte-identical to the review baseline. Read the Apache-2.0 license. |

The IntroMail commit API resolved to the requested SHA, tree `2a803fd43d6322c5a64d3cede78e33f924401e1c`. Repository metadata reports `private: true`, `license: null`; its recursive tree contains no LICENSE/LICENCE/NOTICE/COPYING file. Accordingly, the authorization basis is the owner's permission recorded in FOUNDING section 3, not a verified MIT license. `NOTICE` names the exact six IntroMail paths and SHA and identifies that authorization. The Codeg commit resolves to tree `06d0da02a774af27fa4d1cba5f15debcc82c62e4`; its retained `LICENSE` is unchanged. No prohibited source import was identified in this diff. I cannot independently establish private repository ownership beyond the owner's instruction and repository metadata.

Other correctness checks and limits:

- Reviewed the first-write task CAS, run generation/status predicates, live task/folder checks, pending-proposal uniqueness, original-payload snapshot comparison, edited-payload policy recheck and transactional proposal/audit resolution. An authorization value is returned only after commit. These address concurrent reviewers and stale generations; no additional blocking defect was identified in those paths by inspection. Runtime concurrency was not independently stress-tested.
- Reviewed credential rejection/redaction, terminal private-payload scrubbing and audit payload digests. Pending proposals intentionally retain private content for review. The module relies on a trusted action registry and authenticated actor supplied by its caller; ownership decisions belong to the trusted action callback. There are no new external handlers here with which to verify that future authentication boundary. Expiry/retention and transport execution remain outside this bounded review.
- Independently verified transactional DDL rollback on a collision partway through creation: the pre-existing `ops_agent_rule` remained and the partially created proposal/audit objects did not. Also checked up/down preservation of an existing task, NULL-resource scope uniqueness, invalid scope mode rejection, single pending proposal per task/run, and rejection of ordinary audit UPDATE/DELETE. SeaORM writes migration history after `up` returns; crash recovery across that boundary was not fault-tested.

Commands and evidence:

| Reviewer action | Result |
| --- | --- |
| `git show a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba:reports/approvals.md`, complete diff and targeted immutable source reads | Exit 0; read directly rather than relying on the worker's summary. |
| `git diff --check 2f74992e6b133a3ae3ffe3c1e4d0c14444b29137 a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba` | Exit 0. |
| `git diff --exit-code` between those commits for `Cargo.lock`, `pnpm-lock.yaml`, `LICENSE`, FOUNDING, ORCHESTRATOR, STATUS and DECISIONS | Exit 0; unchanged. |
| `gh api repos/IntroInnovation/intromail/commits/<pinned-sha>`, exact-ref contents and recursive tree; corresponding Codeg commit/contents requests | Exit 0; provenance and license evidence above. Existing client credentials were reused without exposing secrets. |
| Offline code-context `guide 'Review Rust approval authorization ownership concurrency CAS audit redaction SQLite' --project ops-desk`, using `/Users/mohamedadan/projects/rag-skills/.venv/bin/python` and `HF_HUB_OFFLINE=1` | Exit 0. Returned general SQLite persistence guidance, no task-specific approval coverage. No API claims were inferred from missing corpus coverage. |
| Local pinned docs/source reads | Read `sea-orm-migration@1.1.19` migrator implementation and `sea-orm@1.1.19` transaction implementation before deriving migration/CAS behavior; checked pinned dependencies in the target lockfile. |
| `python3 -B` in-memory SQLite checks extracting actual migration SQL with `git show` and a read of the tickets migration | Exit 0, SQLite `3.53.4`. All constraint/atomicity assertions passed; both integration-test failure conditions reproduced as described. No database or build artifact was written. This SQLite build is not the application's Rust runtime. |
| Docs-first hook audit read | Live `PreToolUse` records at lines 501–504 and `PostToolUse` records at 505–508 of `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`, session `01a07c1c-d82f-7022-84db-778a438632f1`, cwd `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`. Hooks remained enabled. |

I did not rerun Cargo checks/tests or TypeScript checks during this review. The owner reports that the orchestrator independently reran all 19 approval tests successfully at the reviewed head; that is attributed evidence, not a result from this reviewer. The worker report contains its other validation results. No main changes were pulled, no integration was performed, and no Step 2 REST/GitHub App behavior was assessed. The sole review deliverable is this report; no commit, push, merge, deployment or message to another person was made.
