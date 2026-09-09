# Output discovery — independent source review

**R4 / P2 remains open at `4251987071e8e7213a678be94039cd3fad1328f1`: a newer successful empty/no-op scan does not fence an older observation.** This is a source-confirmed defect in an unexposed internal helper. No reviewer Cargo, test execution, source archive, fixture or provider action occurred. The owner separately executed eight publication tests; the five output tests compiled but were not selected.

Own `gh api` resolves this commit to tree `21c7b03b9d6efa8c8f7eb89b2043c70f0dc02d23`, parent `bdfa7ca69334196448c3745a8ee0aac97166a877`. The complete six-file delta and both new output source/test files were read. The exact read scopes, blobs, hashes, commands and limits are in [source-ledger.json](source-ledger.json).

## R4 / P2: empty or unchanged scans leave the ordering fence unchanged

Location: [outputs.rs:251](https://github.com/Adanmohh/codeg/blob/4251987071e8e7213a678be94039cd3fad1328f1/src-tauri/src/business_execution/outputs.rs#L251), with the captured vector at line222 and retention at line255. The fence contains only existing output IDs/revisions. A successful scan that changes no rows leaves that vector unchanged; there is no separate accepted-scan counter in generation storage.

Deterministic trigger using the existing private `list_with` test seam:

1. Admit a synthetic session with no output rows, then write `brief.md` in its fixed workspace.
2. Begin scan A. Its captured revision vector is empty; its real file scan observes `brief.md`.
3. In A's `before_commit` future, remove `brief.md` and await a second list B. B scans the empty directory, commits and returns an empty page. It inserts or updates no output row.
4. Resume A. Its authority/generation still match and both revision vectors are `[]`. A therefore inserts and returns the disappeared file as `Available` after B's newer empty result.

The same gap occurs with unchanged retained rows plus a newly appearing/disappearing file: B's unchanged result does not invalidate A. The existing reordered-scan case changes a retained row from revision1 to2, so it does not cover this path. The consequence is stale candidate resurrection; this is not evidence of cross-tenant access, successful import of missing bytes or publication bypass. The reproduction above was reasoned from exact source, **not executed**. Root independently confirmed the source defect.

Required fix: compare and advance a monotonic generation-scoped accepted-scan fence in the same writer as retention, including every successful active empty/no-op scan. A rejected older scan must leave the newer accepted observations unchanged. Add the exact empty case, an unchanged-row case and a fresh-scan positive control while retaining the existing authority/generation checks. Stopped cached-only reads must not inspect late workspace bytes. The owner has proposed a sidecar in root-reserved `m20260909_000015_business_execution_scans`, with retention/FK/rollback/receipt tests; that proposal has no reviewed immutable implementation or executed result here. Migration14 must remain byte-identical. R4 stays open pending the committed correction and its evidence.

## Authority, scope and stopped generations

No additional concrete authority/disclosure defect was identified in this delta. `source` calls the unchanged `scope::session`: actual `Principal::is_operator`, current identity authorization, same organization/member, retained original grant and authorization epoch, current task scope and profile revision, and non-revoked/non-closed status. The same Principal is rechecked in the final writer after blocking file I/O. A role-owner credential is not upgraded to the original operator.

The source locator comes from that session's last bound generation/admission, not a caller path or engine ID. It records both the current session fence and actual bound workspace generation. Starting returns Busy; stop/generation changes invalidate an in-flight scan. Ended runs return only already captured observations. A newly bound generation uses its own rows and cannot accept the old generation's cursor. This assessment depends on the existing private admission records; real engine binding, stop/continue ownership and process teardown are still future runtime gates.

The reused Unix scanner anchors file opens to the server-selected workspace, excludes hidden/config paths and links, validates stable file identity/hash/size, and bounds entries/depth and individual files. Two blocking slots limit concurrency. These are not an end-to-end HTTP deadline or tenant OS isolation proof. Listing writes only private output observations; it does not stage a managed object, publish a version or create an operation receipt. Candidate projection exposes only the declared ID/revision/basename/type/size/UTC-time/status fields. Stored relative paths, admission IDs, filesystem stamps and hashes remain private. Import must still verify the selected exact observation and original authority when its consumer is implemented.

## Five output test bodies: coverage and limits

All names below have prefix `execution_outputs_`; **none was independently run, and the owner's publication selector did not run them**.

| Test suffix | Actual source coverage | Limit |
| --- | --- | --- |
| `revisioned_discovery_and_pagination_do_not_publish` | Three real files, unchanged keyset traversal, edit/removal revision changes, private DTO-field checks, zero asset-version rows. | No import, HTTP/UI rendering, or page traversal while the file set changes. |
| `foreign_cursor_and_member_transport_fail_before_scan` | Unknown cursor; real resolved owner-role credential denied at the core; another admitted session's valid cursor denied without output changes. | The foreign cursor is another session under the same operator/task/profile, not a second-tenant or HTTP transport test. |
| `late_profile_revocation_retains_no_observation` | Real scan followed by profile replacement through `sync_profiles`; final writer rejects and leaves observations empty. | No actual client/process revocation or new transport request. |
| `reordered_scan_cannot_replace_newer_observation` | Later nested scan changes a retained row to revision2; older scan conflicts and preserves its snapshot. | Misses R4's empty/unchanged accepted scan. |
| `stop_fence_and_new_generation_cannot_rebind_old_scan` | Synthetic SQL stop after scanning; cached stopped list retains old metadata; synthetic new admission/generation rejects old cursor and gets distinct IDs. | Uses synthetic engine links and direct SQL lifecycle changes, not the actual stop/continue runtime or teardown. |

Owner evidence `03c701dba3ade317a9597a49872530ddd2a17c93` records `execution_publication_`: eight passed, exit0, compile1m57s/runtime1.05s at this exact product. I read the committed report delta; these remain owner results, not independent output evidence. The log/source digests were verified by root. Assets6, outputs5, history2, task controls and server check were not run in that window. Final owner disk9.29GiB was below the next-command threshold. No reviewer allocation was taken.

## Grounding and preservation

The nine-line NOTICE addition names Apache-2.0 Codeg `business_intake/imports.rs` at `c8453a48d441f40eb47f9c9af856235b4f9986e5`, blob `c0338191beff26817d2bf85b943fd0c2602a47c0`. Own `gh api` fetched that blob and confirmed exact equality with local Git bytes, SHA256 `9cf444a632b46c49f13abe7473de47d4e890f3821d7941c26ae08ae0f3356f8b`. Its captured claim and final writer revalidation were inspected. The Apache licence is unchanged from the previously fully read accepted source; earlier NOTICE entries remain intact. No third-party implementation, dependency or licence source was added by this review.

Installed chrono0.4.43 `datetime/mod.rs:633/802` grounds `Observation::modified_at`: checked timestamp/nanoseconds to UTC, then RFC3339 formatting. Cargo.lock confirms chrono0.4.43, Tokio1.49.0, SeaORM/migration1.1.19, SQLx0.8.6, serde1.0.228 and UUID1.20.0. Earlier blocking/semaphore and writer documentation remains applicable. Offline code-context guide exited0 using the existing rag-skills Python; only its relevant “Atomic per-task staging” rule was applied. No dependency-corpus coverage or hook enforcement is newly claimed.

The primary report and this report/ledger/digest directory are the only reviewer changes. Protected-source comparisons exit0: no migration, dependency, identity/task, receipt, transport or native registration changes in this delta. Prior `9949`, `d0`, `3841/bdfa`, `177` and `3773` evidence remains byte-identical. R1–R3's independent fourteen-pass closure stays frozen at `de3a6b8fdc2602ef743ec2e1da72f488707980af`; the content/recovery source handoff stays at `2c011b27a53f34b6e625797ab4ec43460608e66f`. Branch: `review/business-integration`. No further execution is authorized without a new root allocation.
