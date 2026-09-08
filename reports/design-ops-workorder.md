# Ops design corrections after host acceptance

Reserved for rebrand in a new fix/design-ops branch after PR9 acceptance.
Root writes no product code. Tickets owns Pi/settings/shell accessibility;
approvals owns the phone page and typed issue notification extension.

Use the complete founding/orchestration docs, current STATUS, installed dependency
source, docs-first hook and exact gh-api borrowing/NOTICE rules. Read root
reports/design-loop-1/review.md and locale-edit-loss.md before implementation.

1. Preserve mounted workspace state after the initial i18n boot. A separate
   Settings tab switching language currently unmounts the entire provider tree
   while messages load and silently loses unsaved reply edits. Keep the initial
   boot guard and backend/account isolation. Never persist private drafts or
   credentials in localStorage. Check reply, note and exact-review edits across
   real language changes, including desktop/mobile transitions and restored
   English. Loading the next message bundle must not discard the current tree.
2. Mirror directional back arrows in RTL and isolate email identifiers/message
   text direction where needed. Preserve the surrounding Arabic layout and
   mixed-language content; inspect actual 390px screenshots and overflow.
3. Remove internal account-number prefixes and redundant connection guidance
   that take space ahead of correspondence. Keep useful context that helps
   distinguish the actual inbox/task; do not change authority or data scoping.
4. Make terminal review copy agree with delivery state. A real provider receipt
   is currently followed by generic no-receipt wording. Distinguish provider
   acceptance from recipient delivery, denial, failure and unknown accurately;
   retain the existing database-only Finish recording operation and no-resend
   behavior. This is presentation, not a delivery-policy change.

Use an owned fixture/export at4326, preserving other running outputs. Meaningful
regressions must prove state survives the locale-loading transition and backend
change still isolates it. Run relevant typecheck/lint/export and actual Playwright
CLI before/after cases. Apply Design Studio measured rechecks on changed surfaces,
record any remaining findings, commit/push a draft PR and deliver
reports/design-ops-fixes.md. Do not start this branch before PR9 is accepted.
