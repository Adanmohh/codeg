# Business native chrome — N2 checkpoint

Branch **fix/business-native-chrome**, draft [PR24](https://github.com/Adanmohh/codeg/pull/24),
from accepted main **e6eb7b56c76f0f162f5255857c4f8e0ab8f7d804**. The bounded
composition and focused tests are implemented; browser/export checks continue.
No corrected native-runtime result is claimed. Root owns final package/drag
verification. Existing fixtures, exports, targets and the untracked paused
visual report are preserved.

Root's committed finding, reports/business-native-root/N2-native-chrome.md and
startup/local-workspace screenshots, shows macOS traffic lights over the brand
and an inert business header drag. Root's later positive control moved the
engineering window from126,42 to206,67 using the blank titlebar at screen1000,62;
the earlier engineering600 point hit a conversation tab and is excluded. The
same pointer helper completed the actual local human workflow through Done4.
This correction changes only chrome, not that workflow or engineering access.

## Implementation contract

Reuse AppTitleBar above the business route's connect/bootstrap/workspace states.
Reserve the existing40px workspace strip (commands/windows.rs uses traffic-light
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
dependency or global token changes. Tests must cover all three scenes, web
absence, native platform controls and private-draft continuity. Actual browser
geometry will be labelled separately from root's real native drag verification;
Windows/Linux source/mock checks will not be claimed as executed platforms.

## Source and docs-first grounding

Read FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS completely, plus the committed
native finding and screenshots. Applied code-context and Playwright CLI skills.
Offline guide retrieval with the existing rag-skills venv exited0; results were
general workflow guidance, not a native-chrome rule. Installed corpus query
exited3 because rebrand.db is absent. No corpus or package was installed.

- Codeg Apache-2.0 v0.30.4, **6f6bd648b206412644842a98d9ffeebf57292bed**, resolved
  through gh api. src/components/layout/{app-title-bar,window-controls}.tsx and
  src/hooks/use-platform.ts are unchanged at the accepted main pin. Their
  implementation is imported intact. Read src/hooks/{use-mobile,use-media-query}.ts,
  src/app/project-boot/page.tsx, src/lib/window-chrome.ts and native windows.rs.
- Accepted business connection/state/layout sources at **e6eb7b56**:
  src/app/business/page.tsx and src/components/business/{connect,bootstrap,workspace}.tsx.
  No alternate session or authentication contract is introduced.
- React19.2.4 and @types/react19.2.13 useSyncExternalStore signature;
  @tauri-apps/api2.10.1 core.js isTauri and window.d.ts were read locally.
- Locked Rust tauri2.10.2 src/window/scripts/drag.js reads the actual event target's
  data-tauri-drag-region attribute before invoking start_dragging. Local source
  read; official tauri-v2.10.2 resolved via gh api to
  **06374a902a50d2bd8b8d85593623ad16ac32325a**. Existing capabilities already permit
  dragging/minimize/close/maximize. No dependency code is ported or upgraded.

NOTICE records the exact Codeg source-to-composition mapping. No AGPL/GPL source
or new provider calls. The native-chrome.test.tsx scene/client setup reuses the
accepted business test-fixtures.ts and workflow/session test patterns. Installed
Testing Library16.3.2 render/rerender types and Vitest2.1.9 stub/restore APIs were
read before using them.

## Focused checkpoint

- pnpm exec vitest run src/lib/business/client.test.ts src/components/business:
  exit0, **52 tests/6 files**,2.06s. Includes13 new chrome cases: all3 scenes on
  macOS/web; all3 states with Windows/Linux mocked control callbacks and cleanup;
  RTL caption placement and connection draft DOM identity across locale rerender.
  The existing39 business privacy/auth/workflow tests also pass.
- pnpm exec tsc --noEmit: exit0.
- Scoped eslint on the new layout/test and3 scene files: exit0 after formatting
  the new test. Initial lint reported only Prettier formatting; no product failure
  or assertions were changed to pass it.
- Logs: .build/business-native-chrome-{tests,typecheck,lint-final}.log. Isolated
  export and actual CLI geometry/drag-target checks remain pending. Windows/Linux
  checks use mocked OS APIs; root owns real macOS movement verification.
