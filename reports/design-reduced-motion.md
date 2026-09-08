# Drawer reduced motion — checkpoint

Sole writer on `fix/design-reduced-motion`, clean branch from accepted main
`0c11345803c77e24cba433a69d51c9669b74fe36`. Draft PR pending creation.

## Scope and contract

Fix the verified 390px sidebar transform transition under reduced motion.
Use one popup-scoped `motion-reduce:transition-none` utility; preserve the
existing normal transition, state transforms, swipe variables, nested-layer
guards, focus and dismissal. No global CSS, other shell surfaces, configuration,
provider/model/engine work or dependency changes.

Read current complete FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS,
`docs/design/BRIEF.html`, `reports/design-acceptance-checklist.md` (BC-3), and
the current owner workorder. Applied code-context, design-checklist and actual
Playwright CLI methods. Local code-context guide passed; installed-doc corpus
query exited3 because `data/code/tickets.db` is absent. Use directly read pinned
source instead; no missing corpus coverage claimed. No extra workers.

## Grounding

- Codeg v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed`,
  `src/app/globals.css` browser-tab reduced-motion `transition: none` pattern,
  verified via `gh api`; reuse through Tailwind in inherited `ui/drawer.tsx`.
  Existing Apache-2.0 LICENSE/NOTICE retained.
- Installed `@base-ui/react@1.7.0`: `drawer/popup/DrawerPopup.{mjs,d.ts}`,
  `drawer/viewport/DrawerViewport.mjs`,
  `internals/{useAnimationsFinished,useOpenChangeComplete,useTransitionStatus}.mjs`.
  Animation completion handles an empty `getAnimations()` collection; no synthetic
  transition-end event is needed. Popup focus and state handlers remain upstream.
  Tag resolves via gh API to `254f4744f0a241c20697b9eeab33402f4469a081` (MIT).
- Installed `tailwindcss@4.1.18` source: media variant targets exactly
  `prefers-reduced-motion: reduce`. Tag resolves via gh API to
  `6db2a6d637ed12489f9fd5a64ca394d934eb14f4` (MIT). React19.2.4,
  Next16.1.6 and existing export contract read. No version upgrades.
- Live docs-first audit has Pre/Post records for exact tickets cwd and session
  `01a07c1c-d82f-7022-84db-778a438632f1` at1788832611/1788832635;
  live docs-first tool context also emitted during this task. Hooks stay enabled.

## Pending evidence

Capture actual CLI reduce/no-preference opening and closing frames, visible
390px open/closed states, keyboard focus, Escape and outside dismissal using
owned4324/4325 and a new isolated export/data fixture. Run existing drawer and
consumer regressions, focused ESLint, typecheck and production export. Report
exact source/head, measurements, fixture shutdown, limitations and final PR URL.
