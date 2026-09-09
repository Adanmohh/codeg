# Shared business workbench and Sources — PR29 handoff

Product **60d600db0f224d44ff191490ab79bc25530ba959**, branch
`feat/business-intake-ui`, [draft PR29](https://github.com/Adanmohh/codeg/pull/29).
Base accepted `a40b03393a466672060066ae6e0e8c9054a2349d`.
Published source/evidence handoff **08417b4ff**. The final report/evidence commit
follows without another product change. Root retains integration and acceptance.

The actual corrected4355 recovery and bounded Sources design checks pass. The
shared workbench delivers List/Board/Table, persistent tabs/panes, scoped settings
and protected source-to-task decisions. It **does not yet deliver the owner's
required persistent AI/terminal and managed Documents & assets workflow**. The
concrete next implementation mapping below preserves the same workspace.

## Corrected4355 export and preservation

**http://127.0.0.1:4355/business**, Node **85232**, new export
`/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand/.build/business-intake-ui-recovery-60d600db0`.
Built once from docs/helper head `e9b69ee518d12fab465acbfd7ff78e44285017a8`,
product60d600db0, with `CODEG_EXPORT_DIR` and telemetry disabled. Exit0;
34 static routes. No new Rust target, native bundle, dependency or old output copy.
Before launch4355 had no listener (`lsof` exit1/no output) and disk had8.6Gi free.

HTML disk/HTTP SHA256:
`6fd453d6e8153f16119f882fe3766644f3dbd2085e5956ebc6dee2391ff1f65f`.
All27 HTML/JS/CSS assets match over HTTP. [Manifest](business-intake-ui-evidence/recovery-4355-60d.sha256)
SHA256 `6d0ee0ca8c48350589dd94a0b70a1eff5f873acb7e8fc60033a64385d66adbc3`.
[Metadata](business-intake-ui-evidence/recovery-4355-handoff.json) records the
helper hash, source, build head, PID and guard outcomes.

`serve-recovery-4355.mjs` preserves the fixed loopback4351 API allowlist,
rejects supplied foreign/null Origin403, legacy routes403 and non-POST context403.
Same-origin unauthenticated context401 proves inner auth remains active. It never
forwards Origin or exposes WebSocket/global legacy routes. No backend allowlist,
fixture or authentication implementation changed.

Backend/upstream **4351/4352 PID66200** remains tickets-owned: fixture
`7ed0dd0c28f5065f1c37983f366cc6dcb0727e08`, production
`e55f3bfd1f069d6d6111370223993596b19ecb9b`, handoff1042a0b7. One real protected
identity/settings/tasks/intake DB; only the external Fireflies reader is synthetic.
Own browser mutations use **ui only**. Login reads ui's assigned0600 credential
file directly into CLI memory, fills Workspace address4355 and Personal access
token, then waits for the form to unmount. No token in URL, legacy storage, report,
request log, filled-form screenshot or runtime response interception.

New owned sessions: **intake-recovery-a4355** (CLI controller95907, ui owner) and
**intake-recovery-b4355** (96312, ui manager). Both retain the saved candidate3
review without publication. Root/reviewer may use their separately allocated
namespaces and fresh sessions. Do not mutate another namespace's records.

Preserved unchanged:

- **4354 PID81173**, export `.build/business-intake-ui-unified-1974d97f`, product
  `1974d97f44b69db96051ace1c68717cc8c27b6e9`. HTML SHA256
  `42f37747c38d180ef270e59e0a7956bfc48c848ee35782e4a5b35c9c1ae2c6b7`;
  manifest `9b53750c2b200d47cd20f5e5a7272005b5fac5a12ea3d5d9a6616c727ca377a1`.
  User-inspected `intake-worker4354` and root `root-intake-unified` were untouched.
  Own `intake-unified-a4354`/`intake-unified-b4354` remain separate and open.
- **4350 PID90939**, `.build/business-intake-ui-nav`, c1e618de; HTML
  `2d3f87ea738e4b191dd77f43373c34f2540e8f8beeba59a81d4542670bb38b1a`.
  Old `intake-ui-a4350` retains its unsaved private task draft; b4350 remains open.
  Its task/settings authority is4353, not the new unified DB.
- **4353 PID99613**, isolated29774b50 task/settings fixture, all older4326/4340/
  4342/4346/root fixtures, native outputs, paused report and stopped old Codex40904.

## Actual Sources flow and recovery evidence

| Protected UI flow | Actual result |
| --- | --- |
| Initial owner/manager | Empty owner has setup; manager has none; neither inherits legacy entry or stored credential. `unified-empty-{owner,manager}.raw` |
| Tenant Fireflies setup | Binding `06587e8e-a83d-47dd-bd34-ebb24a3ccf44`; only returned Fireflies/Feedback scope. Created disabled with zero grants. Human explicitly grants retained-history/current/future read/import/triage and Feedback publication to owner/manager, then enables. Key cleared. `unified-setup-after.raw` |
| Bounded real import | Import `658cdac0-68ba-45c1-a38e-74319ea0fb5a`: queued→complete, three records, zero failures, bounded_end. `unified-import-final.raw` |
| Original passage→private draft→full audience | Human selects source passage, writes exact title/brief, keeps calendar date2026-11-02, chooses people and confirms complete destination audience. No source text is automatically published. `unified-accept.raw` |
| Explicit acceptance and receipt | Candidate `4abb3601-d064-4818-ba33-510cfc98e517` accepted200, task `5e9d66ac-1286-4e70-a17e-6eb79db7ab6b` opens in actual shared task detail. Confirmation initially disabled. |
| Text-free link | Candidate `c07df0e3-761a-44dc-8401-6434404844f8` linked200 to existing target `bbf2436d-e84c-480b-91e6-bfd321cc07e7`; title/brief unchanged, revision1→2, execution null. Explicit target/domain confirmation. `unified-link-after-refresh.raw` |
| Corrected empty editor4355 | Seeded unprepared candidate `675fe092-1265-4201-9117-863e2dde2dde` opened after real expiry: metadata_only, no title field. Explicit upstream refresh queued→complete restores empty editable fields/selection; blank Save remains disabled; human preparation saves200 at revision2; publication unchecked/disabled. `recovery-empty-editor.raw` |
| Prepared local draft4355 | Another successful source refresh preserves exact unsaved title/brief, no current-version comparison or task write. `recovery-retained-resume.raw` |
| Actual concurrent correction4355 | Manager saves candidate2→3. Owner's stale save409 retains exact local draft, shows base2/current3, disables Save. Explicit saved adoption replaces fields only then and resets publication confirmation. `recovery-concurrent-manager.raw`, `recovery-explicit-adoption.raw` |

The source-review pending candidates are intentional backend seeding in
`src-tauri/src/business_intake/sources.rs@e55f3bfd` (283–310), not automatically
created tasks. Link's original target was Todo; this UI run does not claim a
Review→In progress demonstration. No paid model, engine, live provider or send.

Independent evidence is separate: reviewer **6895bedf4** closes IUI-1 with11/11
real browser checks at1974 and27 matched assets; task B entry prompts, Keep editing
retains A across1280→390, explicit discard opens B,26 reads/no writes and saved A
unchanged. Root **353b3a07** reports12 EN/AR/theme/width draft/storage checks.
Reviewer additionally reports five focused60d source tests passing and a genuine
1974 negative control (missing-title failure; newer-version adoption passes).
Those are attributed reviews, not additional own runs.

Probe accuracy: earlier setup/close/source selector/search label mistakes are
retained with their errors. Expired controls required real upstream refresh.
During the prepared-draft refresh, source revalidation collapsed the existing
refresh disclosure, so the first script timed out waiting for its hidden next
step. The continuation reopened that disclosure and advanced the **same queued
attempt**, preserving the draft; it did not start a replacement or blind replay.
The empty-editor script's result originally omitted savedRevision by reading a
CandidateDetail as Candidate; actual subsequent manager read verifies saved2.
Five earlier failed raw files had only trailing blank EOF lines normalized for
`git diff --check`; no failure text/assertion was removed. No blanket CLI pass.

## Bounded Design Studio closure

Applied Design Studio55c8614dcfff33b4caa5a544b4f1f91877214878 methods inline:
art direction, component/checklist, aesthetic/a11y/flow synthesis; no agents or
paid runner. Existing [BC ledger](business-intake-ui-evidence/design-checklist.md)
and its Input/Modal/Saving changes references ground this bounded review.
The root CLI Canvas ancestor-composition probe runs through installed Playwright
CLI; only pure `buildReport` executes outside the browser.

[Corrected matrix](business-intake-ui-evidence/recovery-matrix.json):12 EN/AR,
light/dark,390/768/1280 cases, **840 direct painted-text samples, zero sampled
contrast failures, zero unnamed interactive controls, no document overflow**.
Exact local title/brief/date survive; manual tab-arrow focus does not activate
another pane; visible Back focus remains at least40×40 and reverses in RTL.
Reduced-motion state retains one editable task and no overflow. BC-4/6/9/12–15
have direct bounded evidence. Screenshots inspected at desktop English light and
390 Arabic dark show readable wrapping, clear private/source/draft separation and
visible focus. These are the actual protected pages, not intercepted fixtures.

[Before defect](business-intake-ui-evidence/unified-fresh-empty-draft-diagnostic.png),
[restored editor](business-intake-ui-evidence/recovery-empty-after-1280-light.png),
[desktop](business-intake-ui-evidence/recovery-1280-light-en.png),
[narrow RTL](business-intake-ui-evidence/recovery-390-dark-ar.png),
[exact current comparison](business-intake-ui-evidence/recovery-current-before-adopt-390-light.png).

Scope limits: this matrix samples the follow-up reader/editor, not every long
passage/placeholder/opacity/gradient, every receipt state or the whole app. It
measures no CLS/long-frame claim. Prior c1 measured workbench/primary-action/
selected-navigation matrices remain valid for those unchanged product seams,
with exact artifacts and before/after failures in the evidence directory. No
new global token/font or unrelated cosmetic change was made to obtain a score.
Full email/Hafidh projections, response-lost final decisions, changed-source
rebase and final independent whole-B design acceptance remain separate gates.

## Product contracts, borrowing and commands

Accepted B670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5/access18be55ed and UI plan
1a876afc govern25 closed intake operations. Exact compiled types f51154b9 plus
949adb02/e55f3bfd retain response-only setupKinds/setupDomains, source freshness,
capabilities and explicit null drafts. Settings use accepted29774b50/PR30;
restricted native tenant windows remain unavailable. No caller epoch/tenant
selector, native member fallback, source grant inference or second task gate.
Root owns accepted backend/main integration; fixture equality is not merge acceptance.

| Borrowed Apache-2.0 Codeg source | Adaptation |
| --- | --- |
| `src/lib/business/{client,identity,tasks,presentation}.ts` at a40b0339 | Closed intake/settings client, canonical task references and role/domain projections |
| `src/components/business/{task-form,task-detail,ui,workspace}.tsx` at a40b0339 | Task fields, CAS/adoption, source-entry unsaved guards and role shell |
| `src/app/workspace/layout.tsx`, conversations/detail, tabs/tab-bar, `src/lib/tab-group-layout.ts` at a40b0339 | Stable keyed surfaces, existing geometry/resize primitives; no engine/provider mounting |
| UI resizable/overlay-host-hidden/dialog/drawer and use-media-query at a40b0339 | Same scoped portals/responsive behavior, preserved shared Drawer |
| globals.css/theme-presets.ts at a40b0339 | Existing semantic palettes scoped to the business subtree; no global overwrite |
| candidate/source-review/imports at1974d97f |60d same-revision fresh editor recovery; explicit current adoption/withholding unchanged |
| own serve-unified.mjs at60d600db0 | Fixed4355 report-only proxy with supplied-Origin guard |

Original Codeg v0.30.4 pin **6f6bd648b206412644842a98d9ffeebf57292bed**,
Apache-2.0 attribution/LICENSE and every existing NOTICE block retained. This UI
copies no IntroMail provider, Edublend proprietary source, Payload implementation,
AGPL/GPL or enterprise source. New code is scoped glue around approved components.

Installed sources read: React19.2.4/@types-react19.2.13, Next16.1.6 static export,
Tailwind4.1.18, TypeScript5.8.3, next-themes0.4.6, panels2.1.9, TauriAPI2.10.1,
Testing Library16.3.2/DOM10.4.1, Vitest2.1.9, CLI0.1.18/Playwright
1.63.0-alpha-2026-08-05, Node24.19.0/@types-node25.2.2. No dependency change.
Separate React manifest/Cargo reads precede resumed edits; live audit records
PreToolUse and PostToolUse for **01a084ee-12a8-7833-ace9-f3f4985ba926**, exact own
worktree, epoch1788948436–1788948448 and subsequent calls. Hooks stay enabled.
Offline code-context guide exit0; docs exit3 honestly reports missing rebrand.db.
Official Node HTTP/FS docs were read using **gh api** at
cdc1b38d40cb567b7ad0b39c86addf830a0af0ae after local types. No pin upgraded.

| Check | Result |
| --- | --- |
| Recovery negative control before product edit |1 missing-title failure/1 explicit-adoption pass; original erroneous locator recorded separately |
|60d focused intake workflow/source-entry/setup tests |33/33, exit0,1.93s; `unified-recovery-after.log` |
|60d `pnpm exec tsc --noEmit --incremental false` |exit0; `unified-recovery-tsc.log` |
| Scoped candidate/workflow ESLint |exit0; `unified-recovery-eslint.log` |
| `CODEG_EXPORT_DIR=.build/business-intake-ui-recovery-60d600db0 NEXT_TELEMETRY_DISABLED=1 pnpm build` |exit0,34 routes; `recovery-4355-build.log` |
| Fixed helper syntax /27 HTTP hashes / five guard probes |all expected results; handoff JSON |
|4355 actual expiry/revalidation/local draft/CAS/adoption |pass, with the disclosed disclosure-panel probe retry |
| Prior source-entry races/setup/workbench gates |52/52 at a58; eligibility27/27 at1974; prior scoped tsc/lint/export pass; exact history in prior report08417 and existing logs |

No new Rust product change requires repeated native/server/Clippy here. Prior
backend-owner gates are separate. All local secret scans emit counts only; the
current scan artifact records the checked file count and zero ui-secret matches.

## Next: same-workspace AI and managed assets

UI reconciliation now targets **3f164c2a989cd08a523e51f1e47559c15a48c0ef**,
`docs/contracts/business-ai-execution.md`, read via gh api after full f2af30f7 and
f454db75 addition reads. This supersedes the early d1f22f85 plan. Root product map
`docs/BUSINESS-WORKSPACE.md@f30a27da45c41ed7593c5da433759af8314a08ef` was read
completely. Contract operations are proposed, not implemented/usable today.

Minimum usable E1: original operator opens an authorized human task, chooses a
returned ready profile/mode, starts one non-Git persistent session beside that
task, prompts/continues/inspects/stops through the actual scoped adapter. An actual
synthetic ACP/PTY process produces a document and deck; output inspection imports
retained immutable versions. A Documents pane finds both, previews supported bytes,
downloads a deck if no renderer exists, and offers explicit exact-version task
submission. The named member reviewer discovers selected `Deliverable.assets`
after reload and reads them through task-owned get/content before existing human
review. No member visits E1 private execution routes, no textarea/path hint counts
as a managed document, no task/approval status changes merely by opening a pane.

Exact frontend sources inspected at60d600db0 (original Apache Codeg; relevant
imports/props/effects and named functions, not a claimed full3000-line panel audit):

| Existing file / exact seam | Reuse and required typed boundary |
| --- | --- |
| `business/workbench.tsx`: `WorkSurface.render(visible)` and stable keyed panes; blob8d46bb7a9ada8d0e25e89da7dc00a03eff8b9f0b | Add real session/terminal/asset-version surfaces to these same tabs. Key private state by server/org/member/session and assetId/versionId. Closing detaches; explicit Stop is separate. **PR29 must be accepted/merged before E1 shell wiring.** |
| `business/workspace.tsx`, `business/task-detail.tsx` | Task action opens/reopens the authorized session; task remains alongside. Extend existing deliverable rendering with `PublishedAssetRef[]`, keeping text rows `[]`, Task Review and CAS. No shadow task model/engine. |
| `chat/chat-input.tsx`: status, onSend/onCancel/config/queue props; `chat/message-input.tsx` | Preserve composer appearance/send/stop/IME behavior. MessageInput also invokes legacy skills/reference/attachment helpers and optional persisted drafts; it is **not** a safe direct business mount. Inject scoped sources or use its existing lower-level controlled composer. Personal shortcut hook itself reads personal browser preferences, not host config. |
| `chat/composer/rich-composer.tsx`: `RichComposerProps`/handle; blob7d1a646bc1bf0ad0692dd36974f4799f431341f8 | Existing Tiptap editor supplies controlled draft, IME-safe submit, focus and optional reference callback. New glue owns in-memory draft and closed InputRef selection; no arbitrary path/session/skill picker. No new editor dependency. |
| `conversations/conversation-detail-panel.tsx`: ConversationTabView, ChatInput wiring and runtime actions; `message/message-list-view.tsx` | Retain transcript/tool/message composition and ordered view behavior. Existing panel/list subscribe to global numeric conversation stores and legacy load/actions; extract/inject the authorized transcript model, not a fake numeric host conversation. |
| `contexts/conversation-runtime-context.tsx` | Provider is a passthrough; state actually lives in global Zustand. A wrapper alone does not isolate a tenant. The new scoped session/event cache must be explicit and clear only on authorized scope exit. |
| `terminal/terminal-panel.tsx`, `terminal/terminal-view.tsx`; view blobffe2fa67e6c9491198438176a62f0c900e44afea | Reuse existing xterm rendering, Fit/web-links/ligatures and mobile keybar/write ordering. View currently subscribes globally and spawns legacy terminal from cwd/shell/command on mount; replace those callbacks with authorized session/generation attach/write/resize. It must not spawn on mount or use caller cwd. |
| `contexts/terminal-context.tsx` | Current folder/system terminal settings and legacy kill/create helpers are host authority. Do not mount this provider on the business route; session controller owns current capabilities and detach/stop distinction. |
| `message/reply-artifacts.tsx`, `lib/session-files.ts` | Current completed-reply cards parse tool file hints and open local paths. They can suggest a backend-returned OutputCandidate only after resolution; hints never authorize bytes or publish an asset. |
| `files/office-preview.tsx`, `lib/office-actions.ts` | OfficePreview starts a legacy path/port watch; shortcuts merely inject skill prompts. Reuse layout only with version-authorized content. E1 first supports safe text/Markdown preview and exact download; no claimed slide rendering/install if unavailable. |
| `app/workspace/layout.tsx` provider composition | Rich composition is the design foundation, but its all-settings/Git/ACP/remote/global provider tree must not be mounted wholesale for a member. Reuse components after scoped adapters, preserving cold route isolation. |

Typed transport needs are now concrete: authenticated POST NDJSON snapshot/replay/
event/heartbeat/detached, opaque cursor and generation checks, bounded split-UTF8
parsing and snapshot reset that preserves unsent draft. Deduplicate cursor/message
part; unexpected EOF triggers receipt/reconnect, never prompt/terminal resend.
Task-owned public version metadata is a distinct safe projection without private
session/turn/profile references. Content fetch validates exact version size/hash,
uses no token URL, then creates/revokes a local Blob URL; no Range or active
same-origin document. E1 refuses account_snapshot and every non-original-operator
private operation. E2 manager stop/native isolation remain unavailable here.

Remaining small DTO reconciliation for tickets before relevant wiring: give exact
JSON envelopes for assets/list and assets/get/current capabilities, a way to
rediscover retained **private older version IDs** after reload (AssetSummary only
names latest), and the precise successful prompt receipt fields (message ID/hash).
Event/content framing and public reviewer metadata gaps are closed by3f164c2a.
Preparation can implement the fully specified types/parser/components independently
while these fields are closed; no `unknown` payload is treated as usable data.

New `feat/business-ai-workspace` will start from accepted main for isolated typed
client/component preparation. It must not copy or import PR29's unaccepted shell;
wire the same surfaces only after root merges PR29 and accepted main is integrated.
No operational routes, native fallback, provider/profile launch or runtime AI
acceptance before the independent authority verdict and real backend handoff.
`reports/business-ai-workspace.md` owns subsequent work. Marketing account identity,
freshness, capabilities and future calendar/performance remain real data contracts;
no empty module promises, fake metrics or social actions are added to this handoff.
