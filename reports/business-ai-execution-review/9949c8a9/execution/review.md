# Independent execution of the E1 corrections

R1–R3 are closed within the internal helper scope at product **`9949c8a9ad99ffb70232c05bf1c2ad243dcaf194`**, tree `84fbbc223c58efd661ea386eebad734b65d4403b`. Independent unchanged-source execution passed **14 tests, zero failures, zero ignored**, following the complete source assessment published at reviewer checkpoint `42d5978b2fde232b8c066131407323955148ed71`. No additional blocking finding was established in this correction delta.

## Results and exact attribution

| Prefix | Passed | Exit | Cargo compile / test runtime |
| --- | --- | --- | --- |
| `execution_pagination_` | 3 | 0 | 2m16s / 0.29s |
| `execution_admission_` | 3 | 0 | 0.97s cached / 0.29s |
| `execution_authority_` | 3 | 0 | 0.53s cached / 0.26s |
| `execution_assets_` | 5 | 0 | 0.62s cached / 0.10s |

[results.json](results.json) contains each actual argv, working directory, Cargo PID, timestamp, exit, raw log path and disk samples. Commands ran sequentially, using `cargo test --locked --offline --no-default-features --lib -j 2` and exactly the four prefixes requested. Tool execution session 71295 confirmed the runner's final exit 0; each child exit was collected with `wait()`. There was no failed iteration, assertion adjustment, added reviewer test or product patch. Six cases are the new corrections; eight are prior controls.

The four raw logs are [pagination](execution_pagination.log), [admission](execution_admission.log), [authority](execution_authority.log) and [assets](execution_assets.log). Each retains seven build warnings: unused internal consumers and the inherited linker unwind-size notice. The proc-macro-error2 future-compatibility notice is also preserved. No warning suppression or Clippy result is claimed. Unfiltered staged `git diff --check` exits 2 solely for the four raw logs' terminal blank lines; preserving those exact bytes is intentional. The documentation/JSON check excluding raw logs exits 0. Runner elapsed values include five-second monitoring intervals; use the raw Cargo and test result lines for compile/runtime timing.

These are independent reviewer executions. The separately verified owner 14 tests and server check at `b15ded12cd24f557d0de9bb55b43db30b9f8bc89` remain owner evidence in the unchanged [source-phase index](../owner-evidence-verified.json). No server check was repeated here.

## What the passes establish

- **R1 pagination:** leading/interleaved stale sessions do not consume eligible page slots; revocation of a returned cursor does not prevent continued own-scope traversal; that revoked session remains unreadable. Foreign task/member/tenant cursor positions are rejected alongside valid own results. These exercise actual admission and provisioned identity controls; labelled foreign retained rows remain schema fixtures.
- **R2 receipt/admission:** cross-operation substitution, stale generation and changed authority are rejected with complete stored table snapshots unchanged. Valid A/B completion and replay controls pass. Original operator, other-tenant, profile away/back, task cancel/reopen and fresh-login controls do not reconstruct historical authority.
- **R3 durability:** both injected first-sync failure positions and repeated recovery failures remain errors until real file then directory synchronization succeeds. Recovery preserves the same inode/bytes and one object after scratch removal. Existing symlink/hardlink, changed observation, corrupt object, oversized file and replaced-parent denial controls also pass.

These tests exercise internal database/file helpers on synthetic temporary data. They do not launch an agent, stream output, call a provider or implement a public route. `complete_launch` still borrows an Admission; real runner one-time consumption, captured-authority checks around future async effects, launch recovery, mid-copy mutation, managed import and human task/publication/receipt CAS remain later consumer gates. No macOS crash/power-loss durability or non-Unix runtime test is claimed. E2 tenant/native isolation and E3 account snapshots are outside this verdict.

## Exact staging, capacity and release

Own `gh api repos/Adanmohh/codeg/commits/9949c8a9ad99ffb70232c05bf1c2ad243dcaf194` returned the exact product/tree above, exit 0. Staging reads immutable Git blobs for `src-tauri`, `integrations`, NOTICE and LICENSE into `.build/business-integration-review/9949c8a9/source`; 18,865,417 new source/asset bytes were written. No whole worktree, build output or dependency cache was copied.

The unchanged tracked `src-tauri/vendor/sacp-tokio` is a symlink to the already existing exact vendor directory under the earlier177 source archive. Every reused blob was verified before and after. [source-before.json](source-before.json) and [source-after.json](source-after.json) each match **802/802** expected Git blobs and SHA256 values; no extra staged file appeared. The prior177 reviewer-only test patch was not carried into this source.

The sole compiler target was the existing absolute `.build/business-integration-review/target-15bb402b`. Fresh disk immediately before the first Cargo start was **15.829 GiB** at 2026-09-09 15:44:08 UTC. Each subsequent prefix started above 10 GiB. A five-second monitor would interrupt only the current review process group near 5 GiB; this threshold was never reached. The minimum observed free space was **14.818 GiB**; the final release observation was **14.867 GiB**. Shared filesystem values can change because of other system activity; these are actual measurements, not reserved capacity.

Cargo PIDs were 63296, 67391, 67872 and 67966. Every child was reaped before **all-command release at 2026-09-09 15:46:43 UTC**. Herdr prompts to root wR:p1 and tickets wR:p4 acknowledged each actual start/exit and the final release, all exit 0; exact messages and timestamps are retained in results.json. Report work followed that release. No fixture, browser, old process, native artifact or other target was changed. No cleanup occurred.

## Preservation and publication

Historical3773 findings, f4/177 schema evidence, and the four source-phase9949 report/ledger/digest files remain byte-identical to reviewer checkpoint42d5978b. Their then-pending statements describe that earlier checkpoint; this execution report supplies the closure. The historical index's main-report hash belongs to that frozen commit, while this execution's [digest index](digests.json) covers the current main report and new evidence.

Installed/pinned grounding and exact Apache Codeg borrowing are recorded in the unchanged [source ledger](../source-ledger.json). No dependency/source version or NOTICE entry changed. No third-party test hunk was added. Normal Git/gh tools were used; removed enforcement hooks were not restored or probed, and no fresh-hook claim is made. The previous missing dependency RAG corpus remains disclosed in the main report.

Branch: `review/business-integration`. Deliverable: [review-business-ai-execution-implementation.md](../../../review-business-ai-execution-implementation.md). The publication commit is sent to root/tickets for immutable import. Only the current main review and this new execution evidence are staged; all paused/untracked files and prior artifacts are preserved.
