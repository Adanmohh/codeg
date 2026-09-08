# Increment B protected setup and access seam — contract, not implemented

Identity reviewer proposal against intake **79a922945668a633ea6b5f7e68f9bdc08a0725a3**,
accepted core **4e64476c7ad9161a5e535b5c75f592716d3ca6e2**, UI plan **99cecbac**
and root BI-1–10 at **650be302**. Root accepted this direction and the explicit
binding-grant policy; the intake owner must reconcile it in the final B contract.
Implementation still needs root dispatch. No endpoint, configuration or passing
B test is claimed here.

## Ownership and authority

The intake owner implements `business_intake::access` and its binding/grant
storage in the single B migration, plus a relative `/intake` router. Reuse
`business_identity::http`'s existing authenticated Principal and shared runtime
registration. Identity needs **no new constructor, token class or permission
enum**. Actual native commands derive the existing operator Principal; HTTP
uses the existing business bearer middleware. Neither may accept caller identity.

Every setup read checks current human identity and `Principal::is_operator()`;
every setup write calls `begin_write` first, then repeats both checks and
`authorize(..., ManageMembers, None)` on that writer transaction. An owner/admin
member credential fails the operator check. Source use separately requires
current domain permission and an explicit grant, even for the protected operator.
The setup capability grants no transcript read, import or publication by itself.

`require_setup(tx, principal) -> Member` and `check_binding(tx, principal,
bindingId, Use)` belong to intake access. `Use` is closed: `Read`, `Import`,
`Triage`, `Publish(Domain)`. Helpers take `&DatabaseTransaction`; writes must
obtain it through identity's `begin_write`. Reads use one read transaction.
`check_source(tx, principal, sourceId, Use)` resolves its exact stored binding
and applies the same checks before returning any private projection. Returned
checked scope fields stay private/non-serializable, and no cached scope replaces
rechecking the current Principal/rows at a later transaction.

## Binding and explicit grant semantics

Binding identity is immutable: organization, kind, source domain, named human
source owner and provider resource. Fireflies resource is verified `user_id` plus
fixed `mine:true`; email is the exact account/inbox; Hafidh is the exact
account/product and trusted host source-configuration identity. All IDs come from
the checked backend resolution below. No first-account/product fallback.

For the smallest B1 policy, a grant explicitly covers **all retained historical
versions, including those captured before the grant, plus current and future
sources imported through this exact binding**. The operator grant confirmation
must state all three parts; the wire scope literal below means this complete
scope. It is not an organization-wide grant or a grant
inferred from a meeting participant, source-owner label, task owner/assignee or
domain membership. New bindings have **zero grants**, including for their named
source owner; the operator must use grant upsert. A narrower per-meeting audience
must use a separately reviewed policy before it is advertised. This makes the
binding-scope choice explicit for reconciliation with the draft's source ACL.

One retained grant row per `(org,binding,human)` has read/import/triage flags,
publicationDomains, expiry, active/revoked state and revision. Every source stores
its binding; disclosure reads the current grant rather than copying an authority
snapshot. Grant revocation therefore blocks old and new source versions.
Historical-version disclosure additionally requires current fresh access to the
source under the current binding. A retained version or archived grant never
authorizes private passages or a saved private draft after access is stale or
denied. Separately reviewed public task text follows its retained publication
policy; source history does not retroactively remove or authorize that text.

Successful setup pins the named source owner's current member revision privately
as `ownerAuthorityRevision`. Every source use and claim activation rechecks that
same immutable member: active human, source-domain Contribute, and revision equal
to the pinned revision. Any drift fails closed, including change-away-and-back;
even a rename may conservatively pause use. Actual operator `bindings/update`
must revalidate the same owner and pin its current revision while incrementing
the binding revision/access epoch. Fresh source access is required afterward.
Changing the owner requires a new binding; no owner, source or grant migration.
Protected setup/status remains available to diagnose and repair the paused
binding. Source ownership itself still grants no private read/import/publication.

Owner member authority and the current request's credential lineage are separate:
revoking the owner's membership or removing Contribute blocks all binding use;
revoking an individual member credential blocks that credential's requests.
The provider key is not assumed to derive from an arbitrary owner login token.
Each requesting human's own original credential is revalidated normally.

Recipients must be active same-org humans with source-domain Read. Import/triage
requires current Contribute; nonempty publicationDomains requires read+triage
and current destination Create authority. Import or triage implies read.
Publication domains are a subset of the binding's explicit publication ceiling
and the recipient's current domains. Agents cannot receive grants. Viewers may
receive read only. Membership/role/domain changes are rechecked on use; stored
grant flags never widen identity authority. Task Create/Assign/edit/reference
checks remain additional requirements.

All writes CAS the binding revision. Binding changes and grant upsert/revoke
increment binding revision and its monotonic access epoch; changed grants also
increment their revision. No counter wraps. Expansion of the binding publication
ceiling does not expand a human grant. Any epoch change invalidates previous
access observations/claims/previews; compare epochs rather than bulk-rewriting
source content. Disable retains rows/history, blocks use and fences in-flight
results. Enable does not restore freshness or grant access automatically.

## Closed wire DTOs

All POST `/api/business/intake/<operation>`, `{input: ...}`, camelCase,
deny-unknown-fields including nested unions, no-store. Native names use
`business_intake_` and slash-to-underscore, over the same core. IDs are UUID
strings unless explicitly marked legacy; revisions are positive integers.
`operationId` is a request UUID, never authority. No raw URL/query/header,
organization, actor, role or credential-reference input.

| Operation | Exact input | Result |
| --- | --- | --- |
| `bindings/list` | `{page?:0}` | `BindingList`; current human may call, including an operator with no grants |
| `bindings/status` | `{bindingId}` | `BindingView`; operator sees setup projection, a granted member sees only safe use projection; otherwise 404 |
| `bindings/create` | `{operationId,label,domain,sourceOwnerId,source:SourceSetup,publicationDomains:Domain[],retainedTaskText:boolean}` | `BindingAdmin`; always initially disabled with no grants |
| `bindings/update` | `{operationId,bindingId,expectedRevision,label,enabled,publicationDomains:Domain[],retainedTaskText:boolean,credential?:CredentialReplacement}` | `BindingAdmin`; replacement optional, omitted means keep; no null/delete/retarget shortcut |
| `bindings/disable` | `{operationId,bindingId,expectedRevision}` | `BindingAdmin`; disabled, epoch fenced, no deletion |
| `grants/list` | `{bindingId,page?:0}` | `{items:Grant[],page,hasMore}`; protected setup only |
| `grants/upsert` | `{operationId,bindingId,expectedBindingRevision,memberId,expectedGrantRevision:null|Rev,scope:"binding_current_and_future_sources",read,import,triage,publicationDomains:Domain[],expiresAt:null|RFC3339}` | `{binding:BindingAdmin,grant:Grant}`; full replacement; null revision means row must not exist |
| `grants/revoke` | `{operationId,bindingId,expectedBindingRevision,grantId,expectedGrantRevision}` | same result; retained revoked row with empty effective rights |

`SourceSetup` is only `{kind:"fireflies",apiKey:string}`,
`{kind:"email",inboxId:positiveInt}` or
`{kind:"hafidh_testflight",productId:existingIdentifier}`.
`CredentialReplacement` is only `{kind:"fireflies",apiKey:string}` and requires
a Fireflies binding. API keys are write-only, bounded 1–4096 ASCII graphic bytes,
never Debug/Serialize/logged, echoed, stored in job bodies or placed in agent env.
Provider identity comes from the fixed verified user query, not an input field.
No legacy provider key can be changed through B.

Label is nonblank plain text, at most 120 characters. Domain lists contain no
duplicates and at most the six existing domains. sourceOwnerId must resolve to
an active same-org human with source-domain Contribute. Publication defaults must
be supplied explicitly as `[]`; nonempty publicationDomains requires
`retainedTaskText:true`. This records authorization for separately reviewed task
planning text to remain after source access ends; it makes no consent/legal claim.
Expiry, when supplied, must be a future UTC instant. Paginated lists have 50 rows.

`BindingList={canManageSetup:boolean,items:BindingView[],page,hasMore}`.
canManageSetup is derived from actual operator transport plus current identity,
not inferred from an empty list or a role. Operator lists include disabled/no-grant
bindings. Member lists filter by current read grant before pagination/counts.

`BindingView={binding:BindingSummary,admin:null|BindingAdmin}` is a response-only
projection: admin is nonnull only for checked setup authority. Member
BindingSummary exposes `{id,kind,label,domain,revision,accessEpoch,enabled,
credentialState:"missing"|"present",capabilities:{read,import,triage,
publicationDomains:Domain[]}}`. Effective use capabilities become false/empty
when disabled, expired or unauthorized. Presence is local storage availability,
not successful provider connectivity. No provider/account/key-reference fields.

`BindingAdmin={id,kind,label,domain,sourceOwnerId,revision,accessEpoch,enabled,
publicationDomains,retainedTaskText,credentialState,resource:ResourceIdentity}`.
ResourceIdentity contains only verified Fireflies providerUserId/mine=true, or
exact legacy accountId/inboxId, or accountId/productId plus an opaque local
configuration identity. No secret reference, endpoint, App key, repository proof
or provider response body. `Grant={id,bindingId,memberId,revision,state,scope:"binding_current_and_future_sources",
read,import,triage,publicationDomains,expiresAt}`; no actor claims in mutations.

## Secret staging, activation and resource changes

Native OS keyring and server private `tokens.json` are **not in the SQLite
transaction**. The existing server mutation lock is process-local. Support the
existing single backend-process credential-writer model; do not claim coordinated
multi-process secret updates or introduce a global store rewrite here.

**Narrow existing-store prerequisite:** at accepted core `keyring_store.rs`,
`read_tokens_at` maps read/parse failures to an empty map and `change_token_at`
then writes that map. Reusing this mutation path unchanged for staged set/delete
could overwrite unrelated credentials. The key-store owner must add a strict
writer-read path within this existing module and under `TOKEN_WRITE_LOCK`: only
a true missing file starts empty; every other read/parse error fails without a
rename, deletion or replacement map write. Intake must use the corrected existing
adapter, not implement a second credential store. This prerequisite does not
claim SQLite/file atomicity or change the process-local locking limit.

1. Authenticate/setup-check; begin writer, revalidate Principal, references,
   expected revision and operation digest. Reserve a bounded setup attempt with
   a backend-generated random immutable reference and expiry, then commit.
   Persist no secret bytes. The old active reference stays active; a new binding
   is not usable. A reservation confers no source grant or publication right.
2. Store the submitted secret at that **new** reference using the existing
   `keyring_store` adapter. Never overwrite an active reference or accept a key
   path/reference from JSON. Run only the fixed user identity query with this
   staged key; malformed/error/timeout/identity mismatch cannot activate it.
   Reject missing/blank/control-containing/oversized IDs and the pinned
   credential test's `auth_failed` sentinel, even in HTTP-200 responses.
3. Reacquire writer ownership and recheck the same live Principal, attempt ID,
   deadline, binding revision/epoch and source-owner reference. On replacement,
   verified providerUserId must equal the immutable old value. Commit activation,
   epoch/revision increment, safe audit and operation receipt together. An epoch
   invalidates all older content-access observations and pending decision fences.
   The credential-store write is staged, not part of this atomic DB statement.
4. On proven pre-commit failure, retire the attempt and best-effort delete only
   its unreferenced staged secret. If commit outcome is uncertain, first resolve
   the operation receipt/active reference under current setup authority; never
   delete a reference that may have become active. Identical authenticated retry
   reconciles the same operation, with no new binding/activation by guesswork.
5. After confirmed activation, old-reference deletion is best-effort and only
   after proving it is not active/referenced. Crashes may leave a protected orphan
   reference; retain a private staged/retired-reference ledger for bounded cleanup
   during later authorized setup. No secret enumeration, background authority
   reconstruction or claim of DB+keyring/file rollback. Missing active secret
   fails closed; never fall back to a previous key or another account.

The provider read is at most **12 seconds** and the whole request budget is
**15 seconds**, preserving the existing 20-second UI deadline. Apply this policy
to credential verification and import advance. Blocking credential I/O must not
stall the request deadline; a late store completion can leave only a staged
reference and must never activate after the expired attempt. Existing source
request/response size caps and no-redirect/no-proxy policy remain. No retry starts
automatically after the caller disappears; a current authenticated human retries.

Kind/domain/source-owner/resource reassociation is not an update. Disable the
old binding and create a new binding with fresh grants; preserve all old sources,
decisions, attempts and receipts under their old binding IDs. No source or grant
is migrated automatically. In-place key rotation is allowed only for the same
verified provider principal. Legacy configuration change similarly invalidates
the recorded association: helpers compare the current host account/resource/
configuration identity before each capture/disclosure, even if B's local epoch
has not changed. That identity must fence away-and-back changes: a reusable hash
of current fields alone is insufficient. The existing host config has no promised
monotonic generation API; its owner must expose that small trusted invalidation
seam before B capture is enabled. Never treat productId alone as unchanged scope.

Legacy binding setup needs a narrow transport-owned resolver: only after actual
operator authentication, obtain the existing Ops account capability and resolve
the requested inbox/product on the same writer transaction. Store that exact
tuple in B; no account1/first-product default in intake. The host owner supplies
a pure checked projection/access helper; member use consumes only the stored
binding plus current grants, never calls `Operator::server`, operator detail,
Pi RunContext or upstream refresh. This resolver may reuse existing Operator
construction **at the verified operator transport**, not in member intake core.

## Decision, claim and UI integration

Task owner extracts `create_in_transaction(&DatabaseTransaction,&ActorContext,
CreateInput) -> Detail` from existing create without changing validation, creator,
reference, activity or human-only checks. The public wrapper opens/commits; the
helper does neither. Intake passes `ActorContext::authenticated(principal.clone())`
from the actual current request. A second task-owned `link_source_in_transaction`
checks visibility/edit/revision, invalidates review and appends only a typed
opaque source-link ID activity, without changing task text or committing.

Intake owns the outer writer: current identity/source grants/epochs/freshness,
exact prepared draft + candidate/source CAS, task helper, immutable decision/link
and operation receipt commit together. An explicit serializable `PreparedTask`
response must resolve ownerId; existing CreateInput is an input type. Accept
contains only expected revisions and the confirmed destination. Private passages
are never a public task payload or agent authority.

Support UI's small completion seams: `candidates/select` saves exact current
passage IDs and explicitly rebases while preserving a null or existing draft;
`imports/list` rediscoveries use current binding import grants, not stored caller
credentials; CandidateDetail exposes its terminal decision with inaccessible
task ID/revision redacted. A different authorized human may resume; original
requester remains audit history, not an impersonated Principal or automatic right.

A detail refresh also needs a **source-scoped access/attempt fence across imports**.
Invalidate freshness and advance that fence before I/O; final commit compares it
as well as the import attempt/lease and content revision. Otherwise two distinct
imports can have individually valid leases while the older response overwrites
the newer observation. List discovery alone never grants detail freshness.
Source A→B→A content versions remain distinct; a stale network response is not a
legitimate new source observation. Expired/retired attempts cannot commit even
without a replacement worker. Receipts reconcile completed operations only after
current access; operationId is not a resume/execute credential.

## Evidence required at implementation

BI-1: actual protected operator setup/list/status, explicit own/other-human grant,
member-owner/agent setup denial; staged-store failure, DB failure, ambiguous
commit, crash/late secret write, same-provider rotation and changed-provider
rebind. Revocation/disable during awaits must fence activation and disclosure.
Synthetic corrupt/unreadable-store set and delete must preserve the original
bytes and unrelated entries; genuine missing-file creation remains supported.
Owner revision drift, loss/restoration of Contribute and membership revocation
must block use until protected revalidation plus fresh access. A newly granted
human can see retained versions only within the explicitly confirmed binding
scope while current access remains fresh; revocation hides the entire history.
BI-2–3: bounded 12/15/20 deadlines, competing/reclaimed claims, source-scoped late
response, duplicate/response-loss recovery and a different currently authorized
human resuming without old credential reconstruction. BI-4–7: explicit audiences,
public task versus private source, permission ceilings, retained/null draft rebase,
atomic create/link/discard, revoked and A→B→A cases. BI-8–10 retain existing task
continuity/floors, actual protected two-session CLI and final runtime/artifact
gates. These are requirements, **not passing tests**.

## Read sources and provenance

Read accepted `business_identity/{mod,http,types}.rs`, native identity commands,
`business_tasks/{mod,store,policy}.rs`, `ops/{mod,email}.rs`,
`ops_intake_host/{operator,runtime}.rs` and `keyring_store.rs` at core4e64476c.
These are existing Codeg Apache/approved IntroMail-derived boundaries; retain
their NOTICE mappings. No third-party code is copied by this document.
The strict writer-read prerequisite is pinned to `keyring_store.rs` blob
`29fc3fb38280338aa26939c45f80ef9aefc2a394` (also verified by the intake owner
at `c7f7366f`), specifically the existing read/mutate paths at lines 80–117.
Installed SeaORM1.1.19 transaction and SQLx SQLite0.8.6 transaction sources were
read before proposing the writer seam. No library upgrade or new crypto.

Official Fireflies adapter **fbd24607bc784a2294ce402426aefe2cb8c00f50** was read
via immutable `gh api`: credential query/header blob
`a39bced68aeceb23b33641e774e3509554caac49`, query blob
`c775b8bbe6c0bbb16f8f7e8a60467c248ad93859`, transport blob
`0b60c1c9e9d61f4583298cd2c0c464e9864861f5`, MIT LICENSE.md blob
`1e4b3a6e245384b89f24f2aef5e3f8e7fa1f4d23` (Copyright 2022 n8n).
Future query/error ports must retain that full notice and exact attribution;
the n8n runtime, mutations and raw error disclosure are excluded. This proposal
does not certify live provider access/semantics, multi-process secret-store
atomicity, malicious same-user process isolation or legal consent.
