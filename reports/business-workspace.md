# Business workspace — implementation checkpoint

Active, 2026-09-08. Branch `feat/business-workspace`, created from accepted
`origin/main` at `4ec04d7282a50529335d724438d42b99a53385a2`. Deliver direct human
task creation/assignment/progress/review and a finished role-based workspace;
engineering and optional agent conversation remain supporting capabilities.
UI contract: `docs/contracts/business-ui.md`.

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
| `src/components/tasks/{board-columns,task-card}.tsx` | Canonical status mapping and task row/card composition; business DTOs stay distinct from engineering-run state. |
| `src/app/{layout,page,login/page}.tsx`, `src/app/workspace/layout.tsx` | Existing entry/navigation behavior and precise member isolation boundary. |
| `src/lib/transport/{web-auth,web-connection-store,index}.ts`, `src/components/connection/web-connection-guard.tsx` | Inspect operator transport; never put member credentials into its global token slot. |
| IntroMail research at `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | Domain guidance only for frontend; no direct IntroMail or Plane/OpenProject source copied. Identified authorization gaps are requirements for backend integration. |

NOTICE will record actual component adaptations with exact paths before product
handoff. No AGPL/GPL/enterprise source will be ported. No dependency/runtime
change is planned.

## Progress and remaining checks

- Early UI contract prepared; identity and task contracts not published yet.
  Continue independent composition and source grounding, then wire their actual
  APIs. No fabricated production DTO/response or second authentication model.
- Own new fixture proposed at 4340 with a separate export and two named CLI
  sessions. Await actual guarded backend fixture contract; other outputs remain
  untouched.
- Implementation, tests, lint, typecheck, build and browser/design gates pending.
  No product pass or live configuration claim at this checkpoint.
- No live provider/model calls, account setup, credential output or deployment.
