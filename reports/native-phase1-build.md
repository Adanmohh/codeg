# Phase 1 native build — passed

The final unsigned debug app contains accepted Phase1 product through login
mergeff319380, including badge and reduced-motion corrections, built from root89fef6fd.
The checkpoints below preserve earlier intermediate results; final evidence
is recorded at the end. Combined Design Studio report acceptance is separate.

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

## First combined package and isolated startup checkpoint

`CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=4 pnpm tauri build --debug --bundles app --no-sign --ci`
passed, exit0. Before-build completed Next16.1.6 compilation/type checking,
all33 static pages, and the real release companion preparation. Native custom
protocol build completed in2m02s; signing was explicitly skipped.
Log `/tmp/ops-phase1-app-build.log`.

Artifact: `/Users/mohamedadan/projects/ops-desk/src-tauri/target/debug/bundle/macos/Hafidh Ops Desk.app`.
Info.plist: name Hafidh Ops Desk, ID app.hafidh.opsdesk, executable codeg,
inherited version0.30.4. Actual executable contents:

| File | Bytes | SHA-256 |
| --- | --- | --- |
| codeg | 305145480 | b33c084835e742d2d935e9b53caddbc80b1da8c7d451003fd99f6671391a2607 |
| codeg-mcp | 21710248 | a62ddc7e486909b33df369a06b9afbd7f647136ab5d26a59725c3461df66690e |
| codeg-server | 236515680 | 7de3d4b51cdad729215fcdd76b8ff09ea8f72c15f9499ac04c07a77c98806005 |

Bundled Resources/web exactly matches all1006 final out/ files by path and
SHA-256: zero missing, extra or different files. Evidence:
`reports/browser-phase1-final/bundle-web-verification.json`.

Launched only the bundle executable with CODEG_DATA_DIR and CODEG_HOME set to
`/tmp/ops-desk-native-data`, plus CODEG_INTAKE_PYTHON pointing to the existing
ignored root adapter environment. Owned PID12691 stayed alive and upgraded
the existing Step1 test DB through migrations000003–000008. Read-only SQLite
verification finds all eight Ops migrations. No panic or ERROR was recorded.
Stopped only that owned process with TERM (expected exit143); confirmed absent.
Log `/tmp/ops-phase1-native-startup.log` remains local, not committed.

Separate actual Playwright CLI against final root export passes reduced/normal
motion, focus/dismissal and empty Ops/Morning layouts. Root82 focused integrated
tests pass, exit0,1.93s (`/tmp/ops-phase1-final-design-tests.log`), supplementing
the earlier6158 frontend/178 Ops/13 Desk passes. This is native startup plus
separate Chromium interaction evidence, not full WKWebView, signing, portability,
live provider or successful Astra inference certification.

## Final refresh including the login landmark

After PR20 acceptance, repeated the same unsigned offline Tauri app build at
root89fef6fd. Exit0: all33 static pages/type checking, real companion staging,
native build33.20s and app packaging. The artifact path and app identity remain
as above. Final binary sizes/hashes are in
`reports/browser-phase1-final/bundle-binaries.json`; all1006 bundled web files
again match the new out/ exactly, with zero missing/extra/different files.
Log `/tmp/ops-phase1-app-build-final.log`.

Started the refreshed artifact in the same isolated CODEG_HOME/CODEG_DATA_DIR,
with the explicit root Python adapter path. Owned PID59320 stayed alive over
30 seconds, reported no pending migrations,0 ERROR lines and no panic. Stopped
only that process with TERM (expected143). Log
`/tmp/ops-phase1-native-startup-final.log`. This rechecks the actual refreshed
artifact; the earlier migration test remains separately recorded above.

Root actual final-export Playwright CLI390/1280 passes one main enclosing the
login heading/form, invalid-token error association, retry and workspace
navigation, without horizontal overflow. Synthetic local token only; browser
closed. Evidence `reports/browser-phase1-final/login-recheck.json` and inspected
invalid-state images. No broad test rerun was needed for the two semantic tags;
worker15 existing authentication/locator tests and focused gates pass.
