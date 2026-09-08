// Pure report formatting; browser extraction runs only through Playwright CLI.
import fs from "node:fs"
import {
  buildReport,
  countUnnamedInteractive,
} from "/Users/mohamedadan/projects/design-studio/lab/tools/probe.mjs"
import { parseBrief } from "/Users/mohamedadan/projects/design-studio/scripts/brief.mjs"

const brief = parseBrief(fs.readFileSync("docs/design/BRIEF.html", "utf8"))
const rows = JSON.parse(fs.readFileSync("reports/bug-probe-raw.json", "utf8"))
const reports = rows.map(({ viewport, theme, raw }) => {
  raw.aria.unnamedInteractive = countUnnamedInteractive(raw.aria.snapshot)
  const report = buildReport(raw, brief)
  return {
    viewport,
    theme,
    scope: "P1 Bug intake only; preliminary",
    ...report,
    targetsUnder44: raw.smallTargets,
    inputsUnder16: raw.smallInputs,
    horizontalOverflow: raw.horizontalOverflow,
    viewportOverflow: raw.viewportOverflow,
  }
})
fs.writeFileSync(
  "reports/bug-design-probe.json",
  JSON.stringify(reports, null, 2) + "\n"
)
for (const {
  viewport,
  theme,
  contrast,
  aria,
  targetsUnder44,
  horizontalOverflow,
  motion,
} of reports)
  console.log(
    JSON.stringify({
      viewport,
      theme,
      contrast,
      unnamed: aria.unnamedInteractive,
      targetsUnder44,
      horizontalOverflow,
      reducedMotionEffective: motion.reducedMotionEffective,
    })
  )
