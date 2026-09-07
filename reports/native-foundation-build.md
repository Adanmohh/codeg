# Native foundation build — 2026-09-07

Built the current merged Step 1 foundation, with root documentation through `f8573d59`. This is not the later unmerged Ops UI/transport/intake implementation.

- Read the actual sidecar preparation script, Tauri configuration and installed build help first. `CARGO_BUILD_JOBS=4 pnpm tauri:prepare-sidecars` exited 0 and replaced the ignored zero-byte placeholder with the real host-arm64 release companion.
- `CARGO_BUILD_JOBS=4 pnpm tauri build --debug --bundles app --no-sign --ci` exited 0, including the production frontend build and native bundle. Signing was explicitly skipped; no signing/notarization/store/distribution action was performed.
- Bundle: `src-tauri/target/debug/bundle/macos/Hafidh Ops Desk.app`. Info.plist identifies `app.hafidh.opsdesk`, executable `codeg`; the bundle contains real codeg-mcp and codeg-server binaries.
- Started only this bundle executable with both CODEG_DATA_DIR and CODEG_HOME set to `/tmp/ops-desk-native-data`. It stayed alive, created its isolated codeg-dev.db and applied both Ops migrations without a panic. Stopped only the owned test process afterward. This is a native startup smoke check, not a full WKWebView interaction or production-data test.
- Browser behavior remains separately tested using Playwright CLI against the real standalone server; see `reports/browser-step1.md`. No native browser-automation equivalence is claimed.
- Root git status remains clean; generated bundles/sidecars are ignored artifacts. Existing proc-macro future-compatibility warning remains. Local logs are `/tmp/ops-desk-real-sidecar-build.log`, `/tmp/ops-desk-foundation-app-build.log` and `/tmp/ops-desk-native-startup.log`.

Rebuild the app after accepted Step 2 product changes before treating it as the final Phase 1 artifact.
