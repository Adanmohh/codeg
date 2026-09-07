# Design Studio loop1 — preliminary Ops review

Specialists: aesthetic-judge, a11y-auditor, flow-validator, applied sequentially
by the root orchestrator using their source methods because all three authorized
Herdr worker slots were occupied. These are not three independent agent reviews.
Audited against `docs/design/BRIEF.html`, island schema1, scan mode, and the stable
BC checklist. Scope: accepted Ops email screens and inherited connection/shell
surfaces. P1/Telegram integration and final release acceptance are still pending.

| Dimension | Score /10 |
| --- | --- |
| States | 8 |
| Feedback | 7 |
| Accessibility | 6 |
| Responsive | 6 |
| Visual/tokens | 8 |

Design-weighted verdict: about7.4/10 for this limited surface (Design8×40%,
Usability6.5×30%, Creativity7×20%, Content8×10%). **Not accepted as final design:**
verified edit loss and accessibility defects remain regardless of the score.

## Verified findings and top fixes

1. **High — preserve editing across locale changes** (BC-3/BC-9). The checklist
   requires cancellation to return to the “preserved draft”; Settings in another
   tab currently destroys it without a choice. Root reproduced the loss and
   verified the AppI18nProvider loading branch replaces the mounted children.
   Keep mounted state during later locale loads, then repeat reply/note/review
   language transitions. Evidence: `locale-edit-loss.md`.
2. **High — name workspace controls** (BC-2 plus Button accessible-name check).
   Terminal new-tab and the empty status-alert popover have no name in the
   actual ARIA tree, corroborated by source. Reuse their existing translated
   action labels and retain keyboard focus; verify empty/populated states.
3. **High — correct measured text contrast** (BC-2/WCAG1.4.3). Inherited sidebar
   labels/empty text and status text fail on their effective backgrounds. Remove
   excessive opacity or use a readable foreground treatment, then remeasure
   light/dark. The Ops destructive-button contrast fix already passes.
4. **Medium — persistent login label and announced validation** (BC-1/Input
   labels/errors). Fresh 390px actual invalid-token submission showed the retry
   message, but DOM labels=[], aria-invalid/describedBy=null and error role/live
   both null. Add the associated visible label and error semantics. An initial
   executor assertion used the wrong shortened error sentence and timed out;
   corrected to the actual source string, then verified the existing result.
5. **Medium — RTL directional/content treatment** (BC-3, CRAP/alignment).
   The mobile drawer/thread have no horizontal overflow, but Back to inbox keeps
   a left arrow and English message punctuation reorders under inherited RTL.
   Mirror navigation and isolate content direction.
6. **Low — compact the Ops header** (BC-18, CRAP/proximity). Remove the internal
   account-number prefix while retaining useful connection guidance.
7. **Low — make receipt copy consistent** (BC-12). Remove the generic no-receipt
   implication when an actual accepted provider receipt is shown.

High findings were self-verified inline because no fourth/nested worker was
available. The three sources of evidence agree: executed CLI transitions,
actual DOM/probe values, and the matching source conditions. No static-only
heuristic was promoted to a high finding.

## Measured

Post-fix light review: status text #737373/#f5f5f5=4.35. Earlier light workspace:
group labels3.68, No chats2.66; dark No chats4.06. Dark Ops controls pass.
Two inherited unnamed workspace controls remain. Arabic viewport390/scroll390.
Probe JSON and screenshots: `../browser-step2/`; lint outputs: `lint/`.
Raw off-palette reports include unsupported OKLCH conversion/rounding and the
light-only brief applied to paired dark/semantic colors; they are not confirmed
palette defects. No motion/jank/reduced-motion pass is claimed; no motion judge
was selected.

## Functional

Executed email terminal-state, private-note/draft, stale-review, viewport-edit
preservation and offline-retry checks passed; see `flows/`. The first failure
for the added locale scenario is wrong-state-rendered when the workspace
subtree disappears after the settings change. BC14 actual receipt-recording
recovery, full P1/phone flow and final combined keyboard/theme coverage are
pending. Existing Rust tests supplement but do not replace these browser checks.

## Next loop

The high locale finding is assigned to approvals after its Telegram checkpoint.
Other fixes will be grouped in a bounded worker design branch after feature
handoffs. Run the final selected specialists through available Herdr workers
when slots free, mechanically merge their findings, apply verified fixes, then
repeat actual CLI/probes on integrated main. Do not end at this preliminary score.
