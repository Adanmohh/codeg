# Pi P1 bridge review

In progress,2026-09-08. Productb15f11a7 and clean additive accepted-main
integrationae131cf8 reviewed. Root read full cached host projections, token
engine routing, closed schemas, adapter configuration/broker and four full-path
regressions. No acceptance before real companion/adapter and browser evidence.

- Cached reads derive one enabled product from live account/task/folder in the
  same read transaction; missing/ambiguous scope is explicit. Public fields
  exclude proof objects, private reporter refs, credentials and rendered evidence.
  No cache/freshness/draft mutation occurs in reads.
- Native proposal accepts only draftId/revision and reuses accepted host
  preparation/validation. Response is pending public metadata, no capability.
- Fixed companion intake feature exposes only three reads; adapter enables
  one-use approval, closes resources/sampling/elicitation/automatic auth and
  overrides trusted launch path. Existing broker snapshots/freeze remain.
- Independent real token→listener→engine→host tests pass: **4/4,exit0,0.42s**
  at integratedae131cf8, root Cargo target. Cache/private-field invariants,
  closed/foreign/ambiguous input, stable Pi deny/pending floor/idempotent exact
  proposal, cancellation/peer-abort are exercised. Log:
  `/tmp/ops-pi-p1-independent-cases.log`. An initial incorrect module filter
  ran zero tests; it is not counted and was replaced with `desk_issues_`.

Final worker runtime/companion/process/browser gates and exact head remain.
