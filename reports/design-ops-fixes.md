# Ops locale, direction and review copy corrections

The four scoped production corrections are implemented at **d20b1f8d** and pushed in draft [PR14](https://github.com/Adanmohh/codeg/pull/14). This evidence checkpoint precedes the requested integration of accepted main f4da7027; final combined export/recheck and report remain.

- Sole worktree: `/Users/mohamedadan/projects/_worktrees/ops-desk/rebrand`.
- Branch: `fix/design-ops`, created after a clean check/fetch from accepted main **617385194f67b3d9e5aca5d9d29f3b1baeebb070**. PR9 is accepted at 8703e00f and merged as 02e3f5d8.
- Scope: the four items in the fully read [workorder](design-ops-workorder.md). Root remains reviewer/merger. No additional agent, live provider action, inference, dependency upgrade or global/other-worktree write.
- Owned preview: **127.0.0.1:4326**, separate **out-design-ops/** export and this worktree's existing `.build/intake-host` Rust target. Other fixtures/exports remain untouched.

## Implementation boundaries

1. Keep the initial i18n boot guard, then retain the mounted provider tree while later message bundles load. Existing locale request cancellation and the backend-keyed, memory-only Ops session remain authoritative. No draft or credential persistence.
2. Mirror the two Ops mobile Back arrows using Codeg's existing RTL utility pattern. Isolate email identifiers and derive message/subject/note direction from content while the surrounding Arabic layout stays RTL.
3. Remove the account-number/duplicated connection banner ahead of correspondence. Retain inbox identity, task/thread context and actionable per-inbox connection controls.
4. Make closed-review explanations follow the real receipt state: provider acceptance is not recipient delivery; local receipt recording does not send again. Preserve approve/deny/dispatch/reconciliation conditions and payloads.

Owned production seams: `src/components/i18n-provider.tsx`, scoped `src/components/ops/{ops-page,inbox-view,proposals-view,reply-editor}.tsx`. Public component props and backend/API types remain unchanged. The phone owner may continue to reuse `ReviewCard`, `proposalStatus` and `ReplyEditor`; this task changes their presentation only. The phone route and typed issue notification implementation stay with approvals.

The only Rust change is an ignored `src-tauri/src/ops/tests/integration/design_ops_browser.rs` fixture and its one module declaration, reusing accepted synthetic provider/store/router helpers. No Rust runtime, gate, sender, migration, facade or component-prop change.

## Current evidence and fixture access

Baseline Playwright CLI reproduction is in `design-ops-before.raw` and `browser-design-ops/before-*.png`: the separate Settings tab changed English → Arabic, the Ops subtree disappeared, and reopening restored the saved `Original approved body` instead of the unsaved synthetic reply. The mobile Back SVG had no rotation; the accepted receipt also displayed the contradictory no-receipt disclaimer. The before export was built before any product edits; the same loaded baseline browser pages were used for the final captures. Two initial probe attempts needed ordinary sidebar/drawer handling; the completed returned result is the evidence.

The real-editor locale suite first failed **5/8** on the baseline, then passed **8/8** after the boot-latch correction. Initial boot, delayed/failed/superseded locale bundles, reply/private note/complete review, desktop/mobile subtree changes and colliding backend IDs/base URLs are exercised. The focused receipt/session suite passes **22 tests**. An initially unsupported test matcher was replaced with this repository's existing Vitest assertions; nested paragraph markup was corrected to a span. The first type/build attempt caught confusing the existing `DeliveryStatus` DTO with its `status` field; the function and fixtures now use the read DTO's indexed status type and the receipt callback's actual DTO return. No package or lockfile upgrade.

Root can open **http://127.0.0.1:4326/login**, enter the deliberately nonsecret fixture token **ops-design-synthetic-operator**, then open Ops desk. Current owned server PID **63050**, fixture state `.build/design-ops/e0b6e800-4361-442d-963d-c137628b62c9`, export **out-design-ops/**. Its authenticated read-only `/api/ops_design_fixture_stats` returns only synthetic IDs and provider request count. Proposal **1** is reserved for locale/edit checks; **2** is provider-accepted/local-recording-pending, **3** accepted/recorded, **4** unknown, **5** rejected, **6** denied. All four provider requests happened only against the fixture's loopback server during seeding; no runtime engine or inference starts. Please keep proposal 2's recording action untouched until this worker records its no-resend recheck. Other terminal records are safe for read-only review.

The actual rebuilt d20b1f8d UI passed English → Arabic → English for reply/private note and every review field at 1280×900 and 390×844, light/dark. Returned evidence: `design-ops-after.raw`, `design-ops-review.raw`; captures are `browser-design-ops/after-*.png`. The workspace stayed visible; both mobile Back icons rotate 180 degrees in RTL; emails/thread IDs stay LTR and Arabic note/subject content resolves RTL. No account prefix or document overflow. Synthetic private values are absent from localStorage, and provider count remains **4 → 4**. The real leave prompt still appears; cancelling preserved the note and focus on Approvals (`design-ops-leave-cancel.raw`). Two CLI attempts using event listeners closed only the named browser session; bounded CLI actions and explicit native-dialog commands completed the checks without changing browser tooling or application code.

All five actual terminal states were captured and checked (`design-ops-terminal.raw`): recording-pending, accepted/recorded, unknown, rejected and denied. No terminal offers approval/send. Proposal **2 is now fully recorded** after clicking the existing **Finish recording receipt**; the provider request count stayed **4 → 4**. Do not reseed or replay it. All provider fixtures remain synthetic; unknown proposal 4 is untouched.

Pre-integration gates: **75 frontend tests**, default desktop/server checks, desktop/server Clippy with warnings denied, and **167 Rust Ops tests (4 ignored explicit manual fixtures)** all passed, exit 0. Typecheck and separate export also passed. Exact commands are in `design-ops-rust-gates.sh` and the local `design-ops-*.log` evidence. Root independently passed all **22 Ops flow/session tests**, exit 0, `/tmp/ops-design-locale-independent.log` (owner-reported evidence, not a worker rerun).

Design Studio **55c8614dcfff33b4caa5a544b4f1f91877214878** pure buildReport consumed actual CLI probes of pending/thread plus five terminal states at both widths/themes (28 rows). The terminal surfaces have no measured contrast failures. Light selected-row muted metadata measures **4.2:1**, requiring a small owned-list foreground correction. The initial extractor also sampled unpainted inputs inside closed disclosures; that measurement limitation will be corrected explicitly before the final probe. Raw measurements remain in `design-ops-probe-{pending,thread}.raw`, `design-ops-terminal.raw` and `design-ops-measured*`.

Remaining: checkpoint/push this evidence, merge accepted main while preserving both NOTICE additions, correct/recheck the selected-row contrast, rebuild only out-design-ops, run affected frontend/integration gates, release stable 4326 and rewrite the final report. Existing 4320/4322/4323/4324 exports and state remain untouched.

## Source authority and attribution

Read the complete current STATUS/workorder/root design review/locale reproduction, and the unchanged complete FOUNDING/ORCHESTRATOR/DECISIONS/AGENTS from the preceding integration. Verified those four files are unchanged at this baseline. Read the actual i18n provider, message loader, Ops session and its backend-isolation tests, complete scoped Ops screens/DTOs and accepted browser/provider helpers before implementation.

| Immutable authority | Exact source | Reuse / intended correction |
| --- | --- | --- |
| Codeg v0.30.4, Apache-2.0, **6f6bd648b206412644842a98d9ffeebf57292bed** | `src/components/i18n-provider.tsx`, `src/i18n/messages.ts`, `src/app/layout.tsx` | Existing asynchronous locale/cache/provider architecture; minimal mount-preservation glue |
| Same Codeg pin | `src/components/ui/pagination.tsx` | Existing `rtl:rotate-180` directional icon pattern |
| Accepted Ops UI **756d064f1cc391ed1da32ba90429adef225f080d**, with accepted main **61738519** | `src/components/ops/{session,inbox-view,proposals-view,reply-editor,ops-page}.tsx`, `session.test.tsx`, `ops-flows.test.tsx` | Existing private memory lifetime, exact editor/review props and receipt presentation |
| Accepted Ops/Telegram fixture source at **617385194f67b3d9e5aca5d9d29f3b1baeebb070** | `src-tauri/src/ops/tests/integration.rs`, `integration/{browser,telegram_browser}.rs` | Real protected routes, SQLite, in-memory secrets and loopback Resend fixture; dedicated port/export only |

Original Apache LICENSE and all existing NOTICE/MIT text are preserved; the scoped Codeg reuse is appended in NOTICE. No AGPL, enterprise or PolyForm source. All remote research, if needed, uses `gh api` at immutable refs; local exact source is first and no latest-source repinning is authorized.

## Docs-first and planned verification

Repeated separate React package/Cargo manifest reads. Observed live hook records for session **01a07c1c-cf2e-73e1-bbe3-e758c8363042**, exact own cwd: **PreToolUse 7130 / PostToolUse 7114** in `/Users/mohamedadan/.codex/hooks/ops-docs-first-audit.jsonl`. Hooks remain enabled.

Code-context uses the existing `/Users/mohamedadan/projects/rag-skills/.venv/bin/python` with `HF_HUB_OFFLINE=1`: guide **exit 0**, dependency docs **exit 3**, because `data/code/rebrand.db` is absent. Relevant guidance: installed-version source, retaining navigation and clear terminal states, and no unsafe email HTML. Other returned deployment/delegation rules are outside this assignment. No corpus ingestion or private source environment change.

Installed authority so far: React **19.2.4** and its installed TypeScript props/hooks; next-intl **4.8.3**; TypeScript **5.8.3**; existing Next **16.1.6** export configuration. Further used primitives will be read before edits. Applied code-context, Playwright CLI, Design Studio audit/checklist and mobile methods. No specialist fan-out under the owner's no-agent constraint.

Planned meaningful regressions: delayed initial boot; reply/private-note/full-review edits through delayed locale messages and responsive remounts; failed/superseded locale loads; synchronous backend isolation with colliding IDs; no private values persisted or save/send invoked by a locale change; receipt-state copy and recording-only callback. Actual CLI will reproduce the original loss first on 4326, then verify English/Arabic/restored-English behavior, 390px mixed-direction content and terminal receipt views. Measured Design Studio evidence will cover the changed surfaces, with remaining inherited findings reported separately.

Commands completed: separate source reads, clean check, fetch, branch creation and Codeg pinned source reads **exit 0**; code-context exits above. One attempted Design Studio skill path was absent; its wrapper points to the actual `commands/design-audit.md`, which was then read completely. No product edit or pass was inferred from that missing path.
