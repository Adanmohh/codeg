# Business tasks — implementation contract checkpoint

Owner: tickets worker, `feat/business-tasks`, base `4ec04d7282a50529335d724438d42b99a53385a2`. This is the early interface checkpoint; implementation and validation are in progress. Identity scalar types, principal construction and authorization helpers will follow the approvals-owned `business-identity.md` contract before integration. No alternate authentication model is introduced.

## Shared record and operations

A business task has its own ID, organization, title, notes, domain, status, priority, nullable due date, accountable human owner, assigned human/agent executor, immutable creator, designated human reviewer, revision and timestamps. Owner, executor, creator and reviewer are separate fields. A task remains useful with no git folder, connection or agent. The existing engineering `work_task` is an optional linked execution, never the business task itself.

- Status: `todo | in_progress | review | done | cancelled`.
- Domain: `marketing | channels | ads | website | feedback | engineering`.
- Priority: `low | normal | high | urgent`.
- Dates: nullable RFC3339 UTC strings on the wire; invalid dates rejected.
- Text: bounded plain text; no HTML execution, arbitrary attachment fetch or source-private context.
- Every mutation takes positive `expectedRevision` except creation. Unknown input fields, caller actor/role/organization overrides and invalid principal references are rejected.
- All mutations use one domain operation path for HTTP, Tauri and the scoped agent bridge. Authorization, reference validation, revision CAS and immutable activity append occur in the same writer transaction.
- Read results include real per-task capabilities for edit/assign/progress/review/comment/archive/link execution. Views and assignment never grant access.
- Task/activity reads are organization/resource scoped. A foreign or inaccessible task is indistinguishable from missing; no audit or principal metadata leaks through list/detail/errors.
- Archival/deletion preserves authorship/activity. No hard deletion of audit rows.

## HTTP/frontend shape to coordinate

Reserved task surface: `/api/business/tasks`, with list/create and task detail/mutation endpoints under `/{id}`. The identity owner owns mounting the member-authenticated subtree; member credentials must not enter legacy operator routes.

DTO field names planned for the UI: `id, organizationId, title, notes, domain, status, priority, dueAt, ownerId, assigneeId, creatorId, reviewerId, revision, createdAt, updatedAt, archivedAt, capabilities`. List filtering covers My work/shared, domain/status and bounded pagination. Detail includes append-only activity and deliverable records with actual actor identity and optional linked execution metadata. Scalar identity IDs will exactly match the identity contract.

Separate typed operations: edit task fields; assign owner/executor/reviewer; change progress; add note; submit deliverable for review; accept/return review; archive; link existing execution. Routing is thin: no duplicated SQL or identity supplied by the UI. A stale revision returns a stable conflict code and requires reload. Rebrand owns the actual frontend/API client; this worker owns the Rust endpoints/core.

## Identity helper needs — approvals owner

Please publish the concrete opaque authenticated principal type and organization/principal ID scalar; transaction-compatible revalidation of session/member active status; same-organization active reference lookup; human versus agent kind; domain/discipline read/write/assign/review capabilities; and an operator-to-business owner boundary usable by Tauri without accepting a serialized actor.

Task operations need to validate all referenced principals under the same transaction, including changed owner/assignee/reviewer and changed domain. Agency is not a human member bearer: stable agent principal plus live trusted parent/task/run and current delegating-human authority must be revalidated before contribution. Agents get no review, role, identity administration or provider credential operation. No impersonated legacy `Operator::server`.

Reserved migration: `m20260908_000010_business_tasks`; identity owns 000009 and initial shared registries/router. Minimal module/migration/route registrations will be coordinated additively, preserving all existing entries.

## Agent boundary and acceptance

The existing token/listener/task/run ancestry remains authoritative. Human-authorized linkage binds the business task to an existing executor generation and its agent principal; an old run, cancellation, revoked member or reassignment cannot contribute. The agent may read assigned authorized public context and submit progress/deliverables. Review completion is human-only. Linking does not launch a model or change engineering engine behavior.

Planned proof: human-only CRUD/progress; viewer denial; same-org and foreign/inactive reference checks; inaccessible create/change destination regressions; independent simultaneous revision races; audit rollback/immutability; scoped human/agent read and contribution; agent self-review denial; revoked/stale/cancelled execution; existing engineering regression selectors; default/server/companion compilation and Clippy where touched; frontend typecheck; real two-session guarded fixture and Playwright CLI after the accepted identity/UI seams are available.

## Source/provenance boundary

Codeg Apache v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed` and accepted local `4ec04d7282a50529335d724438d42b99a53385a2`: entity/migration patterns, conditional task updates with activity in one transaction, protected transport wrappers and existing live-run bridge.

IntroMail `0bd24dfe284b888aa9f602fa1fd00e337ea38874`: common human/agent task operation boundary and basic title/notes/due/creator model, subject to hunk provenance review. The actual `docs/BORROW-PLANE-OPENPROJECT.md` was read. Favorites, relations, watchers, source-key/field-journal implementations and other provenance-uncertain copyleft-derived portions are excluded. Source authorization gaps in create/PATCH destination checks are not carried forward. No Plane/OpenProject/GPL/AGPL source is ported. Exact retained mappings will be added to NOTICE with the product implementation.

