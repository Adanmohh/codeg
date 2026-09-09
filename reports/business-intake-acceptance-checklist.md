# Increment B — root acceptance plan

Prepared against frozen contract `670af9ca2b8e3cdb7c0858ab15b58036fafbc2d5`
and accepted access seam `18be55edc276713fc6d46d075baec363245ba285`;
the bounded contract and UI reviews are closed and implementation is dispatched.
No passing B test or live provider access is claimed. Root
orchestrates and independently checks the integrated result; workers own code.
This complements the contract's18 detailed cases with observable user outcomes.

September9 integration update: accepted PR30 handoff
`b3dfbcb602314cee6eb0039d92cb997762c02dd8` and the
[intake tenancy addendum at949adb02](https://github.com/Adanmohh/codeg/blob/949adb02c18c72d0804eeb74b420520a7681ee86/docs/contracts/business-intake-tenancy.md)
supersede the original operator-only Fireflies setup rule. The gates below reflect
that authorized change; this update does not accept PR28 or PR29. Independent
pre-epoch tests are recorded in the integration review, while integrated epoch,
fixture/browser and artifact verdicts remain pending.

| Gate | Required outcome and evidence |
| --- | --- |
| BI-1 Source setup | Current tenant human owner/admin can establish Fireflies only within current Contribute domains and Create publication ceilings, with an active same-tenant pinned source owner. New setup grants no source read; human grants are explicit. Manager/member/agent and foreign or out-of-scope setup are denied. Legacy inbox/product association remains actual operator-only in the original mapped organization. UI uses server setupKinds/setupDomains, not role inference. Secrets are never returned. Credential-store/SQLite failure boundaries and rebind fencing are tested explicitly. |
| BI-2 Bounded import | A permitted human imports a defined meeting window through the real adapter core against a guarded synthetic upstream. Fixed queries, response size/time limits and partial/capped coverage are observable. No live provider, model, bot or external write is needed. |
| BI-3 Durable recovery | Duplicate pages/detail and response loss do not duplicate sources, candidates or accepted tasks. Competing writers, expired claims, cancellation and restart have actual persistent evidence; resume uses a currently authorized human. |
| BI-4 Source privacy | Two independent sessions with different grants demonstrate source visibility separately from task-domain visibility. Private passages, attendee/proof/config fields and hidden counts do not appear in unauthorized results, task activity, search or agent context. |
| BI-5 Human task preparation | A person can inspect exact source passages, prepare a normal task with visible owner/assignee/reviewer/date and explicitly review the destination audience. No chat, repository or running agent is required. Ambiguous source names/dates remain unresolved suggestions. |
| BI-6 Atomic decision | Accept produces exactly one ordinary task and decision/link/activity transaction. Link uses actual target edit permission/revision and preserves task content. Discard creates no task. Failure/replay races leave no orphan or duplicate records. |
| BI-7 Changed source/access | Source refresh retains drafts but requires explicit current-passage rebase. Revoked credentials, grants and binding epochs fence in-flight results and decisions. Tenant suspend/resume invalidates captured processing authority; fresh login cannot reauthorize old observations/previews, including identical-content refresh. NULL historical epochs require explicit new validation. A→B→A is a new revision, not a stale-pass loophole. |
| BI-8 Business continuity | Accepted work appears in My work/shared work and follows the existing human progress/review flow. Assignment does not launch an agent; existing scoped execution remains the supporting path. Email/Hafidh capture does not resend, file an issue, refresh evidence or alter old receipts. |
| BI-9 Actual interface | Playwright CLI exercises the integrated real API with two synthetic sessions: setup/readiness, import/resume, passage selection, draft/accept/link/discard and stale/revoked recovery. EN/AR, narrow/wide, light/dark, keyboard/focus and settled scrolling are checked on the affected flow. |
| BI-10 Integrated artifact | Worker and independent core/permission/concurrency tests, necessary desktop/server/Clippy and frontend checks pass at immutable heads. Root runs the final Design Studio correction loop and normal package/affected native checks; executable/export hashes identify the accepted artifact. |

A source is not permission to publish a transcript. The public task contains
only explicitly reviewed business text; source evidence keeps its own audience.
No automatic external publishing, provider delivery or unattended synchronization
claim. Platform signing/distribution and live service setup remain separate.

The pinned API and staged credential lifecycle govern the implementation gates,
including strict store failure preservation and cross-import source fencing.
Reuse existing fixtures only
with their owner's coordination; root and reviewers create separate synthetic
records and preserve paused work. Do not substitute mocked frontend JSON for the
actual shared API acceptance.

## Unified fixture and final review sequence

1. Tickets publishes immutable compiling source plus one complete green intake
   run and final command evidence, retaining exploratory failures with their
   corrections. Approvals runs the eight949adb02 epoch/migration/HTTP cases in
   its existing isolated target, sequentially with-j2; no desktop build or copied
   build outputs. Three actual Migrator installation paths, receipt retry,
   rollback, retained rows/NULL epochs and foreign keys remain required.
2. Tickets supplies backend4351/upstream4352 absolute executable/source hashes,
   listener PIDs, health/lifetime details and synthetic-only access instructions.
   Identity/settings/tasks/intake must share one database. Each tester owns
   separate synthetic records; old4350/4353 and earlier fixtures are preserved.
   Recorded fixture traffic uses no live provider, engine or native keyring.
3. Rebrand aligns the frontend with that backend and coordinates one export
   window. Approvals rechecks the original IUI-1 probe against the committed
   correction. Real Playwright CLI checks must also prove cancel keeps the exact
   draft, explicit discard opens the requested source, newer navigation defeats
   an older response, and busy/unknown writes remain recoverable before leaving.
4. Root reviews the combined protected flows and runs the final Design Studio
   correction loop against the identified export. Backend tests, component
   probes, browser evidence and native artifact results retain separate verdicts.
   Any final build is serialized within available disk space; no old target or
   fixture cleanup is implicit. Restricted tenant native windows remain disabled;
   wrapper checks cannot close the documented shared-channel isolation gap.
