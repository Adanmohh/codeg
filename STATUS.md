# Status — 2026-09-07

Step 0 complete. Step 1 dispatched; three Codex workers are implementing in parallel.

- Fork: https://github.com/Adanmohh/codeg
- Project root: `/Users/mohamedadan/projects/ops-desk`
- Branch: `main`, based on `v0.30.4` (`6f6bd648b206412644842a98d9ffeebf57292bed`).
- Original fork tip preserved as `archive/upstream-main-at-fork`.
- FOUNDING.md and ORCHESTRATOR.md preserved; versions and initialization decisions recorded in [DECISIONS.md](DECISIONS.md).
- Validation: frozen frontend dependency installation, default desktop `cargo check --locked`, frontend production build (32 static pages), and standalone `tsc --noEmit` all exited 0. Evidence and known upstream warnings: [reports/step0.md](reports/step0.md).
- Product source and both dependency lockfiles match the upstream tag. Apache LICENSE retained. No AGPL source imported.
- Herdr environment verified and CLI syntax read. No workers started for Step 0.

## Step 1 — dispatched 2026-09-07

All three workers are running through Herdr, each in its own worktree and branch from `2f74992e`. Each topic has a separate tab in the Ops Desk workspace (`wR`), with its worker pane inside. Orchestrator: `wR:t1`, pane `wR:p1`.

| Worker | Tab / pane | Branch | Worktree | Report | Review / merge |
| --- | --- | --- | --- | --- | --- |
| rebrand | `wR:t2` / `wR:p2` | `feat/step1-rebrand` | `/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand` | `reports/rebrand.md` in worktree | Pending |
| approvals | `wR:t3` / `wR:p3` | `feat/step1-approvals` | `/Users/mohamedadan/projects/_worktrees/ops-desk/approvals` | `reports/approvals.md` in worktree | Pending |
| tickets | `wR:t4` / `wR:p4` | `feat/step1-tickets` | `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets` | `reports/tickets.md` in worktree | Pending |

- Verified all four active project agents use `gpt-6-astra`, the most capable model in the installed Codex model catalogue. Owner requires the best current models for all project workers; no cheaper-model delegation.
- Worker launch arguments explicitly set `-a never -s danger-full-access`.
- Every prompt requires complete founding/orchestration document reads, docs-first pinned-source reads, exact-source borrowing, licence/NOTICE attribution, no AGPL or enterprise source, early commits/pushes, draft PRs to main, and committed file reports.
- Review gates: default desktop `cargo check --locked`, frontend typecheck, focused behavioral tests and server compilation for Rust changes; inspect source citations and licence provenance before accepting.
- The orchestrator writes no product code. Update this file after each review and merge. Step 2 has not started.

- Owner docs-research update relayed to every worker: use `gh api` for latest documentation research and record exact refs; retain mandated borrowed-source pins.

- Docs-first hook audit found Claude-only registration. Added and trusted a scoped Codex adapter; five smoke tests passed. All three workers pushed checkpoints and resumed; live PreToolUse and PostToolUse records verified for each session. Details: [reports/docs-first-hook.md](reports/docs-first-hook.md).

- All three workers resumed on explicit `gpt-6-astra` with `model_reasoning_effort=max`, keeping `never` approval and full access. Product implementation and acceptance checks are still in progress; no worker branch merged yet.
