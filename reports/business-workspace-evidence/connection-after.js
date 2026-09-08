async (page) => {
  await page.goto("http://127.0.0.1:4340/business.html")
  await page
    .getByRole("heading", { name: "Give shared work a clear next step." })
    .waitFor()
  await page.evaluate(() => document.fonts.ready)
  const results = []
  for (const [width, theme, language] of [
    [1280, "light", "en"],
    [390, "light", "en"],
    [768, "light", "en"],
    [1280, "dark", "en"],
    [768, "dark", "ar"],
    [390, "dark", "ar"],
  ]) {
    await page.setViewportSize({ width, height: 900 })
    await page.locator("select").nth(0).selectOption(language)
    await page.locator("select").nth(1).selectOption(theme)
    await page.waitForFunction(
      ({ theme, language }) =>
        document.documentElement.classList.contains("dark") ===
          (theme === "dark") &&
        document.documentElement.dir === (language === "ar" ? "rtl" : "ltr"),
      { theme, language }
    )
    await page.evaluate(
      () =>
        new Promise((resolve) =>
          requestAnimationFrame(() => requestAnimationFrame(resolve))
        )
    )
    await page.screenshot({
      path: `reports/business-workspace-evidence/connect-after-${width}-${theme}-${language}.png`,
    })
    results.push(
      await page.evaluate(() => ({
        width: innerWidth,
        dark: document.documentElement.classList.contains("dark"),
        dir: document.documentElement.dir,
        overflow:
          document.documentElement.scrollWidth > innerWidth ||
          document.querySelector("main").scrollWidth > innerWidth,
        connectTop: document
          .querySelector("button[type=submit]")
          .getBoundingClientRect().top,
        fields: [
          ...document.querySelectorAll("select,input:not([type=checkbox])"),
        ].map((el) => ({
          type: el.type,
          height: el.getBoundingClientRect().height,
          font: getComputedStyle(el).fontSize,
        })),
      }))
    )
  }
  await page.locator("select").nth(0).selectOption("en")
  await page.locator("select").nth(1).selectOption("light")
  await page.setViewportSize({ width: 390, height: 900 })
  await page.getByLabel("Workspace address", { exact: true }).focus()
  await page.keyboard.press("Tab")
  const focus = await page.evaluate(() => {
    const el = document.activeElement
    const css = getComputedStyle(el)
    return {
      type: el.type,
      visible: el.matches(":focus-visible"),
      boxShadow: css.boxShadow,
      outline: css.outline,
      inViewport:
        el.getBoundingClientRect().top >= 0 &&
        el.getBoundingClientRect().bottom <= innerHeight,
    }
  })
  await page.screenshot({
    path: "reports/business-workspace-evidence/connect-focus-390.png",
  })
  await page
    .getByLabel("Workspace address", { exact: true })
    .fill("http://unsafe.invalid")
  await page
    .getByLabel("Personal access token", { exact: true })
    .fill("synthetic-invalid-fixture-input")
  await page.getByRole("button", { name: "Connect", exact: true }).click()
  const alert = page
    .getByRole("region", { name: "Sign in to your workspace" })
    .getByRole("alert")
  await alert.waitFor()
  const invalid = await alert.innerText()
  if (invalid !== "Check the entered information and try again.")
    throw Error("Invalid connection copy")
  await page
    .getByLabel("Workspace address", { exact: true })
    .fill("http://127.0.0.1:4340")
  await page.getByRole("button", { name: "Connect", exact: true }).click()
  await alert
    .getByText("The workspace could not be reached.", { exact: true })
    .waitFor()
  await page.getByLabel("Personal access token", { exact: true }).fill("")
  await page.screenshot({
    path: "reports/business-workspace-evidence/connect-unavailable-390.png",
  })
  await page.emulateMedia({ reducedMotion: "reduce" })
  const reduced = await page
    .getByRole("button", { name: "Connect", exact: true })
    .evaluate((el) => ({
      property: getComputedStyle(el).transitionProperty,
      duration: getComputedStyle(el).transitionDuration,
    }))
  if (
    !focus.visible ||
    !focus.inViewport ||
    results.some((v) => v.overflow) ||
    results
      .filter((v) => v.width === 390)
      .some((v) => v.fields.some((f) => parseFloat(f.font) < 16)) ||
    (reduced.property !== "none" && reduced.duration !== "0s")
  )
    throw Error(
      "Connection accessibility check failed: " +
        JSON.stringify({ results, focus, reduced })
    )
  await page.setViewportSize({ width: 1280, height: 900 })
  return {
    synthetic: true,
    backendConnected: false,
    results,
    focus,
    invalid,
    unavailableFromFixtureGuard: true,
    reducedTransition: reduced,
    credentialsClearedBeforeCapture: true,
  }
}
