# Business integration regressions — 2026-09-08

**BW-17 passes at accepted main `294fb634b1833ebb13c4223484624e595598235b`: 312 tests passed, zero failed, five manual browser fixtures ignored, exit 0.** No product correction was needed. Local session metadata supports Astra/max throughout this worker's PR22 implementation and handoff; the later Luna/low turn made zero tool calls. This is a bounded regression and provenance review, not overall business UI/native acceptance.

Owner/worktree: tickets, `/Users/mohamedadan/projects/_worktrees/ops-desk/tickets`. Review branch: `review/business-integration-regressions`. PR22 accepted head: `c3af44948b78eb31a26cb0b5848a3e25922b0418`; merge: `5541857a125678dfb604f542693d9c82dfe5441e`. Independent R1 review remains at `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5`, published in reviewer commit `a8d1dbbebf5cfef57fb489f5417f303fa8dd422f`. Its 13 passes are the reviewer's evidence, separate from the new run below.

## Exact handoff and integration audit

Read the full Rust/NOTICE/contract diff and classified every changed path between `1ba73e3c` and `c3af4494`: **12 files, 655 additions, 33 deletions**.

| Changed area | Verified effect |
| --- | --- |
| `src-tauri/src/acp/pi_desk.rs` | Only `#[cfg(test)] mod tests`, lines 314 onward: the ignored extracted-assets fixture polls actual registration of seven required tools within ten seconds. It retains successful RPC, zero model-message/provider-connection and model-guard assertions. No production launcher/bridge change. |
| `NOTICE` | Corrects full IntroMail source paths and adds attribution for existing browser orchestration and the adapter lifecycle reference. Existing licence sections remain. |
| `docs/contracts/business-tasks.md`, `reports/business-tasks.md` | Final source, validation, provenance and limitation documentation. |
| Eight files under `reports/business-tasks-evidence/` | README, three standalone browser evidence scripts, their three sanitized results, and hook metadata. No product module registration. |

The production prefix of `pi_desk.rs` (lines 1–210, before the test module) hashes identically at both heads: SHA256 `1407c6f4d8d4d182c3d0d3bba255c9c3ac0264ebaa53f7229860fbd9c7e16d3f`. Every other production file is unchanged. Thus the independently reviewed source-entrustment, original DelegationGrant lineage, stable Pi policy identity and human review floor are preserved.

Preserved the clean tracked handoff, fetched main, created the review branch, then `git merge --ff-only origin/main` exited 0. Only root's `STATUS.md` updates and `reports/review-business-tasks.md` were added by that integration. `git diff --quiet c3af4494..294fb634 -- NOTICE LICENSE src-tauri integrations src pnpm-lock.yaml` exited 0: runtime, tests, registrations, licence and locks match the accepted handoff. No root planning document or other worktree was edited.

Read the merged auth registration again: `web/router.rs:1667–1672` keeps Ops/intake/Telegram under the original `auth::require_token`; `business_identity/http.rs:168–187` applies its separate member middleware only to `/business`, including `/tasks`. Operator recognition still requires the protected transport token. Existing identity authorization evidence, including `full_router_requires_original_operator_bootstrap_and_keeps_member_bearers_out_of_legacy`, remains attributed to the accepted 13-test identity/business run; that unchanged suite was not repeated.

## Model/session metadata audit

Inspected only allowlisted metadata from the existing rollout: timestamps, record type, session/turn ID, cwd, CLI version, model, effort, approval/sandbox settings, tool name and record counts. No messages, reasoning, arguments, tool results, credentials, global configuration or secret payloads were printed or copied.

Session `01a07c1c-d82f-7022-84db-778a438632f1`, own worktree, CLI **0.153.4**, no fork parent. Local source authority: accepted `src-tauri/src/parsers/codex.rs:2660–2713`; installed `codex-package.json` also reports 0.153.4. Official source resolved through `gh api`: `openai/codex` tag `rust-v0.153.4` → commit **`3d2ee51ca2d5db578f328aa75e20aa22c0197c9a`**; `codex-rs/protocol/src/protocol.rs`, blob `ae2107b801b7115654b7a076b060a8686a3b3e68`, defines SessionMeta and TurnContextItem's model/effort fields. [Immutable source](https://github.com/openai/codex/blob/3d2ee51ca2d5db578f328aa75e20aa22c0197c9a/codex-rs/protocol/src/protocol.rs#L3194).

| UTC timestamp, 2026-09-08 | Recorded model / effort |
| --- | --- |
| 07:46:37.303; 08:12:45.467; 08:59:31.325; 09:41:56.444 | `gpt-6-astra` / `max`, same business implementation turn `01a07ffb-b46c-7821-8bbb-87ed54efae6d` |
| 12:22:55.953 | `gpt-5.6-luna` / `low`, turn `01a080f8-acb9-7f13-8c2b-88caa78d0fd8` |
| 15:30:56.205 | `gpt-6-astra` / `max`, current regression turn `01a081a4-cc3a-7c02-91c0-a8e1c43da156` |

All listed contexts record approval `never` and sandbox `danger-full-access`. Git's nine first-parent PR22 checkpoints fall inside the Astra/max window:

| Commit | Commit time, UTC | Scope |
| --- | --- | --- |
| `cb2e184fd14414ccbd646c90d7732493dcd2b7e6` | 07:53:38 | Initial contract |
| `bf4309f5abdb077fd8e1a8e42db4861dadcda24b` | 08:24:13 | Core/schema |
| `6ccceac6fa93c8674c6cb4807277b957b63dc575` | 08:24:54 | Published identity integration |
| `76bb6909511016a11e864abe19d4ffd493aafa0c` | 08:30:07 | Transports/tests |
| `1e8b525076efe1aa3c4563c954d52c0affd5d3c7` | 08:49:32 | Core/agent wiring |
| `94a643ca75a49fbbce13b5d9be4c734e9067041e` | 08:49:56 | Accepted main integration |
| `f83bf6c82731230adbb1aa5c6c13be4fe4fd795a` | 09:02:27 | Generation fencing |
| `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5` | 09:13:11 | Reviewed R1 ownership fix |
| `c3af44948b78eb31a26cb0b5848a3e25922b0418` | 09:37:29 | Fixture/docs handoff |

Metadata-only aggregation found **164 `exec` calls** during 07:46:37–09:42:00, all in the recorded Astra/max window. From the Luna context until restored Astra, it found **zero function calls, zero custom tool calls, one user message and one assistant message**. Therefore the observed Luna turn did not perform filesystem/tool work and occurred after the product and handoff commits. Current Herdr metadata also reports Astra. No model selection/global configuration was changed during this audit.

Limits: these are local configuration/timeline records, not signed provider attestations or a line-by-line proof of model authorship. They do not establish why the idle model changed, or certify other workers' imported source. The full session starts with Astra/**medium** at 2026-09-07 13:45:35, then Astra/max at 13:54:49; this report does **not** claim every historical project turn used max. PR22's implementation window is unambiguous in the inspected records.

## BW-17 execution and coverage

After notifying root of the exact source and own output plan, ran the existing server-mode library selectors once:

```text
cargo test --locked --no-default-features --lib -- ops:: ops_intake:: ops_intake_host:: ops_telegram:: ops_approvals:: email_transport:: ticket_service:: work_task_service:: work_task::engine::tests::
```

**Exit 0; 312 passed, 0 failed, 5 ignored, 3,387 filtered out; 19.33s test runtime, 1.33s Cargo preparation.** Own existing `src-tauri/target`; exact compiled test `codeg_lib-b542e2757758a7bb`.

| Existing coverage | Passed |
| --- | ---: |
| Approval policy/audit/CAS and destructive floor | 21 |
| Ticket threading, scope, persistence and blocked-primary-contact reopening | 18 |
| Resend normalization, Reply-To, recipient/header safety, pagination and duplicate delivery | 18 |
| Ops thread/draft/review/agent and protected-router delivery | 22 |
| GitHub intake/client evidence, exact payload, unknown outcomes and reconciliation | 22 |
| Intake host cached freshness, proposal/review, authenticated HTTP and fix-task binding | 17 |
| Telegram email/issue decisions, opaque links, no-resend and migrations | 24 |
| Engineering work-task service | 39 |
| Engineering engine lifecycle, locks, cancellation and overlapping ACP/Ops waits | 131 |

Read the actual loopback/mock helpers before execution. The seven email integration tests include exact edited approval once, persisted unknown outcome after restart/draft edit, receipt-only recovery without network, serialized approval/pull, payload mismatch and cancellation-before-dispatch. Intake tests assert one issue POST for concurrent approval and lost-response reconciliation, no automatic retry on ambiguous outcomes, and zero provider calls when evidence/configuration is invalid. Engineering tests use isolated engine instances/fake Forge delivery; they do not start the production scheduler or a model.

Five manual browser fixtures remained ignored (email, design Ops, Telegram email, intake host, Telegram issue). No browser/listener/export was started, restarted or stopped. Existing passing business/Desk/Pi process, desktop/server checks, Clippy, frontend and 55 browser API assertions were not redundantly rerun; see the accepted [task handoff](business-tasks.md) for those exact heads and limits.

Full local test log: `.docs/checkpoints/business-tasks-20260908/bw17-rust.log`, SHA256 `48ae9805f88c9961ca2b08ffae5f744ab1f81cab9a6ded96a5d391a796c704f1`. Existing linker unwind-size and proc-macro-error2 future-compatibility warnings remain; neither is a test failure. No failure/retry or assertion change occurred in this run.

## Docs-first and preservation

First resumed reads were separate `cat node_modules/react/package.json` and `cat src-tauri/Cargo.toml`; React19.2.4 and installed Cargo/libtest help were read. Read complete project instructions/workorder and BW-17 checklist, then local test/auth source. Applied code-context offline with the existing rag-skills .venv: guide exit0 returned general, mostly other-project rules; docs exit3 reports missing `tickets.db` coverage. No installation or invented corpus coverage. Official remote metadata schema research used immutable `gh api` only. No third-party implementation was copied; existing NOTICE/LICENSE stay byte-identical to accepted main.

Live own-session docs-first audit records include PreToolUse `1788881601` and PostToolUse `1788881657`, both exit0, tool Bash, correct session/worktree. Hooks stayed enabled. Read-only discovery also found no `archived_sessions` directory; the exact existing rollout above was used. Herdr group help conventionally exits2; commands, source reads, metadata extraction, merge and test run exited0 except the disclosed missing-corpus query.

Before/after own real companion SHA256 remains `c1a6ba22c959c5b0ed1bae8e61980c3dd043f9e6fd74641e7626ad678ea4f7f5`; no default-feature build replaced it. Paused untracked `reports/visual-correspondence.md` remains untouched, SHA256 `a2120d9cc87dbb4823b1223e02d377660fff370d846f2fd102fe1eb36599a50d`. Root was notified of results and unchanged outputs. No dependency, lockfile, production source, fixture ownership or external service state changed. Business UI integration/final Design Studio and native artifact acceptance remain root's separate gates.
