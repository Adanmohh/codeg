# Ops UI acceptance review

Fixes verified, final gates pending, 2026-09-08. PR #7 is not accepted. Reviewed Ops source at
`85a5dd18`, integrated with accepted GitHub/main at `c6066921` (no Ops source
change during integration).

The findings below are fixed at `0e375c9770505ac708ca97d12d07163c81a03f15`.
Root reviewed the per-backend memory provider and preserved draft/review
bindings, then independently repeated actual CLI reply/note/review resize cases
and discard dismissal successfully. Seven independent component tests pass,
including full subtree remount and colliding IDs after a backend switch.
Log `/tmp/ops-ui-independent-fixed-vitest.log`. No implicit server or browser
storage writes were introduced. Light Deny contrast no longer fails the probe;
dark and final end-to-end checks remain. After-fix screenshot/probe are in
[`browser-step2/`](browser-step2/).

## P2 — responsive transition silently discards unsaved edits

Independent Playwright CLI reproduction against the real protected fixture on
127.0.0.1:4320, session `ops-ui-independent`:

1. At 1280×800, select Reader / A real local thread.
2. Replace Reply message with `Unsaved viewport preservation sentinel`; do not save.
3. Resize to 390×844 and dismiss the mobile drawer with Escape.
4. The Ops selection has reset. Select the same ticket again: the value is
   `Original approved body`, and the unsaved sentinel is gone. No discard
   confirmation appeared.

Requested preservation of draft, private-note and approval edits/selection
across responsive layout changes, without leaking state across backend
connections or adding unrequested persistent storage of private content.
Require a regression and actual CLI repeat. Evidence:
`/tmp/ops-ui-viewport-draft.yml`, `/tmp/ops-ui-root-draft-lost-mobile.png`.
Root changed only browser-local unsaved state; no provider/DB mutation.

## Design correction — enabled destructive action contrast

On the light mobile stale-review surface, the enabled `Deny proposal` button
renders at 14px, opacity 1, foreground sRGB `#df2225` on effective `#fce8e9`:
4.07 contrast, below the 4.5 text requirement. Root verified enabled state and
browser-computed colors; this is not an exempt disabled control. Requested a
worker correction preserving the existing theme pairs, followed by light/dark
measurement. The stale send action is correctly disabled, and document width
remains 390px at the 390px viewport.

## Independent validation already passed

- `cargo test --locked --no-default-features --bin codeg-server --lib ops::tests`:
  exit 0, 22 passed in 0.91s, one explicitly ignored manual browser fixture.
  Log `/tmp/ops-ui-independent-rust-integrated.log`, root-owned Cargo target.
- `pnpm exec vitest run src/components/ops/ops-flows.test.tsx`: exit 0, 5 passed.
  These component mocks are not E2E; log `/tmp/ops-ui-independent-vitest.log`.
- Initial Rust attempt collided with temporary merge markers before tests;
  corrected by waiting for the worker's stable integrated checkpoint. That
  coordination failure is not a source finding.
- Actual fixture login, Inbox navigation, populated thread/private-note
  distinction and read-only desktop/mobile inspection work. Fixture uses the
  actual router/store/gate/HTTP client and test-only loopback provider, with no
  live Resend key. Root has not yet independently exercised provider mutations.

## Design evidence in preparation

Desktop light probe: 89 text samples, seven contrast candidates in the inherited
sidebar/status bar and two unnamed workspace controls. They require final
specialist verification against actual backgrounds/states; this is not a final
audit pass. Palette warnings include unsupported OKLCH/rounding and semantic
status colors, so raw linter warnings are not accepted as palette defects.
Files: `/tmp/ops-ui-design-probe.json`, `/tmp/ops-ui-design-lint.txt`,
`/tmp/ops-ui-root-thread-desktop.png`. The first mobile screenshot captured the
open responsive drawer; it is not evidence of a clipped thread layout.

Selected before-fix screenshots and measured JSON are preserved in
[`browser-step2/`](browser-step2/) for the final Design Studio reviewers.

Browser console errors are fixture Git-head/state-stream 404s for its synthetic
folder, plus inherited input/form advisory messages. No zero-console claim.
Final browser scenarios, review fix, complete gates and Design Studio loops remain.
