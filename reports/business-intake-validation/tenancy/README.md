# Evidence correspondence

The product cleanup and final source snapshot are committed at
`e55f3bfd1f069d6d6111370223993596b19ecb9b`. `source-correspondence.json` hashes
the91 listed files at that checkpoint. It is a final snapshot, not a claim that
every historical log used identical source.

The earlier merge/epoch/setup checks and exploratory intake runs are preserved
unchanged. Their exact sequence and assertion corrections are documented in
`reports/business-intake.md`. `tests-intake-final.log` is the complete green
38-case run with the final production cleanup. The subsequent recovery test
change only replaces `drop(keys)` with a lexical block; that one changed test was
rerun in `test-recovery-scope-final.log`. Desktop Clippy and desktop/server checks
then passed on the final snapshot. No case was removed or weakened.

Command arrays, exits and wall times are in the two `*-results.json` files;
typecheck was a separate `pnpm exec tsc --noEmit --incremental false` exit0.
Logs are raw, including terminal blank lines and inherited warnings. Thus
`git diff --check` flags three raw-log trailing blank lines; product source has no
whitespace finding. `SHA256.json` retains artifact integrity rather than editing
the original logs to silence a whitespace check.

The later manual fixture is test-only and has separate source/runtime evidence.
No fixture, real provider, native keyring, model or packaged application was run
by the commands in this directory.
