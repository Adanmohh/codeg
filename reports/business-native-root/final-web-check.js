async page => {
  const origin = 'http://127.0.0.1:4318'
  const blocked = []
  const api = []
  let sockets = 0
  await page.route('**/*', async route => {
    const url = route.request().url()
    if (!url.startsWith(origin + '/') ||
        (url.startsWith(origin + '/api/') && !url.startsWith(origin + '/api/business/'))) {
      blocked.push('outside-business-boundary')
      await route.abort()
      return
    }
    await route.continue()
  })
  page.on('request', request => {
    if (request.url().includes('/api/')) api.push(request.method())
  })
  page.on('websocket', () => { sockets++ })
  await page.addInitScript(() => {
    try { localStorage.setItem('codeg_token', 'ops-desk-local-test') } catch {}
  })
  await page.setViewportSize({ width: 1280, height: 900 })
  const cold = []
  for (const path of ['/', '/index.html', '/business', '/business/', '/business.html']) {
    const before = api.length
    const response = await page.goto(origin + path)
    await page.getByRole('heading', { name: 'Sign in to your workspace', exact: true }).waitFor()
    await page.waitForTimeout(200)
    const geometry = await page.evaluate(() => ({
      width: document.documentElement.scrollWidth,
      viewport: innerWidth,
      nativeChrome: document.querySelectorAll('[data-business-native-chrome]').length,
      dragTargets: document.querySelectorAll('[data-tauri-drag-region]').length,
    }))
    if (response.status() !== 200 || api.length !== before || sockets || blocked.length ||
        geometry.width !== geometry.viewport || geometry.nativeChrome || geometry.dragTargets)
      throw new Error('Final cold business route failed')
    cold.push({ path, status: response.status(), apiRequests: api.length-before, ...geometry })
  }
  await page.getByLabel('Personal access token', { exact: true }).fill('ops-desk-local-test')
  await page.getByRole('button', { name: 'Connect', exact: true }).click()
  await page.getByRole('heading', { name: 'Start with your organization.', exact: true }).waitFor()
  await page.getByLabel('Organization name', { exact: true }).fill('Root Integrated Studio')
  await page.getByLabel('Your name', { exact: true }).fill('Root Integration Owner')
  await page.getByRole('button', { name: 'Create workspace', exact: true }).click()
  await page.getByRole('heading', { name: 'My work', exact: true }).waitFor()
  await page.getByRole('button', { name: 'Create task', exact: true }).click()
  const title = 'Final package: customer feedback follow-up'
  await page.getByLabel('Task title', { exact: true }).fill(title)
  await page.getByLabel('Brief', { exact: true }).fill('Keep this draft intact while the window changes size.')
  const draft = []
  for (const width of [400, 1280]) {
    await page.setViewportSize({ width, height: 900 })
    if (await page.getByLabel('Task title', { exact: true }).inputValue() !== title)
      throw new Error('Final layout lost task draft')
    const g = await page.evaluate(() => ({ width: document.documentElement.scrollWidth, viewport: innerWidth }))
    if (g.width !== g.viewport) throw new Error('Final layout overflow')
    draft.push(g)
  }
  const dialog = page.getByRole('dialog')
  await dialog.getByRole('button', { name: 'Create task', exact: true }).click()
  await page.getByRole('heading', { name: title, exact: true }).waitFor()
  await page.screenshot({ path: 'reports/business-native-root/final-web-task.png' })
  if (blocked.length || sockets) throw new Error('Unexpected final business boundary access')
  return { cold, draft, taskCreated: true, blocked, sockets, apiRequests: api.length,
    scope: 'actual final integrated server, synthetic operator bootstrap/task; no native API emulation' }
}
