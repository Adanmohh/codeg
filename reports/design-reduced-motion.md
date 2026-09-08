# Drawer reduced motion — completed handoff

Draft [PR #19](https://github.com/Adanmohh/codeg/pull/19),
`fix/design-reduced-motion`. **Validated product head:
`cebce09d0df73205277d59c7048e733cc4b588f4`.** Clean branch from accepted main
`0c11345803c77e24cba433a69d51c9669b74fe36`; checkpoint `93da1406` and product
fix pushed early. Final handoff adds report/evidence only. No merge or deployment.

## Change and measured result

One `motion-reduce:transition-none` utility on `DrawerContent`'s popup suppresses
automatic transform/height/opacity/filter transitions for reduced motion. Ending
and swipe duration overrides cannot re-enable a transition property. Normal
animation, transforms, direct swipe tracking, focus, nested-layer guards and
dismissal handlers remain unchanged. No global CSS, other product surfaces,
dependencies, lockfiles, Rust source, planning docs or shared registrations changed.

Actual headed Playwright CLI at **390×844**, production exports and the accepted
no-engine Rust fixture. Before and after use the same fresh isolated database.
The probe records requestAnimationFrame positions and actual `getAnimations()`;
it waits for mounting and settled geometry before visible screenshots.

| Popup path | Before, reduce | After, reduce | After, no preference |
| --- | --- | --- | --- |
| Keyboard opening | −333.5→8px, 450ms transition | Only closed/open positions; zero intermediate/animated frames | −333.5→8px, normal 450ms transition |
| Escape closing | Intermediate positions and active transitions | Zero intermediate/animated frames; unmounted | Normal ending transition and unmount |
| Outside press | Intermediate positions and active transitions | Zero intermediate/animated frames; unmounted | Normal ending transition and unmount |
| Touch drag/release | Not remeasured | Finger tracks 8→−202px; release has zero animated frames; dismisses | Same tracking; release animation retained; dismisses |

Normal preference also passed before the correction. Both preferences preserve:
focus into the popup, Tab into its controls, Escape returning visible keyboard
focus to **Show Sidebar**, and nested **View options** menu Escape dismissing only
the menu. Touch events were dispatched through actual Chromium CDP via Playwright
CLI, using installed protocol types; no DOM event/content substitution.

Settled popup: x8/y8, 331.5×828px; document width390. The sidebar is intentionally
non-modal, so the visible page strip supplies outside dismissal; there is no
backdrop on this particular surface. Backdrop code and nested stack code were
not edited. Existing nested/Radix/hidden-host and drawer-consumer tests pass.
Duration tokens still compute to450ms/ending400ms under reduce; **transition
property is `none`**, with zero actual transition animations. A duration-only
scanner is insufficient to assess this correction.

[Frame/focus/geometry assertions](design-reduced-motion-evidence/summary.json),
[before frames](design-reduced-motion-evidence/before.json),
[after frames](design-reduced-motion-evidence/after.json),
[nested/touch evidence](design-reduced-motion-evidence/gestures.json).
Inspected actual [reduced open](design-reduced-motion-evidence/after-reduce-open.png)
and [closed with restored focus](design-reduced-motion-evidence/after-reduce-closed.png)
images. Both preferences' before/after screenshots and reusable CLI scripts are adjacent.

## Immutable source mapping and docs-first evidence

- **Codeg v0.30.4**, `6f6bd648b206412644842a98d9ffeebf57292bed`:
  `src/app/globals.css` browser-tab reduced-motion `transition: none` pattern,
  read locally then verified through `gh api`, adapted through Tailwind in
  inherited `src/components/ui/drawer.tsx`. Apache LICENSE blob
  `261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64` matches immutable upstream exactly.
  NOTICE adds this mapping and preserves all existing sections.
- **Base UI1.7.0**, `254f4744f0a241c20697b9eeab33402f4469a081`:
  installed `drawer/popup/DrawerPopup.{mjs,d.ts}`, `drawer/viewport/DrawerViewport.mjs`,
  `utils/useSwipeDismiss.mjs`, and `internals/{useAnimationsFinished,useOpenChangeComplete,useTransitionStatus}.mjs`.
  Empty `getAnimations()` completion and default focus/return behavior were read.
  Official `docs/src/app/(docs)/react/handbook/animation/page.mdx` read via gh API
  at that resolved tag commit. MIT Material-UI SAS2019; API reference only, no port.
- **Tailwind4.1.18**, `6db2a6d637ed12489f9fd5a64ca394d934eb14f4`:
  installed `dist/chunk-CT46QCH7.mjs` defines the exact reduce media variant and
  transition `none` static value. MIT Tailwind Labs; API reference only, no port.
- Fixture reuses accepted Pi PR8 `44e30733e6a83f9b7e73f65a8487cf1a87075308`,
  `src-tauri/src/work_task/desk/tests.rs::pi_desk_browser_fixture`, unchanged.
  Guard adapts accepted PR15 `c17ce81fd206f7694691f9d274f38ef3692bba20`,
  `reports/pi-issues-evidence/guard.js`; both retain Codeg Apache attribution.
  Report-only static/API proxy uses installed Node24.19.0/@types-node25.2.2
  HTTP/FS types. React19.2.4, Next16.1.6 export source, Vitest2.1.9 and
  Playwright CLI0.1.18 help/installed Playwright protocol/types read before use.

Read complete current FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS,
`docs/design/BRIEF.html`, acceptance checklist (BC-3) and owner workorder.
Applied code-context, design-checklist and actual CLI methods. Existing local
rag-skills Python with `HF_HUB_OFFLINE=1`: guide exit0; docs query exit3 because
`data/code/tickets.db` is absent. Direct pinned reads fill that gap; no corpus
coverage invented. Owner's no-agent/gh-api-only rules prevail over generic skill
suggestions. Remote research used gh API only, no version upgrades or AGPL source.

Live hook audit: exact tickets cwd/session `01a07c1c-d82f-7022-84db-778a438632f1`,
paired PreToolUse/PostToolUse at1788833713 in
`/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`. Separate React package
and Cargo manifest reads preceded work. Hooks remained enabled; no bypass.

## Gates, cleanup and limits

All meaningful final commands exited0; [exact commands/results](design-reduced-motion-evidence/gates.json):

- `pnpm exec vitest run` with `drawer-nested-layer`, task-detail session drawer,
  file-viewer drawer and canvas-conversation drawer test paths: **39/39,4 files**.
- `pnpm exec eslint src/components/ui/drawer.tsx`; `pnpm exec tsc --noEmit`.
- `CODEG_EXPORT_DIR=src-tauri/target/design-reduced-motion-{before,after} pnpm build`:
  two separate successful production exports. Source baseline and final head above.
- Actual `playwright-cli -s=drawer-motion --raw run-code --filename=.../measure.js`
  before/after, `gestures.js`, screenshots, geometry/focus/frame assertions pass.
- `PI_CODING_AGENT_DIR=<owned empty catalogue> cargo test --locked --no-default-features
  --lib pi_desk_browser_fixture -- --ignored --nocapture`: **1 passed,564.33s,exit0**.
  Browser closed; fixture stop file triggered graceful shutdown and temporary-data
  cleanup. Owned4325 proxy also exited0. `lsof` found neither4324 nor4325 listening
  (expected no-match exit1). No other fixture/export or root build output touched.
- `git diff --check` passes. Raw gate logs remain locally ignored under
  `reports/design-reduced-motion-*.log`.

Guard counters stayed **0 remote,0 agent/model attempts,0 blocked writes**. No
provider, paid prompt, engine loop, configuration save or install was invoked.
Final page console has0 messages. Initial exploration hit a proxy `.html`
resolution404 and the expected unauthenticated session-expiry screen; fixed the
report-only proxy and logged in with the public synthetic fixture token. The first
probe captured only one mounted opening frame; that incomplete attempt is labelled
and retained, then mounting-aware measurement recaptured both preferences.

This is the bounded sidebar motion/BC-3 recheck, not full Phase1 design acceptance.
Native WebView, physical-device touch, all modal/nested drawer variants and a new
Arabic-locale run were not certified here. Single-user filesystem trust and live
Pi/Astra setup limitations are unchanged; final combined audit remains owner-led.
