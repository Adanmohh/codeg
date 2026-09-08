async (page) => {
  const requests = [],
    blocked = [],
    errors = []
  await page.route("**/*", (route) => {
    const url = route.request().url()
    if (url.startsWith("http://127.0.0.1:4340/") || url === "about:blank")
      return route.continue()
    blocked.push(url.split("?")[0])
    return route.abort()
  })
  page.on("request", (request) => {
    if (request.url().includes("/api/"))
      requests.push(request.url().split("?")[0])
  })
  page.on("pageerror", () => errors.push("pageerror"))
  await page.addInitScript(() => {
    localStorage.setItem("codeg_token", "synthetic-ambient-old-operator")
    localStorage.setItem("codeg-workspace-bg-enabled", "1")
    localStorage.setItem(
      "codeg.system_language_settings",
      JSON.stringify({ mode: "manual", language: "en" })
    )
    localStorage.setItem("theme", "light")
  })
  await page.setViewportSize({ width: 1280, height: 900 })
  const paths = []
  for (const path of [
    "/business.html",
    "/business/",
    "/business",
    "/index.html",
    "/",
  ]) {
    const response = await page.goto(`http://127.0.0.1:4340${path}`)
    await page
      .getByRole("heading", { name: "Give shared work a clear next step." })
      .waitFor()
    await page.evaluate(() => document.fonts.ready)
    await page.evaluate(
      () =>
        new Promise((resolve) =>
          requestAnimationFrame(() => requestAnimationFrame(resolve))
        )
    )
    paths.push({
      path,
      status: response.status(),
      rendered: true,
      apiRequests: requests.length,
    })
  }
  const measures = []
  for (const [width, theme, language] of [
    [1280, "light", "en"],
    [390, "light", "en"],
    [768, "dark", "en"],
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
      path: `reports/business-workspace-evidence/connect-before-${width}-${theme}-${language}.png`,
      fullPage: true,
    })
    measures.push(
      await page.evaluate(() => ({
        width: innerWidth,
        dir: document.documentElement.dir,
        dark: document.documentElement.classList.contains("dark"),
        pageOverflow: document.documentElement.scrollWidth > innerWidth,
        mainOverflow: document.querySelector("main").scrollWidth > innerWidth,
        fields: [...document.querySelectorAll("input,select")]
          .filter((el) => el.getClientRects().length)
          .map((el) => ({
            type: el.type,
            height: el.getBoundingClientRect().height,
            font: getComputedStyle(el).fontSize,
          })),
      }))
    )
  }
  if (
    requests.length ||
    blocked.length ||
    errors.length ||
    paths.some((v) => v.status !== 200) ||
    measures.some((v) => v.pageOverflow || v.mainOverflow)
  )
    throw Error(
      "Connection preflight failed; inspect explicit result without credentials"
    )
  return {
    synthetic: true,
    backendConnected: false,
    source: "0e5eb3e7 plus saved-version copy",
    paths,
    requests,
    blocked,
    errors,
    measures,
  }
}
