Step 2 intake and GitHub App implementation contracts — 2026-09-07.

Research checkpoint; report-only work for FOUNDING pieces 4, 5 and 5b.
No implementation, dependency change, credential change or live service action.
The final source mapping, contracts and validation ledger are being completed
on `docs/step2-intake-contracts`.

The existing worktree was clean before `git fetch origin` and
`git switch -c docs/step2-intake-contracts origin/main` (both exit 0).
Branch base: `5bd0b1e1985fc2e0e703e500b6b06864b7b15f8c`.
Read FOUNDING.md, ORCHESTRATOR.md, STATUS.md, DECISIONS.md and AGENTS.md
completely from that base. The owner's integration amendment supersedes the
original GitHub MCP choice: use intromail's GitHub App client.

**Verified source authority**

| Source | Immutable revision | Findings |
| --- | --- | --- |
| IntroInnovation/intromail | `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | Read `backend/app/services/github/{client,actions}.py`, `backend/app/routers/github.py`, `backend/tests/test_github_pack.py`, `docs/GITHUB-INTEGRATION.md`, `backend/requirements.txt`; inspected GitHub fields in `backend/app/config.py` and `GithubInstallation` / `GithubRepo` in `backend/app/models.py`. Installation JWT/token client exists; create-issue helper/action does not. |
| modelcontextprotocol/python-sdk v2.0.1 | `8b191a433634d64b1306d7d51be8b16e14cc0893` | Tag points directly to this commit. Read `pyproject.toml`, `LICENSE`, `src/mcp/server/fastmcp.py`, `src/mcp/server/__init__.py`, relevant `src/mcp/server/mcpserver/server.py` methods, and `examples/mcpserver/{readme-quickstart,simple_echo,weather_structured}.py`. The actual class is `MCPServer`; the old FastMCP import deliberately raises `ModuleNotFoundError`. Preserve v2.0.1. |
| Local Hafidh | `a83794708193a18611c8e6eb8e88637b8136e471` | Read root AGENTS.md and backend/mobile CLAUDE.md read-only. Feedback/TestFlight models, schemas, routers, triage, ASC client, auth dependencies and client configuration names inspected. Existing unrelated dirty documentation left untouched. |
| xintaofei/codeg v0.30.4 | `6f6bd648b206412644842a98d9ffeebf57292bed` | Existing `src-tauri/src/db/entities/work_task_template.rs`, migration `m20260801_000003_work_task_template.rs`, model/service/command template surfaces supply reusable composer snapshots, not evidence enforcement. |
| github/docs | `831337b0fed60b90a72e2711a41dfcad72b5f288` | Official App JWT/install-token documentation and `src/github-apps/data/fpt-2022-11-28/server-to-server-permissions.json` confirm installation auth and Issues write for create-issue. |
| github/rest-api-description | `3cef12e8a02d612ad032473d4fb87266f2befeae` | Read relevant operations from `descriptions/api.github.com/api.github.com.2022-11-28.json`. Preserve intromail's API version `2022-11-28`; the separate newer API description does not authorize a version upgrade. |

All remote research used `gh api` after installed CLI help. No browser/web
search, alternate source version or GitHub MCP dependency was used.

**Resumable findings**

- In-app feedback exposes `POST /api/v1/feedback`, which persists and sends an
  email. There is no list/detail read route in that module. Never use submission
  as an intake poll or fixture setup against a live backend.
- TestFlight exposes authenticated admin `GET /api/v1/admin/testflight`, with
  offset/limit pagination, filters and an `OperationResult` wrapper. Its total
  counts describe the whole inbox, not the filtered page. No GET-by-ULID detail
  route exists. Screenshot and sync-status reads exist; PATCH triage and POST
  sync are outside read-only intake.
- `buildVersion` comes from the ASC included build's `attributes.version`.
  In-app Flutter feedback writes `appVersion` as `version+buildNumber`. Preserve
  source values and provenance; do not treat every app-version string as a build.
- Neither feedback model has structured screen, reciter or diagnostic-log
  fields. Recitation sessions have an optional `diagnosticPath`, but the existing
  admin listing returns metadata, not log bytes, and has no feedback relationship.
  A session must be explicitly linked to the report; do not join by email/time.
- The recorder's JSONL header has `surface`, `appVersion`, `mode` and
  `startedAt`; no reciter field in the inspected header. The listen-tap debug
  diagnostic explicitly does not run in TestFlight builds. Missing evidence
  must remain missing; no N/A or guessed reciter bypass.
- `triage.py` supplies nine named tags plus fallback `other`, and
  `high|medium|low` severity. These are editable guesses. Keep both seeded and
  human-confirmed values; do not overwrite upstream operator triage on rescan.
- Intromail authenticates with RS256 App JWT (`iat` minus 60 seconds, `exp`
  plus nine minutes), exchanges it for an installation token, caches until
  60 seconds before expiry, and uses 30-second HTTP timeouts. Its token exchange
  requests all granted installation scope. Narrow the filing token to the
  configured repository ID and Issues write; cache by App, installation,
  repository and permission set rather than installation alone.
- Official create-issue is `POST /repos/{owner}/{repo}/issues`, returning 201
  with issue identity/URL. Minimal payload is string `title`, markdown `body`,
  and approved existing label names. GitHub requires only title; the four-field
  build/screen/reciter/log gate belongs in trusted Desk glue and runs before
  proposal creation and again on the approved snapshot before dispatch.
- Reuse Step 1 approval/destructive-floor/audit behavior for outward filing.
  The branch base does not yet contain approvals. PR #3 was open at
  `e1585541014e2bd55ee882335b6ee0ff032d2af1` during research; accepted integration
  and trusted reviewer authentication are prerequisites, not assumed present.
- The existing generic Desk server bearer proves access, not a named human
  reviewer. Never expose approval/execution to the intake MCP tool set or accept
  an agent-supplied actor/approval flag.

**Configuration evidence and limits**

Only names/presence were emitted. No keys, tokens, cookies, database URLs or
credential-store values were printed or copied.

- Intromail declares `github_app_id`, `github_app_private_key`,
  `github_app_slug`, `github_webhook_secret` (environment names uppercase).
  Installation/repository mappings are database rows, not an implicit PAT.
- Hafidh declares `asc_key_id`, `asc_issuer_id`, `asc_private_key`, `asc_app_id`,
  `testflight_sync_interval_minutes`; sync requires all four ASC values.
  Keep these on Hafidh; the Desk reads already-ingested records.
- Hafidh also declares `hf_bucket_token`, `huggingface_token`,
  `recitation_bucket_id`, `recitation_retention_days`, and `admin_emails`.
  The bucket credential resolver prefers the dedicated token, then the existing
  model-repository token. Do not export either credential to agents.
- Hafidh web uses `VITE_API_URL` and local-storage key `hafidh_access_token`;
  admin access is rechecked through `Users.isAdmin` in the database. No
  purpose-built read-only machine token was found in the inspected auth code.
- `Hafidh/backend/.env`, `.env.local`, and root `.env` were absent. The web
  production config has a nonempty `VITE_API_URL`; its value was not emitted.
  The current worker environment has no names beginning `GITHUB_APP_`,
  `HAFIDH_`, `ASC_`, `TESTFLIGHT_` or `CODEG_`. This does not establish deployed
  credential absence. No live installation, repository permission, admin login,
  database, bucket or ASC API was queried.
- Installed Hafidh metadata: FastAPI 0.115.13, Pydantic 2.11.7, SQLAlchemy
  2.0.41, HTTPX 0.28.1, PyJWT 2.10.1, cryptography 49.0.0; MCP is not installed
  there. SDK v2.0.1 requires Pydantic >=2.12, HTTPX2 >=2.5.0 and matching
  `mcp-types`. Plan an isolated intake environment; do not upgrade Hafidh's
  environment or repurpose the RAG environment.

**Grounding and hooks**

Applied the code-context skill with the existing rag-skills `.venv/bin/python`
and `HF_HUB_OFFLINE=1`. Guide exited 0; the relevant general rule is
“This is not the framework you remember — heed deprecations and breaking
changes.” The owner's local-first/gh-api rule overrides the generic context7
suggestion. Docs lookup exited 3: no `data/code/rebrand.db`; no corpus coverage
for these APIs is claimed and no index/dependency was installed.

The live audit contains this session
`01a07c1c-cf2e-73e1-bbe3-e758c8363042` and this worktree: PostToolUse at line
1092 and PreToolUse at lines 1093–1095 of
`/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`. Hooks remain on.

**Remaining report work**

Finish proposed local file/endpoint/data contracts, retry and evidence rules,
licensing handoff, focused local test matrix and command ledger. Then review
the report, verify only it changed, commit/push and record delivery provenance.
