# N2/P2 — business route lacks native window chrome

Actual packaged macOS run from6cfff7d6 (producte72), PID27415/window49837.
`local-workspace.png` shows native traffic lights over the brand mark; connect
startup.png also has intersecting native buttons/brand. Window bounds are
1260x860 at126,42. A guarded real pointer drag on the blank top header from600,62
to680,102 leaves window bounds unchanged (header-drag.json). The same pointer
helper successfully opens administrator access, chooses local desktop and
creates the actual local organization; this is not inferred from failed AXPress.

Source: business connect/bootstrap/workspace render no AppTitleBar or drag region.
Shared native styling in commands/windows.rs uses Overlay and workspace traffic
light placement. Existing AppTitleBar reserves native controls and supplies a
real drag region; WindowControls supplies Windows/Linux controls. Those platforms
were source-inspected, not run. No claimed Windows native failure screenshot.

Required bounded correction: reuse existing window chrome for all business
connection/bootstrap/work states, reserve macOS controls and a usable drag area;
retain native Windows/Linux controls via existing implementation. Preserve web
layout, scoped identity/state/draft lifetime and explicit engineering navigation.
Rebrand owns product changes in a new branch/PR from accepted main. Root will
repeat actual native startup/chrome/drag plus affected browser checks after merge.
No new auth/engine/domain behavior or new dependency is requested.

Positive drag control: after explicit engineering navigation, the same helper
moves the existing blank titlebar from126,42 to206,67 with pointer1000,62 to
1080,92. An earlier engineering600 coordinate hit a tab and is excluded from
that control. Window restored with reverse drag. No unrelated window moved.

## Closure — 2026-09-08

Resolved at product3d000874, reviewed independently at b33b1a9f and merged PR24.
Actual normal rebuilt package f4757d8d passes three-state clearance/drag,400px
setup/workspace and native minimize/hide/reopen checks. See README.md and
n2-runtime-result.json for exact evidence and limits. No remaining verified N2
finding; green fullscreen and other desktop platforms were not runtime tested.
