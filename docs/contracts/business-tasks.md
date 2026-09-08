# Business tasks contract

Owner: tickets, `feat/business-tasks`, draft [PR22](https://github.com/Adanmohh/codeg/pull/22). Base `4ec04d7282a50529335d724438d42b99a53385a2`; initial checkpoint `cb2e184f`; implemented runtime source `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5`, integrated with accepted identity/main. Validation and limitations are recorded in [business-tasks report](../../reports/business-tasks.md). Identity follows approvals' [business-identity.md](business-identity.md): private Principal, UUID string IDs, current credentials/grants and one writer transaction. No alternate auth system.

## Record, dates and defaults

A business task is independent of engineering `work_task`: no folder, chat, agent, connection or run is required. Owner (accountable human), assignee (human/agent executor), immutable creator and optional designated human reviewer are distinct.

- IDs on SQLite and JSON are UUID strings. Domain is identity's `marketing | channels | ads | website | feedback | engineering`. Status is `todo | in_progress | review | done | cancelled`; priority is `low | normal | high | urgent`.
- `dueDate` is **null or a strict valid calendar date YYYY-MM-DD, years 0001–9999**. It is not an instant, midnight UTC or reminder. Return the exact date; UI uses a date input and calendar components without timezone conversion. This supersedes the initial dueAt proposal. Actual creation/update/activity timestamps are RFC3339 UTC strings.
- Create requires title/domain. Notes default empty, priority normal, dueDate null, ownerId the authenticated creator, assigneeId null, reviewerId null. Initial status todo, revision 1, archive/current deliverable/execution null. Title is 1–240 nonblank characters; notes/comments/deliverables are plain text up to 20,000 characters; newlines/tabs allowed, other controls rejected. Submitted deliverables and notes must be nonblank.
- Null reviewer explicitly means **any currently authorized human reviewer in that domain** (owner/admin/manager), not a fabricated member or the creating ordinary member. A designated reviewer must be active, human and Review-authorized. Humans can submit a task for review without an agent or deliverable; review acceptance completes it.
- Owner must be active human with Contribute in the destination domain. Assignee, when present, must be active human/agent with Contribute. Every reference must be same-org and permitted in the destination domain; assignment itself grants no access.

## Fixed HTTP/Tauri contract

All operations below are POST under `/api/business/tasks/`, JSON **`{"input": {...}}`**, inside identity's bearer boundary (`Extension<Principal>`). Tauri commands are `business_tasks_` plus operation name with underscores. No ID path parameters. Unknown envelope/input fields and supplied actor/org/role claims are rejected. Results are direct JSON DTOs; errors are identity's AppCommandError contract, including stale already_exists/409 with business.revisionConflict. Responses are no-store.

`revisionInput` means `{taskId: UUID string, expectedRevision: positive integer}`. Nullable fields accept missing or explicit null with the behavior stated; non-null fields are required unless a default is shown.

| Operation | Input | Result |
|---|---|---|
| list | `{view?: "shared" (default) / "mine", domain?: Domain, status?: Status, query?: string, page?: 0, archived?: false}` | `{tasks: Task[], page, hasMore, canCreate}`; 50 rows/page, bounded literal text search |
| get | `{taskId}` | Detail |
| create | `{title, domain, notes?: "", priority?: "normal", dueDate?: null, ownerId?: self, assigneeId?: null, reviewerId?: null}` | Detail |
| update | `{...revisionInput, title, notes, domain, priority, dueDate?: null}` | Detail; full metadata replacement, missing/null date clears it |
| assign | `{...revisionInput, ownerId, assigneeId?: null, reviewerId?: null}` | Detail; full assignment replacement, missing/null clears optional roles |
| progress | `{...revisionInput, status: "todo" / "in_progress" / "review"}` | Detail; terminal values are not deserializable here |
| note | `{...revisionInput, body}` | Detail |
| submit | `{...revisionInput, body}` | Detail; immutable deliverable and enters review |
| review | `{...revisionInput, decision: "accept" / "return", comment?: ""}` | Detail; review→done or review→in_progress, human only |
| cancel | revisionInput | Detail; human only |
| archive | `{...revisionInput, archived: boolean}` | Detail; terminal tasks only, history retained |
| link-execution | `{...revisionInput, workTaskId: positive integer}` | Detail; human Assign permission, existing live backend run only, never launch |
| entrust-execution | same closed input as link-execution | Detail; **actual protected operator transport only**, establishes source ownership before human linking, never launches/prompts |

`Task = {id, organizationId, title, notes, domain, status, priority, dueDate: string|null, ownerId, assigneeId: string|null, creatorId, reviewerId: string|null, revision, currentDeliverableId: string|null, createdAt, updatedAt, archivedAt: string|null, capabilities}`. `capabilities = {edit, assign, progress, review, comment, submit, cancel, archive, linkExecution}` are current per-task booleans, not authority tokens. canCreate is current human Create permission in selected/any allowed domain. My work means owner, assignee or designated reviewer equals the authenticated member.

`Detail = {task, activity: Activity[], deliverables: Deliverable[], execution: Execution|null}`. `Activity = {id, revision, kind, actor: {id, displayName, kind}, payload, createdAt}`; payload is a typed public change summary, never arbitrary input claims. `Deliverable = {id, revision, author: {id, displayName, kind}, body, createdAt}`. `Execution = {workTaskId, runSeq, agentMemberId, active}`; no folder paths, connection IDs, delegation grants, credentials or private-source content are returned. All task content in this increment is public to its authorized domain; no private import/attachment feature is implied.

## Authorization and revisions

- Visibility is current organization/domain access. Hidden/foreign tasks and references return not-found. Viewer reads only. Human managers/admins/owners manage within their domains; ordinary contributing humans can edit/progress/submit/cancel tasks they own, created or are assigned. Any contributing human in the task domain may add a note. Only Assign-authorized humans can change assignments/link an executor; creation may default to self owner and optionally self assignee without Assign. Naming another owner/assignee or a reviewer requires Assign.
- Review requires current human Review grant and, if designated, that reviewer ID. Agents cannot review, cancel, archive, assign, link, create or administer identities. Progress has no terminal completion value. Editing metadata/assignment while in review invalidates review and returns to in_progress. Every change has one new revision/activity row, so a stale accept cannot approve edited content. Review always reads current exact task/deliverable revision.
- Done/cancelled tasks cannot receive contributions; authorized humans may explicitly reopen through progress. Reopening, cancellation, domain/assignment change or archival revokes stored execution binding; an old agent grant cannot revive. Archive is retained-history deletion; no hard delete API.
- Writes call business_identity::begin_write **before** auth reads, then credential/member authorization, task/reference/live-run checks, revision CAS, immutable activity and commit on that same transaction. Reads revalidate identity/scope in a read snapshot. Cached capabilities grant nothing.

## Agent linkage and transport

R1 correction: a live engineering run is not business ownership. The protected operator must first explicitly entrust that exact generation to the task's current organization/domain/assigned business agent. `entrust-execution` checks actual `Principal::is_operator()` (an owner-role member credential remains false), reauthorizes in the writer transaction, derives root/run/key under the engine lifecycle lock and stores immutable source provenance. It creates one `execution_entrusted` activity/revision but delegates nothing. No input can supply the agent UUID, scope, root, generation, authority object or delegation grant.

Link requires that prior source record, a currently assigned permitted agent and the same already live/indexed engineering generation. The source task revision must equal the current task revision before linking. Assignment/domain/cancel/reopen changes, including changing away and back, advance it and invalidate the earlier source authority. V1 deliberately fences any intervening task edit; a new generation must be entrusted if that source revision becomes stale. This is stricter than matching final field values and avoids hidden resurrection. Only the entrusted root may make business agent calls; a same-type delegated Pi child is not that business member. Existing non-business delegation semantics are unchanged.

Backend derives root connection, run sequence and stable wire key; the operator's explicit source entrustment establishes the separate business member identity. Link does not start/instruct a model, expose legacy routes or give members filesystem authority. One immutable source entrustment and one binding per engineering generation prevent silent retargeting: even after revocation, that generation cannot be linked again to another task or grant. A human must start a new engineering generation before linking again. Otherwise a delayed payload could land on a different task whose numeric revision happened to match.

In the authorized link transaction, persist only `business_identity::delegation_grant(&Principal)?.to_storage()?`, plus immutable org/task/agent/run/connection/key provenance. The opaque grant has no bearer and cannot be supplied by a request. Token-parent resolution uses existing live engine index/ancestry, loads exact binding, then agent_principal_from_binding restores original human **and original credential** lineage. Current human/agent grants and live row are rechecked in the same task transaction. Never construct an operator principal from a task ID. Cancellation, changed generation, retired ancestry, reassignment and credential/member revocation fail closed.

Scoped agents read their one linked task and contribute progress/notes/deliverables through the same core. No cross-task search, role/review/approval or provider credential operation. Stable pi policy identity, Astra/max guard, destructive human floor and engineering behavior remain intact. Membership does not provide OS isolation in the inherited single-user full-filesystem trust model.

Migration `m20260908_000010_business_tasks` is owned here; identity owns 000009 and initial shared registry/router wiring. Relative router export: `business_tasks::http::router()`.

## Provenance and required evidence

Codeg Apache v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed` and accepted `4ec04d7282a50529335d724438d42b99a53385a2`: entities, migrations, CAS/event transaction, protected dual-runtime wrappers and existing token/run bridge. Exact files in NOTICE/report.

IntroMail `0bd24dfe284b888aa9f602fa1fd00e337ea38874`: owner-authorized basic task fields and common human/agent operation boundary only. Actual docs/BORROW-PLANE-OPENPROJECT.md read. Favorites, relations, watchers, source-key/field-journal and uncertain copyleft portions excluded. Create/PATCH destination authorization gaps corrected. No Plane/OpenProject/GPL/AGPL source or tests ported.

Required evidence: human-only flow; active/domain/reference/spoof/viewer denial; two-connection revision race/audit rollback; date validation/roundtrip; scoped agent and original credential/run cancellation fencing; relevant legacy regressions; focused default/server/companion gates; actual guarded synthetic Playwright CLI integration. Passing results are recorded only after execution.
