# Internal session pagination finding

Open P2 at frozen product `3773a027a8d580bd9bac1808efdb718ae6f9e135`.
The independent reviewer reported it; root confirmed the source path through
`session_store::list` and the complete `scope` module. The immutable commit was
verified using `gh api`. This is source analysis, not an executed reproduction.

`session_store.rs:142` validates an incoming cursor through `scope::session`,
which requires the session's authority/profile/task generations to remain current
and rejects revoked/closed rows. At lines150–158, listing selects its next cursor
from raw SQL rows before dropping entries with changed authority.

With limit1 and a revoked row first in descending UUID order, followed by an
eligible row, page1 returns no items and the revoked row's ID as next cursor.
Page2 rejects that cursor with AuthorityChanged. The eligible later row is
unreachable through that returned continuation. Revocation between page requests
also needs an explicit regression. UUID order does not represent creation time.

Tickets owns the correction; approvals owns independent re-review. Preserve
current principal, organization, member and task authorization, and validate
every returned item. A cursor must not grant access to its referenced session.
Required bounded cases cover leading/interleaved stale rows, cursor revocation
between pages, traversal of remaining eligible rows, and foreign task/org/member
cursor rejection. The implementation choice remains with the product owner.

No product code, test execution, fixture, browser or build output was changed by
root. Both Rust build windows remain on hold for disk capacity coordination.
The prior11-pass schema/DTO verdict remains pinned to177d0f3e; it did not cover
this later list consumer. This finding blocks acceptance of that consumer until
the committed correction is independently reviewed and tested.
