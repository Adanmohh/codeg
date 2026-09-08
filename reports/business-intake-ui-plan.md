# Increment B — source-to-task interaction plan

Docs-only worker: rebrand, branch **docs/business-intake-ui**. Root accepted
[PR26](https://github.com/Adanmohh/codeg/pull/26) at
**91a69f339e99266b35bb912ae7d1cb155a6acd76**, merged as
**56c3ad91fcf5398ed2b6746f770fd88bd0f54c4f**. This follow-up starts from accepted
main **2233cd4366a38f22ca308dcc81191be4489b40e5** and verifies the newly pinned
contract: **Q1–Q4 are closed for UI consistency at 670af9ca; no remaining UI
contract defect was found**. Root has accepted PR25 at report head
**7f4d4dbc66f3b7487ffcb623769d79c9be2ac044** (merge aa16a9b9); its contract bytes
remain 670af9ca. This worker's frontend implementation awaits separate dispatch.
No B endpoint, provider connection or passing UI result is claimed by this plan.
Follow-up draft: [PR27](https://github.com/Adanmohh/codeg/pull/27).

Contract authority: tickets' [accepted PR25 contract at
670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5](https://github.com/Adanmohh/codeg/blob/670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/docs/contracts/business-intake.md).
Read the complete local draft first, then verified identical bytes through
`gh api` at that immutable commit: SHA256
**6d4be7f3be2c41264ab3bb9c27311e879306a75af0aaae6c0e240f434c874dbd**.
This supersedes the plan's earlier 79a92294/85f6001f contract references. Its
operations remain implementation contracts, not existing production APIs.

The serializable draft, passage-only rebase, restricted disclosure, durable
rediscovery/decision and bounded safe-error contracts are now explicit at this
pin. The existing authorized task snapshot plus exact target revision/domain
checks also closes text-free link review without a new preview entity. The
canonical protected access seam at
[18be55edc276713fc6d46d075baec363245ba285](https://github.com/Adanmohh/codeg/blob/18be55edc276713fc6d46d075baec363245ba285/docs/contracts/business-intake-access.md),
incorporated by the final intake pin, defines setup once. Read all 307 lines
locally, then verified identical `gh api` bytes: SHA256
**2d3312aa9b85e3f2af5763785b248a6e55ccc06a71c39be8c5962bb505fd4675**.
No role, empty list or client guess replaces the protected setup contract.

## Smallest useful B slice

An authorized human imports a bounded window of their configured Fireflies
meetings, reads an exact passage, writes a private task draft, sees its precise
destination audience, and explicitly creates shared work or links existing work.
They can discard a candidate without deleting the source. A second authorized
human sees the resulting ordinary task; a task reader without source permission
sees only a restricted source link. Human progress/review uses Increment A.

This requires usable protected binding/grant/publication setup, a real read-only
adapter, durable import recovery and atomic task acceptance. A fixture-only
binding or an operator SQL recipe is not a finished setup path. Actual provider
access can remain unconfigured, with accurate next steps and synthetic validation;
the product must never label that state connected.

Keep **My work**, **Shared work** and task **Review** primary. Add a useful
**Sources** destination within the existing business session once its API exists.
Its heading is “Turn source decisions into work”, with actual source titles and
task outcomes leading. “Review” in the existing navigation continues to mean
reviewing task completion; private intake drafts do not enter that queue or task
totals. Do not foreground jobs, claim IDs, engineering runs or provider schemas.

Fireflies is the first complete slice. Email/Hafidh capture follows through the
same source review once explicit mappings and safe projection helpers exist.
Do not display enabled capture controls, empty marketing modules, connected
badges or source counts for unsupported adapters. This sequences B's delivery;
it does not remove the contract's later email/Hafidh obligations.

## Concrete interaction

| Step | Visible content and direct action | Proposed contract operation |
| --- | --- | --- |
| 1. Sources | Permitted binding label/kind and truthful readiness. Show last observed date only when returned. Select a bounded, visibly UTC import window; “Import meetings” starts one explicit pull. | Canonical `bindings/list/status` at access pin 18be55ed, `sources/list`, `imports/start`; no duplicate readiness endpoint |
| 2. Import progress | “Reading meetings”, “Waiting to retry”, “Stopped” or “Finished this window”; counts describe this import only. Rediscover permitted unfinished imports after reload, resume explicitly and keep terminal history distinct. | `imports/list/get`, `imports/advance`, `imports/cancel` |
| 3. Read source | Show source title, observed/local version, access state and a separate private-source notice. Read complete returned plain-text passage blocks. Optional sentence position/time appears only when present and valid. | `sources/get`; explicit record refresh through `imports/start` |
| 4. Select evidence | Checkbox selection of exact server passages, with selected count and readable selected text. At least one, at most 20 passages / 20,000 combined characters, all from one source version. Reject excess with an explanation; never silently truncate. | `candidates/list/get/create/select`; select can save/rebase passages while preserving a null draft; send IDs, never caller quotations or proof |
| 5. Save private draft | Existing title/outcome/domain/priority/calendar-date/responsibility fields. A separate source column remains visibly private. “Save private draft” returns the normalized exact draft for review. | `candidates/edit`, using the candidate and source revisions |
| 6. Review audience and decide | Show the exact saved task text, owner, optional executor/reviewer, priority/date and selected destination domain. Unchecked publication confirmation precedes “Create task in [area]” or the separate link flow below. | `candidates/accept`; no task payload in accept |
| 7. Continue work | Open the returned ordinary task detail and refresh the existing task list. Show an accurate created/linked receipt and a restricted source reference. No automatic progress, agent launch, external notification or completion. | Existing `TaskDetailDialog` / task operations; `tasks/sources` |

The provider's action-items summary is source material, not a list of verified
commitments. Preserve string versus string-array boundaries. Empty, missing and
unsupported summaries have different copy; valid transcript passages still allow
human drafting. No paid extraction, guessed speaker, email-to-member match or
automatic prose-to-task split. A synthetic example may say “Alex, next Friday”;
the owner picker and due date remain unresolved until a human decides.

Task text begins as an intentional human draft, not an automatic transcript
copy. Save explicit ownerId for the editing human; assignee/reviewer/date default
null and priority normal. Changing a destination revalidates its human/agent
references and publication grant. Null reviewer means any currently authorized
human reviewer. Calendar input/display stays strict YYYY-MM-DD; never construct
an instant or silently convert “next Friday” to a deadline.
The normalized review uses the pinned serializable PreparedTask response, with
all task fields and resolved defaults present; the existing Rust CreateInput is
an input contract, not a response DTO.

**Audience confirmation:** “People with access to [Website] work can read this
task. The selected source passages stay restricted.” The full reviewed title and
outcome, responsibility and date remain visible beside this statement. Include
that the accepted task is retained if source access is later revoked. Domain
visibility includes future authorized readers and separately authorized assigned
agent contributions; it is not assignee-only visibility. A selection of people
never grants access to the meeting. Changing draft, domain, passages, source or
candidate revision clears confirmation and requires a new saved preview.

**Link existing work:** search only through the existing authorized task list,
within a permitted publication domain; fetch current task detail and require its
current edit capability/revision. Show its saved title, domain, status, revision
and owner before confirmation. Linking records a source reference without
replacing task text. If the task is in Review, explain that linking returns it
to In progress and requires another review, per the contract's metadata-edit
rule. No archived/terminal target, guessed UUID or engineering link-execution
shortcut. The link's destination audience is explicit even though private
passages are not copied into task activity.

A separate durable prepared-target record is unnecessary for this text-free
link. The pinned transaction contract requires checking taskId +
expectedTaskRevision + publishToDomain against the exact live task, together with
candidate/source revisions and grants.
Freeze the authorized existing tasks/get Detail as a local review snapshot;
changing target or receiving 409 clears confirmation. No private draft text is
published by link, and its domain must not silently replace the chosen target
domain. `candidates/select` now supplies passage-only selection/rebase with both
expected revisions, fresh access and pending-candidate checks. It retains draft
or null and suggestions without changing task text; link requires no PreparedTask.

**Discard:** distinguish “Discard candidate” (durable terminal decision, no task,
source/history retained, identical reimport does not seed it again) from “Discard
unsaved edits” (leave an editor without saving its local changes). Closing a dirty
editor offers Keep editing; it never silently calls the terminal discard route.
Accepted/linked/discarded candidates show history, not another create action.
Follow the explicit discard capability: a pending candidate with current read+
triage permission can be discarded without fresh passages or a task destination.

## Visibility, readiness and recovery

Private source titles, passages, suggestions, unpublished drafts, participant
metadata and private audit stay in their authorized source surface. Do not put
them into task notes, domain search, ordinary activity, notifications, URL state,
analytics, localStorage or agent context automatically. A task source link for an
unauthorized reader shows only “Restricted source” and its opaque local link
identity, with no source title, provider URL, ID-derived tooltip or hidden DOM
copy. Plain text renders as content, including instruction-like text.

| State / event | User-facing recovery and required behavior |
| --- | --- |
| Missing/disabled binding, no source grants | Honest setup or “No sources available to you”. Do not distinguish hidden-source existence or expose forbidden counts. An actual operator may use the accepted protected setup flow; a member sees a concise administrator handoff, never legacy settings or a credential prompt. |
| Binding unavailable / owner or resource drift | The allowlisted binding_unavailable reason permits a generic “Review access and setup” action for canManageSetup, or an administrator handoff for members. Status/grant inspection and protected same-owner update are available without exposing the private owner revision. Revalidation changes the epoch and requires fresh source access. |
| Configured but unverified | “Access has not been checked”; explicit authorized import/refresh, no green connected claim. Provider summary readiness remains independent of connection and passage availability. |
| Initial list observation | SourceSummary.revision is null before the first validated detail. Show “Not yet read” without a fabricated version or available passages; list discovery does not grant freshness. |
| Expired access or refresh begins | Hide private passages/returned drafts, block passage editing/publication and clear confirmation; discard follows its separate current capability. disclosure=metadata_only returns empty passages and null draft/suggestions; hasPreparedDraft distinguishes withheld content from a never-prepared draft. Failed refresh cannot restore freshness. Respect accessValidUntil and server access state; the 300-second ceiling is not a client-issued grant. |
| Fresh source, restricted draft destination | Fresh source access can coexist with hasPreparedDraft=true and draft=null. Label the saved draft unavailable with current access; do not treat it as never prepared or hydrate cached task/member data. Use current action flags and publicationDomains, never a guessed destination. |
| Successful fresh refresh, same source | Re-read current candidate and capabilities; preserve an authorized local edit deliberately, then save/review again. No automatic acceptance or source-to-task mutation. |
| Source or access changed / A→B→A | After fresh access revalidation, label retained passages/draft with candidate.sourceRevision and the current source with source.revision. Load replacement passages from sources/get. A changed prepared access epoch requires rebase even if text is unchanged. Explicit select/edit checks current revisions and epoch, adopts the current base and clears requiresRebase; clear confirmation and review again. Reading alone never rebases. |
| Candidate edit or target task 409 | Lock mutations and retain the local draft. Load current authorized state, show base and current status/revisions, then offer explicit adoption. Do not relabel the stale header as current, overwrite another human or reuse a checked confirmation. |
| Session 401 / source revoked / hidden 404 | Existing session 401 closes the business client and clears session state. Provider rejection must not become authentication_failed/401. Source-specific denial removes sensitive DOM/state and late responses; never show a cached transcript as an offline fallback. An independently authorized task remains governed by its own task access. |
| Another import is busy / retry later | Show status and the returned next permitted attempt time. Disable conflicting advance while busy. Do not invent a lease token, steal ownership or run an unconditional retry loop. |
| Disconnect/restart | Stop local advancement. imports/list requires current read+import grants and returns up to 50 visible jobs/page. ImportView is unfinished/all, default unfinished and page0. Unfinished means queued/running/waiting; history includes terminal jobs. A currently authorized human resumes as themselves, never as a reconstructed requester. Terminal jobs need an explicit new bounded start, not advance. |
| Mutation response lost | Reconcile CandidateDetail.decision under current source/task authority; restricted target returns no task ID/title/revision. Retain the original operationId/body for permitted replay; never call ordinary create or use a fresh accept key to recover uncertainty. Replaying an advance with a live claim only returns status; after timeout/expiry, an explicit new advance key starts a fenced attempt. |
| Import cap / invalid provider response | Explain partial coverage or “Window limit reached”, with an explicit narrower/repeated window. HTTP-200 errors, null/malformed/oversized responses are failures, not an empty successful list or full synchronization. |

Already published task text is not silently deleted on source revocation.
Source privacy cannot recall what an authorized human previously read; the UI
must accurately explain publication before the decision, not imply revocable
transcript access also revokes the separate business task.

## Existing seams to reuse; no parallel task/auth system

All current source paths below were read at accepted
**4e64476c7ad9161a5e535b5c75f592716d3ca6e2**. This is composition planning,
not a claim that a new intake component/helper already exists.

| Existing source | Planned reuse / concrete limit |
| --- | --- |
| `src/app/business/{page,layout}.tsx` | Preserve the private session lifetime and accepted native titlebar. Add no new principal/token storage and do not reparent drafts on locale/viewport changes. |
| `src/components/business/workspace.tsx` | Add Sources within the current business navigation; retain My work/shared/task Review, guarded engineering entry and the unchanged shared Drawer. Load source APIs only inside this business flow. |
| `src/components/business/task-form.tsx` | Reuse exported MetadataFields and AssignmentFields. Its CreateTask submits directly to task create, so it is not the intake acceptance component; compose the fields inside a private candidate editor. |
| `src/components/business/{ui,task-detail}.tsx` | Reuse Action/Field/Modal, real focus return, dirty-close pattern and labelled base/current comparison. Open returned TaskDetail after accepted creation/link. Existing SavedTask is private to task-detail; do not claim it is exported. |
| `src/lib/business/{client,tasks,identity}.ts` | Add a closed typed intake operation map only after wire acceptance. Preserve in-memory bearer, no-store/omit-cookie/no-redirect and late-result checks. TaskFields/Assignment remain the existing task shape. Q4 defines an intake-only reason projection with the existing 20-second client timer unchanged. |
| `src/components/business/{work-list,activity,preferences}.tsx`, `src/lib/business/{copy,presentation}.ts` | Retain list/board/task review, public-only activity, EN/AR copy, existing themes/fonts and literal DueDay presentation. Source links need an explicitly safe display, not raw activity JSON. |
| `src-tauri/src/business_tasks/{store,policy}.rs` | Read-only verification: create currently owns/commits its transaction; edit capability and task visibility already exist. Tickets must extract an internal transaction-aware create/link seam. UI must never sequence ordinary create then source-link as separate writes. |

## DTO coordination — exact closure status

Rechecked the existing Q1–Q4 checklist against frozen 670af9ca and its canonical
18be55ed access reference. All four fit the interaction plan; no new survey or
contract surface is proposed. Backend responses remain the authority.

1. **Q1 — closed at 670af9ca: setup, capabilities and disclosure.**
   [Binding authority/revalidation](https://github.com/Adanmohh/codeg/blob/670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/docs/contracts/business-intake.md#L150-L198)
   uses the canonical access list/status, actual-operator setup, zero initial
   grants and explicit retained/current/future scope. The
   [closed source/candidate DTOs](https://github.com/Adanmohh/codeg/blob/670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/docs/contracts/business-intake.md#L419-L460)
   pin nullable initial source revision, select/edit/accept/link/discard flags and
   publicationDomains. A fresh source with an unreadable destination returns
   hasPreparedDraft=true/draft=null. binding_unavailable plus the generic protected
   “Review access and setup” action closes the owner-drift recovery detail without
   exposing private revisions or introducing another readiness route.
2. **Q2 — closed at 670af9ca: rebase and exact review.**
   [Prepared access epochs](https://github.com/Adanmohh/codeg/blob/670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/docs/contracts/business-intake.md#L268-L277)
   and [selection/decision rules](https://github.com/Adanmohh/codeg/blob/670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/docs/contracts/business-intake.md#L328-L365)
   require current-passage rebase after content or authority changes, preserving
   null/existing draft and resetting confirmation. Reading never rebases.
   Normalization does not rewrite title/notes or reinterpret calendar dates.
   Existing task Detail plus exact taskId/revision/domain checks supports text-free
   link with no PreparedTask or persistent target preview. Terminal flags are false;
   discard has its explicit read+triage rule, independent of publication freshness.
3. **Q3 — closed at 670af9ca: durable rediscovery and results.**
   [Claim recovery](https://github.com/Adanmohh/codeg/blob/670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/docs/contracts/business-intake.md#L288-L314)
   and [terminal/import DTOs](https://github.com/Adanmohh/codeg/blob/670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/docs/contracts/business-intake.md#L461-L476)
   preserve current-human recovery and task-target redaction. ImportView is the
   explicit unfinished/all union, default unfinished/page0. Live-claim replay
   returns status; a new fenced advance after expiry differs from an uncertain
   accept, which must reconcile its existing decision. Terminal imports cannot
   restart through advance. The shared source attempt fence prevents an older
   response from a different import restoring freshness.
4. **Q4 — closed at 670af9ca: deadlines and safe reasons.**
   [Provider/core limits](https://github.com/Adanmohh/codeg/blob/670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/docs/contracts/business-intake.md#L107-L114)
   remain 12/15 seconds below the existing 20-second HTTP timer. The
   [finite error/recovery contract](https://github.com/Adanmohh/codeg/blob/670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/docs/contracts/business-intake.md#L479-L516)
   includes binding_unavailable through existing configuration_invalid, retaining
   generic task/auth behavior and an intake-only allowlist. Unknown/server prose
   is not rendered; provider rejection never closes a valid member session.
   The backend bound includes final writer work for HTTP/native; HTTP abort alone
   cannot cancel Tauri invoke. Precommit expiry rolls back with fenced recovery;
   a lost committed result is reconciled, never assumed failed.

This closes the UI contract checklist. Implementation of strict credential-store
reads, owner/source fences, atomic task helpers and the actual protected UI remains
future work with the existing acceptance gates. Tickets' backend dispatch does
not start this worker's frontend assignment. No tested runtime guarantee follows
from this documentation agreement.

**Q1 access review at 18be55ed:** canonical bindings/list returns canManageSetup
and BindingView; status returns the same checked projection. Only the actual
operator receives BindingAdmin or protected grant controls. Credential presence
means local availability, never verified connectivity. Create starts disabled
with zero grants, even for the named source owner. The operator explicitly sets
the publication ceiling/retained-task-text acknowledgment, grants named active
humans and enables use; a source-owner label never grants source access.

Grant confirmation must say “All retained historical versions, including those
captured before this grant, plus current and future sources imported through this
binding.” The scope literal is binding_current_and_future_sources but the UI must
include its full historical meaning. Separate grant rights and publication areas
remain visible; assignment never changes them. Operator key entry is write-only,
masked and transient, with no echo, logging or member credential prompt. Setup
responses and operation receipts reconcile uncertainty; no guessed new binding.

Owner revision drift pauses use. The pinned bindings/update operation
revalidates the same owner and changes the epoch; fresh source access is then
required. Changing owner/resource needs a new binding, never silent migration.
The 670af9ca binding_unavailable reason and generic “Review access and setup”
affordance resolve the prior diagnostic gap. The UI never guesses the cause from
false flags or exposes private authority revisions. The strict existing-store
writer read and staged-secret/DB failure bounds are implementation prerequisites,
not UI guarantees already tested here.

Email/Hafidh also need the contract's protected mapping and pure projections
before capture entry points. Public email message selection must exclude private
notes/activity/headers/addresses; Hafidh selection must omit reporter IDs, proof,
signed URLs and operator config. Capture cannot refresh upstream or mint host
freshness/evidence. Existing issue filing, email sends and receipt floors remain
separate. Their unavailable controls are explained rather than simulated.

## Later acceptance gates — planned, not run here

Read root's full [BI-1–BI-10 acceptance plan at
650be3025c25386649bc906f6bed335abafcb011](https://github.com/Adanmohh/codeg/blob/650be3025c25386649bc906f6bed335abafcb011/reports/business-intake-acceptance-checklist.md)
via gh api. It supplements PR25's B01–B18, including protected usable setup,
credential-store/database failure and rebind boundaries, two-session privacy,
durable claims and final integrated artifact correlation. The UI checks below
implement that acceptance plan; they do not replace its backend or native gates.

Use two named Playwright CLI sessions against a guarded synthetic backend with
real protected APIs, a separate agreed port/export and labelled synthetic source
records. Reserve nothing and start no browser/service in this docs task. Provider
fixtures stay loopback; no production response override or credential snapshot.

- **Direct vertical flow:** truthful setup → bounded import → exact passage →
  saved private draft → full audience confirmation → one accepted task; second
  session sees the real shared task, then existing human submit/review/Done.
  Separate link/discard flows verify their different effects and review invalidation.
- **Privacy/authority:** granted source reader versus task-only reader, viewer,
  revoked/foreign principal, changed grant/publication domain and agent. Inspect
  responses, permitted counts, DOM, ordinary activity/search and source links;
  inaccessible passages must be absent, not merely visually hidden.
- **Recovery:** expiry at review, failed refresh, source edit and A→B→A, real
  competing candidate/task 409, response loss before/after commit, reload/resume,
  competing import claim and cancelled late response. Backend tests prove
  atomic one-task/one-decision and no orphan rows; UI proves usable recovery.
- **Visual/keyboard:** EN/AR at 390/768/1280, light/dark and reduced motion;
  long source titles/passages/task text and exact calendar dates. At 390 use one
  reading column with named source/draft/review steps, not two squeezed panes.
  Source text remains available before confirmation; no fixed footer obscures it.
  Logical Tab order, visible focus, Space-selectable passages, Enter only on the
  intended form, Escape/dirty-close and correct focus return. Arabic content uses
  logical layout/bidi isolation without flipping literal dates or native controls.
- **Design Studio:** apply Saving changes, Input error and Drawer checks; inspect
  real empty/loading/partial/stale/denied/conflict/terminal screens, measured
  contrast and focus/target/overflow results. Preserve existing user tokens/fonts;
  no ceremonial engineering dashboard or fake metrics. Fix verified findings and
  rerun affected states, with exact product/export hashes and screenshots.
- **Engineering gates later:** meaningful intake/client/component tests plus
  affected business session/provider isolation tests; typecheck, scoped lint and
  isolated export. Backend owner runs appropriate default/server/Clippy and
  concurrency/privacy tests. Existing acceptance counts do not pass these new B
  paths. Root's integrated/native acceptance remains separate.

## Provenance and this docs checkpoint

Read complete FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS,
BUSINESS-IMPLEMENTATION and BUSINESS-WORKSPACE; Increment B and the current
explicit docs-only dispatch govern over historical paused prose. Read the full
immutable intake draft and existing identity/task/UI contracts plus sources above.

Code-context offline guide used the existing rag-skills .venv/bin/python with
HF_HUB_OFFLINE=1: exit 0. Its reuse/isolation rules are general guidance; the
user's no-worker/no-product scope overrides generic delegation/merge advice.
Installed docs retrieval exited 3: rebrand.db is absent; no ingestion/install.
Local React 19.2.4 / Next 16.1.6 and existing controlled form/session components
ground the reuse plan. Design Studio checklist methods were read locally;
generic localStorage/visual-motion advice does not override private in-memory
state or this bounded planning scope.
Live docs-first PreToolUse and PostToolUse records for this worktree/session
01a07c1c-cf2e-73e1-bbe3-e758c8363042 were read at timestamps
1788889500/1788889589, both exit 0. Hooks remain enabled.

Approved Codeg Apache-2.0 baseline is v0.30.4,
**6f6bd648b206412644842a98d9ffeebf57292bed**; future composition will cite the
accepted local files above and retain NOTICE. No third-party hunk is copied here,
so NOTICE is unchanged. Fireflies query pin
**fbd24607bc784a2294ce402426aefe2cb8c00f50** and its MIT/provenance limitations
are tickets' verified adapter ledger, not an independent API audit by this UI
report. IntroMail **0bd24dfe284b888aa9f602fa1fd00e337ea38874** remains an approved
pattern source only where provenance is checked; no AGPL-inspired source-key,
dispatcher or authorization path is copied into this plan or future UI seam.

The final reconciliation read all 591 contract lines locally before `gh api` at
670af9ca; both copies have the SHA256 above, exit 0. Compared the accepted client
timer/error/native branches directly and confirmed ConfigurationInvalid in
src-tauri/src/app_error.rs before using the new safe reason. An initial filename
probe exited 2; rg --files located the actual file, then the source read exited 0.
No dependency API change is proposed.
The complete 307-line access seam at 18be55ed was likewise read locally and
matched against exact-commit `gh api` bytes, exit 0; 670af9ca incorporates it.
Read the full independent report at ca787542 and the final owner-report diff;
these provide reconciliation context, not separate runtime evidence.
Read the corresponding source-ledger passages without expanding the provider
survey. `git fetch origin`, ancestry check and `git merge --ff-only origin/main`
exited 0: the existing docs branch advances from its merged PR26 to 2233cd43.
No product tests/builds, UI export, browser/native run,
provider call, configuration, credential, new dependency or other-worktree write.
All prior outputs/fixtures remain untouched. The paused visual report stays
untracked and the ignored research checkpoint stays preserved. Only this report
belongs to this branch's change. `git diff --check` passes; no A gates were rerun.
PR26's docs-only report/push/PR creation exited 0 and root merged it. This
follow-up changes only the contract reconciliation in this report; no source,
NOTICE, lockfile, protected planning document or fixture is edited by the worker.
Root retains review/merge authority; Q1–Q4 have no remaining UI contract defect
at the two immutable pins above. This worker remains docs-only until dispatch.

Reconciliation checkpoint **57b07ca81257421b9cbb414f7ac456c1bb39ea06** is pushed;
`git commit`, `git push` and draft PR27 creation exited 0. The follow-up report
records that checkpoint/PR and the final 670af9ca/18be55ed Q1–Q4 closure; it adds
no implementation scope or passing evidence.
