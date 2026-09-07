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
- Live activation pending worker checkpoint/restart. Existing sessions predate registration. The shared daemon proxy socket is absent, so hot reload through that route was unavailable; preserve/push checkpoints and resume sessions to load hooks.

The inherited gate is heuristic: strict framework-import checks, unfamiliar-language blocking, and reminders. It does not prove every API use is correct; source review and tests remain acceptance gates.
