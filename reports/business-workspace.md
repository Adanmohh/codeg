# Business workspace — implementation checkpoint

Active, 2026-09-08. Branch `feat/business-workspace`, created from accepted
`origin/main` at `4ec04d7282a50529335d724438d42b99a53385a2`. Deliver direct human
task creation/assignment/progress/review and a finished role-based workspace;
engineering and optional agent conversation remain supporting capabilities.
UI contract: `docs/contracts/business-ui.md`. Early docs commit `cf404c6e`, pushed;
draft PR [21](https://github.com/Adanmohh/codeg/pull/21).

## Preserved work and source authority

Tracked state was clean before switching. The untracked research report was
byte-identical to main and would collide; its original was moved, without
overwriting, to ignored
`.build/rebrand-paused-checkpoints/20260908-business-resume/research-intromail-shared-work.md`
(SHA256 `d4d99598d10870b55e0700f439e49df86a57255a2c01e0ae00b7cf04b523a7a0`).
`reports/visual-workspace.md` stays untracked and unchanged. Existing 4326 fixture,
`out-design-ops/` and browser remain untouched; no new fixture launched yet.

Read current BUSINESS-IMPLEMENTATION, FOUNDING, ORCHESTRATOR, STATUS, DECISIONS,
AGENTS, the three research reports and design BRIEF. Business resumption
supersedes the paused narrow visual scope. No protected planning documents edited.

Docs-first live audit includes PreToolUse and PostToolUse records for session
`01a07c1c-cf2e-73e1-bbe3-e758c8363042`, worktree `rebrand`, after separate reads of
`node_modules/react/package.json` and `src-tauri/Cargo.toml`. Hooks stay enabled.
React installed version is 19.2.4. Code-context offline guide retrieval exited 0;
dependency retrieval exited 3 because `rebrand.db` is absent. No new corpus,
package or model was installed. Relevant guidance: installed-version authority,
one route per job, precise staging and existing source reuse. No extra agents.

Applied Design Studio art-direction/frontend-design/checklist methods. Direction,
token inheritance, mobile composition and self-critique are in the UI contract.
Source analysis confirms `/workspace` mounts privileged engineering providers;
restricted business members need a separate protected business surface, retaining
shared UI primitives and minimal navigation seams.

## Initial source-to-adaptation map

All local source below read at `Adanmohh/codeg@4ec04d7282a50529335d724438d42b99a53385a2`;
Apache upstream baseline remains `xintaofei/codeg@v0.30.4`,
`6f6bd648b206412644842a98d9ffeebf57292bed`.

| Read source | Intended reuse |
| --- | --- |
| `src/components/ui/{button,input,textarea,dialog}.tsx` | Existing primitives and nested-dialog guards; scoped presentation only. |
| `src/components/tasks/board-columns.ts`, `task-card.tsx` | Status grouping and task row/card composition; business DTOs stay distinct from engineering-run state. |
| `src/app/{layout,page,login/page}.tsx`, `src/app/workspace/layout.tsx` | Existing entry/navigation behavior and precise member isolation boundary. |
| `src/lib/transport/{web-auth,web-connection-store,index}.ts`, `src/components/connection/web-connection-guard.tsx` | Inspect operator transport; never put member credentials into its global token slot. |
| IntroMail research at `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | Domain guidance only for frontend; no direct IntroMail or Plane/OpenProject source copied. Identified authorization gaps are requirements for backend integration. |

NOTICE records the inherited provider adaptation. No AGPL/GPL/enterprise source
is ported. No dependency/runtime change is planned. Read installed React19.2.4
hooks/types, Next16.1.6 navigation types, Tailwind4.1.18 theme/package,
next-intl4.8.3 exports, Radix Dialog types and existing primitives,
@tauri-apps/api2.10.1 invoke/core types, next-themes0.4.6 types,
TypeScript5.8.3 DOM fetch/clipboard types and Vitest2.1.9/testing-library16.3.2
test APIs. No newer documentation silently substitutes for these versions.

## Progress and remaining checks

- Provider isolation implemented for `/business`, trailing slash and direct
  `/business.html`. Tests include an old operator token and enabled local
  wallpaper. `/business-other` and `/workspace` retain operator behavior.
  Current Axum static-file rewrite/source confirms direct export reachability;
  actual fixture browser/network verification is pending.
- Identity contract `f3c36dc6` and task checkpoint `cb2e184f` read in their owners'
  worktrees. Dedicated typed identity client and composition are in progress.
  Task operation DTOs pending. UUID IDs, all-day `dueDate` and separate human-only
  completion review are confirmed. Only `legacyOperator`, never a role/domain,
  permits engineering access. Bootstrap/member setup is included.
- Own new fixture proposed at 4340 with a separate export and two named CLI
  sessions. Await actual guarded backend fixture contract; other outputs remain
  untouched.
- Focused provider and inherited regressions: 22/22 passed, exit 0
  (`.build/business-workspace/provider-tests.log`). Initial two harness failures
  used `true` for the inherited boolean preference; source uses `1`. Corrected
  fixture value, assertions retained. Typecheck exit 0
  (`.build/business-workspace/typecheck-checkpoint.log`). Implementation continues;
  final lint/build/browser/design gates pending. No live configuration claim.
- No live provider/model calls, account setup, credential output or deployment.
