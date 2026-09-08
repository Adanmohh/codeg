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

## Accepted host/Pi/Telegram upgrade

Root built frontend, server and real companion from main `61738519`, containing
accepted PR #9 merge `02e3f5d8`. Both build commands exited 0; frontend is a
static export. Server/companion build took 1m14s. Logs:
`/tmp/ops-main-host-frontend-build.log` and
`/tmp/ops-main-host-server-build.log`. Root's isolated Python 3.13.14 environment
also imports the accepted host under `env -i` and `-I`, exit 0.

Stopped only prior owned server PID 64903; rebuilt server PID 49874 now serves
4318 using the same `/tmp/ops-desk-browser-data` and root `out/`. Startup applied
000006 host then 000007 Telegram successfully. Actual Playwright CLI session
`ops-desk-check` reloaded, opened Ops and then Bug intake. Empty correspondence
and unconfigured product screens render with the expected navigation and form
labels; screenshots were inspected by root:
[Ops](browser-step2/main-host-upgrade-ops.png),
[intake](browser-step2/main-host-upgrade-intake.png).

No configuration was saved and no provider/model call was made. This is an
integrated startup/migration/navigation check, not another populated P1/phone
fixture pass or final design acceptance. Existing shell unnamed buttons and
Ops internal-copy findings remain assigned to active design workers. The
unsigned native bundle still needs the final integrated rebuild.
