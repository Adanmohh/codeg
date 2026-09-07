# Step 2 intake / GitHub implementation

Status: implementation and final validation underway. Branch `feat/step2-intake-github`, created after
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

Exact functions for UI registry/review integration:

```rust
prepare(&DatabaseConnection, IssueDraftV1) -> Result<PreparedIssue, IntakeError> // async
PreparedIssue::payload(&self) -> Result<serde_json::Value, IntakeError>
PreparedIssue::payload_digest(&self) -> Result<String, IntakeError>
dispatch(&DatabaseConnection, &GithubAppClient, AuthorizedAction)
    -> Result<FilingReceipt, IntakeError> // async
filing_status(&DatabaseConnection, &SourceRef, repository_id: i64)
    -> Result<Option<FilingReceipt>, IntakeError> // async, no network
reconcile_filing(&DatabaseConnection, &GithubAppClient, attempt_id: i64)
    -> Result<FilingReceipt, IntakeError> // async, GitHub reads only
GithubAppClient::new(Option<GithubAppConfig>) -> Result<GithubAppClient, IntakeError>
GithubAppClient::is_configured(&self) -> bool
```

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
An explicit existing `ops_agent_scope` for the agent/domain/repo (or domain
default) must be `propose` or `act_low_risk`; missing/read scope is denied.
Host calls `record_source` only after authorized successful GET revalidation,
never from an agent-asserted timestamp/revision. Local source freshness is
bounded to 15 minutes; re-read via intake before human review when stale.

`EvidenceAttachment` deliberately is not Deserialize. It takes the source/ref
revision, field, value, reviewed sanitized UTF-8 `content`, optional capture
time/session ULID/expiry and `EvidenceProvenance` (AscBuild, HumanReport,
Recorder, LocalDiagnostic, SessionDiagnostic). The human actor is a trusted
opaque ID. SessionDiagnostic requires the explicit session ULID; screenshot,
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
1584 and PostToolUse line 1585, timestamp 1788813122. Hooks remain enabled.

## Exact source mapping (implementation ledger)

The accepted contract contains the full immutable research ledger. Port inputs:

| Source / immutable revision | Exact files | Destination / adaptation |
| --- | --- | --- |
| codeg v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed`, Apache-2.0 | `src-tauri/src/db/service/work_task_service.rs`, `src-tauri/src/models/work_task.rs`, `src-tauri/src/db/migration/m20260801_000003_work_task_template.rs` | Existing template snapshot, SQLite transaction/CAS and migration patterns; strict evidence validation is new boundary glue. |
| intromail `0bd24dfe284b888aa9f602fa1fd00e337ea38874`, owner-authorized source without repository license | `backend/app/services/github/client.py`, `backend/app/services/github/actions.py`, `backend/tests/test_github_pack.py` | Rust App JWT/install-token cache and destructive action patterns. No create-issue helper exists; narrow REST call is glue. |
| python-sdk v2.0.1 `8b191a433634d64b1306d7d51be8b16e14cc0893`, MIT | `src/mcp/server/mcpserver/server.py`, `examples/mcpserver/readme-quickstart.py`, `examples/mcpserver/weather_structured.py` | `MCPServer` stdio tools and strict structured results. v2 FastMCP path intentionally unavailable. |
| Hafidh `a83794708193a18611c8e6eb8e88637b8136e471`, owner's read-only source | `backend/app/modules/testflight/{models.py,schemas.py,admin_router.py,triage.py}`, `backend/app/modules/feedback/{models.py,feedback_schemas.py,feedback_router.py}`, `backend/app/common/utils/operation_result.py` | Read-only TestFlight adapter, normalized DTOs and exact triage port. No invented in-app read API. |
| github/docs `831337b0fed60b90a72e2711a41dfcad72b5f288` and github/rest-api-description `3cef12e8a02d612ad032473d4fb87266f2befeae` | Exact paths in accepted contract | Issue creation / installation token HTTP contracts, API `2022-11-28`, not a source-version upgrade. |

## Validation and limits

Pending: isolated Python dependency pins and stdio/GET/privacy tests; synthetic
Rust RSA/HTTP/SQLite gate/cache/lost-response tests; desktop/server locked
checks, focused Clippy and TypeScript check. No live credentials or external
sends will be used. Missing installation, repository IDs and intake credential
remain real configuration gaps, not blockers to independent fixture validation.

Initial interface checkpoint: `39d00445`, pushed. Draft PR:
https://github.com/Adanmohh/codeg/pull/5 (main base, no merge).

Python checkpoint: isolated Python 3.13.14 environment; exact `mcp==2.0.1`,
`httpx==0.28.1`, `pydantic==2.12.5` and all resolved transitive/test pins recorded
in `integrations/hafidh-intake/requirements.lock`. No existing env changed.
Installed SDK client stdio, MCPServer tool/call handler, HTTPX streaming and
Pydantic validation primitives read before implementation. A guessed SDK
`client/transports/stdio.py` lookup failed; actual installed `client/stdio.py`
was discovered and read. SDK-generated nested validation echoed bad input;
the public `call_tool` boundary now emits a fixed sanitized error instead.

Commands/exits: initial pytest collection RED (2, module absent); unit tests
10 passed (0); real stdio privacy test RED (1, input echo); fixed suite 11 passed
(0), including actual SDK stdio discovery/list/read against synthetic HTTP.
GET-only auth failures, unavailable in-app, strict source shapes, filtered
counts, duplicate rows, cursor binding and stale-cache access covered. HTTPX
query logging suppressed. Free-text redaction remains bounded, human outward
review mandatory.

Rust signing choice: established `jsonwebtoken = "=9.3.1"`, tag commit
`87bbe49004de17ac1c64bf25d7751c0e43cff5dc`; local registry had none before
selection. Read README, Cargo.toml and `src/encoding.rs` via immutable `gh api`;
retained dependency MIT license. Reqwest 0.12.28 source shows default protocol
retries, so the new client must use `reqwest::retry::never()` and no redirects.
Bootstrap server `cargo check --no-default-features --bin codeg-server` passed
(0) while resolving the new lock entries; focused implementation checks remain.

Rust implementation checkpoint: locked server check passed (0); first focused
server library suite passed 15/15 (0). Coverage includes real RS256 verification,
narrow single-flight token cache, repo/installation separation, early expiry,
malformed token grants, strict proofs, edited payload handoff, read scopes and
destructive floor, stale review/run, evidence tamper/revocation, concurrent
filing reservation, lost response, read reconciliation, label mismatch, known
rejection and migration round trip. Latest follow-up persists/observes retry
deadlines across new client instances; final rerun remains pending.
