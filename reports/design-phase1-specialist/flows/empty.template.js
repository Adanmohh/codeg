async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4330/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__, cases = [], checks = []
  const shot = async name => {
    await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/${name}.png` })
    cases.push({ name, raw: await capture(page) })
  }
  await page.setViewportSize({ width: 1280, height: 900 })
  await page.getByLabel("Access Token", { exact: true }).fill("ops-intake-synthetic-operator")
  await page.getByRole("button", { name: "Connect", exact: true }).click()
  await page.getByRole("button", { name: "Ops desk", exact: true }).click()
  const scope = page.getByRole("region", { name: "Ops desk", exact: true })
  await scope.getByRole("button", { name: "Morning", exact: true }).waitFor()
  await page.waitForTimeout(350)
  await shot("empty-inbox-1280-light")
  checks.push({ check: "BC-6-empty", copy: await scope.innerText(), method: "Accepted host fixture EMPTY=1, zero local tickets and no configured products; real protected Ops API" })
  await scope.getByRole("button", { name: "Morning", exact: true }).click()
  await scope.getByRole("heading", { name: "What needs your attention", exact: true }).waitFor()
  await shot("empty-morning-1280-light")
  const copy = await scope.innerText()
  if (!copy.includes("No pending reply proposals.") || !copy.includes("No tasks in this queue.") || !copy.includes("No open correspondence.")) throw Error("Dishonest empty morning")
  checks.push({ check: "BC-15-empty", copy })
  await page.setViewportSize({ width: 390, height: 844 })
  await page.keyboard.press("Escape")
  await shot("empty-morning-390-light")
  return { cases, checks, method: "Real protected empty data, no provider activity or browser response interception" }
}
