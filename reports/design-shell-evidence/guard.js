async (page) => {
  // Review-only guard, including the Settings popup. All data stays in the
  // accepted test fixture; no prompt, installer or credentials mutation.
  const evidence = { piConnects: 0, blockedPrompts: 0, blockedRemote: 0, blockedWrites: 0 }
  page.context().piReviewEvidence = evidence
  await page.context().route("**/*", async (route) => {
    const request = route.request()
    const local = "http://127.0.0.1:4324/"
    if (!request.url().startsWith(local)) {
      evidence.blockedRemote++
      return route.abort()
    }
    const path = request.url().slice(local.length).split("?")[0]
    if (path === "api/acp_prompt") {
      evidence.blockedPrompts++
      return route.abort()
    }
    if (/api\/acp_(install|uninstall|update_pi|pi_set_project_trust)/.test(path)) {
      evidence.blockedWrites++
      return route.abort()
    }
    if (path === "api/acp_connect") {
      if (JSON.parse(request.postData() || "{}").agentType !== "pi") {
        evidence.blockedWrites++
        return route.abort()
      }
      evidence.piConnects++
    }
    return route.continue()
  })
  return "Review context guard installed"
}
