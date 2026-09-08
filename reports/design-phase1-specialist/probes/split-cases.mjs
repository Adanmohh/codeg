import fs from "node:fs"
const result = JSON.parse(fs.readFileSync(process.argv[2], "utf8"))
for (const { name, raw } of result.cases) fs.writeFileSync(`reports/design-phase1-specialist/probes/${name}.raw.json`, JSON.stringify(raw) + "\n")
console.log(JSON.stringify({ cases: result.cases.length, checks: result.checks }))
