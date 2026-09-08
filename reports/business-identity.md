# Business identity implementation

In progress on `feat/business-identity`, based on accepted `4ec04d7282a50529335d724438d42b99a53385a2`. The published interface is [business-identity.md](../docs/contracts/business-identity.md). Identity migration000009 and the separately authenticated business router are owned here; tasks000010 and frontend remain with their assigned workers. No product implementation is claimed complete at this checkpoint.

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

Contract checkpoint only: no compile/test result claimed yet. Next: compiling auth/principal/migration checkpoint, early draft PR, identity CRUD/revocation/security regressions, default/server checks and Clippy, frontend typecheck. Task/UI integration uses the published contract. No live providers/accounts, agent launches, external sends or broad Settings snapshots. Existing fixtures and outputs are preserved.

Commit/PR fields will be updated after this initial publication and each implementation checkpoint.
