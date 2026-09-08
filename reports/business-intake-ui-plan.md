# Increment B — source-to-task interaction plan

Docs-only worker: rebrand, branch **docs/business-intake-ui**, accepted main
**4e64476c7ad9161a5e535b5c75f592716d3ca6e2**. Implementation awaits root's contract
acceptance and dispatch. No B endpoint, provider connection or passing UI result
is claimed by this plan.

Contract authority: tickets' [PR25 draft at
79a922945668a633ea6b5f7e68f9bdc08a0725a3](https://github.com/Adanmohh/codeg/blob/79a922945668a633ea6b5f7e68f9bdc08a0725a3/docs/contracts/business-intake.md).
Read the complete local draft first, then verified identical bytes through
`gh api` at that immutable commit: SHA256
**dcd57be31965821917ad8c0c9c92bf296e70cf80782c756da6ed80f371e45722**.
Its operations remain proposed. The open wire questions below are requests to
tickets/approvals, not alternate API definitions.

Tickets' subsequent coordination note proposes an explicit serializable
`PreparedTask` matching CreateInput's fields/defaults with resolved ownerId,
plus `disclosure=fresh|metadata_only` and `hasPreparedDraft`. These are noted
below as pending contract updates, not fields already present at 79a92294.

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
| 1. Sources | Permitted binding label/kind and truthful readiness. Show last observed date only when returned. Select a bounded, visibly UTC import window; “Import meetings” starts one explicit pull. | `readiness`, `sources/list`, `imports/start` |
| 2. Import progress | “Reading meetings”, “Waiting to retry”, “Stopped” or “Finished this window”; counts describe this import only. Human can stop/resume, inspect failed records when authorized and open available sources. | `imports/get`, `imports/advance`, `imports/cancel`; durable rediscovery needs Q3 |
| 3. Read source | Show source title, observed/local version, access state and a separate private-source notice. Read complete returned plain-text passage blocks. Optional sentence position/time appears only when present and valid. | `sources/get`; explicit record refresh through `imports/start` |
| 4. Select evidence | Checkbox selection of exact server passages, with selected count and readable selected text. At least one, at most 20 passages / 20,000 combined characters, all from one source version. Reject excess with an explanation; never silently truncate. | `candidates/list/get/create`; send passage IDs, never caller quotations or proof |
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
The normalized review must use the proposed serializable PreparedTask response;
the existing Rust CreateInput is an input contract, not a response DTO.

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
link if the transaction checks taskId + expectedTaskRevision + publishToDomain
against the exact live task, together with candidate/source revisions and grants.
Freeze the authorized existing tasks/get Detail as a local review snapshot;
changing target or receiving 409 clears confirmation. No private draft text is
published by link, and its domain must not silently replace the chosen target
domain. The remaining wire need is saving/rebasing passage selection when
PreparedTask is null, without creating a fictitious new-task draft.

**Discard:** distinguish “Discard candidate” (durable terminal decision, no task,
source/history retained, identical reimport does not seed it again) from “Discard
unsaved edits” (leave an editor without saving its local changes). Closing a dirty
editor offers Keep editing; it never silently calls the terminal discard route.
Accepted/linked/discarded candidates show history, not another create action.

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
| Configured but unverified | “Access has not been checked”; explicit authorized import/refresh, no green connected claim. Provider summary readiness remains independent of connection and passage availability. |
| Expired access or refresh begins | Hide private passages/returned drafts, disable decisions and clear confirmation. Pending disclosure=metadata_only plus hasPreparedDraft distinguishes withheld content from a never-prepared draft. A failed refresh cannot restore previous freshness. At most 300 seconds is a backend limit, not a client-issued grant; use a server expiry/reason if supplied. |
| Successful fresh refresh, same source | Re-read current candidate and capabilities; preserve an authorized local edit deliberately, then save/review again. No automatic acceptance or source-to-task mutation. |
| Source changed / A→B→A | Label retained draft's source/candidate base and the current source version separately. Preserve human edits without replacing the base. Require current passage selection and explicit rebase before saving a new exact preview; Q2 fixes the missing wire detail. |
| Candidate edit or target task 409 | Lock mutations and retain the local draft. Load current authorized state, show base and current status/revisions, then offer explicit adoption. Do not relabel the stale header as current, overwrite another human or reuse a checked confirmation. |
| 401 / source revoked / hidden 404 | Existing 401 closes the business client and clears session state. Source-specific denial removes that source's sensitive DOM/state and late responses; never show a cached transcript as an offline fallback. An independently authorized public task remains governed by its own task access. |
| Another import is busy / retry later | Show status and the returned next permitted attempt time. Disable conflicting advance while busy. Do not invent a lease token, steal ownership or run an unconditional retry loop. |
| Disconnect/restart | Stop local advancement. After current authentication, rediscover permitted unfinished imports and resume explicitly; no reconstructed operator or background auto-sync promise. Rediscovery is a wire dependency in Q3. |
| Mutation response lost | Preserve the original operationId/body in memory; first reconcile current authorized candidate/import result. Retry only the same operation identity where the contract guarantees idempotent replay. Never issue a fresh accept/create because the response is uncertain; reload recovery needs Q3. |
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
| `src/lib/business/{client,tasks,identity}.ts` | Add a closed typed intake operation map only after wire acceptance. Preserve in-memory bearer, no-store/omit-cookie/no-redirect and late-result checks. TaskFields/Assignment remain the existing task shape. Timeout and safe intake errors need Q4. |
| `src/components/business/{work-list,activity,preferences}.tsx`, `src/lib/business/{copy,presentation}.ts` | Retain list/board/task review, public-only activity, EN/AR copy, existing themes/fonts and literal DueDay presentation. Source links need an explicitly safe display, not raw activity JSON. |
| `src-tauri/src/business_tasks/{store,policy}.rs` | Read-only verification: create currently owns/commits its transaction; edit capability and task visibility already exist. Tickets must extract an internal transaction-aware create/link seam. UI must never sequence ordinary create then source-link as separate writes. |

## DTO coordination — outstanding against PR25 at 79a92294

These are small contract-completion requests, not requests for scope or money.
Implementation waits for root acceptance; this docs task can finish with explicit
unresolved dependencies. Backend responses remain the authority.

1. **Tickets + approvals: capabilities/setup.** Specify the actual
   BindingSummary.readiness/capabilities and Candidate.capabilities keys, allowed
   publication domains and safe access-valid-until/disclosure state. Pin the
   smallest protected binding, initial source-owner grant and publication-purpose
   setup route/result with actual-operator authority. A member role or an empty
   binding list cannot determine setup authority or grant a publication domain.
2. **Tickets + approvals: explicit rebase.** Does candidates/edit with the current
   expectedSourceRevision and newly selected passage IDs clear requiresRebase?
   What current/base passages and draft are available after fresh revalidation,
   and what is withheld in list/get while stale? Define the revision transition,
   safe old-version disclosure and suggestion origin before UI compares them.
   Tickets proposes PreparedTask, disclosure and hasPreparedDraft; pin these in
   the next immutable contract. Also specify a null-draft passage selection/rebase
   path for link. Existing authorized task detail plus final target CAS is enough
   for a text-free link preview; no additional durable preview entity is requested.
3. **Tickets: durable rediscovery/decision result.** The proposed imports/get
   needs an importId but no list/resume discovery is defined. CandidateDetail
   lacks the terminal Decision/task link needed after a lost accept response or
   reload without its in-memory operationId. Add a protected status/result seam,
   including how inaccessible destinations are redacted; do not require guessing
   IDs, ordinary task creation or replay under a different human.
4. **Tickets + UI: timing/error contract.** Existing BusinessClient aborts at
   20 seconds and collapses safe errors to eight generic kinds; draft advance may
   await a 30-second provider read. Agree one bounded advance/client deadline and
   closed safe intake reasons, preserving all existing task/auth behavior. No
   global timeout increase or raw provider-error rendering by inference.

Email/Hafidh also need the contract's protected mapping and pure projections
before capture entry points. Public email message selection must exclude private
notes/activity/headers/addresses; Hafidh selection must omit reporter IDs, proof,
signed URLs and operator config. Capture cannot refresh upstream or mint host
freshness/evidence. Existing issue filing, email sends and receipt floors remain
separate. Their unavailable controls are explained rather than simulated.

## Later acceptance gates — planned, not run here

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

Approved Codeg Apache-2.0 baseline is v0.30.4,
**6f6bd648b206412644842a98d9ffeebf57292bed**; future composition will cite the
accepted local files above and retain NOTICE. No third-party hunk is copied here,
so NOTICE is unchanged. Fireflies query pin
**fbd24607bc784a2294ce402426aefe2cb8c00f50** and its MIT/provenance limitations
are tickets' verified adapter ledger, not an independent API audit by this UI
report. IntroMail **0bd24dfe284b888aa9f602fa1fd00e337ea38874** remains an approved
pattern source only where provenance is checked; no AGPL-inspired source-key,
dispatcher or authorization path is copied into this plan or future UI seam.

Own branch creation/fetch, complete document/source reads and pinned `gh api`
contract read exited 0. No product tests/builds, UI export, browser/native run,
provider call, configuration, credential, new dependency or other-worktree write.
All prior outputs/fixtures remain untouched. The paused visual report stays
untracked and the ignored research checkpoint stays preserved. Only this report
belongs to this branch's change. Draft PR/commit handoff follows the early push.
