// Specialist final rerun of PR18 actual browser measurement on owned synthetic fixture.
// Measurement glue adapted from the accepted design-ops-probe.playwright.
// Run only through the installed Playwright CLI against the existing fixture.
async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Fixture only")
  const stats = () => page.evaluate(async () => {
    const response = await fetch("/api/ops_design_fixture_stats", {
      headers: { Authorization: "Bearer ops-design-synthetic-operator" },
    })
    if (!response.ok) throw Error("Fixture stats unavailable")
    return response.json()
  })
  const before = await stats()
  if (!before.syntheticOnly || before.providerRequests !== 4) throw Error("Unexpected fixture state")
  const themeSetting = await page.evaluate(() => localStorage.getItem("theme"))
  const appearance = page.context().pages().find(p => p.url().endsWith("4327/settings/appearance"))
  if (!appearance) throw Error("Owned Appearance tab required")
  await page.setViewportSize({ width: 1280, height: 900 })
  const show = page.getByRole("button", { name: /^Show Sidebar/ })
  if (await show.count()) await show.click()
  if (!await page.getByRole("region", { name: "Ops desk", exact: true }).count()) {
    await page.getByRole("button", { name: "Ops desk", exact: true }).click()
    await page.getByRole("region", { name: "Ops desk", exact: true }).waitFor()
  }
  const group = page.getByRole("button", { name: "Synthetic badge group 6 sessions running", exact: true })
  const surface = await group.count() ? "group" : "folder"
  const header = surface === "group" ? group : page.getByRole("button", { name: "ops-ui-task-fixture 6 sessions running", exact: true })
  const badge = header.locator('span[title="6 sessions running"]')
  await badge.waitFor()
  const phase = (await badge.getAttribute("class")).split(" ").includes("text-amber-800") ? "after" : "before"
  const rows = []
  for (const theme of ["light", "dark"]) {
    await appearance.getByRole("combobox").first().click()
    await appearance.getByRole("option", { name: theme === "dark" ? "Dark" : "Light", exact: true }).click()
    await page.waitForFunction((t) => document.documentElement.classList.contains(t) && document.documentElement.style.colorScheme === t, theme)
    for (const state of ["rest", "hover", "focus"]) {
      await page.mouse.move(1200, 850)
      await header.evaluate((el) => el.blur())
      if (state === "hover") await header.hover()
      if (state === "focus") {
        // Enter via the keyboard; do not activate/collapse the folder.
        await header.evaluate((el) => el.focus())
        await page.keyboard.press("Shift+Tab")
        await page.keyboard.press("Tab")
      }
      await page.waitForTimeout(300) // Existing 150 ms row transition fully settled.
      const measurement = await badge.evaluate((el) => {
        const canvas = document.createElement("canvas")
        canvas.width = canvas.height = 1
        const ctx = canvas.getContext("2d")
        if (!ctx) throw Error("Canvas unavailable")
        const painted = (colors) => {
          ctx.clearRect(0, 0, 1, 1)
          for (const color of ["white", ...colors]) {
            ctx.fillStyle = color
            ctx.fillRect(0, 0, 1, 1)
          }
          const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data
          return `rgb(${r}, ${g}, ${b})`
        }
        const layers = []
        for (let node = el; node; node = node.parentElement) {
          const style = getComputedStyle(node)
          if (style.opacity !== "1" || style.backgroundImage !== "none") throw Error("Unaccounted compositing")
          layers.unshift(style.backgroundColor)
        }
        const style = getComputedStyle(el)
        const button = el.closest("button")
        return {
          text: el.querySelector('[aria-hidden="true"]').textContent,
          label: el.title,
          srOnly: el.querySelector(".sr-only").textContent,
          className: el.className,
          foreground: style.color,
          background: style.backgroundColor,
          layers,
          fg: painted([style.color]),
          bg: painted(layers),
          // Exact installed Tailwind 4.1.18 theme.css amber-800, measured only.
          candidateFg: painted(["oklch(47.3% 0.137 46.201)"]),
          sizePx: parseFloat(style.fontSize),
          bold: parseInt(style.fontWeight, 10) >= 700,
          fontFamily: style.fontFamily,
          fontWeight: style.fontWeight,
          visible: el.checkVisibility(),
          focused: button === document.activeElement,
          focusVisible: button.matches(":focus-visible"),
          hovered: button.matches(":hover"),
          focusShadow: getComputedStyle(button).boxShadow,
          expanded: button.getAttribute("aria-expanded"),
          themeClass: document.documentElement.className,
          colorScheme: document.documentElement.style.colorScheme,
        }
      })
      if (!measurement.visible || measurement.text !== "6" || measurement.srOnly !== "6 sessions running") throw Error("Badge semantics changed")
      if (state === "focus" && (!measurement.focused || !measurement.focusVisible)) throw Error("Keyboard focus missing")
      if (state === "hover" && !measurement.hovered) throw Error("Hover missing")
      rows.push({ phase, surface, theme, state, ...measurement, aria: await header.ariaSnapshot() })
      await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/final-badge-${surface === "group" ? "group-" : ""}${phase}-${theme}-${state}.png` })
    }
  }
  await header.evaluate((el) => el.blur())
  // Leave the last actual Appearance preference in place.
  const after = await stats()
  if (JSON.stringify(before) !== JSON.stringify(after)) throw Error("Fixture changed")
  return { phase, surface, viewport: { width: 1280, height: 900 }, themeSetting, before, after, rows }
}
