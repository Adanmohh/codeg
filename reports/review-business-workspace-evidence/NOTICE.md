# Review evidence attribution

Report-only orchestration; no product code or dependency was changed.

- Reviewed Codeg frontend: `33b9cbcb63d3023e3ccd36a21f9cb9e4e2e425b7`; handoff `a126730274c8a3dc822345da536eebed1f8dc253`. Root LICENSE/NOTICE remain unchanged.
- Canvas/visibility/selected-option probe is reused from the accepted Codeg report `reports/design-phase1-specialist/probes/capture.js` at `a126730274c8a3dc822345da536eebed1f8dc253`, blob `2e335c24e23849790def6b5a01ff39a61213cc1f`, Apache-2.0. Its original attribution is `reports/design-phase1-specialist/NOTICE.md`, blob `d5c3dec3b67c35b2fe730db931555fdfa1498ee6`; retained without alteration. Report scripts use the existing installed Playwright CLI 0.1.18 API as orchestration glue.
- Design Studio `55c8614dcfff33b4caa5a544b4f1f91877214878`, MIT: `lab/tools/probe.mjs` blob `d1d6fc1386cda2f9b2365ed4eade4adf26cde717`; pure report/contrast/ARIA helpers only, plus installed selector/lint/merge and specialist method files. No direct browser launcher, MCP or paid flow SDK is invoked.

Design Studio original MIT licence (blob `886f194fc4222ee4d2dcec241e0b68a78bf0a9e0`):

MIT License

Copyright (c) 2026 AdanMohh

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

`connect-reviewer.js` adapts the credential-in-memory test orchestration from
`reports/business-workspace-evidence/member-connect.js` at accepted review target
`a126730274c8a3dc822345da536eebed1f8dc253`, blob
`0e54c8a47fc187b57fb4bb62afcc028041cd87e1` (Codeg, Apache-2.0).
Only synthetic setup names and reviewer assertions changed; no live credential is
embedded, persisted or reported. The fixture operator literal is public test data.

The embedded Canvas brief normalization in `matrix.js` adapts the Apache-2.0
`reports/design-phase1-specialist/probes/normalize-brief.js` at the same a126
source, blob `8146af12cc4bcb7277399f2da9beeddda04b419a`. It changes the input
delivery to a read-only JSON argument; original BRIEF is unchanged.

Exact Design Studio55c8614 tool/method blobs used without copying runtime code:

| File | Blob |
| --- | --- |
| scripts/brief.mjs | f0320ff468ac219b009f26bdee2de46cebb24a2b |
| scripts/design-lint.mjs | a5b26491665c8dd4fababcd9d9ffbcfa33d8f4f3 |
| scripts/select-judges.mjs | 3e2a6ca6ebc0d429e369a268c67c136c17b8ec89 |
| scripts/merge-findings.mjs | 1a230f2d157b2733174f814250f951516a0df302 |
| agents/aesthetic-judge.md | 085979ab893cafdc4d78c50ce6a4eca5f870bb09 |
| agents/a11y-auditor.md | b73facb885fca451d7cc716ced93dd6feb3feb29 |
| agents/flow-validator.md | 2f1cfd089acce45de3315a247d2bb512643c098c |
| agents/motion-judge.md | db3ab48d568480b00e2ba0aa8e5c00197ba79752 |
| agents/design-reviewer.md | 6cb08cc97c704b0ae5eda49ad94b246582fe62d0 |

Local installed API references: React19.2.4, Playwright CLI0.1.18 with
playwright-core1.63.0-alpha-2026-08-05, @types/node25.2.2 fs.d.ts, and existing
@fontsource-variable/inter5.3.0 index.css (OFL-1.1 font, no font code copied).
All specialist methods were applied sequentially by this reviewer; no agents,
MCP browser, paid flow SDK, dependency installation or live service was used.

Final correction review at `e72cc44b612068e67a3e6dc3bc593f10988ae7ed`:
`final-search.js` embeds the same accepted capture.js (blob2e335c24 above);
`final-conflict.js` reuses the same synthetic credential-in-memory orchestration
(blob0e54c8a above) with real protected task API calls read from the reviewed
Codeg types. `final-guard.js` adds browser SHA-256 artifact verification using
installed TypeScript DOM Body.arrayBuffer/SubtleCrypto.digest types. No new
third-party implementation, dependency or product source is copied.

The product correction's original foreground/80 token was independently read
through gh api: `src/components/chat/feedback-notes-display.tsx` at Codeg
`3a189d1822f9fd335cc58ecb440f75ab9cbe937e`, blob
`a897fae2cb089bbda3e94c203071bae4c3ef440d`, Apache-2.0. Its existing e72 root
NOTICE block remains untouched. Source presence verifies attribution, not
production readiness; rendered contrast/keyboard checks provide behavior evidence.
