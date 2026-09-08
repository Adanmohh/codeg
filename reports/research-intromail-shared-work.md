# IntroMail shared work foundation — research only

2026-09-08. **Recommendation: selectively borrow the task service and human/agent
attribution patterns, not the complete authorization or execution foundation.**
IntroMail supports several users sharing tasks inside one installation. It does
not yet supply organization membership, task assignment, or distributed task
execution suitable for the proposed desktop plus shared-server product.

Product implementation remains paused. This report is uncommitted on
`feat/visual-workspace`, local HEAD
`5613962451389b4c50b17b0126ab9e84a9af6b58`. The existing visual checkpoint,
product files, exports, fixtures and browsers were left unchanged.

**Authority and verification.** `gh api repos/IntroInnovation/intromail/commits/main`
confirmed the requested immutable commit
`0bd24dfe284b888aa9f602fa1fd00e337ea38874`, committed
`2026-09-02T21:54:25Z`, tree
`2a803fd43d6322c5a64d3cede78e33f924401e1c`. The recursive tree was not truncated.
All remote source reads used `gh api .../contents/<path>?ref=<that SHA>` with the
raw-content accept header, after reading installed `gh api --help`; these calls
exited 0. Every IntroMail link below uses that commit, not moving main. Source was
cached only in this worktree's ignored `.build/research-intromail-shared-work/`.

The existing code-context preparation found no installed-dependency corpus for
this worktree (`rebrand.db` absent; docs retrieval exit 3); its offline guide
retrieval exited 0 using the existing RAG environment. This audit relied on
actual source, not missing corpus coverage. IntroMail's
[requirements file](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/requirements.txt)
uses dependency ranges; I did not install dependencies, execute its tests, or
claim verification against an installed IntroMail environment.

## Verified foundation and limits

**Identity and assignability.** `User.is_agent` is real, and agent actions retain
separate agent and approving-human identities. However, the complete `Task`
model and `TaskIn`/`TaskPatch`/`TaskOut` have **no assignee field**. A task has a
list, creator, optional parent/thread/client links and a completion timestamp.
`client_id` identifies a billing customer, not a responsible user. The agent
appears in the team directory; actual assignment belongs to **email Thread**
(`assignee_id`). `enqueue_assignment` accepts a thread, while task notes can
enqueue mentions. The task action pack offers create, complete, reopen, set-due,
move and add-note; no assign action. Watchers and favourites are subscriptions
and personal preferences, not assignment. The identity helper selects the first
agent user globally; it is not an organization-scoped agent directory.
Sources: [models.py:68, 137, 341–425](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/models.py#L68),
[identity.py:69, 167–205](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/agent/identity.py#L69),
[task DTOs](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/routers/tasks.py#L186),
[tasks_pack.py](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/agent/tasks_pack.py#L83).

**Shared access is installation-wide.** Lists are personal (`owner_id=user`) or
shared with everyone (`owner_id=NULL`); boards use a similar convention. The
model contains no organization or task-list membership entity. Authentication
resolves a stored user from a signed token; admin is a global boolean. Login
rejects the agent account. MCP has a useful distinct authenticated
`Principal(user, token)` with hashed/revocable tokens; it does not accept a caller
supplied user identity. That is an authentication foundation, not organization
authorization. Also, MCP's asynchronous “task” is a **Proposal**, not the business
`Task` row. Sources:
[task visibility](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/task_ops.py#L43),
[security.py](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/security.py#L84),
[login](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/routers/auth.py#L216),
[MCP principal](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/mcp/auth.py),
[MCP task mapping](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/mcp/tasks.py).

**The shared service is useful, but authorization is not consistently centralized.**
`task_ops` owns completion effects, recurrence, reminder resets and coherent
parent/list moves. The action pack's apply paths check visibility for both agent
and human. Static inspection nevertheless found these concrete gaps:

- HTTP create calls `task_ops.create_task`, which checks that the destination list
  exists but not that the caller can access it. A private-list ID reaches this
  mutation without the pack's visibility check.
- HTTP PATCH checks the original task, then assigns a supplied destination
  `list_id` without destination visibility validation. The dedicated move service
  does validate the destination. These paths have diverged.
- Shared-list edit/delete permissions allow any authenticated member; changing a
  shared list to personal makes the caller its owner. There are no separate
  reader/editor/assignment-manager roles.
- Legacy `email.create_task_from_thread` creates ORM tasks outside `task_ops`;
  its list fallback can select any list if no shared list exists. Its existing
  task lookup is global. Do not carry this path forward as shared authorization.

These are source findings, **not live exploits or executed regressions**. Sources:
[create/PATCH routes](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/routers/tasks.py#L1119),
[create service](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/task_ops.py#L232),
[validated move](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/task_ops.py#L322),
[list permissions](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/routers/tasks.py#L625),
[legacy thread action](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/agent/actions.py#L244).

**Gating and execution require separate judgments.** The gate supplies useful
deny/ask precedence, a destructive floor and default propose behavior. An ask
rule returns before `check_permission`; therefore permission checks that must
precede queueing cannot live only there. Task create/move previews also use raw
list lookups, so apply-time checks alone are not a sufficient preview boundary.
Approval reparses edited input and invokes apply, but does not rerun the gate.
The HTTP router checks pending status; the service does not atomically claim it.
`_execute` applies first, then stores the result and resolves the proposal.
Consequently, concurrent approval and crash recovery need additional guarantees;
schema revalidation is not stale-policy or exactly-once validation.
Sources: [gate ordering](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/agent/gating.py#L105),
[preview/apply boundaries](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/agent/tasks_pack.py#L121),
[proposal execution](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/agent/proposals.py#L120),
[approval route](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/routers/agent.py#L210).

This does **not** describe the accepted Desk GitHub filing safeguards: local
`src-tauri/src/ops_intake/store.rs:263` records a durable `unknown` receipt before
dispatch, with bound evidence and conflict checks; `ops_intake/mod.rs:127` supplies
GET reconciliation. Preserve those hardened Ops contracts rather than replacing
them with generic IntroMail proposal execution.

**Queue and idempotency have a deliberate single-process boundary.**
`pending_events` loads all pending rows, sorts in Python and only then limits.
`dispatch_pending` calls hooks before marking processed/failed; that module has
no atomic claim, lease or retry policy. The scheduler explicitly documents a
single-Uvicorn-worker requirement, uses an in-memory scheduler, and registers
dispatch/sweep jobs with `max_instances=1` and coalescing. Both call the synchronous
dispatcher. This supports a single-dispatcher foundation; it is not evidence of
a deployed multi-worker bug. Deployment topology was not inspected or tested.
Sources: [queue.py:71–101](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/agent/queue.py#L71),
[scheduler assumption](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/scheduler.py#L22),
[dispatch/sweep](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/scheduler.py#L224),
[job registration](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/scheduler.py#L494).

Task `(external_source, external_id)` uniqueness prevents duplicate stored source
keys, and the thread action reuses an existing task sequentially. It is global,
not organization-scoped; manual creation deliberately permits multiple tasks
per thread. Completion checks an in-memory `completed_at` value without a CAS.
Neither that check nor the unique source key is a general request/side-effect
idempotency system. Task also lacks a revision/`updated_at` for concurrent edits.
Sources: [Task model](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/models.py#L367),
[completion](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/task_ops.py#L292).

**Recurrence, relations and notes are useful bounded features.** Recurrence
supports FREQ plus INTERVAL for daily/weekly/monthly/yearly steps, not a complete
RRULE/timezone engine. Completing creates the next task from the previous due
date and copies favourites, but omits client/thread/GitHub links, labels,
attachments and child trees. Reopen then complete can generate another successor;
there is no occurrence identity. `blocks`/`relates` have reverse presentation,
both-end visibility and bounded cycle rejection, but do not prevent completion
or schedule dependent work. Opposite-edge concurrency is not made atomic by the
ordered triple unique constraint. Notes have sanitized HTML/plain text,
authorship, soft deletion and author/admin edits; standalone journal notes are
private to their author. Their visibility helper includes archived lists while
`task_ops` excludes them—another policy distinction to settle before reuse.
Sources: [recurrence](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/task_ops.py#L151),
[relations](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/routers/tasks.py#L1226),
[note service](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/services/task_notes.py#L18),
[note authorization](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/routers/task_notes.py#L51).

## Test evidence and borrowing provenance

Inspected test bodies establish intended coverage, **not a test pass in this
audit**:

| Exact test source | What its assertions cover; important limit |
| --- | --- |
| [test_tasks_pack.py:87–201](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/tests/test_tasks_pack.py#L87) | Completion effects, sequential no-op, daily recurrence, private-list rejection in pack apply. The human-visibility case also hides the task from the agent, so it does not independently isolate the human check. |
| [test_task_idempotency.py:53–147](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/tests/test_task_idempotency.py#L53) | Sequential thread-action reuse, deliberate manual duplicates, unique index enforcement. No concurrent request/crash proof in these cases. |
| [test_agent_core.py:349, 686–741](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/tests/test_agent_core.py#L349) | Edited target/schema validation; actual **thread** assignment event, team inclusion and agent-login rejection. |
| [test_task_relations.py:220–263](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/tests/test_task_relations.py#L220), [test_task_notes.py:45–106](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/tests/test_task_notes.py#L45) | Hidden relation endpoints, sanitizer/author guards and private journal behavior. |

The [test harness](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/tests/conftest.py)
uses disposable SQLite with foreign keys enabled and disables rate limiting.
Those choices do not demonstrate PostgreSQL isolation, distributed dispatch,
desktop synchronization, performance or production recovery. No claim is made
that the entire upstream suite lacks other coverage.

FOUNDING authorizes IntroMail as owner-controlled source; the complete tree
search found no LICENSE/COPYING path establishing a separate public licence.
That authorization does not establish third-party provenance for every hunk.
[BORROW-PLANE-OPENPROJECT.md](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/docs/BORROW-PLANE-OPENPROJECT.md)
explicitly traces source-key idempotency to Plane, favourites to OpenProject,
and relations/watchers to both. It includes suggestions to copy some unrelated
guards verbatim; suggestions are not proof they were implemented verbatim.
[Model comments](https://github.com/IntroInnovation/intromail/blob/0bd24dfe284b888aa9f602fa1fd00e337ea38874/backend/app/models.py#L570)
confirm these pattern dependencies. The cited upstream paths are not accompanied
by immutable upstream SHAs in that document.

Therefore, treat Plane-inspired portions as a provenance review dependency,
not permission to import AGPL source through an owner-controlled repository.
Inspiration alone does not establish copied expression or make all IntroMail
AGPL. No Plane/OpenProject code or licence was independently fetched in this
bounded audit, and no blanket licence classification is inferred for OpenProject.
Any future port needs exact hunk/source-history verification and original
attribution; exclude prohibited source under FOUNDING. This report ports no code
and changes no NOTICE entries.

## Five concrete recommendations

1. **Make shared business work server-authoritative.** Desktop and web should use
   the same authenticated task service. Do not call independent desktop SQLite
   databases shared work. Keep business task identity separate from engineering
   runs and MCP proposal handles; link supporting execution records explicitly.
2. **Add explicit membership and assignment before a shared assignee picker.**
   Bind tasks to an organization/workspace and assignees to authorized human or
   agent principals. Preserve creator and approver separately. Assignment must
   not grant access; define read/edit/assign/approve/manage rights and revocation.
3. **Borrow `task_ops`'s common mutation boundary, then close its route gaps.**
   Every HTTP/desktop/agent path must authorize source and destination, including
   preview and edited approval. Add independent human/agent visibility cases,
   private-destination create/PATCH cases and membership-revocation regressions.
4. **Keep hardened Ops approval/receipt semantics.** Add revision-bound mutations,
   scoped idempotency and transactional event recording for shared work. Stay
   explicitly single-dispatcher until atomic claims/leases and recovery tests
   exist. Exercise concurrent approval/completion and response-loss cases on the
   intended server database; never interpret an unknown external outcome as safe
   to resend.
5. **Keep the first task feature set honest and provenance-cleared.** Reuse bounded
   notes, personal preferences and simple relations only after the source audit.
   Label dependencies advisory unless enforcement is implemented. Either retain
   explicit simple recurrence limits or add occurrence identity, clone policy
   and timezone rules with reopen/concurrency tests. Do not promise a complete PM
   or distributed work engine from this pin.

Research complete; implementation stays paused. No live accounts, outbound calls
other than read-only GitHub source research, installs, services, tests, builds,
commits or PR changes were performed for this audit.
