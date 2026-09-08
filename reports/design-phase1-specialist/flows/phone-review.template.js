async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4328/ops-review")) throw Error("Owned fixture only")
  const capture = __CAPTURE__, cases = [], checks = []
  const appearance = page.context().pages().find(p => p.url().endsWith("4328/settings/appearance"))
  if (!appearance) throw Error("Actual Appearance tab required")
  const shot = async name => {
    await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/${name}.png` })
    cases.push({ name, raw: await capture(page) })
  }
  const approve = page.getByRole("button", { name: "Approve and file issue", exact: true })
  const body = page.getByLabel("Exact issue body", { exact: true })
  const fullBody = await body.textContent()
  if (!["## build", "## screen", "## reciter", "## log", "human-confirmed"].every(t => fullBody.includes(t))) throw Error("Incomplete issue projection")
  if (!(await approve.isDisabled())) throw Error("Human checkbox bypass")
  for (const [width, height] of [[1280, 900], [390, 844]]) {
    await page.setViewportSize({ width, height })
    for (const theme of ["light", "dark"]) {
      await appearance.getByRole("combobox").first().click()
      await appearance.getByRole("option", { name: theme === "dark" ? "Dark" : "Light", exact: true }).click()
      await page.waitForFunction(t => document.documentElement.classList.contains(t) && document.documentElement.style.colorScheme === t, theme)
      await page.waitForTimeout(300)
      await page.getByRole("heading", { name: "Review GitHub issue", exact: true }).scrollIntoViewIfNeeded()
      await shot(`phone-review-${width}-${theme}-top`)
      await page.getByRole("heading", { name: "Exact GitHub issue", exact: true }).scrollIntoViewIfNeeded()
      await body.evaluate(el => { el.scrollTop = 0 })
      await shot(`phone-review-${width}-${theme}-body`)
      await approve.scrollIntoViewIfNeeded()
      await body.evaluate(el => { el.scrollTop = el.scrollHeight })
      await shot(`phone-review-${width}-${theme}-decision`)
    }
  }
  checks.push({ check: "BC-17", fullBody, repository: "owner/repo", title: await page.getByLabel("Issue title", { exact: true }).inputValue(), labels: await page.getByLabel("Repository labels", { exact: true }).inputValue(), severity: await page.getByRole("combobox", { name: "Human-confirmed severity", exact: true }).inputValue(), approvalRequiresConfirmation: await approve.isDisabled() })
  const headers = { Authorization: "Bearer ops-issue-phone-synthetic-operator" }
  const stats = () => page.evaluate(async headers => (await fetch("/__issue_fixture/stats", { headers })).json(), headers)
  const before = await stats()
  const confirm = page.getByRole("checkbox", { name: /I reviewed this exact repository/ })
  await confirm.focus()
  await page.keyboard.press("Space")
  await page.keyboard.press("Tab")
  const focused = await page.evaluate(() => ({ text: document.activeElement?.textContent, focusVisible: document.activeElement?.matches(":focus-visible"), shadow: getComputedStyle(document.activeElement).boxShadow }))
  if (!focused.text?.includes("Approve and file issue") || !focused.focusVisible) throw Error("Keyboard approval focus missing")
  await shot("phone-confirmed-keyboard-390-dark")
  await page.keyboard.press("Enter")
  await page.getByRole("heading", { name: "Filing receipt", exact: true }).waitFor()
  await page.getByText("Created issue", { exact: false }).waitFor()
  await page.getByRole("heading", { name: "Filing receipt", exact: true }).scrollIntoViewIfNeeded()
  await shot("phone-created-390-dark")
  const after = await stats()
  if (after.githubPosts !== before.githubPosts + 1 || after.githubIssues !== before.githubIssues + 1 || after.telegramSends !== before.telegramSends) throw Error("Unexpected provider counts")
  checks.push({ check: "BC-17-keyboard-decision", before, after, focused, noInterception: true, syntheticProvidersOnly: true })
  await page.reload()
  await page.getByRole("heading", { name: "This review link is no longer current", exact: true }).waitFor()
  await shot("phone-used-link-390-dark")
  checks.push({ check: "used locator", unavailable: true, afterReload: await stats() })
  await page.getByRole("button", { name: "Open workspace", exact: true }).click()
  await page.waitForURL("**/workspace")
  checks.push({ check: "phone exit copy", label: "Open workspace", destination: page.url() })
  return { cases, checks, method: "Actual protected phone UI, real theme selection, keyboard exact-payload decision; synthetic providers only" }
}
