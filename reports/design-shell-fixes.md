# Shell accessibility and Pi setup fixes

Complete bounded correction, 2026-09-08. The five accepted specialist findings and root's additional mobile Settings menu finding are addressed. Draft PR: https://github.com/Adanmohh/codeg/pull/13. Branch: `fix/design-shell`.

**Final product commit: `e29cab3e6126fd0a0bf59dc9b63b52b625e3a67d`.** The final handoff commit contains this report and artifacts only; its exact head is recorded in the PR body. First product checkpoint: `f8ba45973925f5e78331de1d47e913f5a56cb1f5`; early report checkpoint: `8bb283cc`. Base: accepted main `ba93e87200b16d9fd15411b885f8c852b92078c0`, including PR11 review merge `625d8ed7266ed4cf49454942f3dbb5c48409bd07`.

## Corrections

1. **High — Pi field associations.** `pi-config-panel.tsx` now binds existing Provider, Model, Thinking and API Key labels with `useId`, `htmlFor` and labelled Select triggers. Owner-expanded custom-provider ID/API protocol/base URL fields have the same associations. Available reasoning levels form a named group; wire-value inputs retain their full explicit names and gain visible label associations. The disclosure exposes expanded state. Actual rendered selectors find every requested control, including the revealed high wire-value input. Model's empty placeholder is `gpt-6-astra`; saved values remain intact. [Standard fields](design-shell-evidence/09-named-config-desktop.json), [custom checks](design-shell-evidence/10-custom-provider.json), [ARIA](design-shell-evidence/10-custom-provider.yml), [mobile provider](design-shell-evidence/11-custom-mobile-provider.png), [mobile reasoning](design-shell-evidence/11-custom-mobile-reasoning.png).

2. **High — terminal and empty-alert names.** The existing translated New Terminal shortcut label names the new-tab button. Alerts retains its translated name with or without a count and gains an explicit focus ring. Actual ARIA contains both names. The popup was cleared through its existing in-memory action: the trigger still reports Alerts, with no count and visible No alerts text. [ARIA](design-shell-evidence/05-after-light.yml), [empty facts](design-shell-evidence/14-empty-alerts.json), [empty popup](design-shell-evidence/14-empty-alerts.png). No terminal was started.

3. **High — inherited shell contrast.** Section labels and six empty-state templates now use `text-sidebar-foreground/70`; desktop/mobile status parents use `text-foreground/75`. Existing populated folder/group ink remains unchanged and passes. No new palette or design system.

4. **Medium — readable setup recovery and readiness guidance.** Both composer layouts share the extracted `AgentSetupNotice`: wrapping full reason, alert role and explicit translated Open Agents settings button. Optional Diagnose retains its callback. At 390px the reason has equal scroll/client dimensions: **332px wide, 100px high**; the action is **44px high**, document width 390px. Initial Shift+Tab/Enter reached `/settings/agents?agent=pi`; final Enter activation correctly reused the existing named Settings window. This is navigation, not expansion. [Light phone](design-shell-evidence/06-after-mobile-light.png), [unoccluded text facts](design-shell-evidence/06-mobile-unoccluded.json), [dark phone](design-shell-evidence/13-after-mobile-dark.png), [final recovery](design-shell-evidence/13-final-recovery.json).

The panel explains the existing pi 0.85.1 / adapter 2.32.1 / Astra/max requirements, native client configuration and retry through a new conversation. Adapter PASS is explicitly distinct from model/provider readiness. Generic saved Thinking preferences are preserved; this legacy editor's Off through Extra high options are not represented as Desk's max readiness check. The unchanged accepted launcher sets max and refuses fallback. Four guidance keys were added only within each of ten Pi translation namespaces because the current loader does not merge English keys. Locale loading/RTL and unrelated copy remain with their owner.

5. **Medium — setup text contrast.** The scoped destructive tint/border uses existing `text-foreground`, following the accepted paired foreground pattern. Both themes pass. Global destructive tokens and unrelated banners are unchanged.

6. **High — mobile Settings menu, root follow-up.** Root found the unnamed 32x32 control in `settings-shell.tsx:238` before the fix, corroborated by this worker's actual capture. It now uses the existing translated Navigation label, expanded/dialog semantics and a 44x44 target in a 48px mobile header. Actual rectangle: x12/y1.5/44x44. Open, current-page selection, Escape close and outside-press close pass; route remains `/settings/agents?agent=pi`. Navigation/dismissal handlers are unchanged. [Before](design-shell-evidence/07-before-menu-fix.json), [final facts](design-shell-evidence/08-mobile-navigation.json), [open](design-shell-evidence/08-mobile-navigation-open.png), [closed with full guidance](design-shell-evidence/08-mobile-navigation-closed.png).

## Contrast

| Text surface                                          | Before light | After light | Before dark | After dark |
| ----------------------------------------------------- | -----------: | ----------: | ----------: | ---------: |
| Folders / Chat / Recent                               |         3.71 |        7.54 |        5.09 |       8.81 |
| No chats / No conversations / No recent conversations |         2.66 |        7.54 |        4.05 |       8.81 |
| Status inherited text                                 |         4.35 |        8.90 |        5.86 |       8.75 |
| Setup reason                                          |         4.42 |       18.31 |      4.57\* |      18.17 |

Ratios include composited backgrounds. \*This run's dark before banner was hovered (`hover:text-destructive/80`); accepted PR11's unhovered before sample is 6.57. These different states are disclosed. Sidebar folder-name samples remain 9.12 light / 9.96 dark. The fresh fixture has no populated stats widget: status evidence measures the real inherited parent color, grounded in unchanged `StatusBarStats` source, without injecting records or DOM content.

[RGB/ratios](design-shell-evidence/contrast-summary.json), [light before](design-shell-evidence/01-before-light.png), [light after](design-shell-evidence/05-after-light.png), [dark before](design-shell-evidence/02-before-dark.png), [dark after](design-shell-evidence/12-after-dark.png). The browser helper converts Lab/OKLCH through Canvas, composites backgrounds and ranks only samples without group opacity or images. `summarize.mjs` calls Design Studio's existing pure `contrastRatio` helper.

## Exact source-to-port mapping

All inherited UI here is Codeg **v0.30.4**, immutable **`6f6bd648b206412644842a98d9ffeebf57292bed`**, Apache-2.0. `gh api repos/xintaofei/codeg/git/ref/tags/v0.30.4` resolved that commit; official appearance/alerts/LICENSE reads used `gh api` at the SHA. Other exact original files were available through local `git show SHA:path`, alongside complete/current relevant components. Original LICENSE was read and compared with the fetched licence (equal after trimming terminal whitespace); LICENSE itself is unchanged. NOTICE appends exact attribution and retains all preceding sections.

| Codeg source paths under `src/`                                                      | Local modification or reuse                                                          |
| ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------ |
| `components/terminal/terminal-tab-bar.tsx`, `i18n/messages/*.json`                   | Existing New Terminal label on the same button/callback                              |
| `components/layout/{status-bar,status-bar-alerts,status-bar-stats}.tsx`              | Trigger name/focus and inherited status ink; stats logic untouched                   |
| `components/conversations/{sidebar-section-header,sidebar-conversation-list}.tsx`    | Existing sidebar foreground tokens with sufficient opacity                           |
| `components/conversations/conversation-detail-panel.tsx`, `components/ui/button.tsx` | Extract original setup message/actions into `components/chat/agent-setup-notice.tsx` |
| `components/settings/pi-config-panel.tsx`, `components/ui/{input,select}.tsx`        | Bind existing form labels; scoped guidance/placeholder; preserve state/save logic    |
| `components/settings/settings-shell.tsx`, `components/layout/app-title-bar.tsx`      | Name/size existing mobile navigation button/header using existing props              |
| `lib/api.ts` (`openAppWindow`, `openSettingsWindow`)                                 | Unchanged named-window reservation/reuse contract verified for final Enter check     |

New code is presentation/test/report glue. No additional borrowed product, restricted source, dependency, lockfile or migration. Pi requirements came from accepted `src-tauri/src/acp/pi_desk.rs`, `integrations/pi-desk/{launch.mjs,index.ts}` and the existing integration contract. They retain pi-mono v0.85.1 `d981de1229ef899957bbe968bc8dcda02a21f477` and adapter v2.32.1 `10a45367e033a32026987a75d6f401e37340c86f`; this correction does not silently select another model/package version.

## Docs first

Completely read current FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS, accepted specialist report, brief and Pi contract. Applied code-context, design-audit/checklist and Playwright CLI methods from their actual skills. Design Studio local commit: `55c8614dcfff33b4caa5a544b4f1f91877214878`; actual specialist definitions/checklists and `lab/tools/probe.mjs` supplied the method, not a paid judge or additional agent.

Code-context ran through existing rag-skills `.venv/bin/python` with `HF_HUB_OFFLINE=1`. Guide exit 0: headed CLI verification and installed-version-first rules. Docs exit 3: `rag-skills/data/code/tickets.db` absent. No repository-corpus coverage or ingestion is claimed. Direct pinned reads supplied missing coverage:

- React **19.2.4** package/source `useId`; **@types/react 19.2.13** `LabelHTMLAttributes.htmlFor` and ARIA definitions.
- **radix-ui 1.6.0**, installed Select **2.3.1** `dist/index.d.ts`/`index.mjs` forwarded trigger IDs/ARIA; existing shared Input/Button/Select, SettingsShell/Drawer/AppTitleBar source.
- **Tailwind 4.1.18** manifest and `dist/chunk-CT46QCH7.mjs`: `wrap-anywhere` maps to `overflow-wrap:anywhere`; **next-intl 4.8.3** client translation types and current locale contract.
- **@testing-library/react 16.3.2** render/export types, DOM **10.4.1** role-query types, **Vitest 2.1.9** mocking/runner `each` source. Additional official `gh api` reads: RTL v16.3.2 `f32bd1b033d5e3989ae1cb490d515ce389c54e53` (`types/index.d.ts`); Vitest v2.1.9 `c9e59a089d94642eea29a43f2ee1986a5afb99c6` (`packages/vitest/src/integrations/vi.ts`).
- Installed **Playwright CLI 0.1.18**, Playwright **1.63.0-alpha-2026-08-05** help/types; **Node 24.19.0**, **@types/node 25.2.2** fs read/write types for evidence helpers. No global update/install.

Live hooks show exact tickets cwd and session `01a07c1c-d82f-7022-84db-778a438632f1`, both PreToolUse and PostToolUse, including audit lines 7231–7287. Only safe metadata is copied into [hook-records.json](design-shell-evidence/hook-records.json). Hooks remained enabled; no bypass/command approval. All remote research used `gh api` at resolved immutable refs; no web/latest-source substitutions.

## Validation and isolation

| Command/check                                                                                            | Exit and observed result                                             |
| -------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `pnpm exec vitest run` with six files below                                                              | 0; **171 tests passed**, six files                                   |
| `pnpm exec eslint` on all eleven initial changed TSX/test files                                          | 0                                                                    |
| Focused ESLint after two test-harness corrections                                                        | 0                                                                    |
| `pnpm exec tsc --noEmit` after corrections                                                               | 0                                                                    |
| `pnpm build`                                                                                             | 0; Next 16.1.6 static production export                              |
| `pnpm exec eslint src/components/settings/settings-shell.tsx`                                            | 0 after menu follow-up                                               |
| `pnpm exec tsc --noEmit`, then `pnpm build` after menu follow-up                                         | Both 0; final export used for menu/custom/dark/final recovery checks |
| `node reports/design-shell-evidence/summarize.mjs`                                                       | 0; actual before/after color calculations                            |
| Headed `playwright-cli -s=design-shell` accepted scoped captures                                         | 0; evidence linked above                                             |
| Owned `cargo test --locked --no-default-features --lib pi_desk_browser_fixture -- --ignored --nocapture` | 0; **1 passed**, listener stopped via its own stop file              |
| `git diff --check`; protected-path comparison to base                                                    | 0; no protected product/planning/lock changes                        |

Exact Vitest files: `src/components/settings/{pi-config-panel,acp-agent-settings}.test.tsx`, `src/components/layout/status-bar-alerts.test.tsx`, `src/components/chat/agent-setup-notice.test.tsx`, `src/components/conversations/sidebar-section-header.test.tsx`, `src/lib/pi-thinking.test.ts`. Tests verify preserved saved values, explicit save arguments with no replacement API key, no automatic install/env/config write, revealed custom controls, recovery callbacks and both alert states.

Initial red run found missing names and a harness mistake: Vitest spreads array rows in `it.each`. The first expanded run had 170 passes and one harness failure. Initial typecheck caught that issue and unsupported Testing Library `exact` options. Both were corrected from pinned types/source; final 171-test run and typecheck pass. No product behavior was changed to accommodate them. Full logs remain ignored: `reports/design-shell-{red,tests,lint,typecheck,build}.log` and `reports/design-shell-menu-{lint,typecheck,build}.log`.

Fixture: only own temporary DB, `out/`, data directory and empty Pi catalogue on **127.0.0.1:4324**, with existing availability-only adapter wrapper. Context guard covers every page, including named Settings windows. Before context: **2** metadata preflights; after context: **7**. Both have **0 prompt requests, 0 off-origin HTTP requests, 0 blocked install/config/other-agent writes**. No API key entered. Synthetic custom-provider values stayed in component state and were discarded on navigation. [Before counts](design-shell-evidence/before-network.json), [after counts](design-shell-evidence/after-network.json).

No Rust product code changed; desktop/server Cargo checks and Clippy were not rerun for this frontend-only assignment. Owned real companion remained unchanged: SHA256 `58211aff505f32828595239370be3d3890ea59e862d8b0351b12c05f7d891972`. Own browser/listener are closed; no other fixture/worktree was changed.

Root separately reports nine focused tests passed at `f8ba4597`, then reviewed `e29cab3e`, contrast evidence and screenshots with no remaining blocker in this bounded correction. Its independent `ops-shell-independent` CLI verified 44x44 named menu, open/Escape states, settled drawer geometry, four Pi names, Astra placeholder and 390px width; all guard counters zero. These are owner-reported independent results, distinct from this worker's evidence. Root released its browser before this worker closed the listener.

## Limits and next task

Targeted source/browser evidence, not a whole-app WCAG certificate, real screen-reader/phone/native-webview audit, successful provider setup or model-readiness certification. Generic saved configuration and credential storage are unchanged. No global config, login, install, inference, live send, App installation or deployment.

Excluded exploratory attempts: one sidebar lookup omitted its shortcut suffix; repeated responsive remount/focus attempts reached a disconnected state with the inline error absent, so that repeated Shift+Tab assertion did not pass. Initial Shift+Tab/Enter evidence remains valid; no sticky-error guarantee across repeated remounts is asserted. Final Enter initially waited for a _new_ popup, but unchanged `openAppWindow` correctly reused the named window; final recovery evidence verifies that actual destination. Raw focus/churn observations are labelled as attempts, not passing focus evidence. The final fixed-viewport dark capture shows the full reason and disabled Send. Connection lifecycle logic was not changed.

P1 host merge `02e3f5d8a15a3fee792cd0625966dee6c500d184` is accepted per owner. After PR13 review/merge, next is the **separate** Pi P1 bridge in `reports/pi-p1-follow-on-checklist.md`: three scoped cached reads, real companion/adapter discovery and native `desk_propose_issue` through accepted host logic. No P1 source is mixed into this design PR. Planning docs, Ops/approval/task logic and locale-provider/RTL remain untouched.
