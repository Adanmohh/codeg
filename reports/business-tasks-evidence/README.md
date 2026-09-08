# Business task evidence

Product source: `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5` (R1 entrustment correction). This is real Chromium fetch against the production protected Rust router, using synthetic in-memory data. The landing page is explicitly test scaffolding. No production UI response is mocked; no business-workspace visual acceptance is claimed here.

The ignored manual fixture was started with:

```sh
src-tauri/target/debug/deps/codeg_lib-b542e2757758a7bb business_tasks::tests::fixture::business_tasks_browser_fixture --exact --ignored --nocapture
```

It bound only `127.0.0.1:4342`, PID794, with no engine/scheduler/provider startup. The server guard permits only business API paths, health and its test landing. Browser guards aborted off-origin requests; the first session recorded zero off-origin attempts. No real credentials, config snapshots, model calls or sends. Synthetic credential tokens stayed in browser memory and were never included in these artifacts.

Actual Playwright CLI0.1.18 commands (all exit0):

```sh
playwright-cli -s=business-task-author4342 open about:blank
playwright-cli -s=business-task-author4342 --raw run-code --filename=reports/business-tasks-evidence/browser-human.js
playwright-cli -s=business-task-reviewer4342 open about:blank
playwright-cli -s=business-task-reviewer4342 --raw run-code --filename=reports/business-tasks-evidence/browser-reviewer.js
playwright-cli -s=business-task-author4342 --raw run-code --filename=reports/business-tasks-evidence/browser-final.js
playwright-cli -s=business-task-author4342 close
playwright-cli -s=business-task-reviewer4342 close
```

Each run-code output was redirected to its matching JSON file. **55/55 assertions passed**: human34, reviewer12, final9. They cover individual actor attribution; human-only create/progress/submit/review; shared reads across two independent browsers; calendar-only deadline; real concurrent HTTP revision conflict with one activity; viewer/domain/spoof denial; exact deliverable review; forbidden generic done; restricted source entrustment; revocation and unchanged history after rejected writes. Source scripts were subsequently formatted with Prettier only; assertions and operations are unchanged. Artifact ESLint exits0 with three expected unused-expression warnings because the CLI requires a function expression; product Pi lint has zero warnings.

Browser processes4549 and5528 both reported closed. The manual fixture was deliberately stopped with Ctrl-C through its own PTY: tool reported exit1, not a passing ignored test. `lsof -nP -iTCP:4342 -sTCP:LISTEN` then exited1 with no output, confirming listener shutdown. No other fixture/browser was touched. A later read found a **new** listener PID21431 on4342, whose cwd is the rebrand worker's `.build/business-workspace/task-fixture-1ba73e3c/src-tauri`; that separately owned fixture was left intact and its owner notified. This worker has no remaining fixture/browser.

`hooks.json` contains sanitized live PreToolUse/PostToolUse records from this worker/session and worktree; both exit0. It contains no command arguments or credential values. The remaining Rust, native, companion and actual extracted Pi process gates are recorded in `../business-tasks.md`.
