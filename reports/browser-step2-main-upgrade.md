# Accepted main upgrade browser check

2026-09-08, main f7650379. Root `pnpm build` and
`CARGO_BUILD_JOBS=4 cargo build --locked --no-default-features --bin codeg-server`
passed. Logs: `/tmp/ops-main-step2-build.log`, `/tmp/ops-main-step2-server-build.log`.
Restarted only the owned 4318 server (old PID33790; new PID64903) using the
existing isolated Step1 browser database. Migrations 000003 Ops UI, 000004 email
and 000005 intake all applied successfully. No production database used.

Actual Playwright CLI session `ops-desk-check` reloaded settings, navigated to
workspace, expanded sidebar and opened Ops desk. Empty instance presents real
inbox creation fields with Inbox/Approvals/Morning navigation, without invented
records. Screenshot: `browser-step2/main-upgrade-empty.png`. This complements
the populated provider fixture checks in the Ops acceptance review.

Server still runs on 127.0.0.1:4318 with isolated `/tmp/ops-desk-browser-data`;
this is a local development check, not deployment or a final pi/P1/Telegram build.
Inherited proc-macro-error2 future-compatibility warning remains.
