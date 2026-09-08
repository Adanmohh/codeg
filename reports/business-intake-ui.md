# Business Sources and shared workbench

Current product: **c1e618dede15368d4af39ac74391f18931b317a5**, branch
`feat/business-intake-ui`, [draft PR29](https://github.com/Adanmohh/codeg/pull/29).
Base: accepted `a40b03393a466672060066ae6e0e8c9054a2349d`.
The independent frontend implementation and structured-workspace checks are
published. **Complete B intake runtime acceptance remains pending tenant-epoch
integration, the protected B fixture handoff and accepted backend integration.** No complete
provider, native-tenant, or final product acceptance is claimed.

## Delivered behavior

The business workspace uses real authorized tasks in List, Board and Table,
with persistent work tabs and optional side-by-side or stacked panes. Existing
Codeg pane geometry, resizable controls and hidden-overlay context keep editors
mounted across locale, viewport and reference-pane changes. Closing a task tab
uses its existing pending/unsaved guard. Arrow/Home/End focus is distinct from
tab activation; close returns focus to surviving work. Task deadlines remain
calendar `YYYY-MM-DD`, with no instant conversion or reminder claim.

The closed Sources client and UI implement connection/access setup, import
rediscovery, exact passage selection, private task preparation, full destination
audience confirmation, acceptance, text-free linking, discard and recovery.
Preparation reuses the accepted task fields; acceptance never substitutes an
ordinary task-create request. Link uses the authorized task detail and exact
ID/revision/domain CAS, copies no passage text, and describes review invalidation.
Unknown writes retain their operation identity and require status/decision
recovery. Metadata-only disclosure withholds private source/draft content;
local expiry can hide text but cannot renew freshness. No provider implementation
or backend endpoint is duplicated in this branch.

Organization appearance consumes the exact settings/get+update seam. Display
name, neutral/blue/violet palette and preferred pane arrangement stay within
the business subtree and its portals. An owner/admin edits with expectedRevision;
conflict/lost-response recovery compares current settings before explicit
adoption. Other members see disabled fields and no save control. Language and
light/dark preference remain personal. Scope/epoch change closes the client and
clears private state; a cancelled disconnect preserves it. Credentials and drafts
stay in memory, outside legacy token storage and providers.

The native host offers its existing original-organization operator path.
Personal HTTP sign-in is blocked there while restricted tenant windows remain
unavailable. Browser personal access is usable. There is no native member
fallback, caller-selected tenant authority, fake tenant switcher, generic window
permission grant, or invented scoped terminal/chat/calendar capability. The
existing engineering entry remains available only with legacyOperator.

## Exact contract and source authority

| Authority | Immutable source / use |
| --- | --- |
| Accepted B contract | `670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5`, `docs/contracts/business-intake.md`; SHA256 `6d4be7f3be2c41264ab3bb9c27311e879306a75af0aaae6c0e240f434c874dbd` |
| Access prerequisite | `18be55edc276713fc6d46d075baec363245ba285`, `docs/contracts/business-intake-access.md`; explicit zero grants/history/owner lifecycle and setup authority |
| Accepted UI plan | `1a876afc2ae81c7ea2066a14cdbb268fb363e37c`, `reports/business-intake-ui-plan.md`; Q1–Q4 closed |
| Compiling Rust DTOs | `9a4c8c882cec938665bc233b4d658d8de019ccfd`, `src-tauri/src/business_intake/types.rs`; canonical closed field/capability shapes, byte-unchanged through `15bb402b9265006930cc9ce4428332d04c9fb34a` (`git diff --quiet` exit0) |
| B runtime checkpoint | Product `e9c6323760497e7b294282d2746cfe1d9b2774fa`; latest reviewed handoff `15bb402b9265006930cc9ce4428332d04c9fb34a` adds recovery tests/report. Actual `http.rs` registers all 25 accepted intake operations, including candidate decisions; integrated tenant-epoch persistence, full transport validation and the real B fixture remain pending |
| Tenant settings | Contract `7516461633c163c2ac683930487e33231e630b0b`; actual `29774b50aafc29658a2f48fab1f44d366ed2c8a0` types/settings/http/native commands; final owner report `d3a176d2287c23b649cd1d266cb1a9187bbcc0bb` leaves product unchanged |

The final bounded reconciliation resolved PR28's immutable head through `gh api`
and read its committed report, DTO comparison and complete HTTP registration.
Its seven candidate and four recovery test passes are backend-owner evidence,
not frontend runtime acceptance. Retained observations/previews still need the
captured tenant epoch so fresh authentication after suspend/resume cannot revive
old evidence. No epoch default, source copy or premature integrated fixture is
introduced here. PR30's later report-only head
`b3dfbcb602314cee6eb0039d92cb997762c02dd8` retains product29774b50; root
acceptance/integration of both backends remains separate.

Complete governing documents, business implementation/interaction plan and design
brief were read before product work. Remote source research used `gh api` at
immutable refs; no mandated borrowed-source pin was upgraded. Current owner
direction supersedes the earlier lightweight dashboard: shared tabs/panes and
structured human/agent work are the foundation, with execution tools requiring
their own real scoped authority.

All product ports are approved Apache-2.0 Codeg at
`a40b03393a466672060066ae6e0e8c9054a2349d`, retaining original Codeg v0.30.4
`6f6bd648b206412644842a98d9ffeebf57292bed` attribution:

| Files read | Adapted frontend glue |
| --- | --- |
| `src/lib/business/{client,identity,tasks,presentation}.ts` | Closed intake/settings operation maps, canonical task fields and role/domain projections |
| `src/components/business/{task-form,task-detail,ui,workspace}.tsx` | Existing editors, CAS comparison/adoption, modal focus, navigation and unsaved guards |
| `src/app/workspace/layout.tsx`, `conversation-detail-panel.tsx`, `tab-bar.tsx`, `src/lib/tab-group-layout.ts` | Stable keyed pane composition/tab chrome; computeRects remains unchanged |
| `src/components/ui/{resizable,overlay-host-hidden,dialog,drawer}.tsx`, `src/hooks/use-media-query.ts` | Existing resizing, hidden portal behavior and responsive shell; no shared Drawer rewrite |
| `src/app/globals.css`, `src/lib/theme-presets.ts` | Existing paired semantic presets scoped through React context; no global token/font changes |
| `session-details-content.tsx` | Existing container-query pattern applied to list, task fields and Sources grids in narrow panes |
| `src/components/ui/button.tsx`, business `ui.tsx` | Scoped primary-action foreground/hover corrections for actual contrast findings |
| `reports/business-workspace-evidence/serve.mjs` | Owned guarded export helper; no production server changes |

Source references above are under `src/components/conversations/` for
conversation/session-detail files and `src/components/tabs/` for tab-bar.
NOTICE appends exact sources and preserves every preceding license/attribution.
The frontend copies no IntroMail provider code, proprietary Edublend source,
Payload implementation, AGPL/GPL code or restricted enterprise source. Tickets
owns provider/task-transaction/source-license implementation mappings.

Installed authority read: React19.2.4/@types-react19.2.13, Next16.1.6 static
export, Tailwind4.1.18, TypeScript5.8.3, next-themes0.4.6,
react-resizable-panels2.1.9, @tauri-apps/api2.10.1, Testing Library
React16.3.2/DOM10.4.1 and Vitest2.1.9. CLI0.1.18 uses installed
Playwright1.63.0-alpha-2026-08-05; its actual local types were read. Node24.19.0
and @types/node25.2.2 fs/http/path primitives ground the evidence helpers.
No install or dependency/lockfile change occurred.

Evidence file reads/writes also use `@types/node@25.2.2` `fs.d.ts`
readFileSync/writeFileSync signatures. Official Node24.19.0 `doc/api/fs.md`
sections were verified through `gh api` at commit
`cdc1b38d40cb567b7ad0b39c86addf830a0af0ae` (resolved annotated tag v24.19.0).
Context7 resolve/query lacks that exact version and returned main; it was not
used as pinned authority. This is API grounding, not an implementation port.

Code-context used the existing RAG `.venv/bin/python` with `HF_HUB_OFFLINE=1`.
Guide exit0 supplied canonical-type/source-first rules; docs exit3 reported no
rebrand installed-doc corpus. That missing coverage is explicit; installed
source/types supplied dependency grounding. The live docs-first audit contains
PreToolUse and PostToolUse for session01a07c1c-cf2e-73e1-bbe3-e758c8363042 in
this worktree. Separate React manifest and Cargo.toml reads ran on resume. Hooks
remain enabled; no workers, paid inference or other-worktree writes were used.

## Runnable owned synthetic fixture

UI **http://127.0.0.1:4350/business** (also `/business.html`), Node **90939**,
export `.build/business-intake-ui-nav`. HTTP/disk `business.html` SHA256:
`2d3f87ea738e4b191dd77f43373c34f2540e8f8beeba59a81d4542670bb38b1a`.

```text
node reports/business-intake-ui-evidence/serve.mjs .build/business-intake-ui-nav --backend=4351 --synthetic-intake-fixture --workspace-backend=4353
```

Task/settings API **4353**, Rust **99613**, isolated backend29774b50 archive
`.build/business-intake-ui-api-29774b50`, own target
`.build/business-intake-ui-target`. Only its ignored existing manual fixture is
adapted by [fixture-tenancy-4353.patch](business-intake-ui-evidence/fixture-tenancy-4353.patch):
owned loopback port, disposable disk SQLite, and exact POST identity/human-task/
settings/platform allowlist. Use `git apply --unidiff-zero`. The production
router/auth/core/migrations are unchanged. Its compile retry passed after adding
six omitted Pi include_str assets from the same immutable archive; no product fix.

`GET /__business_intake_fixture` publishes routing and method/path/status only.
The public, nonsecret fixture operator token is `business-tasks-synthetic-operator`.
There are no live provider credentials, calls, model/engine launches, sends or
deployment. Personal credentials are issued through the actual protected API,
used only in memory, and never recorded in reports/screenshots/storage exports.
All six work records and human/agent names explicitly say Synthetic.

Two named CLI sessions remain: **intake-ui-a4350** (browser50564, synthetic
owner Rania) and **intake-ui-b4350** (browser50617, synthetic member Yusuf).
A retains an unsaved contrast-check draft in task
`479e4d43-8b78-48b9-93cc-bcbb9d9ea928`; its loaded DOM is fb3004dc, whose only
later product change is selected-navigation styling. B loaded the final export.
Read-only review is safe; coordinate before mutating the reserved task/settings.
All older fixtures, browsers, targets, exports and paused reports are preserved.

Intake4351/upstream4352 remain tickets-owned and were not started or altered.
Current intake requests return the genuine unavailable-backend state. Complete
B checks must use one integrated backend/identity database and newly coordinated
synthetic sessions; credentials minted on4353 must not be treated as4351 access.
The export helper can route all closed business calls to4351 without the optional
workspace-backend flag when that real handoff is ready.

## Actual browser evidence and design corrections

| Check | Observed result / evidence |
| --- | --- |
| Two human sessions | Real submission reaches Review3; stale draft receives409, remains locked/intact, compares base2/currentReview3, and saves exact content at4 only after explicit adoption. Execution remains null. [Evidence](business-intake-ui-evidence/conflict-panes-live.raw) |
| Scoped settings | Owner saves Blue with200; open private draft survives, global palette remains neutral, no private localStorage/legacy entry. [Evidence](business-intake-ui-evidence/scoped-appearance-live.raw) |
| Narrow split pane | Before: six titles at width0 in475px pane. After container-query correction110c8a29: all393.1875px wide/26px high, no page overflow. [Before](business-intake-ui-evidence/workbench-split-1280-light-before.png), [after](business-intake-ui-evidence/workbench-split-1280-light-after.png) |
| Primary actions | Actual Blue dark3.46, Blue light hover3.48, Violet light4.01/hover2.97, Hafidh light hover4.37. Scoped existing colors plus95% hover in fb3004dc pass all18 preset/theme/normal-hover-focus cases; minimum4.5299. [Before](business-intake-ui-evidence/primary-palette-before.json), [after](business-intake-ui-evidence/primary-palette-after.json) |
| EN/AR pane matrix | Twelve390/768/1280 light/dark cases:936 measured text samples, zero contrast failures/unnamed controls, no page overflow, exact draft/date retained, manual tab focus and44×48 visible close focus. [Measured report](business-intake-ui-evidence/workbench-matrix-after.json) |
| Selected navigation | Blue selected text measured4.32 on its tinted background. Existing sidebar foreground +semibold now measures at least15.2989 across12 locale/theme/width cases, including narrow portal navigation and focus. [Before](business-intake-ui-evidence/design-member-sources.json), [after](business-intake-ui-evidence/navigation-matrix.json) |
| Real member views | Board/Table show the same three assigned tasks. At390px the348px table region scrolls its856px content by keyboard without page overflow. Appearance fields are disabled/no save; no legacy entry. [Evidence](business-intake-ui-evidence/member-views-final.raw) |
| Sources availability | Actual bindings/list502 produces a visible retry error, zero setup controls and no invented connected content. This is unavailable-backend evidence, not successful B setup/import. [Screenshot](business-intake-ui-evidence/member-sources-unavailable.png) |
| Reduced motion | Actual reduce preference retains visible task/controls and no overflow. Continuous motion/CLS/long-frame quality is not assessed by these stills. |

Screenshots and exact CLI scripts/raw samples are in
`reports/business-intake-ui-evidence/`. The initial table screenshot captured
the prior selection's transition; `*-initial.png` is retained and final captures
wait500ms after state changes. An initial palette probe exited1 on an ambiguous
duplicate task heading; the probe was scoped to the task pane and retried0.
Initial focus samples include hover; final focus samples explicitly move the
pointer away. The empty-baseline script used the wrong expected phrase and
returned false; its screenshot shows “Your work will appear here.” No passing
assertion is invented for that initial probe.

Design Studio art-direction/checklist/aesthetic/a11y/flow/reviewer methods were
applied inline, with no subagents or paid flow runner. Source55c8614dcfff33b4caa5a544b4f1f91877214878,
read-only root CLI Canvas probe, and pure buildReport ground the measurements.
The [BC checklist](business-intake-ui-evidence/design-checklist.md) remains the
page-specific ledger; the [structured review](business-intake-ui-evidence/design-review.md)
records the bounded verdict and open evidence gates. Source lint reports0 workspace gaps; its workbench “no
empty state” flag is a component-boundary heuristic: the always-present work tab
owns its real loading/error/empty content. The helper also misses useMediaQuery
responsiveness, which the rendered matrix verifies. Brief palette warnings are
retained: the hex-only comparator does not normalize the brief's OKLCH tokens or
the explicitly authorized scoped presets. No clean whole-brief verdict is claimed.
Full Sources/passage/review/receipt layout and independent final design review
remain pending real B integration.

## Validation and remaining handoff

| Product / command | Exit / result |
| --- | --- |
| a997cfb5 client/session/native/scoped-settings/workflow tests | 81/81; added Unicode settings case7/7. 82 distinct cases across those runs |
| 110c8a29 `pnpm exec vitest run` workbench/workflow/intake-workflow | 0;39/39,2.39s |
| fb3004dc same plus ui/settings-editor | 0;50/50,2.94s |
| c1e618de workbench/workflow | 0;22/22,3.52s |
| Each affected checkpoint `pnpm exec tsc --noEmit --incremental false` | 0 |
| Scoped ESLint on changed product files | Final0; pane checkpoint first1 only for JSX wrapping, corrected0 |
| Each own `CODEG_EXPORT_DIR=.build/business-intake-ui-{pane-layout,contrast,nav} NEXT_TELEMETRY_DISABLED=1 pnpm build` | 0;34 static routes each; all prior outputs retained |
| Product and staged `git diff --check` | Final0; evidence staging first2 for a trailing blank line in the retained failed probe log, normalized without changing its output |
| Real named CLI runs described above | 0 after documented probe correction; no mocked production responses |

Full local logs and the [final checkpoint summary](business-intake-ui-evidence/design-final-checkpoint.txt)
retain exact commands/exits.
No Rust product change requires new desktop/server/Clippy runs here; the separate
real fixture compiled locked/offline against29774b50 with its own output, and
PR30 owner reports its backend gates separately, with root acceptance pending.

Outstanding: tickets' captured tenant-epoch integration and unified protected B
fixture; real Fireflies/email/Hafidh safe-projection UI checks; protected B two-user
passage/draft/publication/receipt/privacy flows; accepted backend integration and
final independent Design Studio review.
No sources/provider success is substituted with fixtures that invent responses.
Tenant native windows remain unavailable; scoped AI execution and a content
calendar are separate authority/data work, not hidden legacy access. These
constraints do not prevent review of the published frontend/source checkpoints.

Paused visual/research hashes remain the recorded originals. Earlier chronological
report state is in git97409949 and the ignored own checkpoint
`.build/business-intake-ui-checkpoints/report-97409949.md`; no paused work was lost.
