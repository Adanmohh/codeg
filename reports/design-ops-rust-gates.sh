#!/bin/sh
# Run from this worktree's src-tauri directory. Never uses another target.
set -eu
export CARGO_TARGET_DIR=../.build/intake-host
cargo check --locked > ../reports/design-ops-desktop-check.log 2>&1
cargo check --locked --no-default-features --bin codeg-server --bin codeg-mcp > ../reports/design-ops-server-check.log 2>&1
cargo clippy --locked --all-targets --features test-utils -- -D warnings > ../reports/design-ops-desktop-clippy.log 2>&1
cargo clippy --locked --no-default-features --bin codeg-server --bin codeg-mcp --lib -- -D warnings > ../reports/design-ops-server-clippy.log 2>&1
cargo test --locked --no-default-features --bin codeg-server --lib ops > ../reports/design-ops-rust-tests.log 2>&1
