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
