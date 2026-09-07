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
