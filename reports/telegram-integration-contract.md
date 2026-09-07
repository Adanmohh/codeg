# Telegram integration contract — source review

Planning evidence only, 2026-09-07. No bot was connected, polled or messaged.

Read inherited `src-tauri/src/chat_channel/session_bridge.rs`, permission handling in `session_commands.rs`, `session_event_subscriber.rs`, and the existing `backends/telegram.rs` client. These remain Codeg v0.30.4 patterns, not new Ops functionality. Future ports must verify the immutable upstream files through gh api and retain NOTICE attribution.

The existing Telegram backend uses the shared channel manager, Reqwest and a configured bot token/chat target. Keep it; no new chat backend or webhook plumbing is needed. Native topic targets and canonical numeric chat IDs are already handled.

The existing `PendingPermission` stores one ACP request per active session. `/approve` and `/deny` resolve that ACP request, not an Ops proposal. The current `approve always` setting can automatically allow subsequent ACP requests. Therefore an Ops destructive approval MUST NOT be represented as an ordinary ACP permission and assumed to enforce the human floor. The Ops registry/gate remains authoritative, independently of channel auto-approval.

For Step 3, route typed Ops queue notifications/review links through the existing channel backend, with explicit proposal/task/run/snapshot binding and private recipient authorization. A link contains no operator token, private email/log text or execution capability. Mobile UI performs the complete payload review. If direct phone commands are added, they need an exact proposal/snapshot and an explicitly authorized sender; a generic latest-request approve is insufficient, and standing ACP approval must never authorize it. Denied/stale/canceled/edited proposals and unrelated chat/topic/sender callbacks require regression tests.

Test through an injected local channel backend or loopback HTTP fixture, using Playwright CLI for the linked mobile review. No real Telegram message or polling of a live channel is part of a local test. Existing bot configuration availability has not been inspected; lack of configuration is a visible setup state. Live channel activation and actual notifications remain unperformed.
