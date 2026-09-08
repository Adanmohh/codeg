async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4330/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__
  await page.locator('[data-slot="drawer-popup"]').waitFor({ state: "detached" })
  const geometry = await page.evaluate(() => ({ popupCount: document.querySelectorAll('[data-slot="drawer-popup"]').length, width: innerWidth, documentWidth: document.documentElement.scrollWidth }))
  await page.screenshot({ path: "reports/design-phase1-specialist/screenshots/empty-morning-390-light.png" })
  return { cases: [{ name: "empty-morning-390-light", raw: await capture(page) }], checks: [{ geometry }], method: "Detached popup, settled empty mobile replacement; no mutation" }
}
