import fs from "node:fs"
const dir = "reports/design-phase1-specialist/lint/"
const all = fs.readdirSync(dir).filter(f => f.startsWith("measured-") && f.endsWith(".txt")).map(file => {
  const text = fs.readFileSync(dir + file, "utf8")
  const report = JSON.parse(text.slice(0, text.indexOf("\n\n")))
  return { file, exit: 0, source: report.file, missing: report.missing, signals: report.signals, probe: report.probe }
})
fs.writeFileSync(dir + "measured-index.json", JSON.stringify(all, null, 2) + "\n")
console.log(JSON.stringify(all.map(r => ({ file: r.file, flags: r.missing.length }))))
