async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4328/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__
  const cases = [], checks = []
  await page.setViewportSize({ width: 390, height: 844 })
  await page.getByLabel("Access Token", { exact: true }).waitFor()
  await page.screenshot({ path: "reports/design-phase1-specialist/screenshots/phone-login-390.png" })
  cases.push({ name: "phone-login-390", raw: await capture(page) })
  await page.getByLabel("Access Token", { exact: true }).fill("invalid-fixture-only")
  await page.getByRole("button", { name: "Connect", exact: true }).click()
  await page.locator("#login-error").waitFor()
  const invalid = await page.getByLabel("Access Token", { exact: true }).evaluate(el => ({ invalid: el.getAttribute("aria-invalid"), description: el.getAttribute("aria-describedby"), error: document.getElementById("login-error")?.textContent }))
  await page.screenshot({ path: "reports/design-phase1-specialist/screenshots/phone-login-invalid-390.png" })
  cases.push({ name: "phone-login-invalid-390", raw: await capture(page) })
  const setup = await page.evaluate(async () => {
    const headers = { Authorization: "Bearer ops-issue-phone-synthetic-operator" }
    const before = await (await fetch("/__issue_fixture/stats", { headers })).json()
    const refresh = await fetch("/__issue_fixture/refresh", { method: "POST", headers })
    return { before, refresh: refresh.status }
  })
  await page.getByLabel("Access Token", { exact: true }).fill("ops-issue-phone-synthetic-operator")
  await page.getByRole("button", { name: "Connect", exact: true }).click()
  await page.getByRole("heading", { name: "Review GitHub issue", exact: true }).waitFor()
  await page.screenshot({ path: "reports/design-phase1-specialist/screenshots/phone-login-success-390.png" })
  cases.push({ name: "phone-login-success-390", raw: await capture(page) })
  const after = await page.evaluate(async () => (await fetch("/__issue_fixture/stats", { headers: { Authorization: "Bearer ops-issue-phone-synthetic-operator" } })).json())
  checks.push({ check: "BC-1", invalid, retryWorked: page.url().includes("/ops-review?notice="), fixtureSetup: setup, after, realProtectedAPI: true })
  return { cases, checks, method: "Actual invalid/valid human login; controlled freshness only; no provider decisions or request interception" }
}
