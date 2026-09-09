// Actual UI setup; only the worker's private synthetic credential enters memory.
async (page) => {
  if (page.url() !== "http://127.0.0.1:4354/business")
    throw new Error("Expected the dedicated unified business fixture")
  const fs = page.constructor.constructor("return process")().getBuiltinModule("node:fs")
  const fixture = JSON.parse(fs.readFileSync(
    "/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-intake-fixtures/unified-YQmRz8/worker-credentials.json",
    "utf8"
  ))
  if (fixture.synthetic !== true || fixture.namespace !== "worker")
    throw new Error("Expected the worker synthetic namespace")
  const dialog = page.getByRole("dialog")
  await dialog.getByLabel("Connection name", { exact: true }).fill("Worker meeting review")
  await dialog.getByLabel("Area of work", { exact: true }).selectOption("feedback")
  await dialog.getByLabel("Source account owner", { exact: true }).selectOption({ label: "worker owner" })
  await dialog.getByRole("checkbox", { name: "Feedback", exact: true }).check()
  await dialog.getByRole("checkbox", { name: "I allow explicitly accepted task text to remain in the chosen work area, even if source access is later removed.", exact: true }).check()
  try {
    await dialog.getByLabel("Fireflies API key", { exact: true }).fill(fixture.firefliesApiKey)
    await dialog.getByRole("button", { name: "Create disabled connection", exact: true }).click()
    await dialog.getByRole("heading", { name: "Review access and setup", exact: true }).waitFor()
  } catch {
    const input = dialog.getByLabel("Fireflies API key", { exact: true })
    if (await input.count()) await input.fill("")
    throw new Error("Synthetic source setup did not complete; credential withheld")
  }
  const passwordsEmpty = await dialog.locator('input[type="password"]').evaluateAll(
    (inputs) => inputs.every((input) => input.value === "")
  )
  const noGrant = await dialog.getByText("Nobody has an explicit grant yet.", { exact: true }).count()
  const disabled = await dialog.getByRole("button", { name: "Enable connection", exact: true }).count()
  if (!passwordsEmpty || noGrant !== 1 || disabled !== 1)
    throw new Error("Expected disabled source, no grants and cleared credential")
  return { namespace: "worker", disabled: true, noImplicitGrant: true, passwordsEmpty, guard: page.__workerGuard }
}
