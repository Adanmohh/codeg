# Playwright CLI baseline — merged rebrand and tickets

Reviewed main product at merge `d5a7127187cd107d2bdd6d00e7c7900e542b3903`; later commits at this run are documentation only. Production `pnpm build` and `cargo build --locked --no-default-features --bin codeg-server` both exited 0. Browser: installed Playwright CLI 0.1.18, named session `ops-desk-check`, real Axum server bound to localhost:4318, separate throwaway data directory `/tmp/ops-desk-browser-data`. Authentication used a local fixture token, not an existing user credential. No route mocks or live external actions.

## Checks actually performed

- Login heading/title show Hafidh Ops Desk. Invalid token stays on login with explicit invalid-token text; valid fixture token opens `/workspace`.
- Workspace loads on the real backend. To-dos navigation opens the empty task view, and New task opens a dialog with initial title focus. Saving an empty task keeps the dialog open and displays an alert, “Title is required.” Escape dismisses it.
- At 390×844 the sidebar becomes a dismissible mobile drawer. Escape closes it and reveals the task view. `document.documentElement.scrollWidth === innerWidth === 390`.
- Real Appearance settings changed the mode from Follow system to Dark, persisted across navigation back to workspace. The dark workspace also has scrollWidth 390 at viewport 390. Screenshots captured through Playwright CLI and inspected visually.

## Evidence

- [Desktop workspace](browser-step1/workspace-desktop.png)
- [Mobile task view](browser-step1/tasks-mobile.png)
- [Mobile dark workspace](browser-step1/workspace-mobile-dark.png)
- Local raw snapshots/console entries are in ignored `.playwright-cli/`; build logs `/tmp/ops-desk-browser-build.log` and `/tmp/ops-desk-server-build.log`.

## Limits and follow-up

The only console errors observed were the intentional invalid-token 401 and an `acp_describe_agent_options` 500 when the unconfigured default Claude ACP adapter was inspected by New task. The UI explains that the adapter is missing. No agent was launched, task executed, email sent or issue filed. Pi default/adapter wiring is still planned. No Ops email/approval UI exists yet, so these checks establish the inherited shell baseline, not end-to-end P1/P2 acceptance.

Accessibility snapshots show unnamed icon controls and some appearance selectors without explicit accessible labels; login uses a placeholder and renders its error as an ordinary paragraph. These are concrete review candidates for the final Design Studio loop, not a claim that all accessibility checks passed. Full measured/specialist audit and the new Ops flows remain pending. Native Tauri/WKWebView and a packaged binary were not tested.
