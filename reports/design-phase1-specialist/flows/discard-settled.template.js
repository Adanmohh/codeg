async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__, scope = page.getByRole("region", { name: "Ops desk", exact: true })
  await page.waitForTimeout(350)
  const body = scope.getByLabel("Reply message", { exact: true })
  if (await body.inputValue() !== "Final focus leave-guard sentinel; synthetic and unsaved.") throw Error("Discard cancellation lost text")
  const raw = await capture(page)
  await page.screenshot({ path: "reports/design-phase1-specialist/screenshots/discard-cancel-settled-1280-dark.png" })
  const after = await page.evaluate(async () => (await fetch("/api/ops_design_fixture_stats", { headers: { Authorization: "Bearer ops-design-synthetic-operator" } })).json())
  await body.fill("Bounded design draft: all text remains after validation.")
  return { cases: [{ name: "discard-cancel-settled-1280-dark", raw }], checks: [{ check: "BC-9", retainedText: true, focus: raw.focus, after, restoredSavedValueWithoutSave: true }], method: "Separate actual CLI native confirm/dismiss commands; settled post-cancel capture; no draft save or provider action" }
}
