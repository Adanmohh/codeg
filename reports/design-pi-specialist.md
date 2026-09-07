# Pi setup/default — Design Studio specialist review

Complete bounded report-only review, 2026-09-08. **Three verified high findings
and two medium findings.** Default Pi selection, disabled Send and keyboard
navigation to Settings work in the isolated fixture. Setup accessibility and
guidance still need correction; this is not a full Phase 1 design acceptance.

Draft PR: https://github.com/Adanmohh/codeg/pull/11.
Branch: `docs/design-pi-review`. Early pushed checkpoint: `4d09492b`.
Reviewed accepted main: `4d39c7d74722d217454df1d642928176aed8a938`, including
PR #8 merge `70980fac8f2151a6ba579c45e5fe798aa5a671a7`.
`git diff eb6f7113543f6219d8aba1cbedac4f1cfd2e6e29 HEAD -- src src-tauri
integrations package.json pnpm-lock.yaml` was empty: the existing production
export belongs to the accepted Pi product. No product, lockfile, NOTICE,
planning-document or other-worktree changes in this PR.

## Scope and method

Reviewed Pi default selection, blocked setup guidance, actual Settings
destination and relevant inherited shell accessibility. Read complete
FOUNDING/ORCHESTRATOR/STATUS/DECISIONS/AGENTS, `docs/design/BRIEF.html`, root's
`reports/design-loop-1/{README,review,merged}.json/md` as applicable, all three
preliminary findings files, retry/locale evidence, and the accepted Pi report.
Read root's ignored reviewer memory at
`/Users/mohamedadan/projects/ops-desk/.claude/agent-memory/design-reviewer/MEMORY.md`
read-only after its location was clarified. Learnings stay here.

Applied code-context, design-audit, design-checklist and Playwright CLI skills.
Local Design Studio checkout was clean at
`55c8614dcfff33b4caa5a544b4f1f91877214878`. Read its actual
`agents/{aesthetic-judge,a11y-auditor,flow-validator,design-reviewer}.md`,
`commands/design-audit.md`, checklist/award/banner/button/error/a11y references,
Aura reference, `lab/tools/{brief-checklist.md,probe.mjs}` and selector/lint/merge
scripts. The project brief's operational Inter, neutral themes and paired
Hafidh accent take precedence over generic marketing aesthetics.

The selector chose aesthetic, a11y and flow. This worker applied those methods
sequentially, then attempted to refute every high finding using source and
browser evidence. No additional agents or independent-person claim: this
worker also authored the accepted Pi bridge. No paid flow executor ran.

Findings use Design Studio's actual agent-definition contract (there is no
`schemas/` directory):
`{specialist,scores,findings:[{surface,element,severity,check,fix}],notes}`,
severity `high|med|low`. Each specialist file was written before mechanical
merge. [Structured findings](design-pi-specialist/merged.json),
[12 local brief checks](design-pi-specialist/brief-checks.xml), and
[executed-flow record](design-pi-specialist/flow-run.json) are committed.
These BC IDs are local to this audit, distinct from root's 18-check checklist.

## Verified findings, in fix order

1. **High — Pi configuration fields lack associated labels.**
   `src/components/settings/pi-config-panel.tsx:919` (Provider), `:1000`
   (Model), `:1140` (Thinking), `:1171` (API Key). Open the missing-setup action,
   then scroll the Pi card to Configuration. The actual ARIA tree exposes two
   unnamed comboboxes. All four controls have `labels=[]`, no `aria-label` and
   no `aria-labelledby`; Model/API Key rely on example placeholders.
   [Field facts](design-pi-specialist/evidence/settings-fields.json),
   [mobile ARIA](design-pi-specialist/evidence/07-settings-mobile.yml),
   [screenshot](design-pi-specialist/evidence/07-settings-mobile.png).
   **Required fix:** associate existing translated visible labels with stable
   control IDs, including SelectTrigger, and verify names with empty/populated
   controls. Refutation: visible adjacent `<label>` text and placeholders do
   not establish the missing association; adapter-version control elsewhere
   is labelled correctly and is excluded from this finding. BC-6; checklist
   Input labels / accessible names.

2. **High — new-terminal and empty-alert buttons are unnamed.**
   `src/components/terminal/terminal-tab-bar.tsx:160` and
   `src/components/layout/status-bar-alerts.tsx:158`. Start a fresh workspace
   before an alert exists; terminal plus and the empty alert trigger appear
   as bare buttons in [actual initial ARIA](design-pi-specialist/evidence/02-workspace.yml).
   The terminal button's icon-only DOM is captured in
   [desktop measurement](design-pi-specialist/evidence/02-desktop-light.json).
   **Required fix:** put existing translated New terminal / Alerts labels on
   the actual controls; preserve focus and test empty/populated states.
   Refutation: the terminal wrapper's tooltip only exists when disabled and
   is not a name for the enabled button. Radix Popover adds popup/state
   semantics but no action name. Other icon controls with native titles or
   named images were not automatically classified as unnamed. BC-8; root's
   corresponding high finding is independently confirmed.

3. **High — inherited shell text misses the contrast threshold.**
   `src/components/conversations/sidebar-section-header.tsx:161`,
   `sidebar-conversation-list.tsx:2806,2854,2872,2882,2892,2987` and
   `src/components/layout/status-bar.tsx:21,37`.
   Fresh desktop neutral-theme measurements: Folders/Chat/Recent **3.71:1**
   light, **5.09:1** dark; No chats / No conversations / No recent conversations
   **2.66:1** light, **4.05:1** dark. Root's earlier rendered status text is
   **4.35:1** light (`#737373` on `#f5f5f5`); its unchanged parent class was
   read, but this fixture has no loaded stats, so no new status-text render is
   claimed. [Ratios and RGB facts](design-pi-specialist/evidence/contrast-summary.json).
   **Required fix:** reduce opacity loss and use readable existing foreground
   tokens, then measure both themes. Refutation: section source explicitly
   records an upstream deliberate 3:1 tradeoff, but 14px regular labels are
   normal text under the current brief/checklist's 4.5:1 requirement. Current
   owner explicitly authorized correcting this inherited tradeoff. BC-9;
   checklist Accessibility last mile, dimmest labels. Tiny RGB rounding
   differences from root's 3.68/4.06 values do not change the result.

4. **Medium — setup reason is clipped and destination lacks Desk guidance.**
   `conversation-detail-panel.tsx:2178,2257`, `pi-config-panel.tsx:1006,1144`,
   `src-tauri/src/acp/pi_desk.rs:92`. At 390px the full 1310px string occupies
   a 332px one-line button. Shift+Tab from the composer focused that button
   with a browser outline; Enter opened the real Settings tab at
   `/settings/agents?agent=pi`. This is **navigation, not expansion**.
   [Mobile banner](design-pi-specialist/evidence/04-mobile-banner.png),
   [keyboard focus](design-pi-specialist/evidence/05-keyboard-focus.json),
   [actual destination](design-pi-specialist/evidence/06-settings.yml).
   The destination shows generic provider setup, `claude-sonnet-5` placeholder,
   Thinking Off and an adapter PASS badge, with no Desk-specific Astra/max
   catalogue instructions. **Required fix:** readable full reason without
   hover, an explicit Open Pi settings action, and Desk readiness guidance
   separated from adapter availability. Retain Astra/max fail-closed checks;
   do not auto-login, install or infer. Refutation: full accessible name/title
   exists and keyboard navigation works; this is not a missing error-string,
   missing link or silent fallback defect. Existing experienced-user native
   configuration remains possible. BC-3/7; Banner and Error recovery guidance.

5. **Medium — setup banner text is slightly below contrast target in light.**
   `conversation-detail-panel.tsx:2175,2254`. Actual 12px red text is **4.42:1**
   on the light tinted background; dark is **6.57:1**. Use an existing scoped
   readable destructive foreground and remeasure both themes. This measured
   Pi banner is separate from root's already corrected Ops destructive text.
   Refutation: blending the translucent background is included; comparing
   only the raw red token to white would overstate the ratio. BC-9;
   CRAP Contrast. No global palette redesign is proposed.

## Executed evidence and limits

Actual `playwright-cli` **0.1.18**, installed Playwright
**1.63.0-alpha-2026-08-05**, named browser `pi-design-review`. Fresh accepted
`pi_desk_browser_fixture`, loopback **4324**, own temporary database and
`src-tauri/target/pi-desk-browser/empty-pi-design-review`. Existing
availability-only `reports/pi-desk-evidence/pi-acp` permits version discovery,
while actual Pi setup fails on the empty Astra catalogue before inference.
The existing real owned companion remained untouched. Context routing covered
the Settings popup as well as the workspace. Counts:
**3 Pi connection preflights, 0 prompt requests, 0 off-origin HTTP requests,
0 blocked install/credential/other-agent writes**.
[Guard](design-pi-specialist/evidence/guard.js) and
[counts](design-pi-specialist/evidence/network-counts.json).

Six checks have new supporting evidence, one explicit-default check is
supported by accepted prior screenshots/source (not rerun), and five fail
design criteria. Newly inspected states: 1440px light/dark workspace, 390px
sidebar and blocked composer, actual Settings destination at 1200px and
390px, keyboard focus/Enter, real Appearance theme selection. Workspace and
Settings each measured 390px document width at 390px. Theme change preserved
Pi selection. No assertion of successful configured setup, usable provider,
screen-reader session, native Tauri webview, real phone, RTL/motion/zoom audit,
complete focus order, Ops/Telegram/P1 acceptance or OS sandboxing.

The measurement helper uses browser Canvas to convert computed OKLCH/Lab and
color-mix to sRGB, composites background layers, then uses Design Studio's
actual `contrastRatio` pure helper. Only samples without group opacity or
background images are ranked. This is targeted measurement, not an automated
whole-page conformance certificate. The 03-mobile sidebar screenshot has an
open drawer: only its visible sidebar samples are used; the unobscured banner
is assessed from 04-mobile-banner instead.

Root's locale draft-loss finding has a matching source explanation in
`src/components/i18n-provider.tsx`, but is already assigned to approvals and
was not rerun or duplicated here. Root's later PR10 login-label/error fix and
390px check are acknowledged as owner-reported pending acceptance; no login
fix recommendation. Port 4323 and other fixtures were untouched. Internal IDs,
exact To/Cc/Bcc approval presentation and P1/Telegram follow-ups remain outside
this bounded Pi/shell review.

## Commands, exits and docs-first evidence

- Initial separate `cat node_modules/react/package.json` and
  `cat src-tauri/Cargo.toml`: exit 0. Read pinned React **19.2.4**, local
  TypeScript DOM types, next-themes **0.4.6** manifest/types, installed
  Playwright types/help, `@types/node` **25.2.2** fs types (Node runtime
  **24.19.0**) and exact current source before relying on these contracts.
- Code-context guide with existing rag-skills `.venv/bin/python` and
  `HF_HUB_OFFLINE=1`: exit 0. Applied installed-version-first guidance.
  The prior docs lookup reports `data/code/tickets.db` absent (exit 3);
  no claim of repository corpus coverage. Direct pinned source supplies the
  missing coverage; no corpus ingestion/global changes.
- Live docs-first hook audit contains this session
  `01a07c1c-d82f-7022-84db-778a438632f1`, exact tickets cwd, both PreToolUse and
  PostToolUse. [Filtered evidence](design-pi-specialist/evidence/hook-records.json).
  Hook reminders remained enabled. No remote documentation was needed; no
  web/context7/latest-source substitutions or new borrowed product code.
- `git fetch origin`, clean branch creation and early commit/push: exit 0.
  `gh pr create --draft ... --body-file reports/design-pi-specialist-pr.log`
  after installed help reads: exit 0, PR #11.
- Design Studio `select-judges.mjs ... --page --flows`, four
  `design-lint.mjs` calls with this brief, and `merge-findings.mjs`: exit 0.
  Outputs under `design-pi-specialist/{selection.json,lint,merged.json}`.
  Lint exit 0 means tool completion, not design pass. Refuted its high empty
  list claim: StatusBarAlerts actually renders `t("empty")`. Its generic
  pending/focus warnings cannot establish a defect in synchronous toggles or
  shared Button primitives; PiConfigPanel has real saving/disabled branches.
- Browser fixture command: `cargo test --locked --no-default-features --lib
  pi_desk_browser_fixture -- --ignored --nocapture` with own empty catalogue,
  fixture PATH/data/companion env: final exit 0 after own stop file. Initial
  attempt exit 101 because a previously used catalogue contained auth.json;
  used a new empty directory without reading/deleting that file. A shell
  `rm -f` cleanup was rejected; exact prior own stop file was removed through
  apply_patch. No approval/hook bypass. A stale login ref returned CLI exit 1;
  fresh snapshot supplied e15 and login then passed.
- All final CLI capture/measure/keyboard/theme/tab/close commands: exit 0.
  [Evidence directory](design-pi-specialist/evidence). Fixture log stays
  ignored at `reports/design-pi-browser.log`; final result 1 passed.
  No product gates rerun for this report-only diff.

Scores are scoped judgments, not acceptance certificates: states **7/10**,
feedback **5/10**, accessibility **5/10**, responsive **6/10**, visual/tokens
**7/10**. Design **7**, usability **5**, creativity **7**, content **5**, weighted
40/30/20/10 = **6.2/10**. Specialist JSON preserves its required score keys.
The three verified high findings, rather than the score, determine fix order.

## Continuity

Retain these lessons: a navigable error title is not readable mobile guidance;
adjacent labels are not associations; generic adapter PASS is not Desk model
readiness; translucent text must be measured on actual theme backgrounds;
source lint is a triage input. No new memory or product writes outside this
report tree. Owner authorized a separate branch for only inherited shell names
and contrast after this handoff; Pi setup guidance/labels remain reported for
coordination. P1 helper integration waits for accepted host source.
