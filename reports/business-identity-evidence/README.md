# Identity API evidence

Production source: `861fb0ef4394d4980a19ba375bb4c1f3f218d39b`. This directory contains synthetic API checks, not screenshots or acceptance of the business workspace frontend.

The actual loopback fixture is live on `127.0.0.1:4341`, owned PID29883. It has an in-memory database and allows only identity routes, harmless health and its explicit test landing. No scheduler, provider, model, agent or old export is used. Both worker browser sessions are closed and root may now mutate this fixture.

The original launch, from the approvals worktree (verified compiled artifact):

```sh
.build/business-identity-target/debug/deps/codeg_lib-b542e2757758a7bb --ignored --exact business_identity::tests::fixture::business_identity_browser_fixture --nocapture
```

To build/start a replacement after that owned listener has deliberately stopped, use the ignored test with an own target directory; it binds only4341 and refuses an occupied port:

```sh
CARGO_TARGET_DIR=/Users/mohamedadan/projects/_worktrees/ops-desk/approvals/.build/business-identity-target cargo test --locked --manifest-path src-tauri/Cargo.toml --no-default-features --lib business_identity::tests::fixture::business_identity_browser_fixture -- --ignored --exact --nocapture
```

The test-only operator constant is `business-identity-synthetic-operator`. The CLI scripts issue synthetic individual credentials internally, never return their plaintext in evidence, and use real browser fetch against the protected API. They do not intercept response JSON. Off-origin requests are aborted.

Commands run from the approvals worktree:

```sh
playwright-cli -s=identity-api-a-4341 open about:blank
playwright-cli -s=identity-api-a-4341 --raw run-code --filename=reports/business-identity-evidence/browser-api.js
playwright-cli -s=identity-api-b-4341 open about:blank
playwright-cli -s=identity-api-b-4341 --raw run-code --filename=reports/business-identity-evidence/browser-second-session.js
playwright-cli -s=identity-api-a-4341 close
playwright-cli -s=identity-api-b-4341 close
```

`browser-a.json`:21 assertions passed. `browser-b.json`:5 assertions passed. Both were parsed and cross-checked: same organization, different member IDs. All passing commands and closes exited0. `browser-a-attempt1.txt` preserves the initial run-code URL-global failure (exit1); it occurred before database/API mutations and was fixed only in test glue. `hooks.json` contains sanitized live records for this worker/session only. No production credentials, provider secrets, broad Settings snapshots or local browser storage are present.
