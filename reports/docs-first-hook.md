# Docs-first hook verification — 2026-09-07

Initial check: Codex `hooks/list` returned no hooks for Ops Desk. The existing hook was registered only in `~/.claude/settings.json`; worker prompt instructions were present but a Codex hook was not running.

## Repair

- Reused existing `~/.claude/hooks/docs-reminder.py` and `docs-seen.py` via `~/.codex/hooks/ops-docs-first.py`.
- Registered PreToolUse and PostToolUse in `~/.codex/hooks.json`. The adapter is restricted to the Ops Desk root and its worker worktrees.
- Normalizes Codex apply_patch file additions/updates to the existing Write checks. Bash already uses the existing hook payload shape.
- Added Ops Desk root/worktree prefixes to the existing strict-projects list.
- Persisted trust for the two reviewed command definitions through Codex's config/batchWrite API; no hook-trust bypass flag.
- Logs only event/session/tool/cwd/exit/context-emitted metadata in `~/.codex/hooks/ops-docs-first-audit.jsonl`, excluding arguments, source and output.

## Grounding

All remote hook research used `gh api` on openai/codex at installed `rust-v0.153.4`, commit `3d2ee51ca2d5db578f328aa75e20aa22c0197c9a`: app-server/README.md, config/src/hook_config.rs, core/src/tools/hook_names.rs, core/src/hook_runtime.rs, hooks/src/engine/discovery.rs, generated pre-tool-use input schema, core/tests/suite/hooks.rs, app-server/tests/suite/v2/hooks_list.rs. Paths are under codex-rs/.

## Verification

- Codex hooks/list: both hooks enabled and trusted for the root and all three worktrees; no load warnings or errors.
- Synthetic payload smoke tests (no product writes): framework apply_patch blocked before documentation (exit 2); ordinary git read quiet (exit 0); same patch allowed with reminder after actual local React package metadata read (exit 0); another session remains blocked; unrelated cwd unaffected. All five passed.
- Live activation verified for all three resumed worker sessions (counts below). Shared daemon proxy socket was absent, so each worker pushed a checkpoint and resumed in its existing pane to load hooks.

The inherited gate is heuristic: strict framework-import checks, unfamiliar-language blocking, and reminders. It does not prove every API use is correct; source review and tests remain acceptance gates.

Live execution verified after checkpoint/push and resume on GPT-6 Astra with max reasoning:

| Worker | Session | PreToolUse | PostToolUse |
| --- | --- | --- | --- |
| rebrand | `01a07c1c-cf2e-73e1-bbe3-e758c8363042` | 9 | 9 |
| approvals | `01a07c1c-d3a3-7c22-a5b6-cedce2970d8d` | 6 | 6 |
| tickets | `01a07c1c-d82f-7022-84db-778a438632f1` | 12 | 10 |

Counts are a snapshot, not final totals. The current orchestrator session predates registration; its hook loading has not been verified. It writes no product code.

## Continued autonomous work — 2026-09-08

Root independently inspected fresh metadata-only audit pairs after PR7 merged
and Telegram dispatch. Rebrand lines 4538/4540, tickets 4533/4534, approvals
4513/4518 have exit 0 in their respective worktrees (timestamps 1788820484–
1788820485, 1788820409 and 1788820395). Session context had already been emitted,
so these records correctly have context_emitted=false. This confirms ongoing
worker hook execution, not universal write-proof enforcement or root-session
live enforcement. Existing smoke-test and scope limitations still apply.

## Continued independent audit —2026-09-08

Latest metadata inspected by root: rebrand PostToolUse lines6064/6065 at
1788823961, approvals Post/Pre lines6021/6022 at1788823901, tickets Post/Pre
6045/6055 at1788823936/1788823943. All exit0 and correct existing session/cwd;
context_emitted=false after prior reminders. Root session still predates hook
registration; this is worker observation, not root enforcement or universal
write interception. No secret values are included in the audit evidence.

## Design and integration checkpoint

Root inspected fresh metadata records: rebrand PostToolUse lines 6813/6814
(timestamp 1788825704), approvals PostToolUse 6825/6826 (1788825733), and
tickets apply_patch Pre/Post 6809/6810 (1788825696). All exit 0 in their
expected worktrees and sessions. The tickets source-patch pre-hook emitted
context; its post-hook did not repeat it. This verifies a live source-patch
hook event as well as ongoing worker tool hooks, within the heuristic limits
above. Root's older session remains manually grounded, not live-hook verified.

## Fresh root verification during P1 review

Read metadata-only audit rows after phone acceptance. Live context-emitting
PreToolUse records remain present for all three current worker sessions:
rebrand row8851/time1788829809 (Bash), approvals8478/time1788829124
(apply_patch), tickets8747/time1788829621 (apply_patch), each exit0 and exact
owned worktree. Later paired Pre/Post records also continue (9024/9025,
9044/9045,8994/8995 respectively). No command payload or secrets printed.
Root continues manual docs-first because this session predates hook activation;
these worker records are not a claim of live root interception.
