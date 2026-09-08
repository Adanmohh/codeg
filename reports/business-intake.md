# Business intake implementation

Owner: tickets, sole writer in `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`,
branch `feat/business-intake`, base `086eee485e6f40a8b02ea77c9dbeefe315c81596`.
Implementation is in progress; no B01–B18 or BI1–10 pass is claimed yet.

The accepted wire contract is `670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5`
(`docs/contracts/business-intake.md`, final report handoff `7f4d4dbc`), with canonical
access `18be55edc276713fc6d46d075baec363245ba285`. PR25 merged as `aa16a9b9`.
Independent review `2a765db2` found no open contract blockers. Those are contract
reviews, not implementation evidence.

## Work and integration contract

- Single `business_intake` module/access, sole migration
  `m20260908_000011_business_intake.rs`; existing Principal, writer and credential store.
- Fireflies protected setup → explicit grants → bounded read/import → exact human
  preparation → atomic shared task is first. Email/Hafidh protected projections
  and monotonic configuration fencing follow within this assignment.
- Rebrand owns the frontend. The accepted camelCase POST `{input:...}` DTOs and
  slash-to-underscore native command names remain authoritative. Binding capabilities
  do not imply connectivity, and task-domain access does not grant private sources.
- The first compiling checkpoint contains strict credential mutation reads and
  task-owned transaction extraction. Further checkpoints will publish concrete module
  and route types for UI wiring and immutable independent review.

## Provenance and docs first

Complete current FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS, implementation scope,
both intake contracts and existing identity/task contracts read before edits.
React19.2.4 package and Cargo manifest read in separate commands first. Applied
code-context using existing rag-skills `.venv/bin/python`, `HF_HUB_OFFLINE=1`:
guide exit0; docs exit3 because `data/code/tickets.db` is absent. No corpus-backed
API claim is made; local pinned source is the authority. Guide's reuse rule is
applied without subagents, per owner instruction.
Live hook metadata confirms PreToolUse and PostToolUse at1788892494, exit0,
session `01a07c1c-d82f-7022-84db-778a438632f1`, this worktree. Hooks remain enabled.

Existing Apache Codeg credential adapter at base086eee48, `keyring_store.rs`
blob `29fc3fb38280338aa26939c45f80ef9aefc2a394`, supplies the existing process-local
lock and private atomic rename path. The narrow correction refuses mutation on
malformed/unreadable storage. Native keyring and SQLite are still separate resources.
Task extraction retains the existing Apache/approved IntroMail task authorization
and attribution; no copyleft task code is imported. Full intended Fireflies source
ledger remains in `reports/business-intake-contract.md`; actual ports and NOTICE
will be recorded as implementation lands.

## Commands and limits

- Tracked clean check, fetch and new branch from accepted main: exit0. The paused
  untracked `reports/visual-correspondence.md` is preserved (SHA256
  `a2120d9cc87dbb4823b1223e02d377660fff370d846f2fd102fe1eb36599a50d`).
- Existing fixtures, browser sessions, exports, targets and bundled executables are
  preserved. New compilation uses an isolated target; no fixture is started yet.
- No provider/model/engine/email/issue action, dependency or configuration change.
- `CARGO_TARGET_DIR=$WORKTREE/.docs/business-intake-target cargo check --locked
  --offline --no-default-features --bin codeg-server`: exit0 (initial64s,
  updated helper source29.8s). Helper checkpoint has two temporary unused-function
  warnings until intake consumes prepare/link; no lint waiver added.
- Same isolated target, `cargo test --locked --offline --no-default-features --lib
  intake_`: exit0, **21 passed/1 manual ignored**, including four new strict-store
  and task-transaction regressions; execution2.35s. Existing host cases matched
  the selector. Logs in `.docs/business-intake-logs/`; linker unwind-size and
  proc-macro-error2 future-compatibility warnings are disclosed.
- New focused names: `intake_store_mutation_preserves_malformed_and_unreadable_stores`,
  `intake_store_missing_file_and_staged_cleanup_preserve_other_entries`,
  `intake_task_transaction_rolls_back_creation_and_preserves_explicit_editor_owner`,
  `intake_task_source_link_is_atomic_text_free_and_invalidates_review`.
- Planned validation: strict-store failures, current identity/grant/source fences,
  source refresh races, durable recovery, exact atomic decisions, all relevant existing
  business/Ops regressions, desktop/server/companion checks and Clippy. Guarded
  synthetic fixture/port coordination and actual Playwright CLI follow implementation.

Checkpoint SHA / draft PR: pending first compiling checkpoint.
