# Source re-review of the three E1 corrections

**R1–R3 are addressed in source at `9949c8a9ad99ffb70232c05bf1c2ad243dcaf194`; no additional blocking source finding in this bounded correction.** Independent execution is pending, not passed or waived. Owner evidence at `b15ded12cd24f557d0de9bb55b43db30b9f8bc89` was verified: eight source blob/SHA pairs and five log hashes match, with14 helper passes and server check0 recorded. These remain owner execution. Farha owns the next production build/browser window; reviewer Cargo remains held. Runtime launch, transport, import/publication and tenant/native authority remain outside this internal-helper verdict.

Immutable commits were independently resolved with `gh api`, fetched through the normal Git client, and read directly from Git objects:

| Correction | Immutable source |
| --- | --- |
| R2 admission/receipt | `836fbdcf451b5d9181f8b6d13e14ba962f57f151` |
| R1 pagination | `5a231349edafa0a57d3fad8792ea1ac7b95f0787` |
| R3 durability / final product | `9949c8a9ad99ffb70232c05bf1c2ad243dcaf194` |
| Owner report only | `e56affc361ba7901e624dcc7e4d8e5a2776aee5b` |
| Owner executed evidence, no product delta | `b15ded12cd24f557d0de9bb55b43db30b9f8bc89` |

R2's parent is accepted-main integration `54daac22eba8aaab1e7b2e6da9e39a03e80e6be1`. The correction delta changes exactly eight files under `business_execution`; final tree is `84fbbc223c58efd661ea386eebad734b65d4403b`. Complete changed production functions, all six new test bodies, existing-test adaptations and the owner report delta were read. Exact blobs and command results are in [source-ledger.json](source-ledger.json).

## R1: eligibility and cursor position now agree

[session_store.rs:138](https://github.com/Adanmohh/codeg/blob/9949c8a9ad99ffb70232c05bf1c2ad243dcaf194/src-tauri/src/business_execution/session_store.rs#L138) first checks the actual current operator/task authority. A supplied UUID is then looked up only as a structural position under that same org/member/task. It does not authorize reading the anchor session. This permits an issued cursor to survive revocation while rejecting missing and foreign-scope positions.

The query at155 filters captured tenant epoch, authority JSON, task epoch, session status and joined profile revision/retirement **before LIMIT**. These stored predicates match the unchanged shared scope checks. Each returned item is still passed through `scope::validate_session` in the same transaction. Cursor selection therefore uses an eligible item and withheld rows consume no visible page slots.

The three new tests create real admitted rows, order their actual generated IDs and verify leading/interleaved stale traversal. Between-page revocation is paired with a direct get that remains denied. The scope test uses a separate real task, an issued same-org human credential, and an actually provisioned second tenant; retained foreign session rows are clearly labelled schema fixtures, not authorized E1 launches. Valid own rows remain visible and foreign cursors fail. This corrects the original source failure without making a cursor an access grant.

## R2: receipt target checked before and after state changes

[Admission:170](https://github.com/Adanmohh/codeg/blob/9949c8a9ad99ffb70232c05bf1c2ad243dcaf194/src-tauri/src/business_execution/session_store.rs#L170) has private fields, remains non-Clone/non-Serialize, and exposes only a read-only session ID accessor. The nested test module can deliberately corrupt private fields; production sibling consumers cannot.

[complete_launch:324](https://github.com/Adanmohh/codeg/blob/9949c8a9ad99ffb70232c05bf1c2ad243dcaf194/src-tauri/src/business_execution/session_store.rs#L324) loads the exact captured operator/kind/operation receipt and compares task/session/generation/resource to the admission's session before any linkage/status write. It also requires a launch kind, pending/uncertain receipt, current generation, matching admission/profile/mode and no previous engine linkage. The linkage UPDATE must affect exactly one row. Receipt completion repeats target and current-authority checks in the same writer transaction.

Two new tests use genuine A/B reservations and compare every column of session, generation and operation tables on rejection. Substituting B's operation ID into A fails without changing either reservation; restored A and untouched B complete independently. Stale generation and an actual profile discovery change are rejected with unchanged snapshots and a valid completion control. Existing exact-input replay, original-operator/tenant denial and away/back tests remain relevant controls.

The original mismatch is fixed in this DB completion seam. `complete_launch` still borrows `&Admission`; an actual runner must separately consume one launch permit before the external effect. No runner/launch handoff exists at this pin, so this source verdict does not certify one-time process execution or uncertain-launch recovery.

## R3: recovery confirms durability on the validated descriptors

[files.rs:375](https://github.com/Adanmohh/codeg/blob/9949c8a9ad99ffb70232c05bf1c2ad243dcaf194/src-tauri/src/business_execution/files.rs#L375) retains the opened object descriptor and hashes a `try_clone` of that same underlying handle. It checks mode/hash/size, then synchronizes the validated file and object directory before returning Retained. It does not reopen by a mutable path. Both initial staging and recovery propagate sync failures.

The production `stage` closure always calls real `File::sync_all`. The replacement callback entry is `cfg(test)`; the shared implementation is private. No production caller can select a successful no-op through the exposed helper. Existing source/retained-object identity, symlink/hardlink and no-overwrite behavior is retained.

One new deterministic test covers both first-failure positions: post-chmod file sync and directory sync. It then removes scratch, repeats failure at each recovery sync point, and checks the same inode/bytes remain with no success. A successful retry performs real file then directory sync, preserves a single object, and is followed by the production entry point. This directly targets the skipped-sync defect. It is a test-body assessment, not an independently executed crash or filesystem fault test.

## Execution attribution and remaining independent gates

The four prefixes select14 current tests by independent source-name inspection; six are new corrections and eight are prior controls. Immutable b15 logs record `execution_pagination_`3 passes/0.27s, `execution_admission_`3/0.30s, `execution_authority_`3/0.25s, `execution_assets_`5/0.09s, and server check0/27.46s. The owner manifest records all exits0 and exact locked/offline, existing-target, -j2 commands. All13 artifact hashes and eight source blobs were independently matched to Git bytes; test output and server finish were read. See [owner-evidence-verified.json](owner-evidence-verified.json). This verifies **owner evidence**, not execution by this reviewer. Seven test warnings and62 server warnings remain, chiefly pending consumers plus the inherited linker/future-compatibility notices. No Clippy or runtime pass is inferred.

For a later explicitly released reviewer window, the narrow new-case selectors are:

- `business_execution::pagination_tests` — three cases.
- `business_execution::session_store::completion_tests` — two cases.
- `business_execution::file_tests::execution_assets_recovery_retries_file_and_directory_sync_before_success` — one case with both sync-failure positions.

Keep the existing admission replay/authority and four earlier file cases as changed-boundary controls. No unchanged migration/schema suite is requested. Original-authority async revoke, actual runner single-use/recovery, mid-copy mutation, managed import and task/publication/receipt CAS require their later committed consumers and separate execution evidence.

## Provenance and preservation

The eight-file correction leaves NOTICE, LICENSE, manifests/lock, migrations, identity, tasks, intake, commands and web source unchanged from54; exact `git diff --quiet` exit0. Existing Apache2.0 Codeg attribution and pinned source mappings remain valid. This is owner glue over its already attributed modules, with no new dependency or third-party port.

Grounding reuses the installed/pinned sources recorded in the3773 ledger: Rust1.98.0 File metadata/clone/sync and SeaORM1.1.19/SQLx0.8.6 transactions. The installed Rust reference's visibility rules were additionally read: struct fields default private; the defining module and descendants can access them; `pub(super)` scopes the exposed item to its parent. This supports the private Admission plus child-test arrangement. The `try_clone` documentation confirms shared underlying handle identity. No new corpus coverage is claimed; the previous missing local dependency index remains disclosed.

Updated owner AGENTS forbids restoring enforcement hooks. Normal Git/gh commands succeeded after the same-session restart; no hook installation, repair, probe or enforcement claim occurred. Old hook failures remain historical. Only report/evidence paths are written in this lane. The dc3773 findings, f4 schema evidence, old targets, source archives, paused files and every fixture/browser remain intact. No additional agent, provider call or reviewer Cargo process was started. Corrections remain source-addressed/pending independent execution in this report checkpoint.
