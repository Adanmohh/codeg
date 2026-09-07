# P2 — language changes discard unsaved Ops edits

Independent Playwright CLI reproduction, accepted PR7 product at0e375c97,
2026-09-08. Real protected local fixture4320, root session ops-ui-independent.

1. Arabic mode,390×844, Ops Inbox → Reader / A real local thread.
2. Fill Reply message with `Unsaved locale preservation sentinel`; do not save.
3. In the separate Settings tab, System → Language → English.
4. Return to workspace: lang=en; Ops region and Reply message count both0,
   and the chat landing page replaced Ops. No discard confirmation appeared.
5. Open sidebar → Ops → same Reader thread. Reply value is
   `Original approved body`; the sentinel is lost.

Screenshot `reports/browser-step2/locale-draft-lost.png`; detailed local
snapshot `/tmp/ops-locale-edit-after.yml`. No provider send/DB draft write.
Root restored the earlier connectivity sentinel before this separate case.

Root read complete `src/components/i18n-provider.tsx`: its appReady condition
includes appLocale===messagesLocale and renders AppBootLoading instead of all
children during asynchronous locale-message loading. That unmounts the
workspace and its memory provider. This is a concrete source explanation for
the observed loss. The responsive-remount fix remains valid for breakpoints.

Sent to approvals worker for required final Design correction after Telegram
handoff: preserve already-mounted state through subsequent locale loads, retain
initial boot behavior/backend isolation, add actual reply/note/review locale
regressions and repeat CLI across settings tabs. No private persistent storage
or root product edit. BC-3/BC-9 apply. Finding remains open.
