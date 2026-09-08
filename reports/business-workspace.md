# Business workspace — review handoff

Branch **feat/business-workspace**; draft [PR21](https://github.com/Adanmohh/codeg/pull/21).
Implementation, owned browser checks and final gates are complete at product
**e72cc44b612068e67a3e6dc3bc593f10988ae7ed**. The corrected export is live on
**4346, PID60964**; the original review export on4340 is preserved. The independent
reviewer reports final targeted **PASS**, closing BUI-R1/R2/R3. Root remains
reviewer/merger and owns actual packaged native startup; that separate gate is
not claimed from browser or compile evidence.

Accepted task PR22 merge **5541857a125678dfb604f542693d9c82dfe5441e** is integrated
from main **294fb634b1833ebb13c4223484624e595598235b** as
**0192ac3f33dff82fae424bd30b797b14044f4197**. Only NOTICE required additive conflict
resolution; both complete sections remain. The live fixture's1ba73e3c production
runtime matches accepted tasks; the later task delta is a test-only discovery wait.
The merged full frontend suite passed6,194 tests/438 files. After the bounded
review corrections, all39 affected tests, typecheck, scoped lint and34-page export
pass; desktop/server checks and Clippy pass after the native entry correction.
There is no production mock layer.

## Result and boundaries

The default entry is a branded business workspace with My work, Shared work,
Review and People & agents. A human can connect, create/assign work, edit its
brief, contribute notes/deliverables and review the result without chat, a git
folder, terminal or agent. List/board, actual loaded counts, area/status/search,
archive and pagination use the real protected task API. Each task distinguishes
its accountable human, human/agent executor and nullable authorized reviewer.
Due dates remain exact nullable **YYYY-MM-DD calendar days**; no timezone conversion.
Done uses the separate human review operation and explicit confirmation of the
saved revision/current deliverable. Cancel, archive and restore describe their
actual effects. Execution linking remains subordinate and never starts an engine.

An original operator can bootstrap the organization and issue personal access.
Member credentials use a **separate in-memory business client**, never the legacy
bearer store or WebTransport. Owner role/engineering domain do not grant legacy
access: only server capability **legacyOperator** exposes that destination and
source entrustment. Agent directory entries have no human sign-in issuer. Issued
credentials are masked by default, intentionally revealed/copied, and cleared on
close. Context/principal/revision changes reset private editing; authentication
failure closes the client and late responses are discarded. Locale/viewport
changes preserve drafts. Safe fixed errors replace raw backend/provider bodies.

Cold /business, /business/, /business.html, / and /index.html omit legacy
settings/wallpaper/connection providers even with an ambient old operator token.
/business-other retains inherited providers. Browser member requests use only
closed /api/business POST {input} operations, omit cookies and refuse redirects.
Native remains operator-only; a desktop member can choose the separate shared
HTTP connection. Native OS isolation/multi-desktop operation is not claimed from
browser testing. Rust changes are limited to the fresh main-window route and its
comment. No engine/approval changes, new dependencies,
lockfile changes, live provider/model calls, credential exposure or deployment.

## Closed review findings

**Conflict comparison,095c61642dc2f52ca8fd6e7c10556f16cf61c904.** After a real409,
the old status/revision now reads "Your draft's base version" and the named
"Current saved version" region shows current status/revision/archive state before
adoption. The worker reproduced the old omission on33b9cbcb, then verified
Review/revision3 versus To do/revision4, retained draft, locked save and explicit
adopt/save to revision5 through the real API. English and Arabic390 light/dark
screens show358px dialog/content without overflow. Two red/green regressions also
retain the write lock and reset review confirmation after adoption. Root separately
verified its own real409/adopt/save flow and13 workflow tests. Permission, CAS and
confirmation behavior are unchanged.

**Native N1,3a189d1822f9fd335cc58ecb440f75ab9cbe937e.** Fresh main-window creation
uses WebviewUrl::App("business"). Showing an existing window preserves its route;
the explicit legacyOperator-only engineering link remains. Root's native source
review169015cf accepts this correction. Both compile modes and Clippy pass, but
the inherited build.rs zero-byte sidecar placeholder does not certify a runnable
bundle. Root owns normal packaging, companion hashes/protocol and actual startup.

**Search,e72cc44b612068e67a3e6dc3bc593f10988ae7ed.** The enabled light placeholder
was #737373 over composed #f7f7f7,4.43:1. Only the business search now uses inherited
foreground/80 in light mode; the dark token and global Input remain. The redundant
sr-only submit has tabIndex=-1, preserving native Enter submission. The prior1×1
clip-path inset(50%) control had a UA auto1px outline and no shadow: an invisible
tab stop, not a trap preventing Tab away. Actual CLI checks cover EN/AR ×
390/768/1280 × light/dark:12/12, no page overflow, visible44×44 Refresh focus with
3px ring, and Enter/clear each return200 through the real task list API. Canvas
ancestor/alpha measurement plus Design Studio gives **10.78:1 light,7.07:1 dark**;
no disabled/opacity/image exemptions. No task/provider writes in this search run.
The two added keyboard tests failed before the focus correction. An initial test
assertion incorrectly required explicit page:0; the accepted contract permits
omission/default0, so the new assertion checks query/view. All39 affected tests
pass. Root independently passes the same39 and verified the served export hash.

Evidence: conflict-clarity-{before,after}.{js,json}, comparison PNGs,
search-measure.js, search-{before,after}.raw.json and search-summary.json under
business-workspace-evidence/. The pure summarize-search.mjs calls the installed
Design Studio helpers at55c8614dcfff33b4caa5a544b4f1f91877214878, lab/tools/probe.mjs;
no browser launcher/tool implementation is copied. This is a targeted correction
review, not a new whole-page score.

**Independent final recheck: PASS.** The reviewer confirmed e72cc44b and served
business.html hash c029f99d5c92e677c97f2ebe45f4bd185746b73e84e77912a683e1d92de55a54.
Six reviewer-owned width/theme/English-Arabic cases measured 10.7805:1 light and
7.0679:1 dark, visible settled Refresh focus and Enter200. Its own-record real409
showed draft base revision1 against current Review/revision3, retained the exact
draft and saved revision4 only after explicit adoption; stale review was
invalidated. BUI-R1/R2/R3 are closed. This records the reviewer-delivered result;
its final report/evidence publication is owned by that reviewer. These six cases
are separate from the worker's12, not additional full-matrix coverage. All
fixtures remain available and unchanged by this documentation update.

## Exact source mapping

All frontend adaptation is Apache-2.0 Codeg. Upstream baseline:
xintaofei/codeg v0.30.4, **6f6bd648b206412644842a98d9ffeebf57292bed**.
Accepted fork source for the following rows:
Adanmohh/codeg **4ec04d7282a50529335d724438d42b99a53385a2**.

| Source files read | Adaptation |
| --- | --- |
| src/components/ui/{button,input,textarea,dialog,drawer}.tsx | Imported controls; scoped business forms/modals/navigation. Shared Dialog/Drawer unchanged. |
| src/components/tasks/{board-columns.ts,task-card.tsx}; src/components/ops/session.tsx | Status/list/board composition and keyed in-memory lifetime, using separate business DTOs. |
| src/app/{layout.tsx,page.tsx,login/page.tsx,workspace/layout.tsx}; src/components/layout/sidebar.tsx | Minimal business default entry and retained authorized engineering navigation. |
| src/components/{i18n-provider,appearance-provider}.tsx; connection/web-connection-guard.tsx; src/lib/transport/{web-auth,web-transport,tauri-transport,index}.ts; app-error.ts | Local-only provider boundary, typed dedicated HTTP/native client and safe error decoding. |
| src-tauri/src/web/router.rs; next.config.ts | Static export/rewrite authority; direct .html and trailing-slash verification. |
| src-tauri/src/lib.rs; commands/windows.rs at0192ac3f | Owner-requested N1 correction changes only fresh main-window App("workspace") to App("business"); existing-window focus remains unchanged. |
| public/icon.svg | Accepted original Hafidh code-native vector mark; no image service/new asset system. |
| reports/design-reduced-motion-evidence/server.mjs | Owned synthetic static export/closed loopback proxy, no production server edits. |

Identity authority: **861fb0ef4394d4980a19ba375bb4c1f3f218d39b**, accepted
c911c406/main ab46c9d9; docs/contracts/business-identity.md and
src-tauri/src/business_identity/{types,mod,store}.rs. Accepted main through
a670163d was integrated as **0e5eb3e7**; both complete NOTICE sections retained.

Task DTO authority: **bf4309f5abdb077fd8e1a8e42db4861dadcda24b**, then tested
R1 source **1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5**, accepted atc3af4494
and merged as5541857a;
docs/contracts/business-tasks.md, business_tasks/{types,policy,validation,store}.rs
and HTTP/native wrappers. Public activity fields were read at76bb6909. No actor
or org override, new auth boundary, or unapproved backend endpoint was invented.
Operator-only entrust-execution requires confirmation; linking is separate and
uses the returned revision. A real unbound legacy task returns400 without launch.

Test-only fixture patches use the same1ba73e3c source: tests/fixture.rs (blob
1c2f30c657fb339be53392cda21e508bd866929e), db/test_helpers.rs (blob
98cc3fa307e793cad07af75f577c24d3189a181a), and business_identity/tests.rs
(blob c606ac88e9556bb8284dc28ce647938c921d5a59). Both patches are committed
under the evidence directory and apply only to the ignored owned snapshot.
NOTICE preserves original attribution and every preceding worker entry.
IntroMail0bd24dfe284b888aa9f602fa1fd00e337ea38874 supplied research context only;
no IntroMail/AGPL/GPL/provenance-uncertain implementation was copied here.

Review corrections reuse src/components/business/{task-detail,ui,workflow.test}.tsx
and src/lib/business/copy.ts at33b9cbcb63d3023e3ccd36a21f9cb9e4e2e425b7; native
lib.rs/commands/windows.rs at0192ac3f33dff82fae424bd30b797b14044f4197; search
src/components/ui/input.tsx, chat/feedback-notes-display.tsx and
tasks/task-settings-dialog.test.tsx at3a189d1822f9fd335cc58ecb440f75ab9cbe937e.
The three precise attribution sections are appended to NOTICE.

## Docs-first evidence

Read complete founding/orchestration/status/decisions/AGENTS, BUSINESS-IMPLEMENTATION,
all three research reports, root BW-1–18 checklist and unchanged design brief.
Applied code-context, art direction/frontend design/checklist and Playwright CLI.
Offline RAG guide retrieval exited0 using the existing rag-skills venv and
HF_HUB_OFFLINE=1. Installed-doc query exited3 because rebrand.db is absent;
coverage was not invented and no corpus/model/package was installed.

Local installed authority: React19.2.4/@types19.2.13 hooks, Next16.1.6 export and
navigation, Tailwind4.1.18 tokens/variants/important utilities, next-intl4.8.3,
next-themes0.4.6, Tauri API2.10.1, radix-ui1.6.0 resolving Dialog1.1.17 and
FocusScope1.1.10, Base UI1.7.0 Drawer/FloatingFocusManager/enqueueFocus,
Lucide0.563.0, TypeScript5.8.3 DOM types, Vitest2.1.9 and Testing Library16.3.2.
Dialog close callbacks suppress default Trigger focus; these controlled business
modals have no Trigger, so scoped return-focus glue is necessary. The shared
focus trap remains intact. Node24.19.0/@types-node25.2.2 fs/http types and
Playwright CLI0.1.18/bundled1.63.0-alpha-2026-08-05 sources/help were read.
Search interaction grounding uses actual installed user-event14.6.3 setup,
keyboard/tab types and keypress implementation, not the manifest's lower range.
Report conversion reads @types/node25.2.2 fs.d.ts readFileSync/writeFileSync,
url.d.ts pathToFileURL and path.d.ts join; Node runtime remains24.19.0.
No advertised CLI upgrade. Native entry authority is the actual locked Rust
tauri2.10.2 (distinct from the JavaScript Tauri API version), manager/webview.rs:415–430
joining App paths to the application URL, manager/mod.rs:373–424 falling back to
the path's .html asset, and tauri-utils2.8.2 config.rs:72–124 defining App(PathBuf).
The export contains business.html; native explicit engineering navigation remains
in src/components/business/workspace.tsx. Axum-test17.3.0 local TestServer/response docs ground
static routing verification. All remote research used gh api immutable refs.

Live docs-first audit: session **01a07c1c-cf2e-73e1-bbe3-e758c8363042**, cwd this
worktree, PreToolUse and PostToolUse exit0 at epoch1788864741, following separate
React package/Cargo manifest reads. Audit path:
/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl. Hooks remain enabled.

## Verification

Logs are under .build/business-workspace/ in this worktree; final evidence copies
are under reports/business-workspace-evidence/checks/. Committed transcripts
normalize terminal color/progress whitespace; original raw logs remain in .build.

| Command/check | Result |
| --- | --- |
| pnpm test, merged0192ac3f | Exit0;6,194 tests/438 files,34.05s; accepted-main-frontend-tests.log. Root independently passed the same6,194. No full-suite rerun after the bounded corrections; affected coverage below. |
| pnpm exec vitest run src/lib/business/client.test.ts src/components/business, final e72 source | Exit0;39 tests/5 files,2.31s; search-final-tests.log. Includes15 workflow tests and provider/native access isolation. |
| pnpm exec tsc --noEmit | Exit0 on accepted integration and final search source; accepted-main-typecheck.log/search-final-typecheck.log. |
| Scoped eslint on changed production/provider/navigation files, then src/components/business src/lib/business | Exit0, no warnings; accepted-main-lint.log/search-final-lint.log. |
| CODEG_EXPORT_DIR=.build/business-workspace/review-export NEXT_TELEMETRY_DISABLED=1 pnpm build | Exit0;34 static pages from e72, search-final-build.log. Corrected4346 serves exactly this export. Earlier integration/comparison exports are retained. |
| cargo check --locked --offline --jobs4 | Exit0 after native3a correction,1m13s; native-entry-desktop-check.log. |
| cargo check --locked --offline --no-default-features --bin codeg-server --jobs4 | Exit0,44.20s; native-entry-server-check.log. |
| cargo clippy --locked --offline --all-targets --features test-utils --jobs4 -- -D warnings | Exit0,1m19s; native-entry-desktop-clippy.log. |
| cargo clippy --locked --offline --no-default-features --bin codeg-server --lib --jobs4 -- -D warnings | Exit0,40.26s; native-entry-server-clippy.log. |
| Actual static router test, owned source snapshot/target | Exit0;1 test with6 URL/body checks, static-router-test.log. Uses the unmodified Axum rewrite/ServeDir stack via axum-test's default in-process transport. |
| playwright-cli -s=business-owner4340 --raw run-code --filename reports/business-workspace-evidence/search-measure.js | Exit0 before and after;12 final cases. summarize-search.mjs with the existing Design Studio path exits0;12 contrast samples pass. |
| git diff --check; secret-pattern scan of report/evidence | Exit0 diff check; scan exits1 (no matching issued credential/PAT). No lockfile change. |

Cargo commands run from src-tauri with the own
.build/business-gates-target. No other worktree/old fixture target was used.
Inherited proc-macro-error2 future-compatibility warning and test-binary unwind
warning remain; neither is a new product failure. The native placeholder warning
is recorded above. No Rust changes after3a, and no broad unrelated Rust retest.

Exact integrated frontend command (113 at that checkpoint; the later3 focus
regressions are covered by the final25-test run):

```sh
pnpm exec vitest run src/lib/business/client.test.ts src/components/business \
  src/components/appearance-provider.test.tsx \
  src/components/connection/web-connection-guard.test.tsx \
  src/components/layout/sidebar.test.tsx src/components/ops/ops-flows.test.tsx \
  src/components/ops/session.test.tsx src/components/ops-intake/bug-workflow.test.tsx \
  src/components/ops-telegram/settings.test.tsx src/lib/ops-telegram/locator.test.ts
```

Actual CLI evidence, not production mocks:

| Workflow | Evidence under business-workspace-evidence/ |
| --- | --- |
| Five cold entry paths200, ambient operator token, zero API calls; failed/offline connection and responsive sign-in | connection-before.json; connection-after.json; connection screenshots. |
| Operator bootstrap, human/agent setup, masked one-time token, personal owner and independent member sessions | bootstrap-people.json; owner-personal-session.json; member-connect.json. No issued credential in artifacts. |
| Human/agent/unassigned work, explicit/nullable reviewer, exact dates incl leap day, private locale draft | create-work.json; member-own-work.json; member-draft-locale.json. |
| Real409, preserved draft, compare/adopt/save; member note/deliverable and independent human review | member-conflict-review.json; manager-review.json. |
| Actual viewer downgrade, forbidden forged requests403, revocation401 and private-state teardown | generation2/{owner-downgrade-member,member-downgrade-check,viewer-forgery,owner-revoke-member,member-revocation-check}.json. |
| Current separate assignee/reviewer, area/no-result filter, unbound source400, submission and cross-session Done | generation2/{assignment-filters-source,manager-deliver,owner-accept-final,member-done-cross-session}.json. |
| Cancel→archive→restore, status/history retained, focus after disappeared row | generation2/archive-focus.json; final-style-focus.json and screenshots. |
|390/768/1280 list and board, light/dark; Arabic drawer/date/review, keyboard and reduced motion | generation2/after-filters/; drawer-rtl-date.json; review-focus-rtl-after-motion.json; final-style-focus.json. |
| Corrected actual409 base/current status and revision, explicit recovery, EN/AR | conflict-clarity-before.json; conflict-clarity-after.json and PNGs; only new worker-owned record changed. |
| Corrected search contrast, visible Tab destination and real Enter submission | search-before.raw.json; search-after.raw.json; search-summary.json;12 corrected screenshots. |

Root BW checklist: frontend aspects BW-1–8/11–15 covered above and focused tests;
BW-9/10 agent/transaction authority belongs to task backend owner (no new agent
bridge here); BW-16 native limitation is explicit; BW-17 integrated frontend
regressions passed. BW-18 worker Design Studio review/evidence is provided for
root's independent accepted-source assessment.

## Visual review and images

See [measured review](business-workspace-evidence/design-review.md), BC-1–16,
raw/pure reports and specialist-method JSONs. Methods applied sequentially with
no agents or inference. Worker scores8/10 for states/feedback/accessibility/
responsive/visual; not an independent or award-level certification.

Measured fixes: mobile filters32.2→171×44px; long dialog3974→358px content within
358px; actual reduced-motion dialog animation none; destructive text4.07→7.11
light and7.99 dark (hover6.08/6.59); opaque focus borders7.11/7.99. Final review
samples94 per theme have zero contrast failures and zero unnamed interactive
controls. That generic text-node probe did not sample search placeholders; the
independent finding is explicitly corrected/measured separately above. Original
failing/raw brief reports remain. Auto margin247px,18px
inherited radius,6px icon gap, Inter fallback stack and paired-dark/status colors
are source-classified, not silently erased from lint. Continuous motion/CLS/
long-frame counts and assistive technology were not assessed.

[Desktop list](business-workspace-evidence/generation2/after-filters/work-list-1280-light.png) ·
[Mobile dark board](business-workspace-evidence/generation2/after-filters/work-board-390-dark.png) ·
[Arabic review](business-workspace-evidence/generation2/review-390-arabic-dark.png) ·
[Keyboard return](business-workspace-evidence/generation2/final-keyboard-return.png) ·
[Viewer](business-workspace-evidence/generation2/viewer-task-real.png).

[Corrected mobile search](business-workspace-evidence/search-after-390-light-en.png) ·
[Arabic dark search/focus](business-workspace-evidence/search-after-390-dark-ar.png) ·
[Current-versus-draft comparison](business-workspace-evidence/conflict-clarity-after-390-ar-dark.png).

## Live owned review fixture

**Final corrected preview:** http://127.0.0.1:4346/business.html serves
**e72cc44b612068e67a3e6dc3bc593f10988ae7ed**, static **PID60964** (owned session11100).
Absolute export:
/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand/.build/business-workspace/review-export.
HTTP and disk business.html SHA256 both:
**c029f99d5c92e677c97f2ebe45f4bd185746b73e84e77912a683e1d92de55a54**.
Index SHA256:58816191c20995f6cfaca4cf46165ddb7ea0253677c7b23326802db704ccddc1.
Root independently verified this path/hash/PID and source/NOTICE correlation.
Command: node reports/business-workspace-evidence/serve.mjs
.build/business-workspace/review-export --backend=4342 --port=4346.

Only4346's static listener was replaced for corrections; prior listeners1233
(baseline) and12331 (comparison095c6164) are stopped. Their exports remain in
.build/business-workspace/{integrated-export,conflict-export}. Backend4342/data
and frozen4340 were never restarted/replaced for these corrections. Reload and
sign in to load the final search/comparison source. The Rust entry correction
does not affect this running backend.

**Independent review window is open.** Root and approvals may use4346 and create
uniquely named Reviewer/Synthetic members and tasks for their own review flows.
Existing Amal/Samira and earlier captured records remain read-only for comparison.
Worker-only task ec9d106d-fa40-4e26-b2dc-612cedd587ad finished the correction flow
at To do/revision5. No worker task mutations remain in progress. Root/reviewer
records and browser sessions were not touched.4340 remains available for baseline
comparison against product33b9cbcb, handoff a126730274c8a3dc822345da536eebed1f8dc253.

Reviewer setup: open **Administrator access**, select **Original administrator
token**, enter the public fixture-only literal below, then Connect. Organization
already exists. In People & agents, create a human Member and human Manager with
the required task areas (e.g. Marketing/Website/Feedback), and intentionally issue
each a personal access token through its member detail. Use those two personal
accounts in the reviewer's own CLI sessions; keep issued values only in memory,
masked except during intentional transfer, and out of snapshots/reports/storage.
All new review data is confined to this temporary synthetic backend. No existing
worker browser session needs to be attached or changed.

Baseline **http://127.0.0.1:4340/business.html**: static proxy **PID36044** serves own
out-business-workspace; guarded real task API **PID81950 on4342** owns temporary
disk SQLite. Named CLI session hosts: business-owner4340 **PID85074** (tab0 personal
Amal; tab1 corrected4346 synthetic operator) and business-member4340 **PID85139**
(personal Samira manager). Exactly two
named sessions. The fixture remains available for root. Operator setup choice can
use the **test-only public literal business-tasks-synthetic-operator**. Use People
in that context to intentionally issue a fresh personal credential; do not copy
real credentials or install a member token as the legacy operator. All records
in this fixture are synthetic, with no provider/engine/scheduler process.

PID21431 was **this worker's earlier** in-memory4342 fixture, not tickets' PID794.
After initially successful flows it later returned500; no unsupported diagnosis
or passing downgrade claim was made. It was stopped and replaced only here.
SeaORM1.1.19 uses SQLite max1; SQLx-core0.8.6 retains10-minute idle/30-minute
connection lifetime defaults, a concrete manual in-memory fixture concern, not
proven root cause. Production database uses disk. Committed
fixture-temporary-disk.patch switches only the ignored test to its existing
fresh_disk_db helper; failed and generation1 evidence is preserved. Current
manual Cargo session18511 remains serving. Static router verification uses a
separate target and never restarts that fixture.

Reproduction source: .build/business-workspace/task-fixture-1ba73e3c/ archive;
manual fixture command uses its src-tauri/Cargo.toml, --locked --offline
--no-default-features --lib --jobs4, test
business_tasks::tests::fixture::business_tasks_browser_fixture -- --ignored
--exact --nocapture, with own .build/business-fixture-target. Proxy command:
node reports/business-workspace-evidence/serve.mjs out-business-workspace --backend=4342.
Only closed business POSTs are forwarded; legacy APIs/WebSockets/outbound are
blocked. Proxy logs only method/path/status. No credential snapshots/traces.

Preserved4340 export SHA256: business.html
37caadb86dd9dbf828eb42ab4bf053303801f1a96cdedc4f4accccc0258b09ac;
index.html 4ed8eac14e81a0f07a39560ccbef20dda2f9b8dfba90c773015a3423a8a22f5c.
The static-router test uses BUSINESS_EXPORT_DIR pointing to that own export,
CARGO_TARGET_DIR pointing to .build/business-gates-target, and the snapshot test
business_tasks::tests::fixture::business_export_paths_use_real_static_router
with --locked --offline --no-default-features --lib --jobs4 -- --exact.
fixture-static-routing.patch uses zero context; reproduce with git apply
--unidiff-zero after the disk-fixture patch, only inside an isolated snapshot.

## Limits and preservation

Identity and task backend are integrated from accepted main294fb634. Root must
review/accept the exact UI head before declaring combined Increment A accepted.
The final targeted review uses corrected e72 on4346 and matching accepted backend
runtime; the baseline33b9cbcb export is separately preserved on4340.
Entrust success with a real running agent is intentionally
not exercised: no model launches. Browser guard/static router tests do not claim
a production server deployment or native-window certification.

English and Arabic business copy are supplied; the other8 app locales preserve
preferences and use English business copy. Reload requires sign-in; task lists
use manual refresh and context revalidation on focus/every30s, not realtime push.
Member directory follows500 cap. Pagination wiring is tested; the original manual
fixture contained5 tasks before worker/root/reviewer additions and does not claim
a second50-record page browser exercise.

Probe errors are retained as labelled error files: duplicate date locator (date
also present in activity), early Drawer starting-style/focus-guard sampling,
BODY focus handoff before the final correction, initial reviewer selector, and
waiting on dormant global scroll timelines. Final runs wait for actual settled
surfaces; no mutation was blindly repeated after an uncertain response.

Paused reports/visual-workspace.md stays untracked, SHA256
 ded5a851409095dc40fffff7f778f668f79a614aa4040a1bb0349c81b0f5768e.
The colliding research copy remains in ignored
.build/rebrand-paused-checkpoints/20260908-business-resume/, SHA256
 d4d99598d10870b55e0700f439e49df86a57255a2c01e0ae00b7cf04b523a7a0.
Existing4326/out-design-ops/browser and all other worker fixtures/outputs are
untouched. Protected founding/planning docs and all prior NOTICE entries remain.
