# Independent N2 business native chrome review

**No blocking source or focused-test finding at product
3d0008747ab1933bf9c1329f8e1b59d949992623 ([PR24](https://github.com/Adanmohh/codeg/pull/24)).**
The correction addresses the missing native business frame without changing
permissions, transport or the private session key. Independent tests passed
**23/23 across three files, exit 0**. Source review is complete; N2 native-runtime
closure remains root's rebuilt macOS clearance, drag and control check. These
jsdom results do not certify native movement.

Reviewed the complete commit/diff against owner base
**e6eb7b56c76f0f162f5255857c4f8e0ab8f7d804**, all 183 lines of the new tests,
relevant unchanged source/import paths and the owner's full report. This verdict
freezes that product head, independently of later owner report/export commits.
[Structured verdict](review-business-native-chrome-evidence/verdict.json).

## Source assessment

| Concern | Exact-head assessment |
| --- | --- |
| Native frame and controls | [business/layout.tsx:12](https://github.com/Adanmohh/codeg/blob/3d0008747ab1933bf9c1329f8e1b59d949992623/src/app/business/layout.tsx#L12) adds the existing AppTitleBar as a native-only 40px sibling above connect/bootstrap/workspace. The unchanged titlebar reserves 92px on macOS and 138px for Windows/Linux controls. Physical chrome is LTR; business content retains its inherited direction. |
| Drag semantics | [app-title-bar.tsx:49](https://github.com/Adanmohh/codeg/blob/3d0008747ab1933bf9c1329f8e1b59d949992623/src/components/layout/app-title-bar.tsx#L49) places the drag attribute on both overlay and actual row. The centered title ignores pointer events. The locked Tauri script checks the actual event target, not an ancestor. [WindowControls:79](https://github.com/Adanmohh/codeg/blob/3d0008747ab1933bf9c1329f8e1b59d949992623/src/components/layout/window-controls.tsx#L79) retains separate no-drag controls without that attribute. Hit-testing and native movement remain runtime checks. |
| Remaining height | The only changes in connect/bootstrap/workspace replace `h-dvh` with `h-full`. The outer viewport frame and `min-h-0 flex-1` content wrapper allocate the remaining height. Root `/` redirects to `/business`; it does not render a height-dependent scene outside this layout. |
| Private edits/session | The content wrapper at layout line 28 remains the same sibling when the native snapshot changes. No replacement content branch or key is introduced. BusinessPage's principal/revision key and clients/session helpers are unchanged; intended teardown on credential rejection or changed membership revision remains. The new test retains connection-input DOM/value across locale rerender; existing session/provider tests separately cover their boundaries. |
| Cold web/auth | Installed `isTauri()` only reads a runtime marker; the server snapshot is false and no web titlebar mounts. Following AppTitleBar → WindowControls → platform → transport finds lazy transport construction, with no client creation at module evaluation. The only added native effect is existing platform-gated window controls. Root providers, auth, Rust, capabilities and engineering navigation are unchanged. No broader member authority or ambient operator selection is introduced. |

The quiet diff for `src/app/business/page.tsx`, `src/lib/business`, `src-tauri`,
`src/components/layout`, i18n/appearance providers and dependency manifests exited
0. The product diff contains only the new layout/test, three height substitutions,
additive NOTICE and owner report.

## Independent validation

Created an unchanged archive at `.build/review-business-native-chrome/3d000874`,
using only this reviewer's existing installed `node_modules`. No other worker's
target/cache or new dependency was used. After the run, git-blob verification
matched all **1,213 archived source/manifest/licence files**. The compact
[verification record](review-business-native-chrome-evidence/archive-verification.json)
includes affected files; the complete manifest remains beside the own archive.

From that archive:

```sh
node node_modules/vitest/vitest.mjs run \
  src/components/business/native-chrome.test.tsx \
  src/components/business/session.test.tsx \
  src/components/business/provider-isolation.test.tsx \
  --reporter=verbose
```

**Exit 0; 23 tests, three files, 1.66s.**
[Captured output](review-business-native-chrome-evidence/focused-tests.txt); terminal
color escapes and the trailing empty line were normalized. Raw output remains
locally preserved, with both hashes in the structured verdict.

- 13 new chrome cases: three scenes on macOS/web; three scenes each on mocked
  Windows/Linux, control callbacks/listener cleanup, RTL caption direction and
  connection-input DOM/value retained across locale rerender.
- Two existing session cases: rejected credential clears private edits/bearer
  while preserving an unrelated ambient slot; membership revision change removes
  private editing state and reflects viewer access.
- Eight existing provider cases: five cold business aliases avoid legacy
  providers despite ambient operator/wallpaper settings; cross-tab locale keeps
  private input mounted without persistence; two other paths retain providers.

Only the inherited Vite CJS Node API deprecation warning appeared. No failed test
was patched, skipped or retried. The owner's frozen report separately records
52 focused tests, typecheck and scoped lint passing; these are attributed owner
results, not independent reruns. No broad suite/build, browser/native launch or
fixture mutation was needed for this bounded review.

## Borrowing and pinned documentation

Verified official refs with `gh api`, after reading installed pinned sources.
Codeg v0.30.4 resolves to **6f6bd648b206412644842a98d9ffeebf57292bed**.
Its shared titlebar, controls and three hooks are byte-identical at the correction.

| Exact source | Immutable blob | Use |
| --- | --- | --- |
| Codeg `src/components/layout/app-title-bar.tsx` | `c139d99f18a8b67806a045a604d9738aa22cdaab` | Unchanged layout, drag targets and insets. |
| Codeg `src/components/layout/window-controls.tsx` | `8d8248a3e9b54f5b998873ed6932c0625bb4070a` | Unchanged controls/callbacks. |
| Codeg `src/hooks/use-platform.ts` | `da2c8303ec44171e1a27293eabae0950d2a6b8aa` | Existing platform classification. |
| Codeg `src/hooks/use-mobile.ts` | `ed459cc436dfec8150c08a5100c269292b8475b3` | Existing responsive hook. |
| Codeg `src/hooks/use-media-query.ts` | `0fde6d44df78f650e5e14d82a1445bbb24783af9` | Existing responsive subscription. |
| Codeg `LICENSE` | `261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64` | Full Apache-2.0 licence read; unchanged. |
| Accepted e6eb7b56 `src/app/project-boot/page.tsx` | `6d79d24ac9785386cefe720f92422782fe0b099b` | Existing flex/titlebar composition. |
| Accepted e6eb7b56 `src/components/business/connect.tsx` | `232ae305f627314d1d566340d7e04f6d9ec20d68` | Existing native snapshot idiom and explicit connection boundary. |
| Official Tauri `crates/tauri/src/window/scripts/drag.js` | `1c9461b63d76f1b742fa15eeceaffa53bf38951c` | Read-only actual-target semantics; no copied dependency code. |

Official `tauri-v2.10.2` resolves to
**06374a902a50d2bd8b8d85593623ad16ac32325a**. Read its local locked Rust 2.10.2
drag script first, plus existing window geometry/capabilities. Read installed
React/ReactDOM **19.2.4** external-store hydration implementation,
`@types/react` **19.2.13**, `@tauri-apps/api` **2.10.1** marker/window APIs,
Vitest **2.1.9** source/help and existing test setup/bodies before execution.

Product NOTICE maps these imports/composition references to exact commits. Its
previous content is an exact byte prefix of the new NOTICE; no upstream or other
worker attribution was overwritten. No new dependency, AGPL/GPL/enterprise source
or framework port appears. Review artifacts contain output/verification metadata,
with no copied third-party implementation.

Applied code-context with the existing rag-skills venv and `HF_HUB_OFFLINE=1`.
Guide retrieval exited 0; relevant route/job guidance did not establish a
native-specific rule. The docs query exited 3 because `approvals.db` is missing;
no corpus ingestion/install or invented coverage. Live before/after hook records
for this session/worktree were verified before report writes;
[filtered evidence](review-business-native-chrome-evidence/docs-first.json)
contains no command bodies or credentials. Hooks remained enabled. Remote
research used immutable `gh api` refs only.

## Native evidence and limits

Read root's full [N2 finding](business-native-root/N2-native-chrome.md),
[native report](business-native-root/README.md), header-drag/positive-control
JSONs; viewed startup/local-workspace PNGs. These are root-run baseline
observations at native source **6cfff7d64e4d45f84d8cd5a26325b6ad4e1f311f** /
product **e72cc44b612068e67a3e6dc3bc593f10988ae7ed**. Traffic lights overlap the
brand. Business drag leaves x126/y42 unchanged; the valid blank engineering
control moves to x206/y67. The earlier pointer hit on a tab is excluded.

No corrected native result is inferred from jsdom. The new locale test covers a
connection field, not a real authenticated task draft during hydration; its copy
hook is mocked. Existing provider/session tests exercise their own boundaries
separately. Source structure supports preservation; root retains packaged
business editing and native interaction acceptance. Windows/Linux callbacks are
mocked, with no executed-platform claim. Owner browser/export checks were still
pending in the frozen report. These are explicit limits, not hidden passes.

## Publication and preserved work

Branch `review/business-native-chrome` started tracked-clean from accepted
**1f02eb17b1d57f387dc391d5ce27bf59e0f96861**. Historical docs-only checkpoint
**443718f9c061d01f3943d6038834b4700bd45e0a** was pushed before product arrival;
its waiting status is superseded by this verdict. Results were relayed to root
and rebrand through authorized internal Herdr prompts.

This branch changes only this report/evidence. Prior pushed task/UI reviews,
paused visual report, `.build/`, `out-design-final/` and all existing fixtures,
exports and processes remain preserved. No product fixes, agents, dependency
installs, live calls, merge or deployment.
