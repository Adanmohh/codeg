# Email transport acceptance review

Accepted after correction, 2026-09-07. Initial reviewed commit `2555fff2`; final reviewed head `80b0cb0e6fe26695801b1d661054f1c8155cd99c` (product fix `998a733023ac29739cbde40f713128a15dd52298`).

## P2 — Reply-To discarded during ticket normalization

`src-tauri/src/email_transport/normalize.rs`, `ReceivedEmail::into_ticket`, selects `sender(&self.envelope.from)` and does not use the received Reply-To values. A message from `noreply@service.test` with `Reply-To: reporter@example.test` therefore creates the noreply contact. Since the ticket contact supplies the later default reply target, the information needed to reach the reporter is lost.

Independently verified through gh api at Chatwoot v4.17.1 SHA `b354a9550e1fb59fa537a9c384232cb076213e72`: `app/presenters/mail_presenter.rb`, `from` prefers reply_to over from and `sender_name` follows the same header. Requested owner-worker correction with distinct sender/reply address, empty fallback, encoded name and malformed/ambiguous cases. Keep inbox recipient scoping unchanged. This finding is source-based; the orchestrator has not claimed a failing executable reproduction.

## Checks performed so far

Read the committed transport types, HTTP client, bounded pull ordering and complete normalization module. Independently read official Resend receiving methods and response types through gh api at `resend-node@v6.26.0` SHA `c61cccae2999d50d2aca9ce5fd1064f3bb855219`; list/detail query and nullable response contracts align. Worker reports first 15 transport tests passing before this newly identified case. Final report, fixes, integrated runtime gates and independent test rerun remain before acceptance.

## Final verification

The worker restored Reply-To precedence and header display names, rejecting malformed/multiple/conflicting targets instead of silently falling back. Three additional tests cover decoded names, absent/empty fallback, ambiguity, actual persisted contact and unchanged inbox scoping. Orchestrator inspected the full fix and tests, confirmed the final commit changes only the report, and independently ran all 18 transport tests with server features and the root-owned target directory: exit 0, 18 passed. Worker final report records both runtime compile/test/Clippy gates and typecheck passing. No blocking finding remains for this bounded internal transport.

PR #6 merged as `2fecb1cfdce57cba451520ce192bc404fe081051`. Approval dispatch, durable delivery attempts, credentials/settings, scheduling and UI are subsequent integration work; this merge does not claim a live email send or browser end-to-end reply.
