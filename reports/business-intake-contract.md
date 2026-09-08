# Business intake contract preparation

**Docs-only; Increment B is not implemented.** The closed contract is ready for
the final bounded consistency review. It incorporates accepted access seam
`18be55edc276713fc6d46d075baec363245ba285` by immutable reference and closes
Q1–Q4 without a second setup schema, credential store or identity model.
Product work still requires root's exact contract acceptance and dispatch.

Branch `docs/business-intake-contract`, accepted base
`c7f7366fef2d7945cea18d4da1b7594fa3026e1c`;
[draft PR25](https://github.com/Adanmohh/codeg/pull/25).
Initial pushed checkpoint `e5139af8`, complete DTO/cases checkpoint
`79a922945668a633ea6b5f7e68f9bdc08a0725a3`.
Deliverable: [business-intake.md](../docs/contracts/business-intake.md).

## Contract decisions and required seams

- B1 uses fixed read-only Fireflies queries, explicit human source grants and
  durable resumable import steps in the existing Rust/SQLite service. Authenticated
  bounded `advance` calls avoid a daemon or reconstructed stored Principal. A
  disconnected import waits for an authorized human; no continuous-sync claim.
- Imported material and candidate passages stay private to their source audience.
  Task-domain access, matching names/participants and task assignments grant no
  source authority. Accept publishes only the exact human-reviewed title/notes
  to an explicitly permitted domain; its private source link remains gated.
- Immutable source versions, attempt UUID/lease fencing, writer-first identity
  revalidation, revision CAS and operation receipts prevent duplicate work or late
  revoked commits. Changed imports preserve edited drafts and terminal decisions.
  Read failures never reuse email send/unknown receipts or mint Hafidh freshness.
- Actual task create owns a transaction today. The task owner must extract its
  validator/create path into a crate-only transaction helper while preserving the
  public wrapper. Intake must not duplicate auth/SQL or call create then link in
  separate transactions. Existing-task links need the task-owned edit/CAS/activity
  seam and review invalidation, without engineering execution linkage.
- The accepted protected access seam supplies all eight setup/grant operations:
  disabled creation with zero grants, explicit historical/current/future binding
  audience, pinned ownerAuthorityRevision, same-provider staged credential
  rotation, and new binding on resource/domain/owner changes. Tickets owns the
  single backend/migration, task/credential/host helpers and registration at later
  dispatch; rebrand owns UI and approvals independent review. Missing setup
  alone is not the planned B delivery.

The contract defines closed HTTP/native operations, null/default behavior,
date-only deadlines, readiness/error distinctions, source-safe DTOs and **18
planned synthetic acceptance cases**. No case has been executed in this docs task.

Read rebrand's complete interaction plan at
`99cecbac3e8eab1da6f977ec649d8c874e668ab9`, its later reconciliation
`57b07ca81257421b9cbb414f7ac456c1bb39ea06`, and root's complete BI-1–BI-10 plan
at `650be3025c25386649bc906f6bed335abafcb011`. Closed the proposed passage-only
rebase/null-draft disclosure, durable import/terminal-decision rediscovery and
12-second read/15-second backend bound beneath the existing 20-second client.
Text-free link uses the existing task Detail and exact target ID/revision/domain
CAS; no durable target-preview entity. Final binding setup/readiness uses the
access contract's BindingList/View/Summary only, with no duplicate readiness
endpoint. Candidate/import capabilities, publication domains and nullable source
revision are explicit. Source-refresh fences apply across distinct imports.

Read the complete access contract and reviewer report via local git show at
`18be55edc276713fc6d46d075baec363245ba285`; no uncommitted doc is the final
authority. Also read root's complete prepared B ownership in
docs/BUSINESS-IMPLEMENTATION.md at `eecc7142a1132b70401a708806279dadee902755`.
All of that work remains docs-only; the prepared ownership is not dispatch.

One concrete prerequisite came from this worker's full keyring_store source
read: its mutation read maps errors to an empty map. The existing adapter needs
a strict writer-read under its current lock before B staged set/delete; malformed
or unreadable storage must preserve original bytes and unrelated credentials.
Approvals independently confirmed and included it in the accepted seam. This
is a future synthetic failure-preservation gate, not an executed exploit or
claim that SQLite and keyring/file updates become atomic.

## Verified sources

All remote research used `gh api` with immutable refs. Local accepted code,
installed types and CLI help were read first. The three prior research reports
and root synthesis provide context; live provider behavior is not inferred from
them. No new survey, runtime adoption or third-party source port was performed.

**Existing Codeg core**, pin
[`c7f7366fef2d7945cea18d4da1b7594fa3026e1c`](https://github.com/Adanmohh/codeg/tree/c7f7366fef2d7945cea18d4da1b7594fa3026e1c).
Exact inspected files and intended reuse:

| Files relative to `src-tauri/src/` unless stated | Verified boundary / intended glue |
| --- | --- |
| `business_identity/mod.rs`, `business_identity/types.rs` | Private Principal, current credential/member/domain checks; `begin_write`, `authorize`, `active_reference`. Agent delegation restoration is not general import-job authority. |
| `business_tasks/store.rs` (especially 354–420), `business_tasks/types.rs`, `business_tasks/policy.rs`, `business_tasks/http.rs`, `business_tasks/mod.rs` | Existing create/default/date/reference/visibility/HTTP contract; create currently owns commit. Task-owner helper extraction is future work, not an existing API. |
| `app_error.rs`, `web/handlers/error.rs`, `business_identity/http.rs`; `src/lib/business/client.ts` (repo root) | Source-read exact error/status mappings, identity-specific401 and current 20-second timer. Proposed intake-only safe reasons do not change global auth/task errors; native core must supply its own deadline. |
| `ops/mod.rs`, `ops/store.rs`, `ops/agent.rs`, `db/service/ticket_service/mod.rs` | Operator account and public message filtering. Scope account/inbox is routing, not member authentication. Never serialize full operator thread or impersonate agent RunContext. |
| `ops_intake_host/store.rs`, `ops_intake_host/operator.rs`, `ops_intake_host/types.rs`, `ops_intake_host/agent.rs`, `ops_intake_host/fix_task.rs` | Exact account/product/folder mapping, host-only refresh, cached public projection and existing issue-receipt engineering link. Member-safe projection helpers do not yet exist. |
| `ops/delivery.rs`, `ops_telegram/mod.rs` (claim and post-await paths) | Claim UUID/config-revision checks and immutable receipt discipline. Read jobs need separate records; preserve no-resend and approval behavior. |
| `keyring_store.rs` (complete file) | Native OS keyring/server private token file, process-local writer lock and exact unsafe error-to-empty mutation-read behavior. Reuse with the narrow strict writer-read prerequisite; no separate store. |
| `integrations/hafidh-intake/src/hafidh_intake/client.py`, `integrations/hafidh-intake/src/hafidh_intake/schemas.py`, `integrations/hafidh-intake/README.md` (repo root paths) | Bounded GET-only client, normalized source digest excluding fetched time, strict ULID/private-field projection and process-local cursors. No durable-cursor or upstream fine-grained-token claim. |
| `LICENSE`, `NOTICE` (repo root) | Preserve existing Apache/owner-approved/MIT attributions and exclusions. No edits in this docs task. |

Key exact blobs: identity mod `22a99cae327682b04fbcced34cf5ce19f6120dad`;
task store `d48fdf1e85feb4410dbff51e8a671d6c9d525691`;
ticket service `17b3f26b54281a87b0d0f23f823437e1d6dc7e96`;
host store `bb016b603412ebb22a0f5ef44252b7397b9eb0e0`;
keyring store `29fc3fb38280338aa26939c45f80ef9aefc2a394`;
Hafidh client `d2a9b205c30c51ced13ff7733914466a3935b4ad`;
root NOTICE `9731e731cea397b607ba76aa60021a0422a1f337`.

**Official Fireflies adapter**, pin
[`firefliesai/n8n-nodes-fireflies@fbd24607bc784a2294ce402426aefe2cb8c00f50`](https://github.com/firefliesai/n8n-nodes-fireflies/tree/fbd24607bc784a2294ce402426aefe2cb8c00f50),
package 2.2.2, commit 2026-07-16. Independently read full query file, both read
operations, transport, error helper, credentials, package and licence:

| Exact path | Blob / verified fact |
| --- | --- |
| `nodes/Fireflies/helpers/queries.ts` | `c775b8bbe6c0bbb16f8f7e8a60467c248ad93859`; fixed transcripts/transcript/user reads; limit/skip/date/mine variables and sentence/summary/access fields. Mutation queries are excluded. |
| `nodes/Fireflies/operations/transcript/getTranscriptsList.ts` | `b9479e5f618de87ff820ab13dbbbf839f4150f23`; default 50/skip 0, variables forwarded. No snapshot/order/updated-since guarantee established. |
| `nodes/Fireflies/operations/transcript/getTranscript.ts` | `99d9e86bf7a27d3ffbd17abbae7f6186b9d74e19`; fixed detail ID query, not a production schema validator. |
| `nodes/Fireflies/transport/index.ts` | `0b60c1c9e9d61f4583298cd2c0c464e9864861f5`; fixed GraphQL POST endpoint, HTTP200 errors checked before returning data. |
| `nodes/Fireflies/helpers/errors.ts` | `7141e7c99969c6fd32f2e96fa7b2a561037a43de`; recognizes upstream errors. Raw error/correlation disclosure is deliberately not reused. |
| `credentials/FirefliesApi.credentials.ts` | `a39bced68aeceb23b33641e774e3509554caac49`; Bearer credential, user.user_id check. No actual key/account checked here. |
| `package.json` | `fe1e413862f5d5dddf9c0ce8153bfd4d0a27d56c`; official Fireflies package identity/version and MIT declaration. |
| `LICENSE.md` | `1e4b3a6e245384b89f24f2aef5e3f8e7fa1f4d23`; complete MIT licence, **Copyright 2022 n8n**. |

**Schema cross-check only:**
[`firefliesai/fireflies-node-sdk@76d7983a1b3592a1662e77157f1ff1d85216acb4/src/types.ts`](https://github.com/firefliesai/fireflies-node-sdk/blob/76d7983a1b3592a1662e77157f1ff1d85216acb4/src/types.ts),
blob `0c8c835beb093ba56817cf9ed14b3540cc040fb6`, declares action_items:string[],
summary_status:string and mine as organizer-owned meetings. The adapter query
does not establish return types; no completion enum or normalized assignee/date
is assumed. Tree inspection found no SDK LICENSE/COPYING path; **no SDK code
port**. Official public-api-ff docs repository returned 404; no current hosted
schema or authenticated provider request was substituted.

**Approved IntroMail boundary**, unchanged founding pin
[`IntroInnovation/intromail@0bd24dfe284b888aa9f602fa1fd00e337ea38874`](https://github.com/IntroInnovation/intromail/tree/0bd24dfe284b888aa9f602fa1fd00e337ea38874):

| Exact path | Blob / use or exclusion |
| --- | --- |
| `backend/app/services/agent/tasks_pack.py` | `af500fbd419ece67787c84ad4e805424c48fec12`; human/agent intersection and common service boundary only. Preview raw-list lookup before authorization is excluded. |
| `backend/app/services/task_ops.py` | `3b1014af27a505c96633c767a9a82de6dc2d4b13`; common operations pattern. List existence without destination authorization is excluded. |
| `backend/app/services/agent/actions.py` | `fc1461ece64d28774638d714ebcc9981358d3fc6`; inspected CreateTaskFromThread fallback/direct ORM/global source-ID behavior, **not reused**. |
| `docs/BORROW-PLANE-OPENPROJECT.md` | `637881c17aaef59d98bf67e06b5304e915a3902a`; source-key/favorites/relations/journals provenance checked and excluded. |

**Intended attribution if implemented:** retain Codeg Apache 2.0 upstream
v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed` and the existing owner-approved
IntroMail NOTICE (no inferred blanket third-party licence). Preserve inherited
Chatwoot MIT v4.17.1 `b354a9550e1fb59fa537a9c384232cb076213e72` and Hafidh
owner pin `a83794708193a18611c8e6eb8e88637b8136e471` on their existing seams.
Any Fireflies query/error-pattern port must add exact path/pin/change mapping and
reproduce the full MIT permission/disclaimer plus `Copyright 2022 n8n` from the
licence above in NOTICE/licence materials. No n8n runtime, SDK code, Plane,
OpenProject, AGPL/GPL source or uncertain task-source lineage is ported.

## Commands, coverage and limits

Read complete FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS and implementation
scope, accepted identity/task contracts, all three business/meeting research
reports and root synthesis. Initial large output was truncated; complete required
files were reread in bounded chunks. Actual source bodies, not search hits alone,
ground the proposed seams.

| Command / action | Observed result |
| --- | --- |
| Existing venv `code_context.py guide ... --project codeg`, `HF_HUB_OFFLINE=1` | Exit0; mostly other-project rules. Applied installed-version/source-first rule; no Codeg-specific rule or delegation requirement invented. |
| Same tool `docs ... --repo <tickets>` | Exit3: tickets.db absent. No ingestion/install or fabricated corpus coverage. |
| Local Cargo/lock and installed SeaORM transaction source | Read SeaORM 1.1.19 writer/commit/drop/ConnectionTrait; pinned reqwest 0.12.28, serde 1.0.228, serde_json 1.0.149, sha2 0.10.9, tokio 1.49.0; existing keyring 3.6.3/SQLx SQLite 0.8.6 verified in lock. No dependency/lock change. |
| Installed `gh api --help`, PR-create help, Git help; fetch/switch/add/commit/push/PR25 create | Successful authorized docs branch/checkpoint operations; switch usage help exits 129 by convention. |
| `gh api` commit/tree/content reads at the immutable refs above | Successful inspected content; public-api-ff lookup 404/exit1 explicitly retained as a coverage gap. No provider endpoints called. |
| Local Fireflies/n8n dependency search | Exit1/no installed package found; not adopted. Two initial guessed file lookups failed (Hafidh models.py / split ticket entities); actual schemas.py and nested ops_ticket.rs located, relevant source reread. No API claim rests on those guesses. |
| `git diff --check` at checkpoint 79a92294 | Exit0. No runtime/test/build gate was run for B docs. |

Documentation editing checks caught two invalid patch shapes before applying;
corrected patches were applied normally. A guessed singular error.rs lookup
exited2; discovered and read actual app_error.rs plus its HTTP and identity
mappings before specifying the error contract. No hook was bypassed.

Live own-session docs-first PreToolUse/PostToolUse records were inspected without
payloads: session `01a07c1c-d82f-7022-84db-778a438632f1`, tickets cwd, exit0
(including 1788889191/1788889192). Hooks remain enabled. Astra is the selected
model; no extra agents, installs or provider/model/configuration actions.

Provider schema/access/expiry/offset behavior remains untested. Proposed limits,
grant semantics, durable import schema, helper extraction and the 18 acceptance
cases are contracts, not passing implementation evidence. No freshness,
authorization, lossless sync or OS sandbox guarantee is inferred from an API key.

Paused untracked `reports/visual-correspondence.md` remains untouched, SHA256
`a2120d9cc87dbb4823b1223e02d377660fff370d846f2fd102fe1eb36599a50d`.
Only the contract and this report belong to PR25; root planning docs, NOTICE,
product, locks, targets, outputs and existing fixtures are unchanged. During this
preparation root separately requested the final N2 packaged-companion recheck:
completed report/result-only handoff `e784f8fca6d9410d7112c6b7c158a1a311e6227e`
on the owned review branch, root accepted. That 275-assertion synthetic run is not
B evidence and is not mixed into this PR; the original result is preserved.
