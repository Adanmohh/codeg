# Existing worker restart

Owner explicitly requested restarting the workers after docs-first removal.
Root verified hooks.json, installed launcher and engine are absent and parsed
config has zero hook trust entries. No hook was installed, restored or modified.
The owner's replacement AGENTS instruction forbidding restoration was relayed.

All three idle Codex processes were restarted in their existing Herdr panes,
using the same saved sessions and worktrees:

| Worker | Pane / name | Resumed session |
| --- | --- | --- |
| Rebrand | wR:p2 / ops-rebrand | 01a084ee-12a8-7833-ace9-f3f4985ba926 |
| Approvals | wR:p3 / ops-approvals | 01a084ee-129a-7561-9f10-64266115e759 |
| Tickets | wR:p4 / ops-tickets | 01a07c1c-d82f-7022-84db-778a438632f1 |

Each launch explicitly uses gpt-6-astra, model_reasoning_effort=max,
approval_policy=never and sandbox_mode=danger-full-access. Actual visible footers
show Astra/max. Herdr confirmed interactive readiness, then working state after
the assignments were delivered. No new worker, pane, tab or worktree was created.
Root then read actual resumed command output in all three panes: normal Git and
gh api commands pass exit0. Rebrand is applying its preserved type fixes;
approvals is publishing the staged3773 findings; tickets is preparing focused
regression tests. Tickets' initial abbreviated commit endpoint returned404, then
the full immutable SHA succeeded; this was not a hook or authentication failure.

Root snapshotted each HEAD/status and hashed all seven changed files before
restart, then verified all remained identical before resuming work. Preserved
heads are rebrand b7b3b2bb, approvals f4b12e8d and tickets54daac22. Approvals' three
staged report files and tickets' four product changes remain intact.

Only the exact idle Codex PIDs30498/30499/78542 received SIGKILL, after verifying
their shell parents. No process group, pane or fixture was terminated. This
avoids graceful shutdown cleanup of child fixtures. The seven checked fixture/
preview PIDs29883/63050/66200/81173/81950/85232/99613 remain alive; affected orphaned
children were reparented to launchd. No browser interaction or fixture restart ran.

Assignments resumed: rebrand normal E1 parser/client fixes and workspace UI;
approvals saved source-finding publication and independent correction review;
tickets the three bounded corrections/regressions and server check in the single
existing-target build window. Tickets must check10GiB free before compiling and
stop/report near5GiB; observed volume free space was18GiB. Approvals compilation
waits for release. All earlier fixture and no-live-provider boundaries remain.
