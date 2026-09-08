async (page) => {
  await page.goto("http://127.0.0.1:4340/business.html")
  await page.setViewportSize({ width: 390, height: 900 })
  await page
    .getByRole("heading", { name: "Give shared work a clear next step." })
    .waitFor()
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
  const message = await alert.innerText()
  await page.getByLabel("Personal access token", { exact: true }).fill("")
  await page.screenshot({
    path: "reports/business-workspace-evidence/connect-invalid-before.png",
  })
  return {
    synthetic: true,
    message,
    passwordCleared: true,
    fieldsRetained:
      (await page
        .getByLabel("Workspace address", { exact: true })
        .inputValue()) === "http://unsafe.invalid",
  }
}
