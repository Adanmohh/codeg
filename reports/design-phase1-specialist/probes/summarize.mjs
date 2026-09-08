import fs from "node:fs"
import { parseBrief, renderBrief } from "/Users/mohamedadan/projects/design-studio/scripts/brief.mjs"
import { buildReport, countUnnamedInteractive } from "/Users/mohamedadan/projects/design-studio/lab/tools/probe.mjs"
const dir = "reports/design-phase1-specialist/probes/"
const original = parseBrief(fs.readFileSync("docs/design/BRIEF.html", "utf8"))
const canvas = JSON.parse(fs.readFileSync(dir + "brief-canvas.json", "utf8"))
fs.writeFileSync(dir + "BRIEF.measurement.html", renderBrief(canvas.derived))
const summary = []
for (const file of fs.readdirSync(dir).filter((f) => f.endsWith(".raw.json"))) {
  const raw = JSON.parse(fs.readFileSync(dir + file, "utf8"))
  raw.aria.unnamedInteractive = countUnnamedInteractive(raw.aria.snapshot)
  const asAuthored = buildReport(raw, original)
  const normalized = buildReport(raw, canvas.derived)
  const stem = file.replace(".raw.json", "")
  fs.writeFileSync(dir + stem + ".original-brief.json", JSON.stringify(asAuthored, null, 2) + "\n")
  fs.writeFileSync(dir + stem + ".normalized-brief.json", JSON.stringify(normalized, null, 2) + "\n")
  const hex = (rgb) => "#" + rgb.slice(0, 3).map((n) => Math.round(n).toString(16).padStart(2, "0")).join("")
  const opaqueTokens = raw.tokens.filter((t) => t.rgba[3] === 1).map((t) => ({ name: t.name, css: t.css, hex: hex(t.rgba) }))
  const remainingPalette = normalized.briefViolations.filter((v) => v.kind === "off-palette-color").map((v) => {
    const color = v.detail.split(" ")[0]
    const tokens = opaqueTokens.filter((t) => t.hex === color)
    const evidence = raw.colorEvidence.filter((e) => hex(e.composed.match(/\d+/g).map(Number)) === color)
    return { color, classification: tokens.length ? (raw.theme.class.includes("dark") ? "paired-dark-token; not present in original light palette" : "inherited token omitted from brief palette") : evidence.some((e) => e.alpha < 1) ? "alpha-composite; inspect token/class/backdrop evidence" : "inspect exact conversion or inherited source", tokens, evidence: evidence.slice(0, 5) }
  })
  summary.push({ file, url: raw.url, viewport: raw.viewport, theme: raw.theme, checked: normalized.contrast.checked, failures: normalized.contrast.failures, unnamed: normalized.aria.unnamedInteractive, headings: raw.aria.headings, overflow: raw.documentWidth > raw.viewport.width, transitions: { all: raw.transitionsCount, visible: raw.visibleTransitionCount, underReduce: raw.reducedMotion }, exclusions: raw.exclusions, originalBriefFlags: asAuthored.briefViolations, normalizedBriefFlags: normalized.briefViolations, remainingPalette })
}
fs.writeFileSync(dir + "summary.json", JSON.stringify(summary, null, 2) + "\n")
console.log(JSON.stringify(summary.map(({ file, checked, failures, unnamed, overflow, remainingPalette }) => ({ file, checked, failures, unnamed, overflow, remainingPalette: remainingPalette.length })), null, 2))
