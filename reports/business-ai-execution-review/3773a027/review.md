# E1 admission and file prerequisite review

**Three source-only P2 findings remain open at product `3773a027a8d580bd9bac1808efdb718ae6f9e135`.** Root independently read and confirmed all three; tickets has the required corrections. These are internal, unexposed helpers. No remote exploit, operational route, actual engine launch, import/publication transaction or tenant/native execution is demonstrated or accepted. The independent schema verdict at177 remains valid.

Baseline `177d0f3e2b426d65de9573ba534ac8c11f9250f7`; product tree `8fee327f94ac0d3b6b180180b80118d6d3bf66d5`; owner evidence-only `79703eeb50a9632d86d6c390bf7260d921593539`. Full new modules and eight new test bodies, the complete delta, migration14 lifecycle triggers and retained SQL constraints were read. Later PR movement does not change this pin.

## R1 / P2 — a returned session cursor can be unusable

Location: [session_store.rs:142](https://github.com/Adanmohh/codeg/blob/3773a027a8d580bd9bac1808efdb718ae6f9e135/src-tauri/src/business_execution/session_store.rs#L142), selection150–151 and filtering155–158; [scope.rs:107](https://github.com/Adanmohh/codeg/blob/3773a027a8d580bd9bac1808efdb718ae6f9e135/src-tauri/src/business_execution/scope.rs#L107).

Trigger: two same-task/operator rows are ordered by descending ID, the first revoked/stale and the second valid. With `limit=1`, list selects both, computes `nextCursor` from the first, then filters that first item after `AuthorityChanged`. It returns an empty page and a revoked cursor. The next request passes that cursor to `scope::session`, which rejects it; the valid session is unreachable. Revoking an originally valid cursor between pages causes the same failure.

Required fix: make issuance and consumption consistent without relaxing authority on returned items. Any structural anchor must stay bound to the same org/member/task; using a stale anchor must not disclose its private payload. Traverse leading/interleaved stale rows safely. Regression gates: leading stale, interleaved stale, issued cursor revoked between pages, valid pagination, foreign-task and real foreign-tenant cursors. Valid current items must remain reachable once, with no withheld item disclosed. **Source deduction only; no reviewer execution.**

## R2 / P2 — completion does not bind a receipt to its admission

Location: [session_store.rs:166](https://github.com/Adanmohh/codeg/blob/3773a027a8d580bd9bac1808efdb718ae6f9e135/src-tauri/src/business_execution/session_store.rs#L166), [complete_launch:306](https://github.com/Adanmohh/codeg/blob/3773a027a8d580bd9bac1808efdb718ae6f9e135/src-tauri/src/business_execution/session_store.rs#L306), [receipts.rs:80](https://github.com/Adanmohh/codeg/blob/3773a027a8d580bd9bac1808efdb718ae6f9e135/src-tauri/src/business_execution/receipts.rs#L80).

Trigger at the current internal API: reserve genuine pending starts A and B for the same original operator; replace writable `A.operation_id` with B's operation ID, then complete A. A's generation/admission/profile checks pass. Receipt completion independently validates B's current session but never compares its task/session/generation/resource to A. The source permits A to become idle with its linkage while B's receipt becomes confirmed with a result describing A; A's receipt stays pending. SQL only requires syntactically valid result JSON.

Required fix: private Admission authority/binding fields and read-only runner accessors, preserving non-Clone/non-Serialize semantics. Under the same writer, completion must match the exact stored operation kind/task/session/generation/resource to the admission before changes. Preserve generation/admission checks and duplicate-completion denial; use a consuming handoff for launch permission. Regression: cross-pair two real reservations and require complete unchanged snapshots of both receipts/sessions/generations; correctly paired completion succeeds once and replay issues no new permit. **Source deduction, not a caller-controlled wire exploit.**

## R3 / P2 — retained-object retry skips failed synchronization

Location: [files.rs:343](https://github.com/Adanmohh/codeg/blob/3773a027a8d580bd9bac1808efdb718ae6f9e135/src-tauri/src/business_execution/files.rs#L343), new-object synchronization368–373.

Trigger: staging copies/validates/syncs bytes and changes mode to0400, then file sync at372 or directory sync at373 fails. The object remains readable with the expected mode/hash. Retry takes343–355 and returns `Retained` without retrying either synchronization. Reading cached bytes does not confirm the failed metadata/directory persistence; a future consumer could record durable custody after this unconfirmed failure.

Required fix: recovery must complete required file/directory synchronization on the anchored validated object before success, without truncating/replacing it. If reopening, recheck identity. Keep corrupt/partial/hardlink/wrong-mode rejection. Deterministic gates: inject post-chmod file-sync and directory-sync failures separately; persistent failure stays failed, successful retry returns the same object/hash/bytes and confirms the missing durability steps. Later import integration must prevent asset/version/publication/confirmed receipt before durable custody. **No fault injection or crash test ran here.**

## Established boundaries and remaining consumers

Actual operator marker plus existing identity authorization gates the helpers. Reads bind organization/member, captured authority/tenant epoch, task scope and profile revision. Start validates revisions/readiness under the existing writer; replay yields no new Admission. Member/tenant triggers revoke sessions. These checks must still be exercised through actual async engine/event/file consumers.

Unix descendant opens use directory descriptors, no-follow flags and exclusive private creation. Source checks reject nonregular/hardlinked/oversize files, bound depth/entries, compare descriptor metadata before/after reading and hash copied bytes. Enumeration provides names only; bytes are reopened through anchored descriptors. Retained objects are outside scratch; corrupt/partial objects are not overwritten. This trusts the host/data-directory ancestor and is not OS isolation. Non-Unix methods fail closed.

Pending with committed consumers: aggregate scan byte/time budgets and blocking-work cancellation, controlled mid-copy mutation, commit-time authority revocation after file work, exact operation/object custody and receipt-loss recovery, selected task/publication/receipt CAS, `Deliverable.assets` with historical `[]`, and engine operation/resource/event/file/child enforcement. These are prerequisites, not additional demonstrated defects in absent routes.

## Evidence, grounding and preservation

**No reviewer tests ran at3773.** No Cargo, new archive, output copy, cleanup, fixture/browser/provider/native action occurred. Root observed2.5GiB, later own `df` showed117MiB. A native report patch failed with `Failed to write file`; Git/status verified the old report intact. A smaller patch through the same native tool succeeded before the writing hold arrived. Read-only inventory then identified unused incremental caches; none was deleted. Root recovered approximately17GiB through its own cleanup and explicitly resumed small report publication. Reviewer Cargo remains held while tickets owns the next bounded window. No denied edit was rerouted and no hook state/key inspected.

Owner797 logs were independently hashed and results read:12 tests pass (schema4/admission4/files4),0 failures/ignored,0.80s runtime/1m57s compile; server check11.50s. **Owner execution only.** Logs retain58 unused-consumer server warnings and9 test warnings; no Clippy pass. Pre-copy change is tested, mid-copy change is not; synthetic EngineLink is not a real process or protected transport. Exact hashes and source blobs are in [source-ledger.json](source-ledger.json). Applied migrations10–13, Cargo manifest/lock and LICENSE match177; only unaccepted14 gains lifecycle triggers. Actual dated paths were enumerated before comparison.

Code-context used existing rag-skills Python with `HF_HUB_OFFLINE=1`: guide0 (six mostly generic rules; exact per-task staging applies), docs3 (missing `data/code/approvals.db`). No ingestion/install/coverage claim. Generic context7/browser directives from other projects do not override this gh-api-only source review and compile hold.

Installed Rust1.98.0 rendered source was read first: sync_all748–780, metadata1124–1144, clone1142–1183, read_dir3210–3235 and Unix MetadataExt identity/link/time docs. Raw rust-src is absent. Installed libc0.2.180 openat/mkdirat/fchmod/Apple flags were read. Official gh API confirms Rust `88d9e12ae178fab0fb5cc050a94da85685d449ea`; relevant source was also read. File drop ignores sync errors and is not a durability substitute. Existing SeaORM1.1.19/SQLx0.8.6/serde1.0.228 grounding remains in177 evidence.

Complete Apache2.0 Codeg upload_jail at9e61fe was read; gh API verifies its blob and all five B source mappings appended to NOTICE. No third-party code was added by this reviewer; original attribution is preserved. Actual own Git/gh/authorized Herdr tools succeeded. Before the owner's hook-removal override, the native source-ledger patch emitted a manual-grounding reminder that explicitly disclaimed automatic verification of this surface. Relevant installed/pinned source had been read first. This is historical tool evidence only, not a fresh-hook or all-tool audit claim. No generic Pre/Post JSONL was available. The subsequent owner override assigns global removal elsewhere; this lane performs no hook changes, repairs or additional probes.

Next execution should use only the corrected owner `business_execution::session_tests` / `business_execution::file_tests` plus focused R1–R3 controls when an existing-target window is explicitly released. No unchanged schema rerun is requested. Findings were relayed to root/tickets; correction re-review requires an immutable head. Branch `review/business-integration`; prior f4 evidence is retained.
