# R4 correction — independent source assessment

**The source correction at `9e58f4e6a41569a96d79fc2af963acec6e5c5aa6` addresses R4's empty/no-op ordering defect. No additional blocking source finding was identified in this bounded review. R4 remains open pending compilation and execution evidence.** The eleven `execution_outputs_` bodies are planned tests, not passes. No reviewer Cargo, source archive, target/output copy, fixture or provider action occurred.

Own `gh api` resolves product9e58 to tree `9d7c7638f93fdda9484928c5602b0495b2605bb8`. Its parent is the earlier report-only03c701db. Successor `7e28988190176caea1ae405e7c3d31e82ec53bce` changes only the owner report and source ledger. Full product delta, final output helper, migration15, six added test bodies, and the two historical receipt-count expressions were read. Own verification matches the owner's seven changed source/NOTICE pairs and two preserved migration14 pairs. Exact sources, installed references and commands are in [source-ledger.json](source-ledger.json).

## Why the source addresses R4

[outputs.rs:185](https://github.com/Adanmohh/codeg/blob/9e58f4e6a41569a96d79fc2af963acec6e5c5aa6/src-tauri/src/business_execution/outputs.rs#L185) reads a persisted counter for the exact organization/session/bound generation. Missing state fails unavailable; it never invents counter0 in a request. After file I/O, the original Principal/task/profile/session checks still run. The final writer conditionally advances the captured counter by one before retaining candidates, regardless of whether an active scan found changes or even any files.

Thus B's accepted empty/no-op result changes the fence. An older A's conditional update affects zero rows and returns Conflict before retaining its stale candidates. Counter, candidate updates, response projection and commit share the same transaction. Statement/read/commit failure cannot return a successful page; a rollback covers the counter and observations together. The counter cannot exceed the existing safe-integer bound. This is source reasoning, not a newly executed race or failure test.

Stopped/ended reads take `found=None`, so they inspect no workspace and skip the counter update. They still revalidate original authority and the session/bound-generation locator. The retained stop test checks cached observations and old-cursor rejection; it does not explicitly assert the counter around the stopped read. The conditional skip was inspected in source. Future output-metadata writers must preserve this shared ordering fence; this delta introduces no separate import writer or runtime consumer.

## Additive migration and receipt retry

[migration15](https://github.com/Adanmohh/codeg/blob/9e58f4e6a41569a96d79fc2af963acec6e5c5aa6/src-tauri/src/db/migration/m20260909_000015_business_execution_scans.rs) adds a composite-key sidecar referencing the exact existing generation. Existing generations receive counter0 without changing their authority, workspace binding or output metadata. An insert trigger creates the sidecar for future generations within their transaction. Identity updates/deletion are rejected; the counter must be an integer within range and each update must increment exactly once. Downgrade refuses removal of retained ordering state.

The migration begins one transaction, requires FK enforcement already enabled and acquires SQLite's writer through the existing metadata row before inspecting its completion marker. Schema, initial rows, triggers and marker commit together. A receipt retry skips DDL/reseeding, checks generation coverage and FK health, and preserves accepted counters. There is no FK-disable window or pool reacquisition in this migration.

Installed SeaORM/migration1.1.19 confirms the distinction: SQLite `exec_with_connection` does not wrap all migration work; `exec_up` calls `migration.up` and then inserts the separate SeaORM receipt. The planned trigger-induced receipt failure therefore targets the real commit gap. Installed `DatabaseTransaction::commit` leaves the transaction open on failure; Drop queues rollback. These library reads support the transaction assessment, but do not replace executing the new failure tests.

Both migration14 Rust/SQL blobs are byte-identical to425. The registry adds15 after14. The two `receipts + 1` → `receipts + 2` edits are two assertions in **one** existing14 history/receipt-retry test, reflecting the current two pending migrations. Existing data, immutability, FK and receipt-loss checks remain. No14 DDL or existing historical fixture writer was changed.

## Six added test bodies and precise limits

The five earlier output cases remain; the following six additions bring the selector inventory to eleven. None has been compiled or executed at this correction head by this reviewer or in the supplied owner evidence.

| `execution_outputs_` suffix | Intended check present in the source |
| --- | --- |
| `newer_empty_scan_cannot_resurrect_older_candidate` | Exact reported nested `list_with` race: B accepts empty, counter1; A conflicts and leaves rows empty. A later fresh scan succeeds at counter2; visible session fields stay unchanged. |
| `newer_unchanged_scan_still_fences_older_candidate` | Existing row/revision unchanged by B, but counter advances; A cannot add its vanished extra file. A subsequent unchanged scan also advances without changing row/session snapshots. |
| `writer_and_commit_failures_preserve_fence_and_rows` | Output-insert trigger failure, then a deferred-FK COMMIT failure; both expect counter0/empty outputs and a later successful control at counter1. |
| `migration_retains_generation_and_real_receipt_retry` | Starts after real14 receipts with retained admission/grant/engine/output data; fails actual15 receipt insertion after schema commit, advances a test-only counter in that interval, then retries without resetting it. Snapshots cover all columns of twelve existing tables. |
| `migration_failure_rolls_back_schema_seed_and_receipt` | Late trigger-name collision after partial DDL/seed; expects no sidecar/marker/earlier new trigger, unchanged retained rows/receipts, then successful retry. |
| `migration_generation_fk_and_rollback_keep_exact_binding` | Future-generation insert seeds counter0 atomically; rollback removes the seed, commit keeps it. Rejects unmatched composite parents, parent/sidecar deletion, counter reset/skip, identity update and downgrade. |

The FK negatives use nonexistent organization/session/generation references, not a real second tenant's existing resources. Lifecycle links remain synthetic; direct SQL generation creation is not engine ownership/teardown proof. The added cases do not perform async task-abort or process-crash testing. The receipt-gap list is deliberately test-only and does not authorize serving a failed application startup. These limits remain explicit even if the planned selector later passes.

## Provenance, checks and preservation

The thirteen-line NOTICE addition preserves its entire prior prefix. Own `gh api` fetched and byte-compared both Apache-2.0 sources at accepted `c8453a48d441f40eb47f9c9af856235b4f9986e5`:

- `business_intake/sources.rs`, blob `56e8ca543207335e65469e17444af755c82b0dbd`: captured source-fence compare/update. SHA256 `876f9cd0da5220f03f2ad90b7fbadb1ad51d1fd6c4aae23daf35787006142614`.
- `m20260909_000013_business_intake_epochs.rs`, blob `19d3436cf4e3e5a6e337df72de55fbb4200d56a3`: additive writer/FK/receipt-retry pattern. SHA256 `f193bb36656915aa25429f6ab20078a00edc83adb9bf41a785a51a2c2281c2b8`.

Installed migration `connection.rs` and `migrator.rs`, plus SeaORM `database/transaction.rs`, were read before this assessment. Exact ranges/hashes are recorded. The already applied offline code-context staging rule and pinned dependency grounding remain; no new dependency-corpus coverage, installation or hook claim is made. Normal immutable `gh api`, Git source/preservation comparisons and owner-ledger verification all exited0. Owner rustfmt/diff checks are attributed only to the owner; no Rust command ran here.

Protected-source comparison confirms unchanged dependencies, original identity/task authority, files/receipts/session scope, HTTP/native registrations and both14 files. The earlier425 finding stays frozen at `2c37bb7c3103cc8a391521d37a60a21b12ca5be8`; no original finding or evidence file is rewritten. Earlier9949 independent fourteen passes, 3841/bdfa source verdict, and owner425 publication8 remain separate historical evidence. Branch `review/business-integration`; only this report/ledger/digest directory and the primary report opening change. R4 awaits an allocated exact-head execution window; runtime/import/public transport acceptance remains outside this source verdict.
