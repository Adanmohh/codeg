# Business workspace — active implementation checkpoint

Branch `feat/business-workspace`; draft [PR21](https://github.com/Adanmohh/codeg/pull/21).
Base `4ec04d7282a50529335d724438d42b99a53385a2`; pushed source checkpoint
`c1a35955`, plus the scoped long-content correction in this checkpoint.
Real protected bootstrap/member creation, two personal sessions, task creation,
409 draft recovery, independent human review and cross-session completion have
passed. Role/revocation and complete responsive/Design Studio checks are still in
progress; this is not a final acceptance claim.

## Implemented behavior

The default entry opens a branded business workspace. A separate in-memory
connection signs into the shared server or explicitly connects the local native
operator. Original operators can bootstrap an organization; authorized owners and
admins can manage human/agent members and issue/revoke personal credentials.
One-time secrets remain masked until intentional reveal/copy and disappear on
closing the credential surface. Agent directory entries cannot receive sign-in
credentials. Existing operator login and engineering routes remain available;
only the server's `legacyOperator` capability displays the engineering entry.

My work, shared work and review support list/board views, server filters and
50-record pagination. Direct task creation and detail expose human ownership,
human/agent execution, nullable reviewer, calendar-only due date, notes,
deliverables, public activity and capability-controlled mutations. Progress has
only todo/in_progress/review; completion uses the distinct human review operation.
Null reviewer means any currently authorized human reviewer. Review displays the
actual saved revision and current deliverable. Existing execution links are
subordinate detail and do not launch an engine.

Conflicts preserve the edited draft and original revision, lock writes, and
require loading/comparing the current version before explicit adoption. Ambiguous
create failures do not automatically retry. Locale/viewport changes preserve
private drafts; disconnect, authentication failure or membership revision change
unmount the private workspace and discard late responses. Errors use safe fixed
copy instead of raw backend/provider bodies.

Cold `/business`, `/business/`, `/business.html`, `/` and `/index.html` omit legacy
settings/wallpaper/operator-connection reads, even with an ambient old operator
token. `/business-other` and `/workspace` retain inherited behavior. The business
client uses only the fixed `/api/business` POST envelope, omits cookies, refuses
redirects and never reads/writes the legacy bearer store. Local desktop business
commands remain operator-only; member HTTP credentials use the separate client.

## Source-to-adaptation map and authority

| Exact source read | Adaptation |
| --- | --- |
| `Adanmohh/codeg@4ec04d7282a50529335d724438d42b99a53385a2`: `src/components/ui/{button,input,textarea,dialog,drawer}.tsx` | Existing imported controls, modal focus/dismissal and shared Drawer; scoped business composition, no new UI system. |
| Same commit: `src/components/tasks/{board-columns.ts,task-card.tsx}`, `src/components/ops/session.tsx` | Status list/board presentation and keyed in-memory editing lifetime; separate business DTOs. |
| Same commit: `src/app/{layout.tsx,page.tsx,login/page.tsx,workspace/layout.tsx}`, `src/components/layout/sidebar.tsx` | Minimal business entry/navigation and inherited engineering access. |
| Same commit: `src/components/{i18n-provider,appearance-provider}.tsx`, `src/components/connection/web-connection-guard.tsx`, `src/lib/transport/{web-auth,web-transport,tauri-transport,index}.ts`, `src/lib/app-error.ts` | Local-only provider boundary; isolated typed HTTP/native client and safe command-error decoding. |
| Same commit: `src-tauri/src/web/router.rs`, `next.config.ts` | Read-only static-export routing authority, including direct `.html`; actual server/browser verification remains pending. |
| Identity `861fb0ef4394d4980a19ba375bb4c1f3f218d39b`, accepted `c911c406`, merged `ab46c9d9`: `docs/contracts/business-identity.md`, `src-tauri/src/business_identity/{types,mod,store}.rs` | Exact UUID/camelCase inputs, member grant limits, protected owner credential semantics and transport-only legacy capability. |
| Tasks `bf4309f5abdb077fd8e1a8e42db4861dadcda24b`: `docs/contracts/business-tasks.md`, `src-tauri/src/business_tasks/{types,policy,validation}.rs`; activity/HTTP checkpoint `76bb6909511016a11e864abe19d4ffd493aafa0c`, `store.rs` and handlers | Exact operation DTOs, nullable all-day date/reviewer, public activity fields and task capabilities. New combined `1e8b5250` is reported by its owner; runtime integration pending. |

Inherited Apache baseline: `xintaofei/codeg@v0.30.4`,
`6f6bd648b206412644842a98d9ffeebf57292bed`. NOTICE appends the exact frontend
adaptation and preserves all earlier licenses/entries. The Hafidh vector mark is
the accepted original `public/icon.svg`. IntroMail research at
`0bd24dfe284b888aa9f602fa1fd00e337ea38874` supplies domain guidance only; no
IntroMail, Plane, OpenProject, AGPL/GPL or provenance-uncertain implementation is
copied into this frontend. No dependency or lockfile change.

Read complete BUSINESS-IMPLEMENTATION, FOUNDING, ORCHESTRATOR, STATUS, DECISIONS,
AGENTS, the three research reports, root's 18-item acceptance checklist and design
BRIEF. Applied code-context and Design Studio art-direction/frontend-design/
checklist methods. Offline guide retrieval exited 0; installed-doc retrieval
exited 3 because `rebrand.db` is absent. No corpus/model/package installation.
Relevant retrieved guidance: installed-version authority, precise staging,
existing-source reuse and one route per job. No invented repository rule.

Local API authority: React19.2.4 hooks/types, Next16.1.6 navigation/build help,
Tailwind4.1.18 theme.css, next-intl4.8.3 provider exports, next-themes0.4.6,
@tauri-apps/api2.10.1 invoke/core types, radix-ui1.6.0 Dialog types,
@base-ui/react1.7.0 Drawer Root/Popup types, lucide-react0.563.0 declarations,
TypeScript5.8.3 DOM/Intl types, Vitest2.1.9 and Testing Library React16.3.2.
Playwright CLI0.1.18 help and skill read; no update installed. All remote source
reads use `gh api` immutable refs, not latest-library substitution.

Docs-first audit still contains live PreToolUse/PostToolUse for session
`01a07c1c-cf2e-73e1-bbe3-e758c8363042`, this worktree, following separate
`cat node_modules/react/package.json` and `cat src-tauri/Cargo.toml` reads.
Hooks remain enabled; no bypass or global configuration change.

## Validation at this checkpoint

| Command / evidence | Exit and result |
| --- | --- |
| `pnpm exec vitest run src/lib/business/client.test.ts src/components/business src/components/appearance-provider.test.tsx src/components/connection/web-connection-guard.test.tsx src/components/layout/sidebar.test.tsx` | 0; 75/75 across 7 files, `.build/business-workspace/checkpoint-tests.log`. |
| `pnpm exec tsc --noEmit` | 0; `checkpoint-typecheck.log`. |
| Scoped `pnpm exec eslint` on business files, root/login/sidebar and provider seams | 0, no warnings; `checkpoint-lint.log`. |
| `CODEG_EXPORT_DIR=out-business-workspace NEXT_TELEMETRY_DISABLED=1 pnpm build` | 0; 34 static pages including business. `build.log`. This build predates the final activity/unknown-member presentation adjustment; final export rebuild pending integration. |
| `git diff --check`, lockfile diff | 0; no lockfile changes. |

Behavioral tests cover dedicated bearer transport, unsafe origins, cookies and
redirect denial, native command mapping, authentication teardown/late responses,
cold-route legacy isolation, membership downgrade clearing private state, viewer
restrictions, capability-only engineering access, human creation/review,
calendar-day roundtrip, nullable reviewer, locale draft retention, stale CAS
comparison, masked one-time credentials and public plaintext activity allowlists.
Synthetic DTOs/fetch stubs exist only in unit tests, never production fallbacks.
Initial test-only harness issues (inherited boolean `1`, unsupported Testing
Library `exact` option and narrowed locale type) were corrected without weakening
assertions. Connect hydration was aligned with installed React's external-store
pattern; final lint/typecheck pass. No browser result is inferred from these tests.

## Remaining work and concrete limits

1. Push this frontend checkpoint, integrate accepted identity main while preserving
   all attribution, then integrate the accepted task backend when available.
2. Launch only the owned guarded fixture on **4340** with
   `out-business-workspace/` and two CLI sessions `business-owner4340` /
   `business-member4340`. Backend owner has reserved 4342; no other fixture is
   modified. Need the published real task fixture/static-export setup contract.
3. Complete actual protected task/member UI flows, 390/768/1280 light/dark/RTL,
   focus/reduced-motion/long-content checks and measured Design Studio review.
   Rebuild after integration and record exact tested head; no API-only acceptance.
4. New business copy currently has English and Arabic. The other eight existing
   app locales retain their global preferences and use English business copy;
   this limitation is explicit, not a claim of ten-language translation coverage.
   Credentials/drafts intentionally do not persist across reloads. The directory
   follows the backend's 500-member cap.

Preservation: paused `reports/visual-workspace.md` stays untracked and unchanged
(SHA256 `ded5a851409095dc40fffff7f778f668f79a614aa4040a1bb0349c81b0f5768e`).
The byte-identical colliding research copy was preserved under ignored
`.build/rebrand-paused-checkpoints/20260908-business-resume/` (SHA256
`d4d99598d10870b55e0700f439e49df86a57255a2c01e0ae00b7cf04b523a7a0`).
Existing 4326 fixture/export/browser and all other worker outputs remain untouched.
No backend edits, live provider/model calls, secret outputs, new agents or deployment.

## Integration and first browser checkpoint

Frontend implementation `79cfe080` is pushed. Accepted identity/main through
`a670163d` was merged as **`0e5eb3e7`**, pushed. Only additive NOTICE conflicted;
both complete append sections and original attribution were preserved. A local
resolution script initially misparsed the equals separator; corrected from the
two parent notices and amended the unpublished merge before push. The inherited
registration patch contains a trailing-space context line; it was preserved.
Identity integration typecheck and isolated export build exited0.

Actual CLI connection evidence is in `reports/business-workspace-evidence/`.
All five entry paths returned200 and rendered with **zero API calls**, including
ambient old operator token/wallpaper. Initial mobile capture put sign-in too low;
the scoped mobile introduction is now compact and selects are16px. Six after
captures cover390/768/1280, light/dark and Arabic RTL, with no horizontal overflow,
44px fields, visible keyboard focus and sign-in within the390px capture. Invalid
connection copy now describes entered information instead of task fields.
`connection-after.json` records actual values. The unavailable state came from
the disconnected fixture guard, not a simulated production success.

Probe corrections are retained honestly: initial error locator also matched
Next's route announcer, so it was scoped to the sign-in region; reduced motion
correctly sets `transition-property:none` despite an inert0.15s duration; source
formatting added a leading semicolon incompatible with CLI function-expression
input, removed from test scripts. No product behavior was weakened. The task
owner's4342 listener then closed before my login completed; that timeout is
preserved, not counted as passing integration.

Current real fixture: source snapshot
`.build/business-workspace/task-fixture-1ba73e3c/`, byte-unchanged archive of
**`1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5`** (`src-tauri`, Pi bundled source,
LICENSE/NOTICE), own target `.build/business-fixture-target/`. `cargo test
--locked --offline --manifest-path <snapshot>/src-tauri/Cargo.toml
--no-default-features --lib --jobs4
business_tasks::tests::fixture::business_tasks_browser_fixture -- --ignored
--exact --nocapture` compiled in2m55s and now runs the ignored guarded fixture,
PID21431 on4342, in-memory SQLite, no engine/provider. This listener belongs to
this frontend worktree; the task worker's earlier PID794 was stopped before it
started. Known linker unwind-table
and proc-macro-error2 future-compatibility warnings remain. This is an owned test
snapshot, not a product merge or backend edit. Proxy PID36044 on4340 runs
`node reports/business-workspace-evidence/serve.mjs out-business-workspace
--backend=4342`, with closed operations and no header/body logging. Browser PIDs
85074 (`business-owner4340`) and85139 (`business-member4340`) are owned here.
Initial actual UI sign-in returns a real context needing bootstrap.

The owner created `Hafidh Studio · Synthetic` and four named synthetic human/agent
members through the actual UI. The protected owner then issued its own personal
credential through the masked one-time surface, disconnected the original
operator and signed in as an ordinary owner-role member. The second named browser
signed in as the separate human member. Both personal sessions have no engineering
entry, no credential in localStorage and no legacy API calls. The old ambient
operator token remains unchanged. Evidence: `bootstrap-people.json`,
`owner-personal-session.json` and `member-connect.json`. Credential strings are
absent from the saved evidence.

R1 frontend alignment follows the new explicit operator-only entrust endpoint;
it stays under engineering detail, requires confirmation and never auto-links.
The subsequent link uses the newly returned task revision. An owner-role member
does not get this control. Focused client/workflow tests now23/23 pass, typecheck
and scoped lint exit0 (`entrust-*.log`); earlier12 session/workflow tests also
passed after mobile corrections. Entrust export rebuild exited0. Real task UI,
source-binding failure display, member flows and measured Design Studio acceptance
remain in progress. The task backend is not yet accepted/main-integrated here.

## Actual task workflow checkpoint

`create-work.json`: four tasks created by UI, with a distinct human owner,
human/agent/unassigned execution and explicit/nullable reviewer. Exact calendar
days `2026-10-01`, `2026-12-31`, `2028-02-29` and null roundtrip without conversion.
`member-conflict-review.json`: the assigned personal member kept its draft after a
real409, loaded/compared the owner's revision2, explicitly adopted it and saved
revision3; progress, note and deliverable reached review revision6 with no accept
control. A separately signed-in named manager confirmed and accepted the actual
deliverable, producing done revision7. `manager-review.json` and
`connect-current-owner.json` prove the result and its visibility in the independent
owner session. No conversation, engine, provider or outbound action was required.

The settled390px long-content test found3974px content inside a358px dialog.
The business-only modal now uses a min-zero grid column and inherited anywhere
wrapping; the shared dialog implementation is unchanged. Actual rebuilt export
measures358/358, full page390px (`long-detail-{settled-before,after}.json` and
screens). Tailwind4.1.18 installed `grid-cols` and wrapping utility implementation
was read before editing. Typecheck, scoped lint and isolated export build exited0
(`long-content-*.log`).

Two probe mistakes are retained: the reviewer locator omitted the actual word
“have”, corrected without changing product/assertions; waiting for *all* document
animations included dormant OverlayScrollbars scroll timelines (`currentTime:null`).
Review had already succeeded200. The corrected read-only capture waits for the
dialog's actual opacity1/transformnone and does not repeat the mutation. Failed
selector/timeline artifacts remain labelled, not counted as successful flows.
