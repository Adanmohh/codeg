async (page) => {
  // Adapted from accepted Pi issue browser guard. Report-only, fresh local DB.
  const counts = { blockedRemote: 0, blockedAgentRequests: 0, blockedWrites: 0 }
  page.context().drawerMotionEvidence = counts
  await page.context().route("**/*", async (route) => {
    const request = route.request()
    const local = "http://127.0.0.1:4325/"
    if (!request.url().startsWith(local)) {
      counts.blockedRemote++
      return route.abort()
    }
    const path = request.url().slice(local.length).split("?")[0]
    if (/^api\/acp_(connect|prompt|install|uninstall|update)/.test(path)) {
      counts.blockedAgentRequests++
      return route.abort()
    }
    if (/^api\/(set_|save_|ops_|ops_intake_|acp_pi_set_|check_for_updates)/.test(path)) {
      counts.blockedWrites++
      return route.abort()
    }
    return route.continue()
  })
  return "Owned motion fixture guarded; no model/provider/configuration actions"
}
