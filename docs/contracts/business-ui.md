# Business workspace UI contract — implemented Increment A

Owner: rebrand; branch **feat/business-workspace**, draft PR21.
Final product sourcee72cc44b612068e67a3e6dc3bc593f10988ae7ed; accepted main294fb634 (task PR22 merge
5541857a) integrated as0192ac3f. Only NOTICE required additive conflict resolution;
the baseline4340 export is preserved and final review corrections are served on4346.
Implementation/evidence: reports/business-workspace.md. Backend owners retain
identity/task/agent authorization and migrations. The only Rust presentation
change is the owner-requested fresh native main-window business entry; no
parallel authentication system or task/engine backend changes.

## Source and integration

- Apache Codeg source baseline4ec04d7282a50529335d724438d42b99a53385a2;
  upstreamv0.30.4 SHA6f6bd648b206412644842a98d9ffeebf57292bed. Existing
  Button/Input/Textarea/Dialog/Drawer, task presentation and in-memory session
  patterns are reused. NOTICE records exact source files and preserves all entries.
- Identity: docs/contracts/business-identity.md, published861fb0ef, accepted
  c911c406/mainab46c9d9. Accepted identity main integrated as0e5eb3e7.
- Task: fixed DTOsbf4309f5; production contracts/types/handlers tested at
  **1ba73e3c90eb6e76d8ad7f0a79852dd2c86587e5**, including R1 source entrustment.
  Accepted atc3af4494 and merged as5541857a, now integrated here. The actual API
  fixture runs the same accepted runtime; its only later source difference is a
  test-only tool-discovery readiness wait. No mocked production task responses.

## Navigation and authentication

Default / and /business are direct-action business entries. Cold /index.html,
/business/, and /business.html also avoid inherited settings, wallpaper and
operator-connection providers, including with an ambient old operator bearer.
/business-other retains inherited providers. Exported URLs are checked by actual
Playwright CLI and the unchanged production Axum static rewrite/ServeDir stack.

The business connection has its own typed client and bearer in memory. Web and
shared-server members use POST /api/business/* with {input}; credentials never
enter codeg_token, the legacy WebTransport or CODEG_TOKEN. Cookies are omitted,
redirects rejected, origins validated, and only declared operation paths exist.
Disconnect/401 closes the client; late responses cannot repopulate a closed
session. Context organization/member/revision changes unmount private state.
Locale and viewport updates preserve drafts. Reload intentionally requires sign-in.

An original operator can intentionally reuse same-origin operator access or enter
it in the administrator setup choice, initialize an organization and manage
members. Personal members receive only their server capability context. Native
business_* commands remain original-operator-only; a desktop member chooses
shared HTTP access. No OS-native credential isolation certification is inferred.

Fresh native main windows use App("business"). Locked tauri2.10.2 joins that
path to the application URL and resolves the exported business.html asset.
Existing main-window focus does not navigate; the explicit engineering link
remains unchanged. Root owns actual final isolated native startup verification.

Context.capabilities.manageMembers controls People administration.
**Only Context.capabilities.legacyOperator controls engineering navigation and
source entrustment.** Owner/admin role and engineering domain are insufficient.
Member/owner credentials must never preload legacy config, credentials, terminal
or model-provider routes. Backend authentication/authorization remains decisive.

People lists the authorized capped500-member directory, kind, role, domains and
status. Forms obey the issuer's role/domain limits and protected operator-owner
rules; showing a person is not an authorization grant. Agent records have no
human credential issuer. One-time tokens are masked by default, intentionally
copied/revealed and cleared on close/disconnect; no storage or report/snapshot
persistence. Revocation returns useful sign-in recovery. Context revalidates on
focus and every30s; every mutation still relies on backend live authority.

## Task wire and direct actions

src/lib/business/{identity,tasks}.ts mirror UUID string IDs and camelCase DTOs.
Task operations use /api/business/tasks/{list,get,create,update,assign,progress,
note,submit,review,cancel,archive,link-execution,entrust-execution}, always POST
{input}. Native mappings use business_tasks_* names. TaskId is inside input.
Task mutations add no caller-supplied actor/organization identity override.
Update/assign replacement optional fields are sent as explicit null when cleared.

List input: mine/shared, optional domain/status/query, archived and page.
Response: {tasks,page,hasMore,canCreate},50/page. Detail:
{task,activity,deliverables,execution}. The UI uses task.capabilities for all
mutations and still displays safe forbidden/conflict errors if the backend denies.
My work, Shared work and Review support list/board, filters, manual refresh and
bounded load-more. Counts describe loaded tasks, not invented organization totals.

A task's accountable **human owner** and nullable assigned human/agent executor
are distinct. Null reviewer means **any currently authorized human reviewer**.
No organization role or picker selection grants permission by itself. Unknown or
revoked directory records show an unavailable person without guessed kind/IDs.

**dueDate is YYYY-MM-DD or null**, a calendar day without time/instant/reminder.
The form uses input type=date; display uses the exact literal with LTR isolation,
including Arabic. Never construct Date or truncate an RFC3339 value. Notes and
activity are plaintext; only pinned public activity fields are rendered, never a
raw JSON/provider payload dump.

Direct work: create brief/area/priority/date/responsibility; edit or reassign;
progress to todo/in_progress/review; add a public task note; submit a deliverable;
separate human accept/return review. Done is never a progress option. Review
shows the actual saved version and current deliverable and requires an explicit
confirmation checkbox. Backend capabilities enforce who may review. Human work
requires no conversation, git folder, terminal or engine.

Cancel marks Cancelled; archive removes from current lists; restore retains
status/history and returns to current lists. Each operation has an accurate
confirmation. No hidden deletion or auto-launch. Revisioned execution linking is
subordinate engineering detail. legacyOperator plus assigned agent can explicitly
confirm and entrust an existing source; the subsequent link is a **separate**
action using its returned revision. A stale/unbound run fails closed with useful
copy. No engine start, provider launch or run minting is part of this UI.

## Conflict, privacy and visual states

Edits keep their original expectedRevision. Real409 locks further writes while
retaining the private draft, then exposes Load current task, comparison and
explicit Use my draft with this version. No silent overwriting or blind retry.
The retained status/revision is labelled as the draft's base; the current saved
region shows current status/revision before adoption. Adoption resets the human
review confirmation; the user must review and confirm the newly loaded version.
Ambiguous creation warns the user to refresh before another creation. Errors use
fixed public copy rather than raw server/provider bodies.

The branded workspace foregrounds real business work, human accountability and
review. Existing themes/user font selection remain authoritative; no global token
changes or fake ads/revenue/connectors. At1280 the sidebar and work list establish
hierarchy;390/768 use the shared Drawer, readable filters and direct create action.
Status uses text plus a shape. Long content wraps inside the modal; only the board
scrolls horizontally. Shared Drawer stays unchanged and RTL uses its actual right
edge. Controlled business modals restore a connected opener or the focusable work
area; they never override a succeeding modal's focus. Reduced motion removes the
business entrance zoom and Action transitions. Existing overlay opacity fade is
retained. Scoped destructive text/focus use measured inherited red tokens.
Search has a scoped readable placeholder and skips its redundant hidden submit
in sequential focus. Tab reaches the visible Refresh button; Enter retains real
form submission. Global Input, tokens and search API behavior are unchanged.

English/Arabic business copy is supplied. The other8 existing locales preserve
preferences and use English business copy. No claim of10 translated business UIs.

## Verification and owned fixture

Merged0192 full frontend suite passed6,194 tests/438 files. After final comparison,
native and search corrections all39 affected business client/component tests pass.
Typecheck/scoped lint/34-page export and default desktop/server cargo check/Clippy
pass; the latter were rerun after native3a. The local sidecar placeholder does not
certify a runnable bundle; root owns actual packaged startup. Static router test
executes6 real production rewrite/body checks through axum-test's default
in-process transport. Report records source pins, exact commands and limits.

Actual Playwright CLI uses exactly business-owner4340 and business-member4340,
with synthetic personal owner/manager/member sessions. Real protected API flows
cover bootstrap/member setup, human-only creation, separate responsibility,
calendar roundtrip, concurrent409, draft locale preservation, deliverable/review,
shared Done, viewer403/revocation401, filters, source400 and cancel/archive/restore.
390/768/1280 list/board light/dark, Arabic/detail/date/drawer, keyboard, reduced
motion, long text and measured contrast evidence is committed. No response
replacement or live provider/model calls. Native OS and second50-record-page
browser tests are not claimed.

Corrected frontend4346 PID60964 owns .build/business-workspace/review-export from
e72cc44b; its business.html disk/HTTP SHA256 is
c029f99d5c92e677c97f2ebe45f4bd185746b73e84e77912a683e1d92de55a54.
Actual search EN/AR390/768/1280 light/dark checks pass12/12:10.78/7.07 contrast,
visible44px Refresh focus and successful Enter queries. Corrected comparison has
its own real409/adopt/save evidence; independent targeted recheck is underway.
Frozen frontend proxy4340 PID36044 retains out-business-workspace. Backend4342 PID81950 is
the frontend worker's guarded1ba73e3c snapshot with a fresh temporary disk DB;
test-only fixture patches preserve all production source. Earlier own PID21431
was replaced after fixture500s; task worker PID794 was already stopped. Both
current listeners remain available to root. Details/nonsecret setup literal and
artifact links are in reports/business-workspace.md. Original4326/export and all
other worker fixtures remain untouched. Final Design Studio worker assessment is
provided for root's independent accepted-source review.
