// Report-only adaptations of accepted CLI evidence; exact commits/blobs in NOTICE.md.
import fs from "node:fs"

const dir = "reports/design-phase1-specialist/flows/"
const shots = "reports/design-phase1-specialist/screenshots/"
const appearance = `const appearance = page.context().pages().find(p => p.url().endsWith("4327/settings/appearance"))
  if (!appearance) throw Error("Owned Appearance tab required")`
const setTheme = `await appearance.getByRole("combobox").first().click()
    await appearance.getByRole("option", { name: theme === "dark" ? "Dark" : "Light", exact: true }).click()`

let badge = fs.readFileSync("reports/browser-phase1-final/badge-recheck.js", "utf8")
  .replaceAll("4326", "4327")
  .replace("// Root rerun", "// Specialist final rerun")
  .replace('if (themeSetting && themeSetting !== "system") throw Error("Requires existing Follow system preference")', appearance)
  .replace('await page.setViewportSize({ width: 1280, height: 900 })', `await page.setViewportSize({ width: 1280, height: 900 })
  const show = page.getByRole("button", { name: /^Show Sidebar/ })
  if (await show.count()) await show.click()
  if (!await page.getByRole("region", { name: "Ops desk", exact: true }).count()) {
    await page.getByRole("button", { name: "Ops desk", exact: true }).click()
    await page.getByRole("region", { name: "Ops desk", exact: true }).waitFor()
  }`)
  .replace('await page.emulateMedia({ colorScheme: theme })', setTheme)
  .replace('await page.emulateMedia({ colorScheme: "light" })', '// Leave the last actual Appearance preference in place.')
  .replaceAll("reports/browser-phase1-final/badge-", shots + "final-badge-")
fs.writeFileSync(dir + "final-badge.js", badge)

for (const phase of ["setup", "cleanup"]) {
  const code = fs.readFileSync(`reports/design-running-badge-group-${phase}.playwright`, "utf8").replaceAll("4326", "4327")
  fs.writeFileSync(dir + `final-group-${phase}.js`, code)
}

let drawer = fs.readFileSync("reports/browser-phase1-final/drawer-recheck.js", "utf8")
  .replaceAll("4318", "4327")
  .replaceAll("Root", "Specialist")
  .replace('const result = { viewport: page.viewportSize(), cases: [] }', 'const theme = await page.evaluate(() => document.documentElement.classList.contains("dark") ? "dark" : "light")\n  const result = { theme, viewport: page.viewportSize(), cases: [] }')
  .replaceAll("reports/browser-phase1-final/${stage}", shots + "final-drawer-${theme}-${stage}")
// Both themes use actual settings controls, not a localStorage write or CSS override.
fs.writeFileSync(dir + "final-drawer.js", `async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Owned fixture only")
  ${appearance}
  const stats = () => page.evaluate(async () => (await fetch("/api/ops_design_fixture_stats", {
    headers: { Authorization: "Bearer ops-design-synthetic-operator" },
  })).json())
  const before = await stats(), results = []
  if (!before.syntheticOnly || before.providerRequests !== 4) throw Error("Fixture changed")
  const measure = ${drawer}
  for (const theme of ["light", "dark"]) {
    ${setTheme}
    await page.waitForFunction(t => document.documentElement.classList.contains(t), theme)
    results.push(await measure(page))
  }
  const after = await stats()
  if (JSON.stringify(before) !== JSON.stringify(after)) throw Error("Fixture changed")
  return { before, after, results }
}\n`)
