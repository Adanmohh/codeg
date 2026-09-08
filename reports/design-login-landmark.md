# Login main landmark correction

Status: implementation checkpoint; focused checks and actual browser evidence pending.
Branch: `fix/design-login-landmark`, created from accepted main
`602f852cf25017e41f46091f92a17f075f4a996a`. Draft PR URL and validated source
commit will be recorded after the early push.

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

## Validation plan and checkpoint

- Run existing locator and web-transport tests, focused page lint, production
  export to `src-tauri/target/design-login-landmark-export`, then typecheck.
- Actual installed Playwright CLI **0.1.18**, isolated headed session:
  at 390 and 1280 px, count exactly one main containing heading/form; verify
  empty/invalid/retry/valid navigation and capture DOM plus visible screenshots.
  Only synthetic local credentials; no credential/configuration dumps.
- Reuse the accepted guard against off-origin, agent and configuration actions.
  Record results and close only these owned browser/listener instances.

No product code outside the login wrapper is changed. Rust gates and broad
frontend reruns are outside this two-tag correction; the existing manual Rust
fixture will be executed for the requested browser flow. Final evidence and
limitations will replace this checkpoint section before handoff.
