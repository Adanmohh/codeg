# Step 1: rebrand

Worktree: /Users/mohamedadan/projects/_worktrees/ops-desk/rebrand
Branch: feat/step1-rebrand. One writer; no workers started.

## Progress

Read FOUNDING.md, ORCHESTRATOR.md, STATUS.md, DECISIONS.md, AGENTS.md and reports/step0.md completely. Implementing piece 8 only. Internal unsigned build; preserve codeg binary/module/storage names and dependency lockfiles.

## Sources and decisions

- Base: xintaofei/codeg v0.30.4, immutable SHA 6f6bd648b206412644842a98d9ffeebf57292bed, Apache-2.0.
- Existing paths read: src/components/app-icon.tsx; src/app/layout.tsx; src/app/globals.css; src/lib/theme-presets.ts; src/lib/appearance-script.ts; src-tauri/tauri.conf.json; src-tauri/icons/{icon.svg,macos-icon.gen.py,tray-icon-template.gen.py}; src-tauri/tests/macos_icon_geometry.rs; provider dialog/service; updater library/provider and Rust update entry points.
- Original code-native H icon, deep teal with warm gold; reuse existing Tauri raster/ICNS pipeline and theme tokens. No additional third-party port or AGPL source.
- Sponsor search found no presets in tagged runtime provider code; README sponsor promotions will be removed. User provider configurations remain intact.
- Code-context guide exit 0: no project-specific coverage. Applicable rule: “This is not the framework you remember — heed deprecations and breaking changes.” Docs retrieval exit 3: no rebrand.db corpus. Used installed Tauri CLI schema/help instead; no fabricated retrieval claims. Unrelated project-specific theme rules not applied.
- Owner latest-doc instruction acknowledged: any latest documentation research uses gh api at a recorded immutable ref. Installed pinned source remains first; borrowed versions do not change.

## Commands

- pnpm install --frozen-lockfile: exit 0, worktree-local node_modules, pnpm 11.9.0.
- Read gh help and gh pr create --help; pnpm install help; pnpm tauri icon --help.
- Validation, implementation commit SHA and draft PR URL: see checkpoint below.

## Limitations

Work in progress. No signing, deployment, store distribution, or external messages.

## Orchestrator checkpoint — paused for Codex docs-first hook reload

Owner explicitly requested a checkpoint and STOP before any more product edits.
Implementation is incomplete and not ready for review/merge. Resume only after
an explicit new instruction, with the reloaded Codex adapter. No workers spawned.

Completed so far:

- ProductName Hafidh Ops Desk, bundle identifier app.hafidh.opsdesk; Cargo description updated; internal package/binary/module/storage identifiers retained.
- Original code-native H vector replaces AppIcon and web/native SVG; Tauri CLI regenerated the tracked desktop PNG/ICO/ICNS files and browser icons. Existing macOS generator preserves safe-area inset.
- Tray generator now reuses the Tauri CLI subprocess pattern from the existing macOS generator instead of requiring Pillow; generated H alpha-template image retained.
- Default neutral theme ID retained with teal light/dark accents, so stored preference shape stays compatible; icon carries warm gold.
- Visible name substitutions in translation values, workspace/settings titles, notification titles, exports, desktop/tray labels.
- README sponsor blocks removed in all ten languages. No sponsor presets were found in src/ or src-tauri/ provider configuration at this baseline.
- NOTICE added with upstream Apache attribution; LICENSE and existing vendored notices unchanged. Append-only approach for future workers' entries.
- Internal update policy glue: no updater artifacts/config/plugin/capability; UI availability checks and install entry disabled; server manifest fetch rejects before networking. Existing updater tests opt into retained upstream lifecycle behaviour via mocked brand flag.

Exact source-to-change mapping: all inherited sources below are from
xintaofei/codeg v0.30.4 at 6f6bd648b206412644842a98d9ffeebf57292bed.
There are no outside-source ports or borrowed latest-version upgrades.

| Source | Change/port |
| --- | --- |
| src-tauri/tauri.conf.json; src-tauri/Cargo.toml; src-tauri/capabilities/default.json | Existing config adjusted for own branding and unsigned internal build |
| src/components/app-icon.tsx; public/icon.svg; src-tauri/icons/icon.svg | Original H vector replacing upstream art; no borrowed icon source |
| src-tauri/icons/macos-icon.gen.py | Existing Apache generator reused unchanged for ICNS |
| src-tauri/icons/tray-icon-template.gen.py + macos-icon.gen.py | Existing Tauri subprocess generation pattern adapted to original transparent H template |
| src/app/globals.css; src/lib/theme-presets.ts | Existing token/preset mechanism, teal accents |
| src/lib/updater.ts; src/components/providers/update-provider.tsx; src/components/settings/system-network-settings.tsx; src-tauri/src/update/version.rs; src-tauri/src/lib.rs | Minimal internal-build update disable glue; no new update system |
| README.md; docs/readme/README.*.md | Sponsor promotion removal and explicit upstream attribution |
| src/i18n/messages/*.json and source files listed below | Display text only; internal codeg protocol/module names retained |

Commands/results so far:

- code-context guide: 0; docs: 3 (missing rebrand.db), as recorded above.
- pnpm install --frozen-lockfile: 0. Installed Tauri CLI 2.10.0 schema/help consulted; TypeScript 5.8.3, Next 16.1.6, React 19.2.4 remain pinned.
- pnpm tauri icon src-tauri/icons/icon.svg -o reports/rebrand-generated-icons: 0. Scratch generated mobile variants moved to ignored out/rebrand-generated-icons; only previously tracked desktop assets copied into product paths.
- python3 src-tauri/icons/macos-icon.gen.py: 0.
- Initial tray generator via rag-skills Python: 1, Pillow unavailable. Replaced with existing Tauri generation pattern; python3 src-tauri/icons/tray-icon-template.gen.py: 0.
- pnpm exec prettier --write [edited TS/TSX/config files]: failed on missing closing JSX brace in system-network-settings.tsx. The active safe edit corrected that brace at checkpoint; formatter not rerun yet.
- pnpm exec tsc --noEmit: failed on the same JSX syntax error (TS1005). Not rerun after correction; no passing typecheck claim.
- Default desktop cargo check --locked: stopped intentionally with Ctrl-C, exit 130, for checkpoint. Was still compiling dependencies; no final gate result. Own src-tauri/target output only.
- git diff --check: 0. Dependency lockfile comparison: 0 (unchanged).
- Early report commit 6dc835f5 pushed to origin/feat/step1-rebrand.
- Draft PR: not yet opened; deferred by checkpoint. gh help and gh pr create --help already read.

Resume checklist:

1. Review current diff, rerun formatting and tsc; review neutral fallback CSS consistency before/after hydration. No new UI system needed.
2. Finish remaining user-facing lowercase codeg references, especially app-boot-loading.tsx and auxiliary page document titles (commit/import/merge/pet/project-boot/push/stash). Read files before editing. Do not blindly replace internal provider IDs, protocol URLs, cache paths or code snippets. Translation values still have some lowercase prose branding; new internalBuild notice currently English across locales and needs localisation.
3. Verify update disable behaviour and add meaningful focused tests for real disabled mode (existing upstream tests deliberately mock enabled mode); confirm Rust error guard compiles and no update endpoint is reachable through the internal UI. Review desktop plugin removal against runtime registration.
4. Run default desktop cargo check --locked, pnpm exec tsc --noEmit, focused Vitest coverage for changed branding/notification/updater/theme behaviour; Rust server cargo check --locked --no-default-features --bin codeg-server; meaningful Rust disabled-manifest test and existing macos_icon_geometry tests. No tests beyond initial failed typecheck have run yet.
5. Inspect generated icon visually, verify tracked assets and sponsor searches, preserve both lockfiles and existing notices. Runtime UI/browser verification not performed yet. Browser automation must use Playwright CLI.
6. Commit/push completed work, open small draft PR to main (never merge), record exact commit/PR and successful gate commands here. No deployment, signing or store release.

Current modified inherited files (all based on immutable upstream SHA above):

- `README.md`
- `docs/readme/README.ar.md`
- `docs/readme/README.de.md`
- `docs/readme/README.es.md`
- `docs/readme/README.fr.md`
- `docs/readme/README.ja.md`
- `docs/readme/README.ko.md`
- `docs/readme/README.pt.md`
- `docs/readme/README.zh-CN.md`
- `docs/readme/README.zh-TW.md`
- `public/icon-128x128.png`
- `public/icon-32x32.png`
- `public/icon.svg`
- `src-tauri/Cargo.toml`
- `src-tauri/capabilities/default.json`
- `src-tauri/icons/128x128.png`
- `src-tauri/icons/128x128@2x.png`
- `src-tauri/icons/32x32.png`
- `src-tauri/icons/64x64.png`
- `src-tauri/icons/Square107x107Logo.png`
- `src-tauri/icons/Square142x142Logo.png`
- `src-tauri/icons/Square150x150Logo.png`
- `src-tauri/icons/Square284x284Logo.png`
- `src-tauri/icons/Square30x30Logo.png`
- `src-tauri/icons/Square310x310Logo.png`
- `src-tauri/icons/Square44x44Logo.png`
- `src-tauri/icons/Square71x71Logo.png`
- `src-tauri/icons/Square89x89Logo.png`
- `src-tauri/icons/StoreLogo.png`
- `src-tauri/icons/icon.icns`
- `src-tauri/icons/icon.ico`
- `src-tauri/icons/icon.png`
- `src-tauri/icons/icon.svg`
- `src-tauri/icons/tray-icon-template.gen.py`
- `src-tauri/icons/tray-icon-template.png`
- `src-tauri/src/commands/windows.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/update/version.rs`
- `src-tauri/tauri.conf.json`
- `src/app/favicon.ico`
- `src/app/globals.css`
- `src/app/layout.tsx`
- `src/app/workspace/layout.tsx`
- `src/components/app-icon.tsx`
- `src/components/providers/update-provider.test.tsx`
- `src/components/providers/update-provider.tsx`
- `src/components/settings/acp-agent-settings.tsx`
- `src/components/settings/backup-settings.tsx`
- `src/components/settings/channel-events-tab.tsx`
- `src/components/settings/desktop-notification-settings.test.tsx`
- `src/components/settings/settings-shell.tsx`
- `src/components/settings/system-network-settings.tsx`
- `src/contexts/acp-connections-context.tsx`
- `src/contexts/tasks-view-context.test.tsx`
- `src/contexts/tasks-view-context.tsx`
- `src/i18n/messages/ar.json`
- `src/i18n/messages/de.json`
- `src/i18n/messages/en.json`
- `src/i18n/messages/es.json`
- `src/i18n/messages/fr.json`
- `src/i18n/messages/ja.json`
- `src/i18n/messages/ko.json`
- `src/i18n/messages/pt.json`
- `src/i18n/messages/zh-CN.json`
- `src/i18n/messages/zh-TW.json`
- `src/lib/export-conversation.ts`
- `src/lib/theme-presets.ts`
- `src/lib/updater.test.ts`
- `src/lib/updater.ts`

New product files: `NOTICE`, `src/lib/brand.ts`.

Checkpoint implementation commit: `a743e99fd88350ffc262b27d9849a7b57625e1c3` (pushed). This report-only follow-up records that immutable checkpoint. Draft PR remains pending until resume.
