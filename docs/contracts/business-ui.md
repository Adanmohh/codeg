# Business workspace UI contract

Worker: rebrand; branch `feat/business-workspace`; baseline
`4ec04d7282a50529335d724438d42b99a53385a2`. This is the frontend integration
checkpoint for Increment A in `docs/BUSINESS-IMPLEMENTATION.md`.

The workspace is a direct-action interface: My work, shared work, review,
list/board, creation and task detail. Conversations and engineering execution
are supporting destinations, never prerequisites for human work.

## Ownership and integration needs

- Frontend-owned additions: `src/app/business/`,
  `src/components/business/`, `src/lib/business/`, associated focused tests and
  locale copy. Minimal entry-point changes may touch the existing root/login,
  navigation and provider guards to isolate business members from operator UI.
- Preserve all existing engineering routes, Drawer implementation, themes/fonts,
  Ops approval/receipt behavior and backend files. No Rust edits in this branch.
- Identity contract authority: approvals' `docs/contracts/business-identity.md`.
  Task contract authority: tickets' `docs/contracts/business-tasks.md`.
  Identity wire/implementation is `861fb0ef` (accepted `c911c406`, main
  `ab46c9d9`); fixed task DTOs are `bf4309f5`, public activity/HTTP source
  `76bb6909`, and combined task checkpoint `1e8b5250`. No second authentication
  or production response layer is invented.

Identity needs: connection/sign-in and revocation behavior; current organization,
authenticated principal and server-derived capabilities; authorized human/agent
directory with active status; operator bootstrap and member administration;
explicit transport support for local desktop and a shared server. The business
credential must never be written to `codeg_token`, handed to the inherited
WebTransport, or used for config/terminal/model-provider requests. The member
route must avoid mounting the legacy engineering workspace providers. Current
root appearance/i18n/connection providers need inspection for incidental API
calls. Identity, role and organization are never taken from form-supplied actors.

Task needs: authorized list/detail with domain/status/owner/executor/priority/due
date and revision; creation/edit/assignment/status/review operations; per-task
capabilities or equivalent server-derived action availability; notes, deliverable
and activity; pagination/filter semantics and stable conflict/error codes. An
accountable human and assigned executor are separate controls. A source or target
principal appearing in a picker is not an authorization grant. Status and review
actions must follow the backend's allowed transitions; an agent cannot review
its own work. Human progress cannot mark work done; a separate human review
operation does that. Shared work must function without a folder or running agent.

Confirmed wire decisions: organization/member/task IDs are UUID strings. Due date
is nullable `dueDate`, strictly `YYYY-MM-DD`, a calendar day without an instant,
timezone conversion or reminder. Use a date input and show the exact day. The
older task checkpoint's `dueAt`/RFC3339 proposal is superseded by owner direction.

Identity operations use POST `/api/business/*` with `{input: ...}`; the client
mirrors the published paths and native commands. `Context.capabilities.manageMembers`
controls administration; **only `legacyOperator`** controls the engineering entry.
An owner role or engineering domain is insufficient. Native business commands stay
operator-only; a desktop member uses the separate shared-server HTTP client.
Personal bearer and drafts remain in memory, scoped to one client/session. The
client omits cookies, refuses redirects, and clears itself on authentication
failure. Original operator bootstrap remains useful before any member exists.
One-time issued credentials are masked by default and intentionally copied or
revealed; never persisted or exposed in fixtures, snapshots or reports.

Cold `/business`, `/business/`, `/business.html`, `/` and `/index.html` skip inherited settings,
wallpaper and operator connection providers, even with an ambient old operator
token. `/business-other` retains inherited behavior. Server source at the base
(`web/router.rs:1735`) rewrites extensionless/trailing-slash routes to the export
and serves `.html` directly; actual browser request verification remains pending.

The implemented client retains the revision originally edited and keeps drafts in memory
across resize/locale changes. It exposes conflicts without silently applying an
old draft over a fresh revision. Refresh, inspect current values, then explicit
resubmission. Session/organization changes clear private state and discard late
responses. Errors never render raw provider/server bodies. No mocked production
responses or fallback success.

## Composition and visual direction

Direction: a working agenda, using Codeg's compact navigation and task/detail
patterns with a clear human responsibility trail. World: briefs, commitments,
owners, handoffs, review. Existing user-selected fonts and semantic color tokens
take precedence over generic Design Studio font/palette suggestions.

Default light references: background/card #ffffff, sidebar #fafafa, foreground
#171717, border #e5e5e5, Hafidh primary #245e58. Paired dark semantic tokens remain
intact, including primary #9bd4c5. No global token overrides. Heading 28–32px,
body 14–16px, secondary 12–13px; tabular figures for actual counts/dates. Use the
selected UI font (default Inter), not a newly installed display family.

At 1280px: deliberate branded navigation, a useful page heading and primary
Create task action, compact actionable review area, then a substantial work
list/board. Detail opens as a focused reading/editing surface. At 390px: compact
navigation and filters, full-width task rows and detail, 44px targets and no
horizontal page scroll. At 768px: preserve readable rows and controls rather than
shrinking desktop columns. RTL uses logical alignment; names/content use bidi
isolation. Focus and reduced motion remain visible and stable.

Signature: each task's actual human owner and executor appear as a readable
handoff; the same responsibility trail anchors rows, detail and review. Status
is text plus a shape, not color alone. Self-critique removed the dispatch-console
hero, decorative metric tiles and repeated feature cards. The real work itself
is the visual artifact. No fake ads, revenue, connection state or future modules.

## Validation and fixture contract

Reserve frontend-owned loopback port **4340**, fresh export
`out-business-workspace/`, and two named Playwright CLI sessions
`business-owner4340` and `business-member4340`; no server/browser launched yet.
Fixture must be explicitly synthetic and guarded, with separate named principals
and no live provider/model access. Identity/task workers supply the actual
protected backend and safe fixture setup contracts; do not reseed another
worker's database. Secret values stay out of screenshots, snapshots and reports.

Required checks: two-session task visibility and mutations; viewer restrictions;
stale revision conflict with draft preserved; revoked membership; assigned human
versus agent and distinct reviewer; create/edit/review with no conversation;
empty/loading/offline/permission states; no member calls to inherited operator
APIs. Actual 390/768/1280 light/dark, RTL, keyboard/focus and long content checks.
Focused behavioral tests, lint, typecheck and an owned static export precede the
final Design Studio measured review. Backend implementation/gates remain with
their owners; this report will record exact integrated revisions.

## Implemented operation boundary

`src/lib/business/identity.ts` and `tasks.ts` mirror the owners' exact DTOs;
`client.ts` sends POST `{input}` to closed operation paths, with native
`business_*` / `business_tasks_*` commands. Task IDs are inside input, never URL
segments. Update/assign replacement fields send explicit nulls; no caller-supplied
actor/organization override is added to task operations. List uses `shared`/`mine`,
page/domain/status/query/archived and returned `{tasks,page,hasMore,canCreate}`.
Detail uses returned `{task,activity,deliverables,execution}`. Per-task capabilities
govern edit/assign/progress/comment/submit/review/cancel/archive/link-execution;
the backend revalidates authority on every mutation. There is no `done` progress
option, agent review control or automatic execution launch.

Nullable reviewer is explicitly **any currently authorized human reviewer**.
Activity renders only the pinned public payload fields as plaintext and resolves
names through the authorized directory; unknown fields are not JSON-dumped.
Missing directory entries display an unavailable person, without guessing kind.
Conflict recovery keeps the original edited revision until the operator explicitly
loads/compares and adopts current data. An ambiguous create is not auto-retried.

A native operator may choose the local workspace. Desktop members use the same
separate HTTP client as web members, not legacy remote-connection storage.
Original operator reuse is an intentional same-origin sign-in action only.
A role/domain never grants legacy entry. One-time credential values stay in
component memory, masked in DOM by default and cleared on close/disconnect.
Session/principal/membership revision changes reset private editing state; locale
and viewport changes do not. Reload requires signing in again.

Current validation is 75 focused tests, typecheck and scoped lint passing; export
build succeeded before the latest activity presentation addition. Actual runtime
and browser checks remain pending real task fixture integration. English/Arabic
business copy is supplied; other existing locales use English business copy.
The fresh export is ignored and reserved; no 4340 process/browser is running yet.

### R1 source-ownership alignment and actual fixture

Task contract `1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5` adds
`tasks/entrust-execution`, with the same `{taskId,expectedRevision,workTaskId}`
input and native `business_tasks_entrust_execution`. The engineering detail shows
this control only with server `legacyOperator` and an assigned agent. The human
must explicitly confirm source ownership, then link separately using the returned
revision. No auto-link or engine launch. Role-owner membership alone never shows
entrustment; the backend remains authoritative. Any intervening edit invalidates
the entrusted source revision and requires a new execution generation.

The owned 4340 export guard is now running, with the real task fixture on4342.
The task owner's initial listener closed after its own proof; an unchanged
`git archive` snapshot of `1ba73e3c` was compiled in this worker's ignored build
directory and started with a fresh in-memory database. This does not integrate
unaccepted backend product code into the branch. Original 4326 and all other
worktree outputs remain untouched. Two named CLI sessions are open; initial real
context returned `needsBootstrap=true`. Task/member UI flows are next.
