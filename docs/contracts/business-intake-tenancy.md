# Intake tenancy integration

Tickets owns this additive implementation contract. It applies the accepted
tenancy architecture `af00c956` and PR30 handoff
`b3dfbcb602314cee6eb0039d92cb997762c02dd8` to intake contract `670af9ca`
and access `18be55ed`. The older contracts' operator-only Fireflies setup rule
is superseded below. Their private/public text separation, explicit grants,
exact task review and original credential revalidation remain unchanged.

## Current setup authority

- Fireflies connection creation/rotation, labels, enable/disable and grants require
  an authenticated current human owner/admin, using existing
  `Permission::ManageTenantSettings`. The source domain also requires current
  Contribute; every publication destination must be within that human's Create
  ceiling. The referenced source owner remains an active same-tenant human with
  Contribute and an exact pinned member revision. Setup never grants source read.
- A tenant credential cannot associate an arbitrary legacy inbox/product or obtain
  its setup projection. Existing legacy setup remains actual operator transport
  in the explicitly mapped original organization. An owner-role credential is
  not platform authority. Platform entrustment into newly provisioned tenants
  remains a separate, not-yet-exposed operation; no implicit original-org fallback.
- Binding identity (tenant/kind/domain/owner/resource) stays immutable. Rotation
  must validate the same provider user. Unique staged references, bounded cleanup,
  strict existing store writes and zero initial grants remain mandatory.
- Every grant mutation checks the specific binding's current setup scope before
  receipt replay or response projection. Grant recipients are current permitted
  same-tenant humans; agents receive no source rights. Source membership, source
  ownership and task-domain access do not imply a private-source grant.

`BindingList` adds only response fields:

```ts
setupKinds: ("fireflies" | "email" | "hafidh_testflight")[]
setupDomains: Domain[]
```

`canManageSetup` is true only for a nonempty permitted setup scope. Tenant
owner/admin kinds are only `fireflies`; actual original operator may manage all
three. The UI uses these arrays instead of role names or an empty-list inference.
`BindingView.admin` is nonnull only when the caller can manage that specific
kind/domain. Existing summary source-use capabilities still require an explicit
current grant and ready binding. No new request fields or operations are introduced.

## Lifecycle and durable evidence

Private setup/import/source/candidate rows gain nullable `authorization_epoch`.
The captured value comes only from `Principal::authorization_epoch()`, never
JSON, a current-org lookup after I/O, or a default. No setter/refresh constructor
is introduced.

| Record | Capture and later use |
| --- | --- |
| Setup | Reservation captures the original request epoch. Activation revalidates that same Principal, persisted epoch, owner revision, deadline and binding CAS. Old/NULL stages cannot activate after fresh login; bounded authorized cleanup retires only proven unreferenced keys. |
| Import | A new human advance captures its epoch in the durable attempt. Final commit keeps that Principal and checks epoch, attempt UUID, lease, binding epoch and source-wide refresh fence. Old/NULL running attempts appear waiting to a new current login; explicit advance makes a new fenced claim. |
| Source | Only a validated detail commit captures observation epoch. Lists and fresh authentication do not refresh it. Old/NULL observations disclose authorized labelled metadata only, with passages/drafts withheld until an explicit authorized detail refresh. |
| Candidate | Seed/create/select/edit captures preparation epoch. An identical-content refresh can make source access fresh but does not bless the old preview. Explicit select/edit CAS rebases it, preserving the reviewed draft/null. Accept/link checks both source and preparation epoch in the writer. |

Suspension denies authentication/authorization; resumption increments epoch again.
Old Principals remain invalid. Durable binding/grant policy and history are
retained, not recreated; a newly authenticated human must satisfy all current
identity, source-owner, binding and grant checks. Retained grants alone cannot
revive captured processing work, source freshness or a prepared decision. Revoked
grants/credentials remain revoked. Source history does not remove independently
published, human-reviewed public task text.

Terminal receipts remain historical results and require current access on replay.
They do not re-execute setup/import/task creation or republish withheld content.

## Both installation paths

Root reserves `m20260909_000013_business_intake_epochs`, registered after000012.
Accepted000012 is unchanged. Existing000011 tables and values are preserved;
000013 adds nullable columns in one SQLite writer transaction, checks foreign
keys and uses per-column existence checks for schema-commit/receipt retries.
It never backfills an epoch. Its lossy down operation is refused.

SeaORM1.1.19 selects pending migrations by name, in registry order; it does not
skip newly introduced000011 merely because000012 is recorded. Therefore:

1. Fresh combined install runs000011→000012→000013.
2. Old populated000011 runs pending000012→000013; historical epochs remain NULL.
3. A-only already-recorded000012 runs newly pending000011→000013, retaining the
   original000012 receipt and original organization/task/credential history.

New000011 also records an atomic private schema-completion marker so losing its
separate SeaORM receipt does not cause a destructive rebuild on retry. This
does not alter already recorded migration receipts or assign authority to old data.

## Native and fixture limits

All25 native command wrappers use the accepted invoking-window `NativeSessions`
resolution and optional top-level `session`, then the same bounded core as HTTP.
Legacy Operator construction remains only after actual operator identity. The
prepared tenant ACL is not broadened and `tenantWindowAvailable` stays false;
these wrappers are not a native tenant-isolation acceptance claim.

No new dependency, runtime, agent source tool or host terminal surface is added.
The owned synthetic fixture will use one integrated database for identity,
settings, tasks and intake, fixed loopback provider reads and memory-only provider
secrets. Actual deployed-provider semantics and native restricted-window IPC remain
outside the synthetic evidence. Executed gates and exact heads belong in
`reports/business-intake.md`.
