// Report-only Playwright CLI orchestration. No response replacement or account writes.
async page => {
  const origin = 'http://127.0.0.1:4346'
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
  for (const path of ['/business.html']) {
    const before = evidence.calls.length
    const response = await page.goto(origin + path)
    await page.getByRole('heading', {name: 'Give shared work a clear next step.', exact: true}).waitFor()
    await page.evaluate(async () => { await document.fonts.ready; await new Promise(requestAnimationFrame); await new Promise(requestAnimationFrame) })
    evidence.paths.push({path, status: response.status(), apiCalls: evidence.calls.length - before, width: await page.evaluate(() => document.documentElement.scrollWidth)})
  }
  await page.screenshot({path: 'reports/review-business-workspace-evidence/final-connect-1280-light.png'})
  const fixture = await page.evaluate(async()=>{
    const metadata=await (await fetch('/__business_fixture')).json()
    const body=await (await fetch('/business.html')).arrayBuffer()
    const digest=await crypto.subtle.digest('SHA-256',body)
    const sha256=[...new Uint8Array(digest)].map(v=>v.toString(16).padStart(2,'0')).join('')
    return {export:metadata.export,backendPort:metadata.backendPort,sha256}
  })
  if(fixture.export!=='.build/business-workspace/review-export'||fixture.backendPort!==4342||fixture.sha256!=='c029f99d5c92e677c97f2ebe45f4bd185746b73e84e77912a683e1d92de55a54') throw Error('Corrected export identity mismatch')
  return {fixture,evidence}
}
