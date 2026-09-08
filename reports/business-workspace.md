# Business workspace — active implementation checkpoint

Branch `feat/business-workspace`; draft [PR21](https://github.com/Adanmohh/codeg/pull/21).
Base `4ec04d7282a50529335d724438d42b99a53385a2`; previous pushed source checkpoint
`142b600e`. This checkpoint implements the frontend against the published identity
and task contracts. It is **not runtime UI acceptance**: real guarded task fixture,
two-session browser evidence and final Design Studio checks are still pending.

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
