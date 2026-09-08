// Report-only Playwright CLI orchestration. No response replacement or account writes.
async page => {
  const origin = 'http://127.0.0.1:4340'
  const evidence = {calls: [], blocked: [], websockets: [], paths: []}
  page.__businessReview = evidence
  await page.route('**/*', route => {
    const request = route.request()
    const url = request.url()
    if (!url.startsWith(origin + '/')) {
      evidence.blocked.push({kind: 'non-fixture URL'})
      return route.abort()
    }
    const path = url.slice(origin.length).split('?')[0]
    if (path.startsWith('/api/')) {
      evidence.calls.push({path, method: request.method()})
      if (!path.startsWith('/api/business/') || request.method() !== 'POST') {
        evidence.blocked.push({path, method: request.method()})
        return route.abort()
      }
    }
    return route.continue()
  })
  await page.routeWebSocket('**', ws => {
    evidence.websockets.push({attempted: true})
    ws.close()
  })
  await page.addInitScript(() => {
    localStorage.setItem('codeg_token', 'synthetic-review-ambient-operator')
    if (!localStorage.getItem('codeg.system_language_settings'))
      localStorage.setItem('codeg.system_language_settings', JSON.stringify({mode: 'manual', language: 'en'}))
    if (!localStorage.getItem('theme')) localStorage.setItem('theme', 'light')
  })
  await page.setViewportSize({width: 1280, height: 900})
  for (const path of ['/business.html', '/business', '/business/', '/', '/index.html']) {
    const before = evidence.calls.length
    const response = await page.goto(origin + path)
    await page.getByRole('heading', {name: 'Give shared work a clear next step.', exact: true}).waitFor()
    await page.evaluate(async () => { await document.fonts.ready; await new Promise(requestAnimationFrame); await new Promise(requestAnimationFrame) })
    evidence.paths.push({path, status: response.status(), apiCalls: evidence.calls.length - before, width: await page.evaluate(() => document.documentElement.scrollWidth)})
  }
  await page.screenshot({path: 'reports/review-business-workspace-evidence/cold-1280-light.png'})
  return evidence
}
