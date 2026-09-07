# Phase 1 beta runbook — integration in progress

This follows FOUNDING P1/P2, not Phase2 release/social automation. The application
reuses the existing Tasks/worktree/diff review flows. The root orchestrator writes
no product code. Current source of truth: STATUS.md.

## What has been demonstrated locally

| Path | Evidence | Real beta status |
| --- | --- | --- |
| Received email → thread → draft/note → complete human review → provider receipt | Accepted PR6/7; independent protected-API/loopback Playwright flow and Rust assertions | Live inbox/key not used |
| Stale/denied/failed/unknown email | Independently exercised UI and durable state tests | No real email sent |
| Agent token → live task/run → public ticket/draft/proposal | PR8 final independent checks underway | Astra catalogue/provider setup missing; no inference |
| TestFlight read → human proof → GitHub App issue | Accepted read/filing modules; PR9 host/UI under review | Hafidh admin/App credentials and repository not validated live |
| Phone notification → login → exact review | Step3 Telegram worker implementing local fixture | Reachable protected origin/private recipient/bot not configured or activated |
| Fixed-build note | Existing Tasks plus native reply-draft tools will compose it after actual fix/build evidence | No fix/build on Hafidh or reporter delivery claimed |

## P1 beta rehearsal

1. Configure the authorized Hafidh read origin, product project and single-repo
   GitHub App in the host UI once that PR is accepted. Install the pinned Python
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
