# Business workspace measured review

Audited against unchanged `docs/design/BRIEF.html` (scan, schema v1), with the
business scope and BC-1–16 in `brief-checklist.md`. Design Studio methods:
aesthetic-judge, a11y-auditor, flow-validator, and limited motion review, applied
sequentially by this worker. No additional agents or paid inference. This is a
worker assessment for root's independent review, not an independent certification.
Baseline broad measurements use33b9cbcb. Final review corrections use e72cc44b
on the separate4346 export; the targeted additions below do not replace or inflate
the earlier whole-page sample count.

The visual result is a readable shared agenda: branded navigation, direct task
creation, distinct accountable human and executor, status text plus shape, focused
editing/review, and useful personal/viewer/revoked states. Engineering remains a
subordinate authorized destination. Real task content supplies the visual focus;
no revenue, ads, channel connectors or invented business metrics appear.

Scores (0–10): states8, feedback8, accessibility8, responsive8, visual/tokens8.
Design-weighted verdict: a coherent working application with a clear responsibility
trail; deliberately restrained rather than an editorial/award-style showcase.

## Measured findings and corrections

| Check | Before | Recheck |
| --- | --- | --- |
| BC-7 mobile filters |32.2px values after flex shrink |171×44px controls,16px text at390; twelve list/board frames cover390/768/1280 light/dark without page overflow. |
| BC-9 long content |3974px content in358px dialog |358/358, wrapping within390px page. |
| BC-13 reduced motion |Dialog `enter` animation still0.1s |Business dialog animation `none`; Action transition property `none`. Inherited overlay retains a100ms opacity fade. |
| BC-16 destructive text |Active light Cancel task4.07:1 |Light7.11 normal/focus,6.08 hover; dark7.99 normal/focus,6.59 hover. |
| BC-12 focus contrast |Destructive focus border1.83 light/1.98 dark |Opaque inherited red focus border/ring7.11 light/7.99 dark against its adjacent background. |
| BC-12 modal dismissal |Settled activeElement BODY after controlled dialog close |Scoped opener/work-area restoration; three focused regressions. Actual archive-focus and final-style-focus checks pass, including visible keyboard ring. |
| BC-10 conflict recovery clarity |After real409, old status/revision unlabelled; current comparison omitted both |Named draft base and current region show distinct status/revision before explicit adoption. Real worker409 Review3 versus To do4, preserved draft, locked save, adopt/save5. EN/AR390 light/dark358/358 modal. |
| BC-16 search placeholder |Enabled #737373 on composed #f7f7f7 =4.43:1 |Scoped inherited foreground/80 gives10.78 light; retained dark token7.07.12/12 EN/AR ×390/768/1280 × light/dark samples pass. |
| BC-12 search Tab |A1×1 sr-only submit with clip-path inset(50%), UA auto1px outline and no shadow receives invisible focus; Tab can leave |tabIndex=-1 skips only the redundant submit. Visible44×44 Refresh control has a3px focus ring; actual Enter and clear query still return200 from the protected list API in all12 cases. |

`measured-summary.json`: the final review has94 checked text samples in **each**
theme and zero failed samples; the list has64 and Arabic review81. Accessible-name
counts are derived mechanically from CLI ARIA snapshots: zero unnamed interactive
controls. That generic text-node probe did not sample search placeholders. The
independent4.43 finding is retained in search-before.raw.json; its targeted
correction is search-after.raw.json/search-summary.json, plus12 screenshots.
Canvas records all ancestor backgrounds and placeholder alpha; no disabled,
opacity or image exemption was applied. summarize-search.mjs calls Design Studio
55c8614dcfff33b4caa5a544b4f1f91877214878 pure buildReport helpers only. These are
sampled page/state results, not certification of every user
font/theme preset or an assistive-technology session. Heading order was inspected
from the actual h1 page / h2 task and dialog / h3 section hierarchy; the generic
probe's heading-order field remains null.

The original failing review report is retained. Original-BRIEF and separately
Canvas-normalized reports are both kept; the root brief is never changed to make
the checker green. Raw lint still reports token differences (11 light,14 dark):

- The brief contains the shorter Inter fallback stack. `globals.css:19` retains
  the same primary **Inter Variable** family and its complete system fallback
  chain; the checker compares a brief stack and a primary-family string unevenly.
-18px is inherited `--radius-2xl = --radius ×1.8` (`globals.css:1029`), not a new
  global radius.6px is the existing1.5 spacing utility used for icon/text gaps.
  The247px sample is a computed auto margin in the sidebar, not a specified
  spacing token. Direct computed-style evidence is generation2/final-style-focus.json.
- Paired dark neutrals/primary are explicitly authorized by the original brief;
  its palette lists only light values. Status amber/emerald and destructive
  red800/red300 are deliberate existing Tailwind tokens with measured contrast.
  Alpha/color-mix values and black overlay are existing control treatments, not
  extra brand palettes. Direct selected-navigation styles retain rgb(36,94,88)
  and its0.1-alpha OKLab background; the Canvas alpha inventory's #276258 is not
  a replacement primary token. Original flags remain visible for review.

The People linter's two loading warnings are source-search false positives:
`people.tsx` uses `busy`, disabled fields/actions and `copy.saving`; its parent
`workspace.tsx` owns directory loading with `role=status`. Actual member mutation
and denial flows passed. No claim is made that the linter itself returned no flags.

## Functional and scope review

BC-2/3/8/10/11/14/15: real protected bootstrap and member setup, masked one-time
credential, two personal sessions, direct human task creation, literal calendar
dates, concurrent409 draft comparison/adoption, notes and deliverable submission,
independent human review, and cross-session Done are recorded. Actual viewer
forgery returns403; revocation returns401 and discards private workspace state.
Area/search/empty and unbound execution-link error use real backend responses.

BC-4/9/12: Arabic navigation and exact LTR calendar day, focus containment/return,
private cross-tab English→Arabic→English draft retention, and mobile review are
covered by CLI artifacts. Drawer checks wait for its documented starting style
and queued focus guard; early timing/duplicate-selector failures are preserved.

No remaining verified contrast or overflow defect in the sampled states.
Remaining review limits: continuous scroll smoothness, CLS and long-frame counts
were not measured; no motion filmstrip/video was captured. No screen-reader,
native OS window or standalone production-server launch is claimed. English and
Arabic business copy are implemented; other app locales use English business copy.
The manual fixture uses real protected APIs but remains synthetic and guarded.

Accepted task integration and owned final gates are complete. The independent
reviewer reports final targeted PASS on the e72 export/hash c029f99d: six own
width/theme/EN-AR cases give10.7805:1 light/7.0679:1 dark, visible settled Refresh
focus and Enter200. Its own real409 clearly shows base1/current Review3, preserves
the exact draft and requires explicit adoption/save4 while invalidating stale
review. BUI-R1/R2/R3 are closed. This records the delivered independent verdict;
the reviewer owns its final evidence publication. Root owns merge and actual
packaged native startup. Broader localization remains future scope.

Local learning: a state variant can outrank an ordinary reduced-motion utility;
measure the actual dialog, not dormant global scroll timelines. Controlled Radix
dialogs need an explicit focus return when they have no Trigger. Retained user
tokens must be classified against their real source before treating a scan-only
brief's incomplete inventory as a product defect. No shared RAG/memory was modified.
