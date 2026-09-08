# Ops locale, RTL and receipt corrections

Complete and ready for root review in draft [PR14](https://github.com/Adanmohh/codeg/pull/14), branch `fix/design-ops`. Final production/evidence SHA is recorded in the closing documentation commit. Sole worktree: `/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand`.

The four [workorder](design-ops-workorder.md) corrections and root's selected-row contrast follow-up are implemented. No approval/sender/API/DTO/component-prop change, dependency upgrade, live provider action, inference, deployment, extra agent or other-worktree write.

## Final behavior

- `i18n-provider.tsx` keeps the initial settings/message boot guard, then retains the mounted tree while later bundles load. Existing request cancellation and the last loaded bundle remain authoritative. Private Ops edits stay in memory; backend ID/base-URL changes still clear them, including colliding account/object IDs.
- `inbox-view.tsx`, `proposals-view.tsx` and `reply-editor.tsx` reuse `rtl:rotate-180`, native content direction and LTR identifier isolation. `ops-page.tsx` removes the account-number/duplicate connection banner; actual inbox identity and actionable connection controls remain.
- Terminal explanations distinguish accepted/recorded, accepted/recording-pending, failed, stopped-before-send, denied and unknown. Provider acceptance does not assert recipient delivery. **Finish recording receipt** uses the existing local recording action without another send; unknown still prohibits blind retry/new keys.
- Selected Task/Run, Open and date text use existing `text-foreground/75`; unselected rows keep their treatment. Actual-theme verification found no threading-input color correction necessary.

`ReviewCard`, `proposalStatus`, `ReplyEditor`, exact edited-payload handoff and all gate/dispatch/reconciliation conditions retain their contracts. The only Rust change is an explicitly ignored synthetic browser fixture plus its test-module declaration. No runtime, migration or lockfile change.

## Integration and exact reuse

Clean branch base: **617385194f67b3d9e5aca5d9d29f3b1baeebb070**. Implementation **d20b1f8d** and evidence checkpoint **881f5181e351bbe26dddc738735579d626214f69** were pushed before merge **785393397e5dafc3838b98c6dd1c934d1897f561**, which incorporates accepted main **8f63f89cc159babd51d4900f4b235d9fecdaa923** (PR13/f4da7027 included). Its sole conflict was additive NOTICE text; both complete blocks survived. Rust, locks, Ops/session/API and locale-provider source were unchanged by integration. The own export was rebuilt afterward.

| Immutable authority | Exact source files read | Reuse |
| --- | --- | --- |
| Codeg v0.30.4, Apache-2.0, **6f6bd648b206412644842a98d9ffeebf57292bed** | `src/components/i18n-provider.tsx`, `src/i18n/messages.ts`, `src/app/layout.tsx` | Existing async locale/cache architecture; minimal mount-preservation glue |
| Same Codeg pin | `src/components/ui/pagination.tsx` | Existing RTL icon utility |
| Accepted Ops UI **756d064f1cc391ed1da32ba90429adef225f080d**, accepted main **61738519** | `src/components/ops/{session,inbox-view,proposals-view,reply-editor,ops-page}.tsx`, `session.test.tsx`, `ops-flows.test.tsx` | Private memory lifetime, editor/review handoff and receipt presentation |
| Accepted local **617385194f67b3d9e5aca5d9d29f3b1baeebb070** | `src-tauri/src/ops/tests/integration.rs`, `integration/{browser,telegram_browser}.rs` | Protected router, SQLite, MemorySecrets, loopback provider reused by `integration/design_ops_browser.rs` |
| Accepted shell **e29cab3e6126fd0a0bf59dc9b63b52b625e3a67d** | `src/components/layout/status-bar.tsx` | Existing foreground/75 treatment |
| Local Design Studio **55c8614dcfff33b4caa5a544b4f1f91877214878** | `lab/tools/probe.mjs`, `scripts/{brief,design-lint,select-judges,merge-findings}.mjs`, audit/specialist methods | Tool use: actual CLI measurements passed to the pure formatter |

Original Apache LICENSE and every accepted NOTICE/MIT block remain; precise local reuse is appended in NOTICE. No AGPL/enterprise/PolyForm source. Local pinned reads sufficed; no remote dependency research or latest-source repinning. Remote documentation, when needed, remains `gh api` at immutable refs.

## Actual browser evidence

Installed Playwright CLI **0.1.18**, owned session `ops-design4326`, real protected local server, separate **out-design-ops/**. All records/providers are synthetic. Valid unchanged pre-integration flow evidence is retained.

| Check | Evidence/result |
| --- | --- |
| Baseline failure | [before](design-ops-before.raw): locale unmounted Ops and reverted the unsaved reply; Back had no rotation; accepted receipt also displayed the contradictory no-receipt disclaimer |
| Reply/note and complete review | [after](design-ops-after.raw), [review](design-ops-review.raw): English→Arabic→English passes at 1280×900 and 390×844, both themes; all edits retained, private localStorage absent, provider **4→4** |
| RTL/leave guard | Both Back icons **180°**; English body/emails LTR, Arabic note/subject RTL; no account prefix/overflow. [Cancel](design-ops-leave-cancel.raw) preserves note and focus |
| Terminal/recording | [terminal](design-ops-terminal.raw): five real fixture states offer no second send. Proposal 2 completed UI recording, provider **4→4** |
| Integrated final measurement | [final report](design-ops-final-measured.json): pending review, thread with open headers and recorded receipt; **12 cases, 468 text samples**, zero scoped contrast failures, unnamed controls, heading jumps, targets under 44px or horizontal overflow; mobile inputs ≥16px |

The **original measured summary is not a clean contrast pass**. [Original results](design-ops-measured.json) retain selected light metadata **4.2:1**, dark threading candidates **2.27/2.66:1** and blank samples. Final probes select actual **Appearance → Light/Dark**, wait for matching class/color scheme, open threading disclosures and check native visibility; no injected theme class.

| Target | Final settled contrast |
| --- | --- |
| Selected Task/Run, Open and date | **8.66:1 light**, **9.76:1 dark**; light RGB(67,68,68) on RGB(238,242,242) |
| Open review In-Reply-To / References | **18.48:1 light**, **17.50:1 dark**; dark RGB(250,250,250) on RGB(21,21,21) |
| Open draft In-Reply-To / References | **18.48:1 light**, **15.43:1 dark**; dark RGB(250,250,250) on RGB(33,33,33) |

[Direct styles](design-ops-threading-styles.raw) show light `lab(98.26 0 0)` text in the real Dark setting. A closed textarea still returned a rectangle while Playwright visibility and native `checkVisibility()` were false. [Control evidence](design-ops-control-visibility.raw) identifies closed disclosures and visibly empty Cc/Bcc inputs; their labels remain sampled. The blank-valued **All statuses** select paints a name and is included. Every populated, enabled threading field is included. The initial extractor cannot conclusively attribute every stale color sample; its failures are preserved rather than relabelled as product passes.

Screens: [selected row](browser-design-ops/final-selected-thread-light.png), [review fields](browser-design-ops/final-headers-pending-human-review-desktop-dark.png), [draft fields](browser-design-ops/final-headers-synthetic-locale-review-desktop-dark.png), [mobile RTL](browser-design-ops/after-mobile-dark-ar-thread.png), [recording pending](browser-design-ops/after-terminal-2-mobile-light.png), [recorded receipt](browser-design-ops/final-sent-provider-accepted-mobile-light.png). Reproducible scripts/raws use `design-ops-*`; all final CLI/probe commands exit 0.

## Gates

Every final command below exited **0**. Logs remain locally in `reports/` (gitignored); browser results and measurements are committed.

| Command | Result |
| --- | --- |
| `pnpm exec vitest run src/components/ops` | **22 passed**: 8 locale/session, 14 flow/receipt. Locale suite first failed 5/8 on baseline, then passed 8/8 |
| Integrated Vitest selection below | **92 passed**, 11 files; `design-ops-integrated-frontend.log` |
| `pnpm exec tsc --noEmit` | Pass; `design-ops-integrated-tsc.log` |
| `pnpm exec eslint` on five changed UI files plus both Ops test files | Pass; `design-ops-integrated-eslint.log` |
| `CODEG_EXPORT_DIR=out-design-ops pnpm exec next build` | Pass, 33 routes; `design-ops-integrated-build.log` |
| `cargo check --locked` | Default desktop pass |
| `cargo check --locked --no-default-features --bin codeg-server --bin codeg-mcp` | Server/MCP pass |
| `cargo clippy --locked --all-targets --features test-utils -- -D warnings` | Desktop pass |
| `cargo clippy --locked --no-default-features --bin codeg-server --bin codeg-mcp --lib -- -D warnings` | Server/MCP pass |
| `cargo test --locked --no-default-features --bin codeg-server --lib ops` | **167 passed, 4 manual fixtures ignored**, 0 failed, 5.95s |

Integrated frontend command:

```sh
pnpm exec vitest run src/components/ops src/components/ops-telegram src/components/ops-intake src/components/settings/system-network-settings.test.tsx src/i18n/messages.test.ts src/lib/transport/web-transport.test.ts src/components/settings/pi-config-panel.test.tsx src/components/layout/status-bar-alerts.test.tsx src/components/chat/agent-setup-notice.test.tsx src/components/conversations/sidebar-section-header.test.tsx
```

Rust uses only `.build/intake-host`; [gate script](design-ops-rust-gates.sh) records exact invocations. It passed on d20b1f8d; Rust/locks remained byte-identical after integration, so unrelated Rust tests were not repeated. The final CSS-only follow-up passed the rebuilt export, integrated frontend suite, scoped lint/typecheck and actual contrast checks.

Meaningful tests cover initial boot, delayed/failed/superseded bundles, responsive remounts, complete edited payload, colliding backend IDs/base URLs, no private persistence/save/send on locale change, and receipt-only callbacks. Initial harness/type errors were corrected against actual DTOs/Vitest APIs without upgrades.

Root independently reported **22 tests passing** (`/tmp/ops-design-locale-independent.log`) and final 4326 cross-tab reply/note preservation, 390px width, RTL Back/email direction, no private localStorage and unchanged provider count. These are owner-reported independent results.

## Design Studio verdict and limits

Audited against `docs/design/BRIEF.html` schema 1, scan mode, BC-3/4/5/7/9/10/12/14/18. Aesthetic/a11y/flow methods ran **sequentially inline**, with no extra agent or paid flow executor. [Merged findings](design-ops-findings.json) retain the specialist evidence. Scores /10: states **9**, feedback **9**, accessibility **8**, responsive **9**, visual/tokens **8**. Design-weighted verdict: requested fixes are review-ready; no whole-app certification.

One remaining measured limitation: inherited shared-control transitions persist under reduced motion (**6–19**, `reducedMotionEffective=false`); jank/CLS unassessed. Shared motion/global CSS is outside this workorder. Static lint's ReplyEditor list/loading warning comes from address-normalization `.map`; parents own loading/errors. Literal brief checks flag converted/composited OKLCH colors, differing Inter fallback stacks and inherited **6px** label gaps. Diagnostics remain committed, not an all-token pass. Existing Ops copy remains English under Arabic layout; native WebView and the separate phone route were not certified here.

The inherited nonexistent `/tmp/ops-ui-task-fixture` folder causes unrelated shell Git/workspace 404 diagnostics. No engine starts. Existing sidecar-placeholder/future-compatibility warnings and expected mocked Settings errors remain in logs. No packaged/signed/distributed build claim.

## Stable root fixture

Keep **http://127.0.0.1:4326/login** alive; deliberately nonsecret token **ops-design-synthetic-operator**. Server PID **63050**, own data `.build/design-ops/e0b6e800-4361-442d-963d-c137628b62c9`, export **out-design-ops/**. Proposal **1** pending; **2/3** accepted/recorded; **4** unknown; **5** rejected; **6** denied. Proposal 2's recovery already ran once; do not reseed/replay for this evidence. Root browser remains open; worker browser writes are finished.

[Release state](design-ops-fixture-release.raw): English/LTR, Follow system, provider **4**, synthetic only. `/api/ops_design_fixture_stats` is authenticated/read-only and **test-only**. Launcher from own `src-tauri/`:

```sh
CODEG_OPS_ACCOUNT_ID=1 CARGO_TARGET_DIR=../.build/intake-host cargo test --locked --no-default-features --lib ops_design_browser_fixture -- --ignored --nocapture
```

A fresh launch creates new synthetic data; current review reuses the running instance. Every other fixture/export is preserved.

## Docs-first evidence

Complete founding/orchestration documents, AGENTS, current STATUS/workorder and design brief/reviews were read. Protected docs changed only through accepted merge. Separate React package/Cargo manifest reads activated the live hook. Own session **01a07c1c-cf2e-73e1-bbe3-e758c8363042**, exact cwd: initial Pre/Post lines **7130/7114**, final live **8986/8985** in `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`. Hooks stayed enabled.

Code-context used existing RAG `.venv/bin/python`, **HF_HUB_OFFLINE=1**: guide exit **0**, docs lookup exit **3** because `data/code/rebrand.db` is absent. Guidance: installed-source authority, retained navigation/terminal states, no unsafe email HTML. Direct pinned reads replace missing corpus coverage; no ingestion/environment change.

Installed source/types: React **19.2.4** / types **19.2.13**, next-intl **4.8.3**, Next **16.1.6**, Tailwind **4.1.18**, next-themes **0.4.6** (actual localStorage/class/color-scheme behavior), TypeScript **5.8.3** DOM (visibility/selection), Testing Library **16.3.2**, Vitest **2.1.9**, CLI **0.1.18** / Playwright core **1.63.0-alpha-2026-08-05**. Helper fs signatures were read from installed **@types/node 25.2.2**; executing Node is **24.19.0**, not a claimed v25 runtime. Fixture grounding retains locked Tokio **1.49.0**, Axum **0.8.8**, SeaORM **1.1.19**. No dependency/global-tool change.
