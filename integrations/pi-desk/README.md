# Pi Desk extension

This isolated Node extension is embedded by `src-tauri/src/acp/pi_desk.rs`.
Each Pi launch extracts its own assets and loads them explicitly through
pi-acp's executable override. Project/global extension discovery is disabled.
The installed peers must be Pi **0.85.1**, pi-mcp-adapter **2.32.1**, and Pi's
TypeBox **1.3.7**. Missing packages or a missing configured **gpt-6-astra** model
with **max** reasoning produce a setup error. No model fallback, dependency
installation or login is performed by the launcher.

The six native tools read public ticket data, save a revisioned reply draft,
and propose an exact reply or human-prepared issue revision. The backend derives task/run/account/agent
identity from the launch token. No tool approves, denies or executes an action.
Socket health only proves the bridge is running; every Ops call independently
checks the live task/run. This permits ordinary coding and pre-run compaction
without granting those sessions ticket access. This is not an OS sandbox.

The adapter broker synchronously claims calls and permits only three Hafidh
read tools, once per checked call. The trusted launcher configures a fixed
`codeg-mcp --features intake` process, never a caller-selected MCP server:

- `hafidh_intake_status({})`: cache count and operator import guidance.
- `hafidh_feedback_list({page?: 0..10000})`: ten public cached items per page.
- `hafidh_feedback_get({ulid})`: public cached detail and existing draft metadata.

All inputs are closed. Account and unique enabled product come from the live
task's folder. Missing/ambiguous binding fails explicitly. Results label cache
absence, invalidation and expiry; no read fetches upstream or mints freshness.
Human import/refresh, evidence attachment and severity confirmation belong to
Bug intake. Native `desk_propose_issue({draftId, expectedRevision})` accepts a
positive revision and reruns accepted host validation. It returns public pending
proposal metadata, never proofs, private reporter fields or provider config.
No tool approves, files or creates evidence. Trusted intake credentials belong
only to the host process. Resend and GitHub writes remain backend-native APIs.

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
notices are in `LICENSE`, root `NOTICE`, `reports/pi-desk.md` and
`reports/pi-issues.md`.
