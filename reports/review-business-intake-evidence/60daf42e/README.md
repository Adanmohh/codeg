# Frozen prerequisite evidence

Source: `60daf42e79fa7dc10f8118b9cdb8a73b2c07e80d`. The four new owner tests and
one existing protected-router test pass independently in an unchanged archive.
The additional two permission tests fail independently and reproduce review R1.
See `results.json` and the unmodified command logs for exact results and limits.

To reproduce, archive that commit into a new reviewer-owned directory. Run the
first three selectors from `results.json` without source changes. In a separate
copy, apply `60daf42e-independent-permissions.patch` at its root and run selector
`independent_intake_` with the same locked/offline/no-default-features command,
using only your own Cargo target. Do not apply this patch to a product worktree.

The patch only appends tests; its `NOTICE` records the source test and licence.
The unchanged archive's 728 source/integration/licence files were individually
Git-blob verified; `60daf42e-source-verification.json` summarizes that check.
The complete manifest and both original archives remain under the approvals
worktree's `.build/review-business-intake/` directory.

No listening fixture, provider, actual credential store or native keyring was
used. The protected router is exercised through axum-test's mock HTTP transport;
this is not browser or listening-server E2E evidence.
