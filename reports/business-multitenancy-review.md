# Multi-tenant business workspace — independent architecture review

**Architecture proposal, not an implementation or runtime acceptance.** The
target is one rich workspace for business and engineering roles: tabs/panes,
shared tasks, tables/boards, content calendars, visualizations and AI conversation/
terminal surfaces. Tenant authority must control the data and execution behind
those surfaces. Hiding an engineering link cannot provide that boundary.

This supersedes the earlier single-organization target. Preserve accepted A and
the secure parts of B; do not relax existing routes while the new seams are built.
The preceding bounded B review is pushed at
**d92d1b9534025f994a85f070d3bb78174c876e62**: exact67708b07, 12 independently
passing focused tests, no full B acceptance. Its evidence remains unchanged.

**Bounded research is complete.** Early architecture checkpoint
**a2be945d9869c9807a5e3f4d7a98dab1fc2cf2cf** was pushed and read by root. This
version reconciles installed Tauri, B's newer seam list and the concrete identity
contract below. Root reserves forward migration **000012_business_tenancy**;
implementation is a separate dispatch. Execution isolation remains separately
scoped and must not block tenant accounts/settings/tasks or shared-shell design.
No product, credential, dependency, provider, fixture, browser or build action
ran in this phase. Branch remains `review/business-intake` in the approvals worktree.

## Verified current constraints

Accepted source pin **7d546c0ee08a6083b993ff459983a4f48791c4df** was resolved
through `gh api`. Twenty-four inspected source/configuration/pin files match its
Git blobs in the own checkout. Root's direction amendment was read at
**a4a4b6f4b203d0fdcc0b020e59a82669e6a4e0ca**. Independently reviewed B is pinned
**67708b0768cae3cacecd0dd1989c57bb565ab5f0**. These are source observations:

| Boundary | Current behavior and required change |
| --- | --- |
| Organization | Migration000009 has `UNIQUE singleton CHECK(singleton=1)`. `business_identity/store.rs:102` fetches that row; bootstrap conflicts on singleton. `context` and `operator_principal` therefore choose one organization implicitly. Remove that assumption in a forward migration and explicit resolution; adding tenant UI alone is insufficient. |
| Browser member | `business_identity/http.rs` resolves a hashed `bdm_` bearer to stored organization/member/credential. `authorize` compares requested organization and rereads membership/credential; keep this. Caller organization IDs are requested scope, never identity. The business client keeps its bearer in memory and never uses ambient `CODEG_TOKEN`. |
| Platform operator | The original token gets the operator marker and currently becomes the singleton's protected owner. Platform authority must become explicit, outside tenant roles; a tenant owner/admin token must never become that marker. Preserve legacy host access for the actual operator. |
| Native | `commands/business_identity.rs` derives local operator context directly; `BusinessConnection.kind=native` invokes those commands. A tenant selector cannot turn this into restricted native member authentication. A restricted tenant window/session must use the business principal boundary, without ambient owner invocation. |
| Tasks | Migration000010 already carries organization IDs, composite references, immutable history and exact run lineage. Keep task/actor/owner/assignee/reviewer in one tenant. A globally unique work-task/run binding prevents two tenants claiming one host run; do not weaken it to allow duplication. |
| Source access | B67708 has scoped binding/grant/receipt rows, explicit zero grants, owner revision and binding/config epochs, staged unique secret references and current-writer checks. Setup still requires actual operator; legacy account/inbox/product resources and credential custody are host-global. Tenant admin requires a closed tenant-owned connection seam, not access to legacy configuration. |
| UI lifetime | `src/app/business/page.tsx` keys private editing by connection generation, organization, member and revision; client close aborts HTTP and ignores late results. Extend this to explicit tenant changes and all reused tabs/caches. Keep locale/viewport changes inside the same editing lifetime. |
| Host terminal/events | `web/router.rs:1657` terminal operations and `:1730` global events remain behind original-token authentication. Terminal handlers accept host working directory/shell/command and prepare host credential environment. They are not tenant APIs. |
| CLI companion | `acp/delegation/listener.rs:715–737` deliberately accepts any valid companion token for arbitrary non-deleted conversation lookup. Its comment explicitly assumes a single tenant/full filesystem. This is a prerequisite against the **new** architecture, not a newly asserted defect in accepted A. |

Read the complete newer owner report at
**5de1176beb4778604b26df65f2ec669b0ab7987b**, blob
**c13ee91738e57e295b81a4d6ab72a18af0e4150d**. Its source/import seam list agrees:
native source/import registrations wait for tenant selection; explicit Principal/
org core can proceed; global Ops account/default1, product/folder associations
and staged key custody need ownership. Production
**4a194500b76b97aa5caaf9434ce5c1f16e54ea48** and its five passing import tests
remain **owner-reported**, not independently reviewed/executed here. No broader
B verdict follows from reading that report.

## Lessons from the pinned sources

Edublend **735e7695a44ab6e5dbda521c822f3a3809f289c8** distinguishes a platform
superadmin from ordinary organization users, resolves ordinary scope from the
authenticated profile, and makes platform “view as” explicit. Its organization
context records selection and invalidates a tenant policy cache. Branding reads
serve authenticated members; writes require admin authority and an organization
check. These are useful architectural patterns, not source to copy.

Do **not** adopt its `tenant_scope.py` absent-context/inspection-error unfiltered
SELECT behavior, its assumption that relationship loads need no additional
filter, its public-theme draft fallback, arbitrary `customCss`, or header/hostname
selection as proof of membership. Branding writes in the inspected endpoint
also lack the task-style expected-revision CAS we need. No broad Edublend security
verdict is claimed; only the named source paths were inspected.

Payload **54a0e3d24015b2e9c565bd7e695be1ec7184662e**, MIT, is the sole external
comparator. Its multi-tenant plugin separates a selected-tenant UI filter from
access constraints and intersects ordinary collection access with membership
constraints, including historical versions. Tenant settings are per-tenant
records rather than one global object. Its configurable access opt-outs,
all-tenants escape and default related-document cleanup are not proposed defaults
here. Do not install Payload or copy its CMS runtime; reuse the existing Rust core.

## Proposed identity and administration contract

Use the existing UUID organization/member scalars and opaque hashed/revocable
credentials. Initially each credential authenticates one membership in one
tenant. A person using several tenants supplies the corresponding individual
credential for each; a dropdown cannot expand one credential's scope. A future
shared-login/membership model is separate work, not a prerequisite for two secure
tenants on one server. No public signup or bootstrap takeover.

Platform context lists/provisions/suspends tenants and provisions trusted host
resources. Tenant context always names one organization. Missing scope fails
closed on tenant operations, including background work. Platform inspection must
use an explicit target and attributable audit entry, never a null tenant meaning
“all rows.” Platform setup authority is not automatic private-source publication
authority or a tenant human's approval.

Keep one current `Principal` authorization core; add a distinct transport-proven
platform setup context instead of deriving privilege from role names. Existing
operator-owner rows, credentials and history retain their original tenant and
IDs. Do not create an all-powerful synthetic tenant member whenever the platform
operator switches. New tenant owners/admins manage their tenant within current
grant ceilings; none may create platform operators, change another tenant,
rebind host resources, or give agent-kind members admin/owner powers.

The following names/shapes are the proposed next contract, **not callable APIs
at the reviewed head**. Keep the existing business `Principal`; add no alternative
member identity or token scheme.

| Seam | Closed proposal |
| --- | --- |
| `PlatformContext` | Backend-private, non-deserializable authority created only from the verified original HTTP operator marker or trusted native platform window. No implicit organization or tenant role. Public view contains platform capabilities and tenant metadata, never a member bearer. |
| Platform HTTP | Separate original-token router at `/api/platform/business/{context,tenants/list,tenants/create,tenants/status,tenants/reissue-owner-credential}`. POST `{input}`; member/agent credentials are rejected before handlers. No business member endpoint merges into this router. |
| `tenants/create` | Input `{operationId, organizationName, ownerName}`. Same operation ID and canonical input digest returns the same tenant/owner/credential metadata; changed payload conflicts. Transport supplies actual platform attribution. One SQLite transaction creates organization, human owner, initial hashed credential, platform audit and operation receipt. No provider/keyring await inside it. |
| Provision result | `{organization, ownerMemberId, ownerRevision, credential, delivery}` plus the secret **only on first issuance**, using the existing one-response credential type. `delivery` is `issued_once` or `already_provisioned`. Replay never creates another tenant/credential or claims secret redelivery. Platform `tenants/list` also exposes the current provisioning-owner revision/status and current provisioning-credential metadata for recovery. No raw credential in list, receipt, audit, URL, logs or settings. |
| Lost issuance recovery | Explicit original-operator `tenants/reissue-owner-credential` with `{operationId, organizationId, expectedRevision, expectedAuthorizationEpoch, ownerMemberId, expectedOwnerRevision, expectedCredentialId}`. Require the **current active credential** in that tenant's provisioning/reissue lineage and the same current active human owner/revision; a historical ancestor is insufficient. Atomically revoke that credential, mint a replacement hash and record audit/receipt; return the new secret once. A lost result requires another explicit rotation after reading current metadata, never stored-plaintext replay. Preserve unrelated credentials. Named-human delivery is a deliberate operator handoff; no email/send added. |
| Tenant summary | Extend `{id,name}` with `{status:active\|suspended, revision, authorizationEpoch}`. Keep UUIDs stable. A Principal captures the resolved epoch and rechecks current active organization/epoch with membership/credential in snapshots and writers. |
| `tenants/status` | Input `{operationId, organizationId, expectedRevision, expectedAuthorizationEpoch, status}`. Actual platform-only CAS and audit/receipt. Suspension **and** resumption advance epoch and revision. Stale setup/import/decision/run work cannot resume when status returns to active. Existing human credentials may authenticate afresh after resume; previously captured Principals/grants cannot silently refresh their epoch. |
| Tenant settings | `/api/business/settings/{get,update}`, same member middleware; update `{expectedRevision, settings}` with no organization/actor/credential selector. Add `Permission::ManageTenantSettings` for current human tenant owner/admin. Preserve current member/domain grant ceilings. Use a separate settings revision for theme/layout CAS. Settings never change permission/execution entitlements. |
| Member/core | Replace singleton lookup with `organization(conn, principal.organization_id())`. `authorize` checks organization active/epoch before member/credential/domain. Background work restores only validated stored grants with captured epoch, never an operator from requester IDs. Retain `begin_write` ordering. |

Existing operator-owned history needs an explicit compatibility mapping: retain
the original organization ID and protected actor in migration metadata. If old
local business operations remain available, derive their compatibility Principal
only from actual platform context **and explicit selection of that retained
organization**. Do not extend this exception to new tenants or let missing selection
choose the first row. New-tenant task/source decisions use individual Principals;
platform administration uses typed operations with real platform attribution.
Retained operator delegation resolves only against the original mapping/initial
epoch. New epoch-aware grants must not rewrite immutable history or resurrect old
grants after suspension.

Keep tenant member administration inside its current grant ceiling, including
non-owner admin elevation restrictions and agent-kind restrictions. Ownership
transfer must be explicit and atomic; refuse removal/demotion of the last active
human owner. Provisioning/reissue authority is an accountable administrative
capability, not a claim of isolation from the protected host/platform custodian.

Tenant connection setup uses an owned connection ID, fixed provider kind,
immutable resource identity and staged secret reference. Tenant admin manages
only that tenant's connections within its permitted domains. Legacy host account/
inbox/product association remains a separate protected-platform entrustment, with
no member fallback. Setup gives no implicit private-source grant.

Display name, bounded logo asset reference, vetted paired light/dark token choices,
default work areas and available layout preferences are tenant-owned settings.
Personal layout/tab choices remain member-owned. No arbitrary CSS/HTML/scripts,
provider URLs, secret values or host paths in settings. Module visibility never
grants a capability. Public login branding, if later required, serves only a
deliberately published safe snapshot; otherwise use platform defaults.

### Native window enforcement

Installed **tauri2.10.2**, `src/webview/mod.rs:1776–1810`, checks custom application
commands through ACL when an app ACL manifest exists; plugin checks are separate.
`tauri-utils2.8.2/src/acl/capability.rs:77–164` documents window/webview matching;
`tauri2.10.2/src/ipc/authority.rs:439` checks actual origin and labels. These are
usable enforcement points, not proof this app has a restricted tenant window.
Current `build.rs` uses default `tauri_build::build()`; no tracked app permission
manifest was found. The existing `main` capability includes broad local opener
access and host commands are registered. `terminal_spawn` uses its window for
ownership metadata; `terminal_write` does not authenticate a tenant/window.
A frontend HTTP transport choice alone cannot restrict that surface.

Use a **separate restricted tenant webview/window**, excluded from host capability
groups. Its explicit application-command allowlist permits only the authenticated
business-session bridge and necessary window controls, never host terminal/config/
credential/engine commands. The bridge holds the personal credential/session in
backend memory, binds it to the invoking window, and resolves the same Principal
on each call; another window cannot supply its session handle. Using the existing
HTTP client is also possible, but still requires the native IPC denial. Trusted
platform windows retain host functionality. Rich tabs/panes within the tenant
window share that tenant boundary.

Inspect the **actual application ACL manifest**, generated command coverage and
native command/window checks during implementation. Unknown/cross-window sessions
fail closed and teardown revokes them. Test direct privileged invocation from the
restricted window, not just the normal UI's command choices. Untrusted tenant
content never enters the platform window. Neither UI roles, current capability
JSON nor a custom window label alone establishes this isolation.

## Retained-data migration and isolation

Recommend one SQLite application database for the first tenant-aware increment,
preserving atomic identity/task/source/receipt operations. This is application
isolation, not an OS/process boundary. SQLite still has a database writer; the
organization no-op lock does not create independent per-tenant writers.

Add a new forward migration; never rewrite applied000009/000010 or unmerged
000011 history. Removing the singleton uniqueness requires a reviewed table
rebuild preserving references, not a guessed `DROP COLUMN`. Retain the existing
organization/member/token hashes, task/activity/review/execution lineage and
all source versions/grants/receipts. Replace unscoped organization lookup and
bootstrap with explicit organization/context helpers before enabling creation
of the second tenant. Add organization status/revision and per-tenant settings
revision. Pause/revoke must fence ongoing reads and commits; away/back changes
must not resurrect old work.

Inventory every shared record and event as tenant-owned or platform-only:
business tasks, conversations, artifacts/files, source bindings/imports/candidates,
provider connections, drafts/proposals/receipts, sessions and run resources.
Use composite tenant foreign keys and tenant-prefixed unique/dedupe/cursor keys
where IDs are provider-local. Keep host run uniqueness global. Existing unowned
legacy records stay platform-only until explicitly entrusted; do not grant every
tenant all existing inboxes/products or silently assign them to the first tenant.
Migration must preserve foreign-key/trigger guarantees, fail atomically under
injected failure, and refuse lossy down-migration once multiple tenants exist.

Retain B's staged immutable secret references and strict credential writer.
Reference metadata is tenant-owned and never caller-selectable. Active/reference/
owner/grant/epoch checks stay in the SQLite writer; provider/keyring/file work
stays outside it. No DB-plus-keyring/file atomicity claim. Reassociation uses a
new binding with zero grants; cross-import source attempt ordering includes
tenant identity. A task's destination audience never grants private transcript
access, even to that tenant's administrator.

## Minimum safe AI/CLI execution seam

Reuse TaskEngine and its exact generation/ancestry lock; do not build another
runner. A human with tenant execution/assignment permission requests work on an
owned task using its revision and a platform-entrusted execution profile ID.
Backend resolves the tenant, task/domain, assigned agent member, approved resource
profile and original credential lineage. No caller-selected actor, generic
command executor, arbitrary host cwd/env/shell or operator bearer reaches the
agent. Persist the authorization/reservation before the existing engine starts
work, then revalidate cancellation, membership, grant/profile epoch and run
identity before activation and every contribution/dispatch.

Use the accepted source-entrustment plus individual `DelegationGrant` pattern;
run liveness or a generic Pi type is insufficient. The new capability must bind
tenant/task/member/delegator credential/profile/resource epoch/root connection/
run generation and a **closed operation-family allowlist**. Enforce it at token
creation and the actual dispatcher, with tenant resource checks on session reads,
events, files, task authoring and child delegation. Tool discovery filtering alone
does not enforce authority. Child scopes can only narrow; revoke all descendants
and queued effects on parent retirement, cancellation, reassignment, tenant
suspension or original credential revocation. Human review and exact approved
payload/receipt rules remain authoritative.

**Open implementation prerequisite:** arbitrary terminal/CLI programs running as
the server's OS user can read other tenants' files and credential stores despite
API filters. A platform-provisioned isolated worker boundary with tenant-only
filesystem/credentials and controlled network/resources is required before
offering that capability to mutually untrusted tenants. Reuse existing executor
code within that boundary. Until verified, tenant AI may expose only demonstrably
scoped operations; host shell stays separately authorized and the UI must state
unavailable execution honestly. No claim that a working directory or filtered
environment is a sandbox, and no new isolation runtime is selected here.

## Tenant switch and required acceptance probes

Explicit switching must resolve the new authenticated context before rendering
tenant content. Give a discard/cancel choice for unsaved private edits while old
authority remains valid. On confirmed switch, logout or revocation: invalidate
session generation, abort/ignore in-flight responses, unsubscribe events, clear
query/search caches and all draft/selection/pane content, then mount the new
tenant scope. Key private state by backend origin + tenant + member + credential
session generation. Never persist private drafts or bearer values to local/session
storage. Tenant settings may persist server-side with CAS; viewport/locale/theme
updates in the same scope must preserve edits. Cross-tab messages carry only safe
invalidation metadata, never content or credentials.

All following probes are **planned, not executed**:

| Probe | Positive and negative required evidence |
| --- | --- |
| Same-database two tenants | Real protected provisioning creates A/B with current members and normal tasks in one DB. A and B each succeed on their own reads/writes; replacing org/member/task/source/history/search/cursor IDs fails without existence disclosure or changed rows. Include direct composite-FK violations, not a member ID from another DB. |
| Provision/recovery | Race/replay the same create operation: one tenant/owner/credential, changed body conflicts. Drop the successful response: replay contains metadata only; explicit revision-bound reissue revokes exactly its prior credential and returns one new secret. Fail before commit: no partial tenant/owner/receipt. Inspect logs/DB/URLs for absence of plaintext. |
| Browser/native/platform | Issued tenant owner/admin credentials work on allowed tenant operations, fail host legacy/terminal/global events/bootstrap/platform APIs. Native restricted session behaves identically; ambient local operator cannot answer its tenant requests. Actual platform positive remains. |
| Transaction/race | Two SQLite connections race revocation/suspension, role/profile/owner drift, grant changes and settings/task CAS. Stale queued writes and provider results leave no unauthorized task/link/receipt; a fresh currently authorized operation succeeds. |
| Sources/store | Same provider-local IDs in A/B cannot dedupe or disclose across tenants. Wrong secret/resource refs, cross-tenant grants and late stages fail; strict store preserves unrelated keys. Cross-import out-of-order completion cannot restore old freshness. |
| CLI families/resources | A's issued run capability succeeds on A's task/session/artifact and permitted operation; B IDs, forbidden family, parent/child mismatch, forged root, broader child grant and global `get_session_info` fail at the dispatcher. Revoked original credential/profile and retired generations fail before side effects. Check event subscriptions and file/path traversal as well. |
| Retained migration | Snapshot populated A before migration; inject failure, verify rollback, retry, add B and compare original rows/IDs/hashes/history. Foreign-key checks pass. Refuse rollback that would erase B; do not target whichever migration happens to be last. |
| UI/state/settings | Two users and A→B→A with delayed A response/event: no wrong-tenant frame/cache/draft. Cancelled switch preserves old edits; confirmed switch clears them. Concurrent settings edits conflict visibly. Tenant theme cannot hide required review/focus state; light/dark/RTL/narrow and rich panes remain usable without legacy requests. |

## Proposed next ownership and explicit gaps

Root reserves **000012_business_tenancy** and assigns integration sequence. Suggested
bounded ownership: identity/router owner implements explicit platform versus
tenant resolution, organization lifecycle and settings CAS first; tickets owns
task/source composite boundaries, B custody/fences and engine/CLI capability
integration; rebrand owns the same rich workspace, tenant administration/switching
and draft/cache lifetimes; approvals independently reviews exact heads and races.
Keep one writer for shared registries and publish compiling helper/DTO checkpoints
before dependent UI. No worker is started or reassigned by this report.

Next implementation must concretize the proposed native ACL/session bridge and
retained-parent-table migration mechanics; source setup must distinguish tenant
connection management from platform legacy-resource entrustment. The larger
execution-profile/isolation boundary and operation-family enforcement are a
**separate** gate before tenant CLI access. They do not block accounts/settings/
tasks or shared-shell visual work. No migration execution or fourth worker is
authorized by this research handoff.

## Source and licence ledger / evidence limits

Remote source is fetched only through immutable `gh api`; recomputed Git blobs
match. Edublend is proprietary owner source: architectural inspiration only,
no source republication or adaptation in this report/product. Its commercial
licence summary is `docs/site/content/docs/legal/license.mdx`, blob
`fc8f9ab6ce69ca92d08ec37a5651a1355b33d48a`. Payload plugin MIT licence is
`packages/plugin-multi-tenant/LICENSE.md`, blob
`b01626c32d15467951f80dc77a0427ed3483a815`, Copyright2018–2026 Payload CMS LLC.
No code was ported, so root NOTICE is unchanged; a later permissive port must
retain original attribution and immutable file mapping. No AGPL/GPL source,
SDK/framework install, live account or dependency upgrade.

| Exact pin | Inspected paths and blobs |
| --- | --- |
| Edublend735e7695 | `backend/app/db/tenant_scope.py` → `ab6eedfb3eb87ebd2ee12a93b0f92ae4bd1a06cf`; `frontend/src/contexts/OrganizationContext.tsx` → `4c5a2308b3e922060064905542dd9c688b553ad6`; `frontend/src/components/TenantBrandingProvider.tsx` → `35a02af51a9610bee5a6b70e9a53a994e73f627f`; `backend/app/modules/platform/api/org_branding.py` → `a85a35ce3685fed89c7d10caa5f59a1b7782f60f`; relevant authentication/admin/tenant-resolution functions in `backend/app/common/api/deps.py` → `9efca912f7a54311bf8d4081286e08a532319d17` |
| Payload54a0e3d | `packages/plugin-multi-tenant/README.md` → `74cc95335cb2866bdad3ee830513aaa516fc1aeb`; `src/index.ts` → `fee5295724b768a3b703cad4a2144be372427bae`; `src/utilities/addCollectionAccess.ts` → `cb8bc1d4af90f52a8448a349748c98906bd6539a`; `src/utilities/getTenantAccess.ts` → `6d434d74523aa975c425f905b1ba8297ddebe346`; `src/filters/filterDocumentsByTenants.ts` → `85066e7172c03607fadd4da9ad83e62c4e131b3b` (all paths after README relative to the plugin directory) |

Primary links: [Edublend tenant scope](https://github.com/IntroInnovation/edu-blend/blob/735e7695a44ab6e5dbda521c822f3a3809f289c8/backend/app/db/tenant_scope.py),
[Edublend branding boundary](https://github.com/IntroInnovation/edu-blend/blob/735e7695a44ab6e5dbda521c822f3a3809f289c8/backend/app/modules/platform/api/org_branding.py),
[Payload access composition](https://github.com/payloadcms/payload/blob/54a0e3d24015b2e9c565bd7e695be1ec7184662e/packages/plugin-multi-tenant/src/utilities/addCollectionAccess.ts),
[current companion prerequisite](https://github.com/Adanmohh/codeg/blob/7d546c0ee08a6083b993ff459983a4f48791c4df/src-tauri/src/acp/delegation/listener.rs#L715),
[current singleton schema](https://github.com/Adanmohh/codeg/blob/7d546c0ee08a6083b993ff459983a4f48791c4df/src-tauri/src/db/migration/m20260908_000009_business_identity.rs#L12).
These support the described patterns and limits, not deployment claims.

Code-context used the existing rag-skills venv with `HF_HUB_OFFLINE=1`:
guide exit0; dependency docs exit3 because `approvals.db` is absent. The
**Atomic per-task staging** rule applies. Retrieved per-tenant-database and
superadmin conventions belong to other projects; they are relevant comparisons,
not authority to transplant another application's policy. Installed SeaORM1.1.19/
SQLx SQLite0.8.6 writer sources, React19.2.4 and the Tauri pins above remain
primary. No missing corpus coverage is claimed restored.

Command evidence: gh-api content/tree reads and Git-blob recomputation exited0
for the named sources (both recursive source trees were complete). Local-main
correspondence is **24/24**, zero mismatches. Local source/CLI help reads exited0;
path searches that found no local session.tsx/application-permission file were
resolved through actual `src/app/business/page.tsx` and pinned Tauri source,
not presumed APIs. Early report diff check/commit/push exited0. Raw private source
caches and hash ledgers remain in the own `.build/business-multitenancy`, outside
the commit; only this architecture report is published. Existing source NOTICE
and root planning documents remain unchanged.

Live docs-first records remained enabled through report edits: own session
`01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`, approvals cwd, PreToolUse audit line20527
and PostToolUse line20515 in `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`.
No hook was disabled or bypassed. Astra/max and existing-worker-only limits remain.

Independent executed evidence remains at the preceding exact67708 checkpoint;
the newer five import passes are explicitly owner-reported only.
This tenancy phase has run no tests/build/browser/provider. It preserves the user
desktop preview and every existing fixture/export/target. Source presence alone
does not establish production isolation, security acceptance or visual completion.
