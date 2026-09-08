# Bug workflow host review

Accepted and merged, 2026-09-08. Historical review follows below. Early source checkpoint
`506dcc305bb11999eba495634088f0d394d34bc5`. Root read full operator, process,
runtime, review and held-fix adapters plus scoped store helpers and Python host.

## Final acceptance

Reviewed final head `8703e00fae2e1c92e045936b140b83012cfe0f57`, merged PR #9
as `02e3f5d8a15a3fee792cd0625966dee6c500d184`. Final commit changes reports only;
product integration is `11d703c5`. Root checked the additive five registration
resolutions, both-parent preservation and migration order 000005/000006/000007.
Reviewed host/UI behavior is unchanged by that merge. Tokio citation correction
matches locked 1.49.0; no dependency or behavior upgrade occurred.

Root independently ran combined library Ops tests on the integrated product:
167 passed, 3 manual browser fixtures ignored, exit 0, 6.75s. Log:
`/tmp/ops-host-integrated-independent.log`. Earlier independent 17 host, 9
isolated Python and 8 frontend tests plus actual protected CLI evidence below
remain applicable. Worker integrated gates pass both runtime checks and Clippy,
13 Desk, 18 ticket, 18 email transport and 32 frontend tests and typecheck.

No unresolved blocking host finding. This accepts the operator intake, exact
issue filing and held-fix-task path. Pi cached reads/proposals, typed phone
review, final design corrections and rebuilt integrated native artifact remain
separate required follow-ons. No live provider/model or actual fix/build claim.

## Independent evidence

`PYTHONDONTWRITEBYTECODE=1 .venv/bin/python -m pytest -q -p no:cacheprovider tests/test_host.py`
from the worker
`integrations/hafidh-intake` passed nine tests in 0.80s, exit 0. Log
`/tmp/ops-bug-host-independent-python.log`. This covers actual isolated host
process reads/revalidation plus closed-input/private-output failures, not Rust
host wiring or a live Hafidh account. Requested the subprocess tests use `-I`
to match the production launch exactly.

## Changes requested

1. Held-fix reuse calls the inherited global `other_active_with_same_source`,
   which filters source/status/deletion but not folder. Before linking its ID,
   check the current authorized folder and source identity; foreign-folder
   collisions must fail clearly. Cover valid same-folder reuse and rejection.
2. Existing `(product_id, ulid)` links can outlive a product folder/repository
   rebind. Detail/create must not expose or reuse an old unrelated task under
   the new binding. Verify the live linked row and current created receipt.
3. HostIssueAction checks connected tasks only in `running`, while preparation
   and shared RunContext permit `awaiting_input`. Verify overlapping pending
   proposal behavior with a regression, preserving cancellation/run checks.

The held fix starts in the inherited Canceled state to avoid automatic scheduler
claiming. Worker must prove its Review in Tasks path and manual resume are
usably discoverable, and explain the state without claiming an executed fix.
No engine rewrite requested. Registered API tests now pass independently (below). P1 fixture browser scenarios, provenance and final worker gates remain pending.

## Corrections reviewed and independent host suite passed

Scope/rebinding/pending review fixes are committed at
`fd7bcdce92716c8eb2874b6c7acac51b3ca708f1`. Creation and detail share live
linked-task verification against current folder/product/account/source/created
receipt. Foreign global-source collisions fail; same-scope reuse remains.
Repeated pending calls return the identical live proposal without another CAS
or wait; an unrelated await rejects before revising the draft.

The subsequent source invalidation and HTTP boundary tests are at
`4fd9e3a1457cf46db834aef98d4669f23a8b6f5b`. New list attempts invalidate previous
product freshness before reading, including failure/cancellation; only a
successful explicit get restores freshness. Root independently ran:

`CARGO_TARGET_DIR=/Users/mohamedadan/projects/ops-desk/src-tauri/target
CARGO_BUILD_JOBS=4 cargo test --locked --no-default-features --bin codeg-server
--lib ops_intake_host`:17 passed, one manual browser fixture ignored, exit0,
1.86s. Log `/tmp/ops-bug-host-independent-rust.log`. Tested source became the
4fd9e3a1 commit during the run; final hashes matched:
operator.rs a9dfc897f9577ec062c54355f6b688ff73054113c420df91c8c8839d3bc85a6c;
store.rs f6688360a998421edda73f11cbdbc6df2bc65e37beedffcbc80ae669295383ee;
tests/http.rs 9712b48aaef791a0a4d8d40804eadd09ac0ea2e6f386d6aeda12a1b53a42f115.

The earlier worker HTTP run failed because its fixture inserted an invalid
scope mode name; corrected to the accepted `read` enum before this successful
independent run. No gate semantics were relaxed. Tests cover real protected
router inputs, four-proof requirements, tampered/stale/denied reviews, unknown
issue reconciliation and held-task scheduler/manual-requeue behavior.
No live GitHub issue, Hafidh mutation, model call or actual fix is claimed.

## Independent frontend and browser checkpoint

Reviewed full BugWorkflowPage lifecycle and final UI correction: existing pending
payload remains visible after preparation invalidation, approval reflects stale
state, and proof edits lock during pending/unknown/created states. Root passed
all8 frontend tests,exit0, `/tmp/ops-bug-independent-frontend.log`.
Isolated production-aligned Python9-test rerun also passed,exit0,.76s:
`/tmp/ops-bug-host-independent-isolated-python.log`.

Actual protected CLI4322 login/missing config/mobile labels, source invalidation
and refresh, exact human-approved fixture rejection and created-issue-to-held-
Tasks navigation passed. [Evidence](browser-bug-independent/README.md).
No live outbound request/model/task launch. Final committed report/gates and
exact-head review remain before acceptance; cached pi reads follow separately.
