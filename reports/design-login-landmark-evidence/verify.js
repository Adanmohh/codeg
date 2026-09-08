async (page) => {
  // Report-only actual CLI flow against the accepted protected local fixture.
  // The token below is public synthetic test data from pi_desk_browser_fixture.
  const origin = "http://127.0.0.1:4325"
  const width = page.viewportSize().width
  const path = `reports/design-login-landmark-evidence/${width}`
  const assert = (condition, message) => {
    if (!condition) throw new Error(message)
  }
  const measure = async () => {
    const dom = await page.evaluate(() => {
      const main = document.querySelector("main")
      const input = document.querySelector("#operator-token")
      const form = document.querySelector("form")
      const heading = document.querySelector("h1")
      const error = document.querySelector("#login-error")
      const rect = main?.getBoundingClientRect()
      return {
        mainElements: document.querySelectorAll("main").length,
        containsHeading: !!heading && !!main?.contains(heading),
        containsForm: !!form && !!main?.contains(form),
        heading: heading?.textContent,
        forms: document.querySelectorAll("form").length,
        inputType: input?.type,
        label: input?.labels?.[0]?.textContent,
        invalid: input?.getAttribute("aria-invalid"),
        describedBy: input?.getAttribute("aria-describedby"),
        errorRole: error?.getAttribute("role") ?? null,
        errorText: error?.textContent ?? null,
        errorInsideMain: error ? !!main?.contains(error) : null,
        activeElement: document.activeElement?.id,
        submitDisabled: form?.querySelector("button")?.disabled,
        tokenStored: !!localStorage.getItem("codeg_token"),
        viewport: { width: innerWidth, height: innerHeight },
        documentWidth: document.documentElement.scrollWidth,
        mainBox: rect && { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
      }
    })
    dom.mainRoles = await page.getByRole("main").count()
    assert(dom.mainElements === 1 && dom.mainRoles === 1, "Exactly one main element and accessibility role")
    assert(dom.containsHeading && dom.containsForm && dom.forms === 1, "Main contains the heading and form")
    assert(dom.viewport.width === width && dom.documentWidth === width, "Requested viewport without horizontal overflow")
    assert(dom.label === "Access Token" && dom.inputType === "password", "Persistent accessible label and masked input")
    return dom
  }

  await page.goto(`${origin}/login`)
  await page.getByRole("heading", { name: "Hafidh Ops Desk", exact: true }).waitFor()
  await page.waitForFunction(() => document.activeElement?.id === "operator-token")
  const initial = await measure()
  assert(!initial.tokenStored && initial.submitDisabled && initial.invalid === "false", "Fresh empty form")
  const initialAccessibility = await page.getByRole("main").ariaSnapshot()
  await page.screenshot({ path: `${path}-initial.png`, fullPage: true })

  const input = page.getByLabel("Access Token", { exact: true })
  const connect = page.getByRole("button", { name: "Connect", exact: true })
  await input.fill("invalid-fixture-token")
  const invalidResponsePromise = page.waitForResponse(response =>
    response.url() === `${origin}/api/health` && response.request().method() === "POST"
  )
  await connect.click()
  const invalidStatus = (await invalidResponsePromise).status()
  await page.getByRole("main").getByRole("alert").waitFor()
  const invalid = await measure()
  assert(invalidStatus === 401, "Real protected router rejects invalid synthetic token")
  assert(!invalid.tokenStored && !invalid.submitDisabled, "Invalid token is not stored and retry is available")
  assert(invalid.invalid === "true" && invalid.describedBy === "login-error" && invalid.errorRole === "alert" && invalid.errorInsideMain, "Associated announced error remains inside main")
  assert(invalid.errorText === "Invalid token. Please check and try again.", "Accepted invalid-token guidance")
  await page.screenshot({ path: `${path}-invalid.png`, fullPage: true })

  await input.fill("pi-desk-browser-fixture")
  const validResponsePromise = page.waitForResponse(response =>
    response.url() === `${origin}/api/health` && response.request().method() === "POST"
  )
  await input.press("Enter")
  const validStatus = (await validResponsePromise).status()
  await page.waitForURL(`${origin}/workspace`)
  await page.locator("#operator-token").waitFor({ state: "detached" })
  await page.waitForLoadState("networkidle")
  const navigation = await page.evaluate(() => ({
    pathname: location.pathname,
    tokenStored: !!localStorage.getItem("codeg_token"),
    loginFormGone: !document.querySelector("#operator-token"),
    viewportWidth: innerWidth,
    documentWidth: document.documentElement.scrollWidth,
  }))
  assert(validStatus === 200 && navigation.pathname === "/workspace" && navigation.tokenStored && navigation.loginFormGone, "Keyboard retry validates and navigates")
  await page.screenshot({ path: `${path}-workspace.png`, fullPage: true })
  const guard = { ...page.context().drawerMotionEvidence }
  assert(Object.keys(guard).length === 3 && Object.values(guard).every(count => count === 0), "No off-origin, agent or configuration attempts")
  return {
    sourceCommit: "66849a91c7a6bdb5105215bbabd70e659ddb36f4",
    viewportWidth: width,
    initial,
    initialAccessibility,
    invalidStatus,
    invalid,
    validStatus,
    navigation,
    guard,
    assertions: "passed",
  }
}
