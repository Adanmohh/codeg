# Business task → AI session → managed deliverable → human review

Status: proposed implementation contract for bounded review, 2026-09-09. No new execution API,
tenant runtime, managed asset store or social connector is implemented by this PR.
This is the previously required shared workspace flow, with the owner's managed
documents and connected-account clarification included. Governing product map:
`docs/BUSINESS-WORKSPACE.md` at `f30a27da4`. Authority baseline:
`docs/contracts/business-tenancy.md`, accepted architecture `af00c956`, and the
existing private `Principal`/captured-epoch boundary. No member-to-host widening.

## First complete flow

1. Open an authorized business task beside its current sources/account facts.
   Connected accounts must come from actual adapter reads with timestamps and
   stale/unknown/error states; a configured credential is not connected-state proof.
2. Choose an installed, configured, permitted client/profile. Show its real model,
   readiness, credential custody and available chat/terminal modes. Keep the
   accepted Astra/max guard; no login/install/inference or cheap fallback on open.
3. Start or continue a persistent session in the same tabs/split panes as the task.
   Backend creates the non-Git workspace; the user never has to create an engineering
   project or a `work_task`. Reconnect attaches to the authorized session.
4. Prompt, inspect output/tool activity, stop or continue. Session transcripts and
   intermediate files remain private to their authorized session audience.
5. Select real generated documents/slides/assets. Backend imports immutable bytes
   into managed storage, verifies them, assigns version IDs and records source
   session/turn, producing client/model, task and author/requester provenance.
   Neither a textarea nor an agent-reported local pathname is the deliverable.
6. Human explicitly submits selected asset versions to the existing task review.
   Exact bytes/revisions, disclosure and task CAS are checked together. Reviewer
   accepts/returns through the existing human policy; generating or publishing an
   asset does not approve the task or authorize an external post/spend/schedule.

## Verified reuse and required glue

Source baseline is accepted Codeg fork `f30a27da4` (product unchanged from
`cba079c75`); inherited Codeg v0.30.4 is Apache-2.0, original pin
`6f6bd648b206412644842a98d9ffeebf57292bed`. All paths below were read locally.

| Existing seam | What it actually supplies | Required narrow adaptation |
| --- | --- | --- |
| `commands/conversations.rs:create_chat_dir_core/create_chat_conversation_core`; `db/service/folder_service.rs:add_chat_folder` | Hidden chat folder/conversation, dated scratch directory, no Git branch. Existing test `create_chat_conversation_core_creates_dir_folder_and_conversation` covers that behavior. | Server-owned resource binding and recoverable creation; do not accept a caller `existing_dir` as authority. Scratch storage is not managed deliverable storage. |
| `acp/manager.rs:spawn_agent/send_prompt_linked_with_message_id`; `acp/connection.rs:spawn_agent_connection` | Existing process, ACP stream, prompt lock, reconnect and teardown; first persisted prompt needs a folder/conversation row. | Scope session/launch receipts and credential/profile lineage. Do not manufacture a Git task to satisfy the conversation FK. |
| `commands/acp.rs:build_session_runtime_env`; `web/handlers/terminal.rs`; `terminal/manager.rs` | Original host's configured clients and terminal. Runtime setup includes host settings/Git credential environment; terminal strips CODEG_TOKEN but is not a sandbox. | Original-operator reuse only until explicit profile/process/file isolation is implemented. No tenant use of the generic routes. |
| `acp/delegation/listener.rs:TokenEntry/process_session_info` | Per-launch parent token; current session-info explicitly accepts any valid token for any conversation in the original single-user trust model. | Typed operation-family and resource enforcement at dispatch, not hidden MCP tools. Children may only narrow authority. |
| `business_tasks/store.rs:submit/review`; `business_tasks/agent.rs` | Revisioned text deliverable and immutable activity in a writer transaction; protected indexed engineering-run provenance and original DelegationGrant. | Task-owned transaction helper for immutable asset-version references; new session provenance must not forge existing engineering execution IDs or refresh a stale Principal. |
| `components/message/reply-artifacts.tsx`; `lib/session-files.ts`; `lib/office-actions.ts` | File hints parsed from tool output, local file opening, office skill shortcuts. | Hints remain untrusted discovery. Managed bytes, versioning, access-controlled preview/download and review links need backend records. No claim that a skill shortcut means OfficeCLI is installed/usable. |

## Closed delivery slices and ownership

**E1: original-operator vertical slice.** Tickets owns narrow backend admission,
session binding and managed-output/task transaction glue; rebrand owns task entry,
real session pane and asset/review UI; approvals reviews authority and exact-byte
publication. Require the real transport-derived original operator, mapped original
organization and current identity checks. An owner-role member credential is not
that operator. Existing configured client/profile remains in its original
authorized custody. No tenant selector, credential copying or implicit host grant.
Synthetic proof must run an actual local ACP/PTY fixture process, produce a managed
document plus a second version, reconnect, cancel and submit exact bytes to review
through the protected API and actual Playwright CLI. No paid/provider call is needed.

**E2: tenant execution boundary.** Tickets implements the same typed business
session operations with captured tenant/member/credential/profile/generation,
closed events/files/tools and a reviewed isolated execution profile. Approvals
owns independent two-tenant boundary review; rebrand reuses the same pane UI.
Process identity/filesystem/profile custody must prevent arbitrary CLI access to
host or another tenant. Merely changing cwd/HOME/env or running another same-user
process is insufficient. Native tenant windows remain unavailable under the
accepted Tauri channel boundary; browser/server isolation must be proven separately.
Synthetic proof must attempt foreign files, session events, tokens, child delegation,
revoked credentials and suspend/resume, with a positive owned stream and managed
result. Profile admission and wire details are specified below. There is no
automatic fallback to E1.

**E3: connected-account read projection.** Tickets owns a fixed provider read
adapter and revision/freshness contract; rebrand owns actual account/status views;
approvals reviews account grants and credential isolation. Prefer an official
MCP/CLI when verified coverage/auth/failure semantics suffice, otherwise fixed
official API reads. No arbitrary MCP server/command/URL from a task or agent.
The first adapter needs an immutable official source pin before implementation.
Synthetic proof covers two accounts/tenants, pagination, denied/expired access,
last-good retention, reordered refreshes and no write calls. Live accounts remain
unclaimed. This does not delay E1's managed deliverable, and is required for the
larger connected-marketing scenario; no fake enabled social controls.

## Admission, custody and visibility

E1 is an original-operator capability, never a tenant-owner capability. Its new
business operations require `Principal::is_operator()` AND current `authorize`
against the mapped original organization; platform control does not construct a
tenant Principal. No organization selector changes this. New members see the
honest tenant execution readiness state; they never receive the host route/token.
This check applies to **every E1 profile/session/attach/event/terminal/private
output/asset operation**, not just start. The sole public-result route is the
task-owned published-version projection specified below; it exposes no session or
private-asset operation. HTTP derives the operator from the existing protected
transport marker. Native derives it only from the actual allowed platform/main
window. Submit/review actors remain the existing human Member returned by current
task authorization; an owner-role member is not upgraded to operator. E1 records
`authorityKind: operator` and the mapped original owner's ID/display name: a shared
operator token does **not** establish which individual person used it. A separate
human member reviewer is attributed to that actual credential-derived member.
The original host remains a trusted single-user environment, not an OS sandbox.
The existing engine can run under that already-authorized operator, but the new
adapter must select only the chosen existing client/profile. Do not run the
generic all-settings runtime builder or inject the global Git credential helper
for a plain business chat. No copying login files, changing global config, or
reading/reporting credential values. Inherited host tools outside the application
remain outside the app's approval guarantee; E1 must state that scope honestly.

E2's first eligible profile is a **pre-provisioned single-principal execution
environment**: one tenant, one human custodian, fixed supported client and fixed
capabilities. Reuse the existing Rust ACP/PTY managers in a restricted worker role
of the same binary; no replacement scheduler or new framework. The control server
holds business state and managed storage. The worker cannot read its DB, original
operator sockets, other profiles, arbitrary shared mounts or asset-store secrets.
Require a separately provisioned execution host/VM or independently verified OS
boundary; launching a second process under the current desktop user fails this
requirement. This contract does not claim that any such environment is installed.

Only the platform custodian can register that environment's opaque transport and
isolation evidence; current tenant owner/admin separately authorizes its tenant,
member and domains. Neither side alone grants execution. V1 profiles are private
to their human custodian, not a shared shell account for different members.
Client authentication stays in that authorized client/profile; no main-host
credential inheritance. Every supported client needs verified isolated profile,
transcript/session-directory and subprocess behavior. Its approved home/mounts
must not expose other sessions or domains; if native client history/auth cannot
be separated safely, that client is unavailable for E2. A configured credential
does not prove this isolation. Fixed worker transport authentication authorizes
only claimed business sessions, never the legacy server API. Setup/provisioning,
network egress to the selected model/verified read adapter, filesystem and child
process restrictions need actual synthetic proof before a profile is enabled.
No infrastructure, login or credential action is authorized by this document.

Session start/continue requires a current human Principal, task Read and the
task-owned `submit` capability (involved contributor or permitted manager), plus
the current profile grant. Viewers cannot launch/prompt/write a terminal. Task
assignee need not be an agent: a human can use AI as an assistant. Creator,
accountable task owner, session requester, client producer and human reviewer stay
distinct. Do not change task assignment or status just by opening/prompting.

Session transcript, terminal and intermediate files are private to the requesting
human and that exact authorized execution; task Read alone does not disclose them.
Managers may stop an execution using current Assign authority without receiving
its private output. V1 does not grant another member interactive session access.
Sharing happens by explicitly publishing selected managed versions to the task's
current domain audience. Published versions become readable only through current
task Read; no public URL or transcript dump. Export uses the same permission.

An agent's application tools are limited to its selected task/input references,
its own session/output candidates and optional task contribution. Agent identity
is a pre-existing active permitted agent member when such contribution is enabled,
not `AgentType`, a CLI argument or requester text. Persist the original opaque
`delegation_grant` only in the authorized admission writer. Generalize the
task-owned live-proof seam with a validated business-session variant; preserve the
existing indexed engineering-run variant and its source entrustment. No fake
`work_task`, second Principal or operator reconstruction. After trusted parent /
generation / profile / task binding checks, reuse `agent_principal_from_binding`
with the original grant. Agent tools never approve/review, reach terminal done,
change roles/assignments, grant access, publish externally or mint source freshness.
Stable pi policy identity and the destructive human floor stay unchanged; no
`allow_for_session` on mutations. Human-directed E1 needs no agent-role mutation.

## Proposed closed wire contract

These are **new** operation names/types to implement, not existing API claims.
Use the current `{input: DTO}` POST convention under `/api/business/execution/`
with the existing Principal middleware; native original-operator commands call
the same core. No member route may fall back to native operator authority.
All objects reject unknown fields. IDs are opaque UUID strings; revisions are
positive safe integers; timestamps are UTC RFC3339; event cursors are opaque
strings (not lossy JavaScript u64 numbers). Null fields are explicit, never an
implicit current/first profile, task, account or epoch. `operationId` is required
for side effects and scoped to tenant + original actor + operation kind.

| Operation | Exact input | Result / behavior |
| --- | --- | --- |
| `operations/get` | `{operationId,kind:"start"\|"continue"\|"prompt"\|"attach"\|"stop"\|"terminal_write"\|"import_output"\|"submit"}` | `{operation:OperationSummary,resourceId:null\|string}`; rediscover this actor's receipt after a lost response, with current operation/resource authority. Unknown ID is missing; no inferred retry. |
| `profiles/list` | `{taskId}` | `{profiles: ProfileSummary[], unavailableReason: SetupReason\|null}`; only profiles this actor may use. Metadata probe only, no launch/install/authentication attempt. |
| `sessions/list` | `{taskId,cursor:null\|string,limit:1..50}` | `{items:SessionSummary[],nextCursor:null\|string}`; this human's sessions, including stopped/failed history. |
| `sessions/start` | `{operationId,taskId,expectedTaskRevision,profileId,expectedProfileRevision,mode:"chat"\|"terminal"}` | `{session:SessionSummary,operation:OperationSummary}`; one admission, backend-generated workspace/connection. No cwd/command/env/model/agent-member parameter. |
| `sessions/get` | `{sessionId}` | `{session:SessionSummary}` plus current capabilities; no implicit process launch. |
| `sessions/continue` | `{operationId,sessionId,expectedSessionRevision}` | Same receipt/result envelope; attach if already live, otherwise resume the stored client session only if supported and still authorized. Unsupported history is visible, not silently replaced with a new conversation. |
| `sessions/prompt` | `{operationId,sessionId,expectedSessionRevision,text,inputs:InputRef[]}` | Receipt with exact normalized message ID/content hash; text1..32000 Unicode characters, at most16 explicit input refs. Inputs default to no context only when the caller sends `[]`. No arbitrary file/URL/resource URI. |
| `sessions/stop` | `{operationId,sessionId,expectedSessionRevision}` | Stops current generation; records cancellation request and then observed stop/uncertain state. Closing a tab only detaches. |
| `sessions/events` | `{operationId,sessionId,expectedGeneration,cursor:null\|string}` | Read-only attachment receipt plus authorized snapshot/replay/stream using the existing attach ordering. New scoped transport has no global event subscription and never emits a raw host snapshot. |
| `sessions/terminal-write` | `{operationId,sessionId,expectedGeneration,data}` | At most16KiB UTF-8, only for this owner's terminal-mode session. Duplicate operation cannot write twice; uncertain acknowledgement is not automatically resent. It is real shell input within the permitted profile, not a claim of semantic command safety. |
| `sessions/terminal-resize` | `{sessionId,expectedGeneration,cols,rows}` | Idempotent latest size; cols2..500, rows1..200, same current ownership check. No terminalId supplied by caller. |
| `outputs/list` | `{sessionId,cursor:null\|string,limit:1..50}` | `{items:OutputCandidate[],nextCursor:null\|string}` from fixed workspace inspection; no implicit publication. |
| `assets/import-output` | `{operationId,sessionId,outputId,expectedOutputRevision,title,assetId:null\|string,expectedAssetRevision:null\|number}` | `{asset:AssetSummary,version:AssetVersion,operation:OperationSummary}`. Null/null creates; ID/revision adds a version to an owned asset for the same task. Backend reads bytes; caller never supplies a storage path/hash/author. |
| `assets/list` | `{taskId,query:null\|string,cursor:null\|string,limit:1..50}` | Private owned drafts plus versions already published to an accessible task; title/type search, stable pagination, no hidden counts. |
| `assets/get` | `{assetId,versionId}` | Immutable version metadata/current capabilities. Version ID required; no moving `latest` in review. |
| `assets/content` | `{assetId,versionId,disposition:"preview"\|"download"}` | Authorized bounded byte response, safe filename/type headers; no storage path or bearer-in-URL. Preview may explicitly be unavailable for that format/profile. |
| `assets/submit` | `{operationId,taskId,expectedTaskRevision,versions:{assetId,versionId}[],body}` | `{detail:TaskDetail,operation:OperationSummary}`; 1..16 distinct retained versions, optional empty body allowed for file-only submission (otherwise≤20000 chars). Atomic publication + task submit + activity. |

Public human review uses two **task-owned** additive operations outside E1's
private execution subtree: `/api/business/tasks/deliverables/assets/get`
`{input:{taskId,deliverableId,assetId,versionId}}` and `.../content` with the same
fields plus `disposition:"preview"|"download"`. They require current Principal /
task Read and an exact committed deliverable↔version reference in that tenant.
They return only the selected version metadata/bytes, never unpublished versions,
the asset's private latest version, session history or unrelated references.
Thus a named human reviewer can inspect an E1 result without an operator credential
or a manufactured tenant Principal. Existing `tasks/review` provides the actual
human accept/return operation with its current Review policy and audit attribution.
New `assets/submit` additionally requires `MemberKind::Human` in the writer, even
when the existing `submit` capability is true. Existing agent-capable text submit
is unchanged. A producing agent cannot use a parent token or precomputed selection
to perform human publication. Assert rejection leaves task/assets/receipt/audit
unchanged in the negative control.
Add only `assets: PublishedAssetRef[]` to each existing task `Deliverable` returned
by `tasks/get`/mutation Detail; historical text deliverables return `[]`.
`PublishedAssetRef = {assetId,versionId,title,mediaType,byteSize,sha256}` describes
the exact disclosed version. Title/type are snapshotted for that version, not the
private asset's current label. This lets the real review UI rediscover its files
after reload without entering E1's private execution API. No private asset count,
latest-version ID, client profile or transcript is added to TaskDetail.

`ProfileSummary = {id,revision,label,clientId,modes:("chat"|"terminal")[],custody:"original_operator"|"isolated_member",model:null|{id,reasoning},readiness:"ready"|"blocked",reason:SetupReason|null,capabilities:{start,continue,managedOutput,officePreview}}`.
`SetupReason = missing_client | missing_configuration | model_unavailable | profile_unavailable | tenant_execution_unavailable | native_boundary_unavailable | client_resume_unsupported`.
No model/profile credential/config dump. Pi specifically requires installed0.85.1,
adapter2.32.1 and actual gpt-6-astra/max discovery, preserving `acp/pi_desk.rs` /
`integrations/pi-desk/launch.mjs`. An explicit different supported client choice is
not an automatic model fallback; report its actual configured readiness separately.

`SessionSummary = {id,taskId,profileId,profileRevision,revision,generation,status,mode,title,createdAt,updatedAt,lastActivityAt,capabilities:{read,prompt,continue,stop,terminalWrite,importOutput},reason:null|SetupReason|"authority_changed"|"launch_uncertain"|"process_gone"}`.
Statuses: `starting | idle | running | awaiting_input | stopped | failed | interrupted | revoked | closed`.
No raw connection ID, absolute cwd, external client session ID or grant. Durable
private binding holds these plus org/requester/original credential lineage,
captured authorization epoch, immutable task scope, profile revision and live
generation. This is an admission/resource record around the existing runner,
not a second task engine. `OperationSummary = {id,status:"pending"|"confirmed"|"failed"|"uncertain",reason:OperationReason|null}`.
`OperationReason = invalid | forbidden | missing | conflict | busy | setup_required | authority_changed | cancelled | launch_uncertain | prompt_uncertain | content_changed | content_unavailable | transport_unavailable | rate_limited`.
Errors never contain provider bodies, SQL, file paths, arguments or credentials.

`InputRef` is a closed tagged union: `{kind:"asset",assetId,versionId}` or
`{kind:"task",taskId,expectedRevision}` or `{kind:"account_snapshot",snapshotId,expectedRevision}`.
V1 passes reviewed public content only. Backend resolves current Read, same tenant,
exact revision and permitted destination domain; source proof/config/grant objects
remain private. Intake private transcript excerpts are not automatically included.
Adding an intake reference requires the accepted source publication helper, not
a new interpretation of a read grant. Source scope/status is visible in the prompt
preview; stale account facts are explicitly labelled rather than silently freshened.

`OutputCandidate = {id,revision,name,mediaType,byteSize,modifiedAt,status:"available"|"changed"|"unsupported"}`.
`AssetSummary = {id,taskId,revision,title,mediaType,latestVersionId,createdAt,updatedAt}`.
`AssetVersion = {id,assetId,version,sha256,byteSize,mediaType,createdAt,createdBy:{memberId,displayName},producer:{sessionId,turnId:null|string,clientId,model:null|string},visibility:"private"|"task",reviewReferences:{taskId,deliverableId,taskRevision}[]}`.
The current task must be readable before any review reference is returned.
Client/model/turn provenance is backend-observed; `null` means unavailable, not
agent-asserted truth. Store source/input reference lineage privately, return only
permitted public provenance. Task review uses the existing `tasks/review` CAS and
human policy; no second asset-approval endpoint or agent review capability.

## Durable files and exact publication

Managed assets are organization/task-scoped records plus retained immutable bytes
under service-owned storage, separate from session scratch/output. Minimum first
slice: real Markdown/text document preview and a generated deck/file import,
retained versions, task library/search, download and exact-version review. Retain
DOCX/PPTX/XLSX/PDF and image bytes as supported outputs; report preview support per
format. Existing office actions are prompt shortcuts, and OfficePreview starts an
OfficeCLI watch process. Neither counts as a managed library. Reuse its presentation
only after a version-scoped proxy/reader has current authorization; never expose
the legacy path/port watch API to members. No auto-install or renderer/active HTML
from untrusted files in the app origin. If the necessary renderer is absent, show
the actual format/size/version and authorized download plus a clear preview gap;
do not claim successful slide rendering. Preview must block external subresource
fetches and scripts that can access the application origin, credentials, forms or
unrelated URLs; a legacy iframe's permissive flags are not this authorization.
Planned full library organization,
comments, sharing and campaign associations are broader product obligations;
initial same-task versions/review links must work now, not become untracked files.

Treat agent file reports as hints. Enumerate only the bound workspace with bounded
depth/size; no arbitrary URLs, absolute paths, traversal, symlinks, hardlink aliases,
special files, client-profile/config directories or external file-following. Use
the existing `upload_jail.rs` anchored-FD/no-follow create/rename pattern where
applicable; its documented Windows/reparse/root-validation limits are not a
cross-platform sandbox proof. Asset reads/copies also need descriptor-based source
checks, stable identity/size validation and tests against swaps during copy. Limit
one output to50MiB and a publication to16 versions initially; stream bytes and hash,
do not buffer arbitrary files or unpack untrusted archives to shared paths.

Import protocol: reserve an operation and output revision under `begin_write` /
current authority; stage bounded bytes to a unique private file outside the writer;
hash/validate and durably finalize an immutable storage object; then re-enter the
writer, revalidate original captured authority/profile/output claim and commit
asset/version/provenance/audit plus receipt. Rejection leaves no visible version.
FS and SQLite are not one atomic transaction: finalized-but-unreferenced objects
are private recovery candidates. Never delete a referenced object; retry same
operation reconciles its one staged object. On restart a missing/corrupt retained
object is unavailable, never replaced from the mutable original pathname. Backup
and restore must include verified asset objects and references; no backup-complete
claim based on the DB alone. No edits to applied migration history; root reserves
new additive migration names when implementation is dispatched.

Publication is a separate explicit human action showing exact selected content,
file versions and current task-domain audience. Within one task-owned writer:
recheck current actor/domain/reference access; validate every immutable version
and disclosure lineage; CAS the task; insert immutable deliverable↔asset-version
references; add the existing submit activity and operation receipt. Roll back all
DB changes on any failure. Old text-only `tasks/submit` remains compatible. Do not
stuff URLs into its body and call that managed output, or label a human submit as
an engineering-run contribution. New versions do not change prior reviewed bytes;
review/replay never follows `latest`. Changing task scope/assignment or required
source grants invalidates pending publication until explicitly revalidated. Same
operation + same normalized payload replays after current access checks; same ID
with different versions/body conflicts. No external publish/share/send is implied.

## Session lifecycle, races and transport

Admission uses the current writer transaction for Principal/profile/task/grant
checks, revision CAS, durable operation reservation and audit. Capture epoch and
exact original credential lineage once; never authenticate again after spawn/copy
to rescue the operation. Resolve/recheck backend live generation under the engine
lock before committing a connection binding; no caller-owned root/agent claims.
The existing first-prompt lock and snapshot/replay ordering are reused. Since
`create_chat_conversation_core` currently uses compensating folder cleanup, new
admission must explicitly reconcile its folder/conversation/claim instead of
claiming that helper supplies one atomic launch transaction.

Persist original lineage as the existing opaque grant's **Operator versus
Credential** distinction: operator flag + mapped original human/org + captured
epoch, with credential ID null; or the original member's exact credential ID,
human/org and captured epoch. Never invent a human credential for E1, replace the
ID on login, or serialize a Principal into caller data. Recovery authenticates its
new transport request normally, then checks the retained lineage/epoch unchanged;
it cannot mint authority from the stored row. E1 recovery still requires the real
operator on every operation. An agent restore uses only the accepted trusted
binding helper, after its live proof. Audit records producing client/model
separately from the named transport-derived human submit/review actor.

| Durable operation | Receipt and retry rule |
| --- | --- |
| Start / continue | Reserve one operation + session/generation claim before spawn. Confirm only after the exact owned connection is bound. Lost DB acknowledgement reads the receipt; unknown child outcome remains uncertain and blocks replacement launch until ownership/teardown is reconciled. A retry never selects another profile. |
| Attach | Persist read attachment identity/expected generation, no transcript in receipt. Repetition can replace the same caller's stream after fresh current checks, never start a process or add duplicate forwarders. A committed old receipt grants no replay after revocation. |
| Prompt / terminal write | Reserve exact normalized content hash and message ID before dispatch. Confirm only on the observed runner acceptance. If dispatch may have happened, show uncertain; never retransmit automatically. Different content with the same ID conflicts. |
| Stop | Idempotently fence the claimed generation before teardown. Confirm only after owned process/turn stop is observed; late results cannot bind/publish. A repeated stop does not stop a newer generation. |
| Import | Unique staged object belongs to one operation/output revision. Confirm only with committed asset version/provenance. Retry reconciles that same object; changed bytes/revision conflict, and finalized private orphans are not exposed as assets. |
| Submit | Exact selected version IDs/hash manifest, task revision and body define the operation. Publication, task CAS, deliverable references, audit and confirmed receipt commit together. A reply lost after commit replays current authorized result without a second submission. |

`confirmed` describes this local operation only, not completed AI work or an
external provider action. Failed means a known refusal/failure; uncertain never
means safe to resend. Receipt lookup rechecks current access and withholds private
results when authority changed. Keep durable receipts for the retained session /
deliverable lifetime; do not evict them merely when a tab closes.

Process spawn and DB commit cannot be atomic. Persist a reserved launch identity
first; tie child/connection ownership to that identity, not a bare reused PID.
Cancellation during spawn revokes the reservation/token, kills/reaps the owned
process tree and denies late binding. A crash/unknown spawn outcome stays
`interrupted`/`uncertain`; recovery checks the owned runner handle and never blindly
launches or resends a prompt. Repeated operation IDs cannot launch twice. Prompt
receipts preserve the normalized exact message ID; existing UI echo de-duplication
is not by itself a durable no-resend guarantee. A busy turn conflicts without
discarding the user's draft. Fresh reload lists durable sessions and attaches the
stored client session only when supported; ephemeral terminal processes cannot
magically survive a server restart, and must show ended/recoverable state honestly.

Stop increments/fences generation before waiting on external teardown; late output
may remain private diagnostic history but cannot publish/contribute. Authority
loss (tenant suspend, original credential/member revoke, assignment/domain
change, cancelled/archived task, revoked profile) closes event/file/tool access and
requests owned process-tree teardown. Reopening/changing away-and-back or a fresh
login does not rewrite captured epochs or revive the old grant. Normal stop may
continue as a new generation under still-current original lineage. Authority loss
requires a new explicit admission; retained reviewed assets can be selected only
through current Read. Do not auto-attach stale native client history to that new
admission. Live revalidation and lifecycle cancellation must cover buffered output
and admission races; UI capability caching does not authorize delivery.
Use the task-owned revocation hook to irreversibly revoke these new session
bindings when assignment/domain/cancel/reopen authority changes. Do not merely
compare the final assignee/domain strings: changing away and back cannot restore
an old binding. Preserve the current task policy and engineering revocation path;
this is an additive resource revocation, not a new role policy.

Use a new authenticated business event route with closed session/generation
messages; no global `web/ws.rs` subscription. Reuse `ws_attach.rs`'s ordered
snapshot/replay decision under the session read lock, then project permitted
fields and check current subscription authority before delivery. Detach after
revocation and discard unsent buffered payloads. File content follows the same
current check before opening and throughout bounded streaming; revocation closes
the stream and forbids new range/preview/download requests. Already delivered
bytes cannot be recalled; no claim to erase a prior authorized human export.
Transport reconnect cannot switch
tenant or profile using a supplied connection ID. Private tabs/pane state keys
include server + organization + member + session; no credential/private prompt in
layout persistence. Same-scope remount/locale/split moves preserve draft buffers;
explicit logout/switch clears private rendered data and aborts subscriptions.

The companion token entry needs a business-session family and immutable bound
resources at **creation and dispatch**, including direct calls for tools absent
from discovery. Parent/root/task/profile/epoch/generation checks apply to all
business tool families, files, events and child delegation. No generic session
read, arbitrary fetch, approval, provider config or legacy terminal operation.
Children are disabled in E1's new business tool family; a later enabled child must
have a narrower explicit grant, distinct generation and inherited revocation.
Never put connector/host credentials into generic agent runtime env. E1's existing
host remains explicitly trusted; E2 acceptance requires preventing a process from
bypassing the broker through host sockets/files/ports. Native restricted-window
availability stays false: Tauri2.10.2 app-global channel fetch/private callback
paths remain the separately tracked boundary, not closed by these command ACLs.

## Connected marketing accounts and read freshness

`connections/accounts/list` proposes `{input:{domain,cursor:null|string,limit:1..50}}`
under the same business authority. It returns only actually bound accounts with
`{id,provider,accountId:null|string,displayName:null|string,connectionRevision,capabilities:{read,refresh,prepare,publish},state:"not_validated"|"fresh"|"stale"|"unavailable"|"revoked",snapshot:null|{id,revision,observedAt,providerUpdatedAt:null|string,validUntil,facts},lastAttemptAt:null|string,lastSuccessfulRefreshAt:null|string,reason:null|string}`.
No account identity inferred from a credential name. No social accounts means an
honest empty view; Fireflies/email are not relabelled as social platforms. Facts
are a provider-specific closed public schema, never arbitrary provider JSON or
configuration. `prepare`/`publish` stay false unless a separately implemented,
authorized adapter operation exists; reads do not mint write authority.

`connections/accounts/refresh` proposes `{input:{operationId,accountBindingId,expectedConnectionRevision}}`.
Refresh is an explicit current permitted human action; opening a task/marketing
view reads cached state. Background refresh/scheduling requires separate accepted
authorization. Worker uses a fixed verified provider account/read operation,
credential store/client and least scope. MCP names/annotations alone do not prove
read-only behavior: pin actual official tool/CLI/API implementation, input schema,
pagination/limits, permissions, errors and side effects. Official MCP/CLI is
preferred when adequate; fixed API fills documented gaps. Neither protocol is
the document library or task engine. No arbitrary executable, URL or server config
in agent/task requests; social provider identity is not yet selected/verified by
this contract, so E3 does not pretend to implement one.

Persist account ID/resource, config revision, tenant epoch, current grant epoch,
operation/source attempt sequence and immutable normalized snapshot revision.
Cross-import/refresh completion order is fenced at the account source, as in B's
accepted source fence. An old/late attempt cannot replace newer good state.
`fresh` requires all captured authority/config/grant versions current plus a
successful validated read inside the adapter's fixed freshness duration (initial
UI ceiling5min; a shorter provider bound wins). Browser time/refetch, an agent
claim, login, or credential presence cannot extend it. Separate attempt failure
from retained last-good facts; label stale values and unknown fields explicitly.
Revoked access withholds facts rather than replaying a cache. Freshness is about
the observation, not a guarantee nothing changed upstream. Model prompts cite the
snapshot ID/revision/time and stale status; any later external action must rerun
the separately accepted exact-payload/current-authority check and reconcile its
provider receipt. Scheduling/publication/spend is outside this docs-only dispatch.

## Runnable synthetic acceptance recipes (planned, not executed)

These recipes specify new test work; the proposed `business_execution_` selectors
and harness do not exist yet. They must use a new coordinated fixture/output,
never ports4351/4352/4354 or the user-inspected browser. Source/API responses come
from the real backend; only the external AI/connector process is synthetic.

| Slice / owner | Concrete proof and pass condition |
| --- | --- |
| E1 backend — tickets; UI — rebrand; review — approvals | Add `business_execution_operator_flow` to the existing Rust test scaffolding. A test-only installed-client override points to a real stdio ACP fixture implementing the pinned installed handshake/new/load/prompt/cancel contract and a separate real PTY fixture. No network/provider code. It writes a fixture Markdown brief and a valid bundled PPTX sample into its owned workspace, emits ordered tool/output events, records prompt/start counts and supports a paused cancellation. Protected operator creates a human-only marketing task, starts once, reconnects to the same session, prompts again, imports both files, adds a second brief version and submits exact selected versions. Reviewer downloads and hash-compares retained bytes, then accepts/returns through current task review. Real Playwright CLI verifies adjacent panes, long content, keyboard/mobile/RTL and unsaved draft preservation. The PPTX is managed/downloadable even if no installed renderer; report that preview gap. |
| E1 recovery/assets — tickets | `business_execution_recovery` injects lost start/prompt replies, response cancellation, process exit, idle reconnect and crash windows before/after immutable file finalization and DB commit. Same receipt causes one child/prompt/version; unknown remains visible without resend; DB/audit failure leaves no public version/task revision; original scratch deletion does not break a retained version. `business_execution_assets` probes source symlink/hardlink/path races, oversize/unsupported types, edited bytes, stale task CAS, revoked publication inputs, cross-tenant reads and immutable approved versions. |
| E2 authority — tickets; independent approval boundary — approvals | `business_execution_authority` uses real Principal/provisioning with two tenants and private profiles, an injected worker transport only for core tests, and existing original-credential/tenant epoch lifecycle helpers. Probe foreign task/session/terminal/file/event/output/profile, owner-role masquerading as operator, viewer writes, changed-away-and-back task/profile, revoked lineage and late output. Also directly call a forbidden undiscovered companion tool; discovery-only denial is insufficient. Original engineering entrustment suites must stay green. |
| E2 actual process boundary — tickets, separately provisioned environment accepted by root; independent review — approvals | In the explicitly isolated test environment, run the real same-binary ACP/PTY worker with synthetic credentials/other-tenant sentinel files and no model endpoint. An actual child attempts host/foreign file, socket, environment/profile, terminal, event and delegate access; all denied while its own ordered stream/result works. Verify subprocess tree teardown and no leaked buffered payload on revoke/suspend. A same-user local mock cannot pass this gate; no fixture/deployment starts in this docs task. Native tenant support remains a separate blocked gate. |
| E3 connector — tickets; marketing UI — rebrand; review — approvals | `business_execution_account_reads` injects a fixed local official-protocol-shaped read fixture after the provider is pinned: account self/read pages, final empty page, pagination loop, partial error,401/403/429, timeout, changed identity and reordered completions. Prove no mutation operation, stale/unknown display, grant withdrawal, captured epoch and separate member namespaces. Actual Playwright CLI shows connected identity/current facts/last-good time and failed refresh without invented metrics or publish controls. This is synthetic coverage, not live account verification. |

Implementation commands, after those tests exist: own-target `cargo test --locked
--features test-utils --lib business_execution_`; corresponding locked server
selector; default/server/companion checks and Clippy for changed Rust; focused
frontend behavior/typecheck/lint/export. Reuse `integrations/pi-desk/process.test.ts`
for actual stdio/UDS discovery/cancel technique, but parameterize the exact owned
companion artifact (its current hardcoded debug path must not cause target races).
Test extracted per-launch assets as well as source files; separately show actual
installed-client model readiness with zero inference. Shell/stdio synthetic
success, installed CLI readiness, configured-provider execution and final native
bundle are different evidence levels. No new broad reruns or live actions now.

Official protocol design reference: MCP specification2026-07-28 at
`aa8ce049f089f92618340190d4ece141f663310d`, `server/tools.mdx` and
`server/resources.mdx`, read via `gh api` after installed-source checks. Explicit
handles and schema validation do not replace authorization; resource links do
not guarantee retained bytes. No installed protocol upgrade or copied examples.
Exact blobs/license transition and source read limits are in the worker report.

Root dispatches E1 as a vertical delivery first; E2/E3 are required follow-ons,
not implied complete by E1. Tickets owns backend/session/assets/task helper and
later fixed connector adapter; rebrand owns same workspace, document and account
UI; approvals independently reviews. Identity-owned code changes/registration
need the existing owner seam, not concurrent edits. Root reserves migration names
and coordinates new fixture ports before product work. Only the two contract/report
documents change here; all proofs above remain planned.
