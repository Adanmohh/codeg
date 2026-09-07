;async (page) => {
  // CLI-only fixture guard. Real localhost routes are retained for Pi and all
  // preference reads/writes. No prompt or other agent launch is permitted.
  page.deskFixtureEvidence = {
    promptRequests: 0,
    piConnects: 0,
    otherConnects: 0,
    remoteRequests: 0,
  }
  await page.route("**/*", async (route) => {
    const request = route.request()
    const url = request.url()
    const local = "http://127.0.0.1:4324/"
    if (!url.startsWith(local)) {
      page.deskFixtureEvidence.remoteRequests++
      return route.abort()
    }
    const path = url.slice(local.length).split("?")[0]
    if (path === "api/acp_prompt") {
      page.deskFixtureEvidence.promptRequests++
      return route.abort()
    }
    if (path === "api/acp_connect") {
      const input = JSON.parse(request.postData() || "{}")
      if (input.agentType !== "pi") {
        page.deskFixtureEvidence.otherConnects++
        return route.abort()
      }
      page.deskFixtureEvidence.piConnects++
    }
    return route.continue()
  })
  return "Local fixture guard active; prompt and non-Pi launch requests blocked"
}
