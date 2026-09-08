# Role-based business workspace — source research findings

2026-09-08. Research only; product implementation is paused at the owner's
request to ideate. Product baseline is the accepted Phase 1 implementation; recent main changes
are ideation/research documents only. Findings below distinguish
inspected implementation from target architecture. No live integration or new
production behavior was tested. Three Herdr Astra/max workers completed bounded source comparisons. Root read
their complete reports and independently checked the central Intromail model/
route findings and Fireflies query/error examples. No production readiness
certification or executed upstream regression claim is made.

## Recommendation

Keep the existing desktop and engineering executor. Introduce one shared
organization task service, accessible by humans and their agents, with explicit
membership, accountable human ownership, executor assignment and resource/action
permissions. The desktop is the work surface; shared work must survive a closed
desktop. A business task links to agent runs, source records and approvals without
becoming a git execution or an MCP protocol task.

Borrow selectively from Intromail's common task operations and human/agent
attribution, preserve the stronger Ops approval/receipt safeguards, and use the
reviewed mature projects as narrow permission/assignment/recovery references.
Do not replace the app with Frappe, install n8n for a query, or adopt a Java/BPMN
or Temporal service without a demonstrated need. Start with server transactions,
revision checks and durable claimed jobs. This is a proposed direction for owner
ideation; no stack migration or implementation has been authorized by this report.

| Source | Useful contribution | Important limit |
| --- | --- | --- |
| Existing Codeg/Ops | Desktop, agent execution, review and external receipt handling | No complete organization/member task platform |
| Intromail | Shared/private task lists, notes/relations and common human-agent operations | No task assignee/org membership; inspected create/PATCH destination checks and generic dispatch/approval recovery need hardening |
| Frappe framework MIT | Assignment as a linked record and role/document permission mechanisms | Assignment can share access by policy; do not blindly inherit that behavior or adopt the full stack |
| OpenProject GPL, idea-only | Separate author/assignee/responsible member and project roles/revision contracts | No source port under current project constraints |
| Flowable Apache-2.0 | Separate human-task ownership, delegation and revision/worker claims | Not a substitute for application permissions; adopting its engine is not justified by this research |
| Temporal MIT | Durable background workflow execution | Not a business task/role/connector layer; unnecessary infrastructure until need is demonstrated |
| Official Fireflies adapter | Better query/filter and GraphQL error patterns than the inspected SDK | No durable ingestion/task assignment guarantee; source access and summary readiness remain app responsibilities |

Complete reviewed reports:
- [Intromail shared work](research-intromail-shared-work.md)
- [Role/work platform comparison](research-role-work-platform.md)
- [Meeting orchestration](research-meeting-orchestration.md)

OpenProject v17.8.0 source pin is `2d1ae9c1f2de49d363e0ec2cea7d7f2991ed405a`;
Flowable 8.0.0 is `0779d68e5a3385b74d8acb8bc37901ff54513249`. Their exact
source links and the worker's inspection limits are in the comparison report.
Root independently verified Intromail HTTP create only checks destination-list
existence in task_ops, and PATCH directly changes list_id after checking the
original task. These are static source findings, not reproduced live incidents.
Do not silently port those gaps, and do not modify Intromail in this research task.

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

## What would demonstrate production readiness?

These are acceptance scenarios for the proposed shared-work foundation, not
passing tests or implementation claims:

- Two people on separate desktops see the same task, with identifiable owners
  and activity; concurrent edits cannot silently overwrite each other.
- A marketing member can use an allowed business account but cannot acquire
  deployment or another team's private source access by assigning an agent.
- Reimporting a meeting or restarting a worker produces one source record and
  one intended task proposal, with the supporting transcript preserved.
- Restart during an external action cannot turn uncertainty into a blind second
  post, email or budget change; result/reconciliation state remains visible.
- A human can own and finish work without an agent; an agent can provide a
  deliverable without claiming the business outcome is accepted or published.
- Multi-day waiting/approval survives a closed desktop; revoked permissions are
  checked before queued work executes. Background work has an explicit host.

The inspected Intromail queue.py loads pending events, sorts them and processes
them before marking processed/failed. That module alone contains no atomic
worker claim, lease or retry policy. This is an operational-scope limit, not a
claim of an observed duplicate in its present single-dispatcher deployment.

## First vertical slice to discuss

One accessible meeting produces a proposed shared task, reviewed by a human and
assigned across two real organization identities on separate desktop sessions.
An agent contributes a non-published deliverable; the human reviews and completes
the business task. Permission revocation, duplicate source import, concurrent
editing and a runner restart are exercised explicitly. This demonstrates the
shared-work product without requiring an ads account, live publishing or a full
CRM. Engineering execution can be linked when needed rather than being the
required lifecycle of every task.

Because shared organization use is now fundamental, the shared service cannot
be deferred solely to the previous Phase 3 hosting milestone. That is a proposed
roadmap correction for discussion, not authorization to deploy a service now.

## Reviewed meeting specialist result

[Meeting orchestration report](research-meeting-orchestration.md) is reviewed.
Root independently verified the SDK's missing query filters and the official
adapter's GraphQL error-envelope handling and MIT notice through gh api.
Prefer the adapter's narrow query/transport patterns over adopting the SDK
unchanged; do not install n8n to import a query. Fireflies source readiness,
access and deduplication remain explicit ingestion responsibilities. The report
also records SDK type/service-schema mismatch and bounded synchronization limits.
