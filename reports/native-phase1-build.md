# Phase 1 native build — in progress

Accepted Pi P1 product through merge2e0711d4 is on root main. The final UI badge
and reduced-motion corrections remain unmerged; the existing app bundle still
contains Step1 and must not be treated as the final app.

- Read actual `src-tauri/scripts/prepare-sidecars.mjs`, package build hooks and
  Tauri config. An initial lookup used the wrong scripts directory; no command
  was inferred from that missing path.
- `CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=4 pnpm tauri:prepare-sidecars` passes,
  exit0,2m11s optimized build. Real arm64 companion staged at
  `src-tauri/binaries/codeg-mcp-aarch64-apple-darwin`. No dependency/lock change.
  Log `/tmp/ops-phase1-sidecar-build.log`.
- Root locked standalone server/companion debug build is running. Native app
  packaging follows final UI fixes and Design Studio rechecks. No signing,
  notarization, distribution, live credentials or production-data launch.
