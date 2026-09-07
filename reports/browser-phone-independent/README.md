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
