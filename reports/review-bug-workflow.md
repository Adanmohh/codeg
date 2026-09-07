# Bug workflow host review

In progress, 2026-09-08. Not accepted or merged. Early source checkpoint
`506dcc305bb11999eba495634088f0d394d34bc5`. Root read full operator, process,
runtime, review and held-fix adapters plus scoped store helpers and Python host.

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
No engine rewrite requested. Full registered API tests, P1 fixture browser
scenarios, provenance and final worker gates remain pending.
