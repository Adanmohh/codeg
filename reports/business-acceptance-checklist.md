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

All three Increment A product PRs are reviewed/merged. The first actual native
package passes core workflows but has N2 window-chrome correction pending.
This table records scoped evidence, not universal or live-provider certification.

| IDs | Accepted evidence | Remaining acceptance |
| --- | --- | --- |
| BW-1,5,6 | Exact identity/task reviews cover backend principals, transaction/revocation/reference guards. Actual native operator bootstraps isolated organization; shared viewer revocation clears the native session. | N2 does not change this backend contract. |
| BW-2,7 | Root and independent real personal sessions share named tasks; native local human creates/progresses/reviews Done4 without an engine. | Final refreshed artifact correlation. |
| BW-3,4 | Five cold routes make no API/WS calls; real backend viewer403/404 and credential legacy/WS denials. Native shared viewer has only read controls and no engineering link. | Native correction must preserve cold web and member boundaries. |
| BW-8 | Real409/draft retention/explicit adoption independently rechecked on exacte72 export; base/current revision clarity fixed. | No broad repeat required for native-only chrome. |
| BW-9,10 | Exact R1 review:13 independent ownership/bridge/migration/HTTP tests; accepted real Pi/process evidence. Bundled MCP90f9fd8e passes275 bounded synthetic transport assertions. | Correlate final companion digest; rerun only if artifact changes. No live inference. |
| BW-11 | Calendar2028-02-29 survives separate sessions; EN/AR12-case matrix no overflow; native shared detail renders2026-10-01. | Final chrome/layout recheck. |
| BW-12 | Independent actual board/filter/empty/keyboard scrolling and abort/retry; native shared list reads real authorized data. | Preserve business subtree through native wrapper. |
| BW-13 | Independent full12-case baseline; six final search cases close contrast/focus findings. Measured motion106frames and settled focus pass. | N2 native chrome/drag; final targeted browser recheck. |
| BW-14,15 | Six drafts survive other-tab locale and viewport changes. Revoked401 clears private state; actual native viewer revoke clears detail and returns useful sign-in. | N2 must not introduce subtree remounts. |
| BW-16 | Actual macOS WebKit local owner bootstrap/human Done4; separate shared viewer/real tasks/revocation; explicit engineering route and restartbusiness entry pass. | N2 corrected titlebar/native controls plus final package recheck. No Windows/Linux/OS isolation certification. |
| BW-17 | Tickets312 backend regressions at294fb634; root6194 frontend tests atb97e6bd9 plus39 affected tests at e72; accepted worker both runtime/Clippy gates. | Only affected N2 checks. |
| BW-18 | Independentfa8b571a and root Design Studio synthesis:zero open browser findings; exact first package all3 binaries/profile and1016 web hashes match. | N2 correction/review/normal package refresh/native evidence and final summary. |

Evidence: [root integration](business-final-root/README.md),
[root Design Studio synthesis](business-final-root/design-review.md),
[independent UI review](review-business-workspace.md),
[native package and workflows](business-native-root/README.md),
[bundled companion](business-bundled-companion.md),
[backend regressions](business-integration-regressions.md).
