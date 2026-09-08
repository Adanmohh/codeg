// Test-evidence glue. Browser measurements come only from Playwright CLI.
// Calls existing Design Studio pure helpers; no browser launcher or copied tool code.
import fs from "node:fs"
import path from "node:path"
import { pathToFileURL } from "node:url"

const studio = process.argv[2]
if (!studio) throw new Error("Pass the existing Design Studio repository path")
const { buildReport, parseCssColor, contrastRatio } = await import(
  pathToFileURL(path.join(studio, "lab/tools/probe.mjs"))
)
const root = "reports/business-workspace-evidence"
const phases = []
for (const phase of ["before", "after"]) {
  const data = JSON.parse(fs.readFileSync(`${root}/search-${phase}.raw.json`, "utf8"))
  if (data.phase !== phase || data.cases.length !== (phase === "after" ? 12 : 1))
    throw new Error("Unexpected measurement source or incomplete matrix")
  const samples = data.cases.map((item) => item.sample)
  // This is a targeted placeholder check, not a new whole-page/brief audit.
  const { contrast } = buildReport({
    colors: samples.flatMap(({ fg, bg }) => [fg, bg]),
    fontFamilies: [], fontSizesPx: [...new Set(samples.map((s) => s.sizePx))],
    spacingPx: [], radiiPx: [], textSamples: samples,
    transitionsCount: null, easings: [], reducedMotionHandled: null,
  }, null)
  const cases = data.cases.map((item) => {
    const fg = parseCssColor(item.sample.fg)
    const bg = parseCssColor(item.sample.bg)
    if (!fg || !bg) throw new Error("Unparsed measured color")
    const effective = Object.fromEntries(["r", "g", "b"].map((key) => [
      key, Math.round(fg[key] * fg.a + bg[key] * (1 - fg.a)),
    ]))
    return {
      locale: item.locale, theme: item.theme, width: item.width,
      foreground: fg, effectiveForeground: effective, background: bg,
      ratio: Math.round(contrastRatio(effective, bg) * 100) / 100,
      required: 4.5, focus: item.focus, enter: item.enter,
      pageOverflow: item.scrollWidth > item.width,
    }
  })
  if (contrast.checked !== data.cases.length ||
      contrast.failures.length !== (phase === "after" ? 0 : 1))
    throw new Error("Contrast finding or correction did not reproduce")
  phases.push({ phase, export: data.export, contrast, cases })
}
const result = {
  scope: "Search placeholder, sequential focus and Enter; no whole-page score",
  source: {
    before: "095c61642dc2f52ca8fd6e7c10556f16cf61c904",
    after: "e72cc44b612068e67a3e6dc3bc593f10988ae7ed",
    designStudio: "55c8614dcfff33b4caa5a544b4f1f91877214878",
    tool: "lab/tools/probe.mjs pure buildReport/parseCssColor/contrastRatio",
  },
  method: "Canvas sRGB plus ancestor backgrounds and placeholder alpha; all sampled layers opacity 1, no images; enabled empty inputs",
  phases,
}
fs.writeFileSync(`${root}/search-summary.json`, JSON.stringify(result, null, 2) + "\n")
process.stdout.write(JSON.stringify(phases.map(({ phase, contrast, cases }) => ({
  phase, contrast, ratios: [...new Set(cases.map((item) => item.ratio))],
}))) + "\n")
