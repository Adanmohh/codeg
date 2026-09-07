# Status — 2026-09-07

Step 0 and Step 1 complete: all three foundation PRs are reviewed and merged. Step 2 transport, integration and UI work continues autonomously. Browser baseline passes; new Ops flow checks and final Design Studio loops remain.

- Fork: https://github.com/Adanmohh/codeg
- Project root: `/Users/mohamedadan/projects/ops-desk`
- Branch: `main`, based on `v0.30.4` (`6f6bd648b206412644842a98d9ffeebf57292bed`).
- Original fork tip preserved as `archive/upstream-main-at-fork`.
- FOUNDING.md and ORCHESTRATOR.md preserved; versions and initialization decisions recorded in [DECISIONS.md](DECISIONS.md).
- Validation: frozen frontend dependency installation, default desktop `cargo check --locked`, frontend production build (32 static pages), and standalone `tsc --noEmit` all exited 0. Evidence and known upstream warnings: [reports/step0.md](reports/step0.md).
- Step 0 product source and lockfiles matched the upstream tag. Rebrand is now merged; both dependency lockfiles and Apache LICENSE remain unchanged. No AGPL source imported.
- Herdr environment verified and CLI syntax read. No workers started for Step 0.

## Step 1 — dispatched 2026-09-07

All three workers were dispatched through Herdr, each in its own worktree and branch from `2f74992e`. Each topic has a separate tab in the Ops Desk workspace (`wR`), with its worker pane inside. Orchestrator: `wR:t1`, pane `wR:p1`.

| Worker | Tab / pane | Branch | Worktree | Report | Review / merge |
| --- | --- | --- | --- | --- | --- |
| rebrand | `wR:t2` / `wR:p2` | `feat/step1-rebrand` | `/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand` | `reports/rebrand.md` | Reviewed and merged, PR #2 |
| approvals | `wR:t3` / `wR:p3` | `feat/step1-approvals` | `/Users/mohamedadan/projects/_worktrees/ops-desk/approvals` | `reports/approvals.md` in worktree | Reviewed and merged, PR #3 |
| tickets | `wR:t4` / `wR:p4` | `feat/step1-tickets` | `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets` | `reports/tickets.md` in worktree | Reviewed and merged, PR #1 |

- Verified all four active project agents use `gpt-6-astra`, the most capable model in the installed Codex model catalogue. Owner requires the best current models for all project workers; no cheaper-model delegation.
- Worker launch arguments explicitly set `-a never -s danger-full-access`.
- Every prompt requires complete founding/orchestration document reads, docs-first pinned-source reads, exact-source borrowing, licence/NOTICE attribution, no AGPL or enterprise source, early commits/pushes, draft PRs to main, and committed file reports.
- Review gates: default desktop `cargo check --locked`, frontend typecheck, focused behavioral tests and server compilation for Rust changes; inspect source citations and licence provenance before accepting.
- The orchestrator writes no product code. Update this file after each review and merge. Step 2 is now active (see latest dispatch below).

- Owner docs-research update relayed to every worker: use `gh api` for latest documentation research and record exact refs; retain mandated borrowed-source pins.

- Docs-first hook audit found Claude-only registration. Added and trusted a scoped Codex adapter; five smoke tests passed. All three workers pushed checkpoints and resumed; live PreToolUse and PostToolUse records verified for each session. Details: [reports/docs-first-hook.md](reports/docs-first-hook.md).

- All three workers resumed on explicit `gpt-6-astra` with `model_reasoning_effort=max`, keeping `never` approval and full access. All implementation handoffs are complete; acceptance state is recorded below.

## Owner integration amendment — 2026-09-07

Step 2 now borrows direct Resend REST email transport and GitHub App integration from intromail. No Resend CLI/MCP or GitHub MCP product integration. Exact sources and missing polling/create-issue glue documented in FOUNDING.md and DECISIONS.md. Step 1 scope remains branding, approvals/audit and ticket store/threading.

## Acceptance review — rebrand merged, 2026-09-07

- Rebrand [PR #2](https://github.com/Adanmohh/codeg/pull/2) reviewed at `9d036d47ab7a7cebecfac78204aa915eb5ffe8ba` and merged as `079fbf64bcfd8116e83499b5b2e818f95ab84535`.
- Orchestrator inspected the report, attribution, update-disable boundaries, display changes and mobile screenshot. Independently reran desktop/server `cargo check --locked`, frontend typecheck and four focused frontend suites: 71 tests passed. No blocking findings. Worker build/lint/native test evidence remains in [reports/rebrand.md](reports/rebrand.md).
- Existing sidecar-placeholder and Rust future-compatibility warnings remain; no native packaged-app validation or CI-pass claim.
- Approvals worker is independently reviewing tickets; tickets worker is independently reviewing approvals. Each must deliver a review file with findings and exact reviewed commit. Neither implementation is accepted or merged yet. Shared NOTICE/registry integration and combined checks remain.

## Remaining Step 1 acceptance

- Tickets [PR #1](https://github.com/Adanmohh/codeg/pull/1), head `c2dc3be58b0e98999360fa23565aac8a3fe25fb6`: worker reports desktop/server compile, typecheck, Clippy and 15 focused tests passing. Independent reviewer: approvals worker; deliverable `reports/review-tickets.md` in approvals worktree.
- Approvals [PR #3](https://github.com/Adanmohh/codeg/pull/3), head `a4b418837d0b28e5ead6e57de1a8dbddc96ec1ba`: worker reports desktop/server compile, typecheck, Clippy, 19 approval tests and 39 work-task tests passing. Independent reviewer: tickets worker; deliverable `reports/review-approvals.md` in tickets worktree.
- Both PRs remain draft/unmerged. Rebrand introduced a shared NOTICE conflict; preserve all attribution when integrating. Review findings must be fixed by the owning worker, then checked before merge. GitHub has no attached status checks; worker local passes are not CI passes.
- Step 1 is the foundation, not the complete support/bug workflow. Step 2 supplies email transport/UI, Hafidh intake, evidence-bound GitHub App issue filing, pi integration and the morning view. Step 3 supplies Telegram and end-to-end P1/P2 validation. No real email send, GitHub App installation, deployment or packaged app has been performed.

## Backend review findings — changes requested, 2026-09-07

- Independent [approvals review](reports/review-approvals.md): two P2 findings. Resolving an Ops proposal can clear the displayed awaiting state while an overlapping ACP permission request remains unanswered; migration tests incorrectly roll back the last migration instead of targeting approvals. Both sent to the approvals owner for fixes and regression tests. No ACP permission bypass was claimed.
- Independent [tickets review](reports/review-tickets.md): one P2 finding. A reply from another unblocked participant can reopen a conversation whose primary contact is blocked. Sent to the tickets owner to restore the pinned Chatwoot conversation-contact guard and add resolved/snoozed/control regressions.
- Orchestrator independently reran the existing ticket suite (15/15) and approval suite (19/19): passing, but these uncovered cases were missing. Passing existing tests does not waive these findings.
- Both workers are authorized to integrate accepted main/rebrand into their own branches, preserving all attribution, and rerun checks before pushing updated reports/PRs. Orchestrator writes no product fixes. Neither backend PR is accepted or merged.

## Tickets accepted and merged — 2026-09-07

- PR #1 accepted at `f03c11e030d2bd5d972c0867f0990bf797d184a4`, merged as `d5a7127187cd107d2bdd6d00e7c7900e542b3903`. The fix checks the conversation primary contact in the scoped transaction; source attribution includes the pinned Chatwoot mute concern.
- Orchestrator reviewed the fix, tests, NOTICE and final report; re-read the exact upstream concern and message reopening code through gh api; independently reran all 18 ticket tests successfully. Worker reports both runtime checks/test suites, both Clippy gates and typecheck passing.
- Approval findings remain open. Its worker must integrate the accepted tickets migration and pass combined checks before acceptance.

## Autonomous continuation — owner instruction, 2026-09-07

- Continue through the planned Phase 1 work without stopping at status milestones. Retain orchestrator-only product authorship rules and three Herdr workers maximum, each on its own branch/worktree.
- Use Playwright CLI for actual browser checks during integration. Finish with Design Studio measured audits, worker fixes and repeated checks on affected flows; record evidence and remaining limitations honestly.
- Rebrand worker reassigned to a report-only Step 2 GitHub App/Hafidh intake contract task on a new docs branch in its own worktree. Resend transport work is next in the tickets worker worktree. No live sends or external publication are authorized by test automation alone.

## Active Step 2 dispatch and browser preparation

- `wR:t2` / `wR:p2`, tab GitHub · Hafidh intake, worker `rebrand`: report-only source contracts on `docs/step2-intake-contracts` in its original rebrand worktree. Deliverable `reports/step2-intake-contracts.md`.
- `wR:t4` / `wR:p4`, tab Email · tickets, worker `tickets`: direct Resend internal transport on `feat/step2-email-transport`, original tickets worktree. Deliverable `reports/email-transport.md`; no public send routes or live sends in this task.
- `wR:t3` / `wR:p3`, tab Approvals · audit: finishes the two Step 1 findings and combined migration/engine checks.
- All tasks retain exact borrowing pins, gh api research, docs-first hooks, GPT-6 Astra max and isolated worktrees.
- Playwright CLI and Design Studio skills read. Production frontend build for merged rebrand/tickets passes. Real local server build and isolated browser check are in preparation. [Design brief](docs/design/BRIEF.html) generated through Design Studio after narrowing and curating the scanner output; its scenarios are acceptance requirements, not completed flow results.

## Approvals accepted and merged — 2026-09-07

- PR #3 accepted at `85e5f665c59ce040cb412fa95de36c396d9a1ff9`, merged as `65aca88916b4af398a568c09ece431440d660e3b`. Both review findings are fixed: transactional ACP/Ops wait ownership preserves concurrent requests, and migration tests target approvals by name with later migrations present.
- Orchestrator reviewed the fix and concurrency/cleanup regressions, verified the final commit changes only the report, and independently reran all 21 approval tests successfully. Worker combined evidence: 18 ticket, 39 work-task and 128 engine tests; desktop/server checks, both Clippy gates and frontend typecheck pass. See [approvals report](reports/approvals.md). No CI or packaged-app pass claimed.
- All Step 1 PRs are merged. These are foundation services; approval routes, trusted action execution and Ops UI are Step 2 work. The isolated browser server will be upgraded on its existing ticket database to verify migration compatibility.

## Step 2 UI dispatched — 2026-09-07

- Reused approvals worker/pane on new `feat/step2-ops-ui` from accepted main for shared Ops API, real ticket/thread/private-note/draft and approval screens, plus the morning list. Deliverable `reports/ops-ui.md`; actual Playwright CLI checks required in its own isolated instance.
- Email worker owns direct Resend transport; GitHub/Hafidh worker owns intake contracts and subsequent implementation. Product writes remain worker-only. Human review and exact-payload dispatch are required at the UI/API seam; unavailable external configuration must be shown honestly.

- Combined-foundation browser server upgrade passed: existing ticket-only test database applied approvals, retained both migration records and ten Ops tables; Playwright CLI reload recovered at desktop width. Evidence: [browser report](reports/browser-step1.md).
