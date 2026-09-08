# Rebrand test expectations

The two stale visible-brand failures are corrected. The final unchanged two-file retry passed **84/84 tests**. Earlier runs also encountered an unrelated transient Forge assertion; its failures and successful retries are recorded below.

Worktree: `/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand`. Branch: `fix/rebrand-test-expectations`, created after a clean check and fetch from accepted `origin/main` at `21069bf92c3f567202f85bbf03fd86b461761c13`. The two test files and English messages match root's full-suite head `8c2a004d` at this base. Initial implementation checkpoint; the pushed commit and draft PR will be recorded after creation.

Only three expected strings change. The real English `Forge.errors.wrongForge` message already names **Hafidh Ops Desk** (`src/i18n/messages/en.json:5503`). `forge-page.test.tsx:338` now expects that full message. Its successful-retry sibling's negative matcher at line 310 also uses the actual brand, so it continues to detect an incorrectly displayed error. The retry, successful rows and merge-request assertions remain intact. `AcpAgentSettings.openCode.permissions.invalidBlock` already uses the same brand (`en.json:1130`); `opencode-permissions-section.test.tsx:233` now expects that exact message, retaining the disabled auto-accept and enabled Reset assertions. Both harnesses render the real English messages through `NextIntlClientProvider`.

Source-to-change mapping is entirely within accepted commit `21069bf92c3f567202f85bbf03fd86b461761c13`: those two existing message keys supply only the displayed app name in the three existing matchers. No product code, translation, assertion behavior, fixture identity, internal `codeg` name, dependency, lockfile or protected planning document changes. There is no new external source port; existing Apache attribution and all NOTICE sections are preserved. Port 4326, its process and its tested export remain untouched.

Complete founding/orchestration/decision/status and AGENTS instructions were read. Docs-first used installed `@testing-library/react@16.3.2`, its `@testing-library/dom@10.4.1` `dist/queries/text.js` and `dist/matches.js`, `next-intl@4.8.3`, and `vitest@2.1.9` CLI help. Exact string matching and the existing regular-expression negative matcher were retained. The code-context skill ran with `/Users/mohamedadan/projects/rag-skills/.venv/bin/python` and `HF_HUB_OFFLINE=1`: `guide` exited 0, grounding the existing RED-to-GREEN correction in the actual product rebrand and version-specific sources (rules 2 and 6). The dependency `docs` query exited 3 because `data/code/rebrand.db` is absent; no corpus was created and local installed source supplied the missing coverage. No remote documentation research was needed. Live hook evidence in `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`: own session `01a07c1c-cf2e-73e1-bbe3-e758c8363042`, own worktree, PreToolUse line 9689 and PostToolUse line 9687; hooks remained enabled.

Validation on 2026-09-08 used these commands. Logs are local ignored artifacts under `reports/`.

| Command | Exit and result | Log |
| --- | --- | --- |
| `pnpm exec vitest run src/components/forge/forge-page.test.tsx src/components/settings/opencode-permissions-section.test.tsx` before edits | 1; 81 passed, 3 failed: both brand mismatches and the unrelated case below | `rebrand-test-before.log` |
| Same two-file command after edits | 1; 83 passed, only the unrelated case failed | `rebrand-test-after.log` |
| Same two-file command with `-t 'wrong-forge\|unknown shape'` | 0; all 3 affected tests passed, 81 skipped | `rebrand-test-focused.log` |
| `pnpm exec vitest run src/components/forge/forge-page.test.tsx -t 'keeps both comments when two land on an item the list no longer shows'` | 0; 1 passed, 63 skipped; case 1.150 s | `rebrand-test-existing-case.log` |
| Original two-file command, unchanged retry | 0; **84 passed**, 2 files; 5.95 s | `rebrand-test-retry.log` |
| `pnpm exec eslint src/components/forge/forge-page.test.tsx src/components/settings/opencode-permissions-section.test.tsx` | 0 | `rebrand-test-eslint.log` |
| `pnpm exec tsc --noEmit` | 0 | `rebrand-test-tsc.log` |
| `git diff --check` | 0 | Terminal |

The unrelated case, `ForgePage writes from a panel with no row > keeps both comments when two land on an item the list no longer shows`, failed before and after the brand edit at line 2110: the initial searched row remained during the existing 2500 ms wait. It failed before either comment-order assertion ran. Source inspection confirmed a 350 ms search debounce (`forge-page.tsx:152,967–973`); the test's existing comment describes timing sensitivity, and an adjacent drawer test documents focus settling. Those are possible timing contributors, not a proven cause. The exact unchanged case then passed alone and in the complete two-file retry (964 ms in that retry). This establishes transient local behavior, not a fixed timing defect. No timeout, mock, assertion or product change was made for it.

Root's `/tmp/ops-phase1-frontend-tests.log` recorded 6156 passes and the two assigned brand failures across 433 files; the unrelated case passed there. The full suite belongs to root's post-merge verification. This worker did not rerun it or unchanged Rust/browser gates for an expectation-only patch. Vitest's existing Vite CJS deprecation warning remains.
