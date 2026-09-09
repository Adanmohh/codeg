# Independent candidate refresh review

**No new blocking finding in the bounded source/component review at `60d600db0f224d44ff191490ab79bc25530ba959`.** Five independently executed focused cases pass. Actual protected browser recovery on the new export remains pending its immutable 4355 handoff; the running 4354 product and completed IUI-1 verdict at `1974d97f44b69db96051ace1c68717cc8c27b6e9` remain unchanged.

Compared all product/NOTICE changes from 1974 to 60d. Only `candidate.tsx` changes product behavior; `workflow.test.tsx` adds two cases. The other NOTICE additions attribute earlier fixture/evidence work. Read the changed functions, surrounding read/mutation/adoption/visibility guards, test fixtures, both new test bodies and relevant existing tests. The exact source and command evidence is under [ui-60d600db](business-integration-review/ui-60d600db/).

At [candidate.tsx:209](https://github.com/Adanmohh/codeg/blob/60d600db0f224d44ff191490ab79bc25530ba959/src/components/business-intake/candidate.tsx#L209), a changed candidate revision, source revision or rebase flag still selects comparison/adoption. The added same-version branch requires fresh candidate disclosure, fresh source disclosure and a current source deadline. Its functional setter reconstructs only null fields, preserving a local draft. Prepared-but-withheld destination text remains null; confirmation is reset before every read. Existing serial/unmount/active guards still reject superseded replies. No transport, principal, persistence or backend authorization changes occur here.

| Independent execution | Actual result |
| --- | --- |
| Exact committed new tests, with only `candidate.tsx` replaced by exact 1974 product in the new reviewer archive | **1 failed, 1 passed**, exit 1, 2.43s Vitest. The unprepared parent-refresh case fails on the missing Task title input; the explicit newer-version adoption control passes. This is an intentional failing baseline, retained verbatim. |
| Restored exact 60d product; five unchanged selected tests | **5 passed, 15 unselected**, exit 0, 0.913s Vitest / 1.1401s command wall time. Covers same-version restoration, explicit newer-version adoption, local draft retention through locale/view changes, destination-private withholding and revoked source hiding. |
| Post-run source correspondence | **1,235/1,235** archived source/manifest/license files match exact 60d bytes again. No reviewer product/test patch remains. |

The [test-results.json](business-integration-review/ui-60d600db/test-results.json) records full argv, paths, timings and the intentional baseline exception. Both raw logs are retained. These are JSDOM component tests with synthetic client responses, not real protected transport or provider evidence. Owner-reported 33 tests/typecheck/lint are separate evidence and were not repeated wholesale.

Evidence staging initially returned 1 because `*.log` is ignored; only these two named logs were force-added. Staged `git diff --check` returns 2 solely for Vitest's final blank line in each raw log. The original bytes are retained; no product/source whitespace failure is present.

Docs first: React 19.2.4 and `@types/react` 19.2.13 were read locally, including `SetStateAction` and `useCallback`; installed Vitest 2.1.9 CLI/help/config and the existing test helpers were read before execution. The archive reuses installed modules through a symlink and has its own small Vite cache. Its source payload is 17,737,103 bytes; no dependency install, export, new Rust target, fixture, backend record or user-browser action occurred. Code-context's missing worktree dependency corpus remains disclosed in the AI/B reports, not represented as complete coverage.

Borrow attribution is preserved in the additive `NOTICE` block: Codeg 1974 candidate/test helpers, Apache-2.0; React/Testing Library/Vitest are installed API references, not copied dependency implementations. Immutable GitHub commit/blob verification is recorded in [source-ledger.json](business-integration-review/ui-60d600db/source-ledger.json). No third-party source was ported by this report, and no product NOTICE entry was changed.

Findings/results were relayed to root and rebrand. Branch `review/business-integration`; AI authority work remains the priority. All previous reports, paused files, source archives, fixture listeners and the user-inspected browser are preserved. The initial IUI-1 closure and its real API/browser evidence remain in `6895bedf425db263c2d6eb1dd73641f34521f439`, not retroactively attributed to this newer frontend.
