import fs from "node:fs"
import { parseBrief } from "/Users/mohamedadan/projects/design-studio/scripts/brief.mjs"
const base = "reports/design-phase1-specialist/probes/"
const original = parseBrief(fs.readFileSync("docs/design/BRIEF.html", "utf8"))
const adapter = fs.readFileSync(base + "normalize-brief.js", "utf8")
fs.writeFileSync(base + "browser-brief-conversion.js", adapter.replace('JSON.parse(document.querySelector("#design-state").textContent)', JSON.stringify(original)))
const flows = "reports/design-phase1-specialist/flows/"
for (const file of fs.readdirSync(flows).filter((f) => f.endsWith(".template.js"))) {
  fs.writeFileSync(flows + file.replace(".template", ""), fs.readFileSync(flows + file, "utf8").replace("__CAPTURE__", fs.readFileSync(base + "capture.js", "utf8")))
}
