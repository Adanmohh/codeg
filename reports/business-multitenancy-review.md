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

This first publication fixes the proposed boundaries and implementation order.
Final source-ledger/native-boundary consistency and the owner's newer B seam list
are being reconciled; unresolved prerequisites are named below. No product,
credential, dependency, provider, fixture, browser or build action is authorized
by this report. Branch remains `review/business-intake` in the approvals worktree.

## Verified current constraints

Accepted source pin **7d546c0ee08a6083b993ff459983a4f48791c4df** was resolved
through `gh api`. Eleven inspected identity/task/router/client files match its
Git blobs in the own checkout. Unmerged B is separately pinned
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

Proposed closed endpoint families (new designs, not existing APIs):

- Platform tenant list/create/suspend and resource entrustment require original
  authenticated operator/native platform boundary. Creation is idempotent and
  provisions exactly one named tenant owner with individually issued authority.
- Tenant context and settings read derive organization from `Principal`.
  Settings update uses `{expectedRevision, settings}` with a closed schema and
  current human tenant-admin permission inside `begin_write + authorize`.
- Tenant connection setup uses an owned connection ID, fixed provider kind,
  complete immutable resource identity and new staged secret reference. A tenant
  admin may manage only connections belonging to that tenant and their permitted
  domains. Legacy host account/inbox/product association remains a separate
  protected-platform entrustment, with no ordinary-member fallback.

Display name, bounded logo asset reference, vetted paired light/dark token choices,
default work areas and available layout preferences are tenant-owned settings.
Personal layout/tab choices remain member-owned. No arbitrary CSS/HTML/scripts,
provider URLs, secret values or host paths in settings. Module visibility never
grants a capability. Public login branding, if later required, serves only a
deliberately published safe snapshot; otherwise use platform defaults.

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
| Browser/native/platform | Issued tenant owner/admin credentials work on allowed tenant operations, fail host legacy/terminal/global events/bootstrap/platform APIs. Native restricted session behaves identically; ambient local operator cannot answer its tenant requests. Actual platform positive remains. |
| Transaction/race | Two SQLite connections race revocation/suspension, role/profile/owner drift, grant changes and settings/task CAS. Stale queued writes and provider results leave no unauthorized task/link/receipt; a fresh currently authorized operation succeeds. |
| Sources/store | Same provider-local IDs in A/B cannot dedupe or disclose across tenants. Wrong secret/resource refs, cross-tenant grants and late stages fail; strict store preserves unrelated keys. Cross-import out-of-order completion cannot restore old freshness. |
| CLI families/resources | A's issued run capability succeeds on A's task/session/artifact and permitted operation; B IDs, forbidden family, parent/child mismatch, forged root, broader child grant and global `get_session_info` fail at the dispatcher. Revoked original credential/profile and retired generations fail before side effects. Check event subscriptions and file/path traversal as well. |
| Retained migration | Snapshot populated A before migration; inject failure, verify rollback, retry, add B and compare original rows/IDs/hashes/history. Foreign-key checks pass. Refuse rollback that would erase B; do not target whichever migration happens to be last. |
| UI/state/settings | Two users and A→B→A with delayed A response/event: no wrong-tenant frame/cache/draft. Cancelled switch preserves old edits; confirmed switch clears them. Concurrent settings edits conflict visibly. Tenant theme cannot hide required review/focus state; light/dark/RTL/narrow and rich panes remain usable without legacy requests. |

## Proposed next ownership and explicit gaps

Root assigns the concrete migration number and integration sequence. Suggested
bounded ownership: identity/router owner implements explicit platform versus
tenant resolution, organization lifecycle and settings CAS first; tickets owns
task/source composite boundaries, B custody/fences and engine/CLI capability
integration; rebrand owns the same rich workspace, tenant administration/switching
and draft/cache lifetimes; approvals independently reviews exact heads and races.
Keep one writer for shared registries and publish compiling helper/DTO checkpoints
before dependent UI. No worker is started or reassigned by this report.

Unresolved implementation contracts: native restricted-window authorization,
the trusted execution-profile/isolation boundary and operation-family enforcement,
safe retained-parent-table migration mechanics, and tenant-admin setup delegation
versus platform legacy-resource entrustment. These must be made concrete before
claiming tenant execution or full B tenancy. Multi-tenant accounts/settings/tasks
can be an independently usable first increment while that execution work proceeds.

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

Code-context used the existing rag-skills venv with `HF_HUB_OFFLINE=1`:
guide exit0; dependency docs exit3 because `approvals.db` is absent. The
**Atomic per-task staging** rule applies. Retrieved per-tenant-database and
superadmin conventions belong to other projects; they are relevant comparisons,
not authority to transplant another application's policy. Installed pins and
actual source remain primary. No missing corpus coverage is claimed restored.

All tests mentioned as completed belong only to the preceding exact67708 review.
This tenancy phase has run no tests/build/browser/provider. It preserves the user
desktop preview and every existing fixture/export/target. Source presence alone
does not establish production isolation, security acceptance or visual completion.
