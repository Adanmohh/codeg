# Login main landmark correction

Status: complete; focused gates and actual browser assertions passed.
Branch: `fix/design-login-landmark`, created from accepted main
`602f852cf25017e41f46091f92a17f075f4a996a`.
Validated product commit: `66849a91c7a6bdb5105215bbabd70e659ddb36f4`, pushed
early with this report's checkpoint. [Draft PR #20](https://github.com/Adanmohh/codeg/pull/20)
records the final documentation/evidence head; no product change followed the
validated commit. No merge or deployment performed.

## Scope and implementation

The final auditor's `design-phase1-specialist/findings/a11y-baseline.json`
reports a missing main landmark on the connection page (low severity). I read
that report read-only in the approvals worktree and confirmed that the login
page and root layout have no main wrapper. `src/app/login/page.tsx` now uses
`main` for its existing outer container. Only its opening and closing tags
change; classes, heading, form, token handling, error association, retry and
internal navigation remain the accepted implementation. No new test merely
asserts the two-tag edit.

## Docs, source mapping and licence

- Read current FOUNDING, ORCHESTRATOR, STATUS, DECISIONS and AGENTS completely.
  Separate reads of `node_modules/react/package.json` and `src-tauri/Cargo.toml`
  exercised the enabled docs-first hook. Audit file:
  `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`; own session
  `01a07c1c-d82f-7022-84db-778a438632f1`, exact tickets worktree, PreToolUse
  timestamp `1788835349` and PostToolUse `1788835326`, both exit 0.
- Applied code-context with the existing rag-skills `.venv/bin/python` and
  `HF_HUB_OFFLINE=1`. `guide` exit 0: relevant strong rule requires actual
  headed Playwright CLI verification. `docs` exit 3: missing
  `data/code/tickets.db`, so this worktree has no indexed dependency coverage.
  Direct installed source/types supply the missing coverage.
- Installed React **19.2.4**, `@types/react` **19.2.13**:
  `node_modules/@types/react/index.d.ts:4225` explicitly supports intrinsic
  `main` as `HTMLAttributes<HTMLElement>`. Read the complete login page,
  root layout, locator and relevant existing authentication tests first.
  Next **16.1.6**, TypeScript **5.8.3**, Vitest **2.1.9**, pnpm **11.9.0**.
- Source-to-change: Codeg **v0.30.4**, immutable commit
  `6f6bd648b206412644842a98d9ffeebf57292bed`,
  `src/app/login/page.tsx` → the same inherited page, two semantic wrapper tags.
  Read the original file and complete `LICENSE` through `gh api` at that exact
  ref after local source. Apache-2.0; local and upstream LICENSE blob both
  `261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64`. Existing NOTICE attribution already
  lists this page; all NOTICE sections and LICENSE remain unchanged. No new
  third-party port, AGPL code, dependency or lockfile change.
- Browser fixture reuse: accepted `src-tauri/src/work_task/desk/tests.rs::
  pi_desk_browser_fixture` and unchanged
  `reports/design-reduced-motion-evidence/{server.mjs,guard.js}` at this branch
  baseline. The protected router uses only a public synthetic fixture token,
  fresh temporary database and empty local Pi catalogue; it starts no engine.
  The proxy serves a separate owned export on 4325 and forwards to owned 4324.

## Commands and observed results

Commands ran in this worktree, except the fixture command ran in `src-tauri`.
Installed CLI help and source/types were read before use. Playwright CLI
**0.1.18** uses installed core **1.63.0-alpha-2026-08-05**; no upgrade.

| Command | Result |
| --- | --- |
| `pnpm exec vitest run src/lib/ops-telegram/locator.test.ts src/lib/transport/web-transport.test.ts` | Exit 0; 15 passed / 2 files, 633 ms. Existing navigation-locator and authentication/retry cases. |
| `pnpm exec eslint src/app/login/page.tsx` | Exit 0. |
| `CODEG_EXPORT_DIR=src-tauri/target/design-login-landmark-export pnpm build` | Exit 0; 33 static pages, compilation 5.3 s. Separate owned output. |
| `pnpm exec tsc --noEmit` | Exit 0, after export. |
| `PI_CODING_AGENT_DIR=/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/src-tauri/target/pi-desk-browser/login-landmark-empty-catalogue cargo test --locked --no-default-features --lib pi_desk_browser_fixture -- --ignored --nocapture` | Manual protected fixture: exit 0; 1 passed, 267.93 s including browser lifetime. |
| `node reports/design-reduced-motion-evidence/server.mjs src-tauri/target/design-login-landmark-export` | Owned proxy served 4325; closed via SIGTERM, exit 0. |
| `playwright-cli -s=login-landmark open about:blank --headed` | Exit 0; fresh in-memory browser for each width. |
| `playwright-cli -s=login-landmark --raw run-code --filename=reports/design-reduced-motion-evidence/guard.js` | Exit 0 in each browser, installed before navigation. |
| `playwright-cli -s=login-landmark resize 390 844` / `resize 1280 800` | Both exit 0. |
| `playwright-cli -s=login-landmark --raw run-code --filename=reports/design-login-landmark-evidence/verify.js` | Final run at each width exit 0, all assertions passed. |
| `playwright-cli -s=login-landmark --raw console` / `close` | Exit 0 at each width; browser closed. |
| `touch src-tauri/target/pi-desk-browser/stop` | Exit 0; graceful Rust fixture completion above. |
| `lsof -nP -iTCP:4324 -iTCP:4325 -sTCP:LISTEN` | Exit 1 with no output after cleanup: neither listener remains. |

Exact gate output is committed in adjacent `design-login-landmark-*.log` files.
The reused Rust fixture emitted inherited linker unwind-size and
`proc-macro-error2` future-compatibility warnings; Vitest emitted its existing
Vite CJS deprecation notice. No failure in those gates.

## Rendered browser evidence and limits

Both [390 px DOM](design-login-landmark-evidence/390-result.json) and
[1280 px DOM](design-login-landmark-evidence/1280-result.json) measure **one
native main and one accessibility main role**, containing the heading and form,
in empty and invalid states. Main geometry is `(0,0,390,844)` and
`(0,0,1280,800)` respectively; document width equals viewport width.
Initial token input is focused, persistently named and masked; Connect is
disabled. The actual protected router returns **401** for an invalid synthetic
token, preserves `aria-invalid`, `aria-describedby="login-error"` and the alert
inside main, stores no token, and re-enables retry. Enter with the public valid
fixture token returns **200**, stores the synthetic token and navigates to
`/workspace`. The initial accessibility snapshots are included in the JSON.

All six screenshots were inspected: [390 initial](design-login-landmark-evidence/390-initial.png),
[390 invalid](design-login-landmark-evidence/390-invalid.png),
[390 destination](design-login-landmark-evidence/390-workspace.png),
[1280 initial](design-login-landmark-evidence/1280-initial.png),
[1280 invalid](design-login-landmark-evidence/1280-invalid.png),
[1280 destination](design-login-landmark-evidence/1280-workspace.png).
Both guard records are all zero: off-origin, agent actions and configuration
writes. [Metadata](design-login-landmark-evidence/metadata.json) records source
and exported login HTML SHA256, fixture exits and listener shutdown. Only the
synthetic fixture credential was entered; no real credentials or config dump.

One exploratory CLI run exited 1 because an unscoped alert locator also matched
Next's route announcer. The report-only selector was scoped to main and both
complete runs repeated in fresh browser contexts; product source was unchanged.
The [original diagnostic](design-login-landmark-evidence/00-selector-exploratory.log)
is retained. Console logs contain the expected invalid-token 401 and inherited
autocomplete suggestion, not a clean-console claim. Scope is English/light
Chromium at the requested viewport widths; this is not a screen-reader,
cross-browser, native-app or whole-app accessibility audit. No new page unit
test mirrors this markup change, and no broad frontend or Rust gate rerun was
needed. No model, provider, engine, configuration or live external action ran.
