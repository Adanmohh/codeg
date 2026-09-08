# Independent business tenancy review

Review target: `f3b408dae5c724f354763961d79a17a7ae5c86f8`, PR30, tree
`d3f4cc354ceb7034c0b685b76289c1d3de4513af`, parent contract
`7516461633c163c2ac683930487e33231e630b0b`. Source base is
`f3813e3f1edb521f1d1b20d0b372643acc4123a5`; own report branch
`review/business-tenancy` starts at accepted main `0084fd3ef41983eca6cf52fbdf2dc2b8e9936cd2`.
B is preserved/pushed at `15bb402b`, including four passing recovery tests at
`6513dc15`; its production remains `e9c63237`. Paused visual report is preserved.

## Bounded verdict and scope

**No additional blocking findings in the reviewed f3b408da migration/identity/
settings core.** Sixteen unchanged tests and four reviewer probes passed
independently. This completes the bounded first-checkpoint review, not complete
tenancy acceptance: the unlinked-run epoch correction and the other unfinished
boundaries below remain explicit. Read the full implementation report, contract, product diff,
identity/settings/migration source and relevant unchanged transaction/auth code.
Scope is the pinned-connection parent rebuild, foreign-key restoration and retry,
captured/legacy delegation epochs, original organization mapping, and settings
authorization/CAS/audit. No product source, other worktree or existing fixture was
edited. All runtime databases and credentials are synthetic.

## Independently executed evidence

An own ignored `git archive` of the frozen head is under
`.docs/business-tenancy-review/f3b408da/source`. Every **731/731** tracked blob in
`src-tauri`, `integrations/pi-desk`, NOTICE and LICENSE matches the Git object;
`source-blobs-before.json` records each comparison. No uncommitted owner source
was copied. Only the new isolated `.docs/business-tenancy-review-target` is used.

From that archive's `src-tauri` directory:

```sh
env CARGO_TARGET_DIR=/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-tenancy-review-target cargo test --locked --offline --no-default-features --lib business_identity::tests > /Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-tenancy-review/f3b408da/evidence/identity-unchanged.log 2>&1
```

**Exit0: 16 passed, 0 failed, 1 manual fixture ignored; 0.99s execution,
2m23s compile.** This includes the three new owner tenancy tests and thirteen
existing identity/auth tests. The real protected router remains covered; the
ignored4341 browser fixture was not started. Linker unwind-size and existing
proc-macro-error2 future-compatibility notices remain. This is not Clippy,
desktop/native, browser, provider or full B acceptance.

Four reviewer-only probes also passed, **exit0, 4 passed/0 failed/0 ignored,
0.34s execution/1m01s compile**, using the same command/target with selector
`business_identity::tests::independent_review` and log `independent-probes.log`.

| Probe | Independently observed boundary |
| --- | --- |
| `review_tenancy_retained_task_lineage_and_actual_migration_receipt_retry` | Actual SeaORM migration receipt INSERT fails after the schema COMMIT. Every column in eight populated member/credential/history/task/authority/execution/deliverable tables is retained; all three separately checked-out connections enforce FKs. Retry records exactly one migration receipt, preserves a post-commit settings change, original mapping and immutable task/delegation history. |
| `review_tenancy_cancel_closes_pinned_connection_and_retry_preserves_data` | Hold an independent SQLite writer, observe the migration still pending with its single pool connection occupied, abort before releasing the writer, then confirm the replacement connection lacks the old TEMP marker and has FKs on. No partial tenancy marker; retained task lineage survives and retry succeeds. |
| `review_tenancy_settings_roles_closed_payload_and_audit_atomicity` | Viewer/member/manager writes and scoped-agent reads/writes are denied. A real admin credential remains non-operator and can save. Extra actor/org/URL/CSS fields, invalid names and nonpositive revisions fail. An injected audit INSERT failure rolls back settings/revision; valid save adds one event without renaming the organization. |
| `review_tenancy_settings_real_cas_and_queued_epoch_and_credential_fences` | Two SQLite connections race one settings revision: one save, one conflict, one event. A queued save loses authority after suspend/resume commits; explicit fresh authentication can save. A later queued credential revocation denies the stale save without settings/audit mutation. |

Committed [result and artifact hashes](business-tenancy-review/f3b408da/results.json),
[unchanged log](business-tenancy-review/f3b408da/identity-unchanged.log),
[probe log](business-tenancy-review/f3b408da/independent-probes.log),
[test-only patch](business-tenancy-review/f3b408da/independent-probes.patch),
[readable tests](business-tenancy-review/f3b408da/independent_cases.rs), and
[source correspondence](business-tenancy-review/f3b408da/source-correspondence.json).
The clean baseline matches 731/731 blobs. For the additional run, 730 remain exact;
the only changed original file is `business_identity/tests.rs`, with one test
module registration. The new test file exactly matches the report copy; no
production blob differs. `git apply --check --reverse` against the owned archive
exited0 for the committed patch. No owner source correction was needed.

Probe SHA256 `f406841d2eaa31bb834e47efee7bf67a4ac08526036b173343f29758f247f602`;
patch `6edbcbe5ddbee64e2ca3f59133ab2eb7f141508a52552a597a2f6685598f2817`;
unchanged log `52e677dfb9d8db8c64f01dae2bdc7efff555e83051a78cfe4a52935d4d82bd45`;
probe log `bd75a10e76d785dd9aa03bd9cfeb081cf6770271b9ed554594aa7afa1ea744c8`.
The exact tested reviewer file retains an unused `TransactionTrait` import;
that warning is disclosed rather than changing the tested artifact. Rustfmt and
source/report-only diff checks exited0. The complete staged artifact diff check
exits2 only for the two preserved logs' final blank lines and a blank context
line in the unified patch. Raw evidence hashes are preserved. No broad suite or
passing selector was repeated.

## Grounding and attribution

Full current FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS and implementation scope
read. Code-context used the existing rag-skills Python with `HF_HUB_OFFLINE=1`:
guide exit0; docs exit3 because `data/code/tickets.db` is absent. Cross-project
per-tenant-database advice does not override this accepted shared-database
contract. Installed SeaORM1.1.19, SQLx0.8.6 and SQLite3.46.0 source is the API
authority. SQLx pool `close_on_drop`, SQLite worker cancellation/acknowledgement
handling and SeaORM's SQLite migration/receipt ordering were read directly.
Live hook audit for this worktree/session `01a07c1c-d82f-7022-84db-778a438632f1`
contains PostToolUse21203–21206 and PreToolUse21211–21214. Hooks remain enabled.

Verified NOTICE's six Codeg source blobs against the immutable GitHub tree at
f3813e3f using gh api: identity `mod.rs`22a99cae, `store.rs`bb11e604,
`types.rs`145f1d0e, `tests/transactions.rs`e9cac228, migration000009 ad2db145,
and `src/lib/theme-presets.ts`9181e38c. Full original Apache-2.0 LICENSE read;
blob `261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64`. All earlier notices remain.
SQLite official `version-3.46.0` resolves through gh api to
`bebe2d8be8acfd02592c4972f4ba32c3b4e4a33f`; its public-domain LICENSE was read.
Its LICENSE blob `f68a6c175f0b72086c313a9dbd9f2ec8d87525dd` matches NOTICE.
Dependency primitives are API references, not copied implementations. No
Edublend, GPL or AGPL implementation is introduced by this diff or review.
Reviewer fixture glue additionally follows the same pinned Codeg transaction
tests and `db/test_helpers.rs`; attribution is included in the test file. The
production NOTICE/LICENSE and dependency manifests/locks are unchanged by review.

## Explicit pending boundaries

- Actual platform provisioning and same-database two-tenant transport tests are
  pending the owner's compiling implementation. This head exposes no new
  settings/platform route and does not implement restricted native sessions.
- B's persisted setup/attempt/source/preview epoch fields and000011→000012
  integrated retention remain B-owner work. No stale Principal refresh/default
  epoch is permitted. B is not included in the frozen A-only migration prefix.
- Owner has identified the entrusted-but-unlinked A run epoch seam and is
  preparing a separate immutable correction; it is not claimed fixed here.
- Root's newly supplied native prerequisite requires cross-window large/raw
  channel denial, ordered positive host delivery and no fallback queue on send
  error. A custom app-command ACL or channel interceptor alone is not native
  isolation acceptance; platform-specific responder conditions must be retained.
  Caller-window controls must exclude arbitrary target labels. These are future
  native gates, not an executed exploit or a defect claim against accepted A.
- Cancellation was measured while waiting behind an independent writer, not at
  every SQLite/driver await point. Receipt failure was deliberately injected
  after the real schema commit; there was no forced OS process/power loss. The
  populated retention fixture is A-only and launches no actual execution run.
- No old targets, fixtures, exports, A bundle, real stores or credentials changed;
  no live provider/model/engine/Pi process, server launch or new dependency.

Early report checkpoint `8d935c88d061ca5179fb74ad7f9f2a337aae7a36` was pushed
and relayed before these probes completed. Both test processes have exited0.
The final reviewed implementation remains exactly f3b408da; later owner product
commits require their own source correlation and focused review.
