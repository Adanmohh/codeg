# Step 0 verification — 2026-09-07

Baseline: xintaofei/codeg v0.30.4, `6f6bd648b206412644842a98d9ffeebf57292bed`, Apache-2.0.
Fork: https://github.com/Adanmohh/codeg.

| Gate | Result | Local log |
| --- | --- | --- |
| `pnpm install --frozen-lockfile` | PASS, exit 0; pnpm 11.9.0 | step0-pnpm-install.log |
| `cargo check --locked` in src-tauri/ | PASS, exit 0; default desktop features, 36.38 s final run | step0-cargo-check.log |
| `pnpm build` | PASS, exit 0; Next 16.1.6; compiled, TypeScript checked, 32/32 pages exported | step0-frontend-build.log |
| `pnpm exec tsc --noEmit` | PASS, exit 0 | step0-typecheck.log |
| Product-source comparison to v0.30.4 | PASS; no tracked upstream source/config/lockfile changes | `git diff v0.30.4` |

Read the tagged local manifests, build scripts, configuration and CI instructions before running these gates. See DECISIONS.md for versions and initialization choices.

The initial sandboxed Cargo attempt could not write its dependency cache. The first unsandboxed compilation lacked the frontend resource directory; the upstream CI placeholder resolved this. The sandboxed Next compilation stalled and was interrupted; the identical unsandboxed build passed and replaced the placeholder with the real export.

Known upstream warnings: build.rs stages a zero-byte MCP sidecar for compile checks; prepare the real sidecar before packaging. Cargo reports a future-compatibility warning for proc-macro-error2 2.0.1. Neither blocks the requested compile gates. A running desktop app, real MCP sidecar, signing and end-to-end process tests were not validated by Step 0.

Raw .log files remain local under reports/ because upstream ignores *.log. This report is the committed evidence summary. No workers were dispatched; Step 0 is explicitly orchestrator-only. No source ports or AGPL source additions occurred.
