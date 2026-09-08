# Independent business tenancy review

Review target: `f3b408dae5c724f354763961d79a17a7ae5c86f8`, PR30, tree
`d3f4cc354ceb7034c0b685b76289c1d3de4513af`, parent contract
`7516461633c163c2ac683930487e33231e630b0b`. Source base is
`f3813e3f1edb521f1d1b20d0b372643acc4123a5`; own report branch
`review/business-tenancy` starts at accepted main `0084fd3ef41983eca6cf52fbdf2dc2b8e9936cd2`.
B is preserved/pushed at `15bb402b`, including four passing recovery tests at
`6513dc15`; its production remains `e9c63237`. Paused visual report is preserved.

## Current verdict and scope

No concrete blocking finding established in this first source/unchanged-test
pass. Additional reviewer probes below are still pending; this is not a complete
tenancy acceptance. Read the full implementation report, contract, product diff,
identity/settings/migration source and relevant unchanged transaction/auth code.
Scope is the pinned-connection parent rebuild, foreign-key restoration and retry,
captured/legacy delegation epochs, original organization mapping, and settings
authorization/CAS/audit. No product source, other worktree or existing fixture was
edited. All runtime databases and credentials are synthetic.

## Independently executed evidence

An own ignored `git archive` of the frozen head is under
`.docs/business-tenancy-review/f3b408da/source`. Every **731/731** tracked blob in
`src-tauri`, `integrations/pi-desk`, NOTICE and LICENSE matches the Git object;
`source-blobs-before.json` records each comparison. No uncommitted owner source
was copied. Only the new isolated `.docs/business-tenancy-review-target` is used.

From that archive's `src-tauri` directory:

```sh
env CARGO_TARGET_DIR=/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-tenancy-review-target cargo test --locked --offline --no-default-features --lib business_identity::tests > /Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-tenancy-review/f3b408da/evidence/identity-unchanged.log 2>&1
```

**Exit0: 16 passed, 0 failed, 1 manual fixture ignored; 0.99s execution,
2m23s compile.** This includes the three new owner tenancy tests and thirteen
existing identity/auth tests. The real protected router remains covered; the
ignored4341 browser fixture was not started. Linker unwind-size and existing
proc-macro-error2 future-compatibility notices remain. This is not Clippy,
desktop/native, browser, provider or full B acceptance.

Planned additional synthetic probes: cancellation while the migration owns a
connection and waits for the writer; retained populated task/execution/history
and real SeaORM receipt-insert failure/retry; settings role/agent restrictions,
audit rollback and two-connection CAS/queued lifecycle or credential revocation.
These will be reviewer-only test additions, never product corrections.

## Grounding and attribution

Full current FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS and implementation scope
read. Code-context used the existing rag-skills Python with `HF_HUB_OFFLINE=1`:
guide exit0; docs exit3 because `data/code/tickets.db` is absent. Cross-project
per-tenant-database advice does not override this accepted shared-database
contract. Installed SeaORM1.1.19, SQLx0.8.6 and SQLite3.46.0 source is the API
authority. SQLx pool `close_on_drop`, SQLite worker cancellation/acknowledgement
handling and SeaORM's SQLite migration/receipt ordering were read directly.
Live hook audit for this worktree/session `01a07c1c-d82f-7022-84db-778a438632f1`
contains PostToolUse21203–21206 and PreToolUse21211–21214. Hooks remain enabled.

Verified NOTICE's six Codeg source blobs against the immutable GitHub tree at
f3813e3f using gh api: identity `mod.rs`22a99cae, `store.rs`bb11e604,
`types.rs`145f1d0e, `tests/transactions.rs`e9cac228, migration000009 ad2db145,
and `src/lib/theme-presets.ts`9181e38c. Full original Apache-2.0 LICENSE read;
blob `261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64`. All earlier notices remain.
SQLite official `version-3.46.0` resolves through gh api to
`bebe2d8be8acfd02592c4972f4ba32c3b4e4a33f`; its public-domain LICENSE was read.
Dependency primitives are API references, not copied implementations. No
Edublend, GPL or AGPL implementation is introduced by this diff or review.

## Explicit pending boundaries

- Actual platform provisioning and same-database two-tenant transport tests are
  pending the owner's compiling implementation. This head exposes no new
  settings/platform route and does not implement restricted native sessions.
- B's persisted setup/attempt/source/preview epoch fields and000011→000012
  integrated retention remain B-owner work. No stale Principal refresh/default
  epoch is permitted. B is not included in the frozen A-only migration prefix.
- Owner has identified the entrusted-but-unlinked A run epoch seam and is
  preparing a separate immutable correction; it is not claimed fixed here.
- Root's newly supplied native prerequisite requires cross-window large/raw
  channel denial, ordered positive host delivery and no fallback queue on send
  error. A custom app-command ACL or channel interceptor alone is not native
  isolation acceptance; platform-specific responder conditions must be retained.
  Caller-window controls must exclude arbitrary target labels. These are future
  native gates, not an executed exploit or a defect claim against accepted A.
- No old targets, fixtures, exports, A bundle, real stores or credentials changed;
  no live provider/model/engine/Pi process, server launch or new dependency.
