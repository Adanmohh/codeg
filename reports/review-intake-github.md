# Intake / GitHub acceptance review

Accepted, 2026-09-08. PR #5 reviewed at `493df48ca92699c5818fde39d5819596712b9fdb`, merged as `4988f46bf20e7c22a83ea9a231c8073fd5d0b391`. Product source is `140b66b3`; the closing commit changes only the report. The initial findings below are retained as review history and are fixed.

Both P2 findings and canonical operator-label compatibility are corrected at `140b66b3`. Root inspected live evidence validation in `resource`, missing-scope propose semantics and actor validation, with regressions for revoked/expired/rebound source, explicit read/deny, missing scope and canonical HTTP/desktop principals. Independently ran `cargo test --locked --no-default-features --bin codeg-server --lib ops_intake` against that product source using the root-owned target: exit 0, 22 passed in 1.25s. Log: `/tmp/ops-intake-independent-rust.log`.

Root reviewed the final source mapping, NOTICE and dependency delta. Only the established JWT signer and its required dependencies are added; existing versions, Resend transport, approvals core and frontend lock remain preserved. Worker evidence: desktop/server checks and Clippy, 109 server/110 desktop selected Ops tests, 18 ticket and 18 email regressions, typecheck all pass. Independent Python evidence follows. No remaining blocking module finding. Authenticated host/UI integration, live configuration and browser validation are separate remaining work, not implied by this acceptance.

## P2 — live evidence check skipped before queuing under ask

`GithubIssueAction::resource` checks only the serialized shape. The accepted gate’s ask rule can return pending before `check_permission`, so evidence revoked/expired or binding changed after prepare can enter the queue. The reviewed contract requires live evidence before proposal creation as well as dispatch. Requested live resource/evidence validation before ask short-circuit and a regression. Approval/dispatch already recheck; no external filing bypass is claimed.

## P2 — missing scope changes the default from propose to deny

`store::agent_allowed` returns false when no matching `ops_agent_scope` exists. The accepted core explicitly defaults missing scope to propose (`gating.rs`), so a newly configured repository would be denied instead of queued for human review. Requested source verification/correction preserving explicit restrictions and enabled repository/install binding, with missing-scope and explicit-denial regressions. This is not permission to change the core gate.

## Independent Python verification

Reviewed the SDK server boundary and GET-only client. Independently ran isolated pytest with bytecode and pytest cache writes disabled: 14 passed in 1.09s, including real stdio tool discovery/read and input-error privacy. Python production code matches `a4f9f316`; the only additional test-file content had SHA-256 `19fbaa337a16fde21c5f09bfcd8c25c524de1afad1db379f6f6780c949b2650f`. No live backend or credentials used. Final Rust review, source/licence verification, integrated gates and browser wiring remain.
