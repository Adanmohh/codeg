# Working together in Hafidh Ops Desk

This guide describes the accepted Increment A shared-work foundation. See [STATUS](../STATUS.md) for
the current acceptance and packaged-app state.

Start with a piece of work: prepare a customer welcome, review website copy,
summarize feedback, or plan a campaign. You can create, assign and finish it
without opening a chat, terminal or engineering project.

## Join your workspace

Use the workspace address and personal access token supplied by your
administrator. Your role and permitted areas determine what you can see and
change. A viewer can read permitted shared work; someone with contribution or
review rights sees the corresponding actions.

The desktop also offers **Administrator access → Use this desktop’s local
workspace**. On first use, its operator creates the organization and names its
owner. Local desktop work and a shared server are separate stores; connecting
to a shared workspace is an explicit choice. Merely opening another desktop
does not synchronize local data.

Access stays in the current window. Reloading requires signing in again.
Disconnect when switching identity. Treat personal access as a credential;
administrators can revoke it without changing everyone else's access.

## Move a piece of work forward

1. Choose **Create task**. Give it a clear title, brief, area, priority and an
   optional due date. The date is a calendar day, with no reminder or time set.
2. Set responsibility: an accountable human owner, an optional assigned person
   or agent, and a reviewer when one specific person should decide. These are
   separate responsibilities.
3. Use **My work** for what you own or are assigned, **Shared work** for permitted
   organization work, and filters or the board to find the next task. Refresh
   retrieves current shared state.
4. Start work, add notes and submit a deliverable for review. Other permitted
   people see the same saved task and named activity.
5. A permitted human reviews the current work and explicitly accepts it or
   returns it for more work. Agent progress alone cannot mark work Done.

If someone changes a task while you are editing, your stale save is blocked
and your draft stays available. Load the current task, compare the saved
status/revision with your draft's base, then explicitly adopt the current
revision or discard your draft. A changed review requires fresh confirmation.

## People, agents and engineering

Administrators manage named people, agent identities, roles and work areas in
**People & agents**. Agent assignment records responsibility; it does not
automatically start a model. An administrator must first authorize the source
run, and a permitted human links the assigned agent's existing execution.
Agent contributions remain scoped to the assigned task and return for human
review. Revocation or reassignment fences the old authority.

Operators can open the engineering workspace when implementation needs a
project, terminal or code review. Restricted shared-member access does not
grant the operator's engineering powers.

## What the work areas mean today

Marketing, Channels, Ads, Website, Feedback and Engineering organize shared
briefs, responsibility, deliverables and decisions. They do not mean an ad
account, publishing channel, website host or meeting service is connected.
Meeting/feedback import and additional business adapters follow the shared-work
foundation; [implementation scope](BUSINESS-IMPLEMENTATION.md) records that order.
The existing email, bug-intake and approval workflows have their own
[setup and validation runbook](../reports/phase1-beta-runbook.md).

Current limits: lists refresh manually; there is no offline merge or automatic
cross-device sync. English and Arabic business copy are available; other app
language preferences use English for these new surfaces. Local synthetic
acceptance does not establish live provider delivery or paid model execution.
