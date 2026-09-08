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
