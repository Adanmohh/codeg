# Independent Increment B product review

**One open P2: strict credential reads lose legacy file-permission hardening.**
Five unchanged focused tests pass at prerequisite checkpoint
`60daf42e79fa7dc10f8118b9cdb8a73b2c07e80d`; two additional reviewer tests reproduce
R1 (exit101). The separate schema/DTO checkpoint
`9a4c8c882cec938665bc233b4d658d8de019ccfd` passes its two unchanged tests
independently and introduces no further blocking finding in this review.
The incomplete intake backend and UI are **not accepted** by these limited
results; authorization, source ordering and atomic publication consumers remain
pending. R1 is unchanged at9a4c8c88 and awaits a committed correction.

Reviewer branch **review/business-intake** starts from accepted main
**0bd50aedc7bdfea0bc392d4feb63b9101b5af92d**. The preceding contract review
`2a765db2f71c175bb64c8d3b7843fa8f0f299687` is imported on main. Root's newer
0bd50aed commit records this backend/reviewer dispatch. Tracked files were clean
before switching; paused `.build/`, `out-design-final/` and
`reports/visual-refresh-baseline.md` had no tracked collisions and were preserved.
The preparation commit **599aa1e6b74c0841ab2753dd58c806cae0c83dcd** was pushed and
imported by root. Existing fixtures, exports, credential stores, targets and
processes remain untouched. This review created only its own archives, target,
synthetic temporary test data and report evidence.

## Current exact-head review

Resolved the immutable owner commit through `gh api`, parent
**086eee485e6f40a8b02ea77c9dbeefe315c81596**. Read the complete six-file diff,
owner report, changed task/credential source and tests, relevant existing task
policy/HTTP tests, and NOTICE. The checkpoint changes only `keyring_store.rs`,
`business_tasks/{store,types,tests}.rs`, NOTICE and the owner report. It introduces
no dependency, migration, registry or intake endpoint.

The task helpers retain current `Principal` authorization and active reference
checks inside the caller's transaction. Preparation fixes the resolved editor's
owner ID; acceptance recreates authorization rather than trusting PreparedTask.
Create/get and finish keep their public wrappers. Source linking requires current
human edit authority, destination domain and revision, changes no task text,
invalidates review and records only an opaque link ID. The helper tests prove
outer rollback for task/activity creation and linking, and reject a different
member's attempt to adopt the editor-owned draft. The actual protected-router
test exercises the unchanged HTTP handlers with issued credentials, including a
positive create/note/get and spoof, viewer, domain and revision denials.

These results do not yet prove atomic intake claim/link/receipt publication:
the intake consumers and their source/grant revalidation do not exist at this
head. The Rust transaction parameter alone is not an authorization capability.

## R1 — P2: strict mutation reads skip existing credential-file hardening

**Open**, server/Unix. Exact location:
[`src-tauri/src/keyring_store.rs:103–116`](https://github.com/Adanmohh/codeg/blob/60daf42e79fa7dc10f8118b9cdb8a73b2c07e80d/src-tauri/src/keyring_store.rs#L103-L116),
called by `change_token_at` at line124. This is separate from the earlier task
review's R1; the identifier is local to this report.

The former mutation path called `read_tokens_at`, whose lines75–95 deliberately
tighten an existing legacy 0644 store to 0600 **before** consuming its bytes.
The new strict reader calls `read_to_string` directly. A failed set/delete on a
malformed legacy file returns safely without replacing bytes, but also leaves
any recoverable credential text world-readable. A valid strict read skips the
same hardening until a later successful atomic replacement; that replacement
may fail. The new failure-preservation tests check bytes, not the retained mode.

Reproduction: in a temporary directory, seed a 0644 JSON credential map with
synthetic values; compare the existing reader with the strict reader. Then seed
a malformed 0644 map containing an unrelated synthetic entry, invoke both set
and delete, and inspect bytes, directory contents and mode. No real store or
account is involved. Both reviewer tests fail as expected, exit101: valid strict
read mode **420 (0644)** versus required **384 (0600)**, and rejected set/delete
modes **[420,420]** versus **[384,384]**. The existing-reader positive control
reaches 0600; all byte-preservation and no-residue assertions pass before those
mode failures. The added probe is isolated from the immutable source archive.

**Required fix:** retain the existing best-effort Unix permission hardening on
the strict mutation read, under the existing lock, while preserving the new
fail-closed read/parse behavior. A shared narrow pre-read helper is sufficient.
Regression tests must cover valid strict reads and rejected set/delete: 0600,
unchanged malformed bytes and no replacement/temp residue. No credential-store
rewrite or cross-process/SQLite atomicity claim is needed.

## Independently executed evidence

Unchanged archive:
`.build/review-business-intake/60daf42e`; separate reviewer probe archive:
`.build/review-business-intake/60daf42e-probes`. Both are inside this worktree.
After the unchanged tests, all **728/728** tracked backend, Pi integration,
LICENSE and NOTICE blobs matched the frozen commit, zero mismatches. The full
manifest stays in the local evidence directory; the verification summary is
published with this report.

Every selector below uses cwd `<archive>/src-tauri` and the explicit command:

```sh
cargo test --locked --offline --no-default-features --lib SELECTOR \
  --target-dir /Users/mohamedadan/projects/_worktrees/ops-desk/approvals/.build/review-business-intake/target -j 4
```

| Selector / archive | Independent result |
| --- | --- |
| `intake_store_` / unchanged | Exit0; 2 passed, 0 failed, 0 ignored; 0.02s execution |
| `intake_task_` / unchanged | Exit0; 2 passed, 0 failed, 0 ignored; 0.16s execution |
| `real_business_router_derives_actor_rejects_spoofs_and_returns_revision_conflicts` / unchanged | Exit0; 1 passed, 0 failed, 0 ignored; 0.07s execution |
| `independent_intake_` / added tests only | Exit101; 0 passed, 2 failed, 0 ignored; 0.03s execution; both demonstrate R1 |

The HTTP result is the actual protected Axum router through axum-test17.3.0's
in-process mock HTTP transport, not a browser or listening-server E2E result.
Installed Router/IntoTransportLayer/TestServer sources were read to verify that
distinction. No port was allocated. No native keyring API or live provider ran.
Build logs disclose the linker compact-unwind-size warning and
proc-macro-error2 2.0.1 future compatibility warning; no waiver was added.

Owner-reported server check exit0 and `intake_` 21 passed/1 manual ignored remain
**owner evidence**, not independently repeated here. Desktop/server/companion
checks, Clippy and B integration gates belong to later compiling checkpoints;
this review does not claim them.

Committed logs, probe patch, source-verification summary and test result metadata:
[`review-business-intake-evidence/60daf42e`](review-business-intake-evidence/60daf42e/).
The original failure logs are preserved. The new owner checkpoint
**9a4c8c882cec938665bc233b4d658d8de019ccfd** is reviewed separately below. R1 was
relayed to tickets and root through authorized internal Herdr prompts, both
exit0. The finding/log publication is review commit
**473b3b316abdac67bacf09d4fef011ba3ec24ff4**, pushed successfully. Root independently
read the source/probe and agreed with P2; that source confirmation is attributed
to root, not counted as another reviewer test.

## Separate schema/DTO review at9a4c8c88

Resolved **9a4c8c882cec938665bc233b4d658d8de019ccfd** through `gh api`; its parent is
the frozen60daf42e checkpoint. Read the full nine-file diff, error/types/tests,
sole `m20260908_000011_business_intake.rs`, additive module/registry, NOTICE and
report. Keyring/task extraction and Cargo manifest/lockfile are byte-identical
to60daf42e (`git diff --exit-code`, exit0). PR28 remains draft.

Source observations: the migration opens an explicit transaction, uses composite
organization/member/source/task foreign keys, records source refresh fences and
nullable initial source revision, retains history and rejects rollback once a
binding or staged setup exists. DTO inputs reject unknown fields; the key wrapper
has neither Debug nor Serialize/Clone. Credential replacement distinguishes
omission from explicit null. DecisionTask has an explicit restricted variant,
without task ID/revision. Source/grant disclosure, owner-revision revalidation,
safe retries and public-task publication still require their not-yet-present
consumers; these types are not proof of runtime privacy enforcement.

Independent unchanged selector `business_intake::tests`, using the same
locked/offline/server-library command and own target above: **exit0, 2 passed,
0 failed, 0 ignored, 0.05s execution**. Both exact test bodies were read. The
archive `.build/review-business-intake/9a4c8c88` matches **733/733** source,
integration and licence blobs. Logs and verification summary:
[`review-business-intake-evidence/9a4c8c88`](review-business-intake-evidence/9a4c8c88/).
Four unused-consumer warnings, the linker warning and the dependency future
compatibility warning are preserved. No Clippy result or waiver is claimed.

**Evidence correction, not another schema blocker:**
`business_intake/tests.rs:36–72` inserts a random nonexistent owner UUID and
checks five table registrations. It does not seed a foreign-organization member
or any existing task row. The test name's “retains_task_rows” and owner report's
“cross-org member-FK rejection” overstate what runs. The observed result is
missing-member rejection with zero inserted bindings plus fresh table existence.
Describe that accurately; populated upgrade/rollback/atomic-failure coverage
and real positive/negative association cases remain future gates. No relaxed
organization constraints or fabricated cross-org fixture was used here.

NOTICE remains additive. Verified the referenced accepted base086eee48 task
migration blob **3cb107d5bc0ab375e1b82fc64c7fefc82396b318**, identity types
**145f1d0e460967380960323bdfac875a076f46b2**, identity module
**22a99cae327682b04fbcced34cf5ce19f6120dad**, and earlier task types mapping.
This transcribes accepted application contracts and reuses the existing Apache
schema/error patterns; no new external source or dependency. Read installed
serde/serde_derive **1.0.228** missing-field and deny-unknown-field generation;
serde_json remains **1.0.149**. No remote documentation survey was needed.

Publication hygiene: the first staged `git diff --check` exited2 only for the
three raw successful Cargo logs' final blank lines. Those bytes are deliberately
retained as command evidence; the pre-stage tracked report diff check exited0.
No product formatting/test failure was suppressed.

## Source and test-probe provenance

The additive NOTICE block preserves existing entries and cites the accepted
base086eee48 files. Independently verified base blobs:
`keyring_store.rs` **29fc3fb38280338aa26939c45f80ef9aefc2a394**,
task store **d48fdf1e85feb4410dbff51e8a671d6c9d525691**,
types **6d6c4b8c9555d09ea19d57c9fa329eb548832cc3**, tests
**4c7e6000a5881f979dd2bd1b0d13c88d9e1bf49a**. The changed hunks extract existing
approved task logic and the Apache Codeg store; no new third-party adapter port.

Reviewer permission tests adapt the existing Apache Codeg
`test_read_tokens_tightens_existing_file`, frozen60daf42e keyring source blob
**efbaba1cb99423dccf6a6fc04d3fdcaee05d93b9**. The published evidence patch includes
only additional synthetic tests, with attribution in its evidence NOTICE. The
product archive and reviewer branch's product/NOTICE files are unchanged.

Additional live hook metadata before the isolated probe edit: own session below,
PreToolUse **1788893536**, line19284; PostToolUse **1788893505**, line19283;
both exit0 and the approvals cwd. No hook was disabled or bypassed.

## Authority and frozen baseline

- Intake contract **670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5**, blob
  `9b98ce117c01370c93a599732facbef6ca423142`, and protected access seam
  **18be55edc276713fc6d46d075baec363245ba285**, blob
  `42f5a13b9fb6449d599264d34de12bd37227378e`. Both files in this branch were
  compared to those complete previously read versions: byte-identical, exit0.
- Read complete current FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS,
  docs/BUSINESS-IMPLEMENTATION.md and reports/business-intake-acceptance-checklist.md.
  Root's explicit implementation dispatch supersedes historical pending wording
  in the planning documents; this worker remains an independent reviewer.
- Tickets owns business_intake, sole migration **000011**, access/grants,
  staged credentials and the existing-store strict writer, task transaction
  helpers, fixed Fireflies reader, narrow legacy projections and registrations.
  Rebrand owns the later UI. Findings go to their owner and root; no reviewer
  product edits or additional workers.
- New source/grant rules do not widen legacy operator routes, create a Principal
  from stored IDs, launch an agent or authorize external actions. Scope of each
  result below will identify its exact source and whether it was read or executed.

## Remaining B probes and original review plan

The preparation table below remains the full plan. P01 and the task-owned part
of P12 now have the scoped evidence above, including open R1. Other rows are
planned; no full B or UI pass is implied.

Use the implementation owner's real protected router/core and committed test
seams, after reading them. These cases are outcomes to test, not names of APIs
or test helpers claimed to exist. Prefer barriers and separate SQLite writers
for races; a passing sequential helper test does not establish concurrency.

| Probe / BI coverage | Reproducible trigger | Required observable result |
| --- | --- | --- |
| P01 strict store, BI-1 | Synthetic token map with two unrelated entries; corrupt JSON, deterministic read failure, then staged set and delete. Include a genuinely absent-file control. | Existing bytes/entries survive each read/parse failure; no rename/deletion/replacement map. Only true absence initializes empty. Do not use chmod-only failure tests that succeed under elevated permissions. |
| P02 staged activation, BI-1/7 | Fail secret write, provider identity check or DB activation; cancel after staged write; force late store completion and uncertain commit response. | Old active reference remains usable when activation did not commit. Only a proven unreferenced stage is cleaned up. Receipt/current reference resolves uncertain outcomes; expired work cannot activate. No cross-store rollback claim. |
| P03 actual setup authority, BI-1/4 | Use real transport-resolved operator, then an owner-role member credential, viewer, agent, revoked credential and forged actor/org/reference JSON. | Operator-only setup works; restricted credentials cannot configure or read secrets. Closed input rejects spoofing. Member credentials remain denied on legacy config/terminal/engine routes. Native derives its original operator boundary. |
| P04 explicit audience, BI-1/4 | Create disabled binding with a named owner but no grants; enable without granting; later grant another human the explicitly confirmed historical/current/future scope. | Owner label and task/domain assignment confer no source access. Zero initial grants. Per-binding current grants filter lists, counts, details and history; no per-meeting ACL claim. Viewer can only read when separately eligible. |
| P05 owner lifecycle, BI-1/7 | Hold a provider read; change owner's member revision, remove/restore Contribute, revoke membership or rename. Attempt later activation/use. | Pinned ownerAuthorityRevision fails after any drift, including away/back. Actual operator update revalidates the same immutable owner and bumps epoch; fresh access is required. Changed owner needs a new binding. |
| P06 requester/grant fences, BI-3/7 | Revoke the advancing human's original credential, expire/revoke its grant, disable binding or change publication rights during an await or before accept. | Final writer revalidation rejects stale authority. Regrant/re-enable does not resurrect an old preview/claim. Another currently authorized human resumes as themselves; requester IDs remain history. |
| P07 source-wide ordering, BI-2/3/7 | Two different imports refresh one source; return the older response after the newer one, with both per-import leases otherwise valid. Include first-detail/null revision. | Source ID/attempt fence exists before I/O. Only the current source fence plus import attempt/lease/content revision may commit; old response cannot overwrite or restore freshness. |
| P08 claims and recovery, BI-3 | Race two advances, replay operationId while claim is live, cancel/reclaim/expire, drop connection and restart with a new authenticated human. | One accepted step commit; no duplicate live provider request for the same claim. Expired/cancelled/retired attempt cannot commit even without a replacement. Current-grant imports/list/get rediscovers durable work. |
| P09 bounded provider parser, BI-2 | Synthetic HTTP200 errors/partial data/null list, wrong ID/types, oversize/truncated body, duplicate sentence indices, timeout and unknown summary shape. | Fixed safe failure, no scan advance or fresh disclosure. Missing/empty/unsupported summary stay distinct; free text does not assign people/dates. Read12s/core15s remain below client20s in HTTP/native. |
| P10 versions and private drafts, BI-5/7 | Valid source A→B→A, grant-only epoch change, refresh failure; rebase passage selection with null draft and with human-edited draft. | Monotonic versions; no old-hash resurrection. Draft retained privately, stale text withheld, explicit current selection/CAS required. No automatic candidate/task rewrite. Retained history requires current fresh access. |
| P11 atomic accept, BI-5/6 | Race accept with edit/refresh/revoke; fail after task insertion but before link/receipt; replay identical operation and then changed body/actor/disposition. | One exact normal task, initial activity, immutable decision/link and receipt in one writer transaction. Rejected transaction has no orphan task/link/decision/activity. Lost result reconciles rather than issuing a fresh create. |
| P12 link and task review, BI-6/8 | Link without PreparedTask to a current editable task, then race target revision/review/archive; target domain differs from confirmed domain. | Exact live target domain/revision/edit checks, one typed opaque-link activity, review invalidated, existing task text unchanged. No engine/external-issue authority. Discard creates no task. |
| P13 DTO privacy, BI-4/7 | Source-granted human lacks prepared draft's destination Read, or later loses task access; compare source and task-only sessions. | PreparedTask withheld with hasPreparedDraft retained; terminal Decision redacts inaccessible task ID/revision. No transcript/private note/attendee/proof/config/secret in unauthorized payloads, public activity/search or agent context. |
| P14 immutable associations, BI-1/8 | Rotate key with same provider user, attempt changed user, change inbox/product/config away and back; attempt old-binding legacy capture. | Same-user staged rotation only; reassociation creates a new binding with fresh grants and preserves old rows. Monotonic trusted host identity fences away/back; no first-account fallback. |
| P15 migration and retained state, BI-3/8/10 | Apply migration000011 over isolated accepted A data; exercise its transaction failure/targeted rollback/reapply with later unrelated migration present if supported. | Existing rows/registries/NOTICE preserved, one owned migration, constraints/CAS effective. Test the named migration, not whichever migration is last. No use of an existing fixture DB. |
| P16 actual UI/session, BI-4/5/6/7/9 | Later real protected fixture: two synthetic sessions; setup→import/resume→passages→draft→accept/link/discard, refresh/revocation/conflict; affected EN/AR narrow/wide, light/dark and keyboard flows. | Complete exact human review and destination audience; private edits survive allowed layout/locale changes and clear on authority/session loss. No legacy requests, invented connectivity, provider-session401 confusion or frontend-intercepted JSON claimed as E2E. |

Provider request counters, source/candidate/import revisions and task/link/decision/
activity rows will be inspected in synthetic data where needed. A positive control
must accompany denial/race probes. Source strings that resemble instructions stay
plain content; no paid inference, live account, email, issue or Telegram operation.

## Execution and evidence discipline

For each compiling owner checkpoint: resolve its exact commit through gh api,
read the full diff/report and relevant source/tests/NOTICE, then use an independent
archive and reviewer-owned Cargo target. Record archive/source correspondence,
commands, exit codes, counts and limits. Initial unchanged focused tests establish
what that exact head covers; additional probes, if needed, remain test-only in
the isolated review archive and are labelled separately from the unchanged suite.
Do not imply an owner's passing count was independently executed.

Inspect file-backed writer races and actual transport authentication before
accepting substitutes such as a constructed test Principal or a single in-memory
connection. SeaORM1.1.19 queues rollback on Drop; SQLx SQLite0.8.6 delegates to its
worker. Cancellation assertions must wait for an observable database boundary,
and distinguish cancelled precommit work from an uncertain completed commit.

No new port or browser is allocated at this checkpoint. Coordinate a free
loopback-only synthetic fixture before use; preserve A's package and every older
fixture/output. Use only Playwright CLI later. The final integrated Design Studio
and native/artifact gates belong to root; do not repeat broad unchanged A suites.

## Preparation grounding and command history

Read separate installed React package and Cargo manifest first. Applied
code-context with the existing rag-skills venv and HF_HUB_OFFLINE=1: guide exit0,
with **Atomic per-task staging** relevant. Other-project authorization/TDD/delegation
rules were not treated as new project policy. Dependency docs query exited3:
approvals.db is absent. No install or corpus-coverage claim.

Read installed SeaORM **1.1.19** transaction begin/commit/rollback/Drop and SQLx
SQLite **0.8.6** TransactionManager source before specifying cancellation probes.
React remains **19.2.4**. Actual implementation APIs will be read at each head;
no new provider survey or dependency upgrade while awaiting source.

Live hook metadata before this report write: own session
`01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`, approvals cwd; PreToolUse
**1788891968**, audit line18836, and PostToolUse **1788891903**, line18823,
both exit0 in `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`.
Only metadata was printed; hooks remain enabled. Astra/max remains selected.

Exact approved borrowing remains Codeg Apache v0.30.4
`6f6bd648b206412644842a98d9ffeebf57292bed`, unchanged authorized IntroMail
`0bd24dfe284b888aa9f602fa1fd00e337ea38874` where provenance is established,
and Fireflies adapter `fbd24607bc784a2294ce402426aefe2cb8c00f50` with its MIT
Copyright2022 n8n notice. The accepted source ledger retains exact files/blobs.
Review actual ports/NOTICE at the product head; no SDK/n8n runtime, AGPL/GPL,
enterprise or uncertain copyleft-derived task hunk is authorized. This report
copies no product source and changes no NOTICE entry.

| Command/action | Observed result |
| --- | --- |
| git status/fetch/collision check/switch | Exit0; new branch from 0bd50aed, paused files preserved |
| Full governing reads and exact contract comparisons | Exit0; both contract files byte-identical to accepted pins |
| Offline code-context guide/docs | Exit0/exit3 respectively, limits above |
| Installed transaction source and hook metadata reads | Exit0; live pre/post confirmed |
| Published owner source lookup | origin/feat/business-intake was not yet a valid object, exit128; no implementation head or test was inferred |
| Product tests/builds/browser/provider/configuration actions | None run at this preparation checkpoint |

The historical table describes preparation only. Current exact-head findings,
tests and limits appear first; this branch contains review reports/evidence,
with no product changes. Further B review waits for committed compiling source.
