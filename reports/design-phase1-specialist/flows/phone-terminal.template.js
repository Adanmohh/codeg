async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4328/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__, cases = [], checks = []
  const stats = () => page.evaluate(async () => (await fetch("/__issue_fixture/stats", { headers: { Authorization: "Bearer ops-issue-phone-synthetic-operator" } })).json())
  const shot = async name => {
    await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/${name}.png` })
    cases.push({ name, raw: await capture(page) })
  }
  for (const [name, notice] of [["unknown", "42d30251-362d-4296-ae88-7de8ac42d7b2"], ["rejected", "f646d07a-ac13-4a27-a301-fd7404bfd650"]]) {
    await page.evaluate(async () => {
      const r = await fetch("/__issue_fixture/refresh", { method: "POST", headers: { Authorization: "Bearer ops-issue-phone-synthetic-operator" } })
      if (r.status !== 204) throw Error("Controlled freshness failed")
    })
    await page.goto(`http://127.0.0.1:4328/ops-review?notice=${notice}`)
    await page.getByRole("heading", { name: "Review GitHub issue", exact: true }).waitFor()
    const before = await stats()
    const confirm = page.getByRole("checkbox", { name: /I reviewed this exact repository/ })
    await confirm.focus()
    await page.keyboard.press("Space")
    await page.keyboard.press("Tab")
    await page.waitForTimeout(350)
    const focused = await page.evaluate(() => ({ text: document.activeElement?.textContent, focusVisible: document.activeElement?.matches(":focus-visible"), shadow: getComputedStyle(document.activeElement).boxShadow, border: getComputedStyle(document.activeElement).borderColor }))
    if (!focused.text?.includes("Approve and file issue") || !focused.focusVisible) throw Error("Keyboard approval focus missing")
    await shot(`phone-${name}-focus-390-dark`)
    await page.keyboard.press("Enter")
    await page.getByRole("heading", { name: "Filing receipt", exact: true }).waitFor()
    await page.getByRole("heading", { name: "Filing receipt", exact: true }).scrollIntoViewIfNeeded()
    await shot(`phone-${name}-390-dark`)
    const after = await stats()
    if (after.githubPosts !== before.githubPosts + 1 || after.telegramSends !== before.telegramSends) throw Error("Unexpected provider counts")
    checks.push({ name, before, after, focused, receipt: await page.locator("section").filter({ has: page.getByRole("heading", { name: "Filing receipt", exact: true }) }).innerText() })
    if (name === "unknown") {
      await page.getByRole("button", { name: "Check existing issue", exact: true }).click()
      await page.getByText("Created issue", { exact: false }).waitFor()
      await shot("phone-reconciled-390-dark")
      const reconciled = await stats()
      if (reconciled.githubPosts !== after.githubPosts || reconciled.telegramSends !== after.telegramSends) throw Error("Reconciliation resent")
      checks.push({ name: "read-only issue reconciliation", before: after, after: reconciled })
    }
  }
  return { cases, checks, method: "Real protected issue decisions and read-only reconciliation, accepted loopback fixtures only; no request interception" }
}
