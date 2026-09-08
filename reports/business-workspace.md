# Business workspace — review handoff

Branch **feat/business-workspace**; draft [PR21](https://github.com/Adanmohh/codeg/pull/21).
Frontend implementation and actual protected-API browser checks are complete.
Final product source is **33b9cbcb63d3023e3ccd36a21f9cb9e4e2e425b7**, pushed.
Later handoff commits contain documentation/evidence cleanup and accepted main
status integration only; the tested product and live export stay unchanged.
Root remains reviewer/merger. The task backend is tested at **1ba73e3c** in an
owned snapshot and is not yet merged into this branch. No production mock layer.

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
browser testing. No Rust product edits, engine/approval changes, new dependencies,
lockfile changes, live provider/model calls, credential exposure or deployment.

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
| public/icon.svg | Accepted original Hafidh code-native vector mark; no image service/new asset system. |
| reports/design-reduced-motion-evidence/server.mjs | Owned synthetic static export/closed loopback proxy, no production server edits. |

Identity authority: **861fb0ef4394d4980a19ba375bb4c1f3f218d39b**, accepted
c911c406/main ab46c9d9; docs/contracts/business-identity.md and
src-tauri/src/business_identity/{types,mod,store}.rs. Accepted main through
a670163d was integrated as **0e5eb3e7**; both complete NOTICE sections retained.

Task DTO authority: **bf4309f5abdb077fd8e1a8e42db4861dadcda24b**, then tested
R1 source **1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5**;
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
No advertised CLI upgrade. Axum-test17.3.0 local TestServer/response docs ground
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
| pnpm exec vitest run business client/components plus appearance, connection guard, sidebar, Ops flows/session, intake and Telegram integration tests | Exit0;113 tests/12 files, integrated-frontend-tests.log. |
| pnpm exec vitest run src/components/business after final focus/copy corrections | Exit0;25 tests/4 files, including3 focus recovery regressions; final-business-tests.log. These overlap the integrated run, not an added138 tests. |
| pnpm exec tsc --noEmit | Exit0; final-typecheck.log. |
| Scoped eslint on all changed production/provider/navigation files, then src/components/business src/lib/business | Exit0, no warnings; checkpoint-lint.log and final-business-lint.log. |
| CODEG_EXPORT_DIR=out-business-workspace NEXT_TELEMETRY_DISABLED=1 pnpm build | Exit0;34 static pages, final-build.log. Actual final focus/archive/drawer checks use this rebuilt export. |
| cargo check --locked --offline --jobs4 | Exit0; default desktop, desktop-check.log. |
| cargo check --locked --offline --no-default-features --bin codeg-server --jobs4 | Exit0; server-check.log. |
| cargo clippy --locked --offline --all-targets --features test-utils --jobs4 -- -D warnings | Exit0; desktop-clippy.log. |
| cargo clippy --locked --offline --no-default-features --bin codeg-server --lib --jobs4 -- -D warnings | Exit0; server-clippy.log. |
| Actual static router test, owned source snapshot/target | Exit0;1 test with6 URL/body checks, static-router-test.log. Uses the unmodified Axum rewrite/ServeDir stack via axum-test's default in-process transport. |
| git diff --check; secret-pattern scan of report/evidence | Exit0 diff check; scan exits1 (no matching issued credential/PAT). No lockfile change. |

Cargo commands run from src-tauri with the own
.build/business-gates-target. No other worktree/old fixture target was used.
Inherited proc-macro-error2 future-compatibility warning and test-binary unwind
warning remain; neither is a new product failure. No broad unrelated Rust retest.

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
controls. Original failing/raw brief reports remain. Auto margin247px,18px
inherited radius,6px icon gap, Inter fallback stack and paired-dark/status colors
are source-classified, not silently erased from lint. Continuous motion/CLS/
long-frame counts and assistive technology were not assessed.

[Desktop list](business-workspace-evidence/generation2/after-filters/work-list-1280-light.png) ·
[Mobile dark board](business-workspace-evidence/generation2/after-filters/work-board-390-dark.png) ·
[Arabic review](business-workspace-evidence/generation2/review-390-arabic-dark.png) ·
[Keyboard return](business-workspace-evidence/generation2/final-keyboard-return.png) ·
[Viewer](business-workspace-evidence/generation2/viewer-task-real.png).

## Live owned review fixture

Open **http://127.0.0.1:4340/business.html**. Static proxy **PID36044** serves own
out-business-workspace; guarded real task API **PID81950 on4342** owns temporary
disk SQLite. Named CLI browsers: business-owner4340 **PID85074** (personal Amal
owner) and business-member4340 **PID85139** (personal Samira manager). Exactly two
named sessions. The fixture remains available for root. Operator setup choice can
use the **test-only public literal business-tasks-synthetic-operator**. Use People
in that context to intentionally issue a fresh personal credential; do not copy
real credentials or install a member token as the legacy operator. All records
are named Synthetic, with no provider/engine/scheduler process.

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

Final export SHA256: business.html
37caadb86dd9dbf828eb42ab4bf053303801f1a96cdedc4f4accccc0258b09ac;
index.html 4ed8eac14e81a0f07a39560ccbef20dda2f9b8dfba90c773015a3423a8a22f5c.
The static-router test uses BUSINESS_EXPORT_DIR pointing to that own export,
CARGO_TARGET_DIR pointing to .build/business-gates-target, and the snapshot test
business_tasks::tests::fixture::business_export_paths_use_real_static_router
with --locked --offline --no-default-features --lib --jobs4 -- --exact.
fixture-static-routing.patch uses zero context; reproduce with git apply
--unidiff-zero after the disk-fixture patch, only inside an isolated snapshot.

## Limits and preservation

Task1ba73e3c is a tested dependency snapshot, not an accepted-main merge. Current
origin/main at inspection is ebb553de. Identity is integrated; root must accept/
integrate the task backend and review the exact UI head before declaring combined
Increment A accepted. Entrust success with a real running agent is intentionally
not exercised: no model launches. Browser guard/static router tests do not claim
a production server deployment or native-window certification.

English and Arabic business copy are supplied; the other8 app locales preserve
preferences and use English business copy. Reload requires sign-in; task lists
use manual refresh and context revalidation on focus/every30s, not realtime push.
Member directory follows500 cap. Pagination wiring is tested; the manual fixture
contains5 tasks and does not claim a second50-record page browser exercise.

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
