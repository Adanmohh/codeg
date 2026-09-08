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
- Root locked standalone server/companion debug build passed (details below). Native app
  packaging follows final UI fixes and Design Studio rechecks. No signing,
  notarization, distribution, live credentials or production-data launch.

## Standalone backend build and migration checkpoint

Locked offline root server/companion debug build passed,exit0,1m06s at accepted
Pi product. Replaced only verified owned4318 PID49874 with PID55173 using the
same `/tmp/ops-desk-browser-data`. Migration000008 applied successfully; server
listens and protected actual Playwright CLI Bug intake shows the honest
unconfigured state. An initial assertion guessed the wrong heading and timed
out; actual page text and the unconfigured-state assertion then passed.
Screenshot `reports/browser-phase1-final/backend-upgrade-intake.png`.
Static UI is still the earlier root export; final design/native export remains
pending. Logs `/tmp/ops-phase1-server-build.log`,
`/tmp/ops-phase1-server-startup.log`. No live provider/configuration write.

## Native executable checkpoint

`CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=4 cargo build --locked --bin codeg`
passed, exit0,1m53s, at root ecce0442 (accepted Pi product, documentation only
since2e0711d4). Log `/tmp/ops-phase1-native-binary-build.log`. The debug linker
warns that the unwind section exceeds16MB and may affect exception-handling
performance; inherited proc-macro-error2 future compatibility also remains.
This compiles the real desktop runtime, but does not replace the existing
bundle or establish a final frontend export. Packaging and isolated native
startup remain pending the UI corrections.
