# Running badge contrast

Complete and ready for root review in draft [PR18](https://github.com/Adanmohh/codeg/pull/18). Tested product/export commit: **b531501e281d093fb407580bac5c1593ac882b71**; subsequent changes are report/evidence only. Worktree `/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand`; branch `fix/design-running-badge`, clean-check/fetched accepted base **49e75d19483dc4b6851b22e04d73b66326c9aa47**. The sole product correction is `text-amber-700` → existing `text-amber-800` in `FolderHeader`'s running-count badge, `src/components/conversations/sidebar-conversation-list.tsx:491–518`, plus its inaccurate contrast comment. Count, zero-count omission, title/screen-reader label, sizing, hover/focus behavior and `dark:bg-amber-400/15 dark:text-amber-300` remain unchanged. No other product, dependency, lockfile or protected-document edit.

Baseline actual Playwright CLI measurements on the released existing 4326 fixture confirm light failures: **4.39:1 rest/focus**, **3.75:1 hover**. Root's **4.38:1** used fractional effective-background channels; this probe uses Canvas 8-bit pixels, yielding RGB(250,238,220) at rest. The conclusion is identical. Installed amber-800 produces RGB(151,60,0), measuring **6.19:1 rest/focus** and **5.29:1 hover** on the same actual backgrounds. This candidate was measured before the product edit. Baseline dark passes at **9.09:1 rest/focus**, **6.94:1 hover**.

Actual rebuilt-export measurements at **1280×900** confirm **six checked, zero contrast failures**, using the same [CLI script](design-running-badge.playwright) and [Design Studio formatter](design-running-badge-report.mjs). The badge stays **10px**, weight **600**; all six checks use the normal-text **4.5:1** threshold. Raw [before](design-running-badge-before.raw) / [after](design-running-badge-after.raw) retain real computed styles, every ancestor background, focus-visible state and accessible name. The probe rejects unaccounted ancestor opacity or background images. [Before measurements](design-running-badge-before-measured.json) retain all failures; [after measurements](design-running-badge-after-measured.json) contain the passing rebuilt results.

| Theme/state | Before | Rebuilt after | After foreground / effective background |
| --- | --- | --- | --- |
| Light rest | 4.39 | **6.19** | RGB(151,60,0) / RGB(250,238,220) |
| Light hover | 3.75 | **5.29** | RGB(151,60,0) / RGB(233,221,202) |
| Light keyboard focus | 4.39 | **6.19** | RGB(151,60,0) / RGB(250,238,220) |
| Dark rest | 9.09 | **9.09** | RGB(255,210,48) / RGB(58,47,20) |
| Dark hover | 6.94 | **6.94** | RGB(255,210,48) / RGB(76,65,37) |
| Dark keyboard focus | 9.09 | **9.09** | RGB(255,210,48) / RGB(58,47,20) |

All twelve screenshots are in [browser-running-badge](browser-running-badge/), including [before light](browser-running-badge/before-light-rest.png), [after light focus](browser-running-badge/after-light-focus.png) and [after dark hover](browser-running-badge/after-dark-hover.png), inspected directly. Keyboard Tab entry verifies `:focus-visible` and the inherited inset ring in both themes. The button's accessible name/expanded state, count and hidden text/title are identical across all cases. No folder activation/collapse was needed.

Source mapping: the exact inherited badge and comment were read from Codeg **v0.30.4 / 6f6bd648b206412644842a98d9ffeebf57292bed**, `src/components/conversations/sidebar-conversation-list.tsx:491–519` (Apache-2.0). The existing **tailwindcss@4.1.18** `theme.css:33–44` defines all amber tokens; no new palette or dependency source is imported. A precise modification attribution is appended to NOTICE, retaining the original Apache LICENSE and every existing notice. Probe glue adapts this repository's accepted `reports/design-ops-probe.playwright`; the formatter uses the local Design Studio **55c8614dcfff33b4caa5a544b4f1f91877214878** `lab/tools/probe.mjs` functions by import. No AGPL/enterprise/PolyForm source.

Docs-first: complete founding/orchestration/status/decisions/AGENTS documents and applicable code-context/Playwright/design-audit methods were read. Code-context ran through the existing RAG `.venv/bin/python`, `HF_HUB_OFFLINE=1`: guide exit 0 (measure first; actual headed browser checks), dependency docs exit 3 because `data/code/rebrand.db` is absent. Direct installed source supplies that missing coverage; no corpus or environment changes. Additional local authority: **next-themes@0.4.6** media-query handling and existing `defaultTheme="system"`; **TypeScript@5.8.3** Canvas/DOM types; **@types/node@25.2.2** `fs.d.ts` UTF-8 `readFileSync` overload (types, not the Node runtime version); installed **Playwright CLI 0.1.18** and Next build help. No remote/latest documentation was needed; source pins remain unchanged and hooks remain enabled.

Fixture: `http://127.0.0.1:4326/workspace`, headed worker CLI session `ops-running4326`, existing server/data, owned `out-design-ops/`. The existing Follow system preference is exercised through media emulation and checked against the real root class/color scheme; no injected theme class or saved appearance change. The count is **6** and the accessible name remains **6 sessions running**. Authenticated read-only test stats are identical before/after, provider requests **4→4**. No save, send, reseed, engine action or other fixture/output change.

Live docs-first evidence: own session `01a07c1c-cf2e-73e1-bbe3-e758c8363042`, exact worktree, PreToolUse **10384** / PostToolUse **10368** in `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`. Hooks remained enabled.

| Command | Exit/result |
| --- | --- |
| `CODEG_EXPORT_DIR=out-design-ops pnpm exec next build` | **0**, 33 static pages; `design-running-badge-build.log` |
| `pnpm exec tsc --noEmit` after build | **0**; `design-running-badge-tsc.log` |
| `pnpm exec vitest run src/components/conversations/sidebar-conversation-list.test.tsx` | **0**, all **43 existing tests**; `design-running-badge-tests.log` |
| `pnpm exec eslint src/components/conversations/sidebar-conversation-list.tsx` | **0**; `design-running-badge-eslint.log` |
| `playwright-cli -s=ops-running4326 --raw run-code --filename=reports/design-running-badge.playwright` | **0** before and rebuilt after; six states each |
| `node reports/design-running-badge-report.mjs reports/design-running-badge-before.raw` and corresponding `after.raw` | **0** each; Design Studio results above |
| `node /Users/mohamedadan/projects/design-studio/scripts/design-lint.mjs src/components/conversations/sidebar-conversation-list.tsx` | **0**, one source-only advisory below |
| `git diff --check`, clean check/fetch/branch creation, initial commit/push and `gh pr create --draft` to main | **0** |

Existing tests cover sidebar rendering, keyboard focus/paging, running counts and zero-count omission; none were changed or added. No unrelated full-suite/Rust rerun for this single CSS utility. Logs are local ignored files; scripts, raw results, screenshots and report are committed.

Design Studio aesthetic/a11y methods were applied sequentially inline, with no extra agent or paid executor. Badge Color/Detail and Accessibility last-mile micro-label contrast checks pass; default brief tokens and the inherited amber running meaning remain. The [source linter](design-running-badge-lint.raw)'s low **MOT-9** advisory cannot see generated Tailwind media rules: [actual rebuilt CSSOM](design-running-badge-hover-guard.raw) shows this exact folder-hover selector under **`(hover: hover)`**. No hover-guard fix is needed. [Scoped review](design-running-badge-review.json) records the disposition. This is a badge verdict, not full-app, every custom theme, mobile/native or motion certification; root's final reviewer remains independent.

Keep **4326** running for review, serving **out-design-ops/** at **b531501e** with the existing server/data and nonsecret test token **ops-design-synthetic-operator**. Worker browser work is finished. The inherited fixture-folder 404 diagnostics and unavailable Pi warning remain outside this seam. No live provider/inference request or change to another fixture/output.
