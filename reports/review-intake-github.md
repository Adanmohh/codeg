# Intake / GitHub acceptance review

In progress, 2026-09-08. PR #5 is not accepted. Python production code reviewed at `a4f9f316`; early Rust observations below refer to the worker’s in-progress module (`mod.rs` SHA-256 `b1252e1c6359e4f5ee538c398e0a01835386cffaa884d6475095ad42ffa8e8da`, `store.rs` SHA-256 `e1efbd531825abbe6dce350a3d906a2d29803f67f1fd1128aac19376598fd029`), not a final immutable Rust delivery.

## P2 — live evidence check skipped before queuing under ask

`GithubIssueAction::resource` checks only the serialized shape. The accepted gate’s ask rule can return pending before `check_permission`, so evidence revoked/expired or binding changed after prepare can enter the queue. The reviewed contract requires live evidence before proposal creation as well as dispatch. Requested live resource/evidence validation before ask short-circuit and a regression. Approval/dispatch already recheck; no external filing bypass is claimed.

## P2 — missing scope changes the default from propose to deny

`store::agent_allowed` returns false when no matching `ops_agent_scope` exists. The accepted core explicitly defaults missing scope to propose (`gating.rs`), so a newly configured repository would be denied instead of queued for human review. Requested source verification/correction preserving explicit restrictions and enabled repository/install binding, with missing-scope and explicit-denial regressions. This is not permission to change the core gate.

## Independent Python verification

Reviewed the SDK server boundary and GET-only client. Independently ran isolated pytest with bytecode and pytest cache writes disabled: 14 passed in 1.09s, including real stdio tool discovery/read and input-error privacy. Python production code matches `a4f9f316`; the only additional test-file content had SHA-256 `19fbaa337a16fde21c5f09bfcd8c25c524de1afad1db379f6f6780c949b2650f`. No live backend or credentials used. Final Rust review, source/licence verification, integrated gates and browser wiring remain.
