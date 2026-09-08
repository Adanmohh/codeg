# Pi P1 bridge — implementation checkpoint

Work in progress on `feat/step3-pi-issues`, sole tickets worktree. Base is accepted
main `f4da70275932fcb527e9bd86946f740b3be515e5`, including PR13 merge
`783bfb9cd9caf9546f6ef9effc067f5c904fc6be` and accepted PR9 host. No PR12 source
is copied. Draft PR: https://github.com/Adanmohh/codeg/pull/15.
Initial contract commit `70ed832a` and implementation checkpoint `b15f11a7`
are pushed. Accepted main `126e7f24b80f42e8bfe30b6141ad1335359f87f9`
(including PR12 merge `e9ddab88dfc45393aab68de52db8d3848b0bb891`) was merged
without conflicts as `ae131cf871b1092018388e2cd9d814be9bdc8eb3`. Its
`notice.rs`, host registrations and root planning docs remain byte-identical
to that accepted main. NOTICE retains every accepted section plus this port.

## Contract and ownership

- Native `desk_propose_issue` accepts only `{draftId: string,
  expectedRevision: positive i32}`. The existing token registry/listener and
  task engine supply parent connection, live ancestry, task/run, stable `pi`
  principal and configured account. Host `agent::prepare_and_propose` remains
  the only preparation/proposal implementation. Project/folder/product/repository,
  human evidence and human-confirmed severity come from accepted stored state.
- Three MCP reads on the fixed `hafidh` companion: `hafidh_feedback_list`
  (bounded local page), `hafidh_feedback_get` (source ULID),
  `hafidh_intake_status` (empty input). All input objects are closed. No input
  account/product/folder/identity, credential, URL/file, proof, refresh, import,
  configuration, approval or execution capability.
- Reads derive a single enabled product from the live task's existing folder
  and account. Missing and ambiguous product binding fail explicitly; foreign
  records never appear. Host-owned read helpers will reuse the accepted live
  run check inside the same database read transaction. Any visibility-only
  change needed to reuse that checker will be documented; ACP will not copy it.
- Read descriptions and results identify **cached** semantics. Human import
  and successful human refresh alone mint upstream freshness. Cache absence,
  unverified/invalidated and expired data identify the required operator action.
  Reads may report stale public metadata, but never silently claim revalidation.
- Public draft projection includes existing draft ID/revision, title, labels,
  confirmed severity, source revision match and whether preparation belongs to
  this current run, so Pi can choose an existing human-prepared revision.
  Proof objects, credential/configuration references, private reporter fields
  and raw host errors stay out of the agent response.
- The launcher supplies the actual fixed companion path and generated socket/
  token through trusted per-launch plumbing. The installed adapter gets only
  this scoped read server, with one-use broker validation and no session grants.
  Generic runtime variables never configure a Hafidh provider process.
- Existing cancellation, revocation, stable `pi` deny/scope precedence, Astra/max
  setup guard and destructive human floor remain. No model or provider call.

Owned changes: host `agent` cached-read helpers/tests; ACP Desk protocol, existing
companion feature registration and trusted launcher; extension schemas/broker;
focused fixtures and this report. Approvals owns host `notice.rs` and typed phone
review. No migration, shared planning document, locale, Ops UI or dispatch change
is planned. Any host module registration is additive only.

## Grounding / progress

Read complete FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS and
`reports/pi-p1-follow-on-checklist.md`. Read accepted host preparation/store/DTOs,
Ops RunContext and Pi task/extension/launcher source before designing this seam.
Installed React is 19.2.4; Cargo manifest read separately. Full pinned source and
license mapping will be recorded before implementation handoff.

Code-context used existing rag-skills `.venv/bin/python` with `HF_HUB_OFFLINE=1`:
guide exit 0 (source reuse, strict TypeScript, actual headed Playwright CLI),
dependency docs exit 3 because `data/code/tickets.db` is absent. No corpus coverage
is invented; installed package sources/types supply the missing grounding.
Owner's gh-api-only remote research and no-agent instructions override generic
guide suggestions. Hooks remain enabled. Live PreToolUse observed for this tickets
session `01a07c1c-d82f-7022-84db-778a438632f1` at timestamp 1788828696; paired
PostToolUse evidence will be recorded with final bounded safe-field audit output.

Clean check, `git fetch origin main` and new branch creation exited 0. Installed
`git switch -h` exits 129 for help; `gh api --help` and `gh pr create --help` exit 0.
Initial server/companion check and strict Node typecheck pass (exit 0). The first
four full-path P1 bridge tests pass (exit 0): cached/no-write states, closed and
foreign/ambiguous inputs, exact payload/pending floor/stable Pi deny, and the
cancel-writer/peer-abort race. Extension unit tests pass 16/16 (exit 0). An initial
Rust test compile required an explicit `Option<(Arc<DelegationListener>, String)>`
fixture annotation; no product gate was waived.

Implementation now projects cache state and public draft metadata without proof
objects or rendered evidence-bearing body, calls the accepted host proposal seam,
and registers an `intake` companion feature independent of native Desk mutations.
Two one-line visibility changes reuse `ops::agent::require_live` and
`ops_intake::SourceRef::validate`; neither changes validation semantics. Host
`notice.rs` and shared host registrations remain untouched. The generated fixed
companion environment overrides caller runtime plumbing. `/desk-status` reports
actual registered read-tool names for no-inference discovery checks.

Integrated progress: six P1 bridge tests pass, including bounded pagination,
unchanged draft preservation after source import, human edit/CAS, revoked proof,
stable Pi deny and cancel/peer-abort. The pagination test first used an invalid
non-SHA source revision; accepted host validation rejected it. Correcting that
synthetic fixture to a valid different SHA yields 6/6, exit 0.
Implementation checkpoint `0d18f665` is pushed. Default desktop check passed;
Clippy identified a redundant reference in the manual browser fixture, corrected
without an allowance. Final runtime/Clippy sequence is running. Root separately
reports its earlier four-test run at `ae131cf8`; the additional source-revision
and revoked-evidence cases here are this worker's newer evidence.
Frontend and strict Node typechecks and the isolated Next16.1.6 static export
(`src-tauri/target/pi-issues-export`) pass, exit 0. Before this merge, real
companion/extension Vitest passed 19/19 and the extracted-assets actual installed
Pi/adapter fixture passed 1/1, with zero model messages/provider connections.
Final integrated reruns follow restoration of the own real companion.

The browser fixture reuses accepted host loopback/Python and human-evidence
helpers, then executes all three reads through the real companion/listener
and a native framed proposal through the same live engine. It asserts cache
bytes and upstream read count stay unchanged during agent reads, exact host
payload persistence and default pending scope. Actual CLI UI evidence is pending.
The own Python3.13.14 environment was installed offline from the accepted
requirements.lock; no lock/dependency changes or other worktree writes.

Live hook pairs were verified again at timestamp1788830836, PreToolUse and
PostToolUse, for the same exact session/worktree above. No hook is bypassed.
Installed adapter source blob hashes for types/index/config/server-manager/init/
metadata-cache/tool-approval/LICENSE all match `gh api` at immutable
`10a45367e033a32026987a75d6f401e37340c86f`.

Remaining: final desktop/server/companion checks and Clippy, combined Desk/host
regressions, restored-companion/extracted-assets tests, actual synthetic CLI
proposal review on owned4324, and final report/head handoff.
