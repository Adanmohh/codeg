# Increment B business intake contract — ready for review, not implemented

Owner: tickets; docs-only branch `docs/business-intake-contract`, based on accepted
main `c7f7366fef2d7945cea18d4da1b7594fa3026e1c`. This prepares the bounded
meeting/feedback increment in `docs/BUSINESS-IMPLEMENTATION.md`. Root reviews
this contract before implementation ownership. Existing task/identity semantics
remain authoritative; no provider, configuration or model action is authorized
by this document.

## Fixed boundaries

- Fireflies is a read-only, explicit pull adapter. An imported source is not a
  task, assignment, consent to publish, or instruction to run an agent. No
  meeting bot, audio upload, provider mutation, webhook deployment or model call.
- Reuse the existing Rust service/database and protected business transport.
  Server-derived `business_identity::Principal` is the only caller authority;
  no caller actor, role, organization claim, credential reference or raw query.
- Source access is separate from task-domain access. Imported meetings and
  candidate passages default private to their explicit authorized human source
  audience. Participants, matching email/name, Fireflies privacy labels and a
  task assignment do not mint local source grants.
- Human acceptance publishes only the exact reviewed task title/notes to the
  selected domain. Accepted tasks remain visible to that domain per Increment A;
  transcript, attendee list and restricted evidence do not enter its notes,
  activity, agent context, search or notifications automatically.
- Candidate mutations require current source permission; publishing or reading
  a destination also requires its task permission, revalidated in the same writer
  transaction as CAS, task/source linkage, decision and immutable audit. Reimport is durable
  and idempotent; it must not overwrite human-edited or accepted work.
- Existing email/Hafidh storage remains under its accepted operator/account
  boundary. An ordinary business member cannot use a business source ID to
  traverse legacy Ops, credentials, private notes or evidence proofs.

## Accepted core seam that must be preserved

At the base above, `business_identity/mod.rs` exposes `authorize`,
`active_reference` and `begin_write`; private Principal rereads current member
and original credential lineage. `business_tasks/store.rs:354–420` creates a
human-only task, checks destination Create/Assign and active references, inserts
task/activity and commits its own transaction. `business_tasks/policy.rs` defines
task capabilities; sources confer no bypass.

Implementation will need a narrow task-owner extraction of that existing create
logic into a crate-only transaction helper, retaining the public create wrapper.
Candidate acceptance can then use one writer transaction. Such a helper does not
exist at this checkpoint. Calling the HTTP create endpoint followed by a second
link write, or copying task validation/INSERT SQL into intake, is not acceptable.
Link-to-existing similarly needs task-owned revision/capability validation and a
typed source-link activity seam; it is not engineering `link-execution`.

## Verified adapter boundary

Independently reread official Fireflies adapter queries, transport and MIT licence
through `gh api` at `fbd24607bc784a2294ce402426aefe2cb8c00f50`. Its list wires
date bounds/limit/skip; detail selects summary/readiness and sentence indices,
times and sharing metadata; HTTP-200 GraphQL errors are rejected. Intended reuse
is those narrow query/error patterns, never the n8n runtime or mutation queries.
Exact files/blobs, intended NOTICE entry and inspection limits are in
[the source ledger](../../reports/business-intake-contract.md#verified-sources).
The adapter is MIT with `Copyright 2022 n8n`; it is not permission to adopt or
copy the n8n runtime. The SDK's `Summary.action_items` is declared `string[]`,
while the earlier hosted-schema research described text. The immutable adapter
query supplies no return-type schema. V1 handles bounded string/string-array
observations explicitly; it never assumes normalized tasks, dates or people.

The only proposed Fireflies operations are fixed GraphQL reads at
`https://api.fireflies.ai/graphql`, using the existing trusted credential store:
credential check `user { user_id }`, bounded `transcripts` and `transcript(id)`.
Bearer/header shape is verified in the adapter credential source. Query text,
endpoint, method, headers and provider identity are backend-owned, never input.
Credentials/key references stay out of responses, job payloads and agent env.

Use a reduced projection of the verified query fields: `id`, `title`, `dateString`,
`sentences { index text start_time end_time }`, `summary { action_items }`,
`meeting_info { summary_status }` and necessary private access metadata. Do not
request media URLs, bots, analytics, full user/attendee profiles or AskFred.
Privacy/sharing metadata is an access observation, never a local grant. Unknown
sharing expiry formats are not guessed; ambiguous access makes disclosure stale.
No source URL is fetched or rendered as an automatically followed link.

Local limits proposed for B1: 50 list rows/page, five pages/window, explicit UTC
fromDate/toDate no more than 31 days apart, one HTTP read/advance, **12 seconds per
provider read and 15 seconds for the complete core request**. This supersedes the
initial draft's 30-second provider timeout. Keep the existing business client's
20-second timeout, a 2 MiB response cap and no redirects/ambient proxy credentials. These
are application limits, not a verified provider SLA. Store a frozen date window
and offset; the source gives no snapshot/order/updated-since guarantee. Show
`bounded_end` or `capped`, never “fully synchronized”. Revisit overlapping
windows and explicitly refresh selected older records; duplicates are harmless.

HTTP-200 with any GraphQL errors rejects the whole page/detail, including partial
data. Missing/null/wrong-type list data is not an empty successful scan. Reject
ID mismatch, invalid positions/times, duplicate sentence indices, oversized or
malformed text. Retain only a safe fixed error category, never raw upstream error
messages/correlation objects. No list marker advances on a rejected response.

## Source authority and visibility

Proposed backend-only binding contains organization, kind, source-domain, active
human source owner, approved provider scope, credential-store reference or exact
legacy account/inbox/product mapping, enabled flag and monotonic binding epoch.
It is created/changed only by actual protected operator transport with current
identity authorization. A role named owner is not that transport. Configuration
and grant-management implementation is a separately assigned prerequisite;
missing binding/purpose/access remains a visible setup gap, never a fallback to
Ops account1 or the first product. This preparation performs no configuration.

Fireflies B1 scans use `mine:true` for the explicitly bound provider principal;
raw transcript IDs can only refresh records discovered in that permitted scope.
Credential validation checks the same stored provider principal. Different key
ownership requires explicit rebind/epoch change. Shared/team-wide discovery is
outside this first bounded adapter. No local member mapping is inferred from an
email address, participant, speaker name, domain membership or task assignment.

Each source has explicit human grants: read, import/refresh, triage and permitted
publication destination domains, plus revocation/expiry and revision. Initially
only the configured human source owner can read/triage; publication destinations
default empty until explicitly authorized for task planning. Grant recipients
must be active same-org humans permitted for the source domain. No broad org or
agent grant, and no blanket transcript sharing. Source ownership itself does not
grant another domain or the right to assign/review tasks.

| Operation | Required current authority, all conditions intersect |
| --- | --- |
| Readiness/list/detail/passages | Human Principal, identity Read in source domain, active binding and explicit source read grant. Hidden sources and foreign IDs return not-found; counts/search paginate only visible records. |
| Start/advance/cancel an import | Human Contribute in source domain plus source/binding import grant; original requester and each actual advancing human are separately audited. Viewers and agents cannot import. |
| Candidate create/edit/discard | Human Contribute plus source triage/read grant. Preview/edit also validates the selected destination and references before exposing its task/member data. |
| Accept as new task | All source conditions, fresh exact snapshot, explicit permitted publication domain, and task core Create/Assign/reference rules using the same live Principal. |
| Link existing task | All source conditions and publication domain; current task visibility plus human task `edit` capability and expected task revision. Open, unarchived task only; this is not engineering link-execution. |

Source lists/candidates/passages and private audit do not enter domain task search,
generic activity/events, notifications, exports, analytics or agent context.
Task titles/notes are independently published business content only after the
human sees the exact text and destination-domain audience and confirms it. No
summary/transcript/attendee information is automatically copied into those fields.
Publication must explicitly permit retaining that reviewed business record; if
it does not, acceptance/link is unavailable. Source ACL revocation suppresses
future source disclosure; it cannot recall text a human already read. It does
not silently rewrite/delete separately accepted task text or completed work.

Readiness is local and layered: binding `missing | disabled | configured`;
access `unverified | fresh | expired | denied`; content `missing | available |
unsupported`; summary `missing | empty | available | unsupported`, with nullable
sanitized `providerSummaryStatus`. “Available” means a validated observation,
not provider job completion. The pinned SDK only declares summary_status a
string; unverified enum semantics are not turned into success. Missing/empty
summary differ. Human candidates may use validated transcript passages even
while a summary is absent; no inferred action or automatic model extraction.

Before detail refresh begins, invalidate its access freshness. Successful detail
validation can restore it; timeout, cancellation, deny or schema failure cannot
resurrect the old successful observation. Local B1 publication freshness is at
most 300 seconds, additionally bounded by known source/grant expiry. Every final
decision rechecks current binding/grant epoch, source revision and identity in
the writer transaction. Already in-flight provider reads cannot be recalled;
revocation prevents their result from being committed/disclosed afterward.

## Durable records and claims — proposed schema responsibilities

All new local IDs are UUID strings; revisions are positive monotonic integers;
timestamps are RFC3339 UTC. Organization is derived from Principal, never a
mutation field. Add one intake migration under a name reserved by root at actual
dispatch; no migration number is claimed by this document.

| Record | Required identity/invariant |
| --- | --- |
| Source | Unique `(organization, binding, kind, external identity)`. Fireflies uses transcript ID; email uses the bound inbox + conversation/message IDs; Hafidh uses bound product + TestFlight ULID. Titles, timestamps and API keys are never identity. |
| Source version | Immutable `(sourceId, revision)` plus normalized content digest, normalization version and observed time. Digest excludes fetch time/transient health. Provider revision is nullable and labelled; local SHA256 is not a provider revision. A→B→A creates three revisions even if content hashes recur. |
| Passage | Opaque ID, source-version ID, exact plain text and kind (`sentence`, `summary`, `email_message`, `feedback`). Preserve provider sentence index/time only when present/valid. No invented speaker/position. Selection sends passage IDs, not caller-supplied quotations/proofs. |
| Candidate | Stable ID, revision, source-version/selected passage IDs, origin, private suggestion text, nullable prepared task draft, `pending | accepted | linked | discarded`, binding/grant epoch and requires-rebase flag. One automatic review candidate per source; deliberate extra candidates use an explicit human operation. |
| Decision/link | Immutable deciding human, candidate/base/source revisions, exact reviewed public draft, destination audience, disposition and task ID/revision. Unique terminal decision per candidate and unique link `(org,candidate,task)`. Public task activity contains only an opaque link ID, not private source identity/title/text. |
| Import/step | Frozen scope/window, bounded progress/coverage, original requester, state/revision, attempted humans, current attempt UUID, lease expiry, attempt count, next retry time and safe failure code. Source IDs/versions/candidate seeding and successful step completion commit together. |
| Operation receipt/audit | Unique `(org, actor, operationId)` plus operation/payload digest and resulting IDs. Append-only public-safe action metadata; source text belongs only in protected versions/decisions. Same key with changed body conflicts. |

Do not merge sources across connections automatically. The same external meeting
may have separate access provenance through two bindings; show a possible
duplicate only if both are visible and let the human link existing work. This
limits duplicate guarantees to a binding, not global semantic deduplication.

Claims borrow the local SQLite writer/CAS/claim-ID discipline, not IntroMail's
process-local dispatcher. `begin_write` precedes auth and claim reads. Claim a
single due step with a fresh attempt UUID and 60-second lease, commit, perform
one fixed read without holding the writer, then reacquire writer ownership.
Revalidate live Principal, scope/epochs, lease and attempt UUID before persisting.
A late/expired/retired worker cannot commit even if nobody has reclaimed yet.
Atomic commit persists the page's IDs/detail jobs before the scan position moves.

B1 uses authenticated `advance` calls, not a new daemon or reconstructed stored
Principal. UI may advance a bounded import started by its human while connected.
After restart/disconnect, durable pending/expired work waits for an authorized
human to resume; it does not regain operator/agent authority from stored IDs.
Each advance uses its actual Principal and current credential, and preserves the
original requester as history. This deliberately does not promise unattended
continuous sync. Any later automatic job resumption needs an identity-owner
reviewed credential-lineage helper; the agent binding constructor is unsuitable.

`imports/list` rediscovers permitted jobs after reload; never depend on an ID in
localStorage or on knowing another caller's operationId. Claim and its operation
receipt commit together before network I/O. Replaying that advance while its
claim is live returns current import state without a second provider request.
After timeout/crash the human sees waiting/expired state and explicitly advances
with a new operationId; the old attempt remains fenced. A response lost after a
decision is recovered from CandidateDetail.decision, without another create.

Read failures can retry with the same logical step after bounded backoff: at most
three attempts before actionable `failed`, starting at 5 seconds and capped at
60 seconds; a valid provider Retry-After is honored up to that cap, otherwise
manual retry is required. 401/403, access/schema failures and unsupported data
stop automatically. No provider-specific code/retry SLA is assumed. Cancellation
increments the import revision/claim fence; a network response arriving later is
discarded. A read retry never invokes send/file/approve or recycles Ops receipts.

## Candidate review and exact task mutation

Import may seed one private review candidate from available action-items or
source passages. It does not split prose into authoritative commitments. An
action_items string remains one block; a string array retains item boundaries;
other shapes are unsupported. Suggestions carry provider/human provenance, not
verified ownership/dates. Owner/date suggestion strings default null unless a
human explicitly records them; ambiguous Alex/next Friday stays unresolved.

Candidate draft defaults null. The human selects at least one exact passage and
edits a `business_tasks::types::CreateInput`. Reuse its title/notes/priority and
date limits; resolve default owner to the editing human in the persisted preview,
not a latent “whoever accepts” value. Assignee/reviewer default null. dueDate is
null or strict YYYY-MM-DD, with no timezone conversion; suggestion prose never
sets it. Human/agent assignees require existing Assign and active reference checks.
No git folder, live agent, chat, external issue or engineering run is required.

`candidates/select` saves only a human's exact current passage selection; it also
performs explicit rebase when expectedSourceRevision matches the current source.
It increments candidate revision, adopts the current source revision and clears
requiresRebase, preserving draft/null and suggestions without changing task text.
`candidates/edit` performs the same current-passage validation/rebase while
replacing the prepared task draft. Both require fresh source access, a pending
candidate and candidate CAS. No extra automatic change is made during get/list.

After fresh revalidation, CandidateDetail may return its retained old-version
passages and draft, labelled with candidate.sourceRevision; SourceSummary.revision
is the current source. Selecting replacement passages uses current sources/get.
Metadata-only disclosure returns no private draft, suggestions or passages and
uses hasPreparedDraft to distinguish withheld text from an unprepared candidate.
The UI clears publication confirmation on any selection, draft, target, source
or candidate revision change; no stale confirmation survives a rebase.

Accept contains no edited task payload: it references the prepared candidate and
source revisions and explicitly confirms publication to the draft's domain.
Recheck all create/reference/publication conditions in one writer transaction;
create a normal todo task with its existing created activity, terminal candidate
decision, source link and private audit/operation receipt. Never auto-progress,
complete, review, notify externally or launch an assignee. Source linking must
use a task-owner helper that applies edit capability, CAS and typed activity; a
link during task review invalidates that review using the existing metadata-edit
rule. Do not duplicate task SQL/validators or call Operator::server from intake.

A text-free link does not require PreparedTask. The human freezes an existing
authorized tasks/get Detail for visible review, then sends its taskId, exact
expectedTaskRevision and publishToDomain with candidate/source revisions. Final
core validation requires that domain to equal the live target domain and current
publication grant. It copies no candidate draft text and needs no new persistent
target-preview record. Changing the target or a 409 clears confirmation.

Identical reimport changes only observation/progress metadata. Changed source
creates an immutable version and marks pending candidates for explicit rebase;
it preserves their edited draft and old passages, increments candidate revision
and blocks acceptance until the human selects current passages again. Accepted,
linked and discarded candidates keep their decisions and are not auto-created
again. New intended work from an updated meeting is an explicit additional
candidate or an explicit link, never an update to already accepted task text.

Same operationId/body replay returns the existing result only after current
source/task authorization; it creates no second task, activity or decision.
Different actor/payload/disposition after a terminal decision conflicts without
leaking a hidden destination. Concurrent source refresh, grant/credential revoke,
candidate edit or task revision change must either precede acceptance or make it
fail atomically. Rejects leave no orphan task, link, revision or activity.

## Proposed closed transport DTOs

These operations do not exist yet. All are POST `/api/business/intake/<operation>`
with `{input: ...}`, camelCase, no-store responses and deny-unknown-fields at every
envelope/nested union. Native names are `business_intake_` plus slash-to-underscore
operation, delegating to the same core/actual native operator Principal. Shared
member HTTP remains separate from local native owner and legacy Ops transport.
No B1 agent/MCP source or mutation tools are added.

`Id` means UUID string; `Rev` positive integer; `OperationId` a client UUID for
request dedupe, never authority. Optional defaults are stated below; all other
fields are required. `LegacyRef` is a closed tagged union: email
`{kind:"email",conversationId:positiveInt,messageId:positiveInt}` or Hafidh
`{kind:"hafidh_testflight",ulid:existing strict ULID}`. Account/inbox/product
come only from the approved binding. Fireflies IDs are opaque nonblank strings
up to 256 characters without controls, passed as GraphQL variables only.

| Operation | Input | Result |
| --- | --- | --- |
| readiness | `{}` | `{bindings: BindingSummary[]}` for permitted bindings only; missing setup is explicit |
| sources/list | `{bindingId:Id,page?:0}` | `{items:SourceSummary[],page,hasMore}`; 50 locally visible rows/page |
| sources/get | `{sourceId:Id}` | SourceDetail; protected passages only with current fresh read access |
| imports/start | `{operationId,bindingId,selection}` | Import; selection is `{kind:"window",fromDate,toDate}` for Fireflies or `{kind:"record",sourceId}` for explicit refresh |
| imports/capture | `{operationId,bindingId,ref:LegacyRef}` | Import for already stored email/Hafidh record; no upstream pull |
| imports/list | `{bindingId:Id,view?:"unfinished",page?:0}` | `{items:Import[],page,hasMore}`; view unfinished/all, current read+import grant, 50 visible rows/page |
| imports/get | `{importId:Id}` | Import; current read+import grant required, original requester identity alone gives no right; no hidden counts |
| imports/advance | `{operationId,importId,expectedRevision:Rev}` | Import; one due step/current Principal; owned live lease reports busy |
| imports/cancel | `{operationId,importId,expectedRevision:Rev}` | Import; fences queued/in-flight steps |
| candidates/list | `{sourceId:Id,state?:"pending",page?:0}` | `{items:Candidate[],page,hasMore}`, 50 visible rows/page; state is one of four candidate states |
| candidates/get | `{candidateId:Id}` | CandidateDetail and authorized current-source state |
| candidates/create | `{operationId,sourceId,expectedSourceRevision:Rev,passageIds:Id[]}` | Extra Candidate, no task, for deliberate additional work |
| candidates/select | `{operationId,candidateId,expectedRevision:Rev,expectedSourceRevision:Rev,passageIds:Id[]}` | CandidateDetail; explicit passage-only selection/rebase, preserving nullable prepared draft |
| candidates/edit | `{operationId,candidateId,expectedRevision:Rev,expectedSourceRevision:Rev,passageIds:Id[],task:CreateInput,ownerSuggestion?:null,dueSuggestion?:null}` | CandidateDetail with normalized exact draft; full replacement, null suggestions clear |
| candidates/accept | `{operationId,candidateId,expectedRevision:Rev,expectedSourceRevision:Rev,publishToDomain:Domain}` | `{decision:Decision,task:existing Detail,replayed:boolean}` |
| candidates/link | `{operationId,candidateId,expectedRevision:Rev,expectedSourceRevision:Rev,taskId:Id,expectedTaskRevision:Rev,publishToDomain:Domain}` | Same Decision result; no task content copied or replaced |
| candidates/discard | `{operationId,candidateId,expectedRevision:Rev}` | Decision; no task and no deletion of source/history |
| tasks/sources | `{taskId:Id}` | `{links:SourceLinkView[]}` after task read authorization; inaccessible source is only an opaque linkId/restricted label |

`BindingSummary={id,kind,label,domain,readiness,capabilities}`; no origin/key,
provider principal, account/product configuration or credential reference.
`SourceSummary={id,bindingId,kind,title,revision,observedAt:null|instant,
accessValidUntil:null|instant,access,content,summary,
providerSummaryStatus:null|string,requiresRefresh}`.
`SourceDetail={source:SourceSummary,disclosure,passages:Passage[],candidateCount}`; stale
authorized metadata can be shown with an explicit label, but passages/drafts
are withheld until refresh. Raw provider JSON/access metadata never serialize.
`Passage={id,sourceRevision,kind,text,index:null|integer,start:null|number,
end:null|number}`. Limit selection to 20 same-source/version passages and bounded
20,000 combined characters; no silent clipping or cross-source IDs.

`PreparedTask={title,notes,domain,priority,dueDate:null|string,ownerId:Id,
assigneeId:null|Id,reviewerId:null|Id}` is an explicit serialized response and
stored preview; existing Rust CreateInput is not Serialize. All fields are
present, normalized with existing task validation and explicit resolved defaults.
`disclosure` is `fresh | metadata_only`; metadata-only always returns passages=[]
and draft/ownerSuggestion/dueSuggestion=null, even if text is retained privately.
`Candidate={id,sourceId,revision,sourceRevision,state,requiresRebase,disclosure,
hasPreparedDraft:boolean,draft:null|PreparedTask,ownerSuggestion:null|string,
dueSuggestion:null|string,capabilities}`; suggestion text max240 characters.
`CandidateDetail={candidate,passages,source:SourceSummary,decision:null|Decision}`.
`Decision={id,candidateId,fromRevision,sourceRevision,kind,actorId,
task:DecisionTask,createdAt}`; source-authorized audience only. DecisionTask is
`{state:"none"}` for discard, `{state:"restricted"}` if the destination is now
inaccessible, or `{state:"accessible",taskId:Id,taskRevision:Rev}` after current
task read authorization. No hidden task ID/title/revision leaks in a restricted
result. Terminal disposition remains discoverable without an old operationId.
`Import={id,bindingId,revision,state,coverage,discovered,completed,failed,
nextAttemptAt:null|instant,errorCode:null|IntakeReason,capabilities}`; states queued/running/waiting/
complete/failed/cancelled, coverage not_started/partial/bounded_end/capped.
Import capabilities are explicit `{advance:boolean,cancel:boolean}`; no caller
may treat them as authority. Retain terminal imports for status/history; cancelled
or complete/failed jobs are never restarted by advance. The unfinished view
contains queued/running/waiting; history/all also returns terminal imports. A new explicit start is a new
bounded observation with source/candidate dedupe.
`SourceLinkView={linkId,accessible,source:null|SourceSummary}`; no provider URL.

Reuse business identity's error boundary: session unauthenticated/revoked401,
forbidden action403, hidden/foreign404, malformed400 and revision/idempotency
conflict409 with `business.revisionConflict`. No provider rejection may emit
authentication_failed or401 and accidentally revoke the member session.
Existing AppCommandError supplies the envelope; no new global error codes.

`IntakeReason` is the closed set below, encoded only through exact allowlisted
`i18n_key = "business.intake." + reason`, with no detail/parameter payload. The
intake client exposes optional `intakeReason` beside its existing generic error
kind and ignores all unknown keys/messages. Existing auth/task error behavior
remains unchanged. `Import.errorCode` uses the same finite reasons (or null),
never raw GraphQL/SQL/input/token/correlation data.

| Reason | Existing AppErrorCode / HTTP when returned as an error |
| --- | --- |
| binding_missing, binding_disabled, credential_unavailable | configuration_missing /422 |
| source_expired, rebase_required, import_busy, retry_later | already_exists /409 |
| source_denied, publication_not_allowed | permission_denied /403 |
| unsupported_schema, provider_unavailable, request_timeout | network_error /500 |

These are proposed intake keys, not names already implemented in AppErrorCode
or the frontend. Current `src/lib/business/client.ts` has only generic kinds;
the UI owner adds this narrow reason projection without rendering server text.

Deadline/recovery agreement: the core request bound applies to claim, network and
final writer work for both HTTP and native entry. If the bound expires before
commit, drop/rollback that transaction and leave the durable claim recoverable
after expiry; no detached background commit. A commit whose response was lost is
an uncertain result reconciled through imports/list/get or CandidateDetail,
never presumed failure followed by another task create. The 20-second HTTP timer
does not cancel a Tauri invoke today, so the same backend bound is mandatory in
native mode. Do not increase global client timeouts or change other business APIs.

## Email/Hafidh integration and preserved floors

Email capture resolves its exact configured account/inbox/conversation/message
in the same transaction and reuses ticket `MessageView::Public`: private notes
and activity messages are excluded. Importing a public reply is not permission
to expose sender/recipient addresses or headers to all task readers. Use a narrow
source projection; never return the operator thread DTO or fabricate RunContext.
Its legacy Message-ID is provenance, not an org-wide dedupe/authorization key.

Hafidh capture reads the accepted local TestFlight snapshot and a narrowed public
projection, with explicit org→account/product binding. It exposes no external
reporter ID, screenshots/signed URLs, proof objects, repository/App config or
private operator Detail. Missing/ambiguous binding, unavailable in-app reads and
expired host snapshot remain explicit. B capture/read never calls list/refresh,
sets verified_at, mints issue evidence or changes an existing source receipt.
Only the already authorized operator host flow can refresh upstream Hafidh.
Required pure projection/access helpers do not exist for business members yet;
the host owner must expose them under the new checked binding, not through an
impersonated Pi RunContext or by broadening existing operator routes.

Business source links are distinct from `ops_intake_host_fix`, whose existing
engineering handoff requires a confirmed GitHub receipt and folder binding.
Never repurpose that link/table or treat a business task as proof of an issue.
Creating/linking work leaves email revisions/recipients, private notes, issue
evidence/freshness, approval/destructive floors and unknown/no-resend receipts
unchanged. Existing business agent contribution still requires protected source
entrustment, original human credential lineage and exact live task/run. B1 adds
no source disclosure to that agent path; Astra/max and stable Pi policy stay intact.

## Required synthetic acceptance — not executed in this docs task

| Case | Required observable result |
| --- | --- |
| B01 Protected setup and wrong provider principal | Actual operator can configure binding/explicit human grants/publication; ordinary owner credential cannot. Missing/disabled setup is honest; store/DB failure and rebind cannot leave usable partial authority or secret output. |
| B02 Two organizations/hidden source/forged actor | 401/403/404 as appropriate, no content/count/reference disclosure; closed spoof/URL/query inputs400. |
| B03 Viewer and agent | Granted viewer can read eligible source only; import/edit/accept/link/discard denied. Agent receives no B source API or private context. |
| B04 Task-domain access without source grant | Can read public task but source link stays restricted; search/activity/notifications contain no transcript/attendee/private data. |
| B05 HTTP200 errors/null/oversize/ID mismatch | No snapshot/candidate/scan advance; fixed safe failure, no raw provider data in error. |
| B06 Summary, disclosure and null-draft link | Distinct missing/empty/malformed readiness; sentence-only human flow works. Metadata-only withholds retained text; fresh old/current versions labelled. Passage-only rebase permits link with null draft, no fictitious new task. |
| B07 Duplicate page/detail and response loss | Same source/version/seed candidate; repeated accept with identical operationId produces one task/decision/created activity. |
| B08 Source edit and A→B→A | Monotonic revisions, pending draft retained and explicit rebase; old acceptance rejected even when hash matches an older version. Terminal decisions/work unchanged. |
| B09 Offset shifts/cap/resume | Duplicate IDs converge, coverage reports partial/capped honestly; crash after page commit resumes persisted position/detail work. No claimed lossless sync. |
| B10 Competing claims/expired late worker | At most one accepted step commit; stale attempt cannot persist or advance after reclaim/cancel/expiry. |
| B11 Revocation during provider await | Old credential/member/grant/binding epoch fails final writer recheck; no committed source/publication; already in-flight read is disclosed as a limit. |
| B12 Restart and lost response | imports/list rediscovers jobs only under current grants; CandidateDetail returns terminal decision and redacts inaccessible task target. No reconstructed Principal, browser-stored secret source or automatic provider action. |
| B13 Accept/edit/source-refresh race | One winning revision; rejected transaction changes zero task/link/decision/activity rows. Crash after commit returns deduplicated receipt. |
| B14 Human-only task and references | Valid current Create/Assign owner/assignee/reviewer checks; no engine/folder/chat; todo→review→human Done through existing task operations. Revoked/foreign references rejected. |
| B15 Link and task review race | Existing editable task revision/visibility checked; source link invalidates pending review; archived/terminal/unrelated task link denied, no orphan/history mutation. |
| B16 Time, long/untrusted source and deadlines | Same calendar dueDate across Helsinki/US/UTC/DST; ambiguous date stays null. Long/RTL/instruction text is plain content. HTTP/native backend bound15s/read12s remains below client20s; safe reason projection, timeout recovery and no detached commit verified. |
| B17 Email/Hafidh privacy and old floors | Private note/activity/reporter/proof/config excluded; no host freshness change; no email resend/GitHub filing/approval/model request triggered. |
| B18 Actual shared visual flow | Guarded synthetic Playwright CLI: two sessions, import/readiness, select/edit exact task, accept/link/discard, stale/revoked recovery, accessible full review at390/1280 and light/dark. No mock production response. |

Later implementation gates: meaningful new core/import concurrency tests plus
relevant existing business/identity/Ops regressions, desktop/server check and
Clippy, frontend typecheck/changed-file lint/export and actual guarded CLI flow.
No passing count or provider access is claimed here. Live Fireflies schema,
pagination/access behavior and credential setup need separate authorization;
the public-api-ff GitHub documentation repository currently returns404. The
contract is implementable against synthetic verified query shapes with explicit
unsupported states; current hosted service semantics are not fabricated.

Root's [BI-1–BI-10 acceptance plan](https://github.com/Adanmohh/codeg/blob/650be3025c25386649bc906f6bed335abafcb011/reports/business-intake-acceptance-checklist.md)
was read completely and is complementary: setup→B01; bounded import→B05/09;
durability→B07/10/12/13; privacy→B02/03/04/11/17; human preparation→B06/14/16;
atomic decisions→B07/13/15; changes→B08/11; continuity→B14/17;
actual interface→B18. Integrated/native artifact gates follow implementation,
not this documentation proposal. No passing evidence is claimed for BI-1–BI-10.

No product code, provider request, dependency/runtime installation or configuration
change is made by this contract. Root review precedes implementation dispatch.
