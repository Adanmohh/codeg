// Pure Design Studio analysis of actual CLI captures. No browser launcher.
import fs from "node:fs"
import { parseBrief } from "/Users/mohamedadan/projects/design-studio/scripts/brief.mjs"
import { buildReport, countUnnamedInteractive } from "/Users/mohamedadan/projects/design-studio/lab/tools/probe.mjs"

const dir = "reports/design-phase1-specialist/probes/"
const original = parseBrief(fs.readFileSync("docs/design/BRIEF.html", "utf8"))
const derived = JSON.parse(fs.readFileSync(dir + "brief-canvas.json", "utf8")).derived
const data = []
for (const theme of ["light", "dark"]) {
  const raws = ["no-preference", "reduce"].map(preference =>
    JSON.parse(fs.readFileSync(dir + `final-drawer-settled-${theme}-${preference}.raw.json`, "utf8")))
  // Exact probe.mjs:166 threshold; preserve its result despite its known limits.
  raws[1].reducedMotionEffective = raws[1].transitionsCount === 0 ||
    raws[1].transitionsCount < raws[0].transitionsCount * 0.2
  const pair = []
  for (const [index, raw] of raws.entries()) {
    raw.aria.unnamedInteractive = countUnnamedInteractive(raw.aria.snapshot)
    const normalized = buildReport(raw, derived), authored = buildReport(raw, original)
    const stem = `final-drawer-settled-${theme}-${index === 0 ? "no-preference" : "reduce"}`
    fs.writeFileSync(dir + stem + ".normalized-brief.json", JSON.stringify(normalized, null, 2) + "\n")
    fs.writeFileSync(dir + stem + ".original-brief.json", JSON.stringify(authored, null, 2) + "\n")
    pair.push({ stem, checked: normalized.contrast.checked, failures: normalized.contrast.failures,
      unnamed: normalized.aria.unnamedInteractive, overflow: raw.documentWidth > raw.viewport.width,
      transitionsCount: raw.transitionsCount, visibleTransitions: raw.visibleTransitionCount,
      properties: [...new Set(raw.transitions.filter(t => t.visible).map(t => t.properties))], motion: normalized.motion })
  }
  data.push({ theme, pair })
}
fs.writeFileSync(dir + "final-drawer-measured-summary.json", JSON.stringify({
  method: "Same mounted DOM under both preferences. Raw duration-count heuristic remains false; actual lifecycle frames are authoritative for panel travel.",
  actualMotionEvidence: "../flows/final-drawer-summary.json",
  cases: data,
}, null, 2) + "\n")
console.log(JSON.stringify(data.map(d => ({ theme: d.theme, counts: d.pair.map(p => p.transitionsCount),
  checked: d.pair.map(p => p.checked), failures: d.pair.map(p => p.failures.length),
  effective: d.pair[1].motion.reducedMotionEffective }))))
