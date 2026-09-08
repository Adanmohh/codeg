# Role-based business workspace — research checkpoint

2026-09-08. Research only; product implementation is paused at the owner's
request to ideate. Current Ops Desk main is 7225e111. Findings below distinguish
inspected implementation from target architecture. No live integration or new
production behavior was tested. Three Herdr Astra/max workers are researching
bounded source comparisons; final synthesis will incorporate their reports.

## How close are we?

We have a working desktop/agent execution and human-review foundation, plus
specific email and bug workflows. We do not yet have the shared organization
platform in the owner's expanded vision. A percentage would be misleading:
the difficult execution plumbing exists, but identity, shared business-task
ownership, role enforcement, synchronization and most business integrations
remain substantive work. The old Phase 1 completion applies to the earlier
bounded scope, not this expanded product.

## Intromail: verified reusable task basics

Read through gh api at current main commit
`0bd24dfe284b888aa9f602fa1fd00e337ea38874` (same as the prior founding pin):

- `backend/app/models.py`: User includes is_agent, with a system-user identity;
  TaskList.owner_id NULL is team-shared, otherwise personal. Boards can be shared
  and contain existing lists. Tasks have parent/thread/client links, due dates,
  recurrence, completion, labels, attachments and source/id deduplication columns.
  There is no dedicated task assignee field in the inspected Task model or
  TaskIn/TaskPatch/TaskOut DTOs. created_by and list owner are not assignment.
- `backend/app/services/task_ops.py`: human HTTP routes and agent actions reuse
  task operations. Visibility selects own/private and shared nonarchived lists.
  This is a single-team foundation, not a demonstrated multi-organization RBAC
  implementation. Completion and move behavior retain recurrence/event hooks.
- `backend/app/services/agent/tasks_pack.py`: six actions create, complete,
  reopen, set due, move and add note. Visibility binds the agent and approving
  human; delegation does not widen access. Source tests exercise hidden task/
  hidden approver/destination refusal, idempotent completion and shared hooks.
- `backend/app/models.py` TaskRelation + `routers/tasks.py`: directional blocks
  and symmetric relates links, cycle/duplicate prevention and visibility checks.
  Notes have actual authors/source; watchers and personal favorites are distinct.
- `frontend/src/pages/tasks/TaskDetailPanel.tsx`: task-scoped Ask the agent link.
  The agent participates in the existing task surface rather than requiring a
  separate agent-only task list.
- `backend/app/services/mcp/tasks.py`: MCP protocol tasks represent pending
  approval Proposals. They are NOT the team's business Task records. Preserve
  this distinction when designing our API and UI.

Source tests were inspected, not executed in this research pass. No Intromail
product source was copied into Ops Desk. Further ports require per-file provenance
review; references to Plane/OpenProject patterns do not authorize copying their
copyleft source through another repository.

## Existing Ops Desk boundary

`src-tauri/src/db/entities/work_task.rs` is a folder-bound execution lifecycle:
queued/preparing/running/awaiting_input/review/merging/done, run_seq, worktree,
connection, git base/branch, result and acceptance metadata. It has valuable CAS
and review invariants, but is not a general organization assignment model.
`work_task_event.rs` records user/engine/agent actor categories, not a complete
member ownership model. `web/auth.rs` authenticates an operator token; that is
not per-person organization authorization. Multiple desktop applications cannot
become a coherent organization merely by hiding different navigation items.

## Production pattern comparisons — root

Frappe framework v15.120.0, commit
`755b5cb81fabb431265690fca07f4a8038a5599a`, MIT verified from LICENSE:
`frappe/desk/doctype/todo/{todo.json,todo.py}`, `desk/form/assign_to.py`,
`permissions.py`, and `core/doctype/role/role.json` were inspected. Its assignment
records identify allocated_to/assigned_by and reference the business document;
permissions consider roles and the specific document. This is a useful model
for assignment independent of a work artifact. Its assignment helper may share
an inaccessible document with the assignee when sharing is enabled: do not copy
that policy blindly into agent access. No Frappe installation or stack replacement
is proposed by this source comparison.

Temporal v1.31.2, commit `19a774302c613da9adc4436ab14278ccdca8e0a5`, MIT:
README describes durable workflow execution/recovery/retries. This solves a
background execution problem; it is not a human task manager, permission system
or MCP connector. We have not validated a Temporal integration. It is a candidate
only if long-running background workflows justify operating another service;
retry support alone never makes external side effects exactly once.

## Proposed architecture for discussion

1. Shared organization service owns people, role/resource grants, projects,
   business tasks, assignments, source records and approval/audit history.
   Desktop remains the main work surface and local tool executor. This does not
   require moving every local artifact to a server or claiming offline sync exists.
2. One shared task can have a human accountable owner and human/agent assignments.
   Domain and role change its presentation and permitted tools, not its identity.
3. Each agent attempt/run links to that task. Existing engineering execution is
   one execution type; a marketing task need not have a repository/worktree or
   a merging lifecycle. Human work must progress without starting an agent.
4. MCP/CLI/API adapters expose scoped capabilities. Server-side action checks
   enforce shared-system authority. An unrestricted local CLI can use ambient
   machine credentials outside app policy; a role-based menu is not containment.
   Execution credentials and runner isolation need an explicit production design.
5. Source ingestion creates durable, deduplicated records and proposed work.
   Assignments, approvals and attempts retain provenance and distinct lifecycles.

## Meetings and note-taking sources

Fireflies official Node SDK commit
`76d7983a1b3592a1662e77157f1ff1d85216acb4` was read via gh api: transcript and summary
read methods are present. The old docs-fireflies GitHub repository does not
provide the current schema, so official documentation was checked as fallback.
[Summary schema](https://docs.fireflies.ai/schema/summary) exposes action_items
as AI-generated text, not a normalized assignee/due-date task contract.
[Transcript schema](https://docs.fireflies.ai/schema/transcript) supplies source
identity, participant/speaker context and transcript URL.
[Webhooks V2](https://docs.fireflies.ai/graphql-api/webhooks-v2) distinguishes
transcribed from summarized readiness and documents signature verification.

Proposed workflow: source meeting/note → extracted candidate tasks → owner/
project/date clarification → shared task → agent/human execution → reviewed
outcome. Keep source links, exact supporting text, access scope and import IDs.
Treat ambiguous attribution or a suggested date as a question, not a commitment.
The source connector should not create a separate Fireflies-only task store.
No Fireflies account was accessed and no integration is implemented.

## Connector quality evidence

The official modelcontextprotocol/servers README at `d73f99efbfd40c3aa1b61e88728b3d49fb52608f`, read through gh api, explicitly says its bundled servers are educational reference implementations, not production-ready solutions. A directory listing or protocol implementation is not evidence of operational readiness. Prefer maintained provider-supported surfaces where available, then verify concrete scopes, tool contracts, pagination, rate handling, versioning and write outcome semantics.
