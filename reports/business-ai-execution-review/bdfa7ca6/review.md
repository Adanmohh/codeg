# Published bytes and reserved-object recovery — source-only review

**No additional blocking source finding** in the bounded follow-ons at `3841e659bec3d0161d73eaca16058b170fe751be` and `bdfa7ca69334196448c3745a8ee0aac97166a877`. These heads and their added tests are **uncompiled and unexecuted by this reviewer**. No internal-helper result is promoted to transport, import, launch or native acceptance.

Own `gh api` resolved both immutable commits and their parent chain: d0d56a → 3841e659 (tree `9244afb4a5e61899088764feae787ba25dbae728`) → bdfa7ca6 (tree `a1977d7b59bd34ec2915e823119c1bb59bc8a2da`), exits 0. Complete diffs, final publication/files implementations, three added test bodies, the changed late-authority case and relevant existing controls were read directly from Git. Exact blobs, hashes, read scopes and installed references are in [source-ledger.json](source-ledger.json). No new source archive was created.

## Published byte authorization

[publication.rs:332](https://github.com/Adanmohh/codeg/blob/3841e659bec3d0161d73eaca16058b170fe751be/src-tauri/src/business_execution/publication.rs#L332) first resolves the exact committed tenant/task/deliverable/asset/version through the unchanged task-owned reader. That reader checks current task Read using the supplied Principal. It preserves the intended named-reviewer boundary; it does not require or reconstruct private E1 operator authority for a published result.

The same cloned Principal and exact reference survive the bounded file read. The function repeats the task-owned read after I/O, checks retained object ID/hash/size, and returns no Content if revalidation fails. Underlying identity checks retain the captured epoch, credential/member status and native-session marker. The file helper uses the server-owned object selected from storage, checks sealed mode, descriptor identity/size and hash, and never reads a caller path or the mutable source. The return value contains metadata and bytes, without an object ID or storage path.

Only text/plain and application/json preview is allowed by this core. Other supported formats remain download-only; unknown formats fail. The filename helper bounds and sanitizes its output, but this is not a tested HTTP header or active-preview implementation. No route/native command is registered. Final delivery revalidation, content headers/Range refusal, preview isolation and transport buffering/lifetime controls remain the accepted consumer work from contract3f164. A buffered Rust result alone does not certify those boundaries.

## Publication cancellation and receipt preservation

[submit_with:156](https://github.com/Adanmohh/codeg/blob/3841e659bec3d0161d73eaca16058b170fe751be/src-tauri/src/business_execution/publication.rs#L156) keeps the original publication path and adds a private pause point after the real blocking verifier, before the final writer. The callable production wrappers always pass `std::future::ready(())`; callers cannot supply an action/future through the DTO. Tests are child-module users of the private implementation. There is no generic execution or approval capability.

The semaphore permit stays inside each blocking job until that job finishes. Only the awaiting async continuation can reach the existing task CAS/reference/activity/receipt transaction. The adapted late-authority case now runs actual file verification before task cancel/reopen and expects rejection with unchanged business tables. The new abort case waits for the post-verification signal, holds the continuation, aborts it, awaits a cancelled JoinError and compares stored state. Its control flow targets late commits at that exact boundary; it does **not** prove cancellation of an already running blocking read, a process effect or a queued final-writer transaction.

The new content case first reads exact retained bytes/hash/filename after deleting scratch, then changes the viewer's member row to revoked after actual I/O and expects both the in-flight result and subsequent read to fail. Its snapshot covers task/assets/publication/receipt tables; it intentionally excludes the member row that the test changes. This is a planned membership-revocation check via SQL, not an executed credential API, epoch race or browser test.

## Recovery never recopies a missing object

[files.rs:436](https://github.com/Adanmohh/codeg/blob/bdfa7ca69334196448c3745a8ee0aac97166a877/src-tauri/src/business_execution/files.rs#L436) validates the reserved object UUID and opens only that existing object through the anchored directory descriptor. Missing/unsealed/invalid objects return errors. It has no admission, source-path or staging fallback from which to recopy scratch bytes.

The shared [retained helper:246](https://github.com/Adanmohh/codeg/blob/bdfa7ca69334196448c3745a8ee0aac97166a877/src-tauri/src/business_execution/files.rs#L246) extracts the previously reviewed R3 logic: regular single-link bounded file checks, sealed mode, stable descriptor hash/size, then file and directory synchronization before Retained. Fresh staging's existing-object branch calls the same helper. Production recover supplies real `sync_all`; test fault injection remains confined to the existing test-only entry. Non-Unix remains unavailable. This source comparison does not transfer the earlier9949 execution results to the changed helper.

The added recovery test proposes missing-object refusal, original same-inode/hash reuse despite changed scratch, refusal of an unsealed0600 object without altering its bytes, and deleted-object refusal without recreation. The “partial” branch is specifically an unsealed full-byte object, not a crash/truncated-write simulation. The existing injected-sync control still traverses the shared retained logic through staging. None was run at this follow-on pin.

`recover` remains an internal filesystem primitive, not an authorization or receipt capability. The future import consumer must derive its exact object/hash/size from the retained authorized claim, fence that claim/receipt with the original authority, and choose recovery rather than fresh staging on replay. That consumer does not exist in these two deltas, so end-to-end no-recopy/replay acceptance remains pending.

## Evidence, provenance and limits

Source inventory now finds eight `execution_publication_` and six `execution_assets_` cases. These are **counts of test bodies**, not fourteen new passes. The two new publication cases, one new recovery case and changed authority case have been assessed. At closeout, root allocated the next bounded build window to tickets; its next frozen source/results have not been received here. No reviewer compilation is authorized. No Cargo/check/native/export, fixture, browser, provider or cleanup command ran in this source-only follow-on.

Installed Tokio1.49.0 `task/blocking.rs` and `runtime/task/join.rs` ground blocking-job cancellation limits and explicit abort/await confirmation. Installed Rust1.98.0 source `core/future/ready.rs` shows the production pause future immediately resolves. Earlier Rust file, SeaORM1.1.19/SQLx0.8.6 and code-context grounding remains pinned. The earlier dependency RAG lookup lacked the approvals corpus; no new coverage or installation is claimed. Removed enforcement hooks were neither restored nor probed.

Only the four execution source/test files and owner report differ from d0. `git diff --quiet` confirms NOTICE/LICENSE, dependencies/lock/build script, migrations, identity/tasks/receipts/scope/types and HTTP/native registrations unchanged, exit 0. Existing Apache Codeg file and task attribution therefore remains applicable; no new third-party source port or dependency is introduced.

Reviewer branch: `review/business-integration`. The d0 verdict remains frozen at `d1fba7e4a5367b05bd93c7e58d3e3d8f78b98c89`; the 9949 independent closure remains at `de3a6b8fdc2602ef743ec2e1da72f488707980af`. Their evidence directories and all prior177/3773 files are byte-identical, verified by exit-0 Git comparison. This publication adds only the current main-review opening and this report/ledger/digest directory. All old targets, exports, fixtures and paused work remain preserved.
