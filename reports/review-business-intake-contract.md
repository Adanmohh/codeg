# Independent Increment B intake contract review

**Final reconciliation is pending; Increment B is not implemented.** The full
`85f6001f2fa8f9d33748ceddbf7980ce68942ede` contract/report and complete change from
`79a922945668a633ea6b5f7e68f9bdc08a0725a3` have been reviewed. Its preparation,
recovery and deadline amendments are coherent. The remaining setup, explicit
scope and refresh-fence requirements are closed in the root-accepted access
contract; the intake owner must incorporate its exact reference before the final
no-open-contract-blocker verdict. No product or runtime acceptance is claimed.

The protected seam is published at **18be55edc276713fc6d46d075baec363245ba285**:
[business-intake-access.md](../docs/contracts/business-intake-access.md). Root
reviewed it completely and imported it in main `eecc7142`; tickets and rebrand
received the exact SHA. Intake owns its access module and one migration, reusing
the existing private Principal. There is no new identity constructor or credential
class. Original operator/native authority alone manages setup; individual member
credentials remain restricted to business transport.

## Frozen review scope

| Artifact | Exact reviewed pin / status |
| --- | --- |
| Initial owner contract and report | `79a922945668a633ea6b5f7e68f9bdc08a0725a3`; complete read, contract blob `79aab775dc163597b54937bc827c4b5b043af846` |
| DTO/recovery/source-ledger amendment | `85f6001f2fa8f9d33748ceddbf7980ce68942ede`; contract blob `99e5bfccecdb2d7d40afd93ceff315622536ddaf`, report blob `60e8265cc4ea9c664fd41c7e893c081882e4e26e` |
| Protected access seam | `18be55edc276713fc6d46d075baec363245ba285`; complete published contract |
| Owner reconciled contract | Awaiting immutable handoff; not assumed from uncommitted files |
| UI interaction plan | `99cecbac3e8eab1da6f977ec649d8c874e668ab9`; complete read, Q2–Q4 incorporated in 85f6001f |
| Root acceptance plan | `650be3025c25386649bc906f6bed335abafcb011`; all BI-1–BI-10 read, future acceptance requirements |

Owner [PR25](https://github.com/Adanmohh/codeg/pull/25) was independently queried
through `gh api`: draft/open at 85f6001f, based on
`c7f7366fef2d7945cea18d4da1b7594fa3026e1c`. Reviewer branch
`review/business-intake-contract` starts from accepted
`4e64476c7ad9161a5e535b5c75f592716d3ca6e2`. Read full FOUNDING, ORCHESTRATOR,
STATUS, DECISIONS, AGENTS, business scope, all three research reports/root
synthesis and accepted identity/task contracts. This is a documentation boundary
review, not an audit of a future migration or adapter implementation.

## Concrete findings and required closure

These severities describe implementation-blocking contract gaps at the cited
checkpoint. They do not imply that an unimplemented B endpoint is exploitable.

**BI-R1 / P2 — protected setup and audience were not closed at 85f6001f.**
`docs/contracts/business-intake.md:99–121` defers setup and initially gives the
named source owner access; `:316` defines a different BindingSummary from the
new access seam. Following that draft literally could either leave real operators
unable to establish grants or infer access from an owner label. Required closure:
use the access contract's exact list/status/create/update/disable/grant DTOs,
actual current `is_operator` setup check after writer ownership, zero initial
grants, explicit publication ceiling and one canonical BindingSummary. A member
whose role is owner must fail setup. The grant confirmation covers this exact
binding's retained historical versions, including captures before the grant,
and current/future imports. It advertises neither per-meeting ACLs nor org-wide
sharing. Current grant plus fresh source access gates every old-version read.

The access contract additionally pins `ownerAuthorityRevision`. Revoking or
changing the immutable source owner's human/Contribute authority, including an
away-and-back revision, pauses source use and claims. Protected update revalidates
the same owner and bumps the binding epoch; changed ownership needs a new binding.
A rename may conservatively require revalidation. Ownership itself grants nothing;
each requesting human's original credential is separately rechecked.

**BI-R2 / P2 — per-import leases alone do not order source refreshes.**
At 85f6001f, `docs/contracts/business-intake.md:150–156,180–185` invalidates
freshness and checks an import's own attempt. Reproducible future trigger: import A
reads an old source while import B refreshes the same source and commits a newer
observation; A returns last while its distinct lease remains valid. Without a
shared source attempt fence A can overwrite the newer observation or restore
freshness. Required closure: advance a source-scoped access/attempt fence before
I/O across all imports, then compare it with the import attempt/lease, content
revision, current identity and binding/grant epochs in the final writer. Neither
an expired attempt nor an older response commits. A content hash that returns to
an earlier value cannot substitute for this ordering. Access seam lines 255–263
specify the correction and its required two-import regression.

**BI-R3 / P2 prerequisite — existing token-map mutation can erase unrelated keys
on a read failure.** Tickets identified this; the reviewer independently read
`src-tauri/src/keyring_store.rs:80–117`, blob
`29fc3fb38280338aa26939c45f80ef9aefc2a394`. A corrupt/unreadable existing
`tokens.json` becomes an empty map in `read_tokens_at`; `change_token_at` then
persists a set/delete against that empty map. B staging must not reuse the path
unchanged. Required fix is a narrow strict writer-read in that existing module
under `TOKEN_WRITE_LOCK`: only actual NotFound starts empty; every other read or
parse failure returns without rename/deletion/replacement. No second intake
secret store or global authentication rewrite. Synthetic set/delete must preserve
existing bytes/unrelated credentials on failure and support genuine first creation.
No credential file was opened and no failure test was run in this review.

The accepted staging contract separately reserves unique private references,
verifies only the fixed provider identity query, then rechecks authority/attempt/
revision before DB activation. Failed staging cannot change the old active key;
uncertain commit is reconciled before deleting any possibly active reference.
Late writes may leave protected orphans but cannot activate after expiry. Native
keyring and server file writes are not SQLite-atomic; the server lock remains
process-local. Resource reassociation uses a new binding, preserving all old rows.

## Verified decision and recovery agreement

- `candidates/select` explicitly rebases current passage IDs while preserving a
  null or existing prepared draft. No read silently rebases or creates task text.
  Metadata-only responses withhold passages/draft/suggestions; hasPreparedDraft
  distinguishes withheld text from no draft. Fresh old-version text is labelled.
- PreparedTask serializes normalized exact fields with resolved owner, rather than
  serializing the existing input-only CreateInput or defaulting to a later actor.
  Accept sends expected revisions and confirmed destination. Text-free link uses
  an authorized existing task's exact ID/revision/domain and needs no fake draft.
- The existing task create must be extracted into its owner's transaction helper.
  Intake owns the outer writer; task/reference authorization, candidate/source CAS,
  task/activity, immutable decision/link and operation receipt commit together.
  Linking applies existing edit capability and invalidates pending task review.
  It does not authorize engine execution or auto-completion.
- `imports/list/get` uses current import/read grants. A new authorized human may
  resume; stored requester IDs are history, never a reconstructed Principal.
  Durable claim receipts prevent duplicate live reads. CandidateDetail.decision
  recovers lost terminal responses; inaccessible task IDs/revisions are redacted.
- Read12s/core15s apply to HTTP and native below the existing client20s limit,
  including credential verification. Expired work cannot detach and later commit;
  ambiguous completed writes reconcile by receipt/current access. Provider errors
  use safe intake reasons and never masquerade as member-session401.
- Legacy email/Hafidh capture needs exact protected account/inbox/product binding
  and a pure, narrow projection. A monotonic host configuration identity must
  fence rebind-away-and-back before capture is enabled. It may not call operator
  detail/refresh from member core, impersonate Pi context, refresh issue evidence,
  or widen private notes/proofs. This remains explicit implementation work.

Minor wire consistency sent to the owner: imports/list must spell out its allowed
unfinished/all union and default, rather than one literal plus conflicting prose.

## Source and licence verification

Local accepted source was read before proposing helpers: identity mod/http/types
and native commands; task mod/store/policy; Ops mod/email; host operator/runtime;
keyring_store. Verified local blobs at the owner's accepted base include identity
mod `22a99cae327682b04fbcced34cf5ce19f6120dad`, task store
`d48fdf1e85feb4410dbff51e8a671d6c9d525691` and NOTICE
`9731e731cea397b607ba76aa60021a0422a1f337`. Existing Codeg Apache-2.0 provenance
is v0.30.4 `6f6bd648b206412644842a98d9ffeebf57292bed`; approved IntroMail
identity/task mappings remain unchanged. No source hunk or licence text is ported
by these two review documents, so NOTICE is unchanged.

Independently read official Fireflies adapter credential/query/transport and full
MIT licence through `gh api` at **fbd24607bc784a2294ce402426aefe2cb8c00f50**:

| File | Immutable blob / conclusion |
| --- | --- |
| credentials/FirefliesApi.credentials.ts | `a39bced68aeceb23b33641e774e3509554caac49`; fixed user_id credential query and auth_failed sentinel |
| nodes/Fireflies/helpers/queries.ts | `c775b8bbe6c0bbb16f8f7e8a60467c248ad93859`; list/detail variables and selected fields, no guaranteed cursor/completion enum |
| nodes/Fireflies/transport/index.ts | `0b60c1c9e9d61f4583298cd2c0c464e9864861f5`; HTTP200 GraphQL errors checked; raw error text is excluded from reuse |
| LICENSE.md | `1e4b3a6e245384b89f24f2aef5e3f8e7fa1f4d23`; MIT, Copyright 2022 n8n, full permission/disclaimer required for later ports |

Also read IntroMail's complete provenance document
`docs/BORROW-PLANE-OPENPROJECT.md` at the unchanged approved pin
`0bd24dfe284b888aa9f602fa1fd00e337ea38874`. Its source-key/favorite/relation/
field-journal proposals are correctly excluded by the B ledger. Reading that
ledger does not license copyleft code or certify every existing IntroMail hunk.
The SDK is a schema reference only; its string-array declaration does not resolve
the hosted action_items string discrepancy. No SDK/n8n runtime/AGPL/GPL code is
adopted. Provider schema/access, offset completeness and provider expiry remain
untested; the owner's reported public-doc404 was not retried for this review.

## Commands, scope and evidence limits

| Reviewer action | Observed result |
| --- | --- |
| Separate React package/Cargo manifest and installed transaction reads | React19.2.4, SeaORM1.1.19 and SQLx SQLite0.8.6; no dependency changes |
| Existing rag-skills venv with HF_HUB_OFFLINE=1, code-context guide | Exit0; applied Atomic per-task staging; unrelated corpus rules not promoted to project policy |
| code-context docs query | Exit3, approvals.db missing; no install/ingestion or fabricated coverage |
| gh api help, immutable source reads and PR25 head query | Exit0; no provider endpoints or active accounts |
| git show/diff/rev-parse, source reads and git diff --check | Exit0; complete 79a→85 contract diff/report, truncated combined output reread in bounded chunks |
| Checkpoint commits/pushes | 6a16b009b2349e7a57aba7490e4cdba5c7111569 and 18be55edc276713fc6d46d075baec363245ba285, exit0 |
| Authorized internal Herdr prompts | Root/tickets/rebrand received exact seam and bounded findings; no new worker |
| Product tests, builds, fixture/provider/credential actions | Not run; not authorized in this docs-only scope |

Live hook evidence before the first report write: PreToolUse/PostToolUse
1788889161, own session `01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`, approvals cwd,
exit0. Hooks stayed enabled and emitted again during immutable source reads.
Selected model remains Astra/max. No claims rely on another pane's quota label.

BI-1–BI-10 and the owner's B01–B18 remain future implementation acceptance gates.
This report verifies their contract coverage, not a passing runtime assertion.
No account consent, lossless sync, unattended runner, multi-process secret-store
atomicity or same-user OS isolation is certified. Root will dispatch and review
actual backend/UI implementation separately.

Only this report and the access contract are staged on this branch. Paused
`.build/`, `out-design-final/` and `reports/visual-refresh-baseline.md` remain
untouched. All existing fixtures/exports/targets and root planning docs are
preserved. The prior pushed task/UI/native reviews remain intact.
