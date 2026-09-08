# Decisions

## 2026-09-07 — Step 0 baseline

- Read FOUNDING.md and ORCHESTRATOR.md completely. Step 0 runs with the orchestrator alone; later implementation uses Codex workers through Herdr, with one writer per worktree and reports returned as files. No product code was written in Step 0.
- Fork: https://github.com/Adanmohh/codeg. Upstream: https://github.com/xintaofei/codeg. Keep the inherited repository name until the phase-1 rebrand.
- Base: `v0.30.4`, commit `6f6bd648b206412644842a98d9ffeebf57292bed`. Create project `main` from this tag as instructed.
- The freshly forked upstream `main` was `b2eec98ce8d082ad48803918dd9a21ab08d1d3d4`, ahead of the tag. Preserve it remotely as `archive/upstream-main-at-fork` before publishing project `main`; use an explicit expected-SHA lease for that one initialization. Later changes use small PRs to `main`.
- Clone was staged inside this directory and moved into the root after checking for filename collisions. FOUNDING.md, ORCHESTRATOR.md and reports/ were preserved.
- Docs-first evidence: local tagged AGENTS.md, README.md, package.json, pnpm-lock.yaml, .npmrc, src-tauri/Cargo.toml, src-tauri/Cargo.lock, src-tauri/build.rs, src-tauri/tauri.conf.json, src-tauri/scripts/prepare-sidecars.mjs, next.config.ts, tsconfig.json, and .github/workflows/test.yml. CLI syntax was read from installed help before use. No library implementation was needed.
- Install with `pnpm install --frozen-lockfile`; check Rust with `cargo check --locked` in src-tauri/. Preserve both upstream lockfiles.
- Rust's first build reached the Tauri build script but lacked `out/`. Apply the ignored frontend placeholder documented in .github/workflows/test.yml while the real frontend export builds. The upstream build.rs also creates an ignored zero-byte MCP sidecar for checks. A packaged app requires the real sidecar via `pnpm tauri:prepare-sidecars`; Step 0 is a compile gate, not an app distribution.
- The sandbox denied GitHub access and tool-cache writes; use the approved escalations and existing credential clients. The sandboxed Turbopack build stalled at compilation and was interrupted; rerun the same `pnpm build` outside the sandbox.
- Borrowing remains governed by FOUNDING.md section 3: exact pinned source files, MIT/Apache attribution in NOTICE for ports; AGPL sources excluded. No new third-party source was imported in Step 0; the upstream Apache LICENSE is retained.

## Recorded local versions

| Component | Version |
| --- | --- |
| macOS / architecture | 26.5.1 (25F80), arm64 |
| Node.js | 24.19.0 |
| pnpm (packageManager pin) | 11.9.0 |
| Cargo | 1.98.0 (797e8a9bc, 2026-08-05) |
| rustc | 1.98.0 (88d9e12ae, 2026-08-18) |
| Git | 2.55.0 |
| GitHub CLI | 2.97.0 |
| Herdr | 0.8.2; HERDR_ENV=1 verified |
| Next.js / React / TypeScript | 16.1.6 / 19.2.4 / 5.8.3 |
| Tauri Rust / CLI | 2.10.2 / 2.10.0 |
| SeaORM | 1.1.19 |
| sacp / ACP schema | 11.0.0 / 0.11.7 |

Dependency versions above were read from the installed packages and Cargo.lock, not inferred from manifest ranges.

## 2026-09-07 — Step 1 dispatch

- Owner authorized three Codex workers via Herdr, command approval `never`, full filesystem access, isolated worktrees/branches, docs-first borrowing and report-file deliverables. All three launched from the pushed Step 0 baseline.
- One topic per tab in Ops Desk workspace wR, with one worker pane in each. Herdr worktree creation created top-level wrappers despite `--workspace wR`; worker panes were moved into wR tabs before agents started. Empty wrapper shells were left intact to avoid the documented close blast radius.
- Rebrand uses Hafidh Ops Desk as the product name, config/assets/minimal branding glue; preserve internal binary/module identifiers where renaming expands scope. No signing or distribution.
- Reserve distinct migration names: approvals `m20260907_000001_ops_approvals`, tickets `m20260907_000002_ops_tickets`, adapted to verified repository conventions. Workers keep shared registry and NOTICE edits minimal.
- Intromail has no source tag specified: approvals must resolve and record one immutable commit before reading the named private source files via gh api. Chatwoot remains pinned to v4.17.1, MIT files outside enterprise only.
- Owner requires best/latest models throughout. All project agents verified as GPT-6 Astra, the installed catalogue flagship; official model guidance confirms it is OpenAI's most capable model: https://developers.openai.com/api/docs/models/gpt-6-astra. This is worker-runtime selection; FOUNDING's product agent choices remain in force.

- Owner clarification: always use `gh api` for researching latest docs, against official source repositories and recorded refs. Local pinned dependency source remains the implementation authority; newest docs do not authorize changing FOUNDING source pins. This rule was relayed to all three workers.

- Docs-first repair: Codex initially had no hooks. Added a local Ops Desk-scoped adapter over the existing Claude hook scripts, normalized apply_patch to existing write checks, persisted trust through the documented Codex API, smoke-tested five cases, and verified live pre/post records in all three resumed workers. Workers checkpointed and pushed before restart; same branches/worktrees/panes and session histories retained. No product code written by orchestrator. Worker reasoning explicitly max on GPT-6 Astra.

## 2026-09-07 — Owner integration amendment

- Email transport borrows intromail's direct Resend REST client; no Resend CLI and no Resend MCP transport. GitHub uses a GitHub App instead of GitHub MCP. Owner instruction authorizes this plan change; no additional approval is needed.
- Verified via gh api at intromail commit `0bd24dfe284b888aa9f602fa1fd00e337ea38874`: `backend/app/services/resend_client.py`, `backend/app/services/github/client.py`, `docs/GITHUB-INTEGRATION.md`. The Resend client uses async HTTP, idempotency keys and explicit threading headers. GitHub client exchanges an App JWT for an installation token and makes installation-scoped REST calls.
- Source gaps are explicit: this Resend client has receiving-detail fetches but no inbox-list/poll helper; this GitHub client has no create-issue helper. Verify official contracts via gh api and borrow minimal glue during Step 2. Preserve the current pull-first decision unless the owner changes it; do not import intromail's hosted webhook assumption blindly.
- Chatwoot ticket/threading, evidence validation, human approvals and audit remain. No changes to Step 1 worker scope. No App registration, credential changes, installation on repositories, webhook deployment or live sends performed. Update integration effort estimates during Step 2 source review.

## 2026-09-07 — Autonomous continuation and UI quality gates

Owner requests continuing as far as possible autonomously, beyond the original Step 1 dispatch. Proceed through planned Phase 1 Steps 2 and 3, retaining exact borrowing rules and orchestrator-only role. Use Playwright CLI for browser validation; perform final Design Studio audit/fix/recheck loops. No additional approval is needed for implementation, local testing, reviews or the already-authorized PR merges. Keep actual third-party sends/publication separate from local test fixtures and report external configuration gaps.

## 2026-09-07 — Step 2 verified contracts

Use the actual `MCPServer` API at the mandated MCP Python SDK v2.0.1; its old FastMCP import is a deliberate failure stub, independently verified through gh api. This corrects the founding document’s scaffold name without changing the source pin or scope. Intake stays read-only against existing Hafidh GET routes; missing in-app/diagnostic read APIs are reported unavailable, not simulated as live integration. UI/draft, email and intake migrations reserve suffixes 000003/000004/000005 respectively.

## 2026-09-08 — Owner-authorized visual refresh

Owner finds the app basic and requests continued work. Reopen visual quality with the dispatch-desk direction in docs/design/VISUAL-DIRECTION.md. Preserve accepted workflows, user themes/fonts, source/borrow rules and local-only testing. Root orchestrates two implementation workers and one independent report-only reviewer through existing Herdr topic tabs. No new service choice or Phase 2 feature is implied; Sentry versus SigNoz remains undecided.

## 2026-09-08 — Business-facing owner steering

Owner wants a business development viewpoint and finds the current approach too technical. Reframe the active visual refresh around customers, conversations, owner decisions and actionable work; keep diagnostics in detail and preserve required approval evidence. Founder overview is the interim assumption while optional priority is pending. Do not present nonexistent CRM, revenue or launch features as implemented.

Owner confirms engineering is necessary but must not be the main hero. Business outcomes lead; engineering remains subordinate, accessible support. This confirms the founder-first visual direction without authorizing invented business capabilities.

## 2026-09-08 — Business domains and shared human/agent work

Owner explicitly adds marketing, channels, ads import/management, website and feedback, with engineering features backing them. Tasks must support humans and agents across both domains. Record target structure and gaps in docs/BUSINESS-WORKSPACE.md; reuse the existing task engine and approval seam after a schema audit, not a parallel business-task engine. Current visual workers continue their bounded real-surface refresh. Broader integrations and workflow additions will be separate reviewed tasks; no fake dashboard metrics or empty module claims.
