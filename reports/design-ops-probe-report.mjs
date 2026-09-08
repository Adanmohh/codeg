// Pure report formatting; browser extraction runs only through Playwright CLI.
import fs from "node:fs"
import {
  buildReport,
  countUnnamedInteractive,
} from "/Users/mohamedadan/projects/design-studio/lab/tools/probe.mjs"
import { parseBrief } from "/Users/mohamedadan/projects/design-studio/scripts/brief.mjs"

const brief = parseBrief(fs.readFileSync("docs/design/BRIEF.html", "utf8"))
const rows = ["pending", "thread", "terminal"].flatMap((name) => {
  const data = JSON.parse(
    fs.readFileSync(
      `reports/design-ops-${name === "terminal" ? name : `probe-${name}`}.raw`,
      "utf8"
    )
  )
  return Array.isArray(data) ? data : data.results
})
fs.mkdirSync("reports/design-ops-measured", { recursive: true })
const reports = rows.map(({ surface, viewport, theme, raw }) => {
  raw.aria.unnamedInteractive = countUnnamedInteractive(raw.aria.snapshot)
  const report = buildReport(raw, brief)
  const result = {
    surface,
    viewport,
    theme,
    scope:
      "Ops thread/review only; inherited shell is outside this measurement",
    ...report,
    targetsUnder44: raw.smallTargets,
    inputsUnder16: raw.smallInputs,
    horizontalOverflow: raw.horizontalOverflow,
    viewportOverflow: raw.viewportOverflow,
  }
  const slug = surface.toLowerCase().replace(/[^a-z0-9]+/g, "-")
  fs.writeFileSync(
    `reports/design-ops-measured/${slug}-${viewport}-${theme}.json`,
    JSON.stringify(result, null, 2) + "\n"
  )
  return result
})
fs.writeFileSync(
  "reports/design-ops-measured.json",
  JSON.stringify(reports, null, 2) + "\n"
)
for (const {
  surface,
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
      surface,
      contrast,
      unnamed: aria.unnamedInteractive,
      targetsUnder44,
      horizontalOverflow,
      reducedMotionEffective: motion.reducedMotionEffective,
    })
  )
