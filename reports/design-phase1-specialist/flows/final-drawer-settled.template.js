async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__, cases = [], checks = []
  const appearance = page.context().pages().find(p => p.url().endsWith("4327/settings/appearance"))
  if (!appearance) throw Error("Owned Appearance tab required")
  const stats = () => page.evaluate(async () => (await fetch("/api/ops_design_fixture_stats", {
    headers: { Authorization: "Bearer ops-design-synthetic-operator" },
  })).json())
  const before = await stats()
  await page.setViewportSize({ width: 390, height: 844 })
  for (const theme of ["light", "dark"]) {
    await appearance.getByRole("combobox").first().click()
    await appearance.getByRole("option", { name: theme === "dark" ? "Dark" : "Light", exact: true }).click()
    await page.waitForFunction(t => document.documentElement.classList.contains(t), theme)
    await page.emulateMedia({ reducedMotion: "reduce" })
    await page.getByRole("button", { name: "Show Sidebar", exact: true }).focus()
    await page.keyboard.press("Enter")
    await page.waitForFunction(() => {
      const el = document.querySelector('[data-slot="drawer-popup"]')
      return el && !el.hasAttribute("data-starting-style") && !el.getAnimations().length
    })
    await page.keyboard.press("Tab")
    await page.waitForTimeout(350) // Settled focus paint, beyond the shared-control transition.
    for (const preference of ["no-preference", "reduce"]) {
      await page.emulateMedia({ reducedMotion: preference })
      await page.waitForTimeout(350)
      const name = `final-drawer-settled-${theme}-${preference}`
      await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/${name}.png` })
      cases.push({ name, raw: await capture(page) })
    }
    const focus = await page.evaluate(() => {
      const e = document.activeElement, style = getComputedStyle(e)
      return { label: e.getAttribute("aria-label"), inDrawer: !!e.closest('[data-slot="drawer-popup"]'),
        focusVisible: e.matches(":focus-visible"), shadow: style.boxShadow,
        border: style.borderColor, outline: style.outline }
    })
    if (!focus.inDrawer || !focus.focusVisible || !focus.shadow.includes("3px")) throw Error("Settled focus missing")
    await page.keyboard.press("Escape")
    await page.locator('[data-slot="drawer-popup"]').waitFor({ state: "detached" })
    checks.push({ theme, focus, detached: true })
  }
  const after = await stats()
  if (JSON.stringify(before) !== JSON.stringify(after)) throw Error("Fixture changed")
  return { cases, checks, before, after, method: "Settled keyboard paint and paired same-DOM CSS inventory; motion verdict uses separate actual rAF lifecycle evidence" }
}
