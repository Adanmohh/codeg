Step 2 intake and GitHub App implementation contracts — 2026-09-07.

Completed contract research for FOUNDING pieces 4, 5 and 5b. Implementation can
start against local fixtures, but live P1 intake/filing is not configured or
verified. The concrete gaps are read access to in-app feedback and diagnostics,
missing report evidence, an isolated MCP environment, GitHub App configuration,
and integration of the accepted approval service with trusted reviewer identity.
This branch contains a report only. No product code or dependencies changed.

The existing worktree was clean before `git fetch origin` and
`git switch -c docs/step2-intake-contracts origin/main` (both exit 0).
Branch base: `5bd0b1e1985fc2e0e703e500b6b06864b7b15f8c`.
Read FOUNDING.md, ORCHESTRATOR.md, STATUS.md, DECISIONS.md and AGENTS.md
completely from that base. The owner's integration amendment supersedes the
original GitHub MCP choice: use intromail's GitHub App client.

**Verified source authority**

| Source | Immutable revision | Findings |
| --- | --- | --- |
| IntroInnovation/intromail | `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | Read `backend/app/services/github/{client,actions}.py`, `backend/app/routers/github.py`, `backend/tests/test_github_pack.py`, `docs/GITHUB-INTEGRATION.md`, `backend/requirements.txt`; inspected GitHub fields in `backend/app/config.py` and `GithubInstallation` / `GithubRepo` in `backend/app/models.py`. Installation JWT/token client exists; create-issue helper/action does not. |
| modelcontextprotocol/python-sdk v2.0.1 | `8b191a433634d64b1306d7d51be8b16e14cc0893` | Tag points directly to this commit. Read `pyproject.toml`, `LICENSE`, `src/mcp/server/fastmcp.py`, `src/mcp/server/__init__.py`, relevant `src/mcp/server/mcpserver/server.py` methods, and `examples/mcpserver/{readme-quickstart,simple_echo,weather_structured}.py`. The actual class is `MCPServer`; the old FastMCP import deliberately raises `ModuleNotFoundError`. Preserve v2.0.1. |
| Local Hafidh | `a83794708193a18611c8e6eb8e88637b8136e471` | Read root AGENTS.md and backend/mobile CLAUDE.md read-only. Feedback/TestFlight models, schemas, routers, triage, ASC client, auth dependencies and client configuration names inspected. Existing unrelated dirty documentation left untouched. |
| xintaofei/codeg v0.30.4 | `6f6bd648b206412644842a98d9ffeebf57292bed` | Existing `src-tauri/src/db/entities/work_task_template.rs`, migration `m20260801_000003_work_task_template.rs`, model/service/command template surfaces supply reusable composer snapshots, not evidence enforcement. |
| github/docs | `831337b0fed60b90a72e2711a41dfcad72b5f288` | Official App JWT/install-token documentation and `src/github-apps/data/fpt-2022-11-28/server-to-server-permissions.json` confirm installation auth and Issues write for create-issue. |
| github/rest-api-description | `3cef12e8a02d612ad032473d4fb87266f2befeae` | Read relevant operations from `descriptions/api.github.com/api.github.com.2022-11-28.json`. Preserve intromail's API version `2022-11-28`; the separate newer API description does not authorize a version upgrade. |

All remote research used `gh api` after installed CLI help. No browser/web
search, alternate source version or GitHub MCP dependency was used.

The SDK tag API returned object type `commit`, SHA above. Its project is named
`mcp`, Python >=3.10, with a dynamic version derived from the Git tag, not a
literal `version = "2.0.1"` in pyproject. The correct import is
`from mcp.server.mcpserver import MCPServer`; `MCPServer.run()` defaults to
stdio, and `tool()` derives structured output from the return annotation.
The scaffold must use those v2 APIs. A contents request treating `fastmcp` as a
directory returned 404; the discovered `fastmcp.py` is an intentional failure
stub. This is a corrected path/API assumption, not a missing tag or permission
to downgrade. Relevant migration sections were read; the entire long migration
guide was not reviewed.

Official contract references, all read through `gh api` at the recorded refs:

- [GitHub App JWT](https://github.com/github/docs/blob/831337b0fed60b90a72e2711a41dfcad72b5f288/content/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-json-web-token-jwt-for-a-github-app.md),
  [installation token narrowing](https://github.com/github/docs/blob/831337b0fed60b90a72e2711a41dfcad72b5f288/data/reusables/apps/generate-installation-access-token.md),
  [installation permission table](https://github.com/github/docs/blob/831337b0fed60b90a72e2711a41dfcad72b5f288/src/github-apps/data/fpt-2022-11-28/server-to-server-permissions.json).
- [GitHub REST description, API 2022-11-28](https://github.com/github/rest-api-description/blob/3cef12e8a02d612ad032473d4fb87266f2befeae/descriptions/api.github.com/api.github.com.2022-11-28.json):
  `issues/create`, `apps/create-installation-access-token`, repository installation
  lookup, issue listing and referenced issue/token/permission schemas.
- [GitHub retry/pagination guidance](https://github.com/github/docs/blob/831337b0fed60b90a72e2711a41dfcad72b5f288/content/rest/using-the-rest-api/best-practices-for-using-the-rest-api.md),
  [SDK v2 quickstart](https://github.com/modelcontextprotocol/python-sdk/blob/8b191a433634d64b1306d7d51be8b16e14cc0893/examples/mcpserver/readme-quickstart.py),
  [SDK structured-output example](https://github.com/modelcontextprotocol/python-sdk/blob/8b191a433634d64b1306d7d51be8b16e14cc0893/examples/mcpserver/weather_structured.py).

**Source-to-implementation mapping**

Every destination below is a proposal for a later implementation, not a claim
that the file/API already exists. `H/` means the read-only source root
`/Users/mohamedadan/projects/Hafidh`, whose configured origin is
`IntroInnovation/Hafidh`. All named Hafidh source files were clean relative to
the recorded HEAD; the untracked root AGENTS.md is local workflow guidance.

| Exact source files and revision | Proposed destination or existing seam | Smallest adaptation |
| --- | --- | --- |
| SDK SHA above: `examples/mcpserver/readme-quickstart.py`, `weather_structured.py`; API authority `src/mcp/server/mcpserver/server.py` | New `integrations/hafidh-intake/src/hafidh_intake/server.py` and `schemas.py` | Typed read tools over the existing backend. Local stdio process, no public HTTP listener, OAuth server or new UI system. |
| `H/backend/app/modules/feedback/{models.py,feedback_schemas.py,feedback_router.py}` at Hafidh SHA | New intake `client.py` / `schemas.py` in the same package | Transcribe the response shape; explicitly unavailable until a protected read route exists. Do not port the email-sending submission handler. |
| `H/backend/app/modules/testflight/{models.py,schemas.py,admin_router.py,asc_client.py}` and `H/backend/app/common/utils/operation_result.py` at Hafidh SHA | Same intake client/schema files | Read already-synced admin records, unwrap `OperationResult`, retain stable source IDs and source timestamps. ASC client is evidence of field provenance, not a second poller to run in the Desk. |
| `H/backend/app/modules/testflight/triage.py`, `models.py` (`Severity`), `tests/test_triage.py` at Hafidh SHA | New intake `triage.py` and focused tests | Port the pure classifier; remove the ORM dependency by mapping its enum into the typed intake DTO. Preserve pattern order, nine tags, fallback `other` and human-edit semantics. |
| `H/backend/app/common/auth/dependencies.py`, `web/src/lib/{api,testflight}.ts`, `backend/app/common/config.py` at Hafidh SHA | Intake client credential boundary | Preserve Bearer auth and DB-backed admin check; no fake `isAdmin` claim, direct database access or OTP/email login automation. |
| `H/backend/app/modules/recitation_sessions/{models,schemas,router}.py`, `H/mobile/lib/services/{feedback_service,recall_session_recorder,listen_tap_diagnostics}.dart`, `H/backend/app/modules/reciters/{router,schemas,models}.py` at Hafidh SHA | New `src-tauri/src/models/ops_intake.rs` evidence DTO/validator and intake projection | Field provenance/reference only. No mobile recorder, bucket uploader, audio download or reciter service is ported. |
| intromail SHA: `backend/app/services/github/client.py` | New `src-tauri/src/integrations/github.rs` | Port installation auth/cache/REST shape; add one create-issue call. Existing direct Reqwest 0.12.28 supplies HTTP; RS256 signing must use a documented package, never new cryptography. See dependency gap below. |
| intromail SHA: `backend/app/services/github/actions.py` (`Comment`, resource/install lookup), `backend/app/models.py` (`GithubInstallation`, `GithubRepo`), `backend/app/routers/github.py` | Same GitHub module's `github.create_issue` action and minimal repository binding | Retain `domain=github`, repository resource and destructive floor. Add explicit enabled-repo/write checks: source `_installation_id()` only checks row presence; the source enabled flag chiefly controls webhook ingestion. No webhook/connect UI is copied. |
| intromail SHA: `backend/tests/test_github_pack.py` | New GitHub adapter focused tests | Borrow mocked-client/destructive-floor/admin-boundary cases. This source suite does not verify a create-issue call or the complete token-cache contract; those tests are new glue tests. |
| codeg pin: `src-tauri/src/db/entities/work_task_template.rs`, `src-tauri/src/db/migration/m20260801_000003_work_task_template.rs`, `src-tauri/src/models/work_task.rs`, `src-tauri/src/db/service/work_task_service.rs`; current `src-tauri/src/commands/work_task.rs` | Existing template save/list and task composer; new `ops_intake` validator only | Save a named Hafidh bug-report prompt through the existing composer snapshot. Do not put evidence authorization in editable prompt text, invent schema support in `config`, or misuse `config_values` as form fields. |
| Pending approvals PR SHA `e1585541014e2bd55ee882335b6ee0ff032d2af1`: `src-tauri/src/db/service/ops_approvals/{mod,gating}.rs` | Reuse accepted `Action`, `propose`, `approve`, `Review`, `AuthorizedAction` service; new `src-tauri/src/commands/ops_intake.rs` and matching `web/handlers/ops_intake.rs` | Trusted registry/authentication supplies identities. `_core` functions serve both runtimes. Register only the bounded new command/handler surfaces after approvals integration is accepted; do not copy another gate. |

The four codeg template/model/service files checked against local `v0.30.4`
were unchanged (exit 0). Templates contain only id/name/title/config/timestamps;
save validates nonempty name/title and an object config, then upserts by name.
They cannot enforce the filing gate. Existing HTTP routes are POST
`/api/work_task_template_list`, `/api/work_task_template_save` and
`/api/work_task_template_delete`. Keep that existing storage/composer contract.

**Licensing handoff**

No source code was imported by this report. Preserve root LICENSE and all
existing/future NOTICE entries. When implementation copies or ports SDK
examples, append the original `Copyright (c) 2024 Anthropic, PBC` and complete
MIT permission/disclaimer from the pinned SDK LICENSE, with exact source files,
SHA and destinations. Retain codeg's Apache-2.0 attribution for template reuse.
Intromail's recursive immutable tree has no LICENSE/LICENCE/NOTICE/COPYING;
its authority here is the owner's “ours” authorization in FOUNDING, not an MIT
claim. Attribute intromail and owner-authorized Hafidh ports and their exact
files/SHAs in NOTICE when they land. Official GitHub docs are contract evidence,
not another code source. No GitHub MCP Server, AGPL, Chatwoot enterprise or Kun
source is part of this plan.

**Verified behavior and gaps**

- In-app feedback exposes `POST /api/v1/feedback`, which persists and sends an
  email. There is no list/detail read route in that module. Never use submission
  as an intake poll or fixture setup against a live backend.
- TestFlight exposes authenticated admin `GET /api/v1/admin/testflight`, with
  offset/limit pagination, filters and an `OperationResult` wrapper. Its total
  counts describe the whole inbox, not the filtered page. No GET-by-ULID detail
  route exists. Screenshot and sync-status reads exist; PATCH triage and POST
  sync are outside read-only intake.
- `buildVersion` comes from the ASC included build's `attributes.version`.
  In-app Flutter feedback writes `appVersion` as `version+buildNumber`. Preserve
  source values and provenance; do not treat every app-version string as a build.
- Neither feedback model has structured screen, reciter or diagnostic-log
  fields. Recitation sessions have an optional `diagnosticPath`, but the existing
  admin listing returns metadata, not log bytes, and has no feedback relationship.
  A session must be explicitly linked to the report; do not join by email/time.
- The recorder's JSONL header has `surface`, `appVersion`, `mode` and
  `startedAt`; no reciter field in the inspected header. The listen-tap debug
  diagnostic explicitly does not run in TestFlight builds. Missing evidence
  must remain missing; no N/A or guessed reciter bypass.
- `triage.py` supplies nine named tags plus fallback `other`, and
  `high|medium|low` severity. These are editable guesses. Keep both seeded and
  human-confirmed values; do not overwrite upstream operator triage on rescan.
- Intromail authenticates with RS256 App JWT (`iat` minus 60 seconds, `exp`
  plus nine minutes), exchanges it for an installation token, caches until
  60 seconds before expiry, and uses 30-second HTTP timeouts. Its token exchange
  requests all granted installation scope. Narrow the filing token to the
  configured repository ID and Issues write; cache by App, installation,
  repository and permission set rather than installation alone.
- Official create-issue is `POST /repos/{owner}/{repo}/issues`, returning 201
  with issue identity/URL. Minimal payload is string `title`, markdown `body`,
  and approved existing label names. GitHub requires only title; the four-field
  build/screen/reciter/log gate belongs in trusted Desk glue and runs before
  proposal creation and again on the approved snapshot before dispatch.
- Reuse Step 1 approval/destructive-floor/audit behavior for outward filing.
  The branch base does not yet contain approvals. PR #3 was open at
  `e1585541014e2bd55ee882335b6ee0ff032d2af1` during research; accepted integration
  and trusted reviewer authentication are prerequisites, not assumed present.
- The existing generic Desk server bearer proves access, not a named human
  reviewer. Never expose approval/execution to the intake MCP tool set or accept
  an agent-supplied actor/approval flag.

**Minimal read endpoints and intake data**

These are current Hafidh endpoints, confirmed from their routers and
`backend/app/main.py`'s `/api/v1` mounts. This research did not call any of them.

| Method/path | Read contract and boundary |
| --- | --- |
| `GET /api/v1/admin/testflight` | Admin Bearer; optional `status`, `severity`, `tag`, `q`, `limit` (1–500, default 100), `offset` (>=0). Returns `OperationResult<TestFlightFeedbackListResponse>`. `value.items` contains details/screenshots; `total/open/resolved/highSeverityOpen/tagCounts` are whole-inbox counts. No `since`/cursor/ETag or stable secondary sort is implemented. |
| `GET /api/v1/admin/testflight/screenshots/{ulid}` | Same admin requirement; returns stored image bytes and private cache policy, 404 if absent. Only request IDs found on the selected feedback record. A screenshot is neither a screen-name assertion nor a diagnostic log. |
| `GET /api/v1/admin/testflight/sync/status` | `value={lastRun,syncEnabled}`. Preserve never-run, failed and stale states; do not equate an empty page with successful recent sync. Sanitize upstream error text and omit `triggeredBy` identity from agent output. |
| `GET /api/v1/recitation-sessions` | Admin; `since` applies to `createdAt`, not session start; `limit` 1–500. Returns `OperationResult` with a list of session metadata, optional `diagnosticPath`, no log bytes. No paging cursor, user filter, ULID filter or feedback association. Metadata lookup cannot prove a log is retrievable. |
| `GET /api/v1/reciters`, `GET /api/v1/reciters/{reciter_id}` | Existing catalog; list has `skip`, `limit` <=100, `featured`, `style`, `search`; detail accepts numeric ID or ULID. Returns id/ulid/slug/name, useful for explicit reciter selection. A current catalog/default does not establish the tester's historical selection. Check `succeeded` even when HTTP status is 200. |

Proposed read additions for the **Hafidh owner in a separately assigned
worktree**, not edits authorized in this report: `GET /api/v1/admin/feedback`
and `GET /api/v1/admin/feedback/{ulid}` returning the existing `FeedbackDetail`
shape under `CurrentAdminUser`; `GET /api/v1/admin/testflight/{ulid}` for reliable
current-record review; and `GET /api/v1/recitation-sessions/{ulid}/diagnostic`
under that same admin check, resolving the stored object internally rather
than accepting a caller URL. Detail responses need an explicit revision
(`lastModifiedAt` plus a snapshot digest); diagnostic reads must validate the
live, undeleted session and distinguish absent/expired from empty bytes.
For lists, use bounded stable pagination with an ULID tiebreaker and explicit
next-page token. These are proposed contracts, absent from the inspected source.
Manual, locally imported redacted evidence can support fixture/UI work while
those read routes are unavailable; it is not a claim of working live intake.

Proposed MCP tool set, using the pinned SDK's stdio scaffold:

| Tool | Input | Output |
| --- | --- | --- |
| `hafidh_feedback_list` | `source: in_app|testflight`, optional status/severity/tag/query, `limit` 1–100 (default 100), opaque `cursor` | `{items: IntakeRecordV1[], next_cursor, scan_complete, fetched_at, source_health}`. In-app currently returns `source_unavailable`, never an invented empty success. |
| `hafidh_feedback_get` | `source_ref` from an authorized list/import | `IntakeRecordV1`. For existing TestFlight API, resolve from the bounded list scan/cache and disclose its fetch time/revision; do not pretend a GET-by-ULID route exists. A failed revalidation cannot authorize filing from a stale cache. |
| `hafidh_intake_status` | none | Configuration-presence booleans, last successful scan/sync time, stale/failed/unconfigured state, sanitized error category; no credentials or raw sync errors. |

All three tool results use a typed `{status: ok|unavailable|error, value?,
error?: {code,message}}` envelope around the listed output. Stable error codes
include `not_configured`, `source_unavailable`, `access_denied`, `stale_source`
and `upstream_unavailable`; no raw exception or HTTP response is returned.
An unavailable source has no successful empty `items` payload.

The source origin and credentials come from trusted configuration, never tool
arguments. No arbitrary URL, SQL, filesystem read, sync, PATCH, submit-feedback,
approve, execute, email or GitHub write tool is registered in this server.
Screenshots/logs are operator evidence attachments; do not add an unrestricted
attachment-fetch tool. Pi adapter discovery/approval integration belongs to
piece 6 and must be verified separately; launching this server alone does not
prove the agent can discover it.

`IntakeRecordV1` is a proposed strict DTO (snake_case on the new interface):

| Fields | Source/mapping |
| --- | --- |
| `schema_version=1`, `source_ref={product_id,source,ulid,external_id?}`, `source_revision`, `fetched_at` | Trusted product binding; in-app ULID; TestFlight ULID plus `ascSubmissionId`. Revision binds last-modified timestamp and hash of the permitted source snapshot. Namespace by product/source; never deduplicate by title, email or timestamp. |
| `title`, `description`, `feedback_type?`, `source_status` | In-app title/description/type/status; TestFlight comment becomes description and title is an explicitly labelled draft, not a fabricated upstream field. Status retains `pending|in_progress|resolved|closed`. |
| `submitted_at?`, `source_updated_at`, `device?`, `os_version?`, `app_version?`, `build_number?`, `platform?`, `locale?` | In-app createdAt/lastModifiedAt/deviceInfo/appVersion; TestFlight submittedAt/lastModifiedAt/deviceModel/osVersion/buildVersion/appPlatform/locale. Keep raw version alongside any verified split into build and marketing version. |
| `screenshots[]={ulid,content_type,width?,height?}` | TestFlight screenshot summaries. No Apple signed URLs or bytes in the default tool response. |
| `triage={seeded_tags,seeded_severity,source_tags?,source_severity?,confirmed_tags?,confirmed_severity?}` | Pure classifier suggestions plus current upstream operator values; confirmation requires a human event. Existing API has lastModifiedByName but no explicit per-field confirmation flag: never infer confirmation merely because severity is populated. |
| `evidence_candidates`, `missing_required[]` | Candidate field values and provenance only. Missing build/screen/reciter/log is explicit. Candidate assertions cannot satisfy the trusted filing validator by setting a boolean. |

Keep tester identity/contact details and operator notes private in the trusted
intake/review context; default agent and issue projections omit email/name,
userId, raw bucket paths, authentication data and unreviewed notes. Source text
is untrusted content, never instructions to choose a repo, enable tools or send.

Existing TestFlight offset paging is not a durable synchronization cursor.
Scan fixed filters/page size until a short/empty page, deduplicate stable IDs,
bound work per call and return `scan_complete=false` when the bound is hit.
Do not stop using whole-inbox `total` for a filtered scan. Repeat scans with
overlap; inserts/edits/tied timestamps can move rows across offsets, so no
lossless incremental-sync claim is possible until the source gets stable
pagination. Re-read operator triage as source state, but never overwrite a
Desk human confirmation with freshly computed heuristic guesses.

**Evidence gate and local issue command**

Proposed `IssueDraftV1`: trusted product/repository binding, source reference and
revision, template version, title, summary/reproduction text, device/app-version
metadata, triage suggestions, and four evidence-backed fields below. All
strings are trimmed/nonempty for eligibility; values such as `unknown`, `N/A`,
`TBD` and dummy IDs do not satisfy the gate. Draft title limit 200 characters,
rendered body limit 16 KiB UTF-8, and included sanitized log excerpt limit
8 KiB are **local contract limits**, not asserted GitHub API limits.

| Required field | Proof needed before creating a proposal and before dispatch |
| --- | --- |
| `build_number` | Exact build identifier, from ASC buildVersion, a valid mobile `version+buildNumber` value, or explicit human-confirmed report evidence. Preserve original source and refuse ambiguous parsing. Marketing version alone fails. |
| `screen` | Concrete screen/surface and evidence reference. A recorder `surface` or a human-confirmed screenshot/report description can supply it. A screenshot's existence alone fails. |
| `reciter` | The actual selected reciter at the time of the problem, with source reference and explicit identity/name; optionally match the existing catalog ID/ULID/slug. No default, guessed tester preference or unrelated session. If it cannot be established, the report remains unfileable under the founding rule. |
| `log` | An accessible, nonempty diagnostic artifact explicitly bound to the report/session, with immutable local artifact reference, content SHA-256, capture time when available, and a reviewed sanitized excerpt. A URI string, screenshot, empty file or session ID alone fails. Recompute the artifact digest and check access/binding before dispatch. |

Recitation session ULID is optional supporting evidence; require it when using
a bucket session's diagnostic. Do not guess an association from tester email or
nearby timestamps. No exception to the four fields is introduced for UI/crash
reports lacking diagnostics or reciter information.

The artifact resolver belongs to trusted Desk glue and accepts only a previously
attached, allowed artifact ID; no user-supplied filesystem path or remote URL
is fetched during validation. Keep raw evidence private. Include the approved
sanitized excerpt and evidence digest/reference in the issue body so the issue
is useful without publishing bucket tokens, signed URLs or tester contact data.
Expired/deleted evidence blocks a new dispatch. If redaction changes the
outgoing bytes, render and review them again before approval.

Proposed Desk commands, following the existing `/api/<command>` convention:

| Command / proposed HTTP surface | Contract |
| --- | --- |
| `ops_issue_validate` / `POST /api/ops_issue_validate` | Complete draft in; `{eligible, missing_required, field_errors, rendered_preview?, payload_digest?}` out. No side effects or token exchange. This is advisory UI validation; trusted action validation repeats it. |
| `ops_issue_propose` / `POST /api/ops_issue_propose` | `{task_id,run_seq,draft}` from an authorized task/agent context. Caller identity and product/repository mapping are server-derived. Validate evidence and permissions, freeze full render, then Step 1 `propose`. Return pending proposal identity and exact preview; incomplete input returns validation errors and no external action. |
| Human review integration, **no agent MCP tool** | `{proposal_id,task_id,run_seq,expected_payload,approved_payload}` submitted by the authenticated human UI. Reuse Step 1 `Review`/`approve`; identities and registry action come from trusted code. Return filing status/receipt, never an installation token or serialized execution capability. Denial reuses Step 1 `deny`. |
| `ops_issue_filing_status` / `POST /api/ops_issue_filing_status` | Authorized source/proposal reference in; pending/created/failed/unknown outcome, issue ID/number/URL and actual labels out. No resend side effect. |

Proposed HTTP status mapping: 202 for a pending proposal, 401 for missing caller
authentication, 403 for denied scope, 422 for incomplete/invalid evidence, 409
for stale review/run or a conflicting filing, and 503 for unconfigured or
unavailable source access. Tauri returns the same stable error codes through
its normal command result. Filing status keeps `unknown` distinct from `failed`;
neither a UI request timeout nor approval success becomes a created receipt.

`github.create_issue` is `domain="github"`, resource=`owner/repo`, and always
`is_destructive=true`, using the same outward-action floor as intromail Comment.
`Action.validate` checks the complete strict schema and renderer consistency
without rewriting payload. `check_permission` uses the same transaction to
check product/task scope, enabled repository binding, evidence ownership and
policy; deny remains absolute. The frozen payload includes exact
`{title,body,labels}`, repository ID/full name, source revision, evidence digests
and template version. The renderer puts build/screen/reciter/log sections and
confirmed triage in the preview. Human text/label edits produce a complete new
preview/payload before approval; no client-supplied `approved=true` or patched
draft pointer is accepted.

Agent policy is separate from App permissions: read scope permits intake/status
only; propose scope creates pending review; act_low_risk still requires review
for this destructive action. Enforce these bounds in the trusted action/caller
checks, rather than assuming the generic mode fallback supplies repository
authorization. Unknown/disabled bindings fail closed. Resolve evidence and
source freshness before the approval transaction; inside `ActionContext` read
the trusted local snapshot/binding from that transaction, with no network I/O
or sends in callbacks. Recheck the frozen evidence immediately before dispatch
without substituting new outgoing content.

The inspected Step 1 API yields a non-Clone/non-Serialize `AuthorizedAction`
only after transaction commit. Dispatch its consumed `into_payload()` once;
never reload a mutable draft or reconstruct authorization from a resolved row.
Its contract requires a new proposal/human review after a crash, rather than
persisted capability replay. Re-read its final accepted API before implementing:
the report's PR revision is context, not an accepted integration baseline.

**GitHub HTTP contract, auth and failure handling**

1. Trusted configuration maps the intended Hafidh repository (the checked-out
   source is `IntroInnovation/Hafidh`) to a verified installation and repository
   ID. A source remote is not evidence of an installed App or permission. Check
   enabled binding before writes. Do not accept `installation_id`, API host,
   authorization headers or arbitrary method/path from an agent.
2. Borrow intromail's App JWT exchange: RS256, `iat=now-60`, `exp=now+540`,
   `iss=App ID`, then `POST /app/installations/{installation_id}/access_tokens`
   with `repository_ids:[configured_repo_id]` and
   `permissions:{issues:"write",metadata:"read"}`. The official table marks
   create-issue as Issues write with no additional permission requirement.
   Contents, pull requests, Actions, deployments, organization administration
   and account-wide PAT scopes are unnecessary for this operation.
3. Cache only in trusted backend memory, keyed by App identity/key generation,
   installation, repo and requested permissions. Refresh 60 seconds before
   `expires_at`. Unlike the source's fallback to another hour on parse failure,
   reject an invalid expiry. Tokens are opaque strings; do not assume a
   40-character format. Missing App credentials fails `not_configured`; never
   fall back to gh's research credential or the inherited personal Git token.
4. After committed human authorization, `POST /repos/{owner}/{repo}/issues`
   with installation Bearer, `Accept: application/vnd.github+json`,
   `X-GitHub-Api-Version: 2022-11-28` and the exact approved `title/body/labels`.
   Omit assignees, milestone, issue type/fields and sub-issues. Keep source's
   30-second timeout. No comment/label follow-up write is implicit in success.
5. Require 201 with valid `id`, `number`, `html_url`; persist the receipt and
   returned labels. Use fields, not URL parsing. GitHub documents that labels
   can be silently omitted without sufficient repository access: show an
   actual-label mismatch and retain the created issue, never retry issue
   creation to repair labels. Preflight existing label names; label creation
   or a later label update needs its own approved action.

App tokens cannot exceed the installation's granted repository/permission set.
The scope restriction is additional to the local enabled-repo and proposal
checks. A renamed/transferred repository or redirect needs identity and
allowlist revalidation; never forward credentials to another host or silently
change the human-approved destination. No App registration, install callback,
webhook endpoint or public inbound port is required to design this outbound
internal adapter. Those actions were not performed.

Proposed minimal durable filing receipt/attempt storage is glue, not a new job
engine: unique `(product_id,source_kind,source_ulid,repository_id)` logical filing,
proposal/task/run references, approved payload digest, attempt state/time and
GitHub issue receipt. Use 64-bit database IDs (string-safe wire representation
where needed). Reuse existing SQLite transaction/CAS and append-only audit
patterns. Record proposal approval separately from transport outcome; approval
is not evidence that an issue was created. A new migration/entity for this
receipt is not currently present; reserve its name with the orchestrator at
implementation time to avoid other workers' migrations.

GitHub's inspected create endpoint declares no idempotency-key parameter or
conditional-create mechanism. A stable source marker in the approved body plus
the unique local filing record helps reconciliation; it is not an exactly-once
guarantee across machines. Concurrent approval/dispatch of the same source must
win one local reservation only. Before any retry of a response-lost attempt,
read repository issues with `state=all`, follow same-origin Link pagination,
ignore objects with `pull_request`, and match the exact marker/receipt. If a
match exists, reconcile it without POST. If none/multiple can be established,
keep outcome `unknown` for human resolution; never blindly re-POST. A new
attempt after a definitive non-creation failure needs a new proposal/review.

Distinguish missing/expired auth (401), denied/rate-limited (403/429), unavailable
or inaccessible repo (404), disabled issues (410), payload validation (400/422),
and service/network failure (503/timeout). A 404 can hide insufficient private
repository access. Respect `Retry-After` or exhausted `X-RateLimit-Reset`; show
retry eligibility, not a background unapproved retry. Any ambiguous write,
including a malformed/lost success response or crash after POST, stays unknown
until read reconciliation. Raw HTTP bodies/headers are not audit/error text.
Only whitelisted status/request-ID/timing/issue-receipt fields leave the client.

**Configuration evidence and limits**

Only names/presence were emitted. No keys, tokens, cookies, database URLs or
credential-store values were printed or copied.

- Intromail declares `github_app_id`, `github_app_private_key`,
  `github_app_slug`, `github_webhook_secret` (environment names uppercase).
  Installation/repository mappings are database rows, not an implicit PAT.
- Hafidh declares `asc_key_id`, `asc_issuer_id`, `asc_private_key`, `asc_app_id`,
  `testflight_sync_interval_minutes`; sync requires all four ASC values.
  Keep these on Hafidh; the Desk reads already-ingested records.
- Hafidh also declares `hf_bucket_token`, `huggingface_token`,
  `recitation_bucket_id`, `recitation_retention_days`, and `admin_emails`.
  The bucket credential resolver prefers the dedicated token, then the existing
  model-repository token. Do not export either credential to agents.
- Hafidh web uses `VITE_API_URL` and local-storage key `hafidh_access_token`;
  admin access is rechecked through `Users.isAdmin` in the database. No
  purpose-built read-only machine token was found in the inspected auth code.
- `Hafidh/backend/.env`, `.env.local`, and root `.env` were absent. The web
  production config has a nonempty `VITE_API_URL`; its value was not emitted.
  The current worker environment has no names beginning `GITHUB_APP_`,
  `HAFIDH_`, `ASC_`, `TESTFLIGHT_` or `CODEG_`. This does not establish deployed
  credential absence. No live installation, repository permission, admin login,
  database, bucket or ASC API was queried.
- Installed Hafidh metadata: FastAPI 0.115.13, Pydantic 2.11.7, SQLAlchemy
  2.0.41, HTTPX 0.28.1, PyJWT 2.10.1, cryptography 49.0.0; no MCP distribution
  metadata was found there. SDK v2.0.1 requires Pydantic >=2.12, HTTPX2 >=2.5.0 and matching
  `mcp-types`. Plan an isolated intake environment; do not upgrade Hafidh's
  environment or repurpose the RAG environment.

Further implementation prerequisites, without changing scope or configuration
in this report:

- Desk currently has a generic `codeg` keyring service with
  `github-token:{account_id}` and `chat-channel:{channel_id}` slots. Those are
  not GitHub App credentials. Add dedicated App/intake credential slots through
  the existing storage mechanism; never overwrite the personal Git account.
  Server mode uses a restricted file store under `CODEG_DATA_DIR`, not the
  desktop keychain. Native/store fallback must be tested independently.
- There is no verified App ID/private-key binding, installation ID, enabled
  repository ID/full-name binding or label inventory in this Desk checkout.
  `github_app_slug` and `github_webhook_secret` are source configuration names,
  not required for outbound-only filing. App credential availability remains
  unknown outside the explicitly inspected files/environment; no keychain
  contents, Intromail production environment or installation inventory was read.
- A configured Hafidh API origin and existing approved admin client session are
  needed for live reads. The current admin credential also permits mutation;
  calling it a read-only token would be false. Keep it inside the trusted
  allowlisted adapter and out of MCP config/prompt/child-agent environments.
  A backend-issued narrower credential is a future auth improvement, not an
  existing capability. An expired login fails closed; do not initiate OTP email.
- Cargo.lock has Reqwest 0.12.28 for the direct `0.12` dependency, SeaORM
  1.1.19 and keyring 3.6.3, but no `jsonwebtoken` package and no direct JWT
  signing abstraction in the inspected manifest/client seam. Intromail relies
  on PyJWT/cryptography. A native port must resolve a pinned established JWT
  signer with local docs before changing manifests; transitive OpenSSL/RSA/ring
  entries alone are not a supported JWT API. This report neither adds a
  dependency nor proposes handwritten signing. The HTTP/action contracts above
  remain usable while that routine port detail is resolved.
- Accepted approval integration and a trusted human review route remain required.
  `CODEG_TOKEN` by itself cannot distinguish an agent from a reviewer. Keep the
  review command unavailable to agent-facing adapters; derive a local human
  principal at the UI boundary and validate ownership. Do not claim headless
  review authorization is solved by the existing bearer middleware.
- In-app list/detail, diagnostic byte access, lossless pagination and explicit
  report/session linkage are genuine source gaps. Do not copy the email ticket
  ingester to simulate feedback ingestion: its current inbox service explicitly
  creates `Channel::Email` and its `Scope` is routing context, not authorization.
  Missing evidence is a product-visible incomplete state until supplied; this
  report requests no weakening of the founding gate.

**Grounding and hooks**

Applied the code-context skill with the existing rag-skills `.venv/bin/python`
and `HF_HUB_OFFLINE=1`. Guide exited 0; the relevant general rule is
“This is not the framework you remember — heed deprecations and breaking
changes.” The owner's local-first/gh-api rule overrides the generic context7
suggestion. Docs lookup exited 3: no `data/code/rebrand.db`; no corpus coverage
for these APIs is claimed and no index/dependency was installed.

The live audit contains this session
`01a07c1c-cf2e-73e1-bbe3-e758c8363042` and this worktree:
PostToolUse at line 1201/time `1788811866`, PreToolUse at line 1216/time
`1788811932`, tool `Bash`, hook exit 0 for both, in
`/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`. Earlier records
1092–1095 also belong to this run. Hooks remained enabled and emitted their
docs-first reminders. No block was disabled or bypassed.

**Local test contract for the implementation worker**

These are required future tests, not tests executed by this report. All fixtures
must be invented/sanitized; no real tester text, images, credentials, live issues,
emails, App installs or deployments. Mock GitHub/Hafidh at the HTTP boundary and
fail any unmocked outbound call. Use worktree-owned caches, exports and Cargo
target output; do not invoke Hafidh's live application lifespan/scheduler.

| Focus | Meaningful pass condition |
| --- | --- |
| MCP v2 scaffold | Use the pinned `mcp.client.Client(server)` pattern from `weather_structured.py` to list/call tools and verify structured schemas. Also start the actual stdio process for discovery/call/EOF shutdown. Assert only the three read tools exist, no private data in results/stdout, missing config yields explicit unavailable status, and test the protocol mode used by piece 6. No claim that v1 FastMCP imports work. |
| Intake reads | Fixture pages with null/missing fields, duplicate source IDs, tied timestamps, moved pages, filtered results smaller than global totals, final short page and request bounds. Wrapper `succeeded=false`, 401/403, timeout and failed sync are not successful empty intake. Confirm source identity/updated triage preservation; no POST/PATCH issued. |
| Provenance/gate | Table-driven omission of each of build/screen/reciter/log; null/blank/placeholders; marketing-version-only; incorrect session/product; unrelated screenshot; debug-only log unavailable; empty/deleted/changed artifact; malicious path/URL; credentials in log. Every case blocks proposal/dispatch and performs zero token exchanges and issue POSTs. A real four-field fixture passes. |
| Review | Agent cannot choose actor/registry/install; destructive action queues even under standing allow/act_low_risk; deny wins; edited repository/evidence/body/labels revalidated; stale card/source/run and two simultaneous reviewers fail safely. Consume only committed authorization. Re-run the accepted overlapping ACP/Ops wait regressions. |
| App client | Locally generated test RSA key verifies RS256/iat/exp/iss; fake clock tests skew/expiry/malformed expiry/rotation and cache isolation across repos/installations/Apps. Mock token exchange asserts narrowed repository_ids and permissions; missing configuration has no PAT fallback. Header/method/path/body equality verified. No production key read. |
| Issue dispatch | 201 persists id/number/URL/actual labels; silently dropped labels remain a created issue needing follow-up. 400/401/403/404/410/422/429/503 and network errors classified without response/token leakage. Two calls for one source produce one POST; crash/timeout after POST records unknown; matching read receipt reconciles without resend; unresolved outcome never retries automatically. |
| Persistence/audit | Own temporary SQLite: reservation uniqueness, rollback on failed audit, task/run/source binding, approved payload digest, separate approval/transport outcomes, migration up/down preserving existing approvals/tickets. Headless and desktop storage fixtures never touch the real keyring/token file. |
| Browser flow | When UI exists, use Playwright CLI (after its skill/help) with local synthetic sources: incomplete evidence visibly blocks filing; complete evidence renders exact destination/body/labels; human edit/deny/approve and stale/unknown/misconfigured states; keyboard and narrow desktop/mobile layout. Capture final screenshots. Run Design Studio audit/fix/recheck on that implemented flow. This report adds no browser surface to test. |

Hafidh reference tests read: TestFlight `tests/test_asc_client.py` uses injected
HTTPX MockTransport, including missing relationships, paging, error categories
and no auth header on signed screenshot downloads; `tests/test_admin_router.py`
uses dependency overrides for admin boundaries; `tests/test_sync_service.py`
preserves human triage on resync. Its `tests/conftest.py` has an autouse database
availability fixture, and some admin tests mutate existing rows. Do not run
that suite against the owner's backend or interpret database skips as passes.
Port just the relevant fixture/behavior patterns into the isolated new tests.

After Rust implementation, run default desktop `cargo check --locked` and
`cargo check --locked --no-default-features --bin codeg-server` in `src-tauri/`,
the focused new service tests in both runtime modes, and
`pnpm exec tsc --noEmit`. Follow reports/step0.md and CI if an own-worktree `out/`
placeholder is needed. Reuse existing tests/engine/storage; keep lockfiles frozen
except an explicitly justified, reviewed implementation dependency change.

**Command ledger and delivery**

| Action | Result |
| --- | --- |
| Initial `git status --short`, branch check; fetch/switch installed help | Clean; old branch `feat/step1-rebrand`. |
| `git fetch origin`; `git switch -c docs/step2-intake-contracts origin/main` | Both exit 0; base recorded above. No subsequent pull/rebase/integration from main. |
| Complete five project-document reads; Hafidh AGENTS/CLAUDE reads; targeted source/config/installed metadata reads | Exit 0. Truncated aggregate output was followed with targeted reads for the relied-upon sections; complete four founding/orchestration docs and AGENTS were read. |
| `gh api` commit/tag/tree/contents reads for the immutable sources above; installed `gh api --help` | Successful reads exit 0; one mistaken FastMCP-directory request returned HTTP 404, corrected by tree discovery and reading the actual failure stub. No source-pin substitution. |
| Offline code-context guide / docs | Exit 0 / 3 respectively; missing corpus disclosed above. |
| `git diff --exit-code v0.30.4 -- <four template/model/service paths>` | Exit 0: inherited template source unchanged. |
| Name-only AST/config/metadata and hook-audit inspections with existing RAG Python `-B` | Exit 0; no imported Hafidh app, credential value output, source writes or environment installs. |
| `git diff --check`; `git status --short` before checkpoint | Exit 0; only this untracked report. `git push -h` printed help and returned its normal usage exit 129. |
| `git commit -m "docs: checkpoint intake and github app source contracts"`; `git push -u origin docs/step2-intake-contracts` | Exit 0; checkpoint `054561094d0d87b37aaed51aeed1cd1ca713807c`, pushed. |
| Final `git diff --check`, name-only diff from branch base, protected-doc/manifest/lock/license diff, and read-only `git -C Hafidh diff --quiet HEAD -- <inspected source paths>` | Exit 0; this report is the sole changed path; protected files/locks and Hafidh source unchanged. |

No Cargo/TypeScript/runtime/browser tests were run for this report-only change;
none are claimed as passed. Source contracts were inspected, not exercised
against production. No product source, protected founding docs, AGENTS, LICENSE,
NOTICE, dependency locks, global config, Hafidh source or another worktree was
changed. No worker was started and no person was messaged.

Final report checks, content commit and draft review URL will be recorded below
after the report-only diff is verified and pushed.
