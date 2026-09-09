# Unified synthetic intake fixture

Tickets owns the test process, database and synthetic upstream. This fixture
validates the accepted business identity/settings/tasks and intake core together;
it does not start the engineering engine, a native window, an agent, or a real
provider. Rebrand owns the separate UI assets and browser sessions.

The production baseline is `e55f3bfd1f069d6d6111370223993596b19ecb9b`. That
checkpoint has a single green38-case intake run and default/server/companion,
Clippy and typecheck gates. This follow-on adds only an ignored manual fixture,
its test registration, notices and reports. Runtime handoff metadata will be
recorded after source commit and launch; no listener is claimed by this recipe.

## Boundary and record ownership

- Backend is fixed to `127.0.0.1:4351`; synthetic GraphQL is fixed to4352. Both
  sockets must bind before creating any data. Occupied addresses fail; existing
  fixtures are never stopped or reseeded.
- Browser requests accept only the new frontend4354 origins (`127.0.0.1` or
  `localhost`) and the fixture's own127.0.0.1:4351 origin. Requests without Origin
  remain available to scoped command-line API probes. Old4350/4353 fixtures are
  not connected or modified. Normal production API CORS handles preflight after
  the outer fixture origin/path/method checks.
- One new file database under the tickets worktree's ignored
  `.docs/business-intake-fixtures/unified-*` runs the real combined migrations.
  Separate `ui`, `root`, `review`, `worker` tenants are provisioned through the
  actual protected HTTP router. Each namespace has owner, feedback-manager and
  feedback-viewer credentials plus one human-prepared task suitable for linking.
  Use only your namespace for mutations; foreign IDs may be used for denial probes.
- No initial source bindings or source grants are seeded. The actual database and
  synthetic-reader counters assert both counts and provider reads are zero before
  announcing readiness. Setup, explicit grants, enablement, import, passage review
  and task publication are real subsequent API operations.
- Each namespace has a separate0600 credential file with generated member tokens,
  its synthetic-only Fireflies key and its synthetic control token. Files are not
  committed, printed or exposed by HTTP. `owner-controls.json` is for the fixture
  owner/root only; it contains the protected synthetic platform and stop tokens.
  The injected memory store refuses all provider values except the four explicit
  synthetic keys. It has no existing/native store fallback.
- The exact path/method allowlist admits business identity/settings, human task
  operations and25 intake operations, plus protected tenant platform operations.
  It denies original bootstrap, task execution linking, legacy host settings,
  agent/terminal/provider routes and static/file serving. The regular Principal,
  platform bearer, revision and source-grant checks remain inside the allowlist.
  This outer guard is test scaffolding, not production tenant-isolation evidence.

The only upstream queries are the three exact production reader strings, with
closed synthetic variable shapes and recognized synthetic credentials. List rows
are `meeting-follow-up`, `meeting-empty`, `meeting-long`. Long/empty source states
are intentional; the empty source does not become a fabricated task. Each tenant
has distinct source text even when external provider IDs match.

## Safe controls

`GET /__business_intake_fixture/health` returns safe process/port/scenario/counter
metadata only. It includes no credential, private transcript or source proof.

`POST /__business_intake_fixture/control`, bearer from your namespace file:

```json
{"namespace":"review","scenario":"changed"}
```

Allowed scenarios are `fresh`, `changed`, `denied`, `missing`. A control changes
only future synthetic detail responses for that namespace; it never mutates the
database, source revision, source freshness, tenant epoch or a candidate. An
explicit authorized import/refresh must observe the changed response.

Suspension/resumption and credential revocation use the real protected platform
or member endpoints and their current CAS fields. Coordinate these actions within
your namespace. Root may receive the owner platform file read-only for its own
tenant lifecycle probes; it must not modify another reviewer's tenant.

Only the fixture owner may call `POST /__business_intake_fixture/stop` using the
separate stop token, after all consumers release the listener. Both servers stop
gracefully; database, credential files, metadata and final counters are retained.
Provider credentials are intentionally memory-only in this synthetic harness;
its shutdown/restart is not a production credential-store persistence test.

## Build and evidence recipe

Use the existing isolated `.docs/business-intake-tenancy-target` with locked
offline server-mode library compilation. Build the ignored
`business_intake::tests::browser::intake_browser_fixture` without running it.
After committing source, preserve one compiled test executable in a new owned
fixture artifact directory and verify its digest against the original. This is
one executable copy, not a target copy or release/native build.

Recheck4351/4352, announce the stable launch, then invoke only that test with
`--exact --ignored --nocapture --test-threads=1`. Record the absolute executable,
source commit and source hash, binary SHA256, PID, safe health response and
metadata path. The listener remains stable until explicit coordinated release;
later compilation uses the existing target without replacing the running copy.

Actual UI claims require the UI owner's immutable asset identity and a new owned
Playwright CLI session pointed at this one backend. Backend HTTP probe results do
not certify the product UI or native restricted-window isolation. No full B/BI
acceptance is claimed by this recipe alone.

## Exact source/provenance

Apache-2.0 Codeg at `949adb02c18c72d0804eeb74b420520a7681ee86`:

| Source | Git blob | Reuse |
| --- | --- | --- |
| `src-tauri/src/business_tasks/tests/fixture.rs` | `1c2f30c657fb339be53392cda21e508bd866929e` | Protected router/guarded no-engine manual fixture |
| `src-tauri/src/business_intake/tests/support.rs` | `57db886feacfc201f2e38da002ef18832ea073a2` | Injected loopback reader and memory store |
| `src-tauri/src/business_intake/tests/tenant_http_cases.rs` | `9973b928ec7e89239313eb560e94a5ddc03e35a4` | Real protected provisioning and human task HTTP flow |
| `src-tauri/src/keyring_store.rs` | `2973118cb2a47eabfca9fb0b3413fd534e91aba8` | Private create-new0600/write-all/sync file pattern |
| `src-tauri/src/business_intake/fireflies.rs` | `8c2dfc4845dc4e504e833a4e2af8c057a5e78102` | Exact fixed reader query strings; upstream MIT notice preserved |

Original MIT Fireflies mapping remains
`firefliesai/n8n-nodes-fireflies@fbd24607bc784a2294ce402426aefe2cb8c00f50`,
Copyright2022 n8n, exact files/full license in NOTICE. Installed Axum0.8.8,
Tokio1.49.0, tokio-util0.7.18, tempfile3.24.0, SeaORM1.1.19 and axum-test17.3.0
sources are API references for serving, cancellation, retained temporary paths
and synthetic request dispatch. No package, runtime, AGPL/GPL source or credential
client is installed or ported.
