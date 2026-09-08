# Frozen permission correction evidence

Source: `74bde8b6f1aeee135122cf52f78746eec36c6602`, parent
`9a4c8c882cec938665bc233b4d658d8de019ccfd`. The review reruns the original
`intake_store_`, `intake_task_`, protected-router and `independent_intake_`
selectors. The first three use an unchanged archive; the last uses a separate
archive with only the published additional tests. The test addition SHA256 is
`33fd516f1979aba67699d79c0d24fdaad5f1b6b88b776cbe3f422fea55716d55`, identical
to the original failing probe. The context offset in the patch changes because
the product file gained its correction and new test.

Use `results.json` and unmodified Cargo logs for the final execution results.
`74bde8b6-probe-correspondence.json` verifies unchanged tests, and
`74bde8b6-source-verification.json` records all 733 source files matching the
commit and the sole additional-test file in the probe archive. Full source
manifests remain in the worktree's local `.build/review-business-intake/evidence/`.

To reproduce, archive this exact commit into a new owned directory and run the
first three selectors with the command in `results.json`. In a separate copy,
apply `74bde8b6-independent-permissions.patch` at its root and run
`independent_intake_`. Use your own Cargo target. Never apply reviewer tests to
another worker's product worktree or reuse an existing credential/fixture store.

The original failing logs and patch remain under `../60daf42e/` unchanged.
Protected-router evidence uses in-process mock HTTP, not a listening server or
browser E2E flow. No provider or native keyring runs in these checks.
