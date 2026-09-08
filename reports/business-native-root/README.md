# Business native artifact acceptance — in progress

No final business app is accepted yet. Root prepares and checks artifacts only;
product corrections remain worker-authored and independently reviewed.

## Preparation completed

- Accepted backend main at `66c417f4`: offline release companion preparation
  passed in2m52s. Command: `CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=4 node
  src-tauri/scripts/prepare-sidecars.mjs`; log
  `/tmp/business-final-root-sidecars.log`. Actual arm64 binary staged; no skip.
- Main at `0eee9145` (subsequent changes are reports only): offline locked
  `cargo build --no-default-features --bin codeg-server --bin codeg-mcp`
  passed in2m18s with jobs4. Log `/tmp/business-final-root-server-build.log`.
  Inherited proc-macro-error2 future-compatibility warning remains. Neither
  command alone certifies the eventual package or its companion.
- Read the native source review at `6d066b0b`, including pinned Tauri CLI
  binary discovery, build-before-bundle ordering and overlapping MCP copies.
  Final acceptance must correlate all three bundled executables with completed
  profile outputs after the final ordinary Tauri build. Standalone bundling or
  a prior release-stage hash would be insufficient.
- Existing macOS permissions preflight returned accessibilityTrusted=true and
  screenCaptureAllowed=true. No permission change/prompt was requested.
  `inspect.swift` compiles and is a local acceptance helper for the explicitly
  owned native PID only. Apple installed SDK headers were read first; editable
  values are omitted from dumps, synthetic input arrives through stdin, and
  known business-bearer-shaped static text is redacted.

## Remaining gates

Exact corrected UI acceptance/merge, ordinary unsigned debug Tauri build,
all bundled binary/web hashes, isolated startup/migrations, actual native
business entry and owner/shared-member controls, and packaged companion
protocol validation. Browser checks use Playwright CLI; native accessibility
inspection is separate evidence and will not be described as Playwright or
cross-platform certification. No live model/provider operation is required.
