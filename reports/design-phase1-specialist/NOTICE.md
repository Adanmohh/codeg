# Report-tool attribution

This directory is review evidence and measurement glue, not a product port.
Original root NOTICE and docs/design/BRIEF.html remain unchanged.

- Design Studio 0.8.0, immutable commit `55c8614dcfff33b4caa5a544b4f1f91877214878`, MIT. Pure `lab/tools/probe.mjs` (blob `d1d6fc1386cda2f9b2365ed4eade4adf26cde717`) supplies contrast, inventory/brief comparison and ARIA-name parsing. `scripts/intake-scan.mjs` (blob `c4207c4f56ae30338cf99d472671aec96420cc67`) supplies the exact `canonHex` limitation. `scripts/brief.mjs` supplies parse/render of derived copies. Selector, lint, merge and specialist methods are invoked/read from this same installed commit; no browser launcher or paid flow SDK is invoked.
- Accepted Codeg report helper `reports/design-shell-evidence/measure.js` at `0565f197df5754bf14fb37d0be3a13715c70e85c` (blob `0ae35c4ee0ad516ead4d6a4be3026628ec6eed70`), existing repository Apache-2.0 license in ../../../LICENSE. Adapted Canvas/color-layer/visibility structure into `probes/capture.js`; added native checkVisibility, selected-option/value/placeholder sampling, disclosed exclusions and property-level motion records. The existing NOTICE remains authoritative for upstream Codeg attribution.

Design Studio original MIT license (blob `886f194fc4222ee4d2dcec241e0b68a78bf0a9e0`):

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
