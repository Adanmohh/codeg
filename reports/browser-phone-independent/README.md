# Independent phone review checks

In progress,2026-09-08. Headed Playwright CLI `ops-phone-independent`,
390×844, actual protected router at4323 and local synthetic providers.
Worker owns active mutation checks; root has made no approval/send/config edit.

Stale fixture link redirects while signed out to login preserving only its
opaque opsNotice. Invalid fixture token shows the expected error. Actual DOM
now has associated visible Access Token label, aria-invalid=true, describedBy
login-error and role=alert. Width390 equals document scrollWidth390. This
provisionally fixes loop1 login accessibility finding, pending PR10 acceptance.

Valid synthetic login returns to the exact stale link, showing no current review
and no approve/deny action. Pending link displays complete From/To/Cc/Bcc,
subject/body and expandable threading headers in the existing ReviewCard.
Both decision controls measure44px high; no horizontal overflow. Root inspected
the mobile screenshot. No actual remote phone connectivity is claimed.

The invalid login produces its expected401 browser console entry. Final full
phone decision and BC14 database-only receipt completion remain pending.

## Released fixture: phone decision and BC14 passed

After worker closed its browser and released the fixture, counters were email1,
Telegram4. Root reloaded pending proposal1, edited Bcc to the public synthetic
phone-review-copy@example.com and body to “Reviewed from the protected phone
page. Synthetic fixture only.”, inspected complete thread headers and approved
through the real phone ReviewCard. State became Sent · provider accepted with
receipt; email count became2 and Telegram stayed4. Opening original thread
showed one recorded outgoing copy of the edited body.

Root then opened workspace → Sidebar/Ops → Approvals → Reply review2.
Its real stored receipt showed recording pending. Finish recording receipt
changed it to sent, removed the finish button and created exactly one outgoing
“Original approved body” in Conversation messages. Email count stayed2 and
Telegram4: no second provider call for this receipt. Width390/scroll390.
Screenshots record before and after; root inspected the completed receipt image.

The inherited no-delivery-receipt terminal sentence remains a known copy issue
for the final Design fix; it does not override the explicit provider receipt.
Open Ops workspace currently lands in the generic workspace; Sidebar/Ops is
needed to open the actual Ops view. Existing fixture git-head/state-stream404s
and inherited form advisories are recorded, not a zero-console claim.
