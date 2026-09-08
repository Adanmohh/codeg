async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4329/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__, cases = [], checks = []
  const scope = page.getByTestId("bug-workflow")
  await page.setViewportSize({ width: 390, height: 844 })
  await page.keyboard.press("Escape")
  await page.locator('[data-slot="drawer-popup"]').waitFor({ state: "detached" })
  const shot = async name => {
    const geometry = await page.evaluate(() => ({ popupCount: document.querySelectorAll('[data-slot="drawer-popup"]').length, width: innerWidth, documentWidth: document.documentElement.scrollWidth }))
    if (geometry.popupCount !== 0) throw Error("Drawer not detached")
    await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/${name}.png` })
    cases.push({ name, raw: await capture(page) })
    checks.push({ name, geometry })
  }
  await scope.getByRole("heading", { name: "Required evidence", exact: true }).scrollIntoViewIfNeeded()
  await shot("intake-missing-390-light")
  await scope.getByLabel("Sanitized proof content", { exact: true }).scrollIntoViewIfNeeded()
  await shot("intake-invalid-proof-retained-390-light")
  checks.push({ retained: await scope.getByLabel("Sanitized proof content", { exact: true }).inputValue(), prepareDisabled: await scope.getByRole("button", { name: "Prepare exact issue", exact: true }).isDisabled() })
  return { cases, checks, method: "Settled unobscured replacements; original mid-dismissal captures retained under attempts; no provider or draft action" }
}
