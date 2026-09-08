# Shell and Pi design correction review

Accepted and merged, 2026-09-08. Historical review follows. Product checkpoint
`f8ba45973925f5e78331de1d47e913f5a56cb1f5` reviewed, including all presentation
changes, field associations, translated guidance, NOTICE and regression tests.
No credential/config persistence, model guard or approval behavior change.

Root independently ran PiConfigPanel, AgentSetupNotice and StatusBarAlerts
suites: nine passed, exit 0, 1.05s; log
`/tmp/ops-design-shell-independent-tests.log`. These verify saved values survive,
only explicit save persists, custom-provider fields are named and recovery
buttons invoke the intended actions. Worker browser evidence shows the full
setup reason at 390px (332px content/client width), a 44px settings action and
unchanged disabled send state. Final browser review remains pending.

## Additional recovery-path finding

Worker `07-after-settings-guidance.json` identifies a visible unnamed 32×32px
mobile menu button. Root confirmed `settings-shell.tsx` uses Menu inside Button
without a label at line 238. This affects navigation on the Pi setup recovery
path. Requested a bounded accessible translated name/touch-size fix and actual
mobile drawer open/close recheck. No new navigation system or settings mutation.
Keep this high accessibility finding open until the worker correction is
reviewed and verified. The original five corrections also await final measured
light/dark evidence and exact-head acceptance.

## Independent menu correction recheck

Root reviewed the working SettingsShell correction and verified it through actual
CLI4324: named Navigation, 44px target, real drawer open/Escape close, unchanged
route and no overflow. Provider/Model/Thinking/API Key names also passed.
[Evidence](browser-shell-independent/README.md). The menu finding is resolved
in the working change; final commit and worker gates are still required before
PR acceptance. Root closed its session and released the worker fixture.

## Final acceptance

PR #13 accepted at `0565f197df5754bf14fb37d0be3a13715c70e85c`, merged as
`783bfb9cd9caf9546f6ef9effc067f5c904fc6be`. Final commit changes reports and
artifacts only; product is reviewed `e29cab3e`. Root read the full final report,
inspected light/dark screenshots and composited contrast results, and verified
its independent tests/CLI evidence above. All six bounded findings resolved.
Worker 171 focused tests, lint, typecheck and production builds pass.

Sidebar labels/empty text now measure7.54 light/8.81 dark; inherited status
text8.90/8.75; setup reason18.31/18.17. Full mobile setup text and explicit
44px recovery action remain readable. Repeated remount/focus exploratory
attempts did not establish a sticky-error guarantee and are disclosed as such;
accepted lifecycle behavior was not changed. No whole-app/accessibility or
live model-readiness certificate is implied. Final combined design review
and native rebuild remain separate.
