async (page) => {
  const calls = []
  page.on("response", (response) => {
    if (response.url().includes("/api/"))
      calls.push({
        path: response.url().split("/api/")[1].split("?")[0],
        status: response.status(),
      })
  })
  await page.goto("http://127.0.0.1:4340/business.html")
  await page.setViewportSize({ width: 1280, height: 900 })
  await page
    .getByRole("heading", { name: "Give shared work a clear next step." })
    .waitFor()
  await page.getByText("Administrator access", { exact: true }).click()
  await page.getByLabel("Original administrator token", { exact: true }).check()
  await page
    .locator("input[type=password]")
    .fill("business-tasks-synthetic-operator")
  await page.getByRole("button", { name: "Connect", exact: true }).click()
  await page
    .getByRole("heading", {
      name: /^(My work|Start with your organization\.)$/,
    })
    .waitFor()
  const needsBootstrap = await page
    .getByRole("heading", { name: "Start with your organization." })
    .isVisible()
  if (!needsBootstrap) {
    await page.getByRole("button", { name: "Refresh", exact: true }).waitFor()
    await page.waitForFunction(
      () =>
        !document.querySelector('#business-main [aria-label="Refresh"]')
          .disabled
    )
    await page.screenshot({
      path: "reports/business-workspace-evidence/workspace-first-real-1280-light.png",
    })
  }
  return {
    synthetic: true,
    backend: "4342 @ 1ba73e3c",
    needsBootstrap,
    calls,
    passwordInputPresent: await page.locator("input[type=password]").count(),
    heading: await page.locator("h1").innerText(),
  }
}
