async (page) => {
  const result = await page.evaluate(async () => {
    const response = await fetch("/_fixture/counts")
    if (!response.ok) throw new Error("Synthetic fixture counts unavailable")
    const fixture = await response.json()
    const body = document.querySelector('[aria-label="Exact issue body"]')
    if (!body) throw new Error("Exact issue body is not rendered")
    const panel = body.closest("section")
    const fields = Object.fromEntries(
      [...panel.querySelectorAll("dl > div")].map((row) => [
        row.querySelector("dt").textContent,
        row.querySelector("dd").textContent,
      ])
    )
    const expected = fixture.expectedIssue
    if (
      body.textContent !== expected.body ||
      fields.Title !== expected.title ||
      fields.Labels !== expected.labels.join(", ") ||
      fields.Repository !== "owner/repo"
    )
      throw new Error("Rendered proposal does not match the native host payload")
    const rect = body.getBoundingClientRect()
    return {
      viewport: { width: innerWidth, height: innerHeight },
      pageWidth: document.documentElement.scrollWidth,
      exactBody: true,
      exactTitle: true,
      exactLabels: true,
      repository: fields.Repository,
      bodyCharacters: body.textContent.length,
      body: { x: rect.x, width: rect.width, scrollWidth: body.scrollWidth },
      upstreamReads: fixture.upstreamReads,
      githubPosts: fixture.githubPosts,
      githubTokenRequests: fixture.githubTokenRequests,
      proposals: fixture.proposals,
    }
  })
  return {
    ...result,
    confirmationChecked: await page.getByRole("checkbox").isChecked(),
    approveDisabled: await page
      .getByRole("button", { name: "Approve and file issue", exact: true })
      .isDisabled(),
    guard: page.context().piIssuesEvidence,
  }
}
