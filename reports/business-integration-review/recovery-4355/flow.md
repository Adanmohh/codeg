# Actual bounded protected browser sequence

Executed 2026-09-09, own `review-intake-recovery-4355` Playwright CLI session
(PID27696, closed after checks), Chromium1280×900, English, device appearance
rendered light. No browser state export or response fulfillment.

1. Independently compared all27 manifest assets against both disk and HTTP4355;
   `asset-verification.json` records immutable source/build/helper identity.
2. Ran `python3 .../prepare-cases.py prepare` once: exit0,10 protected API
   responses200. This created two new private review candidates and prepared
   one of them. It deliberately refreshed only the existing review-owned
   sourceA/sourceB; no other namespace, task, setup, grant or lifecycle write.
3. Opened a new named CLI browser, installed `browser-guard.js`, and ran
   `browser-login.js` with the approved0600 review credential file. Cold
   pre-login business API count0; actual context200, review member/org,
   operatorfalse, credential form detached before snapshots.
4. Waited for the actual five-minute source deadlines to expire (both
   10:39:32UTC); no clock mock, DB update or fixture control. Opened Sources,
   Review synthetic customer meetings, sourceA, new Pending1/revision1.
   Observer confirmed metadata_only, draftnull, hasPreparedDraftfalse and no
   Task title field. Screenshot/snapshot `unprepared-withheld*`.
5. Clicked actual Refresh source access then Read next step. Real protected
   imports/start + advance completed against synthetic4352. Same candidate
   revision1/source1 became fresh with empty editable fields, review owner,
   no comparison. Typed the two documented synthetic sentinels; Save private
   draft enabled. Did not save. `unprepared-restored*` and browser check data.
6. Clicked Sources; explicit Leave this source review dialog appeared.
   Chose Discard my draft. Opened sourceB/new Pending1/revision2. Actual read
   remained metadata_only, hasPreparedDrafttrue, draftnull; no Brief field.
7. Ran `python3 .../prepare-cases.py newer` once: exit0,6 responses200.
   It refreshed only sourceB and edited only the new prepared candidate2→3.
   Original browser remained on its withheld revision2 baseline.
8. Clicked browser Refresh source access. One Read next step click timed out
   because the now-fresh source collapsed the details section; see
   `attempts.json`. Opened the existing summary and completed that same queued
   import. The comparison explicitly showed local basis2/current saved3 and
   the new exact notes; editor remained absent. `prepared-comparison*`.
9. Clicked Use the current saved draft. Brief now matched the exact newer
   notes, header revision3, comparison gone. Opened Review before sharing;
   confirmation remained unchecked and Accept into shared work disabled.
   No save/accept/link/discard-candidate action. `prepared-adopted*` and
   `prepared-unconfirmed*`.
10. `browser-finish.js` asserted seven checked states,70 business responses
    all200, zero blocked/legacy/external attempts, zero candidate/task writes,
    exactly four browser import start/advance actions. Every response was
    real; guards only continued allowed loopback requests or would abort.
11. Ran `python3 .../prepare-cases.py verify` once: exit0,6 responses200,
    five persistence assertions pass. New unprepared candidate remains1/null;
    prepared remains3; old IUI candidate remains3; prior public text unchanged;
    public task count2. Read-only inspection, no direct DB access.
12. `playwright-cli -s=review-intake-recovery-4355 close` exit0. All4351/4352,
    4354/4355 listeners, user/root/worker browsers, earlier fixtures and
    records remain available. No restart/reseed/export/build/install.

The complete sanitized DTO/response ledger is `browser-results.json`.
API operation IDs and statuses are in `api-prepare/newer/verify.json`;
`browser-cases.json` identifies only the deliberately created synthetic
records. Credentials are absent from every committed artifact.
