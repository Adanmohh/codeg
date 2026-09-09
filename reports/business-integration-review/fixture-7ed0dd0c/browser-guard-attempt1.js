async page => {
  page.__reviewTraffic = []
  page.on('response', response => {
    const url = new URL(response.url())
    if (url.pathname.startsWith('/api/')) page.__reviewTraffic.push({ method: response.request().method(), path: url.pathname, status: response.status() })
  })
  await page.context().route('**/*', route => {
    const url = new URL(route.request().url())
    const local = url.origin === 'http://127.0.0.1:4354'
    const business = !url.pathname.startsWith('/api/') || url.pathname.startsWith('/api/business/')
    if (!local || !business || url.pathname.startsWith('/ws')) {
      page.__reviewTraffic.push({ blocked: true, origin: url.origin, path: url.pathname })
      return route.abort()
    }
    return route.continue()
  })
  await page.setViewportSize({ width: 1280, height: 900 })
  await page.goto('http://127.0.0.1:4354/business')
  await page.getByLabel('Personal access token', { exact: true }).waitFor()
  console.log(JSON.stringify({ coldBusinessApiCalls: page.__reviewTraffic.length, url: page.url() }))
}
