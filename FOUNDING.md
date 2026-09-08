# Hafidh Ops Desk — smoother processes, with agents inside

Founding document. Research date **2026-09-07**, read-only, every claim linked
and dated. Sources in order: `gh api` on the owner's repos → official MCP servers
at their tags → runtimes and starters at their tags → local `herdr api schema
--json` / `pi --version` → docs → web. Detailed working notes are archived beside
this file. Owner amendments are recorded in DECISIONS.md; the 2026-09-07
integration amendment replaces the original Resend/GitHub MCP choices below.

**The point is not managing agents.** It is making four company processes run
smoothly, with agents doing the drafting and triage inside them and a human
approving anything that leaves the building. **Decision: fork
[codeg](https://github.com/xintaofei/codeg) at `v0.30.4`, add the few missing
pieces, and borrow schemas and patterns — not code — from Plane, Twenty,
Chatwoot and Postiz.**

---

## 1. The four processes

Written as they will actually run for Hafidh's beta. **A** marks where the agent
works; **H** marks where a human decides.

**P1 — Tester reports a bug → issue with evidence → fix → build → tester told.**
A tester writes in the group or in TestFlight. **A** ingests it, pulls the
evidence the report is useless without — build number, screen, reciter, device,
app version, the diagnostic log, the recitation-bucket session id — and files a
GitHub issue from a template, labelled from `triage.py`'s existing vocabulary
(`audio`, `recognizer`, `memorization`, `mushaf`, `counter`, `navigation`, `ui`,
`crash`, `translation`) with a seeded `Severity`. **H** confirms or corrects the
label and priority — *these are guesses, as Hafidh's own triage module says*.
**A** may propose a fix as a task in its own worktree; **H** reviews the diff
before it lands. A build goes out; **A** drafts the "fixed in build N" note back
to the reporter; **H** approves the send.

**P2 — Support email → ticket → agent draft → human approves → reply.**
**A** polls the Resend inbox, threads the message, opens or updates a ticket,
and drafts a reply grounded in the FAQ and the tester's history. **H** approves,
edits, or denies. Only then does anything send. **A** may auto-send exactly one
class of reply: deterministic, non-committal acknowledgement ("which build are
you on?", a known FAQ link, receipt confirmation). Refunds, accounts, religious
content, and any promise about a fix always stop at **H**.

**P3 — Release → notes → testers → store.** **A** assembles release notes from
merged issues since the last tag and drafts the tester announcement and the store
"what's new". **H** approves each artefact separately — notes, announcement,
store copy — because they have different audiences and different blast radii.
Publication to the stores stays manual through the existing CI workflows.

**P4 — Social post → draft → approve → schedule.** **A** drafts from a brief and
the release notes, in the product's voice. **H** approves the exact text and
image. **A** schedules it. Nothing posts unapproved, and **replies and DMs stay
manual on every platform** — a misfired auto-reply on a Qur'an app is a
reputational event, not a bug.

Across all four the shape is identical, which is the point: **agent proposes,
human disposes, everything is logged.** That single seam is the product.

---

## 2. What codeg already gives each process

Read at `v0.30.4` (Apache-2.0, ★3,259, 29 contributors, released 2026-09-06).

| Process needs | codeg already has |
|---|---|
| An approval gate that is a real state, not a dialog | `db/entities/work_task.rs`: `todo → queued → preparing → running ⇄ awaiting_input → review → merging → done`. **`awaiting_input`** is documented as *"the agent is blocked on a question / permission / plan approval"*; every transition is a **conditional UPDATE (CAS)** guarded by expected status and `run_seq`; `done` is reachable only via `review` — *"nothing reaches `done` unseen"*. |
| Agent-neutral workers | `acp/registry.rs`: 15 built-in ACP agents including **`AgentType::Pi`** (`pi-acp@0.0.33`) and `OpenCode` (pinned `v1.18.29`), plus `custom_registry` for any other ACP agent. |
| Work in isolation, reviewed before landing | git worktree per task, concurrency limit, diff review, merge verified against git rather than the agent's word. |
| Scheduled/unattended runs | `automation/engine.rs` — replays a saved composer snapshot through the ACP launch chain, settles runs from the event bus, with a reconcile backstop for dropped `TurnComplete`. Entities `automation`, `automation_run`, `work_task_template`. |
| Approvals reaching a phone | `chat_channel/` with `telegram`, `lark`, `weixin` backends; `session_bridge.rs` carries `PendingPermission { request_id, tool_description, options }` from **ACP `session/request_permission`**, and `/approve` \| `/deny` calls `respond_permission(connection_id, request_id, option_id)`. Native iOS/Android clients (Apache-2.0) pair by URL+token. |
| An event feed for anything else | `chat_channel/webhook.rs` — *"a channel-agnostic event sink"*, fire-and-forget POST per ACP event. |
| A terminal | `terminal/manager.rs` (`portable_pty`), tabbed UI, exposed over HTTP in `web/handlers/terminal.rs`; `acp/terminal_runtime.rs` for agent-run commands. |
| Secrets, persistence, headless | `keyring_store.rs` (`keyring::Entry`, real macOS Keychain); SeaORM/SQLite with dated migrations; `codeg-server` + Docker with `--supervise` as PID 1. |

**Roughly 80% of P1–P4's machinery exists.** What is missing is not infrastructure.

---

## 3. The few pieces to add — all borrowed

**Rule: we borrow, we do not write from scratch.** The only code we write is glue
between borrowed parts. Every source below was read via `gh api` at a tag on
2026-09-07.

### Source of every piece

| # | Piece | Borrowed from | Path in that repo | Licence | Glue (days) |
|---|---|---|---|---|---|
| 1 | Approval gate + audit log + edit-before-approve + **destructive floor** | **`IntroInnovation/intromail`** (our own, private) | `backend/app/services/agent/gating.py`, `proposals.py`; `backend/app/audit.py`; `models.py` → `Proposal`, `AuditLog` | ours — no third-party terms | **3** (port Python → Rust/SeaORM) |
| 1b | Destructive-confirm pattern at the agent seam | `badlogic/pi-mono` `v0.85.1` | `packages/coding-agent/examples/extensions/confirm-destructive.ts` | **MIT** | included in #6 |
| 2 | **Email threading** (RFC 5322) | `chatwoot/chatwoot` `v4.17.1` | `app/services/mailbox/conversation_finder.rb` + `conversation_finder_strategies/{base,receiver_uuid,in_reply_to,references,new_conversation}_strategy.rb` — an ordered strategy chain, with specs beside it | **MIT** (everything outside `enterprise/`) | **2** (port Ruby → Rust) |
| 2b | Ticket shape: conversation ↔ contact ↔ inbox, assignment, **private note vs public reply** | `chatwoot/chatwoot` `v4.17.1` | `app/models/{conversation,message,contact,inbox}.rb` | **MIT** | **1** (schema transcription into one SeaORM migration) |
| 2c | Resend email transport | **`IntroInnovation/intromail`**, commit `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | `backend/app/services/resend_client.py`; read adjacent send/ingest/router/test sources before porting them | ours — no third-party terms | re-estimate during Step 2 source review |
| 3 | Email panel UI (thread list, message view, composer) | **codeg itself** `v0.30.4` | `src/components/chat/*`, `src/components/message/*`, `src/components/ai-elements/*` | **Apache-2.0** | **2** (re-point existing React components at the ticket store) |
| 4 | Hafidh intake as an MCP server | `modelcontextprotocol/python-sdk` `v2.0.1` (★24,218) | FastMCP server scaffold; data models are Hafidh's own `modules/feedback`, `modules/testflight` | **MIT** | **2** |
| 5 | Issue filing through a GitHub App | **`IntroInnovation/intromail`**, commit `0bd24dfe284b888aa9f602fa1fd00e337ea38874` | `backend/app/services/github/client.py`; `docs/GITHUB-INTEGRATION.md`; read adjacent action/router/test sources before porting them | ours — no third-party terms | re-estimate during Step 2 source review |
| 5b | Issue template + required fields | **codeg itself** | `db/entities/work_task_template.rs` | **Apache-2.0** | **1** |
| 6 | pi desk extension (route approvals into the queue) | `badlogic/pi-mono` examples + `nicobailon/pi-mcp-adapter` `v2.32.1` | `examples/extensions/{confirm-destructive,bash-spawn-hook,commands}.ts` as the skeleton; the adapter's documented `MCP_TOOL_APPROVAL_REQUEST_EVENT` / `pi-mcp-adapter:tool-approval-request` broker contract | **MIT** both | **3** |
| 7 | Morning view | **codeg itself** | `src/components/tasks/{board-columns.ts,task-card.tsx,task-detail-sheet.tsx}` | **Apache-2.0** | **2** (one new query + filter over existing components) |
| 8 | Rebrand | — no code | config + assets only (§5) | — | **2** |

**Original estimate: ≈19 glue days; revalidate after the owner’s integration
amendment.** Two entries deserve the caveat the rule asks for:

- **Piece 1 has no third-party borrowable source.** Chatwoot's audit log lives in
  `enterprise/app/models/enterprise/audit/` — **restricted, not MIT** — so it is
  out. Plane's activity model is **AGPL-3.0** and viral on linked code, so it is
  out of an Apache-2.0 fork. Kun's `approval-consent.ts` is **PolyForm
  Noncommercial**, out. The closest borrowable substitute is therefore **our
  own** intromail, which is the better borrow anyway: it is already the exact
  six-step order we want (deny → ask → payload-aware check → **destructive floor
  no mode bypasses** → allow → mode), already redacts credential-shaped keys, and
  carries no third-party terms at all. The work is a port, not a design.
- **Piece 5's "refuse to file without evidence" rule is glue with no source.**
  Nobody publishes "reject this tool call unless build, screen, reciter and log
  are present." It is ~30 lines of validation in front of a borrowed filing tool,
  and it is the single highest-value rule in P1.

---

## 4. What we borrow from, and how

Verified via `gh api`, 2026-09-07. **Licence discipline decides the mode of
borrowing**: MIT means code may be ported; **AGPL-3.0 means schema shape and
vocabulary only**, never source into an Apache-2.0 tree.

| Project | Licence · signals | Mode | What we take |
|---|---|---|---|
| [chatwoot/chatwoot](https://github.com/chatwoot/chatwoot) | **MIT** outside `enterprise/` · ★36,558 · Ruby · v4.17.1 2026-08-27 | **Code — ported** | The threading strategy chain and the ticket schema (pieces 2, 2b). Its **private note vs public reply** distinction is precisely P2's internal-notes requirement. Its audit log is enterprise-licensed and excluded |
| [makeplane/plane](https://github.com/makeplane/plane) | AGPL-3.0 · ★58,995 · v1.4.2 2026-08-23 | **Vocabulary only** | State groups (backlog / unstarted / started / completed / cancelled) and issue-properties as the label model for P1 — mapped onto `work_task`, not imported |
| [twentyhq/twenty](https://github.com/twentyhq/twenty) | AGPL-3.0 · ★56,378 · v2.38.1 2026-09-06 | **Idea only** | Custom-object + field-metadata model — how People/Testers/Customers get added later without a migration each. Phase 3 at the earliest |
| [gitroomhq/postiz-app](https://github.com/gitroomhq/postiz-app) | AGPL-3.0 · ★35,539 · v2.23.0 | **Idea only** | One post modelled across many channels, with per-platform validation — read before building P4 |
| [hcengineering/platform](https://github.com/hcengineering/platform) (Huly) | EPL-2.0 · ★27,593 · v0.7.426 2026-07-05 | Reference | Too big to fork, slowest-moving of the set |
| [calcom/cal.com](https://github.com/calcom/cal.com) (MIT, ★48,235), [formbricks](https://github.com/formbricks/formbricks), [documenso](https://github.com/documenso/documenso), [dub](https://github.com/dubinc/dub) | mixed | — | No process here needs them |

**None of these is forked.** The fork is codeg; these supply modules, schemas and
vocabulary into it.

## 5. Carried decisions

Carried decisions, with owner-approved integration amendments dated 2026-09-07:

- **pi is the default agent** (installed 0.85.1; `badlogic/pi-mono@v0.85.1`, MIT,
  ★102,542). Its `tool_call` hook is **async and can `{block: true}`**, so a
  built-in tool can wait on a human — a real approval seam, not a config flag.
  **MCP comes from an extension**: [`nicobailon/pi-mcp-adapter`](https://github.com/nicobailon/pi-mcp-adapter)
  `v2.32.1` (MIT, ★1,430, 15 releases/30 d) brings remote HTTP MCP with **OAuth
  2.1 + PKCE**, DCR fallback, OS-credential-store tokens, layered per-project
  server sets, and a published `pi-mcp-adapter:tool-approval-request` broker that
  fires for *every* uncached MCP call and **fails closed when headless**. Note
  codeg's registry drops ACP-wire `mcpServers` for pi — harmless, because the
  adapter is installed *into* pi, but verify end-to-end in week one.
  **OpenCode** (`anomalyco/opencode@v1.18.29`, MIT) stays one picker change away.
- **Email is Resend through the direct REST-client pattern borrowed from our
  intromail**, not Resend MCP or a Resend CLI. Verified source:
  `backend/app/services/resend_client.py` at
  `0bd24dfe284b888aa9f602fa1fd00e337ea38874`: async HTTP, idempotency keys,
  receiving-detail fetches, and Message-ID/In-Reply-To/References on sends.
  Keep Chatwoot-derived ticket threading and the approval/audit seam. The
  existing pull-first receive decision remains; the inspected client does not
  implement received-email listing, so Step 2 must verify and borrow the missing
  polling/pagination support before implementing it. Do not claim intromail's
  webhook deployment can be copied unchanged into an unsigned local desktop.
- **GitHub integration uses a GitHub App**, borrowing intromail's installation
  authentication and REST-client pattern from
  `backend/app/services/github/client.py` at the same commit, with
  `docs/GITHUB-INTEGRATION.md` as design context. No GitHub MCP Server dependency
  for issue filing. The required build/screen/reciter/log evidence gate and human
  approval remain. The inspected client has comments/labels and other actions,
  but no create-issue helper; Step 2 must verify the issue-creation contract and
  add the minimal client glue. `gh api` remains the development/research tool.
- **One team channel, Telegram first** — codeg's backend already exists, so it is
  free. Slack outbound is free too via `webhook.rs`; a full Slack backend means a
  new `ChannelType` variant (the enum is closed) and Socket Mode in Rust, since
  official [`bolt-js`](https://github.com/slackapi/bolt-js) (MIT, ★2,941) is JS.
  **The desk hosts no chat.** **WhatsApp:** the only legitimate route is the
  Business Platform — business number, webhook inbound, per-message pricing since
  2025-07-01, free non-template replies inside the **24-hour customer service
  window**, templates only after it closes. It **cannot ingest the existing
  personal tester group**; reading the desktop client's local `ChatStorage.sqlite`
  is your own machine but is other people's messages, undocumented, and
  automating it breaks WhatsApp's terms. Move testers to Telegram.
- **Remote** is codeg's: mobile clients plus chat channels relaying ACP permission
  requests. **Internal-first**: no signing, notarisation, auto-update or store
  distribution in phase 1 — an unsigned local build on the team's Macs.
  **Rebrand is in phase 1** (piece #8). **Herdr optional**: codeg's own terminal
  ships first; Herdr (★35,910, Apache-2.0, protocol 20) is worth adding later only
  for `blocked` detection across agents the desk did not start.
- **Kun** ([KunAgent/Kun](https://github.com/KunAgent/Kun), ★6,292) is the best-finished
  rival and still loses: `grep -ic acp` over its 7,716 files returns **0** — it
  replaces the agent layer instead of hosting ours. Its PolyForm Noncommercial
  licence permits internal use only after a **written** confirmation from the
  author, and is a hard stop if the desk ever faces customers.

---

## 6. Phased plan

| Phase | Window | Ships |
|---|---|---|
| **1 — P1 + P2 end to end on Hafidh's beta** | 2 weeks (→ **2026-09-21**) | Fork codeg v0.30.4 + **rebrand**; pieces **1, 2, 3, 4, 5, 6, 7** (≈17 days plus 2 for rebrand, so trim #7 to a list if it slips). **P1**: tester report → evidence-bound GitHub issue → triage → fix task in a worktree → review → "fixed in build N" draft. **P2**: Resend inbox → ticket → agent draft → human approve → reply. **Telegram channel on**, so approvals reach a phone. Terminal comes free with the fork; agents are pi. Unsigned dev build, one product |
| **2 — P3 + P4** | 6 weeks (→ **2026-10-19**) | **P3** release notes → tester announcement → store copy, each approved separately. **P4** social publish-only: LinkedIn + Instagram, scheduled, every post `propose`; borrow Postiz's per-channel post model. Automations on (weekly summary, daily triage). Sentry MCP; Play + ASC reviews into the same queue. TikTok audit **submitted** (unaudited apps are forced to `SELF_ONLY`). Consent/privacy and store presence tracked as work in the desk |
| **3 — More products, less babysitting** | 3 months (→ **2026-12-07**) | `codeg-server`/Docker on the apuri substrate so work survives the desktop closing; mobile clients + Slack; multi-product (UrsinHub, Uramax, Klerit) with per-product MCP sets; Twenty-style custom objects if People/Customers need to grow; narrow `act_low_risk` grants earned from audit-log evidence; optional Herdr status feed; signing only if the desk leaves the team's machines |

**Phase 1 writes no shell, no task engine, no terminal, no chat plumbing and no
mobile client.** It writes an audit log, a ticket store, an email panel, two
adapters and one agent extension — on top of a fork that already runs.

---

## 7. Recommendation

Fork codeg at `v0.30.4` and grow the processes inside it, rather than starting a
management app and embedding codeg's pieces. The reason is concrete rather than
aesthetic: the hard, easy-to-get-wrong parts of P1–P4 are already written and
tested there — a CAS-guarded approval state that cannot reach `done` unseen, an
ACP-generic agent layer that already lists pi, worktree isolation with
git-verified merges, a headless automation engine, an IM control plane that
relays real ACP permission options to a phone, an OS-keyring store, and a
server/Docker deployment. What is missing is small, specific, and
**available to borrow rather than write**: our own intromail supplies the gate,
the destructive floor and the audit log — no third-party terms, a port not a
design; Chatwoot, **MIT outside `enterprise/`**, supplies the RFC 5322 threading
strategy chain and the conversation/contact/note schema; codeg itself supplies
the thread UI and the board components; intromail's Resend REST and GitHub App
clients, the MCP Python SDK and pi's extension examples supply the rest.
**Original estimate: ≈19 glue days, pending integration re-estimation.** Plane and
Twenty are AGPL-3.0, so take their vocabulary and object model as *design* and
keep their source out of an Apache-2.0 tree. Starting fresh would mean rebuilding
everything the fork already runs to reach the small part that is missing. Ship P1 and P2
by **2026-09-21**; the biggest risk to the **2026-11-02** launch remains
untouched by any of this — **consent/privacy and store presence**, which no
tooling can shorten.

## 8. Owner amendment — business workspace, 2026-09-08

The owner expands the product to marketing, channel management, pulling and managing ads, website work and feedback, backed by engineering features. Shared task management serves humans and agents in both business and engineering. Business outcomes lead the main experience; engineering is supporting capability. [Business workspace scope](docs/BUSINESS-WORKSPACE.md) records current gaps, shared-work requirements and staged delivery. This supersedes the narrow engineering-first presentation; previous phase estimates must be revisited. Existing borrowing, audit, human review and local-test boundaries remain.
