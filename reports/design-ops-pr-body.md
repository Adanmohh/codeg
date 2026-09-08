Changing language in a separate Settings tab currently unmounts the workspace and loses unsaved Ops replies, private notes and review edits. Keep the initial boot guard, then retain the mounted workspace while later locale bundles load. Mirror Ops Back arrows in RTL, isolate email/content direction, remove the account/connection banner and make terminal explanations follow the real receipt state.

The existing memory-only backend boundary, component props, approval/sender gates and receipt-only reconciliation remain unchanged. Includes actual-editor locale/backend regressions, receipt-copy/callback tests and a separate synthetic protected-route fixture at 4326/out-design-ops.

Baseline browser failure reproduced; focused tests pass. Final browser/Design Studio and desktop/server gates are in progress. Source mapping, fixture access and current evidence: reports/design-ops-fixes.md. Root remains reviewer/merger; keep this PR draft.
