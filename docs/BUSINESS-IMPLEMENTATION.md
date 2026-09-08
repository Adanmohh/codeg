# Business workspace implementation — autonomous resumption

Owner instruction 2026-09-08: make the best assumptions and get the job done.
Talking is nice to have, not the main interaction. Implementation resumes.
Root orchestrates; product code remains assigned to three Herdr workers.

## Product assumptions

- Visual-first desktop workspace: My work, shared work, decisions and actual
  connected sources. Agent conversation is optional support. A human must be
  able to create, assign, progress and review a task without a chat or an agent.
- One task system across marketing, channels, ads, website, feedback and
  engineering. Domains organize work, not separate task databases/engines.
- Human accountable owner is distinct from assigned human/agent execution,
  creator and reviewer. Engineering runs are optional linked execution records.
- Start with one organization per shared server, with explicit organization IDs
  throughout schema/auth, multiple named human members and agent principals.
  No multi-tenant hosting or offline-conflict sync claim without implementation.
- Roles: owner/admin, manager, member, viewer; role-specific discipline/access
  (business/engineering) must be separate from seniority where needed. Rights
  checked on the backend; choosing a role label in the UI is not authorization.
  Default human members can contribute to authorized shared work; viewers read.
  Admin manages identities; privileged action review is explicit.
- Preserve original operator/engineering access as an administrative boundary.
  New restricted member credentials must never become general CODEG_TOKEN access
  or expose inherited credential/config/terminal routes. Do not weaken legacy auth.
- Shared state is authoritative in the existing Rust server/database, not copied
  SQLite files. Local Tauri can use the same business core as an owner/operator;
  multi-user tests use two independent sessions against the shared service.
  No automatic deployment or account/credential discovery/exposure.
- No new Temporal, Flowable, Frappe or n8n runtime, no new frontend dependencies.
  Borrow exact verified Apache/MIT/current owner-approved patterns and retain
  attribution. Intromail unsafe authorization/queue paths must not be copied.

## Increment A — working shared organization tasks

Deliver a useful end-to-end vertical slice, not a schema-only landing:

1. Individual, revocable member authentication with server-derived principal,
   organization/member status and role/resource permissions. Bootstrap only via
   existing authenticated operator; no unauthenticated public first-user takeover.
   Reuse installed credential/randomness/hash patterns after source verification;
   no invented crypto. Member secret values never appear in logs/reports/screens.
2. Business task CRUD/list/detail, explicit owner and assigned executor, domain,
   status (todo/in_progress/review/done/cancelled), priority, optional due date,
   notes/activity and revision conflict protection. Human-only work is valid.
   Validate every referenced principal and read/write/assign/review action.
   Audit immutable authorship and actor; no caller-spoofed role/identity.
3. Visual task list/board and detail, My work/shared filters, assignment, human
   progress, deliverable/review. Role capabilities reflect actual API response.
   Useful empty/error/permission/conflict states; no fake sales numbers or module
   placeholders. Keep engineering available as supporting capability without
   implying new member credentials grant unrestricted local CLI power.
4. Link a scoped agent contribution using the existing run/executor boundary;
   no duplicate marketing engine. Agent may read assigned authorized context and
   submit progress/deliverable, not approve its own work or change its role.
   Preserve accepted Ops human review/no-resend/receipt rules.
5. Actual two-session shared-state/authorization/conflict checks with synthetic
   principals, plus focused backend tests and frontend behavioral tests, typecheck,
   desktop/server cargo checks and Playwright CLI. Final Design Studio loop and
   refreshed app after integration. No live model/provider action required.

## Increment B — meeting/feedback to shared work

Once A is integrated, add a bounded Fireflies/source adapter grounded in the
reviewed official query contracts: read-only source import, readiness, durable
source IDs/revisions and ingestion claims; candidate tasks with source passages,
owner/date suggestions and accept/edit/link/discard. Reimport does not duplicate
work. Existing email/Hafidh feedback can link to the same business task. Test
synthetically; no assumed provider access or blanket transcript sharing.

## Increment C — business tools

After the common work foundation, stage real marketing/channel/ads/website
capabilities as separate reviewed adapters. Use existing repository context and
verified official APIs/CLIs/MCPs; no new credentials or live actions implied.
Support actual briefs/deliverables/reviews first, then pull/import configured
account data and prepared actions. Unknown platform details are isolated adapter
configuration, not reasons to block the shared product. No fake connected state,
metrics, budgets or publishing. Sentry versus SigNoz remains separate/unselected.

## Parallel ownership and integration

- approvals worker: new business identity/auth/permissions module, migration
  000009, middleware/routes for its endpoints; publish exact principal/interface
  and API contract early in docs/contracts/business-identity.md. Own shared Rust
  registry/router wiring initially. Coordinate task module registration as a
  minimal separate patch from tickets, never overwrite unrelated routes.
- tickets worker: new business task core/entities/migration 000010, endpoints,
  tests and agent task/run binding. Own docs/contracts/business-tasks.md. First
  inspect/reuse prior researched task operations and agree with identity API;
  do not invent a second auth model. Can prepare independent task/model work while
  identity checkpoint is pending, then integrate its published dependency.
- rebrand worker: business-facing visual workspace and member connection/sign-in,
  task board/detail and shared capability-aware API client; own frontend paths
  and docs/contracts/business-ui.md. Preserve paused visual checkpoint and reuse
  its art direction. Begin with API contract/source reading and static composition
  decisions; wire real APIs after checkpoint, no mocked production responses.

Use fresh branches from main; preserve paused report files and existing fixtures.
No worker edits root STATUS/DECISIONS/founding/design brief. Commit/push early,
draft PRs to Adanmohh/codeg main, per-task report deliverables. Cross-review after
handoff, exact-head root acceptance and STATUS update per merge. GPT-6 Astra/max,
docs-first, gh api immutable refs, no AGPL source and no product code by root.
