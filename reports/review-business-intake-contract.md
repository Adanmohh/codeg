# Independent Increment B intake contract review — checkpoint

Review is in progress, frozen to tickets' published draft
**79a922945668a633ea6b5f7e68f9bdc08a0725a3**, draft [PR25](https://github.com/Adanmohh/codeg/pull/25).
The owner's final consistency/source ledger remains pending. **No implementation
or runtime acceptance is claimed.** This review changes no product, configuration,
dependency, provider, build target, export or fixture.

Reviewer branch `review/business-intake-contract` starts from accepted
`origin/main` **4e64476c7ad9161a5e535b5c75f592716d3ca6e2**, after checking
tracked-clean and preserving `.build/`, `out-design-final/` and the paused
`reports/visual-refresh-baseline.md` without collisions. The prior N2 source
review remains pushed; root has separately closed its actual native check.

Read the full FOUNDING, ORCHESTRATOR, STATUS, DECISIONS and AGENTS, business
implementation/scope, all three research reports and root synthesis, accepted
identity/task contracts, plus tickets' draft `docs/contracts/business-intake.md`
and `reports/business-intake-contract.md` read-only. Final review will cite the
published contract commit and exact lines; a frozen documentation proposal is
not an implemented boundary.

## Source-grounded checks so far

- Existing `business_identity::Principal` has private authority. HTTP resolves
  actual operator transport separately from a member credential; an owner role
  is not `is_operator()`. Native commands derive their original operator boundary.
  Intake must reuse this boundary, never restore human authority from stored IDs
  or reuse the agent-only delegation constructor for an import worker.
- `begin_write` obtains SQLite writer ownership before identity reads.
  `authorize` rechecks the current human and original credential, and agents
  retain a separate delegator intersection. The proposed before/after-provider
  transaction checks align with these existing seams.
- `business_tasks::store::create` currently validates references, writes task and
  activity, builds its result and commits its own transaction. Atomic candidate
  acceptance needs the stated task-owner transaction helper extraction; HTTP
  create followed by an intake link write cannot supply that guarantee.
- The draft distinguishes private source grants from domain-public accepted task
  text, preserves changed-source revisions and pending human drafts, and resumes
  imports with each current human request. Those are coherent directions, not
  implemented guarantees.

## Remaining bounded review

The important prerequisite is a closed, protected binding/grant/publication API.
The draft currently reserves its implementation separately. Review will specify
the smallest callable operator contract and transaction interfaces needed to make
B usable, including first-owner import grants, scope/credential rebind fencing,
publication ceilings and revocation. Configuration alone cannot grant disclosure
or turn an ordinary owner credential into operator authority.

Also inspect operation-receipt/claim recovery, current actor versus original
requester history, source freshness races, publication/link receipts and existing
email/Hafidh projection limits. Independently verify proposed borrowed files,
immutable source refs and licence/NOTICE provenance; no broad connector survey.
Concrete gaps will be relayed to tickets/root and frozen against the published
contract. No tests/builds or active account validation are authorized in this
docs-only review.

## Docs-first evidence

Separate local React package and Cargo manifest reads preceded work. Applied
code-context using the existing rag-skills venv and `HF_HUB_OFFLINE=1`: guide
exit 0; relevant rule **Atomic per-task staging** requires staging only this
report. Other-project authorization/legal examples establish no intake rule.
Docs query exited 3 because `approvals.db` is absent; no ingestion/install.
Installed sources and the accepted core remain the authority.

Live hook audit before this report write contains PreToolUse and PostToolUse at
**1788889161**, session `01a07c1c-d3a3-7c22-a5b6-cedce2970d8d`, this approvals
worktree, exit 0. Hooks remain enabled. Git fetch/collision check/branch creation
exited 0. Remote research uses installed `gh api` help and immutable refs only;
no secret values are requested or recorded. Authorized Herdr prompt requested the
owner's exact contract handoff without touching its worktree.
