# Pi Desk extension

This isolated Node extension is embedded by `src-tauri/src/acp/pi_desk.rs`.
Each Pi launch extracts its own assets and loads them explicitly through
pi-acp's executable override. Project/global extension discovery is disabled.
The installed peers must be Pi **0.85.1**, pi-mcp-adapter **2.32.1**, and Pi's
TypeBox **1.3.7**. Missing packages or a missing configured **gpt-6-astra** model
with **max** reasoning produce a setup error. No model fallback, dependency
installation or login is performed by the launcher.

The five native tools read public ticket data, save a revisioned reply draft,
and propose that exact revision. The backend derives task/run/account/agent
identity from the launch token. No tool approves, denies or executes an action.
Socket health only proves the bridge is running; every Ops call independently
checks the live task/run. This permits ordinary coding and pre-run compaction
without granting those sessions ticket access. This is not an OS sandbox.

The adapter broker synchronously claims calls and permits only three Hafidh
read tools, once per checked call. Trusted intake configuration/credentials
belong to the host process. Resend and GitHub writes are backend-native APIs.

The web app's TypeScript project excludes this Node package. With the pinned
peer packages available to its module resolver, run from the repository root:

```sh
pnpm exec tsc --noEmit --project integrations/pi-desk/tsconfig.json
pnpm exec vitest run --config integrations/pi-desk/vitest.config.ts
```

The process tests require installed pinned Pi/adapter clients and the own
worktree's `src-tauri/target/debug/codeg-mcp` (`cargo build --locked
--no-default-features --bin codeg-mcp` in `src-tauri`). The Rust extracted-asset
fixture is opt-in because it needs those clients:

```sh
cargo test --locked --no-default-features --lib pi_desk_extracted_assets_real_rpc_discovery -- --ignored
```

It uses isolated synthetic catalogue metadata and no model request. Root app
tests do not require global Pi peers. Exact upstream mappings and complete MIT
notices are in `LICENSE`, root `NOTICE` and `reports/pi-desk.md`.
