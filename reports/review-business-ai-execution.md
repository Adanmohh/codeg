# Independent business AI authority review

**No open E1 contract blocker at `3f164c2a989cd08a523e51f1e47559c15a48c0ef`.** The four initial requirements and five wire clarifications are closed for the proposed original-operator task → persistent session → managed document → human review slice. Contract SHA256: **`e2cf8304fb5eababce79961da57061e244f2567eae11b75b88dbfcbccc6a114d`**.

This is a **contract verdict**, not implementation, launch, provider, native-isolation or runtime acceptance. E2 tenant execution and E3 connected-account adapters remain separate prerequisites; neither blocks implementing E1. All fixtures and user browsers remain untouched by this review.

## Final consistency

| Boundary | Verified contract consequence |
| --- | --- |
| E1 authority/audience | Every private profile/session/event/terminal/output/asset operation requires the actual operator, mapped original organization and current identity. Owner-role credentials and task Read do not grant host/session access. Manager stop is explicitly E2-only. |
| Original lineage/attribution | Admission retains the opaque Operator/Credential distinction and captured epoch. Recovery authenticates its request but rechecks the unchanged admitted lineage; no fresh credential ID or epoch rescues old authority. Shared-token attribution identifies the mapped owner, not an individually proven person. Producer, human submitter and credential-derived reviewer remain separate. |
| Launch/cancellation | Durable reservation identities precede effects. Unknown launch/prompt/terminal outcomes require reconciliation, not blind retry. Stop fences the exact generation before teardown; task/profile/authority changes irreversibly revoke bindings. Login or assignment away-and-back cannot revive them. |
| Managed bytes/recovery | Service-owned immutable objects are separate from scratch. Bounded descriptor-based import verifies actual bytes, then rechecks original authority/output claims in the activation writer. FS/SQLite atomicity is not claimed. Lost responses reconcile the same object; cleanup cannot delete referenced bytes or recreate them from mutable original paths. |
| Human publication/discovery | New assets/submit explicitly requires a human and atomically commits task CAS, exact versions, disclosure, activity and receipt. Existing text/review policy remains intact. Deliverable.assets exposes selected immutable references after reload; historical text rows return an empty list. |
| Public metadata | Task-owned get/content requires the exact tenant/task/deliverable/version reference and current task Read. Its separate public projection excludes session/turn/profile IDs, private input lineage, latest/counts and unrelated references, even for a session owner. It never returns private AssetVersion wholesale. |
| E1 input scope | account_snapshot fails unavailable before receipt/session/task mutation until E3 has an authorized resolver. Task/asset input IDs remain exact and scoped; read permission is not private-source publication consent. |

All route/type names are explicitly proposed. Existing private Principal/DelegationGrant, task CAS, non-Git folders and runner ordering support the intended reuse, not a claim these new APIs already exist. The full f2 contract and complete f454/3f deltas were read. [Immutable byte verification](business-ai-authority-review/3f164c2a/consistency.json) confirms all three gh-api contents match local objects/stated hashes; only the two owner documentation files changed from d1.

## Wire and implementation gates

Authenticated POST NDJSON now has a closed frame union, 1MiB frame cap, UTF-8-safe parts ≤16KiB, bounded initial state and private history pagination. Snapshot/replay and subscription share the ordering lock. Cursors bind session/generation; explicit reset replaces stream cache while retaining unsent human text. Old/foreign generations cannot silently attach. Authority checks precede initial and subsequent bounded output; revocation discards queued frames. EOF/reconnect never resends prompts. Implementation must still prove split-character/line decoding, bounded oversized replay/history, ordered resync, duplicate cursors and late output with an owned positive stream; the declared caps alone are not runtime evidence.

Content pins immutable size/hash/type, no-store/nosniff, safe disposition, explicit no-range behavior and authorization before/through streaming. UI uses authenticated fetch then a short-lived Blob URL; no credentials enter document URLs. Active previews need restricted origin/network behavior; unsupported preview may honestly be unavailable. Native bytes go only to the actual allowed original-operator window, without tenant-window enablement.

Minimum implementation acceptance remains:

1. Real protected operator positive flow plus same-org owner-role, agent, foreign tenant and revoked/captured-epoch denials across every private operation; rejected writes unchanged.
2. Actual synthetic ACP/PTY process: persistent reconnect, one launch/message per receipt, cancellation/late callback, observed teardown and explicit unknown outcome. No paid/live provider required.
3. Real version 1/version 2 bytes, hash-verified download after scratch deletion, path/link/copy races, failed activation/receipt and safe orphan recovery.
4. Two writers racing human exact-version publication; named reviewer discovers/downloads only committed versions through task routes and accepts/returns via existing CAS. Agent publication and private metadata leakage fail.
5. Actual protected CLI task/session/document/review flow with draft preservation. Component mocks, installed-client readiness and configured-provider execution stay separate evidence levels.

These gates were **not run here**. E2 still requires real process/filesystem/profile custody, complete token/dispatcher/event/file/child enforcement and two-tenant positive/negative proof. Existing Tauri2.10.2 shared response-channel limits keep restricted tenant windows unavailable; original protected platform/native access remains available. E3 still needs a pinned official adapter, account grants/captured epochs, ordered refreshes and honest freshness/last-good state. No new provider survey or framework was added.

## Evidence and handoff

The [initial review](business-ai-authority-review/d1f22f85/initial-review.md) from `3eb8f971df67386f7fd0dd3bf3bcbf920a01390d` is preserved byte-for-byte; its digest entry now points to that historical copy. The [source ledger](business-ai-authority-review/d1f22f85/source-ledger.json) records 14 local blobs/ranges matching immutable GitHub trees. Official Codeg `6f6bd648b206412644842a98d9ffeebf57292bed` Apache-2.0 license was read via gh api and matched local bytes. No third-party code port, product NOTICE change or Edublend/AGPL/GPL copy occurred.

[Live hook/model evidence](business-ai-authority-review/d1f22f85/hook-model-evidence.json) records this session/worktree's actual PreToolUse/PostToolUse and GPT-6 Astra/max, never/full-access metadata. Separate local React19.2.4/Cargo reads preceded remote research. Offline code-context guide passed; missing approvals.db coverage remains explicit (docs exit3), with direct pinned-source fallback. Final remote byte checks passed. Two bounded line-extraction commands failed on heading/range assumptions and were corrected; a same-file delete/add patch was rejected without changes before this report rewrite. Earlier failed URL glob/retry is preserved in the initial report. No AI test/build/provider/browser/fixture action occurred.

Branch: `review/business-integration`. Final verdict is frozen to exact **3f164c2a**; later report-only owner commits do not expand it. B/API/IUI closure remains `6895bedf4`. The separate [candidate correction review](review-business-intake-recovery.md) passed five focused component cases; actual4355 recovery remains pending. Root/tickets/rebrand receive this verdict through authorized internal coordination; root controls implementation dispatch and acceptance.
