# Business native chrome — N2 handoff

Branch **fix/business-native-chrome**, draft [PR24](https://github.com/Adanmohh/codeg/pull/24),
from accepted main **e6eb7b56c76f0f162f5255857c4f8e0ab8f7d804**. Product and test
checkpoint **3d0008747ab1933bf9c1329f8e1b59d949992623** is pushed and frozen for
independent review. Worker checks are complete: 52 focused tests, typecheck,
scoped lint, default desktop check and isolated export pass. This final update
contains only this report and existing gate logs.

Root explicitly owns the remaining integrated Playwright CLI check and actual
native connect/bootstrap/workspace chrome, pointer drag and 400px runtime
verification. No corrected native-runtime pass is claimed here. Root narrowed
the worker handoff before any new preview/browser started; no native emulation
matrix or repeated gates were run. Prior fixtures and exports remain untouched.

Root's [committed finding](business-native-root/N2-native-chrome.md) and
startup/local-workspace screenshots show macOS traffic lights over the brand
and an inert business header drag. The valid engineering positive control moved
the window from (126,42) to (206,67), starting on the blank titlebar at screen
(1000,62). The earlier engineering point at x600 hit a conversation tab and is
excluded. Root's successful local/shared-member workflows are separate evidence;
this correction changes only the chrome around those existing flows.

## Implementation contract

Reuse AppTitleBar above the business route's connect/bootstrap/workspace states.
Reserve the existing 40px workspace strip (commands/windows.rs uses traffic-light
Y22); retain the source's macOS inset and Windows/Linux WindowControls. Keep
physical caption controls left-to-right while the business content retains RTL.
Use the existing native detection with an SSR-safe snapshot. The frame stays
outside the private session subtree; each existing scene consumes the remaining
height instead of adding another viewport. Web renders no extra titlebar.

Product files: new src/app/business/layout.tsx composes the native-only chrome;
connect.tsx, bootstrap.tsx and workspace.tsx change only h-dvh to h-full. No
changes to BusinessPage's session key, any API client or existing shared chrome.
The native frame is stable across locale rerenders; physical controls remain LTR.

No shared chrome implementation, session/auth, Rust, engine, navigation guard,
dependency, lockfile or global token changes. Windows/Linux controls are imported
unchanged; their test callbacks are mocked OS APIs, not executed platform checks.

## Source and docs-first grounding

Read FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS completely, plus the committed
native finding and screenshots. Applied code-context and Playwright CLI skills.
Offline guide retrieval with the existing rag-skills venv exited 0; results were
general workflow guidance, not a native-chrome rule. Installed corpus query
exited 3 because rebrand.db is absent. No corpus or package was installed.
The docs-first audit has live PreToolUse and PostToolUse records at timestamp
1788887853 for session **01a07c1c-cf2e-73e1-bbe3-e758c8363042**, cwd this worktree,
both exit 0. Hooks stayed enabled.

- Codeg Apache-2.0 v0.30.4, **6f6bd648b206412644842a98d9ffeebf57292bed**, resolved
  through gh api. src/components/layout/{app-title-bar,window-controls}.tsx and
  src/hooks/use-platform.ts are unchanged at the accepted main pin. Their
  implementation is imported intact. Read src/hooks/{use-mobile,use-media-query}.ts,
  src/app/project-boot/page.tsx, src/lib/window-chrome.ts and native windows.rs.
- Accepted business connection/state/layout sources at **e6eb7b56c76f0f162f5255857c4f8e0ab8f7d804**:
  src/app/business/page.tsx and src/components/business/{connect,bootstrap,workspace}.tsx.
  No alternate session or authentication contract is introduced.
- React **19.2.4** and @types/react **19.2.13** useSyncExternalStore signature;
  @tauri-apps/api **2.10.1** core.js isTauri and window.d.ts were read locally.
  Next **16.1.6** installed client-layout/children source and the existing
  CODEG_EXPORT_DIR build contract were read; Tailwind remains **4.1.18**.
- Locked Rust tauri **2.10.2** src/window/scripts/drag.js reads the actual event target's
  data-tauri-drag-region attribute before invoking start_dragging. Local source
  read; official tauri-v2.10.2 resolved via gh api to
  **06374a902a50d2bd8b8d85593623ad16ac32325a**. Existing capabilities already permit
  dragging/minimize/close/maximize. No dependency code is ported or upgraded.

NOTICE records the exact Codeg source-to-composition mapping. No AGPL/GPL source
or new provider calls. The native-chrome.test.tsx scene/client setup reuses the
accepted business test-fixtures.ts and workflow/session test patterns. Installed
Testing Library **16.3.2** render/rerender types and Vitest **2.1.9** stub/restore APIs were
read before using them.

## Validation and reproducible handoff

Commands ran in this worktree, except cargo in src-tauri/. Existing output is
committed under [business-native-chrome-evidence](business-native-chrome-evidence/),
with terminal colors/trailing whitespace removed. Exit statuses were captured
from the completed processes; original logs remain in .build/.

| Command | Exit / result | Log |
| --- | --- | --- |
| `pnpm exec vitest run src/lib/business/client.test.ts src/components/business` | 0; **52 tests / 6 files**, 2.06s | [focused-tests.txt](business-native-chrome-evidence/focused-tests.txt) |
| `pnpm exec tsc --noEmit` | 0 | [typecheck.txt](business-native-chrome-evidence/typecheck.txt) |
| Scoped `pnpm exec eslint` on the files below | 0 | [lint-final.txt](business-native-chrome-evidence/lint-final.txt) |
| `CARGO_TARGET_DIR=../.build/business-gates-target cargo check --locked --offline --jobs 4` | 0; default desktop, 14.12s | [desktop-check.txt](business-native-chrome-evidence/desktop-check.txt) |
| `CODEG_EXPORT_DIR=.build/business-native-chrome-export NEXT_TELEMETRY_DISABLED=1 pnpm build` | 0; 34 static routes | [export.txt](business-native-chrome-evidence/export.txt) |

Scoped eslint files: src/app/business/layout.tsx and
src/components/business/{connect,bootstrap,workspace,native-chrome.test}.tsx.
Initial lint exited 1 with 45 Prettier diagnostics in the new test only
([initial log](business-native-chrome-evidence/lint-initial.txt));
`pnpm exec eslint --fix src/components/business/native-chrome.test.tsx` exited 0
([format log](business-native-chrome-evidence/format-test.txt)). No behavior or
assertions were changed; the final scoped lint passed.

The 13 new cases exercise all three real scenes on macOS/web, each scene's
unchanged Windows/Linux control callbacks and cleanup, RTL caption placement,
and native connection draft DOM identity across locale rerender. The existing
39 business privacy/auth/workflow tests also pass. Mocked tests verify component
composition and handlers; they do not verify macOS hit testing or OS movement.

Desktop check reports the existing missing codeg-mcp sidecar placeholder and
proc-macro-error2 future-compatibility warning. It is not a packaged-app check.
No Rust changed, so there was no new server/Clippy run; root owns final packaging.

New isolated export:
`/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand/.build/business-native-chrome-export`.
Its business.html SHA256 is
**48d03e5426d35aac7d1eac260f698a60aead2d74c434b9197e04d8af56d9fb6d**,
built from frozen product 3d000874. No listener or browser was launched for this
export. Unused preview preparation is preserved only in ignored
.build/business-native-chrome-unused-fixture/; it is not delivered test evidence.
No requests or mutations were made to the earlier fixtures. Their exports and
processes were not restarted. The paused visual/research checkpoint hashes
remain unchanged; no provider, credential, engine or root app was used.
