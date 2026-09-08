# Increment B business intake contract — proposed, not implemented

Owner: tickets; docs-only branch `docs/business-intake-contract`, based on accepted
main `c7f7366fef2d7945cea18d4da1b7594fa3026e1c`. This prepares the bounded
meeting/feedback increment in `docs/BUSINESS-IMPLEMENTATION.md`. Root reviews
this contract before implementation ownership. Existing task/identity semantics
remain authoritative; no provider, configuration or model action is authorized
by this document.

## Fixed boundaries

- Fireflies is a read-only, explicit pull adapter. An imported source is not a
  task, assignment, consent to publish, or instruction to run an agent. No
  meeting bot, audio upload, provider mutation, webhook deployment or model call.
- Reuse the existing Rust service/database and protected business transport.
  Server-derived `business_identity::Principal` is the only caller authority;
  no caller actor, role, organization claim, credential reference or raw query.
- Source access is separate from task-domain access. Imported meetings and
  candidate passages default private to their explicit authorized human source
  audience. Participants, matching email/name, Fireflies privacy labels and a
  task assignment do not mint local source grants.
- Human acceptance publishes only the exact reviewed task title/notes to the
  selected domain. Accepted tasks remain visible to that domain per Increment A;
  transcript, attendee list and restricted evidence do not enter its notes,
  activity, agent context, search or notifications automatically.
- Candidate edit/accept/link/discard requires current source permission and
  current destination task permission, revalidated in the same writer transaction
  as CAS, task/source linkage, decision and immutable audit. Reimport is durable
  and idempotent; it must not overwrite human-edited or accepted work.
- Existing email/Hafidh storage remains under its accepted operator/account
  boundary. An ordinary business member cannot use a business source ID to
  traverse legacy Ops, credentials, private notes or evidence proofs.

## Accepted core seam that must be preserved

At the base above, `business_identity/mod.rs` exposes `authorize`,
`active_reference` and `begin_write`; private Principal rereads current member
and original credential lineage. `business_tasks/store.rs:354–420` creates a
human-only task, checks destination Create/Assign and active references, inserts
task/activity and commits its own transaction. `business_tasks/policy.rs` defines
task capabilities; sources confer no bypass.

Implementation will need a narrow task-owner extraction of that existing create
logic into a crate-only transaction helper, retaining the public create wrapper.
Candidate acceptance can then use one writer transaction. Such a helper does not
exist at this checkpoint. Calling the HTTP create endpoint followed by a second
link write, or copying task validation/INSERT SQL into intake, is not acceptable.
Link-to-existing similarly needs task-owned revision/capability validation and a
typed source-link activity seam; it is not engineering `link-execution`.

## Source grounding in progress

Independently reread official Fireflies adapter queries, transport and MIT licence
through `gh api` at `fbd24607bc784a2294ce402426aefe2cb8c00f50`. Its list wires
date bounds/limit/skip; detail selects summary/readiness and sentence indices,
times and sharing metadata; HTTP-200 GraphQL errors are rejected. Intended reuse
is those narrow query/error patterns, never the n8n runtime or mutation queries.
The exact ledger, closed operation DTOs, source/revision/claim lifecycle and
synthetic acceptance matrix follow after the remaining pinned source reads.

This checkpoint defines the boundary only. It is not a passing ingestion,
permission, pagination, UI or provider integration claim.
