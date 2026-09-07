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
