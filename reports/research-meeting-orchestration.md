# Meeting sources → shared human–agent work

Research only, 2026-09-08. **Recommend a read-only Fireflies connector that
produces candidate work for human review, backed by durable ingestion jobs.**
A transcript becoming available is neither an assignment nor permission to
launch an agent. Webhooks can shorten discovery delay; reconciliation pulls
remain necessary. The reviewed components are borrowing candidates, not a
verified production integration.

Worktree branch remains `review/visual-refresh`, HEAD
`57de9ed1def7e8f42f4d70e1d096a829ef828980`. Product implementation and visual
review remain paused. This report changes no product, dependency or fixture.

**Verified source evidence**

| Source and immutable pin | What the actual contract/source establishes | Production qualification |
| --- | --- | --- |
| Official [Fireflies Node SDK](https://github.com/firefliesai/fireflies-node-sdk/tree/76d7983a1b3592a1662e77157f1ff1d85216acb4), package 1.1.3 | Transcript/list/summary GraphQL reads exist. The resolved commit is dated 2026-06-10 and updates Axios. | Do not adopt its convenience methods unchanged. Specific defects below. Package/README declare MIT, but this tree has no LICENSE file despite the package listing one. Obtain the original notice before copying SDK implementation. |
| Official [Fireflies adapter](https://github.com/firefliesai/n8n-nodes-fireflies/tree/fbd24607bc784a2294ce402426aefe2cb8c00f50), package 2.2.2 | List query wires date bounds, limit/skip and scope filters; transport rejects GraphQL `errors` even when HTTP succeeds. Transcript query includes summary readiness, sentence positions, privacy/sharing metadata. Commit dated 2026-07-16; repository is not archived. | Best narrow borrowing candidate: selected queries and error-envelope handling. No durable sync, permission propagation or production reliability is established by this source. Its node depends on `n8n-workflow`; do not bring in the runtime just to reuse the adapter. |
| Existing [Ops delivery](https://github.com/Adanmohh/codeg/blob/57de9ed1def7e8f42f4d70e1d096a829ef828980/src-tauri/src/ops/delivery.rs) and [Telegram scan](https://github.com/Adanmohh/codeg/blob/57de9ed1def7e8f42f4d70e1d096a829ef828980/src-tauri/src/ops_telegram/mod.rs), Apache-2.0 | Transactional claim/receipt patterns, immutable approved payload binding, bounded scan and distinct retryable pre-send versus ambiguous-send states already exist locally. | Reuse these principles and lifecycle seams. They are purpose-built email/notification state machines, not an implemented general ingestion queue. Do not repurpose their tables or replay stored approvals. |

The SDK's [actual list query, lines 199–233](https://github.com/firefliesai/fireflies-node-sdk/blob/76d7983a1b3592a1662e77157f1ff1d85216acb4/src/fireflies.ts#L199)
does **not** pass `fromDate`, `toDate` or `organizer_email`, although its public
parameter types offer them. Its transport returns `response.data.data` without
checking GraphQL errors and sets no explicit timeout. The
[multi-user helper](https://github.com/firefliesai/fireflies-node-sdk/blob/76d7983a1b3592a1662e77157f1ff1d85216acb4/src/helper.ts)
uses a process-local set, assigns overlapping meetings to the first API key,
and uses API keys in output filenames; a caller also logs the full key on an
error path. Those are unsuitable provenance, secret-handling and durable-dedupe
patterns.

The SDK types declare `action_items: string[]`. The official service
[Summary schema](https://docs.fireflies.ai/schema/summary) declares **String**.
Treat it as free text containing suggestions, not structured tasks with stable
IDs, verified assignees or due dates. Runtime validation must follow the service
contract, not the SDK's TypeScript assertion.

The official [list contract](https://docs.fireflies.ai/graphql-api/query/transcripts)
uses `limit` (maximum 50), `skip`, and creation-time `fromDate`/`toDate` filters.
The reviewed list contract provides no opaque cursor, updated-since change feed
or stable snapshot/order guarantee. Offset is a resumable scan position, not
proof of a complete synchronization.

Current [V2 webhook documentation](https://docs.fireflies.ai/graphql-api/webhooks-v2)
separates `meeting.transcribed` from `meeting.summarized`. Payloads include
meeting ID, event and millisecond timestamp. A configured signing secret gives
an HMAC-SHA256 signature over the raw body. Delivery requires a 2xx response
within ten seconds. The page does not establish retry cadence, retention,
unique delivery IDs, ordering or replay guarantees. It describes owner meetings;
do not equate subscriptions with every meeting the connection can read.
[V1](https://docs.fireflies.ai/graphql-api/webhooks) has a transcription-complete
event, which should not be treated as summary-ready.

**Five recommendations**

1. **Start with bounded, restartable pulls; add signed webhook hints later.**
   Keep connection-scoped scan state: frozen date window, page position,
   completed-window marker and retry time. In one database transaction, upsert
   discovered IDs and enqueue detail fetches before advancing that page marker.
   Revisit overlapping windows and perform bounded older-record reconciliation;
   creation dates alone miss later edits or newly granted access to old meetings.
   Deduplicate by local tenant/connection plus provider transcript ID, not title,
   calendar occurrence or API key. Preserve separate access grants if the same
   transcript is visible through multiple connections. Offset pagination can
   still shift during scans: report coverage and lag honestly, and never promise
   lossless sync without a tested provider contract. A webhook should authenticate,
   persist a deduplicated ingestion hint and acknowledge quickly; it must not
   parse tasks or call agents inline. A bounded replay guard plus idempotent
   fetching is needed because signature verification alone does not stop replay.

2. **Make readiness and provenance explicit before extraction.**
   The [readiness schema](https://docs.fireflies.ai/schema/meeting-info) distinguishes
   `processing`, `processed`, `failed` and `skipped`. Persist each observation;
   retry processing within a bounded policy, expose failed/skipped states, and
   distinguish processed-with-no-items from unavailable summary. Re-read the
   source after a summary event. Store a minimal source snapshot with transcript
   ID/link, retrieval time, content hash, summary text and available sentence
   indices/timestamps; the official adapter supplies those query fields.
   A local hash is not a provider revision. Changes create a new candidate
   revision and invalidate stale review; they must not overwrite accepted work.
   Suggestions lacking a supporting passage remain explicitly unsubstantiated.
   Do not parse names or dates in free text into authoritative assignments.

3. **Propagate access to derived work, not just to the transcript cache.**
   Fireflies exposes participants, workspace participants, channels and external
   sharing metadata; these are not a complete local authorization policy.
   [Transcript schema](https://docs.fireflies.ai/schema/transcript),
   [pinned adapter projection](https://github.com/firefliesai/n8n-nodes-fireflies/blob/fbd24607bc784a2294ce402426aefe2cb8c00f50/nodes/Fireflies/helpers/queries.ts#L120).
   Default new imports and candidates to the connecting operator's permitted
   scope until an explicit sharing rule is established. Require the intersection
   of local task access and current source access for excerpts, search, exports,
   notifications and agent context. Recheck before delegation; on revoked or
   uncertain access, suspend new disclosure and reconcile/redact derived copies
   according to the agreed retention policy. Preserve known source sharing expiry.
   Keep credentials in backend credential storage, never in agent environments,
   job payloads, logs or filenames. Importing previously recorded meetings does
   not itself authorize recording, joining a bot, sharing with another person
   or sending content to another model provider. Record the operator's permitted
   use and audience; do not infer consent from a participant list. This is a
   product boundary recommendation, not a legal-compliance certification.

4. **Keep durable jobs separate from both candidates and agent runs.**
   Ingestion jobs need durable unique keys, leased claims with fencing,
   deadlines, bounded backoff/jitter, retry budgets and an actionable failed
   queue. A logical read remains retryable even though GraphQL uses HTTP POST;
   classify the operation and GraphQL envelope, not just the HTTP method/status.
   Transactionally persist source changes plus downstream job/outbox entries;
   only acknowledge completion after commit. Multiple workers and crash recovery
   must converge on the same source/candidate records. Reuse the local Ops claim
   and audit discipline rather than introducing a large workflow platform now.
   Later external effects require separately reviewed payloads, stable operation
   keys and receipts; an ambiguous remote result needs reconciliation, not a new
   key and blind retry. No connector library proves exactly-once external effects.

5. **Present candidate work to a human before assignment or execution.**
   The review should show the proposed outcome, source passage, suggested owner
   and date with uncertainty, destination/access scope and possible duplicates.
   Allow accept, edit, discard or link to existing work. An ambiguous “Alex” or
   “we should” stays unassigned. Keep an accountable human distinct from an agent
   asked to help. Candidate extraction is not business commitment; job completion
   is not task completion; successful agent execution still needs acceptance.
   The existing [work_task model](https://github.com/Adanmohh/codeg/blob/57de9ed1def7e8f42f4d70e1d096a829ef828980/src-tauri/src/db/entities/work_task.rs)
   is explicitly folder/worktree/agent oriented, with CAS/run generation and
   review-before-done invariants. Link approved delegated execution to it; do
   not force every human follow-up or ingestion retry into that engine. A shared
   business-work model and its ownership rules remain a root product decision,
   not an implemented capability. Treat transcript/summary instructions as
   untrusted source content, never as tool authorization.

**Exact borrowing ledger and research limits**

All GitHub reads used `gh api` after installed help/local-source reads. No
package was installed, no provider account was accessed, and no code was ported.
If the official adapter is later ported, preserve its complete
[LICENSE.md](https://github.com/firefliesai/n8n-nodes-fireflies/blob/fbd24607bc784a2294ce402426aefe2cb8c00f50/LICENSE.md)
and original `Copyright 2022 n8n` notice, plus Fireflies source attribution in
NOTICE. That MIT grant applies to the reviewed adapter; this report makes no
licence claim about the n8n runtime. Existing root NOTICE is unchanged. No AGPL,
enterprise or restricted source is recommended for copying.

| Pinned repository | Exact files read for conclusions and blob SHAs |
| --- | --- |
| SDK `76d7983a1b3592a1662e77157f1ff1d85216acb4` | `src/fireflies.ts` `84cd9902c7c47bc2f618cedc92bcf39d922bb4b9`; `src/types.ts` `0c8c835beb093ba56817cf9ed14b3540cc040fb6`; `src/helper.ts` `e0738719c0bd7e94ecf7f9684fda303eee69c0d6`; `package.json` `bebbcfe5c0729a2ae3cb7d8d053f17a72d4040ec`; `README.md` `e5b0e3a5f286dc892a92bd16e4ef310e3a2f8864`. |
| Adapter `fbd24607bc784a2294ce402426aefe2cb8c00f50` | `nodes/Fireflies/helpers/queries.ts` `c775b8bbe6c0bbb16f8f7e8a60467c248ad93859` (list/summary/transcript sections); `nodes/Fireflies/operations/transcript/getTranscriptsList.ts` `b9479e5f618de87ff820ab13dbbbf839f4150f23`; `nodes/Fireflies/transport/index.ts` `0b60c1c9e9d61f4583298cd2c0c464e9864861f5`; `nodes/Fireflies/helpers/errors.ts` `7141e7c99969c6fd32f2e96fa7b2a561037a43de`; `package.json` `fe1e413862f5d5dddf9c0ce8153bfd4d0a27d56c`; `LICENSE.md` `1e4b3a6e245384b89f24f2aef5e3f8e7fa1f4d23`. |

Official hosted-document fallback was necessary: the site's source link points
to `firefliesai/public-api-ff`, which returned HTTP 404 through `gh api`.
The public `firefliesai/docs-fireflies` repository is archived; its inspected
`d85d6b8067350a81911dfeaabb7ae3a00c92bf68` tree/source lacks the current contracts.
Hosted references above were read on 2026-09-08 and are **not immutable**.
They supplement, and do not silently upgrade, the supplied SDK pin or founding
borrowed-source versions. No hosted documentation code sample was copied.

`gh api` source/metadata reads succeeded except the disclosed 404. Offline
code-context guidance succeeded; it supplied general deterministic-first advice,
not Fireflies coverage. Local dependency inventory contains no Fireflies package.
The prior installed-doc corpus check found no `approvals.db`; no corpus or model
was installed. No tests, build, live introspection, webhook delivery, access
revocation, pagination-under-concurrency or summary-generation timings were run.
Recent commits and permissive licences are maintenance signals, not evidence of
production readiness. No MCP server or generic runner was selected on source
presence alone.

Before a later implementation could be accepted, synthetic contract/crash tests
must cover HTTP-200 GraphQL errors, missing versus empty summaries, duplicate and
out-of-order events, offset shifts, restart after transaction commit, concurrent
claims, source edits/revocation, ambiguous assignees and approval races. Actual
provider pagination, account visibility and delivery behavior still need a
separately authorized staging validation. **Research ends here; implementation
and the queued visual assignment remain paused.**
