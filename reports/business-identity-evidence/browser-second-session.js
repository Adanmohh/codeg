// Second independent Playwright CLI browser session against the same guarded DB.
async page => {
  const origin = "http://127.0.0.1:4341"
  await page.unroute("**/*")
  await page.route("**/*", route => route.request().url().startsWith(`${origin}/`) ? route.continue() : route.abort())
  await page.goto(`${origin}/__business_fixture`)
  return page.evaluate(async () => {
    const operator = "business-identity-synthetic-operator"
    async function call(path, token, input) {
      const r = await fetch(`/api/business/${path}`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${token}` }, body: JSON.stringify({ input }) })
      return { status: r.status, body: await r.json() }
    }
    const context = await call("context", operator, {})
    const organizationId = context.body.organization.id
    const created = await call("members/create", operator, { organizationId, displayName: `Independent browser B ${crypto.randomUUID().slice(0, 8)}`, kind: "human", role: "member", domains: ["feedback"] })
    const credential = await call("credentials/issue", operator, { organizationId, memberId: created.body.id, label: "Independent browser B" })
    const token = credential.body.token
    const self = await call("context", token, {})
    const directory = await call("members/list", token, { organizationId, domain: "feedback" })
    const checks = {
      ownNamedIdentity: self.status === 200 && self.body.member.id === created.body.id,
      sameOrganization: self.body.organization.id === organizationId,
      seesFirstBrowserViewer: directory.status === 200 && directory.body.some(m => m.displayName.startsWith("Browser viewer ")),
      noLegacyAuthority: !self.body.operator && !self.body.capabilities.legacyOperator,
      noPrivatePersistence: localStorage.length === 0 && sessionStorage.length === 0,
    }
    if (Object.values(checks).some(passed => !passed)) throw new Error("Second browser identity assertion failed")
    return { kind: "independent browser context; protected API only", organizationId, memberId: created.body.id, checks }
  })
}
