# Business native artifact acceptance — passed within scope

The final unsigned debug business app is locally accepted at f4757d8d. Root prepares and checks artifacts only;
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

## Existing server database upgrade — passed

Verified old root-owned4318 listener PID55173, then replaced only that process
with this package's actual codeg-server (SHA7093247d), new PID61465. Same
isolated `/tmp/ops-desk-browser-data`, loopback4318, public synthetic test token,
root out and existing Python adapter. Log `/tmp/business-final-root-server-upgrade.log`
records both000009 and000010 applied; protected real business/context returns200,
operator:true and needsBootstrap:true. Existing Ops database was upgraded, not
reseeded. `server-upgrade-context.json` records the exact source/identity.
Final browser check will use the final refreshed root export after N2 acceptance.

## Final N2 package and actual runtime — passed

PR24 merged at exact handoff f7a74b67151a75fd05a15910d9cb84d17356e59d,
merge c0ebd3d5bc9414b72411bfff3f8949c9d7b5ca95. Product
3d0008747ab1933bf9c1329f8e1b59d949992623 has independent23/23 source/test
review b33b1a9f74016a1543750108af5f6f3f8ca0de33; owner52 focused tests,
typecheck, scoped lint, desktop check and export pass.

Root ordinary unsigned debug build at f4757d8dcd9e62824bfde577a5595a5a2a5e5c7c
completed exit0, log /tmp/business-n2-root-app-build.log. No product changes
occurred during compilation. Final bundle-manifest.json correlates all three
executables with completed debug outputs and all1,017 web files with out,
zero missing/extra/different. first-bundle-manifest.json preserves the previous
candidate. Final MCP hash9c532913 differs, so the earlier90f9 artifact run is
not claimed for this final artifact; a new bounded run is dispatched in stable
window business-native-n2-f4757d8d.

Actual bundled codeg PID95325/window49887 uses its own backend directory
/tmp/ops-business-native-n2-f4757d8d. AX route is tauri:/business. Root visually
inspected connect/bootstrap/workspace at400 and1260 logical pixels: the40px
caption strip clears the brand and preserves physical macOS controls. Actual
HID drag moves connect (126,42)→(206,67), bootstrap (126,67)→(206,92), and
400px workspace (206,92)→(246,102). Native setup at400px successfully creates
N2 Native Studio/N2 Review Owner using real pointer/keyboard input. n2-*.png,
AX dumps and n2-runtime-result.json record these results.

Actual yellow-button click minimizes (AXMinimized=true); the test-only helper
restores AXMinimized=false using the installed SDK's documented writable boolean.
Actual red-button click hides main, leaving no AXWindow while the app remains
alive, matching unchanged lib.rs main-window tray policy. System open restores
the same N2 studio and visible window. Green zoom/fullscreen was not executed.
No Windows/Linux runtime or complete screen-reader certification. The backend
directory does not replace the existing WebKit profile. No native log ERROR or
panic matched. Root stopped only95325 with TERM after acceptance.

The final server binary7e94301b replaces only root-owned4318 process61465 using
the same upgraded synthetic database. /tmp/business-n2-root-server.log. Actual
Playwright CLI session root-business-package passed five cold aliases200 with
zero API/WS requests and no native chrome/drag targets in web. It bootstrapped
Root Integrated Studio/Root Integration Owner, retained a full task draft at
400/1280 with no document overflow, and created Final package: customer feedback
follow-up through the real backend. final-web-check.raw records8 business API
requests, no blocked outside request and no sockets. The first screenshot caught
the existing opening transition; it is retained as such, not a settled visual
claim. A separate read-only sign-in/detail visit captured final-web-task-settled.png
after500ms; root inspected it. Both owned browser sessions were closed. No
existing worker fixture was restarted or changed by these checks.

N2 runtime is closed within this scope. Final companion evidence and acceptance
summary remain; prior native human Done4/shared viewer/revocation evidence stays
valid for unchanged backend/session behavior.

## Final companion closure

The new exact9c532913 artifact passed the unchanged9e9fdc9b harness:275
assertions, zero failures, clean child exits15576/15577/15578 and removal of
.bpc-KgXqT9. Before/after21711944 bytes and hash match. Root read the complete
result-1788889157262-15575.json and independently verified its SHA256
d970effe076fe8b16dd2a49b4eb5a2c4455c22a91e834ddc65d65e491d87a986.
Stable window ended after execution; no further build occurred. Scope remains
MCP transport with a synthetic UDS peer, not repeated live backend authorization.
Final root server4318 PID97709 remains on the final7e94301b executable; root
native95325 and owned final CLI browsers are closed. All Increment A local gates
are complete, with historical package/runtime evidence above preserved.
