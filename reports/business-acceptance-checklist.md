# Shared business workspace — root acceptance checklist

Active Increment A; implementation is not accepted yet. Root derives this from
BUSINESS-IMPLEMENTATION and the published task/UI contracts. Backend owners
supply fixtures and exact-source gates; root independently exercises integration.
No live provider/model work is needed for these checks.

| ID | Observable acceptance |
| --- | --- |
| BW-1 | Authenticated operator can initialize/manage the organization; anonymous/member/viewer cannot bootstrap or mint elevated identities. |
| BW-2 | Two independent member sessions see the same permitted shared task and actual human/agent names; caller IDs/role/org fields cannot impersonate another principal. |
| BW-3 | Member credentials fail legacy health/config/terminal/agent endpoints and WebSocket entry. Visiting business UI makes no legacy privileged data requests. |
| BW-4 | Viewer sees permitted tasks but cannot create/edit/assign/progress/comment/review/manage; backend rejects forged requests as well as UI hiding actions. |
| BW-5 | Role/domain limits apply to list/detail/activity/directory, create and every changed destination/principal reference; foreign/inactive records do not leak. |
| BW-6 | Revoked session/member/role takes effect on the next operation and is revalidated inside mutation boundaries; no stale in-flight assignment grants privilege. |
| BW-7 | Human can create and complete permitted work with no git folder, terminal, chat or agent. Owner/assignee/reviewer identities remain distinct and visible. |
| BW-8 | Concurrent edits use expected revisions; one stale edit is rejected, its local text retained, and refresh/resubmit is explicit. Activity/state commit or roll back together. |
| BW-9 | An authorized agent receives only assigned context and can provide progress/deliverable; it cannot approve itself, reach done via progress, alter roles, or obtain provider credentials. |
| BW-10 | Reassignment, cancellation, revoked delegator or old execution generation fences agent contributions; link-to-run does not silently launch a model. |
| BW-11 | Dates have explicit, tested all-day/time semantics; name/date/long-text rendering stays correct across timezone/RTL and narrow widths. |
| BW-12 | My work/shared/domain/status filters show actual authorized data and bounded pagination; no fabricated counts or nonexistent integration controls. |
| BW-13 | Keyboard/focus/labels/errors/contrast and reduced motion pass on 390/768/1280 light/dark; detail and forms contain scrolling without clipped actions. |
| BW-14 | Unsaved edits survive viewport/locale changes; changing identity/organization clears private data and ignores late responses. |
| BW-15 | Cold member route, failed sign-in, reconnect, revocation and permission/conflict states provide useful recovery without exposing raw credentials/server errors. |
| BW-16 | Native owner mode/shared-server member mode are accurately distinguished; browser verification is not claimed as OS isolation or multi-desktop native certification. |
| BW-17 | Existing Ops email/intake/review/no-resend and engineering task selectors pass after integration; new principal auth does not weaken original boundaries. |
| BW-18 | Final Design Studio audit records actual visual before/after and verified findings; final app/export hashes correspond to accepted product source. |

Evidence must record source commit, fixture identity, browser sessions, request
outcomes and actual limitations. Synthetic tokens only; never capture credential
values from global Settings. Permission policy may evolve in the identity
contract; changes must be explicit and this checklist reconciled before testing.
