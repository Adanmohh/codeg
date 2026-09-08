// Reuse Design Studio's pure contrast/AA implementation; browser work is CLI-only.
import fs from "node:fs"
import {
  buildReport,
  contrastRatio,
  parseCssColor,
} from "/Users/mohamedadan/projects/design-studio/lab/tools/probe.mjs"

const input = JSON.parse(fs.readFileSync(process.argv[2], "utf8"))
const raw = {
  colors: input.rows.flatMap((row) => [row.fg, row.bg]),
  fontFamilies: input.rows.map((row) => row.fontFamily),
  fontSizesPx: [10],
  spacingPx: [],
  radiiPx: [],
  textSamples: input.rows.map((row) => ({ ...row, text: `${row.theme}/${row.state}: ${row.text}` })),
  transitionsCount: null,
  easings: [],
  reducedMotionHandled: null,
}
const ratio = (fg, bg) => contrastRatio(parseCssColor(fg), parseCssColor(bg))
const report = {
  source: process.argv[2],
  phase: input.phase,
  method: "Actual Chromium Canvas sRGB pixels, compositing every ancestor background; Design Studio 55c8614 pure contrast and normal-text AA threshold. Canvas uses 8-bit channel rounding.",
  scope: "Existing 10px folder running-count badge only; rest, pointer hover, keyboard focus, both themes",
  ...buildReport(raw, null),
  measurements: input.rows.map((row) => ({
    theme: row.theme,
    state: row.state,
    fg: row.fg,
    bg: row.bg,
    ratio: ratio(row.fg, row.bg),
    lightCandidateRatio: row.theme === "light" ? ratio(row.candidateFg, row.bg) : null,
    focusVisible: row.focusVisible,
    aria: row.aria,
  })),
  providerRequestsBefore: input.before.providerRequests,
  providerRequestsAfter: input.after.providerRequests,
}
process.stdout.write(JSON.stringify(report, null, 2) + "\n")
