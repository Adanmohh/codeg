async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__, cases = [], checks = []
  const appearance = page.context().pages().find(p => p.url().endsWith("4327/settings/appearance"))
  const scope = page.getByRole("region", { name: "Ops desk", exact: true })
  const shot = async name => {
    await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/${name}.png` })
    cases.push({ name, raw: await capture(page) })
  }
  await page.setViewportSize({ width: 1280, height: 900 })
  const show = page.getByRole("button", { name: /^Show Sidebar/ })
  if (await show.count()) await show.click()
  for (const theme of ["light", "dark"]) {
    await appearance.getByRole("combobox").first().click()
    await appearance.getByRole("option", { name: theme === "dark" ? "Dark" : "Light", exact: true }).click()
    await page.waitForFunction(t => document.documentElement.classList.contains(t) && document.documentElement.style.colorScheme === t, theme)
    const nav = page.getByRole("button", { name: "Ops desk", exact: true })
    await nav.focus()
    await page.keyboard.press("Tab")
    await page.keyboard.press("Shift+Tab")
    await page.waitForTimeout(350)
    await shot(`navigation-focus-1280-${theme}`)
    checks.push({ check: "BC-2", theme, activeOpsVisible: await scope.isVisible(), focused: await nav.evaluate(e => ({ focused: document.activeElement === e, visible: e.matches(":focus-visible"), outline: getComputedStyle(e).outline, shadow: getComputedStyle(e).boxShadow, background: getComputedStyle(e).backgroundColor, ink: getComputedStyle(e).color })) })
  }
  await page.setViewportSize({ width: 390, height: 844 })
  await page.keyboard.press("Escape")
  await scope.getByRole("button", { name: "Inbox", exact: true }).click()
  const back = scope.getByRole("button", { name: "Back to inbox", exact: true })
  if (await back.count()) await back.click()
  for (const title of ["A real local thread", "Synthetic locale review"]) {
    await scope.getByRole("button", { name: new RegExp(`Reader Open ${title}`) }).click()
    await scope.getByRole("heading", { name: title, exact: true }).waitFor()
    await shot(`selection-${title.startsWith("A real") ? "original" : "locale"}-390-dark`)
    checks.push({ check: "BC-4", title, messages: await scope.getByLabel("Conversation messages", { exact: true }).innerText() })
    if (title.startsWith("A real")) await back.click()
  }
  await page.setViewportSize({ width: 1280, height: 900 })
  await scope.getByRole("button", { name: "Reply draft", exact: true }).click()
  await scope.getByLabel("Reply message", { exact: true }).fill("Final focus leave-guard sentinel; synthetic and unsaved.")
  await scope.getByRole("button", { name: "Morning", exact: true }).focus()
  await page.keyboard.press("Tab")
  await page.keyboard.press("Shift+Tab")
  await page.waitForTimeout(350)
  return { cases, checks, next: "Use CLI press Enter then dialog-dismiss and capture settled focus; never save this sentinel" }
}
