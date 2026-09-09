# Business task → AI session → managed deliverable → human review

Status: docs-only contract checkpoint, 2026-09-09. No new execution API,
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
result. Concrete profile provisioning and wire details are being closed below;
there is no automatic fallback to E1.

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

## Checkpoint limits

Exact DTOs, durable asset publication/recovery, session privacy/cancellation and
profile admission details are still being reconciled from the pinned source.
All proofs above are planned, not run. This PR changes only this contract and its
worker report. Existing intake fixture and user-inspected browser/draft stay frozen.
