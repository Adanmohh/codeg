async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__
  const appearance = page.context().pages().find((p) => p.url().endsWith("4327/settings/appearance"))
  if (!appearance) throw Error("Actual Appearance tab required")
  const scope = page.getByRole("region", { name: "Ops desk", exact: true })
  const cases = [], checks = []
  const shot = async (name) => {
    await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/${name}.png` })
    cases.push({ name, raw: await capture(page) })
  }
  for (const [name, view, row] of [
    ["thread", "Inbox", /Reader Open Synthetic locale review/],
    ["review", "Approvals", /Re: Synthetic locale review Task/],
  ]) {
    await page.setViewportSize({ width: 1280, height: 900 })
    await scope.getByRole("button", { name: view, exact: true }).click()
    await scope.getByRole("button", { name: row }).click()
    await scope.locator("article h1").waitFor()
    for (const [width, height] of [[1280, 900], [390, 844]]) {
      await page.setViewportSize({ width, height })
      if (width === 390) await page.keyboard.press("Escape")
      for (const theme of ["light", "dark"]) {
        await appearance.getByRole("combobox").first().click()
        await appearance.getByRole("option", { name: theme === "dark" ? "Dark" : "Light", exact: true }).click()
        await page.waitForFunction((t) => document.documentElement.classList.contains(t) && document.documentElement.style.colorScheme === t, theme)
        await page.waitForTimeout(350)
        await scope.locator("article h1").scrollIntoViewIfNeeded()
        await scope.locator("article h1").hover()
        await shot(`${name}-${width}-${theme}-top`)
        const summary = scope.locator("article summary").filter({ hasText: "Thread headers" })
        if (!(await summary.locator("..").getAttribute("open"))) {
          const open = await summary.evaluate((e) => e.parentElement.open)
          if (!open) await summary.click()
        }
        await scope.getByRole("textbox", { name: "References · one message ID per line", exact: true }).scrollIntoViewIfNeeded()
        await shot(`${name}-${width}-${theme}-headers`)
      }
    }
    checks.push({ surface: name, completePayloadAndOpenHeaders: true, viewportAndThemeCases: 4 })
  }
  await page.setViewportSize({ width: 1280, height: 900 })
  await scope.getByRole("button", { name: "Morning", exact: true }).click()
  await scope.getByRole("heading", { name: "What needs your attention" }).waitFor()
  await shot("morning-1280-dark")
  await scope.getByRole("button", { name: /Re: Synthetic locale review Task/ }).click()
  await scope.getByRole("heading", { name: "Pending human review", exact: true }).waitFor()
  checks.push({ morningProposalDestination: "real selected Pending human review card" })
  return { syntheticOnly: true, settings: "Actual Appearance controls, settled theme class/color scheme", checks, cases }
}
