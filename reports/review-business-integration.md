# Independent B and tenancy integration review

Review is active; **no integrated runtime verdict yet**. Own branch
`review/business-integration` starts at accepted main
`be46d0544f5fc5f33d3785ac5a04c5d1fd2c1935`. The completed
`feat/business-tenancy` branch remains pushed at
`b3dfbcb602314cee6eb0039d92cb997762c02dd8`; its product29774b50 is accepted in
main mergeb3f2f6d03. This review makes no product edits or native-window enablement.

## Frozen inputs and preservation

- B checkpoint `15bb402b9265006930cc9ce4428332d04c9fb34a`, tree
  `9a30f085bdc346e08c884f6519e53d73dc3a4357`; product
  `e9c6323760497e7b294282d2746cfe1d9b2774fa`. Full owner report read. Earlier
  independent67708b07 review covered only setup/reader/legacy/migration, not the
  candidate/import/recovery implementation now under review.
- PR29 report `191f69b3c2b827ce2f712bfcad1f995a201981bb`, tree
  `bc756e23665015e53e3565f21368e9a25b85a67e`; product
  `c1e618dede15368d4af39ac74391f18931b317a5`. Full report read. Its4350 preview
  currently uses a separate4353 task/settings DB and unavailable4351 intake;
  that is not integrated B evidence. Existing owner sessions/records are untouched.
- Both source pins were verified through `gh api`; own fresh archives are
  `.build/business-integration-review/{15bb402b,ui-191f69b3}/source`.
  Initial blob correspondence: B756/756 Rust/Pi/NOTICE/LICENSE;
  UI1229/1229 src/NOTICE/LICENSE. Full manifests are in each archive's
  sibling evidence directory. No uncommitted worker code copied.
- Fresh session `01a084ee-129a-7561-9f10-64266115e759` emitted own-worktree
  PreToolUse22873/22875 and PostToolUse22874/22876 in the live docs-first audit.
  Old stopped PID49733 and inherited fixtures were not resumed, killed or polled
  through old exec IDs. All previous targets/exports remain unchanged.
  Paused visual report SHA256 `abc5fd394af657568abe32a5333df27210115204c4e7eb2630ac59b71caa4060` remains in place, untracked.

## Bounded gates and ownership

Root reserved tickets-owned **m20260909_000013_business_intake_epochs**.
Accepted000012 remains unchanged. The planned nullable captured-epoch fields
have no backfill/current-epoch default; missing retained authority requires fresh
validation. A helper solely inside000012 would miss an installation that already
recorded it. Installed SeaORM1.1.19 `migrator.rs` filters pending names against
recorded receipts, iterates pending registry entries, calls each up, then inserts
its receipt; tests must use this real path, not call a helper alone.

| Gate | Required positive and negative evidence |
| --- | --- |
| Migration paths | Fresh combined; populated old000011→000012→000013; already-recorded A-only000012 then add B000011/000013. Verify actual receipt/order/retry, FK enforcement, retained rows/NULL epochs and rollback without relying on last migration index. |
| Captured authority | Same Principal across await/final writer; suspend/resume rejects old setup/import work. Fresh credential permits new work but cannot disclose or accept old observations/previews. Explicit refresh/rebase restores only the intended scope. |
| Two tenants | Same DB, real member credentials: own binding/grant/import/source/candidate positive controls; foreign IDs, receipts, member references/counts and actor/org spoofing denied. Tenant Fireflies admin remains separate from actual platform legacy entrustment. |
| Claims and source ordering | Two imports share a source fence; expired/cancelled/older responses cannot commit. Original credential/member/owner/grant changes fence final writes. Durable rediscovery uses the current human, never reconstructed identity. |
| Publication/recovery | Exact reviewed draft, current destination Read/Create/Assign/edit/CAS; task/decision/link/receipt/audit atomicity; same operation replay creates no duplicate; terminal target redaction and private-source audience remain independent. |
| Storage and projections | Staged late/uncertain secret cleanup deletes only proved orphans. Email/Hafidh projections exclude private notes, reporter/proof/config and do not refresh or send. Resolved strict-store tests are not repeated without a changed concern. |
| Shared tabs/panes | Actual session/tenant switch aborts in-flight requests and clears private caches/drafts/selection/portals. Cancelled disconnect and same-tenant locale/viewport/pane/settings changes preserve unsaved edits. No private persistent browser storage or legacy API/engine fallback. |
| Settings and source UI | Current tenant-scoped palette/layout, role/CAS conflict and lost-response recovery; stale/withheld passages/draft never shown as fresh; explicit confirmation resets on payload/target/revision changes. Protected API/browser evidence uses one integrated DB and separate synthetic records. |

Compiling integrated head and focused selectors requested from tickets. No new
fixture, browser, test process or target has been started in this review yet.
Provider-loopback/Playwright CLI checks follow a coordinated immutable fixture
handoff; no intercepted frontend JSON will count as real B acceptance. No broad
unchanged A suite, live providers/models, new dependencies or workers are planned.
Native shared-channel isolation stays outside this increment and unavailable.

## Grounding and current evidence limits

Current governing documents, accepted intake/access contracts and BI1–10 read.
Separate installed React19.2.4 package/Cargo manifest reads ran. Code-context
used existing rag-skills Python with HF_HUB_OFFLINE=1: guide exit0; docs exit3
because approvals.db is absent. Installed pinned source is used directly;
other-project database advice does not override the accepted shared-DB contract.
Git/archive/Herdr help read; archive and immutable pin checks exited0. No
implementation/runtime test pass is claimed from this preparation checkpoint.
Borrowing remains approved Codeg Apache/IntroMail and exact Fireflies MIT source;
no new source port is introduced by this report. Full changed-source/license
verification and focused execution will be recorded by exact integrated head.
