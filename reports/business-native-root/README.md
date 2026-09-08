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

## First actual business package and native interaction — passed with N2 open

Ordinary `CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=4 pnpm tauri build --debug
--bundles app --no-sign --ci` at6cfff7d64e4d45f84d8cd5a26325b6ad4e1f311f
completed exit0. Next16.1.6 compile/typecheck and34 static routes, release MCP
2m28s, native all-bin build2m33s and unsigned app packaging passed. Log
`/tmp/business-final-root-app-build.log`. Inherited large-unwind and
proc-macro-error2 future-compatibility warnings remain. Later main changes were
reports only; scoped product diff was verified empty during the build.

`bundle-manifest.json`: all three actual bundled executables match their completed
debug profile SHA256. All1016 bundled web files match out by path/hash, with
zero missing/extra/different. This is a real built candidate, but N2 requires a
final refresh. Companion90f9fd8e passed275 bounded synthetic transport assertions
through tickets' unchanged harness464ae97a. Root read final report31cf27ea,
verified result hash9f7608ab and actual child exits/cleanup/unchanged binary.
See ../business-bundled-companion.md; counts include repeated RPC health checks.

Launched the actual bundled codeg with CODEG_HOME/CODEG_DATA_DIR both
`/tmp/ops-business-native-6cfff7d6` and existing isolated Python adapter path.
PID27415/window49837 initialized the database through000009/000010. Actual
WebKit AXWebArea reports `tauri:/business`. No broad settings dump or live
provider/model operation. This isolates backend data; it does not replace or
certify a fresh macOS WebKit storage profile.

Root real pointer/keyboard actions opened Administrator access, selected local
desktop, bootstrapped Synthetic Native Studio/Native Review Owner, created
marketing task5dad33f4-cb4c-4861-9969-d99ed9f1bf6f, assigned it to the named
human and progressed To do1 → In progress2 → Review3. Accept stayed disabled
until explicit human confirmation; a review note then completed Done4. This
human-progress path allows review without a deliverable and was not described
as an agent submission. Read-only SQLite confirms four activity rows, current
Done4 and no execution. `local-task-db.json`, task-done.png/AX evidence.

Explicit engineering navigation changes the native route to /workspace. Its
blank titlebar positively moves the window using the same pointer helper;
N2 shows business chrome overlap and absent drag surface. The initial engineering
600 coordinate landed on a conversation tab and is excluded from the drag
control. No engine/model message was sent. Stopped only PID27415 with TERM143.
Restart PID46273/window49871 returns /business/sign-in; backend data persists.

Shared-member WebKit check used a new Root Native Shared Viewer credential on
the existing guarded4342 real backend. Synthetic token was passed only from
in-memory API response to keyboard stdin, never argv/log/clipboard. Native UI
shows Hafidh Studio · Synthetic, named Viewer and Connected instead of local
studio/Local desktop. Shared work/detail contains actual named humans and
Done6/calendar2026-10-01/deliverable; only Close is available inside the detail,
with no editing/engineering controls. Own credential revoke200 followed by
focus/context revalidation clears the task and shows expired-or-revoked sign-in.
Existing/root/reviewer tasks were only read. Setup/revoke metadata and settled
shared-detail/revoked-session screenshots are retained. Stopped only46273 with
TERM143. All pre-existing browser fixtures remain untouched.

Native automation limits: initial AX tree was lazy; its first read exposed no
WebArea, the later read did. AXPress/AXValue and per-PID pointer posting returned
success without UI effect, so none counts as a passing interaction. Actual
CoreGraphics HID events with an explicit active-app PID guard performed the
successful actions. No product instrumentation/eval patch was added. Test-only
helpers use installed Apple SDK CGEvent/AX/NSRunningApplication headers. WebKit
source893cb75bec40fc7ef8e4ea2d63a81b9b1de137e4 WebViewImpl.mm was read via
gh api to understand on-demand accessibility; this is not a claim that this OS
ships that exact WebKit source. Dumps exclude menu bars and editable values;
route identity strips URL query/credentials. Only owned-window screenshots.

**Remaining:** N2 worker correction, targeted checks/review, final normal package
refresh and actual native chrome/drag/recovery recheck. Browser Design Studio
closure remains valid for unchanged business state/data behavior. No Windows/
Linux runtime, signing/notarization, live provider or full screen-reader claim.
