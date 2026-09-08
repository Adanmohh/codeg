// Final settled-theme report; preserve the first probe and its failures.
// Uses the already-read Design Studio 55c8614 pure formatter, not its browser driver.
import fs from "node:fs"
import {
  buildReport,
  contrastRatio,
  countUnnamedInteractive,
  parseCssColor,
} from "/Users/mohamedadan/projects/design-studio/lab/tools/probe.mjs"
import { parseBrief } from "/Users/mohamedadan/projects/design-studio/scripts/brief.mjs"

const brief = parseBrief(fs.readFileSync("docs/design/BRIEF.html", "utf8"))
const rows = ["pending", "thread", "receipt"].flatMap((name) =>
  JSON.parse(fs.readFileSync(`reports/design-ops-final-${name}.raw`, "utf8"))
)
fs.mkdirSync("reports/design-ops-final-measured", { recursive: true })
const reports = rows.map(({ surface, viewport, theme, raw }) => {
  raw.aria.unnamedInteractive = countUnnamedInteractive(raw.aria.snapshot)
  const report = buildReport(raw, brief)
  const targetSamples = raw.textSamples
    .filter(
      (x) =>
        x.label?.startsWith("In-Reply-To") ||
        x.label?.startsWith("References") ||
        x.text === "Task #1 · Run 1" ||
        (x.text === "Open" && x.fg !== "rgb(115, 115, 115)") ||
        (/^\d+[./]\d+[./]\d+$/.test(x.text) && x.fg !== "rgb(115, 115, 115)")
    )
    .map((x) => ({
      ...x,
      ratio:
        Math.round(
          contrastRatio(parseCssColor(x.fg), parseCssColor(x.bg)) * 100
        ) / 100,
    }))
  const result = {
    surface,
    viewport,
    theme,
    scope:
      "Ops thread/review; actual Appearance setting, open painted threading fields",
    ...report,
    targetsUnder44: raw.smallTargets,
    inputsUnder16: raw.smallInputs,
    horizontalOverflow: raw.horizontalOverflow,
    viewportOverflow: raw.viewportOverflow,
    excludedControls: raw.excludedControls,
    targetSamples,
  }
  const slug = surface.toLowerCase().replace(/[^a-z0-9]+/g, "-")
  fs.writeFileSync(
    `reports/design-ops-final-measured/${slug}-${viewport}-${theme}.json`,
    JSON.stringify(result, null, 2) + "\n"
  )
  return result
})
fs.writeFileSync(
  "reports/design-ops-final-measured.json",
  JSON.stringify(reports, null, 2) + "\n"
)
for (const r of reports)
  console.log(
    JSON.stringify({
      surface: r.surface,
      viewport: r.viewport,
      theme: r.theme,
      contrast: r.contrast,
      unnamed: r.aria.unnamedInteractive,
      targetsUnder44: r.targetsUnder44,
      horizontalOverflow: r.horizontalOverflow,
      viewportOverflow: r.viewportOverflow,
      reducedMotionEffective: r.motion.reducedMotionEffective,
      excludedControls: r.excludedControls,
      targetSamples: r.targetSamples,
    })
  )
