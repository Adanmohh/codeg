async (page) => {
  // Reuses the accepted Pi CLI context guard. Only this synthetic local
  // review is permitted; never launch a model, file an issue or save settings.
  const counts = {
    blockedRemote: 0,
    blockedAgentRequests: 0,
    blockedWrites: 0,
    humanImports: 0,
    humanRefreshes: 0,
    humanDenials: 0,
  }
  page.context().piIssuesEvidence = counts
  await page.context().route("**/*", async (route) => {
    const request = route.request()
    const local = "http://127.0.0.1:4324/"
    if (!request.url().startsWith(local)) {
      counts.blockedRemote++
      return route.abort()
    }
    const path = request.url().slice(local.length).split("?")[0]
    if (/^api\/acp_(connect|prompt|install|uninstall|update)/.test(path)) {
      counts.blockedAgentRequests++
      return route.abort()
    }
    if (/^api\/(set_|save_)/.test(path)) {
      counts.blockedWrites++
      return route.abort()
    }
    if (path.startsWith("api/ops_intake_")) {
      const action = path.slice("api/ops_intake_".length)
      if (!["status", "list", "detail", "refresh", "deny"].includes(action)) {
        counts.blockedWrites++
        return route.abort()
      }
      if (action === "list") counts.humanImports++
      if (action === "refresh") counts.humanRefreshes++
      if (action === "deny") counts.humanDenials++
    }
    return route.continue()
  })
  return "Synthetic Pi issue review context guarded"
}
