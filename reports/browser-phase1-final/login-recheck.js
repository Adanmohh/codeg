// Root report-only CLI check against the final protected local export.
// Public synthetic operator token only; no agent/configuration inspection.
async (page) => {
  const results = []
  for (const width of [390, 1280]) {
    await page.setViewportSize({ width, height: 900 })
    await page.goto("http://127.0.0.1:4318/login")
    const main = page.getByRole("main")
    await main.waitFor()
    if (await main.count() !== 1) throw Error("Expected exactly one main")
    await main.getByRole("heading", { name: "Hafidh Ops Desk", exact: true }).waitFor()
    if (await main.locator("form").count() !== 1) throw Error("Form outside main")
    const token = main.getByLabel("Access Token", { exact: true })
    const connect = main.getByRole("button", { name: "Connect", exact: true })
    if (!await connect.isDisabled()) throw Error("Empty token enabled")
    await token.fill("ops-desk-deliberately-invalid")
    await connect.click()
    await page.locator("#login-error").waitFor()
    if (await token.getAttribute("aria-invalid") !== "true" || await token.getAttribute("aria-describedby") !== "login-error") throw Error("Error association lost")
    await page.screenshot({ path: `reports/browser-phase1-final/login-invalid-${width}.png` })
    const geometry = await main.evaluate(el => ({
      viewport: innerWidth,
      document: document.documentElement.scrollWidth,
      mainWidth: el.getBoundingClientRect().width,
    }))
    if (geometry.document !== width) throw Error("Login horizontal overflow")
    await token.fill("ops-desk-local-test")
    await connect.click()
    await page.waitForURL("http://127.0.0.1:4318/workspace")
    results.push({ width, mainCount: 1, formInsideMain: true, associatedInvalidError: true, retryNavigated: true, geometry })
  }
  return { method: "Actual root Playwright CLI, protected local health/login, synthetic token only", results }
}
