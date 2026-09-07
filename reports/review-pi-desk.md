# Pi Desk integration review

In progress, 2026-09-08. PR #8 is not accepted. Reviewed the extension,
closed tool schemas, snapshot validation, one-shot IPC transport, synchronous
adapter broker and launch wrapper at `c0d2542a`. Backend/default integration and shared Ops helpers are connected at7d376edb; final gates and browser default review remain.

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

## Policy identity alignment

Early review found that the in-progress `agent_type:connection_id` identity
would miss a standing rule after every new launch UUID. The worker replaced
it with the trusted parent's existing `AgentType::as_wire()` key (`pi`, not
display label `Pi`), preserving separate task/run/ancestry liveness checks.
Root independently read IntroMail's `identity.py` at the founding immutable
revision through gh api and Codeg's existing `as_wire` implementation: the
borrowed policy identity is persistent, not newly minted per request. No gate
default or permission-order change is needed. Final bridge integration must
prove a persisted `pi` deny applies across two launch UUIDs; this regression now passes through the production listener/engine/Ops path (below).

## Integrated bridge independent check

At `7d376edb` root read the live ancestry resolver, accepted Ops dispatch,
framed token listener/cancellation lease, default-agent changes and all four
new full-path regression tests. No separate operator principal or duplicated
draft SQL/validation was introduced. The shared Ops and destructive gate remain
authoritative. P1 tools intentionally await the host acceptance follow-on.

`CARGO_TARGET_DIR=/Users/mohamedadan/projects/ops-desk/src-tauri/target
CARGO_BUILD_JOBS=4 cargo test --locked --no-default-features --bin codeg-server
--lib desk_` passed:13 tests, one explicitly ignored external-client fixture,
0.42s, exit0. Log `/tmp/ops-pi-independent-rust-bridge.log`. This independently
proves stable pi deny across two launch UUIDs, human floor after deny removal,
closed inputs/public-only projections, exact draft CAS, canceled writer refusal
and peer-abort cleanup. No model inference or real outbound request.

Final extension/process suite had15 passes and one `spawn ENOEXEC` failure when
the worker desktop check replaced its own companion with the documented
compile-only placeholder. Root requested a stable rebuilt executable window
and will rerun the affected process suite. This is an outstanding artifact
validation gate, not a passing result. Log `/tmp/ops-pi-independent-final-vitest.log`.
