# Design Studio loop evidence

Prepared 2026-09-08 against `docs/design/BRIEF.html`, schema1 scan mode,
using the stable BC checklist. Selector returned aesthetic-judge, a11y-auditor
and flow-validator for the full Ops page with flows. These are executor evidence
records adapted to actual Playwright CLI, not a final scored audit or invented
flow-test.mjs/LLM execution. Screenshots and post-interaction states are authoritative.

Final specialists will run through the existing Herdr workers after feature
handoffs free their slots. No fourth worker, paid judge API, Playwright MCP or
Design Studio browser launcher was used. P1, Telegram and receipt-recording
recovery evidence remain to be added before final synthesis/fixes/recheck.

## Deterministic lint interpretation

Initial lint outputs for accepted Ops components, login and two inherited shell
controls are in `lint/`; all tool commands exited0. The measured Ops page lint
also uses the committed post-fix probe and brief. Exit0 is tool completion, not
a design pass. Static high-level heuristics flag reply-editor missing empty/
loading states although its parent owns them, and report no hover/focus despite
shared Button styling. Do not promote these to defects without actual DOM/flow
evidence. Conversely login static lint reports zero gaps despite the previously
observed missing persistent token label; measured accessibility remains required.

Root additionally verified actual unnamed controls: terminal new-tab plus button
(`src/components/terminal/terminal-tab-bar.tsx`) and status-bar alert popover
(`src/components/layout/status-bar-alerts.tsx`). The latter has no accessible
name when empty; source has no aria-label and the ARIA snapshot confirms it.
Controls with title attributes were excluded from this finding. Final a11y judge
should verify and prescribe the smallest inherited-label/focus fixes.
