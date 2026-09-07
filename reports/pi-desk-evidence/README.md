# Pi default and setup browser evidence

Actual headed Playwright CLI 0.1.18, using its installed Playwright
1.63.0-alpha-2026-08-05. No browser MCP, inference, SDK installation or global
configuration change. Port 4324 and all fixture data/build outputs belong to
the tickets worktree.

The ignored `work_task::engine::desk::tests::pi_desk_browser_fixture` uses the
accepted protected router, real migrated database and `AppState::new_for_test`.
It seeds only an empty folder, with no saved agent choice. It avoids the normal
server startup that installs bundled global skills. The fixture exited 0 after
the browser closed and its stop file was created (one manual fixture passed).

`pi-acp` is an **availability-only fixture**, answering `--version` with 0.0.33
and refusing all other invocations. The actual adapter command is unavailable
in this environment. This lets the real connect path reach the real installed
Pi 0.85.1/adapter 2.32.1 launcher probe. An empty, isolated Pi catalogue makes
that probe fail before ACP process construction or a model prompt. This does
not establish a working installed pi-acp session. The separate extracted-assets
fixture tests actual Pi RPC and adapter discovery.

`guard.js` keeps real localhost preference/connect routes, refuses prompt and
non-Pi connection requests, and counts them. Its final counters are in
`network-counts.json`: three Pi connect-only attempts, zero prompt requests,
zero other-agent launch requests and zero off-origin requests in the inspected
workspace page. The initially opened Settings tab was closed without edits.

## Observed sequence

1. Opened a fresh browser context, logged in with the public synthetic token
   `pi-desk-browser-fixture`. `01-login.yml` records the login form.
2. With no saved choice, Pi was selected and Send was disabled. The actual
   structured setup error initially appeared as `[object Object]`
   (`02-fresh-workspace.yml`, `02-fresh-before-error-fix.png`).
3. After reusing Codeg's existing `toErrorMessage` in the ACP context and
   lifecycle hook and rebuilding the frontend, the Pi setup error was rendered
   correctly (`04-pi-default-setup-error.yml/.png`). The banner is one line and
   visually truncated; the snapshot contains its full accessible name.
4. Clicking that banner **opened a new Settings tab**, observed by the CLI at
   `http://127.0.0.1:4324/settings/agents?agent=pi`, title
   `Settings - Hafidh Ops Desk`. `05-setup-settings-link.yml/.png` capture the
   original workspace after the click. They do **not** show expanded guidance
   or the contents of Settings. No setting or credential was changed.
5. Folder More options → Set default agent → Codex used the real preference
   endpoint (`06-explicit-choice-menu.yml`). The existing Pi tab retained its
   prior selection after reload (`07-existing-pi-tab-after-folder-change.*`).
6. Folder New Conversation selected Codex (`08-new-conversation-codex.*`).
   Reload retained Codex (`09-codex-preserved-after-reload.*`). The visible
   missing Codex ACP adapter message and disabled Send were preserved; no
   fallback, installation or model prompt was attempted.

## Reproduction

From this worktree, build the frontend with `pnpm build`. Ensure the real owned
companion is rebuilt **after** desktop checks:
`cargo build --locked --no-default-features --bin codeg-mcp` in `src-tauri`.
Create `src-tauri/target/pi-desk-browser/{empty-pi,data}` and remove only this
fixture's prior `stop` file before starting another run. Then, from `src-tauri`:

```sh
env PATH="$PWD/../reports/pi-desk-evidence:$PATH" \
  PI_CODING_AGENT_DIR="$PWD/target/pi-desk-browser/empty-pi" \
  CODEG_DATA_DIR="$PWD/target/pi-desk-browser/data" \
  CODEG_MCP_BIN="$PWD/target/debug/codeg-mcp" \
  cargo test --locked --no-default-features --lib pi_desk_browser_fixture \
  -- --ignored --nocapture
```

In another terminal at the worktree root:

```sh
playwright-cli -s=pi-desk-final open about:blank --headed
playwright-cli -s=pi-desk-final run-code --filename=reports/pi-desk-evidence/guard.js
playwright-cli -s=pi-desk-final goto http://127.0.0.1:4324
playwright-cli -s=pi-desk-final snapshot
```

Use current snapshot refs for the steps above; do not submit a chat prompt.
Close only `pi-desk-final`, then create
`src-tauri/target/pi-desk-browser/stop` to let the fixture exit and clean up its
temporary database. No live client setup or mobile guidance assessment is
claimed by these artifacts.
