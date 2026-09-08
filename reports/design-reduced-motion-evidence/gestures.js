async (page) => {
  const selector = '[data-slot="drawer-popup"]'
  const result = { cases: [] }
  const cdp = await page.context().newCDPSession(page)
  await cdp.send("Emulation.setTouchEmulationEnabled", { enabled: true, maxTouchPoints: 1 })
  const settle = async () => page.waitForFunction(() => {
    const el = document.querySelector('[data-slot="drawer-popup"]')
    return el && !el.hasAttribute('data-starting-style') && !el.getAnimations().length
  })
  const sample = async () => page.evaluate(() => {
    const el = document.querySelector('[data-slot="drawer-popup"]')
    return el ? { x: el.getBoundingClientRect().x,
      swiping: el.hasAttribute('data-swiping'),
      property: getComputedStyle(el).transitionProperty,
      animations: el.getAnimations().map(a => a.transitionProperty ?? a.constructor.name),
    } : null
  })
  try {
    for (const preference of ["reduce", "no-preference"]) {
      await page.emulateMedia({ reducedMotion: preference })
      await page.getByRole("button", { name: "Show Sidebar", exact: true }).click()
      await settle()
      await page.getByRole("button", { name: "View options", exact: true }).click()
      await page.getByRole("menu").waitFor()
      await page.keyboard.press("Escape")
      await page.getByRole("menu").waitFor({ state: "detached" })
      const nestedEscapePreservedDrawer = await page.locator(selector).count() === 1
      if (!nestedEscapePreservedDrawer) throw Error("Nested menu Escape closed sidebar")
      await page.screenshot({ path: `reports/design-reduced-motion-evidence/after-${preference}-nested-dismissal.png` })
      const drag = []
      await cdp.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ x: 280, y: 610, id: 1 }] })
      for (const x of [250, 220, 190, 160, 130, 100, 70, 40]) {
        await cdp.send("Input.dispatchTouchEvent", { type: "touchMove", touchPoints: [{ x, y: 610, id: 1 }] })
        await page.evaluate(() => new Promise(resolve => requestAnimationFrame(resolve)))
        drag.push(await sample())
      }
      await cdp.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] })
      const release = await page.evaluate(() => new Promise(resolve => {
        const frames = []
        const begin = performance.now()
        function tick() {
          const el = document.querySelector('[data-slot="drawer-popup"]')
          frames.push({ t: performance.now() - begin, present: !!el,
            x: el?.getBoundingClientRect().x ?? null,
            property: el ? getComputedStyle(el).transitionProperty : null,
            animations: el ? el.getAnimations().map(a => a.transitionProperty ?? a.constructor.name) : [],
          })
          if (performance.now() - begin < 500) requestAnimationFrame(tick)
          else resolve(frames)
        }
        tick()
      }))
      await page.locator(selector).waitFor({ state: "detached" })
      result.cases.push({ preference, nestedEscapePreservedDrawer, drag, release, swipeDismissed: true })
    }
  } finally {
    await cdp.send("Emulation.setTouchEmulationEnabled", { enabled: false })
    await cdp.detach()
  }
  result.guard = page.context().drawerMotionEvidence
  return result
}
