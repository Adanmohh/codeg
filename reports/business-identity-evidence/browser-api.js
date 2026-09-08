// Test evidence glue over production protected identity APIs. No frontend
// response interception; only off-origin requests are aborted. CLI0.1.18 /
// installed Playwright1.63.0-alpha-2026-08-05 types were read before use.
async page => {
  const origin = "http://127.0.0.1:4341"
  const blocked = []
  await page.unroute("**/*")
  await page.route("**/*", route => {
    if (route.request().url().startsWith(`${origin}/`)) return route.continue()
    blocked.push("off-origin request")
    return route.abort()
  })
  await page.goto(`${origin}/__business_fixture`)
  const result = await page.evaluate(async () => {
    const operator = "business-identity-synthetic-operator"
    const checks = []
    function check(name, condition) {
      checks.push({ name, passed: Boolean(condition) })
      if (!condition) throw new Error(`Failed assertion: ${name}`)
    }
    async function call(path, token, input) {
      const response = await fetch(`/api/${path}`, {
        method: "POST",
        headers: {
          "content-type": "application/json",
          ...(token ? { authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify({ input }),
      })
      const text = await response.text()
      let body
      try { body = JSON.parse(text) } catch { body = null }
      return { status: response.status, body, cache: response.headers.get("cache-control") }
    }
    const anonymous = await call("business/context", null, {})
    check("Anonymous context is401", anonymous.status === 401)
    const boot = await call("business/bootstrap", operator, { organizationName: "Synthetic shared identity team", ownerName: "Synthetic owner" })
    check("Original operator initializes organization", boot.status === 200 && boot.body.operator)
    const organizationId = boot.body.organization.id
    const suffix = crypto.randomUUID().slice(0, 8)
    const create = (displayName, role) => call("business/members/create", operator, {
      organizationId, displayName, kind: "human", role, domains: ["feedback"],
    })
    const a = await create(`Browser member A ${suffix}`, "member")
    const b = await create(`Browser viewer ${suffix}`, "viewer")
    check("Creates two named humans", a.status === 200 && b.status === 200 && a.body.id !== b.body.id)
    const issue = memberId => call("business/credentials/issue", operator, { organizationId, memberId, label: "Synthetic browser only" })
    const credential = await issue(a.body.id)
    const viewerCredential = await issue(b.body.id)
    check("Individual credentials issued", credential.status === 200 && viewerCredential.status === 200 && credential.body.token !== viewerCredential.body.token)
    // Tokens stay only in this local closure; no storage/log/output serialization.
    const memberToken = credential.body.token
    const viewerToken = viewerCredential.body.token
    const memberContext = await call("business/context", memberToken, {})
    check("Server resolves exact human/org with no legacy authority", memberContext.status === 200 && memberContext.body.member.id === a.body.id && memberContext.body.organization.id === organizationId && !memberContext.body.capabilities.legacyOperator && !memberContext.body.operator)
    check("Identity responses are no-store", memberContext.cache === "no-store")
    const directory = await call("business/members/list", viewerToken, { organizationId, domain: "feedback" })
    check("Viewer reads shared permitted directory", directory.status === 200 && directory.body.some(m => m.id === a.body.id) && directory.body.some(m => m.id === b.body.id))
    const forbidden = await call("business/members/create", viewerToken, { organizationId, displayName: "Forbidden", kind: "human", role: "owner", domains: ["feedback"] })
    check("Viewer cannot mint an identity", forbidden.status === 403)
    check("Member cannot take over bootstrap", (await call("business/bootstrap", memberToken, { organizationName: "Takeover", ownerName: "Impersonation" })).status === 403)
    check("Caller actor/role is rejected", (await call("business/context", memberToken, { actor: boot.body.member.id, role: "owner" })).status === 422)
    check("Foreign organization is inaccessible", (await call("business/members/list", memberToken, { organizationId: crypto.randomUUID() })).status === 404)
    check("Unassigned engineering domain is denied", (await call("business/members/list", memberToken, { organizationId, domain: "engineering" })).status === 403)
    check("Member credential fails real legacy health", (await call("health", memberToken, {})).status === 401)
    check("Original operator health remains available", (await call("health", operator, {})).status === 200)
    const changed = displayName => call("business/members/update", operator, { organizationId, memberId: a.body.id, expectedRevision: a.body.revision, displayName, role: "member", domains: ["feedback"] })
    const races = await Promise.all([changed(`Race A ${suffix}`), changed(`Race B ${suffix}`)])
    check("Real simultaneous HTTP edits have one409", races.filter(r => r.status === 200).length === 1 && races.filter(r => r.status === 409).length === 1)
    const stale = races.find(r => r.status === 409)
    check("Conflict has a stable recovery code", stale.body.code === "already_exists" && stale.body.i18n_key === "business.revisionConflict")
    const metadata = await call("business/credentials/list", operator, { organizationId, memberId: a.body.id })
    check("Credential reads contain no token/hash", metadata.status === 200 && !JSON.stringify(metadata.body).includes(memberToken) && !JSON.stringify(metadata.body).includes("token_hash"))
    check("Operator revokes individual credential", (await call("business/credentials/revoke", operator, { organizationId, credentialId: credential.body.credential.id })).status === 200)
    check("Revoked browser credential immediately401", (await call("business/context", memberToken, {})).status === 401)
    check("Other independent credential still valid", (await call("business/context", viewerToken, {})).status === 200)
    check("No local or session storage", localStorage.length === 0 && sessionStorage.length === 0)
    return { organizationId, memberId: a.body.id, viewerId: b.body.id, checks }
  })
  return { kind: "real protected API through Chromium fetch; test landing only", frontendMocking: false, blockedOrigins: blocked, ...result }
}
