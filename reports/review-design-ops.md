# Ops locale, RTL and receipt-copy review

In progress, 2026-09-08. Reviewed product checkpoint
`d20b1f8d48ba107f313f5f40b4ee3fe3cb79b6b5`, full report, NOTICE and meaningful
new regressions. Root writes no product code.

The i18n correction retains the initial guard and preserves the mounted tree
only after boot. Existing request cancellation and backend-keyed Ops state
remain. The two Back icons mirror in RTL; message/subject/note direction uses
content, and email identifiers remain LTR. Internal account boilerplate is
removed while inbox identity stays visible. Receipt explanations distinguish
accepted/recorded, accepted/recording-pending, definite failure, stopped-before-
send and unknown; the recording-only action and decision conditions are intact.

Root independently ran Ops session and flow suites: 22 passed, exit0,1.26s;
`/tmp/ops-design-locale-independent.log`. Tests include actual editor values
through delayed, failed and superseded message loads, responsive subtree changes,
complete review fields, no private persistence/save/send and colliding backend
IDs/base URLs. Receipt tests preserve exactly-once local recovery callbacks.

No blocking source finding. Actual cross-tab locale/RTL browser checks, terminal
receipt views and final worker gates/report remain before acceptance. Root will
respect reserved fixture proposal2 until the worker completes its recording-only
check and releases the stable4326 fixture. This is not final design acceptance.
