// Report-only reuse of accepted PR20 CLI verification. Exact provenance in NOTICE.
import fs from "node:fs"
const dir = "reports/design-phase1-specialist/flows/"
const stats = `const stats = () => page.evaluate(async () => (await fetch("/api/ops_design_fixture_stats", {
    headers: { Authorization: "Bearer ops-design-synthetic-operator" },
  })).json())`
let verify = fs.readFileSync("reports/design-login-landmark-evidence/verify.js", "utf8")
  .replaceAll("4325", "4327")
  .replaceAll("pi-desk-browser-fixture", "ops-design-synthetic-operator")
  .replaceAll("reports/design-login-landmark-evidence/${width}", "reports/design-phase1-specialist/screenshots/final-login-${width}")
  .replace("// The token below is public synthetic test data from pi_desk_browser_fixture.", "// Public synthetic token from the already running owned4327 fixture.")
  .replace('await page.goto(`${origin}/login`)', `await page.emulateMedia({ colorScheme: "light" })
  await page.goto(\u0060\u0024{origin}/login\u0060)
  ${stats}
  const providerBefore = await stats()
  assert(providerBefore.syntheticOnly && providerBefore.providerRequests === 4, "Owned fixture state")`)
  .replace("const guard = { ...page.context().drawerMotionEvidence }", `const providerAfter = await stats()
  assert(JSON.stringify(providerBefore) === JSON.stringify(providerAfter), "No provider/fixture mutation")
  const guard = { ...page.context().drawerMotionEvidence }`)
  .replace("    guard,\n    assertions", "    guard,\n    providerBefore,\n    providerAfter,\n    assertions")
fs.writeFileSync(dir + "final-login.js", verify)
const guard = fs.readFileSync("reports/design-reduced-motion-evidence/guard.js", "utf8")
  .replaceAll("4325", "4327")
  .replace("fresh local DB", "existing owned loopback DB")
  .replace('if (/^api\\/(set_', 'if (path !== "api/ops_design_fixture_stats" && /^api\\/(set_')
  .replace("Owned motion fixture guarded", "Owned login recheck guarded")
fs.writeFileSync(dir + "final-login-guard.js", guard)
