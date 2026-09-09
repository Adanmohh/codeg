// Playwright CLI0.1.18 run-code function; installed core1.63.0-alpha-2026-08-05.
// Use only the root-intake-unified session with its already authenticated page.
async page => {
  const results = []
  const sentinel = "Root browser sentinel: retain this draft across tabs and panes."
  for (const language of ["en", "ar"]) {
    for (const theme of ["light", "dark"]) {
      for (const width of [390, 768, 1280]) {
        await page.setViewportSize({ width: 1280, height: 900 })
        await page.locator('select:has(option[value="ar"])').selectOption(language)
        await page.locator('select:has(option[value="dark"])').selectOption(theme)
        await page.setViewportSize({ width, height: 900 })
        await page.waitForTimeout(500)
        const measured = await page.evaluate(sentinel => ({
          viewport: innerWidth,
          documentWidth: document.documentElement.scrollWidth,
          direction: document.documentElement.dir,
          draftRetained: [...document.querySelectorAll("textarea")].some(x => x.value === sentinel),
          storedPrivateDraft: Object.values(localStorage).concat(Object.values(sessionStorage)).some(x => x.includes(sentinel)),
          visiblePanels: [...document.querySelectorAll('[role="tabpanel"]')].filter(x => x.getBoundingClientRect().width > 0 && getComputedStyle(x).visibility !== "hidden").length,
        }), sentinel)
        const screenshot = `reports/business-intake-root-evidence/matrix-${language}-${theme}-${width}.png`
        await page.screenshot({ path: screenshot, scale: "css" })
        results.push({ language, theme, width, ...measured, screenshot })
      }
    }
  }
  await page.setViewportSize({ width: 1280, height: 900 })
  await page.locator('select:has(option[value="ar"])').selectOption("en")
  await page.locator('select:has(option[value="dark"])').selectOption("light")
  return { cases: results.length, passed: results.every(x => x.draftRetained && !x.storedPrivateDraft && x.documentWidth <= x.viewport), results }
}
