# Independent locale regression check

Root actual Playwright CLI on released4326, final exported selected-row fix,
390×844 and1280px, public synthetic operator only. No send/save action.

Separate Settings tab changed English→Arabic→English. Unsaved reply
“Root unsaved locale sentinel” and private note “Root private note locale sentinel”
remained in the real editor. Reply also survived desktop→mobile. Email input
has dir=ltr, mobile Back SVG rotates180deg, width390 equals scrollWidth390.
No sentinel in localStorage; provider requests remained4. Settings restoredEnglish.

The first screenshot caught the responsive drawer open; Escape closed it,
then root recaptured and inspected the unobscured Arabic editor. A pending
Private note click completed once that drawer closed. That first capture is
not used as editor evidence. Fixture Git HEAD/stream errors remain expected;
no zero-console claim. This checks English Ops content in Arabic layout,
not a complete Arabic Ops translation.

[Reply](arabic-reply-preserved.png), [note](arabic-note-preserved.png).
Independent22 component tests separately cover complete review fields and
failed/delayed/superseded locale loads. Worker full-review browser evidence
is separately attributed. Final integrated design/native acceptance remains.
