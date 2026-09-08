// Root integration measurement adapted from PR19 report-only CLI probe.
// Actual root server, final export; no provider/model or configuration actions.
async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4318/")) throw Error("Root fixture only")
  await page.setViewportSize({ width: 390, height: 844 })
  const result = { viewport: page.viewportSize(), cases: [] }
  const selector = '[data-slot="drawer-popup"]'
  const trigger = () => page.getByRole("button", { name: "Show Sidebar", exact: true })
  const start = async (opening = false) => page.evaluate((opening) => {
    const frames = []
    const armed = performance.now()
    let begin = opening ? null : armed
    window.drawerFrames = new Promise(resolve => {
      const tick = () => {
        const el = document.querySelector('[data-slot="drawer-popup"]')
        if (begin === null) {
          if (el) begin = performance.now()
          else if (performance.now() - armed > 5000) throw Error("Drawer did not mount")
          else { requestAnimationFrame(tick); return }
        }
        const style = el && getComputedStyle(el)
        const rect = el && el.getBoundingClientRect()
        frames.push({
          t: performance.now() - begin,
          reduce: matchMedia('(prefers-reduced-motion: reduce)').matches,
          present: !!el, x: rect?.x ?? null, width: rect?.width ?? null,
          starting: el?.hasAttribute('data-starting-style') ?? false,
          ending: el?.hasAttribute('data-ending-style') ?? false,
          property: style?.transitionProperty ?? null,
          duration: style?.transitionDuration ?? null,
          animations: el ? el.getAnimations().map(a => ({
            property: a.transitionProperty ?? a.constructor.name,
            duration: a.effect?.getTiming().duration,
          })) : [],
        })
        if (performance.now() - begin < 750) requestAnimationFrame(tick)
        else resolve(frames)
      }
      requestAnimationFrame(tick)
    })
  }, opening)
  const finish = async () => page.evaluate(() => window.drawerFrames)
  const focus = async () => page.evaluate(() => {
    const el = document.activeElement
    const style = getComputedStyle(el)
    return { tag: el.tagName, slot: el.getAttribute('data-slot'),
      label: el.getAttribute('aria-label') || el.textContent?.trim().slice(0, 80),
      inDrawer: !!el.closest('[data-slot="drawer-popup"]'),
      focusVisible: el.matches(':focus-visible'), outline: style.outline,
      shadow: style.boxShadow,
    }
  })
  for (const preference of ["reduce", "no-preference"]) {
    await page.emulateMedia({ reducedMotion: preference })
    if (await page.locator(selector).count()) {
      await page.keyboard.press("Escape")
      await page.locator(selector).waitFor({ state: "detached" })
    }
    await trigger().focus()
    await start(true)
    await page.keyboard.press("Enter")
    const open = await finish()
    await page.getByRole("dialog", { name: "Sidebar", exact: true }).waitFor()
    await page.waitForFunction(() => {
      const el = document.querySelector('[data-slot="drawer-popup"]')
      return el && !el.hasAttribute('data-starting-style') && !el.getAnimations().length
    })
    const stage = await page.locator(selector).evaluate(el =>
      el.classList.contains('motion-reduce:transition-none') ? 'after' : 'before')
    const openedFocus = await focus()
    await page.keyboard.press("Tab")
    const tabFocus = await focus()
    const geometry = await page.locator(selector).evaluate(el => {
      const box = el.getBoundingClientRect()
      return { x: box.x, y: box.y, width: box.width, height: box.height,
        pageWidth: document.documentElement.scrollWidth, viewport: innerWidth }
    })
    await page.screenshot({ path: `reports/browser-phase1-final/${stage}-${preference}-open.png` })
    await start()
    await page.keyboard.press("Escape")
    const escape = await finish()
    await page.locator(selector).waitFor({ state: "detached" })
    const returnedFocus = await focus()
    await page.screenshot({ path: `reports/browser-phase1-final/${stage}-${preference}-closed.png` })
    await trigger().click()
    await page.getByRole("dialog", { name: "Sidebar", exact: true }).waitFor()
    await page.waitForFunction(() => {
      const el = document.querySelector('[data-slot="drawer-popup"]')
      return el && !el.hasAttribute('data-starting-style') && !el.getAnimations().length
    })
    await start()
    await page.mouse.click(378, 420)
    const outside = await finish()
    await page.locator(selector).waitFor({ state: "detached" })
    result.cases.push({ stage, preference, geometry, open, escape, outside,
      openedFocus, tabFocus, returnedFocus, outsideDismissed: true })
  }
  for (const c of result.cases) {
    if (c.stage !== "after") throw Error("Final drawer export missing")
    if (c.geometry.pageWidth !== 390) throw Error("Horizontal overflow")
    if (!c.openedFocus.inDrawer || !c.tabFocus.inDrawer || c.returnedFocus.label !== "Show Sidebar") throw Error("Focus regression")
    const frames = [...c.open, ...c.escape, ...c.outside].filter(f => f.present)
    const animated = frames.some(f => f.animations.some(a => a.property === "transform"))
    if (c.preference === "reduce" && (animated || frames.some(f => f.property !== "none"))) throw Error("Reduced motion regression")
    if (c.preference === "no-preference" && !animated) throw Error("Normal motion missing")
  }
  result.method = "Root actual Playwright CLI against integrated 4318 export; no decision or provider actions"
  return result
}
