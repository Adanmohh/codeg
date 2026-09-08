# Independent business workspace review

Review in progress. Independent protected-browser checks pass for cold route isolation, two personal sessions, shared human work, real 409 draft retention, explicit revision adoption, and private draft preservation through other-tab locale changes at 390/768/1280. **One P3 recovery-clarity finding is reproduced; its committed correction is source-reviewed but still needs the corrected browser export. No final acceptance verdict yet.** The completed task-source review in `reports/review-business-tasks.md` remains unchanged and pushed at `a8d1dbbebf5cfef57fb489f5417f303fa8dd422f`.

## Frozen source and boundaries

- PR21 handoff: `a126730274c8a3dc822345da536eebed1f8dc253`, resolved through `gh api`.
- Frontend product: `33b9cbcb63d3023e3ccd36a21f9cb9e4e2e425b7`, independently resolved through `gh api`; `git diff --quiet` against a126 across src/src-tauri/manifests/lock/NOTICE exits 0. Subsequent handoff changes are documentation/status only.
- Task fixture dependency: `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5`, already independently reviewed for R1. The UI branch has not yet integrated this task product.
- Reviewer branch/worktree: existing `review/business-tasks` in `/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`. No product edits, other-worktree writes, dependencies, new agents, live providers or model launches.
- Evidence ownership: `reports/review-business-workspace-evidence/`; unchanged source archives only under own `.build/`. Existing fixtures, paused files and earlier report evidence are preserved.

## Method and current progress

Read the complete frozen UI report and contract, root BW-1–18 checklist, current brief and visual direction. Applied code-context and read the actual Design Studio audit/checklist, aesthetic/a11y/flow/reviewer methods and relevant control/form/navigation references. These specialist methods will run **sequentially in this reviewer**, with actual Playwright CLI evidence and pure local lint/probe analysis; no paid flow SDK or additional agent.

The owner’s business scope and inherited theme conventions take precedence over generic marketing-page recommendations. Review covers the implemented shared work/people/review surfaces, not nonexistent business integrations or a whole-business readiness claim. BW-9/10 backend authority uses the separate exact-head task review; BW-16 native and BW-17/18 final integrated package gates remain root/owner follow-ups unless independently exercised here.

Offline code-context `guide` exited 0. It returned the relevant “one route/job-to-be-done” and installed-version grounding rules alongside unrelated examples. `docs` exited 3: this worktree has no `approvals.db` dependency corpus; no coverage was invented or installed. Direct local pinned source/types are used instead: React 19.2.4 and Playwright CLI 0.1.18 confirmed. Design Studio is `55c8614dcfff33b4caa5a544b4f1f91877214878`.

Root explicitly authorized separate Synthetic review identities/tasks on UI `http://127.0.0.1:4340/business.html`, guarded task API 4342. This review uses CLI sessions `business-review-a` (browser PID95227) and `business-review-b` (PID98431), individually minted manager/member credentials held only in invocation/client memory. No fixture restarts, exports, old/root record edits or other browser control. Task `e64a0ac7-cc34-4442-a90b-ceaa25a86323` and three members prefixed `Synthetic review` belong to this review. A temporary same-context preferences tab was closed after the locale test. All request interception is guard-only; production task responses are unchanged.

## Reproduced finding and current correction

**BUI-R1 / P3: conflict comparison hides current status/revision.** At frozen `src/components/business/task-detail.tsx:189`, Load current stores a separate comparison; lines220–231 still paint the old task header, while lines247–253 and SavedTask733–829 omit the current status/revision. Trigger: manager opens revision1, member submits a deliverable at Review/revision3, manager saves old metadata (real HTTP409), then chooses Load current task. Header remains To do/Revision1 and the comparison omits both current fields. Required fix: identify the retained draft base and show current saved status/revision before adoption. Independent evidence: `conflict-compare.raw` and `conflict-current-390-light.png`; Save remained disabled and the exact draft was retained. Explicit adoption then saved the draft at revision4, returning the task to In progress and invalidating the old review as intended. This is confusing recovery presentation, not an observed stale-write/approval bypass.

Source correction `095c61642dc2f52ca8fd6e7c10556f16cf61c904` was read through exact `gh api`: existing StatusBadge + labelled draft-base group + current region, including English/Arabic text and status/revision/archive indicators. Mutation/CAS logic is unchanged. The two new assertions cover base/current status/revision and renewed human confirmation after adoption. Root independently ran workflow13/13; not counted as reviewer-executed tests. **4346 has not yet been verified to serve this correction; root reported an old export there. Browser closure remains pending rebrand's exact export handoff.**

N1/P2 fresh native launch was discovered by the tickets reviewer and confirmed/assigned by root. This browser-only review does not independently certify native startup; track the separate correction/packaged recheck rather than treating source routes as native evidence.

## Executed checkpoint evidence

- `cold-route-retry.raw`: five cold paths200, zero API requests/WebSockets/external attempts; 1280 document width. First attempt failed in the CLI VM before API activity because URL is not provided there; preserved as `cold-route.raw`, report-only harness corrected.
- `connect-{manager,member}.raw`: real identity API setup, separate personal sessions, no stored bearer and no engineering entry. No real credential values logged.
- `create-shared.raw`, `member-submit.raw`: actual UI creates a human-only task, calendar date2026-10-01, distinct owner/assignee/reviewer names; member progresses and submits revisions2/3, no execution, no member Accept work control.
- `private-draft-locale.raw`: actual separate tab chooses Arabic/English; title/brief/date survive390/768/1280; note/deliverable/review drafts also retained; Escape prompts, Keep editing retains, deliberate discard closes. No draft or member bearer in local/session storage.
- `conflict-adopt-save.raw` is an unsuccessful harness assertion (it awaited an Accept work button after a metadata update correctly invalidated review). `conflict-adopt-settled.raw` preserves the actual revision4/In progress result and retained saved text. This attempt is not counted as a passing assertion.
- Design Studio selector actually selected aesthetic/a11y/flow/motion; static lint and actual matrix/probes continue. Exact raw counts and exceptions will be preserved.

## Pending independent evidence

Actual 390/768/1280 light/dark and RTL composition, visible settled focus and contrast; cold route/no legacy requests; two-member shared data and revision conflict; private draft preservation across viewport/locale plus teardown on identity changes; review, permission, revocation and recovery states. Findings will give severity, immutable file/line, reproducible trigger, evidence and required fix. Worker screenshots/test counts will be attributed separately from reviewer-run results.
