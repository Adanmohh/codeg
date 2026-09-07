# P1 bridge integration acceptance checklist

Root planning,2026-09-08. PR8 email bridge is accepted. Begin implementation
only after PR9 host acceptance; root writes no product code. The pi worker
will own a new worktree branch, reusing its existing Herdr pane.

- Add native desk_propose_issue with closed draftId/string and positive
  expectedRevision, through the accepted token/listener/live-run path to the
  host prepare_and_propose. No identity/payload/proof/config/approval fields.
- Implement the three scoped public cached reads in the accepted host agent
  module and wire them through fixed per-launch companion/MCP configuration
  to the existing broker allowlist. Human/operator imports and refreshes remain
  the only source of trusted upstream freshness; no agent credential access.
- Make tool descriptions state cached semantics explicitly. The standalone
  Python hafidh_feedback_get revalidates upstream; a cached companion read
  must not claim that it did so. Report cache absence/expiry and the required
  operator action without inventing data.
- Provide enough public scoped draft metadata to select a human-prepared
  draft/revision for proposal. Derive account/task/run/folder from trusted
  state and enforce product binding. No private reporter identity or raw
  secret/error projection. Missing/multiple product cases must be explicit.
- Preserve stable pi policy identity, default propose scope, deny precedence,
  destructive human floor, evidence validation and current-task transaction
  checks. Reuse host logic instead of duplicating validation or SQL elsewhere.
- Prove actual installed adapter/companion discovery of the three read tools,
  not only an empty server configuration; no model messages are necessary.
  Prove closed inputs, foreign/stale/canceled contexts, no freshness writes,
  exact proposal metadata and no execution capability across the real bridge.
- Run relevant desktop/server/companion and focused regression gates, actual
  Playwright CLI review of the resulting proposal using local fixtures, and
  update pinned borrowing/NOTICE/report. Live inference remains a separate
  unavailable-client configuration check; never downgrade silently.
