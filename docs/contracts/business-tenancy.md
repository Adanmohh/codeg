# Business tenancy contract

Owner: approvals, `feat/business-tenancy`, base `f3813e3f1edb521f1d1b20d0b372643acc4123a5`. Implements the accepted [architecture](../../reports/business-multitenancy-review.md), frozen at `af00c956787142f900384f7ba6b34ebbc005eb88`. This initial publication fixes the helper/DTO contract for parallel workers; compiling checkpoints and executed gates belong in [the report](../../reports/business-tenancy.md).

## Identity and epochs

Keep UUID String IDs and individual `bdm_` hashed/revocable credentials, one tenant per credential. `Principal` remains private, non-serializable transport-derived authority. Add **`pub fn authorization_epoch(&self) -> i64`**, a read-only captured value with no setter/refresh helper. Authentication captures the active organization's current epoch once. `authorize` rechecks organization active/current epoch, original credential, membership and domain on the supplied snapshot/writer. Existing `begin_write`/`authorize`/`active_reference` signatures remain stable. Never reconstruct or reauthenticate a Principal after an asynchronous operation to rescue old work.

`Organization = {id, name, status: "active" | "suspended", revision: number, authorizationEpoch: number}`. Initial revision/epoch are 1. Each lifecycle transition, including resume, increments both. Same-status updates conflict rather than consume an operation. Authentication while suspended fails. After resume an existing active human credential can authenticate afresh; a previously captured Principal cannot.

`DelegationGrant` stores captured `authorization_epoch`; restored grants must match current organization epoch. Existing stored grants without that field are valid only for the retained original organization at its initial epoch1. They never inherit a current epoch. No history rewrite or credential serialization.

B owns persisted epoch fields on its setup/attempt/source-observation/candidate-preview lineage. Its caller stores `principal.authorization_epoch()` and compares it during final writer authorization and later freshness reuse. New login does not bless old preview/source freshness. No default epoch for B rows. A retained row lacking evidence of current authority is preserved but requires explicit fresh authorized validation. This branch never edits B tables. Migration order is000009,000010, **tickets-owned000011**, **approvals-owned000012**. On the accepted A-only base000012 can register after000010; integration must insert000011 before000012. Both owners test retained rows when the combined committed source is available. Migration names, not last-index assumptions, select tests.

## Closed tenant settings

HTTP POST `/api/business/settings/get` `{input:{}}` returns:

```json
{"organizationId":"uuid","revision":1,"settings":{"displayName":"Organization name","palette":"neutral","workspaceLayout":"split","defaultWorkArea":"tasks"}}
```

POST `/api/business/settings/update` uses `{input:{expectedRevision,settings}}`; returns the same view with revision incremented. No actor, organization selector, credential, URL, HTML, CSS, asset, secret or path fields. Unknown fields and unknown enums fail. `displayName` is trimmed text, 1–120 Unicode characters, no control characters. It changes tenant presentation only; legal/provisioning name and org lifecycle revision stay separate.

Exact enums: `palette = neutral | blue | violet`; `workspaceLayout = split | stacked`; `defaultWorkArea = tasks | conversations`. Defaults are **neutral / split / tasks**. Palette IDs reuse the installed paired light/dark `[data-theme]` tokens in `src/app/globals.css` and `src/lib/theme-presets.ts` (default neutral retains Hafidh teal). Layout describes the preferred shared shell arrangement, not saved private tabs/panes. Conversations is a shell preference, not an entitlement or claim of available tenant AI execution. No remote logo loading in this increment. Personal theme mode/locale/temporary layout remain member UI choices.

Reads require authenticated human tenant Read; writes add `Permission::ManageTenantSettings`, current human owner/admin only. Agents cannot read domain-less settings or administer them. Settings revision CAS, authority revalidation and audit commit in the same SQLite writer transaction. `Context.capabilities` gains `manageTenantSettings`; all older fields remain. UI must preserve drafts for same-tenant appearance/locale/viewport changes, and discard/abort old client data only after explicit credential/tenant switch. No private draft persistence.

## Platform transport and recovery

Private `PlatformContext` has no tenant/member role and no Deserialize constructor. Only the original authenticated operator HTTP marker or a trusted native platform window constructs it. Tenant owner/admin credentials never qualify. Separate POST `{input}` router `/api/platform/business/`:

| Path | Input | Result |
|---|---|---|
| `context` | `{}` | `{platformOperator:true}` |
| `tenants/list` | `{}` | tenant summaries, current provisioning owner/credential metadata, no secrets |
| `tenants/create` | `{operationId,organizationName,ownerName}` | provision result |
| `tenants/status` | `{operationId,organizationId,expectedRevision,expectedAuthorizationEpoch,status}` | Organization |
| `tenants/reissue-owner-credential` | `{operationId,organizationId,expectedRevision,expectedAuthorizationEpoch,ownerMemberId,expectedOwnerRevision,expectedCredentialId}` | provision result |

Operation IDs are UUID strings. Store canonical input digest + immutable operation receipt in the same writer transaction as organization/owner/credential/audit changes. Replay with identical input returns the original metadata only; changed input conflicts. Provision result: `{organization,ownerMemberId,ownerRevision,credential,delivery,token?}`, delivery `issued_once | already_provisioned`. Secret appears only in the first successful response; no Debug/Clone, durable plaintext, list/audit/log/URL disclosure. No implicit delivery through email.

Reissue requires the exact current active human provisioning owner, current owner revision, organization revision/epoch and current active credential in the provisioning/reissue lineage. Atomic revoke-old/create-new preserves unrelated credentials. Lost response: read current metadata and explicitly rotate with a **new operationId**. Historical ancestor credentials cannot authorize rotation. Replay does not revoke the new credential or return its secret. Member owner demotion/revocation has a same-writer last-active-human-owner guard. Existing protected original operator owner remains immutable; ordinary owner/admin grant ceilings and agent restrictions remain.

Retain original organization/protected owner via explicit compatibility mapping, never first-row selection. Legacy business operator routes/native platform commands select **only that mapped original tenant**. No compatibility Principal for newly provisioned tenants; use their individual member credentials. This is a fixed original-org compatibility selector, not a multi-tenant platform impersonation endpoint. All legacy CODEG_TOKEN/config/terminal/engine routes remain platform-only.

## Native enforcement and ownership

Approvals owns app ACL manifest and a separate `tenant-*` local window/session bridge. Restricted window capability permits only business session commands and necessary local window controls; no host application/plugin opener/config/engine/terminal permissions. Tauri app ACL must be active for **all** registered custom commands, not merely a frontend route guard. Host windows retain explicit host permissions. Backend session is private in memory and bound to actual invoking window; session input cannot choose actor/tenant. Login derives Principal from a member credential, rejects the original operator bearer, and captures epoch. Switch/logout/teardown invalidates old handles; cross-window and closed handles fail. Generic arbitrary command dispatch is not exposed. Native acceptance requires a direct privileged invoke rejection test through the real Tauri ACL path, plus allowed platform and tenant controls.

Native availability correction, approved by root during source review: **production restricted tenant windows remain unavailable**, with `business_window_context` returning `{platformOperator,restrictedTenantWindow,tenantWindowAvailable:false}`. No window-creation endpoint is registered. Tauri2.10.2 exempts its built-in channel fetch command from ACL and uses an application-global queue; its public channel interceptor does not cover the private callback producer on the non-macOS/non-iOS postMessage response branch. Command ACL/session tests alone are not tenant native acceptance. Existing native platform access remains; do not render tenant content in a host window as an isolation workaround. No vendor/dependency change authorized.

Prepared native wire: `business_session_open({input:{token}}) -> {session,context}` accepts only personal member credentials in an actual restricted window; `business_session_close({input:{session}})` revokes that exact window handle. Existing native business commands add optional top-level `session`; restricted windows require it, trusted platform windows reject a supplied handle. Same-core `business_settings_get/update` follow this rule. Platform commands are `business_platform_context`, `business_platform_tenants_list/create/status/reissue_owner_credential`, with the same input/result DTOs above. Custom `business_window_control({input:{action}})` allows only `close|minimize|toggle_maximize|is_maximized|start_dragging` on the **invoking** window, with no caller-selected label. Generic window plugins/global event subscription are denied in tenant capability. These prepared native paths do not make production restricted windows available.

A trusted but not yet linked source authorization also needs an epoch fence. Migration000012 adds an immutable `business_execution_authority_epoch` sidecar, preserving A authority rows. Initial retained rows capture epoch1; a trigger captures current active organization epoch on new trusted source insertion. The narrow A authority query joins that epoch/current organization before any initial link or later run check. Individual `DelegationGrant` remains separate and unchanged in purpose. B tables and source setup ownership are not changed by this sidecar.

Tickets keeps task helper ownership and B source custody/connection setup. Rebrand owns tenant shell/settings UI using these DTOs. Isolated tenant CLI execution, full engine resource/token family authorization, and legacy source entrustment are separately reviewed prerequisites; no member fallback into host commands in this increment.

## Migration and provenance

Forward `m20260908_000012_business_tenancy` safely rebuilds singleton organization storage, preserves IDs, credential hashes, task/source/audit/history and FK/trigger guarantees; migration failure rolls back. Existing organizations start active at epoch1 with default settings. Lossy down is refused. Do not alter applied000009/000010 or owner000011.

Reuse accepted Codeg Apache identity/task/HTTP/Tauri/SQLite code at the base above and existing IntroMail opaque-token adaptation already attributed in NOTICE (`0bd24dfe284b888aa9f602fa1fd00e337ea38874`, auth blob `1faab9205afa2a65c02ff284b12c867c0b85ba99`). Installed SeaORM1.1.19, SQLx SQLite0.8.6, Tauri2.10.2/tauri-build2.5.5/tauri-utils2.8.2, React19.2.4. No Edublend proprietary source port, AGPL/GPL port, new framework or dependency. Application tenancy does not isolate data from the trusted host/platform custodian or malicious same-user filesystem processes.
