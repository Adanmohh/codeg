# Phase 1 Design Studio specialist review

2026-09-08. **BC-1–18 pass within the documented synthetic browser scope. Both
verified P2 findings are fixed and rechecked on the final combined export.
One low login-landmark finding remains open pending its assigned correction.**
No remaining blocking design finding was established in the covered states.
Root owns final acceptance and packaging.

Sole Astra/max worker, branch **review/design-phase1**, worktree
`/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`; draft
**https://github.com/Adanmohh/codeg/pull/16**. No extra agents, product fixes,
root planning edits, paid flow SDK, live sends or model calls.

Final accepted product **2fb837977bc8584c5a5ccbe640e2ffe490ab0609**; combined
export/merge **ab771db4dfd1de5b9ac4aaf8206a17e130886dff**, pushed. Earlier evidence
checkpoints:5e137304,08c3cf5e,43eff896,89e94faa,ce8aae7f,ebf551c0.
PR14 integration **19a698bc1108a5c3dee5018337e6da6eadc27f61** was based on
**8c2a004d**. Its unchanged Ops/phone behavior supplies retained functional
captures; the final export rechecks PR18/19's changed badge/drawer surfaces.

## Methods and verdict

Applied the actual **aesthetic-judge, a11y-auditor and flow-validator methods
sequentially**, followed by design-reviewer synthesis and inline finding-verifier.
These are one worker's methods, not independent delegated judges. Actual selector
outputs, pure lint/probe analysis and mechanically merged findings are committed
in [the evidence directory](design-phase1-specialist/).

| Dimension | Score /10 | Basis |
| --- | ---: | --- |
| States |9 |Real empty, failed load/retry, missing configuration, stale and terminal states. |
| Feedback |9 |Saved/error text retained; clear send/recording semantics and receipts. |
| Accessibility |8 |Labels, keyboard flows and settled rings; low landmark remains, no assistive-technology certification. |
| Responsive |9 |1280/390 light/dark and actual RTL/back/email direction; no observed horizontal overflow. |
| Visual/tokens |8 |Inherited Inter/neutral/teal hierarchy; scan limitations classified without unrelated restyling. |

Design-weighted verdict: **8.1/10** using Design8×40%, Usability8.75×30%,
Creativity7×20%, Content9×10%. The operational list/detail system is coherent;
this is not an award-level novelty or whole-application conformance claim.

Structured outputs: [BC verdicts](design-phase1-specialist/findings/bc-results.json),
[aesthetic](design-phase1-specialist/findings/aesthetic-judge.json),
[accessibility](design-phase1-specialist/findings/a11y-final.json),
[flow](design-phase1-specialist/findings/flow-validator.json),
[merged findings](design-phase1-specialist/findings/merged.json),
[verification](design-phase1-specialist/findings/verifier-final.json).

## Findings and accepted corrections

**Resolved P2 — both running-count badges.** Baseline10px amber700 text measured
4.39:1 on the actual composite. Exact locations:
`src/components/conversations/sidebar-conversation-list.tsx:506` and
`src/components/conversations/sidebar-folder-group-header.tsx:206`.
Accepted handoff **6aefaa59c72f3aa5f2d9dd284cea0d365a111e05**, merge
**a8663104f97dd5728fab3a8594bb94841d54533e**, uses inherited amber800 ink.
Both implementations now measure **6.187 rest/focus,5.287 hover light** and
**9.090 rest/focus,6.944 hover dark**, all above4.5.
[Actual12-state evidence](design-phase1-specialist/flows/final-badge-summary.json).
Folder uses the real protected API. Group uses explicitly labelled frontend-only
overlays of three read-list responses. Cleanup restores real lists, no DB writes,
remainingGroups0, provider4→4. Settled screenshots show actual2px focus rings.

**Resolved P2 — large sidebar travel under reduced motion.** Baseline rAF
evidence proved341.5px travel over450ms under reduce. Exact source:
`src/components/ui/drawer.tsx:339,349`. Accepted handoff
**edd8add757b69d6f368cf813f14aae1f1d738eb2**, merge
**4611d025ba49790adbda780e8d6d44c20e75a9fe**, adds
`motion-reduce:transition-none`. Both final themes have **zero transform-animation
frames and zero intermediate x frames** during open/Escape/outside dismissal.
Normal motion retains450ms opening/400ms closing with43 intermediate opening
and38 intermediate closing frames. Reduce still has closed/open endpoints and
computed duration values; property **none** prevents animated travel.
Popup detachment, keyboard entry and returned focus pass, document390px.
[Raw frames](design-phase1-specialist/flows/final-drawer-results.json),
[summary](design-phase1-specialist/flows/final-drawer-summary.json),
[settled focus](design-phase1-specialist/flows/final-drawer-settled-results.json).

**Open low — login content lacks a main landmark.**
`src/app/login/page.tsx:61` wraps the single connection form in a div; the actual
probe records main=false. Reproduce by opening `/login` and inspecting landmarks.
Required fix: change the outer content tag to main, retaining classes, controls,
error/retry and locator behavior. Root independently verified and assigned
`fix/design-login-landmark` to another worker. Keep open until accepted targeted
recheck. This one-form page has no competing navigation and passes BC-1.

Impact-per-effort follow-up: that single low correction. The two larger fixes
are closed; no invented second/third open issue. Earlier loop findings now have
actual labels, Ops focus, truthful terminal copy, RTL/back/email direction and
reply locale-preservation evidence. Root's separately committed seven-field
locale round trip supplements this worker's reply sentinel; it is not claimed
as this worker's own seven-field run.

## BC-1–18 evidence

[bc-results.json](design-phase1-specialist/findings/bc-results.json) contains the
per-assertion verdict, exact screenshots/results and limits. First point of
failure in completed assertion runs: **none**. Superseded harness attempts remain
identified below. Result/screenshot names in this table are beneath the evidence
directory's flows/ and screenshots/ directories.

| Check | Actual observation / evidence |
| --- | --- |
| BC-1 |390px invalid login, associated error and protected retry retaining issue locator; phone-login-results.json. |
| BC-2 |Selected Ops and settled light/dark2px keyboard ring; navigation-results.json. |
| BC-3 |390px final drawer lifecycle; actual Arabic RTL/Back180°/emailLTR and reply locale round trip; gaps-results.json. |
| BC-4 |Two desktop/mobile selected headings and histories; second-thread-1280-dark.png and both mobile selection captures. |
| BC-5 |Private note before/after, private=true and no public outgoing reply; editor-storage-verification.json. |
| BC-6 |Actual empty inbox/setup and filter; browser offline failure and real retry; empty-results.json/gaps-results.json. |
| BC-7 |Persistent From/To/Cc/Bcc/subject/body labels and full open headers in1280/390 light/dark crops. |
| BC-8 |Invalid To retains body; corrected same draft saves revision2; screenshots and protected post-state. |
| BC-9 |Native confirmation/dismissal, retained reply and settled3px focus; discard-dialog.txt/discard-settled-results.json. |
| BC-10 |Complete pending payload and explicit Approve and send reply; full review crops. No email send repeated in this audit. |
| BC-11 |Controlled revision1→2 makes pending proposal stale and disables old send; editor-poststate.json. |
| BC-12 |Four seeded accepted/unknown/failed/denied states, distinct copy and no another-send action; decisions-results.json. |
| BC-13 |Synthetic key removal reveals Connect Resend and no sent claim. Pending proposal is already stale; not a fresh unconfigured approve-preflight test. |
| BC-14 |Actual Finish recording receipt removes action, provider4→4; terminal-2-390-dark.png/receipt-finished-1280-dark.png. |
| BC-15 |Actual populated and fully empty Morning; selected real review destination; surfaces-results.json/empty-results.json. |
| BC-16 |Four missing proofs, disabled preparation, invalid proof retained; settled mobile captures, DB drafts1/proposals0/filings0. |
| BC-17 |Exact repository/title/body/labels/four proofs plus human confirmation; actual created/unknown/read-only reconciliation/rejected and used-link invalidation. |
| BC-18 |1280/390 light/dark Inter/neutral operational layout, one readable mobile pane, no decorative metrics or observed overflow. |

Fresh4328 phone counters: created **GitHubPosts0→1,issues0→1**; unknown attempt
**posts1→2,issues1→2**, then read-only reconciliation **posts2→2**; rejection
**posts2→3,issues stay2**. Telegram stays3 throughout. All provider traffic is
loopback and no approval JSON is intercepted. Exact payload/proof hashes are
in phone-review-results.json. Open workspace reaches `/workspace`, matching
its corrected label. Earlier accepted email send/preflight evidence is cited
as prior evidence rather than silently counted as fresh UI decisions.

## Palette, motion and evidence quality

Original `docs/design/BRIEF.html` remains unchanged. Actual Design Studio
canonHex only lowercases/expands hex; it cannot convert authored OKLCH.
The labelled [measurement copy](design-phase1-specialist/probes/BRIEF.measurement.html)
uses **actual browser Canvas** to normalize original opaque light tokens. Dark
and composite values are not silently added to make lint pass.

The71 baseline captures contain2188 ranked text samples. Original palette flags
**694** become **444** after Canvas conversion; family79, spacing124 and radius76
flags remain. [Classification](design-phase1-specialist/probes/classification.json)
retains raw evidence and exact source. Dark muted#262626, foreground#a1a1a1 and
primary#9bd4c5 are inherited pairs in globals.css:136. Actual alpha composites
include dark muted/70 #1e1e1e, input/30 #151515, light primary/8 #eef2f2 and
private-note amber/5 #fffaf2. They depend on the backdrop, not separate tokens.
Inter fallback-stack comparison and inherited rem radii/6px gaps are scan-coverage
limits. Real badge contrast was still promoted and fixed despite inherited
provenance. No product tokens were changed to satisfy string comparison.

All **18 painted empty-valued select samples** include selected option text,
including All statuses. **56 closed-details field records** use checkVisibility;
none was counted as painted. Disabled/group-opacity/image/filter exclusions
are explicit. Clipped sr-only badge duplicates remain raw but are not distinct
painted defects.

Baseline duration counts **173→173** made the generic reduced-motion flag false.
Final same-DOM counts are **166→166** in both themes,45 visible nodes; the same
heuristic still returns **false**. It counts duration even with property none
and includes color/border/opacity feedback. Actual rAF/lifecycle evidence closes
the large-motion defect; the heuristic is not rewritten to true. Four final
settled probes add **268 text samples, zero contrast/unnamed/overflow failures**.
[Final measured summary](design-phase1-specialist/probes/final-drawer-measured-summary.json).

The11 static and14 measured lint runs completed exit0; that does not mean no
flags. [Lint classification](design-phase1-specialist/lint/classification.json)
explains fixed four-proof arrays, imported Action/Button states and synchronous
Morning navigation using actual source and rendered behavior.

Original phone first-frame focus had transparent shadow and **is not a pass**.
Subsequent still-pending350ms screenshots show a3px ring and opaque mint border;
root confirmed the timing artifact. Final drawer child focus likewise uses
settled350ms paint, not immediate pseudo-class alone. Original58px mobile drawer
sliver/empty dismissal captures remain under [attempts](design-phase1-specialist/attempts/).
Replacements wait for popup detached and record popupCount0/documentWidth390.
The blank native-dialog result is backed by actual screenshots/protected state
and a separately repeated confirm flow. A final hidden-sidebar locator timeout
is retained; explicitly opening the sidebar corrected the harness setup.

## Grounding, source mapping and validation

Read complete founding/orchestrator/status/decisions/AGENTS, brief, BC checklist,
loop1 reports and actual Design Studio method/probe/lint/reviewer files.
Design Studio **0.8.0 @55c8614dcfff33b4caa5a544b4f1f91877214878**, MIT.
[Report NOTICE](design-phase1-specialist/NOTICE.md) records exact source-to-glue
paths, commits/blobs and full MIT attribution. Root NOTICE entries are preserved;
no new product port or AGPL/enterprise source. Codeg's founding
**v0.30.4 /6f6bd648b206412644842a98d9ffeebf57292bed** pin remains unchanged.

Installed reads: React19.2.4, Next16.1.6, Tailwind4.1.18, Base UI1.7.0 popup and
useAnimationsFinished lifecycle; TypeScript5.8.3 DOM Canvas/visibility/animation;
Node24.19.0 with @types/node25.2.2 fs; Playwright CLI0.1.18 and
core1.63.0-alpha-2026-08-05 locator/route/fulfill/unroute types. Local source
preceded evidence glue. Remote research uses gh api immutable refs, with no
borrowed-version upgrades; no latest-doc fetch was needed for these local APIs.

Code-context guide used existing rag-skills `.venv/bin/python`, **HF_HUB_OFFLINE=1**,
and returned headed CLI/concrete-done guidance. Docs retrieval **exit3**:
data/code/approvals.db absent. No fabricated corpus coverage, download or ingest.
Live own-session **01a07c1c-d3a3-7c22-a5b6-cedce2970d8d** Pre/Post hook evidence:
[hook-live-final.json](design-phase1-specialist/methods/hook-live-final.json).
No hook disabled/bypassed.

Only three **test-only** source adapters differ from accepted main: existing
ignored email design, issue-phone and intake-host fixtures. They add explicit
port/export overrides, preserve defaults and literal127.0.0.1 binding, and bind
issue review_origin to the chosen port using existing typed configuration.
Owner reviewed them before commit. No dependency/lockfile/migration/runtime
change. Fixture DBs and build outputs stay outside commits.

[Commands/exits and logs](design-phase1-specialist/methods/validation.json):
final combined `CODEG_EXPORT_DIR=out-design-final pnpm exec next build` **exit0**,
33 routes. Earlier desktop/server locked cargo checks, frontend tsc and correctly
scoped desktop/server Clippy all **exit0**; all three final fixture files pass
scoped rustfmt. Initial Clippy omitted test-utils for integration tests (**101**),
then corrected runtime command passed. Whole-tree fmt found inherited differences
(**1**); no broad formatting. Inherited proc-macro future-compatibility and debug
sidecar-placeholder warnings remain disclosed; no native bundle claim here.

Final folder/group/motion/settled CLI and pure merge/probe commands **exit0**.
Ignored fixtures compiled/passed startup assertions but remain serving, not
completed test suites. No broad repetition after accepted frontend fixes, per
owner instruction. Root reports82 focused tests, actual4318 motion/focus/empty
checks, isolated native migration through008 and1006 bundled files matching its
export in docs commit **8376d5c0**; those are **root-attributed**, not worker-run.

## Fixture handoff and limits

Owned fixtures all use **out-design-final** and remain running after documented
mutations. Existing4320/PID30815 `out/`,4323/PID45025 `out-telegram-issues` and
4326/PID63050 `out-design-ops` remain untouched/listening.

| Owned port/PID | Synthetic-only token | Current state |
| --- | --- | --- |
|4327/1475 |`ops-design-synthetic-operator` |Provider4; draft1revision2, proposal1pending/stale, receipt2finished; key removed. |
|4328/46726 |`ops-issue-phone-synthetic-operator` |All three locators consumed; GitHubPosts3/issues2/token exchanges1, Telegram3. |
|4329/77051 |`ops-intake-synthetic-operator` |Missing proofs; draft1/proposals0/filings0; invalid proof remains browser-local. |
|4330/83330 |`ops-intake-synthetic-operator` |EMPTY=1 variant; zero inbox/task/queue data. |

4327 DB: `.build/design-ops/2e3036b6-afeb-4e5c-ba19-aa2304e0c921`.
4329 DB: `.build/intake-host/browser-4bbf0d5d-383a-4fba-84e3-8fdf60d7ec12`.
4330 DB: `.build/intake-host/browser-b7cab94e-05e7-450a-af9f-144b3c8306ca`.
4328 SQLite is in-memory, auxiliary directory
`/var/folders/6k/w2fh6wy167726g5nr9zm_ph80000gn/T/.tmpLemf4y`.

Launch pattern from this worktree's src-tauri:

```sh
CODEG_OPS_ACCOUNT_ID=1 CODEG_DESIGN_FIXTURE_PORT=4327 \
CODEG_DESIGN_FIXTURE_EXPORT=out-design-final \
CARGO_TARGET_DIR=target-approvals CARGO_BUILD_JOBS=4 \
cargo test --locked --no-default-features --lib \
ops_design_browser_fixture -- --ignored --nocapture
```

Use4328 plus ops_telegram_issue_browser_fixture, or4329 plus
intake_host_browser_fixture;4330 also uses OPS_INTAKE_FIXTURE_EMPTY=1. Choose
a free owned port for any new instance; never run defaults over an existing
owner. Root may inspect current fixtures. No fresh pending issue locator remains,
so future decisions require a separate new fixture, not replay. These completed
checks need no reseed.

Limits: Chromium emulation only; no screen-reader session, iOS WebKit/physical
touch certification or document-start CLS/long-frame capture. Remote phone use
requires a configured reachable protected origin; loopback links prove only
synthetic flow. Same-user filesystem trust is not a hostile-process sandbox.
Expected unavailable Git/stream/Pi fixture diagnostics mean no clean-console
claim. Provider acceptance does not prove recipient delivery. This is specialist
evidence for root synthesis, not whole-app WCAG or final native design acceptance.
