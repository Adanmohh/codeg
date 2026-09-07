# Step 2 intake / GitHub implementation

Status: independent implementation complete; required local gates passed.
Awaiting root acceptance. PR #5 stays draft; host/UI integration awaits a
separate assignment after merge. Source remains at the reviewed `140b66b3`.
Branch `feat/step2-intake-github`, created after
a clean check and fetch from `dc7ce45d97ce5df3ad33a40fb80923c6467406b9`.
Accepted contract: `reports/step2-intake-contracts.md` (PR #4, merged bb9134d3).
No other worktree, Hafidh source/environment, protected planning document or
core approval-engine edits are authorized or needed.

## Module contract for the Ops UI owner

Additive Rust module: `codeg_lib::ops_intake`, with `github` submodule. This
worker owns these modules, their tests and `m20260907_000005_ops_intake` only;
minimal lib/migration exports are the shared-file seams. UI owns the registry,
authenticated command/HTTP/review adapters and all frontend changes.

Implemented public interface (`src-tauri/src/ops_intake/mod.rs` re-exports types):

- `IssueDraftV1` and `PreparedIssue`: strict serializable draft and complete
  frozen preview/payload; `prepare(conn, draft)` validates the trusted local
  source/evidence/repository bindings and renders exact title/body/labels.
- `GithubIssueAction`: implements accepted `ops_approvals::Action`, name
  `github.create_issue`, domain `github`, resource configured `owner/repo`,
  always destructive. Use existing `propose` / `approve` / `deny`; do not create
  another approval protocol. Pass the entire prepared payload through review.
- `dispatch(conn, client, AuthorizedAction)`: consumes the existing one-use
  committed handoff, checks current bindings/evidence/task generation again,
  durably reserves a filing attempt, sends the exact approved payload once,
  then records created/failed/unknown. Status lookup never sends.
- Trusted host-only source/evidence import and repository-binding functions
  populate local SQLite state. Do not register these as agent MCP tools or
  allow agents to assert human confirmation, freshness, actor or enabled scope.
  Local sanitized artifact bytes are attached by ID; no arbitrary path/URL
  resolver. The UI must derive its human actor from authentication.
- `github::GithubAppClient`: runtime App configuration/private key supplied
  only by trusted host code. Missing config is `not_configured`. No PAT or gh
  auth fallback. Tests inject synthetic local HTTP/RSA fixtures only.

Functions exercised by the compiled integration tests (all fallible results use
`IntakeError`; this table is an API reference, not a runnable Rust example):

| Function | Inputs | Successful result / effect |
| --- | --- | --- |
| `prepare` (async) | database connection, complete `IssueDraftV1` | `PreparedIssue`, local validation only |
| `PreparedIssue::payload` | borrowed prepared issue | complete `serde_json::Value` |
| `PreparedIssue::payload_digest` | borrowed prepared issue | canonical SHA-256 string |
| `dispatch` (async) | connection, borrowed App client, owned `AuthorizedAction` | `FilingReceipt`, consumes capability |
| `filing_status` (async) | connection, borrowed `SourceRef`, repository ID (`i64`) | optional `FilingReceipt`, no network |
| `reconcile_filing` (async) | connection, borrowed App client, attempt ID (`i64`) | `FilingReceipt`, issue GETs and installation-token exchange if needed; no issue write |
| `GithubAppClient::new` | optional `GithubAppConfig` | `GithubAppClient` |
| `GithubAppClient::is_configured` | borrowed App client | `bool`, no network |

`GithubIssueAction` is a unit struct implementing the **existing** `Action`.
For propose, pass `prepared.payload()` to existing `ops_approvals::propose`
with the same trusted task/run IDs stored in the draft. Human edits must call
`prepare` again with the complete edited draft before reviewing the new render;
then existing `approve(Review { expected_payload, approved_payload })` yields
the one-use handoff passed directly to `dispatch`. No network occurs in Action
callbacks. Dispatch also requires the persisted proposal task/run to match the
payload. Do not expose approval, repository configuration, source import or
evidence attachment as agent tools.

`IssueDraftV1` JSON fields: `schema_version:1`,
`template_version:"hafidh-issue-v1"`, `task_id`, `run_seq`,
`source_ref:{product_id,source:"testflight"|"in_app",ulid}`, `source_revision`,
`title`, `summary`, `labels:string[]`, and `evidence:{build,screen,reciter,log}`.
Each evidence reference is `{artifact_id,sha256,value}` minted by trusted
attachment glue. Convert the Python source ref to this subset (its optional
`external_id` is upstream provenance, not the local identity). Unknown fields
are rejected at every draft/prepared boundary. All four proofs are mandatory.
Labels are explicit, human-reviewed, existing repo label names; this module
does not create labels. Source triage remains a suggestion until that review.
V1 accepts at most 20 unique label names of at most 50 bytes, using ASCII
letters/digits or `_:-`. Other label spellings are `invalid_payload`. Titles
are a single line of at most 200 characters/800 bytes; summaries and proof
bytes are at most 8192 bytes each, and the rendered body is at most 16 KiB.

`PreparedIssue` freezes `{draft,repository_id,repository,binding_digest,outgoing}`;
`outgoing` is the exact `{title,body,labels}` posted. Receipt fields are
`attempt_id,proposal_id,state:unknown|failed|created,issue?,error_code?,retry_after?`;
issue fields are `id,number,html_url,labels,labels_match`. No execution capability
or installation token is serialized. Missing App config is `not_configured`;
adapters should also check `is_configured` before showing a filing affordance.

Host-only setup functions are `configure_repository(conn,&RepositoryBinding)`,
`record_source(conn,&SourceRef,revision,fetched_at_epoch)`,
`attach_evidence(conn,EvidenceAttachment,human_actor)` and
`revoke_evidence(conn,artifact_id)` (all async). `RepositoryBinding` contains
product_id,folder_id,app_id,installation_id,repository_id,full_name,enabled.
Existing `ops_agent_scope` for the agent/domain/repo (or domain default) uses
the accepted gate semantics: missing scope defaults to `propose`; `propose`
and `act_low_risk` both require human review for this destructive action.
Explicit `read` (the stored spelling of read-only scope) and deny rules remain
restricted. Repository installation/enabled/product/folder bindings are still
mandatory; a missing policy row does not invent missing repository config.
Host calls `record_source` only after authorized successful GET revalidation,
never from an agent-asserted timestamp/revision. Local source freshness is
bounded to 15 minutes; re-read via intake before human review when stale.

`EvidenceAttachment` deliberately is not Deserialize. It takes the source/ref
revision, field, value, reviewed sanitized UTF-8 `content`, optional capture
time/session ULID/expiry and `EvidenceProvenance` (AscBuild, HumanReport,
Recorder, LocalDiagnostic, SessionDiagnostic). The human actor is backend derived:
canonical UI principals `operator:http` and `operator:desktop` are accepted
unchanged. Validation requires trimmed nonempty text of at most 128 bytes with
no control characters; it does not require an identity alias. Actor is a
separate host-only function argument, never a JSON field or agent assertion.
SessionDiagnostic requires the explicit session ULID; screenshot,
URI, empty log, guessed reciter and marketing-version-only human build fail.
Data is stored by generated artifact ID; no arbitrary path/URL is read.
`GithubAppConfig` contains app_id/private_key_pem and deliberately has no
Debug/Serialize. Supply it through the trusted host credential adapter; no
keyring, PAT, environment auto-discovery or gh fallback is introduced here.

Python package: `integrations/hafidh-intake`, isolated `.venv`. Exactly three
stdio MCP tools: `hafidh_feedback_list`, `hafidh_feedback_get`,
`hafidh_intake_status`; no write/approve/send tools. Existing TestFlight GET
list and sync/status only. GET-by-ID revalidates through a bounded list scan;
in-app is explicitly unavailable because Hafidh has no read endpoint.

## Grounding and progress

Read complete current FOUNDING, ORCHESTRATOR, STATUS, DECISIONS, AGENTS and the
merged contract. Read the accepted approval implementation including its
non-Clone/non-Serialize `AuthorizedAction`, exact `Review` snapshot and
post-approval waiting-owner reconciliation; no engine changes planned.

Applied code-context using the existing RAG Python and `HF_HUB_OFFLINE=1`:
guide exit 0 (installed-version grounding and sibling-source reuse apply).
The owner's local-first / immutable `gh api` / no-worker instructions override
generic corpus suggestions for context7 or delegation. Docs exit 3: corpus
`data/code/rebrand.db` is absent; actual installed source is the fallback, not
an invented corpus hit. Read React 19.2.4 metadata, Cargo.toml, current SeaORM
approval/CAS and keyring seams. No corpus/global environment changes.

Live hook evidence in `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`:
session `01a07c1c-cf2e-73e1-bbe3-e758c8363042`, this worktree, PreToolUse line
1584 and PostToolUse line 1585, timestamp 1788813122. Separate React/Cargo
reads on continuation also produced live PreToolUse line 2640 and PostToolUse
line 2633 for this same session/worktree. Final audit inspection found
PreToolUse line 2929 and PostToolUse line 2908 with the same session/worktree.
Hooks remained enabled through implementation, validation and report edits.

## Exact source mapping (implementation ledger)

The accepted contract contains the full immutable research ledger. Port inputs:

| Source / immutable revision | Exact files | Destination / adaptation |
| --- | --- | --- |
| codeg v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed`, Apache-2.0 | `src-tauri/src/db/service/work_task_service.rs`, `src-tauri/src/models/work_task.rs`, `src-tauri/src/db/migration/m20260801_000003_work_task_template.rs` | Snapshot/render and SQLite transaction/CAS patterns in `src-tauri/src/ops_intake/{types,store}.rs` and `src-tauri/src/db/migration/m20260907_000005_ops_intake.rs`; evidence validation is boundary glue. |
| intromail `0bd24dfe284b888aa9f602fa1fd00e337ea38874`, owner-authorized source without repository license | `backend/app/services/github/client.py`, `backend/app/services/github/actions.py`, `backend/tests/test_github_pack.py` | `src-tauri/src/ops_intake/{github,mod}.rs`, `tests.rs`, `github/tests.rs`: App JWT/install-token cache and destructive action patterns. No create-issue helper exists; narrow REST call is glue. |
| python-sdk v2.0.1 `8b191a433634d64b1306d7d51be8b16e14cc0893`, MIT | `src/mcp/server/mcpserver/server.py`, `examples/mcpserver/readme-quickstart.py`, `examples/mcpserver/weather_structured.py`; installed client API `src/mcp/client/{stdio,client}.py` | `integrations/hafidh-intake/src/hafidh_intake/server.py` and `integrations/hafidh-intake/tests/test_stdio.py`: `MCPServer` tools, strict structured results, sanitized public `call_tool` boundary and real stdio discovery/read. v2 FastMCP path intentionally unavailable. |
| Hafidh `a83794708193a18611c8e6eb8e88637b8136e471`, owner's read-only source | `backend/app/modules/testflight/{models.py,schemas.py,admin_router.py,triage.py}`, `backend/app/modules/feedback/{models.py,feedback_schemas.py,feedback_router.py}`, `backend/app/common/utils/operation_result.py` | `integrations/hafidh-intake/src/hafidh_intake/{schemas,client,triage}.py`: read-only TestFlight adapter, strict DTO projection and exact classifier port (only import/header adapted). No invented in-app read API. |
| Accepted approvals merge `65aca88916b4af398a568c09ece431440d660e3b`, retained in integrated main | `src-tauri/src/db/service/ops_approvals/{mod,gating,redaction,tests}.rs` | Existing Action/Review/AuthorizedAction used by `src-tauri/src/ops_intake/mod.rs`; live source/resource reads use its transaction. Core is unchanged. |
| Keats/jsonwebtoken v9.3.1 `87bbe49004de17ac1c64bf25d7751c0e43cff5dc`, MIT | `README.md`, `Cargo.toml`, `src/encoding.rs`; installed header/validation APIs and LICENSE | Exact `jsonwebtoken = "=9.3.1"` dependency used by `src-tauri/src/ops_intake/github.rs`; established RS256 signing, no handwritten crypto. |

Official HTTP contract evidence, fetched through `gh api` at immutable refs:

- `github/docs@831337b0fed60b90a72e2711a41dfcad72b5f288`:
  `content/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-json-web-token-jwt-for-a-github-app.md`,
  `data/reusables/apps/generate-installation-access-token.md`,
  `src/github-apps/data/fpt-2022-11-28/server-to-server-permissions.json`,
  `content/rest/using-the-rest-api/best-practices-for-using-the-rest-api.md`.
- `github/rest-api-description@3cef12e8a02d612ad032473d4fb87266f2befeae`:
  `descriptions/api.github.com/api.github.com.2022-11-28.json`, operations
  `apps/create-installation-access-token`, `issues/create`, `issues/list-for-repo`,
  `issues/list-labels-for-repo` and referenced schemas. The adapter retains
  intromail's API version `2022-11-28`.

NOTICE appends the owner-authorized port ledger and the original SDK and JWT
MIT notices/license texts. The Apache LICENSE and all accepted-main NOTICE
entries remain intact. No AGPL, restricted enterprise or PolyForm source used.

## Transport and durability

The fixed GitHub API origin, 30-second timeout, `reqwest::retry::never()`,
disabled redirects/proxies and bounded 2 MiB JSON reader apply to this client.
Reqwest 0.12.28 installed source was read: its default protocol retries are
explicitly disabled. Only App configuration can authenticate runtime filing.
RS256 uses `iat=now-60`, `exp=now+540`, App ID issuer and the established signer.
Installation tokens request exactly one configured repository and
`issues:write` / `metadata:read`; returned repository, permissions and expiry
must match. The in-memory single-flight cache has at most 16 entries, keyed
by App/key generation, installation and repository; expiry refresh is 60 seconds
early. Replacing the immutable client rotates the key without reusing old tokens.

Existing labels are checked through bounded GETs before reservation. After
preflight, a SQLite write transaction revalidates source/evidence/repository,
task/run, persisted approval, current scope and deny rules. It commits an
`unknown` receipt and audit before the one issue POST. No network occurs inside
that transaction. Unique source/repository active filing and proposal IDs stop
concurrent duplicate sends. Exact approved JSON and its digest remain in the
local receipt for recovery; no execution capability is persisted/reconstructed.

A valid 201 records actual issue ID/number/URL/labels. Missing requested labels
is a created issue with `labels_match=false`, never a resend or label mutation.
Known 400/401/403/404/410/422/429 non-creation failures retain failed attempts;
retry requires a new human-approved proposal and observes persisted rate
deadlines. Timeout, response loss, redirects, 5xx, malformed success or a crash
after reservation stay unknown. Failed receipt persistence also leaves the
durable unknown reservation. Reconciliation lists closed/open issues, skips PRs
and requires one exact title/body/marker match; it never retries an issue POST.
It uses up to ten trusted numbered GET pages of 100, not arbitrary Link URLs.
No match, multiple matches or an incomplete bounded scan remains unknown.
This is local duplicate prevention, not cross-machine exactly-once delivery.

## Validation

Rust commands ran in this worktree's `src-tauri/` with `CARGO_TARGET_DIR` set
to its own absolute `src-tauri/target`. TypeScript ran at the worktree root.
The package's Python 3.13.14 `.venv` is isolated; no Hafidh/RAG env was modified.
The Python command below is relative to `integrations/hafidh-intake/`.
All runtime HTTP/RSA/SQLite fixtures are synthetic. The committed RSA keys in
`src-tauri/src/ops_intake/fixtures/` are generated test fixtures, not App keys.

| Command / check | Result |
| --- | --- |
| Isolated `.venv/bin/python -m pytest -q` intake suite | Worker 14/14 passed, exit 0; owner independently confirmed 14/14 with bytecode/cache disabled. Includes actual SDK stdio discovery/read against loopback HTTP. |
| `cargo check --locked` | Exit 0, default desktop. |
| `cargo check --locked --no-default-features --bin codeg-server` | Exit 0, final integrated server check. |
| `cargo test --locked --no-default-features --bin codeg-server --lib ops_` | Exit 0, 109 passed; includes 22 intake, 21 core approvals and selected engine/parser regressions. |
| `cargo test --locked --features test-utils --lib ops_` | Exit 0, 110 passed in desktop mode. |
| `cargo clippy --locked --all-targets --features test-utils -- -D warnings` | Exit 0, default desktop including test targets. |
| `cargo clippy --locked --no-default-features --bin codeg-server --lib -- -D warnings` | Exit 0, final integrated server run. |
| `cargo test --locked --no-default-features --bin codeg-server --lib ticket_service` | Exit 0, 18 passed; integrated migration/threading regressions. |
| `cargo test --locked --no-default-features --bin codeg-server --lib email_transport` | Exit 0, 18 passed; accepted Resend integration regressions. |
| `pnpm exec tsc --noEmit` | Exit 0 before and after accepted-main integration. |
| `git diff --check` | Exit 0. |
| Protected docs, `pnpm-lock.yaml`, approvals core and email transport diff against integrated `ebee0cc8` | Exit 0, unchanged. Accepted-main NOTICE remains an exact prefix of this branch's NOTICE. |

Meaningful failures before fixes: initial Python collection exit 2 (module not
yet present); real stdio validation privacy test exit 1 (SDK echoed invalid
input); fixed suite progressed 11/11 to 14/14. Installed v2's public `call_tool`
boundary now returns a fixed sanitized error. Rust review regressions failed
first with exit 101 for ask-rule stale evidence, missing-scope default and
canonical actor labels, then passed on the final implementation. A guessed
SDK `client/transports/stdio.py` lookup was corrected by discovering and reading
the actual installed `client/stdio.py`; no nonexistent API was implemented.

Coverage includes mandatory proof omissions/placeholders, byte tampering,
expiry/revocation/revision/product changes, explicit session/build provenance,
edited approved payload equality, stale review/run, read scope and deny rules,
destructive floor, ask-rule pre-proposal checks, canonical backend actors,
real RS256 verification, token scope/expiry/cache isolation, auth failures,
label preflight/mismatch, rate deadlines across client instances, concurrent
filing, disk reopen, response-lost reconciliation and migration round trip.
Python covers strict projections, auth errors, privacy, bounded/malformed
responses, origin restrictions, filtered counts, duplicate rows, cursor binding,
failed revalidation and unavailable in-app. No full repository test-suite or
browser/runtime deployment result is claimed.

Python production bytes still match `a4f9f316`; no Python rerun was made merely
for docs/main integration. `tests/test_intake.py` SHA-256 is
`19fbaa337a16fde21c5f09bfcd8c25c524de1afad1db379f6f6780c949b2650f`, matching the
owner's review. Exact direct pins are MCP 2.0.1, HTTPX 0.28.1 and Pydantic 2.12.5;
all 37 resolved dependency/test pins are in `integrations/hafidh-intake/requirements.lock`.
The justified Cargo change adds jsonwebtoken 9.3.1, pem 3.0.6 and simple_asn1
0.6.4 plus existing getrandom JS feature edges; no existing package version was
upgraded. Accepted mail-parser 0.11.1 and hashify 0.2.9 remain present.

Build output retains the inherited proc-macro-error2 2.0.1 future-compatibility
warning and missing codeg-mcp sidecar placeholder warning. Checks/tests compile
the app; no distributable bundle, working sidecar or running desktop is claimed.

## Real gaps and integration limits

- Trusted host wiring remains with the Ops UI owner: registry, authenticated
  actor derivation, product/folder access checks, credential storage/launcher
  and review routing. This module does not authenticate a caller by accepting
  an actor string over JSON. Native/server adapters must keep setup, evidence
  attachment, approval and dispatch out of agent-facing MCP tools.
- Live App ID/key, installation, enabled repository ID/name, label inventory
  and Hafidh read credential/origin are not configured or verified by this
  worker. Missing runtime config returns `not_configured`; no App install,
  keyring/PAT overwrite, gh runtime fallback or live login was attempted.
  Python trusted launcher names are `HAFIDH_INTAKE_ORIGIN`,
  `HAFIDH_INTAKE_PRODUCT_ID`, `HAFIDH_INTAKE_BEARER`. The last is an upstream
  admin bearer: the adapter's GET-only API does not reduce that credential's
  backend permissions. Keep it in the trusted intake process only.
- Existing TestFlight GET list/sync-status are the only source calls. There
  is no GET-by-ULID or in-app read route. List continuations use process-local
  five-minute cursors and at most five pages; get revalidates a listed ID
  through at most 500 current records. Offset paging is not lossless; capped
  scans and failed revalidation never claim complete/fresh success.
- Source models lack report-bound screen/reciter/log proof and diagnostic
  bytes. Human-imported sanitized local proof supports this implementation;
  no guessed joins, screenshot-as-log, arbitrary URL/path fetch or Hafidh
  backend change fills those gaps. Heuristic triage remains unconfirmed.
- Text sanitization is bounded and cannot prove universal PII removal.
  Outgoing text still requires human review. The local SQLite receipt/evidence
  store retains approved sanitized content; audit/errors omit credentials and
  raw HTTP/private source data. No new UI was authored, so Playwright/Design
  Studio validation belongs to the UI integration task.

## Commits and delivery

Initial interface `39d00445`, isolated MCP `a4f9f316`, signer pin `606c5266`,
Rust evidence/filing `51509f9f`, and recovery/tests `3a04f14d` were staged in
small commits. After the stable checkpoint, the owner authorized merging
accepted main `ebee0cc8b67b51192db2d6a6f3fd506f1c6e2510` (including Resend
`2fecb1cf`) in `516e6da4`. Only NOTICE conflicted: retained its complete main
contents, then appended this worker's entries. No protected document or other
worker's product source was edited to resolve integration.

Implementation commit: `140b66b3e1bfd2d429da37dcf7c134df86e0eed9`, pushed.
This includes the owner's live resource/missing-scope fixes and canonical
`operator:http` / `operator:desktop` support. The closing report is a later
documentation-only commit; its exact head is available on the draft PR.
Root reviewed all three fixes at this source commit. Its independent 22-test
Rust run was still compiling at the last owner update; no result is presumed.

Draft [PR #5](https://github.com/Adanmohh/codeg/pull/5),
`feat/step2-intake-github` to `main`; never merged by this worker. Installed
gh create/view/edit help was read before using those operations. No worker
was started; no live issue/comment/email/App installation, backend mutation,
production data write, message to a person or deployment was performed.
