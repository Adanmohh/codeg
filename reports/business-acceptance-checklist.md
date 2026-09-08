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

## Integration evidence checkpoint — 2026-09-08

This maps completed evidence and remaining checks; it is not final Increment A
acceptance. Exact source/commands/limits remain in the linked reports.

| IDs | Evidence now available | Remaining acceptance |
| --- | --- | --- |
| BW-1,5,6 | Accepted [identity review](review-business-identity.md) and [task review](review-business-tasks.md): backend-derived principals, transaction/revocation/reference guards. | Final native invocation coverage is separate. |
| BW-2,7 | [Root browser integration](business-final-root/README.md): distinct manager and viewer sessions see the same task, human-only creation/progress/review, named audit actor. | Final export identity. |
| BW-3,4 | Root five cold routes make no API calls; viewer has no editing/engineering controls. Independent identity/router and task tests cover backend denials. | Final native shared-member session; reviewer teardown/revocation checks. |
| BW-8 | Root actual409/draft retention/adoption and13 correction workflow tests; independent source review covers atomic CAS/audit. | Final corrected export source correlation. |
| BW-9,10 | Exact R1 review at1ba73e3c:13 independent ownership/bridge/migration/HTTP tests and scoped lifecycle proof. Accepted worker real Pi/process evidence remains separately attributed. | Final packaged companion protocol; no live inference is claimed. |
| BW-11 | Root preserves calendar2028-02-29 across separate sessions; independent12-frame EN/AR matrix has no page overflow. | Reviewer final rendering synthesis and native view. |
| BW-12 | Authorized actual lists/search and worker component/page tests. | Independent board/filter/empty/error checks in progress. |
| BW-13 | Independent12-frame matrix and actual keyboard human review pass. | Two verified search P2s, final focus/motion checks and rechecks. |
| BW-14,15 | Root viewport draft retention/cold recovery; reviewer actual other-tab locale preserves six edited fields. | Independent identity/revocation/disconnect teardown and recovery. |
| BW-16 | [Native source review](business-native-review.md):22 fixed commands match registration; fresh-entry N1 resolved at3a189d18. | Actual packaged WebKit owner/shared-member controls. |
| BW-17 | [Accepted backend regressions](business-integration-regressions.md):312 passed,5 manual fixtures ignored at294fb634. Root full frontend6194 passed atb97e6bd9. | Only affected later correction checks, not an unnecessary broad rerun. |
| BW-18 | Worker/reviewer Design Studio loops and root inspected before/after conflict evidence. | Final specialist synthesis, fixes closed, package/export hashes and native startup. |
