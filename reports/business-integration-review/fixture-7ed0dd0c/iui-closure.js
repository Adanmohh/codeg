async page => {
  const evidence = page.__reviewIui
  const check = (name, pass) => {
    evidence.assertions.push({ name, pass: Boolean(pass) })
    if (!pass) throw new Error(name)
  }
  const dialog = page.getByRole('dialog')
  await dialog.waitFor()
  check('Real task B link opens destination discard dialog', await dialog.getByRole('heading', { name: 'Leave this source review?', exact: true }).isVisible())
  check('Candidate A is still mounted behind modal with exact unsaved text', await page.locator('textarea').evaluateAll(elements => elements.some(element => element.isConnected && element.value === 'Synthetic unsaved source A sentinel')))
  await page.waitForTimeout(400)
  await page.screenshot({ path: 'reports/business-integration-review/fixture-7ed0dd0c/iui-dialog-1280.png' })
  await page.setViewportSize({ width: 390, height: 844 })
  await page.waitForTimeout(400)
  check('Dialog and draft survive mobile breakpoint', await dialog.isVisible() && await page.locator('textarea').evaluateAll(elements => elements.some(element => element.value === 'Synthetic unsaved source A sentinel')))
  await page.screenshot({ path: 'reports/business-integration-review/fixture-7ed0dd0c/iui-dialog-390.png' })
  await dialog.getByRole('button', { name: 'Keep editing', exact: true }).click()
  await dialog.waitFor({ state: 'detached' })
  const brief = page.getByRole('textbox', { name: 'Brief', exact: true })
  check('Keep editing retains original source A and unsaved exact Brief', await brief.inputValue() === 'Synthetic unsaved source A sentinel' && await page.getByRole('heading', { name: 'review: meeting-follow-up', exact: true }).isVisible())
  await brief.scrollIntoViewIfNeeded()
  await page.waitForTimeout(400)
  await page.screenshot({ path: 'reports/business-integration-review/fixture-7ed0dd0c/iui-kept-390.png' })
  await page.getByRole('tab', { name: 'My work', exact: true }).click()
  await page.getByRole('button', { name: /To do Feedback · Normal review: existing reviewed customer follow-up/ }).click()
  const link = page.getByRole('button', { name: 'review: meeting-long Open source review', exact: true })
  if (!(await link.isVisible())) await page.getByText('Source references', { exact: true }).click()
  await link.click()
  await dialog.waitFor()
  check('Repeated task B link again asks before discarding', await dialog.getByRole('heading', { name: 'Leave this source review?', exact: true }).isVisible())
  await dialog.getByRole('button', { name: 'Discard my draft', exact: true }).click()
  await dialog.waitFor({ state: 'detached' })
  await page.getByRole('heading', { name: 'review: meeting-long', exact: true }).waitFor()
  check('Explicit discard opens exact source B', await page.getByRole('heading', { name: 'review: meeting-long', exact: true }).isVisible())
  check('Unsaved source A text is removed only after explicit discard', await page.locator('textarea').evaluateAll(elements => !elements.some(element => element.value === 'Synthetic unsaved source A sentinel')))
  await page.waitForTimeout(400)
  await page.screenshot({ path: 'reports/business-integration-review/fixture-7ed0dd0c/iui-discarded-to-b-390.png' })
  evidence.traffic = page.__reviewTraffic.slice(evidence.trafficStart)
  check('No candidate/task/source writes during dirty navigation', !evidence.traffic.some(item => /\/(edit|accept|link|discard|create|update|submit|review|cancel|assign|note|progress|archive|revoke|upsert|start|advance|select)$/.test(item.path)))
  check('No legacy or external browser requests in owned session', page.__reviewTraffic.every(item => !item.blocked && item.path.startsWith('/api/business/')))
  check('Private draft and passage content absent from persistent browser storage', await page.evaluate(() => [localStorage, sessionStorage].every(store => Object.keys(store).every(key => !/Synthetic unsaved source A sentinel|Reviewer saved candidate A baseline|private evidence|private passage/.test(store.getItem(key) || '')))))
  evidence.status = 'passed'
  evidence.browserSession = 'review-intake-unified'
  evidence.viewports = ['1280x900', '390x844']
  evidence.responseMocking = false
  evidence.fixtureMutations = 'None during IUI navigation; preceding actual source refresh used review namespace only.'
  return evidence
}
