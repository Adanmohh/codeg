# Root business workspace integration review

In progress. UI product `33b9cbcb63d3023e3ccd36a21f9cb9e4e2e425b7`,
reviewed through the rebrand-owned static export on loopback4340 and actual
guarded Rust API on4342, product `1ba73e3c`. The latter is now accepted main
via PR22 merge `5541857a`. No production response mocking, provider or model
request is used. This is not final native artifact acceptance.

## Independently exercised UI flows

Root used Playwright CLI sessions `root-business-final` (original synthetic
operator), `root-business-member` (new personal manager), and
`root-business-viewer` (new personal viewer). Personal credentials were issued
by the real protected API and passed directly into the UI inside the same script;
no token was returned in tool output or written into evidence/storage.

- Created `Root acceptance: customer onboarding brief` through the UI, with
  brief and `2028-02-29` deadline. No git folder, chat or agent required.
- Personal manager `b74e348e-064b-4165-aa13-4daf58c9b258` opened the same task
  through Shared work and moved it from To do to In progress to Review.
- Original operator's stale revision1 edit returned actual HTTP409. Its local
  brief draft remained intact, including after resizing to390px and loading the
  current saved version. Comparison offered explicit keep/discard actions.
- Discarded only that synthetic local draft, checked the human review confirmation
  and accepted the current task. Actual review request returned200. Manager
  refreshed, reopened and saw Done with unchanged `2028-02-29`.
- Separate personal viewer `478fd519-a65b-40f3-8861-4e4c03b3ab22` saw the same
  Done record, the date and `Amal · Synthetic owner · Reviewed work` activity.
  Create/edit/accept controls and the engineering link were absent. Manager's
  engineering link was also absent. Backend forged-request denials are covered
  by the independent backend review and worker API evidence.
- Five cold entry paths (`/`, `/index.html`, `/business`, `/business/`,
  `/business.html`) reached the sign-in UI with zero observed API requests while
  a synthetic ambient legacy token was present. Root repeated this in a fresh `root-business-cold` browser with a scoped
  request listener removed in finally, waiting for network idle after each
  rendered sign-in: all five passed with zero API requests. That browser is closed.

Root visually inspected desktop list and390px human-review activity screenshots.
The final independent Design Studio review and merged-source export checks remain
open. Root relayed a recovery-clarity question: current-version comparison omits
current status/revision while the top header still shows the stale version.
The reviewer will classify it; no source fix or acceptance waiver is implied.

## Probe corrections and limits

A strict Done locator matched both the status badge and activity text; root
corrected it to the first matching badge, then verified reviewer activity by its
actual text. A viewer observation was sampled during loading; waiting for the
actual Done badge produced the passing result. These are test observation errors,
not production corrections.

The first viewer setup attempt ended with Playwright `Session closed`. Its preceding
cold-route request listener used the sandbox's unavailable Node URL global and
remained installed; that is a suspected probe cause, not an established app defect.
Root recreated only its own viewer browser and used a separately named synthetic
member. The completed viewer check passed. No existing worker record was changed.

No broad Settings snapshot, credential trace, live email, App installation, ad
spend, provider call or deployment occurred. Existing worker fixtures and exports
were preserved. Final signoff will distinguish worker tests from these root checks.

## Full frontend regression suite

Root independently tested an unchanged git archive of integrated UI candidate
`b97e6bd9`, using root's existing installed Vitest2.1.9: **6,194 tests across438
files passed**, exit0,30.91s. Command from the archive:
`node /Users/mohamedadan/projects/ops-desk/node_modules/vitest/vitest.mjs run`.
Log: `/tmp/root-business-full-frontend-b97e6bd9-direct.log`.

Initial `pnpm test` stopped before tests because pnpm's archive dependency-status
check proposed removing the symlinked modules directory and refused without a TTY.
No install/purge was approved. Root read the package script (`vitest run`) and
used that exact installed runner directly; no dependency or product edits.
Final corrections, if any, require affected regressions and accepted-export checks.
# Conflict correction checkpoint

Root reviewed committed `095c61642dc2f52ca8fd6e7c10556f16cf61c904` and ran its
unchanged source archive with installed Vitest2.1.9: workflow.test.tsx **13 passed**,
exit0,2.15s. Log `/tmp/root-business-conflict-095c6164.log`. The correction labels
the retained draft base and displays saved status/revision; it preserves the
existing CAS lock and resets review confirmation after explicit adoption.

Actual corrected-export browser acceptance is pending. Root opened its own
`root-business-conflict` session at4346 and created only synthetic task
`cac9ba87-7f3f-4201-b29b-886c82c7514f`. Actual progress changed revision1 to Review;
the stale edit returned409 and retained the draft. That export still carries
the old comparison: do not attribute this browser run to the committed fix.
The owner was notified to publish the corrected export identity explicitly.
An initial selector used “New task” instead of the actual “Create task” and
timed out before creation; a later named-region wait timed out on the old export.
No mutation was retried blindly and no passing correction claim is made.

Corrected-export follow-up: once4346 explicitly served
`.build/business-workspace/conflict-export`, root reauthenticated and created
new synthetic task `bbfa9c14-086d-478f-b509-4d10b0fb79d2`. Actual progression200
changed it to Review/revision2. Stale update409 preserved the exact draft;
the named base group showed To do/revision1 and the saved region showed
Review/revision2. Save stayed disabled. At390px the labels remained visible;
root inspected [capture](conflict-corrected-390.png). Explicit adoption/save
persisted the draft at revision3, confirmed by UI and actual get200. Metadata
editing correctly invalidated the earlier review to in_progress. A final
text locator matched both detail and activity and was narrowed to the first
visible detail; no mutation was repeated. This completes root's bounded
conflict-correction browser check; the broader independent UI audit continues.

## Final search correction review

Root reviewed `e72cc44b612068e67a3e6dc3bc593f10988ae7ed` source/NOTICE and
independently ran an unchanged archive:39 tests/5files pass, exit0,1.80s
(`/tmp/root-business-corrections-e72cc44b.log`). Coverage is workflow15,
client11, provider isolation8, UI3 and session2. The scoped placeholder override
preserves global Input; tabIndex-1 skips the redundant clipped submit while
EN/AR Enter submission remains covered. No new dependency or backend behavior.

Root independently verified4346's final `.build/business-workspace/review-export`:
served and on-disk business.html both SHA256
`c029f99d5c92e677c97f2ebe45f4bd185746b73e84e77912a683e1d92de55a54`,
nodePID60964, guarded backend4342. Tracked src/src-tauri/NOTICE matched e72
at inspection. Root recomputed contrast from the worker's12 measured Canvas
samples:10.72 light and7.07 dark; each case records44px visible Refresh focus,
Enter200 and no page overflow. This is review of worker measurements, not a
second root browser matrix. The independent reviewer is performing its own
targeted final-export closure. Root's conflict browser is now closed.
