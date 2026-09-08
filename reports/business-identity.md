# Business identity implementation

Implemented on `feat/business-identity`, based on accepted `4ec04d7282a50529335d724438d42b99a53385a2`; product checkpoint `861fb0ef4394d4980a19ba375bb4c1f3f218d39b` is pushed in draft PR23. The published interface is [business-identity.md](../docs/contracts/business-identity.md). Individual revocable member credentials, role/domain checks and transaction-compatible principals are available through a separate business router and native owner commands. Identity migration000009 is owned here; tasks000010 and frontend remain with their assigned workers. Final local API fixture evidence is in progress; no integrated business task/UI acceptance is claimed.

The complete business scope, FOUNDING, ORCHESTRATOR, STATUS, DECISIONS and all three research reports were read before changes. The task worker's `cb2e184f` contract was read directly. Source authorization gaps are addressed by current org/member/credential and destination-reference checks in the same writer transaction; assignment and caller actor fields provide no authority.

## Preservation and docs evidence

- Tracked work was clean before branch creation. `git fetch origin`, exact base resolution and `git switch -c feat/business-identity origin/main`: exit0.
- The sole colliding untracked research report was byte-compared with accepted main (cmp exit0) and moved intact to ignored `.playwright-cli/checkpoints/business-identity/research-meeting-orchestration.md`. Its identical main copy is now tracked. Paused visual baseline, `.build/`, `out-design-final/` and existing fixtures remain untouched.
- Separate full installed reads: React19.2.4 package.json and src-tauri/Cargo.toml. code-context offline guide exit0 using `/Users/mohamedadan/projects/rag-skills/.venv/bin/python`, HF_HUB_OFFLINE=1. Applicable corpus rule: B2B invite-only auth, no public enrollment. No approvals-specific installed-doc corpus exists (docs lookup exit3); direct installed crate sources are the authority, not claimed RAG coverage.
- Live docs-first hook evidence for this session `01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`, own cwd: PreToolUse Bash at1788854101 and PostToolUse Bash at1788853884, exit0, in `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`. Hooks remain enabled.
- Local pinned sources read: rand_core0.6.4 OsRng/try_fill_bytes, sha2 0.10.9 Digest, base64 0.22.1 URL_SAFE_NO_PAD, Axum0.8.8 from_fn/Extension, SeaORM1.1.19 transaction. No installs or lockfile changes.

## Source mapping

- IntroMail `0bd24dfe284b888aa9f602fa1fd00e337ea38874`, `backend/app/services/mcp/auth.py`, blob `1faab9205afa2a65c02ff284b12c867c0b85ba99`: opaque random bearer, SHA-256 storage, revoke and server-derived principal into business_identity. Read through gh api, including history at introduction `13be7b092b6bec9db48e1ac3b71fcdc7493f1fc5`; no identified copyleft provenance in this auth file. Private owner authorization applies per FOUNDING. `backend/app/security.py` blob `deda3cda6ec2883822876153a37fc5ebd4b89640` read as reference only; no JWT/password implementation port.
- Accepted Codeg `4ec04d7282a50529335d724438d42b99a53385a2`: web/auth.rs blob `861ad112be52f682e319255ae3173c65b1aa1cef`, web/handlers/ops.rs `ed2bf828943d0fc538b0e30a71175e42fcbfac11`, commands/ops.rs `03b869d1b2c67ec99e7527c6f8377c75111b4606`, migration008 `abcf6ce3d2bd95a4017c2ee98c6d99da30dbd6f0`: existing protected transport, dual-runtime wrapper and transactional migration style. Under retained Codeg Apache-2.0 attribution; product NOTICE mapping will be appended with implementation.

## Checks and remaining work

The first independently usable identity implementation is ready for source review: separate business bearer auth, operator-only atomic bootstrap, member management, individual credential issue/revoke, domain/role permissions, immutable identity audit, and private delegation lineage. HTTP/native wrappers share the same core. No provider/client/engine authority is exposed to member credentials.

Current checks (own target only):

| Command | Result | Evidence |
|---|---|---|
| `cargo check --locked --no-default-features --bin codeg-server` | exit0, 53.90s | `/tmp/business-identity-server-check.log` |
| `cargo check --locked` | exit0, 1m13s, default native runtime | `/tmp/business-identity-desktop-check.log` |
| `cargo clippy --locked --no-default-features --bin codeg-server --lib -- -D warnings` | exit0, 35.78s | `/tmp/business-identity-server-clippy.log` |
| `cargo clippy --locked --all-targets --features test-utils -- -D warnings` | exit0, 1m12s, default runtime plus test targets | `/tmp/business-identity-desktop-clippy.log` |
| `pnpm exec tsc --noEmit` | exit0, TypeScript5.8.3 | `/tmp/business-identity-typecheck.log` |
| `cargo test --locked --no-default-features --lib business_identity::tests -- --nocapture` | exit0, 13 passed, 0 failed, 0 ignored, 0.66s test runtime | `/tmp/business-identity-tests.log` |
| Initial smaller identity selector before additional cases | exit0, 3 passed | `/tmp/business-identity-tests-checkpoint.log`; superseded by13 |
| `git diff --check` | exit0 | own working tree |

The13 tests cover actual full-router anonymous/member/owner-credential bootstrap denial; all three member roles denied legacy HTTP and WebSocket paths; original operator health preserved; spoof fields rejected; viewer writes denied; at-rest hash/public-response redaction; credential/member revocation; current domain/role and reference checks; admin grant ceiling; no elevated agent role/login; persisted lineage and original credential revocation; independent SQLite connection bootstrap/CAS/revocation races; audit rollback/immutability; named migration atomicity and initialized-row preservation. The race deliberately holds one SQLite writer while a second connection waits, then verifies its cached principal is rejected after revocation commits and no synthetic protected write occurs.

Remaining evidence: owned4341 guarded API fixture. Both runtime checks/Clippy and frontend typecheck pass. Task worker can integrate the committed helper implementation; full task/UI surface stays separately owned. No live providers/accounts, agent launches, external sends or broad Settings snapshots. Existing fixtures and outputs are preserved. Frontend product code is unchanged, so no new export overwrites any running fixture output.

Initial contract commit `f3c36dc6` is pushed. Draft PR: https://github.com/Adanmohh/codeg/pull/23. Both existing workers received the exact contract path via authorized Herdr prompts (exit0); no additional agents were started.

The root BW-1–18 checklist was read in full. Explicit grant ceiling and immutable agent delegation lineage extend the contract: admin cannot grant outside its current domains or manage owner/admin identities; agent role is always member; delegating credential revocation is rechecked after persisted-link restoration. Domain-less agent authorization fails closed. This closes the task worker's persistence seam without exposing a caller principal constructor. All Rust output is isolated in `.build/business-identity-target`.

The test linker reports inherited macOS `__eh_frame` size warning; proc-macro-error2 2.0.1 reports an upstream future-compatibility warning. No dependency upgrade was made. Early unused test imports were removed before the13-test run.

## Guarded manual fixture (test-only addition)

`business_identity::tests::fixture::business_identity_browser_fixture` is ignored by normal suites. It uses a fresh in-memory DB and the actual protected identity router on `127.0.0.1:4341`, with an outer guard allowing only `/api/business/*`, `/api/health` and the explicit test landing `/__business_fixture`. Every other path is blocked. No provider/agent/scheduler starts. Legacy auth is independently tested against the full unguarded application router in the13-test suite; the fixture guard is not claimed as that authentication proof. It does not read or overwrite any old static export. Port4341 was free at initial listener check; previous4327–4330 and root fixtures remain untouched.

The public, test-only operator constant is `business-identity-synthetic-operator`. New synthetic member credentials are minted through protected APIs in browser memory; test output contains only IDs/status/assertions, no tokens or hashes. The two CLI scripts in `reports/business-identity-evidence/` are API validation glue, not a business workspace UI. Playwright CLI0.1.18 with installed Playwright1.63.0-alpha-2026-08-05 source/types/help was read; no update was installed. Fixture exact source mapping is appended to NOTICE. PID/run results will follow after the test-only fixture build finishes.

Shared task registration glue is supplied as `docs/contracts/business-tasks-registration.patch`, generated against immutable task transport checkpoint `76bb6909`, which includes identity861fb0ef. Complete HTTP/native/mod/migration sources were read through git show. Exact blobs: business_tasks/http.rs `f19bed281849971763a87269831aab5c063b13ea`, commands/business_tasks.rs `9d6891843fa2a72b60e745ce0dd11c17663fd0bb`, migration000010 `7eeee212c0bb11443342d901fa4ccea2fe2d82f2`. Four-file patch registers library/native modules, migration,12 commands and the task router before existing business auth. `git apply --check docs/contracts/business-tasks-registration.patch`: exit0. Task worker applies and compiles with its committed module; this PR remains independently compiling and contains no unaccepted task source. The earlier bf4309f5 lacked the committed wrappers and was not used to invent their API.
