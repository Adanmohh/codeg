# Phase 1 beta runbook — local validation and live setup

This follows FOUNDING P1/P2, not Phase2 release/social automation. The application
reuses the existing Tasks/worktree/diff review flows. The root orchestrator writes
no product code. Current source of truth: STATUS.md.

## What has been demonstrated locally

| Path | Evidence | Real beta status |
| --- | --- | --- |
| Received email → thread → draft/note → complete human review → provider receipt | Accepted PR6/7; independent protected-API/loopback Playwright flow and Rust assertions | Live inbox/key not used |
| Stale/denied/failed/unknown email | Independently exercised UI and durable state tests | No real email sent |
| Agent token → live task/run → public ticket/draft/proposal | PR8/15 accepted; real companion/adapter cached reads, exact native proposals and scope/cancellation checks pass | Astra catalogue/provider setup missing; no inference |
| TestFlight read → human proof → GitHub App issue | Accepted read/filing modules and PR9 protected host/UI; local issue/held-task flow passed | Hafidh admin/App credentials and repository not validated live |
| Phone notification → login → exact review | PR10 email and PR12 issue fixtures; independent protected approval and no-resend recovery passed | Reachable protected origin/private recipient/bot not configured or activated |
| Fixed-build note | Existing Tasks and accepted native reply-draft/proposal tools support the evidence-based brief below | No fix/build on Hafidh or reporter delivery claimed |

## P1 beta rehearsal

1. Configure the authorized Hafidh read origin, product project and single-repo
   GitHub App in the accepted host UI. Install the pinned Python
   adapter in its dedicated environment; a packaged app needs a valid host
   CODEG_INTAKE_PYTHON path. Never point an agent at the admin credential.
2. Read TestFlight, choose the real report and refresh it. Fill only actual
   build/screen/reciter/log proof, capture/session metadata when known, and
   human-confirm severity/labels. In-app feedback has no upstream read route;
   missing app-version or diagnostic bytes remain missing.
3. Use a real active triage task in the configured project. The agent proposes
   the prepared revision; review the exact repository/title/body/labels before
   any issue is filed. Local fixture filing does not authorize a live issue.
4. Create the linked held fix task, inspect it in Tasks and explicitly resume
   when ready. The inherited engine supplies its worktree and diff-review
   lifecycle; initially its deliverable is a fix plan. A held task is not a
   completed fix. Review the actual implementation and checks before landing it.
5. After a real reviewed fix and build exist, create a follow-up existing Tasks
   report task using the brief below. Identify the reporter through an explicit
   existing support thread; the public intake projection deliberately contains
   no private contact identity.
6. Review the resulting email proposal in full and approve only the actual
   intended send. Build distribution/store submission remains a separate action;
   Phase1 does not implement a release engine.

### Follow-up task brief for a verified build

Use the actual issue, reviewed fix report/diff, completed check results and
verified build number supplied by the operator. If any is absent, report what is
missing and do not state that a bug is fixed. Use desk_context/desk_tickets/
desk_thread to read the explicitly selected reporter thread. Draft a concise
“fixed in build N” response only when the evidence supports that statement. Save
the complete reply with desk_save_reply and propose that exact saved revision
with desk_propose_reply. Do not send, approve, release or invent build evidence.
The human reviews all recipients and text through the existing Ops gate.

## P2 beta rehearsal

Connect a real Resend inbox only when its scope/key are supplied and reviewed.
Pull now performs a bounded read; repeat to verify source-ID deduplication.
Create a real running task for the inbox, use the native Desk tools to draft
and propose, then review the exact reply. No scheduled polling or autonomous
acknowledgement is claimed by the accepted implementation; manual pull and
human-approved replies are the currently delivered path. Unknown provider
outcomes remain blocked against blind resend; local receipt completion is a
database-only operation.

## Completion boundary

The local release gate is accepted integrated source, focused regressions,
actual Playwright CLI P1/P2/phone fixture flows, final Design Studio
audit/fix/recheck, and a rebuilt unsigned dev app. Live Hafidh beta data, real
provider receipt and actual phone delivery are separate evidence. Configuration
gaps or absent upstream endpoints must remain explicit; synthetic fixtures do
not turn them into live passes.

## Root runtime preparation — 2026-09-08

Root created its own ignored `integrations/hafidh-intake/.venv` using CPython
3.13.14, installed the accepted 37 exact requirements.lock pins and editable
hafidh-intake0.1.0. All three README installation commands exited0. An
`env -i .venv/bin/python -I` import check passed for MCPServer, Settings and
package metadata (mcp2.0.1/httpx0.28.1/pydantic2.12.5). No tracked manifest/lock
changed, no other environment was modified and no live credential was used.
The accepted host module now imports successfully under the isolated Python
command. Subsequent native build/startup and final browser validation are in
[native build evidence](native-phase1-build.md); this does not validate live access.

## Local artifact

The unsigned macOS app is built at
`src-tauri/target/debug/bundle/macos/Hafidh Ops Desk.app`. Its version remains the
inherited0.30.4; source commits and binary hashes in the native report identify
this Phase1 build. The Python intake adapter uses the dedicated environment
above and the explicit CODEG_INTAKE_PYTHON path; it is not a standalone portable
Python distribution. STATUS.md identifies the latest accepted design/build gate.

Root's startup test used CODEG_DATA_DIR and CODEG_HOME set to
`/tmp/ops-desk-native-data`; it upgraded only that isolated existing test database
and stopped its own native process afterward. Actual browser fixtures used
loopback provider implementations. No real email, issue, Telegram notification,
agent inference, store release or deployment follows from those local passes.
