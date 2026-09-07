# Telegram review notification acceptance

Accepted email checkpoint,2026-09-08. Reviewed PR10 at
`c7a46acf30296a9e7c41a5a5c57d47afaeaa00e4`, merged as
`c3cef09a896308b2501947e5aaafa533e36d9053`. Final report/source/fixture
and pinned NOTICE mapping are reviewed. Worker desktop/server checks and
Clippy,13 scoped+12 inherited Telegram tests,11 frontend tests,typecheck/export
pass. Root independently verified13 Rust/11 frontend tests, full phone edited
approval and BC14 receipt-only completion. Earlier review entries follow.

This accepts opt-in email notifications/phone review; typed P1 issue review,
final Design fixes and combined/native validation remain. No live bot/phone
connectivity, provider credentials, model inference or deployment is claimed.

# Telegram review — in progress

2026-09-08. Draft PR10, early uncommitted implementation after contract
84c44414. Not accepted or browser-validated. Root reviewed the contract and
first typed DTO/migration/notice projection plus scoped Telegram plain-send
adapter. Later source and complete report still require review.

Early findings sent to owner before final scheduler integration:

- Account scan permits20 notices; tick permits32 accounts. Each notice may
  await15s recipient lookup plus15s send. Bound the whole operation and
  ensure unrelated inherited schedules cannot stall behind this loop. Preserve
  durable unknown state if cancellation follows a send attempt.
- Failed getChat preflight leaves a unique proposal notice in failed state,
  while later scans exclude every existing notice. A temporary read-only
  preflight outage therefore cannot recover through Notify now even though
  no message was attempted. Distinguish safe pre-send recovery from attempted
  or ambiguous sends, and test recovery without duplicate notifications.
- Keep the scoped credentialed HTTP client consistent with accepted no-proxy
  transport policy, or document a concrete need for proxy behavior.

The contract correctly keeps ACP approve-always separate, sends no private
proposal text/token/capability in its locator, requires an authenticated full
review, and uses fixed errors rather than token-containing Telegram URLs.
No final safety/functional pass is claimed until implementation tests and
actual mobile Playwright login/review fixtures are complete.

The owner also holds the independently reproduced locale-change edit-loss
correction for the final design loop: `design-loop-1/locale-edit-loss.md`.

## Independent source and Rust check

At pushed932419e5 with only test-module visibility additions, root reviewed
the full notification module, scoped Telegram adapter, review resolver/page,
closed locator parser and login/scheduler changes. Whole scan8s plus cleanup1s
now bounds scheduler impact. Preflight failure is retryable; replacement claim
IDs prevent a resumed checker sending after its lease was replaced. Sending
and unknown are never reclaimed. Secret lookup is off-executor and bounded.
Scoped client has no proxy, redirect, protocol retry or rich-send fallback.

Root own-target `cargo test --locked --no-default-features --bin codeg-server
--lib ops_telegram` passed13 tests,1manual ignored,0.86s,exit0.
Log `/tmp/ops-telegram-independent-rust.log`. Tested mod.rs SHA256
`2e4f4bd43a12d375c149dfd7204b74113e5af9c15e5a37294d3d5a40d82351c9`;
tests.rs `cab528bc9fc67e9bf938e5f093ceee9c4aaebbc1cb75904ac958fde1762d13a4`.
Final worker gates/report, browser decisions and exact-head acceptance remain.

Independent frontend selector passed11/11,exit0,1.07s: locator/settings and
accepted Ops flows/session tests. Log `/tmp/ops-telegram-independent-frontend.log`.
These are facade-mocked component tests; the separate actual CLI evidence
uses the protected4323 router/provider fixtures. The worker also corrected the
inherited Telegram test-only CodegTopics string that Step1 rebranding had made
inconsistent with its unchanged configured username; final test result awaits
its committed handoff. No production username-matching logic changed.

Root actual phone decision and BC14 completed successfully after fixture release:
proposal1 edited Bcc/body → accepted receipt/public reply, email requests1→2;
proposal2 stored receipt → local completion/public reply, requests remain2.
Telegram requests remain4 throughout. Exactly one recorded outgoing copy per
checked thread. [Evidence](browser-phone-independent/README.md). Final worker
report/head/gates remain before acceptance; no need to repeat unchanged flows.
