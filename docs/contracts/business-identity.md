# Business identity contract

Owner: approvals, `feat/business-identity`, base `4ec04d7282a50529335d724438d42b99a53385a2`. This wire/helper contract is fixed for the parallel task and UI implementations; implementation gates are recorded in `reports/business-identity.md`.

## Identity and authorization

- Organization and member IDs are UUID **strings**, on the wire and in SQLite. Table names: `business_organization`, `business_member`, `business_credential`, `business_identity_event`. Migration: `m20260908_000009_business_identity`. One organization per server, enforced by a unique singleton column. Every member/credential/task still carries an explicit `organizationId`.
- `MemberKind = human | agent`; `Role = owner | admin | manager | member | viewer`.
- `Domain = marketing | channels | ads | website | feedback | engineering`. Each member has an explicit `domains: Domain[]`; no empty-list-means-unrestricted default. These are access grants, independent of seniority. The five non-engineering domains are the business discipline. Domain access never grants legacy terminal/config/engine access.
- Active owner/admin/manager humans can read, create, contribute, assign and review within their domains. Active member humans can read/create/contribute within their domains; task ownership/assignment further limits individual mutations. Viewers can only read their domains. Owner/admin can manage identities subject to protected-owner and role-escalation checks. Agents can only read/contribute within their domains and their delegating human's current grants; never assign, review or administer identities. **Task-level visibility, current owner/executor/reviewer, live run binding and CAS remain mandatory in business_tasks.** Assignment alone grants no visibility.
- Bootstrap's operator owner is protected against member API demotion/revocation. Owner/admin can manage ordinary identities; admin cannot change/create owners/admins or issue their credentials. An ordinary member can revoke their own credential. Agent entries cannot receive member login credentials.
- Grant ceiling: an admin cannot expand its own grants or grant/manage another member beyond the admin's current domains. Agents must use role=member, enforced in both schema and core; owner/admin agent combinations are rejected. `legacyOperator` derives the actual operator/native transport, never the role name.

`Member` response: `{ id, organizationId, displayName, kind, role, domains, status: "active" | "revoked", revision, operatorOwner: boolean, createdAt, updatedAt }`. No credential hash or provider/legacy token is returned. Referenced task members must be active, in the same organization and authorized for the task's destination domain. Owner/reviewer must be human; reviewer must have Review authority. Validate this on create and every destination/assignment change.

## Rust helpers (business_identity module)

`Principal` is Clone with private fields, **not Deserialize/Serialize**. Public read-only `organization_id() -> &str`, `member_id() -> &str`, `is_operator() -> bool`; current role/domain must come from revalidation, not a cached claim. Construction belongs to authenticated transport/host-only agent glue.

```rust
pub enum Permission { Read, Create, Contribute, Assign, Review, ManageMembers }
pub async fn authorize<C: ConnectionTrait>(
    conn: &C, principal: &Principal, organization_id: &str,
    permission: Permission, domain: Option<Domain>,
) -> Result<Member, IdentityError>;

pub async fn active_reference<C: ConnectionTrait>(
    conn: &C, organization_id: &str, member_id: &str, domain: Domain,
) -> Result<Member, IdentityError>;

pub async fn begin_write(
    conn: &DatabaseConnection, organization_id: &str,
) -> Result<DatabaseTransaction, IdentityError>;

pub(crate) async fn agent_principal<C: ConnectionTrait>(
    conn: &C, delegator: &Principal, agent_member_id: &str,
) -> Result<Principal, IdentityError>;

pub(crate) async fn operator_principal<C: ConnectionTrait>(
    conn: &C,
) -> Result<Principal, IdentityError>;
```

`begin_write` acquires SQLite writer ownership before any authorization read. Task writes call it, then `authorize`, all reference checks, task CAS and audit append on that **same transaction**, committing together. Identity writes use the same lock. `authorize` rereads current member/org and credential revocation each time. Cross-org/inaccessible references return NotFound. `Member::allows(permission, domain)` supports destination/reference checks after lookup; it does not authenticate a caller. Transaction reads can call `authorize` on the transaction to keep resource reads in one snapshot.

`agent_principal` is trusted crate-only glue after business_tasks validates backend-derived parent/task/run linkage. It stores delegator authority for current rechecks and rejects agent-on-agent delegation. It is not an HTTP endpoint and not a substitute for the live execution/resource checks. `operator_principal` is callable only from verified legacy operator HTTP or native Tauri boundaries; never from serialized actor data or the agent bridge. Native business commands use this same core, preserving existing operator-only native access.

Persistence seam: at the authorized link transaction call crate-only `delegation_grant(&Principal) -> Result<DelegationGrant, IdentityError>` and store `grant.to_storage()?` in the backend-owned link. `DelegationGrant` has private fields, Serialize but no Deserialize or public constructor, and contains only org/member/credential IDs plus operator lineage, **no bearer**. Later, after the task module loads and verifies its exact live link/run, call crate-only `agent_principal_from_binding(conn, organization_id, agent_member_id, stored_grant: &str) -> Result<Principal, IdentityError>`. It checks original org, current human member, original credential revocation (or protected operator lineage) and current active agent. `authorize` then checks the current human/agent domain intersection on each operation. No request field may supply or replace stored_grant; no alternate principal constructor in tasks.

## Protected transport

Business routes live only under `/api/business`. HTTP Bearer can be the configured original operator token or a newly minted `bdm_` member credential. Operator matches derive trusted operator authority; member hashes resolve an individual active human and organization. Business bearer auth is mounted **separately** from the existing legacy `require_token`. No member fallback is added to legacy routes or WebSocket/terminal/config/engine access. No bearer in URLs, browser logs or agent environments. Member session storage must be separate from the legacy operator token/client.

All identity endpoints use POST JSON `{ "input": ... }`, camelCase DTOs and `deny_unknown_fields`:

| HTTP path | Tauri command | input | result |
|---|---|---|---|
| `/api/business/context` | `business_context` | `{}` | Context |
| `/api/business/bootstrap` | `business_bootstrap` | `{organizationName, ownerName}` | Context |
| `/api/business/members/list` | `business_members_list` | `{organizationId, domain?: Domain}` | Member[] |
| `/api/business/members/create` | `business_members_create` | `{organizationId, displayName, kind, role, domains}` | Member |
| `/api/business/members/update` | `business_members_update` | `{organizationId, memberId, expectedRevision, displayName, role, domains}` | Member |
| `/api/business/members/revoke` | `business_members_revoke` | `{organizationId, memberId, expectedRevision}` | Member |
| `/api/business/credentials/issue` | `business_credentials_issue` | `{organizationId, memberId, label}` | `{credential, token}` (token once only) |
| `/api/business/credentials/list` | `business_credentials_list` | `{organizationId, memberId}` | Credential[] |
| `/api/business/credentials/revoke` | `business_credentials_revoke` | `{organizationId, credentialId}` | Credential |

`Context = { needsBootstrap: boolean, organization: {id, name} | null, member: Member | null, operator: boolean, capabilities: {manageMembers, legacyOperator} }`. Unbootstrapped operator context returns needsBootstrap=true; no member credential can exist yet. Bootstrap is authenticated **original operator only**, atomically creates the organization/protected named owner once, and thereafter returns existing context without overwriting it. No public first-user enrollment, email/password reset or login takeover. An ordinary owner credential cannot bootstrap or enter legacy routes.

`Credential = {id, organizationId, memberId, label, createdAt, revokedAt: string | null}`. Mint 32 OS-random bytes; store SHA-256 only. Plain token is returned solely by authenticated issue, never list/context/audit. No provider credential storage or sends. Credential rotation = issue a new individual credential then revoke the old one. Revoked membership invalidates all of its credentials and in-flight task authorization at the transaction boundary.

Errors use existing AppCommandError JSON: authentication_failed/401, permission_denied/403, not_found/404, invalid_input/400, stale revision uses already_exists/409 with stable `business.revisionConflict` i18n_key. DB failures return a safe generic database_error/500 without SQL/token details. Before bootstrap, non-context operations report configuration_missing with `business.bootstrapRequired`. Auth/identity responses use Cache-Control: no-store.

Task router owner supplies its relative `/tasks` subtree for mounting beneath this same authenticated business boundary, with `Extension<Principal>` and existing `Extension<Arc<AppState>>`. No second auth system. Identity owner adds shared registrations sequentially when the task module is ready.

## Provenance and limits

Owner-authorized IntroMail auth pattern only: `IntroInnovation/intromail@0bd24dfe284b888aa9f602fa1fd00e337ea38874`, `backend/app/services/mcp/auth.py`, blob `1faab9205afa2a65c02ff284b12c867c0b85ba99`; its single introduction commit is `13be7b092b6bec9db48e1ac3b71fcdc7493f1fc5`. High-entropy opaque token/hash/revoke/server-derived principal are adapted, not its installation-global permissions. No task implementation of uncertain Plane/OpenProject provenance is ported. Codeg Apache protected handler/runtime/SQLite transaction patterns are reused. Installed rand0.8.5/rand_core0.6.4, sha2 0.10.9, base64 0.22.1, SeaORM1.1.19, Axum0.8.8; no dependency change.

This is a single-server application permission boundary, not same-user OS isolation or hosted multi-tenant security. Existing full-filesystem agents and operator access retain their existing trust model. This increment does not expose new member access to Ops inbox secrets, source ingestion, provider dispatch or engineering routes.
