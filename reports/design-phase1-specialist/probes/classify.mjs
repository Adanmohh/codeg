// Evidence interpretation only; never changes BRIEF or a measured raw capture.
import fs from "node:fs"
import { parseBrief } from "/Users/mohamedadan/projects/design-studio/scripts/brief.mjs"
import { buildReport } from "/Users/mohamedadan/projects/design-studio/lab/tools/probe.mjs"
const dir = "reports/design-phase1-specialist/probes/"
const read = file => JSON.parse(fs.readFileSync(dir + file, "utf8"))
const summary = read("summary.json")
const raw = summary.map(item => ({ file: item.file, data: read(item.file) }))
const original = parseBrief(fs.readFileSync("docs/design/BRIEF.html", "utf8"))
const normalized = read("brief-canvas.json").derived
const normal = read("motion-no-preference-open.raw.json")
const reduce = read("motion-reduce-open.raw.json")
const motion = {
  method: "Design Studio probe.mjs:166 threshold applied to paired actual CLI captures; no browser SDK launcher",
  baseline: normal.transitionsCount,
  reduced: reduce.transitionsCount,
  visibleBaseline: normal.visibleTransitionCount,
  visibleReduced: reduce.visibleTransitionCount,
  reducedMotionEffective: reduce.transitionsCount === 0 || reduce.transitionsCount < normal.transitionsCount * 0.2,
  propertiesUnderReduce: [...new Set(reduce.transitions.filter(t => t.visible).map(t => t.properties))],
  interpretation: "Raw threshold is preserved. Nonspatial color/border/opacity transitions do not independently prove a spatial-motion defect. Actual drawer rAF/keyframes in flows/motion-frames.json establish the 341.5px/450ms finding."
}
for (const [name, brief] of [["original", original], ["normalized", normalized]]) {
  const report = buildReport({ ...reduce, reducedMotionEffective: motion.reducedMotionEffective }, brief)
  fs.writeFileSync(dir + `motion-paired-derived.${name}-brief.json`, JSON.stringify(report, null, 2) + "\n")
}
const flags = key => summary.reduce((out, s) => {
  for (const f of s[key]) out[f.kind] = (out[f.kind] || 0) + 1
  return out
}, {})
const selects = raw.flatMap(({ file, data }) => data.fields.filter(f => f.tag === "SELECT" && !f.valuePresent && f.painted).map(f => ({ file, selectedText: f.selectedText, inViewport: f.inViewport, disabled: f.disabled, labels: f.labels })))
const hiddenDetails = raw.flatMap(({ file, data }) => data.fields.filter(f => f.closedDetails).map(f => ({ file, labels: f.labels, painted: f.painted, inViewport: f.inViewport })))
const calibration = {
  originalBriefUnchanged: true,
  derivedBrief: "BRIEF.measurement.html; Canvas-converted opaque LIGHT token colors only, original conventions/families/radii retained",
  counts: { cases: summary.length, textSamples: summary.reduce((n, s) => n + s.checked, 0), originalFlags: flags("originalBriefFlags"), normalizedFlags: flags("normalizedBriefFlags") },
  classifications: [
    { category: "OKLCH string comparison", source: "Design Studio 55c8614d scripts/intake-scan.mjs:51 and lab/tools/probe.mjs:62", conclusion: "canonHex expands/lowercases strings only. BRIEF OKLCH tokens and rendered RGB hex are not comparable until Canvas conversion. No product color correction follows from that mismatch." },
    { category: "paired dark", source: "src/app/globals.css:136; BRIEF conventions", examples: ["--muted #262626", "--muted-foreground #a1a1a1", "--primary #9bd4c5"], conclusion: "Explicitly inherited paired dark tokens, absent from the light-only palette inventory. Retained flags are schema coverage limits." },
    { category: "alpha composite", source: "src/components/ui/{button,input,textarea}.tsx; src/components/ops/{inbox-view,ui}.tsx; raw colorEvidence/chain", examples: ["#1e1e1e bg-muted/70 over dark", "#151515 bg-input/30 over dark", "#eef2f2 bg-primary/8 over white", "#fffaf2 private-note amber500/5 over white", "#fbfbfb read-only:bg-muted/40"], conclusion: "Final paint combines existing token alpha with actual backdrop. These composed values are not separate authored tokens. Each raw has its CSS/classes/backdrop; 8-bit Canvas alpha quantization may shift a channel by 1." },
    { category: "font family scanner mismatch", source: "BRIEF type.families; src/app/globals.css:18; Design Studio lab/tools/probe.mjs:70", conclusion: "Brief records a short Inter Variable fallback STACK. Probe accepts a matching whole stack or primary in its brief set, but does not extract primary from brief stacks. Actual Inter Variable with platform fallbacks is the inherited intended font; no font swap indicated." },
    { category: "spacing/radius scan coverage", source: "src/app/globals.css:1017-1031; shared shadcn controls and explicit running badge", conclusion: "6/10/14/18/22/26px radii and 6px control gaps arise from inherited rem multipliers/spacing, not an added style system. Brief scan plus 4px heuristic is not exhaustive. The 5px badge radius is an explicit inherited class; no layout problem established from arithmetic alone." },
    { category: "real contrast", source: "sidebar-conversation-list.tsx:507; sidebar-folder-group-header.tsx:206; Tailwind4.1.18 theme.css amber tokens", conclusion: "Amber700 on amber500/12 yields 4.39 for 10px text. This is a real finding despite inherited token provenance. The sr-only duplicate is not a second painted defect. Root fix/recheck required." }
  ],
  selectedEmptyValueEvidence: selects,
  closedDetailsEvidence: { count: hiddenDetails.length, incorrectlyPainted: hiddenDetails.filter(f => f.painted), examples: hiddenDetails.slice(0, 12) },
  motion,
  measurementLimits: ["Viewport samples, not whole-page WCAG conformance", "Group opacity, disabled controls, background images and filters excluded explicitly", "Clipped sr-only text is retained in baseline raw; visually verified badge counted once", "No screen-reader session, WebKit/iOS native run or document-start CLS/long-frame measurement", "HTML themes and font settings remain user choices; raw lint flags preserved"]
}
fs.writeFileSync(dir + "classification.json", JSON.stringify(calibration, null, 2) + "\n")
console.log(JSON.stringify({ ...calibration.counts, emptyValuedSelects: selects.length, closedDetails: calibration.closedDetailsEvidence.count, incorrectlyPaintedDetails: calibration.closedDetailsEvidence.incorrectlyPainted.length, motion }, null, 2))
