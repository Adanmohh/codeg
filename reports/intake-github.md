# Step 2 intake / GitHub implementation

Status: implementation underway. Branch `feat/step2-intake-github`, created after
a clean check and fetch from `dc7ce45d97ce5df3ad33a40fb80923c6467406b9`.
Accepted contract: `reports/step2-intake-contracts.md` (PR #4, merged bb9134d3).
No other worktree, Hafidh source/environment, protected planning document or
core approval-engine edits are authorized or needed.

## Module contract for the Ops UI owner

Additive Rust module: `codeg_lib::ops_intake`, with `github` submodule. This
worker owns these modules, their tests and `m20260907_000005_ops_intake` only;
minimal lib/migration exports are the shared-file seams. UI owns the registry,
authenticated command/HTTP/review adapters and all frontend changes.

Planned public interface (final signatures will be recorded here before handoff):

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

Commit / draft PR: initial interface checkpoint pending publication.
