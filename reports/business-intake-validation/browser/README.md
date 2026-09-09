# Worker browser checkpoint

This is partial actual Playwright CLI evidence for the unified fixture, not full
B/UI acceptance. Backend product e55f3bfd and fixture source7ed0dd0c are unchanged.
The independently checked UI product is1974d97f, built head6739c476. All27 manifest
files matched disk; the served business HTML matched42f37747. Exact hashes are in
`asset-verification.json`. Root/rebrand own their separate acceptance results.

The worker opened only the new headed `intake-worker4354`, reported browser
PID1965, at1280×900. It used only the worker namespace and its existing private0600
credential file. Installed CLI0.1.18 / Playwright1.63.0-alpha-2026-08-05 `runCode.ts`
(installed coreBundle.js67013–67069) explicitly executes an unsafe function with
the actual Page in a Node VM. The two checked-in scripts use that documented
execution surface and Node24.19.0 `process.getBuiltinModule`/`fs.readFileSync` to
read the private file in process memory. Installed Node types and Playwright
route/locator types were read. No credential literal enters the script, command,
snapshot, screenshot, browser storage export, report or result. Script errors
after filling a key clear the field and emit only a generic message. Normal
product authentication and input submission remain unchanged. No package or
third-party source hunk was added; these scripts are test orchestration glue.

Executed commands/results:

| Command/action | Result |
| --- | --- |
| `node --check .../worker-login.cli.js` | exit0 |
| `playwright-cli -s=intake-worker4354 open http://127.0.0.1:4354/business --headed` | exit0; new isolated browser |
| `resize 1280 900`, empty login/sources screenshots | exit0 |
| `run-code --filename=.../worker-login.cli.js` | exit0; actual context/settings/tasks/members200; passwordInputs0; offOrigin0 |
| Sources navigation, empty source snapshot | exit0; setup available, no source records |
| `run-code --filename=.../worker-setup.cli.js` | exit0; actual binding create200; disabled revision1; no implicit grant; key cleared |
| Explicit owner read/import/triage/Feedback grant via real form | exit0; grants/upsert200; resulting revoke control visible |
| Enable connection via real form | exit0; bindings/update200; resulting Disable connection control visible |
| Static asset correspondence verifier | exit0;27 local matches and actual HTTP HTML match |

`03-disabled-zero-grants.png` shows the disabled state and empty replacement-key
field. Its lower grant section is below the viewport; the zero-grant assertion
comes from the actual DOM/script and YAML, not that cropped image. All screenshots
were captured after secret fields had been cleared. The page-level request guard
recorded zero off-origin requests through setup/enable; the fixed upstream is
synthetic and read-only. This page guard is not a native or OS isolation claim.

Exploratory limits are retained: CLI `select e261 --label=...` exited1 because
that command accepts a value, not `--label`; it made no selection. The subsequent
`getByLabel('Source connection').selectOption(...)` timed out after30seconds when
the visible window had changed to My work. Another snapshot showed the existing
task editor. Root then confirmed the owner's screenshot matched this window.
Those view changes are user inspection, **not a reproduced spontaneous UI bug**.
The Source-selection/import/publication flow did not complete in this session.

From that confirmation onward the browser and visible task/draft are reserved for
the owner: no further automation, readback, screenshot, refresh, save or close.
No more worker-record mutations occur while the owner inspects. The two source
setup scripts are retained for provenance; do not rerun them in that session.
Root's32 protected HTTP checks and the reviewer's guard-source verdict are
separate reported evidence. No broad test/build rerun was made for this report.

The owned backend/upstream PID66200 and stable copied executable remain running.
No target, native bundle, old fixture/export/browser, or other namespace changed.
The ignored manual fixture has not exited, so no completed-fixture test is claimed.
