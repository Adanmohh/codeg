# Step 3 — Telegram Ops review notifications

Active implementation, 2026-09-08. Sole writer in
`/Users/mohamedadan/projects/_worktrees/ops-desk/approvals`, branch
`feat/step3-telegram`, created clean from accepted origin/main
**f76503792e18445c867e565534c1f0a11f6aa786** after PR7 merge
**f9ae7f1ec91fb0a9569f06fa9dddbde2884db1c6**. No extra workers.
This early contract is not an implementation or final design acceptance claim.

## Ownership and fixture coordination

New owned modules: `src-tauri/src/ops_telegram/`, corresponding commands/handlers,
`src/components/ops-telegram/`, `src/lib/ops-telegram/`, static review/settings
pages and minimal accepted Ops/channel entry points. Reserve
**m20260908_000007_ops_telegram** for notification configuration, snapshot binding
and delivery dedupe; bug host owns 000006. Preserve all accepted migrations and
NOTICE entries. Root planning documents are read-only.

Read the rebrand worker's `reports/bug-workflow.md` directly. Its Bug intake
destination uses the workbench/sidebar and separate host/frontend modules. This
task will use a separate `/ops-review` static page for phone links and scoped
Telegram settings, with at most a small settings link from the existing Ops
header. No new workbench route or changes to the bug host's live modules.
The accepted email review remains authoritative; a typed GitHub review extension
will be coordinated with the separate host rather than copying uncommitted code.

Root's fixture **PID 30815 / port 4320 remains running and unchanged** for final
design. Its static `out/` must also remain unchanged while in use. New Telegram
fixture port is **4323**, with a distinct temporary DB, synthetic credentials,
loopback provider and Playwright CLI session. Isolated build/export setup will be
documented before running it. Root's 4318 and bug host's 4322 are also untouched.

Include the requested **BC-14 receipt_recorded** fixture: a controlled provider
receipt already stored with local recording pending. The real UI's Finish
recording action must create the public message without a second provider call.
This is database-only recovery coverage, not unknown-outcome reconciliation.

## Notification and authorization contract

- Reuse Codeg's existing Telegram backend, channel configuration and keyring
  token lookup. Never enable a channel, start polling, test an existing live bot
  or send a real notification during implementation/tests. New Ops notification
  configuration defaults off and requires explicit operator setup.
- Notification recipients are explicitly authorized **private** recipients,
  bound to a selected Telegram channel and canonical numeric user/chat identity.
  Groups/topics/usernames cannot silently become private destinations. Revalidate
  the target and configuration before dispatch; no caller-supplied arbitrary URL,
  payload, sender or execution action enters the send path.
- Store explicit account, proposal, task, run and complete-payload snapshot hash
  with the recipient/configuration binding. The external message contains only
  fixed review wording and an opaque review URL. No mail subject/body/recipients,
  diagnostic logs, operator credential, snapshot payload or execute capability.
- The review URL is a locator, not authorization. Mobile login uses the existing
  operator boundary, preserves only a validated internal review locator, loads
  the full payload through the protected API and enforces the existing human
  floor and exact edited-payload approval. No open redirect or token in a URL.
- Existing ACP `/approve`, `/deny`, and `approve always` remain ACP-only. No Ops
  approval callback or direct phone approval command is being invented. Unrelated
  recipient/topic/sender inputs must never resolve an Ops proposal.
- Reuse the existing channel scheduling seam for optional bounded queue scans,
  not a new task/notification engine. Durable uniqueness and writer-first claims
  dedupe repeated scans and concurrent attempts. Ambiguous outbound outcomes do
  not trigger blind retries or rich-text fallback sends. A stale link cannot
  revive a denied/canceled/edited proposal or an old task run.

Source finding: Telegram's inherited rich/interactive methods retry **any**
failure as plain text. Ops notification dispatch must use one plain-send attempt
and persist an unconfirmed outcome instead. The inherited `send_message` currently
accepts an empty returned message ID; Ops must require positive provider receipt
evidence before marking notification delivery. These are scoped adapter needs,
not authorization to rewrite the general channel engine.

## Docs-first and borrowing evidence

Read complete ORCHESTRATOR, FOUNDING, STATUS, DECISIONS, AGENTS and
`reports/telegram-integration-contract.md`; also the design acceptance checklist
and bug-workflow coordination report. Applied code-context, frontend-design,
design-checklist, mobile-ui and the already-read Playwright CLI skill. Existing
neutral light/dark surfaces, Hafidh accent pairs and user typography stay intact.

Separate React 19.2.4 package and Cargo manifest reads exited 0. Offline
code-context guide used the existing rag-skills `.venv/bin/python` with
`HF_HUB_OFFLINE=1`, exit 0. Relevant rules: actual Playwright verification;
atomic explicit-file staging; one route per workflow step; human approval.
Dependency query exited 3 because `data/code/approvals.db` is missing. Use local
installed source/types; no invented corpus coverage or model download.

Live docs-first audit records for session
**01a07c1c-d3a3-7c22-a5b6-cedce2970d8d**, this worktree cwd:
PreToolUse **4568 / 1788820633 / exit 0**, PostToolUse
**4550 / 1788820542 / exit 0**. These read commands recorded context_emitted=false;
the source-write hook must emit before implementation writes. Hooks remain enabled.

Borrowed Codeg source remains **v0.30.4**, Apache-2.0, immutable commit
**6f6bd648b206412644842a98d9ffeebf57292bed**. Local source reads preceded gh api
tree verification. Exact upstream blobs:

| File | Blob SHA | Use |
| --- | --- | --- |
| src-tauri/src/chat_channel/backends/telegram.rs | 1a55a50f181cdbe220fcf7299951171b7132ca04 | Existing send/config/recipient primitives |
| src-tauri/src/chat_channel/manager.rs | 6c49b49af5ecf4580e915bf8d08896d738e36a31 | Shared lifecycle; do not start polling for Ops setup |
| src-tauri/src/chat_channel/traits.rs | 670b25efa150ce2c236b9be16e5dcb49a0ab230a | Backend interface |
| src-tauri/src/chat_channel/types.rs | 6c1d342b39d159f5df35d3513ff9aae9c8015c2d | Typed channel/target/receipt |
| src-tauri/src/chat_channel/scheduler.rs | 779439d929773708c9dc3ed591a40d925be61fa4 | Existing bounded recurring host seam |
| src-tauri/src/chat_channel/session_bridge.rs | 980920f906d0efb040a671a0daeb78dd67b2e7ec | ACP permission ownership audit |
| src-tauri/src/chat_channel/session_commands.rs | 58d94126ca966199260445b4f35d54eee7698844 | Existing ACP-only approval path |
| src-tauri/src/chat_channel/session_event_subscriber.rs | 6fed9ec465dffdb7fd618d0953e6ecb38595e681 | Standing ACP approval boundary audit |
| src-tauri/src/db/entities/chat_channel.rs | 1162d74ee913d04801157d212dd4d7cbe399cd1c | Existing channel config storage |
| src-tauri/src/db/service/chat_channel_service.rs | 55a364063b6ec1f3338e534a16406d4a59a643ff | Existing channel lookup |
| src/app/login/page.tsx | a02ef36fe3eeaeee1364c97ca6bff82e0e18edb2 | Existing operator login |
| LICENSE | 261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64 | Original Apache-2.0 retained |

Official Telegram API source resolved once through gh api to
`tdlib/telegram-bot-api` **e3e9dd8e5b3d7ab8537cd5a10dc31d5ffa8f82d1**,
Client.cpp blob **3beb03068fbd79deb13f94359c7192f221849fae**. Relevant API contract
reads are pending; no Telegram C++ code port is planned and Codeg is not repinned.
NOTICE mapping will be appended with implementation. No AGPL/enterprise source.

## Planned validation and progress

Meaningful cases: wrong private recipient/channel/topic/sender; no actor spoofing;
stale/denied/canceled/edited/run-changed snapshots; account scope; concurrent claims
and restart dedupe; ambiguous outcome/no retry; sanitized notifications/errors;
configured-disabled/missing-key states; and ACP standing approval cannot resolve
Ops. Actual CLI mobile link → login → full payload review, desktop/mobile light/
dark, and BC-14 database-only receipt completion. No real provider or paid inference.

Required gates: own-target locked desktop/server checks, both Clippy gates,
focused backend/frontend tests, typecheck and isolated production build. Root
owns final Design Studio acceptance, inherited shell contrast/unnamed controls
and final integration. No Step 3 implementation tests have run at this checkpoint.

Commands so far: clean status, fetch, branch creation, planning/source reads and
immutable gh api lookups exited 0; code-context exits are above. Two exploratory
searches used absent auth filenames/globs, then actual web-auth.ts was discovered.
No source writes, live configuration reads, sends or polling have occurred.
Checkpoint commit and draft PR URL follow when pushed.
