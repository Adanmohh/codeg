# Independent Increment B product review — preparation checkpoint

**Product review is awaiting an immutable implementation checkpoint.** No B
source, migration, test or UI result has been accepted by this report. The
accepted contract has no open contract blockers; that verdict does not establish
correctness of the implementation now assigned to tickets and rebrand.

Reviewer branch **review/business-intake** starts from accepted main
**0bd50aedc7bdfea0bc392d4feb63b9101b5af92d**. The preceding contract review
`2a765db2f71c175bb64c8d3b7843fa8f0f299687` is imported on main. Root's newer
0bd50aed commit records this backend/reviewer dispatch. Tracked files were clean
before switching; paused `.build/`, `out-design-final/` and
`reports/visual-refresh-baseline.md` had no tracked collisions and were preserved.
No fixtures, exports, targets, credential stores or processes were changed.

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

## Concrete regression probes — planned, not run

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

## Docs-first, borrowing and command evidence

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

Only reports/review-business-intake.md belongs to this new branch's review
checkpoint. The final product verdict and specific findings will replace the
awaiting-source status after immutable source and meaningful evidence exist.
