async page => {
  page.__reviewTraffic = []
  page.on('response', response => {
    const path = response.url().split('http://127.0.0.1:4354')[1]?.split('?')[0] || 'nonfixture'
    if (path.startsWith('/api/')) page.__reviewTraffic.push({ method: response.request().method(), path, status: response.status() })
  })
  await page.context().route('**/*', route => {
    const address = route.request().url()
    const local = address.startsWith('http://127.0.0.1:4354/')
    const path = local ? address.slice('http://127.0.0.1:4354'.length).split('?')[0] : 'nonfixture'
    const business = !path.startsWith('/api/') || path.startsWith('/api/business/')
    if (!local || !business || path.startsWith('/ws')) {
      page.__reviewTraffic.push({ blocked: true, path })
      return route.abort()
    }
    return route.continue()
  })
  await page.setViewportSize({ width: 1280, height: 900 })
  await page.goto('http://127.0.0.1:4354/business')
  await page.getByLabel('Personal access token', { exact: true }).waitFor()
  return { coldBusinessApiCalls: page.__reviewTraffic.length, url: page.url() }
}
