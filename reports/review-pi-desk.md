# Pi Desk integration review

In progress, 2026-09-08. PR #8 is not accepted. Reviewed the extension,
closed tool schemas, snapshot validation, one-shot IPC transport, synchronous
adapter broker and launch wrapper at `c0d2542a`. Backend/default integration
and shared Ops helpers are still being connected.

Independent `pnpm exec vitest run --config integrations/pi-desk/vitest.config.ts`
passed: exit 0, 11 tests, including actual installed pi 0.85.1 RPC discovery of
both Desk and the isolated adapter without inference. Log:
`/tmp/ops-pi-independent-vitest.log`. Production extension files matched
`c0d2542a`; the in-progress tests had SHA-256:

- `desk.test.ts`: `401a7c242eef75062e73fbe31bb41fbdf95c5d10c5a472263b502749916f03f2`
- `process.test.ts`: `9c326053b25c9dcc5c162aba985b57b391bc8a9cb7553b6d92cddb42e8523804`

This is protocol/loader evidence, not a successful live task or model request.
Installed pi has no available Astra catalogue entry; explicit setup failure
and no fallback remain required. Agent draft operations must use accepted
Ops validation and transactional live-run checks; trusted intake credentials
must remain inside the intake host process. Final Rust, integration, provenance
and browser/default-agent review remain pending.
