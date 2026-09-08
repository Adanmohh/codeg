# Visual refresh — Hafidh dispatch desk

Owner request, 2026-09-08: the app looks basic; continue improving autonomously.
This reopens visual quality after the bounded Phase 1 functional acceptance. It
adds no service, backend workflow, dependency, paid asset or later-phase feature.
The existing 8.1 score was scoped to correctness and measured operational UI;
it is not evidence that the visual finish satisfied the owner.

## Art direction

Direction: editorial workspace discipline, composed as a dispatch desk.
World: correspondence, work queue, evidence sheet, review stamp, receipt.
Color: retain Hafidh accent #245e58 / dark #9bd4c5 and inherited semantic theme
pairs. In the default theme, aim for a gently differentiated canvas and raised
working surface, dark ink/light ink and quiet rules. Use existing background,
card, muted, foreground, border and primary tokens; subtle token compositing is
allowed within Ops scope. Do not replace user-selected themes or globally
recolor inherited tools. Exact computed values must be measured in both modes.
Type: preserve configured UI font (default Inter Variable). A restrained 28–32px
600/-0.025em workspace heading supplies display character through proportion;
body 14px/1.6, section headings 15–16px/600, metadata 12px, real IDs/counts use
existing ui-monospace/tabular-nums. One display heading per view. No downloaded
font just to satisfy a generic marketing recipe.
Layout: a compact branded workspace header above a clear work surface; desktop
uses deliberate primary/secondary proportions, mobile one navigable pane.

    workspace identity                  refresh / connections
    Inbox    Approvals    Morning       current-view marker
    +----------------------+-------------------------------+
    | queue / filters      | selected correspondence       |
    | readable rows        | evidence or complete review    |
    | status + identity    | primary action / private note  |
    +----------------------+-------------------------------+

Morning: heading + truthful queue summary, primary attention/replies column,
secondary active/up-next/correspondence region. At narrow widths preserve the
priority order. Counts only from already-loaded data, clearly scoped, never
invented trends, totals or success rates. Empty queues should be compact and
intentional; an unconfigured inbox gets a useful setup form, not a giant void.
Signature: the correspondence-to-decision sheet. A selected queue row leads to
one legible working sheet; full approval payload becomes a review sheet; final
receipt echoes its compact status/identity treatment. No arbitrary decoration.
Self-critique: rejected dashboard metric cards, oversized greetings, gradients,
colored card-edge stripes and ornamental illustrations. Retain brand teal,
Inter and installed Lucide intentionally for continuity/user settings. Improve
composition and hierarchy rather than changing font/icon libraries for novelty.
Avoid nested card soup: maximum two visual container levels.

## Implementation boundaries

Rebrand worker owns `src/components/ops/ops-page.tsx`, `morning-view.tsx`, `ui.tsx`,
`src/app/login/page.tsx`, and small presentation-only changes to
`src/components/layout/sidebar.tsx` if useful. May add an Ops-scoped stylesheet
or new Ops visual primitives with no new package; preserve existing exports.
Do not change global theme values, route state, navigation guards, shared Drawer,
agent/task engine, session provider or storage. Publish any shared style contract
early; the other worker should use current semantic utilities independently.

Tickets worker owns `src/components/ops/inbox-view.tsx`, `reply-editor.tsx`,
`proposals-view.tsx`, `email-settings.tsx`; then harmonize existing
`src/components/ops-intake/` and `src/components/ops-telegram/` presentation if
needed for complete review-sheet continuity. No edits to rebrand-owned files,
Rust/API/session/transport logic or schema. Preserve exact human review content,
all recipients, safety notices, revision checks and truthful receipt semantics.
Existing tests can be updated only for changed user-facing copy/structure while
retaining behavioral assertions. Add tests only for meaningful new behavior.

Reviewer owns reports/evidence only. Capture before/after from actual exported
UI and guarded synthetic backends; evaluate visual hierarchy and finish as well
as functional/a11y acceptance. Report real findings without a score target.

## Acceptance

- Clearly recognizable before/after improvement in default light and dark at
  1280px, plus usable 390px and 768px compositions; no fake product data.
- Header, navigation, list rows, detail, editor and review belong to one design.
  Selected/read/unread/status differences do not rely only on color.
- Empty and populated Morning/inbox/approval/intake states look intentional;
  setup and error recovery use real available actions, no nonfunctional controls.
- All inherited scoped functional checks remain: private note vs public reply,
  full payload review, dirty guard, locale/breakpoint preservation, stale/unknown
  receipt handling, no resend, evidence refusal, and protected phone review.
- Keyboard/focus/labels/contrast, long subjects and addresses, RTL, 44px mobile
  actions, scroll containment and reduced motion pass actual browser checks.
- No broad Settings snapshots, real credentials or live outbound actions. Name
  synthetic fixtures/response overlays; never claim live integrations were tested.
- Workers read installed/pinned source and code-context before edits; remote docs
  via gh api at immutable refs; borrow from the existing Apache codeg components
  and named founding sources with precise NOTICE attribution; no AGPL source.
- Final Design Studio aesthetic/a11y/flow/verifier loop, root Playwright CLI
  independent checks, relevant automated gates and refreshed native app build.
