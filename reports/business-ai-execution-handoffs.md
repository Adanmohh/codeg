# Internal execution handoff findings

Open at frozen product `3773a027a8d580bd9bac1808efdb718ae6f9e135`.
Approvals identified both findings; root read the relevant session-store,
receipt and file-stage source and confirmed the missing checks. These helpers
are internal and unexposed at this checkpoint. No executed reproduction, remote
exploit, provider launch or live runtime acceptance is claimed.

## Admission and receipt target mismatch

`session_store::Admission` exposes its fields to sibling execution code.
An internal caller can change a valid admission A's operation ID to another
pending operation B owned by the same principal. `complete_launch` validates A's
session, generation and admission ID, but `receipts::complete` validates only B's
own current scope and pending/uncertain status. It does not match B's recorded
task/session/generation to A before committing the result. This can associate A's
completion with B's receipt while leaving A's receipt pending.

Required correction: encapsulate the admission capability and enforce exact
receipt kind/task/session/generation/resource binding at completion. Rejected
mismatches must leave both receipts, engine linkage and session state unchanged.
Future bounded tests should include two real same-principal pending admissions,
swapped operation identity, stale generation/authority and a correctly bound
positive control. Internal encapsulation does not replace persisted checks.

## Retained-file retry omits durability confirmation

`files::stage` first syncs copied bytes, changes the object mode to0400, syncs
again, then syncs the containing directory. If either final sync fails, the
method returns an error but a0400 object can remain. The existing-object recovery
branch checks mode, content hash and size and immediately returns `Retained`.
It does not retry or confirm the failed metadata/directory synchronization.

Required correction: successful recovery must establish the same durability
conditions as initial staging, while retaining exact object bytes and avoiding
overwrite of partial/corrupt objects. Future deterministic injected-failure tests
must cover post-mode file sync and directory sync failure, repeated failure on
retry, and successful retry only after synchronization succeeds. Hash equality
alone does not establish durability. No power-loss experiment has been performed.

Tickets owns both fixes and focused regressions; approvals owns independent
review. Both findings block acceptance of the affected consumers. The separate
pagination finding remains open. The prior177 schema/DTO verdict is unchanged.
Disk-capacity compile hold remains in effect; root only wrote this report and
coordination state, with no product, target, browser or fixture changes.
