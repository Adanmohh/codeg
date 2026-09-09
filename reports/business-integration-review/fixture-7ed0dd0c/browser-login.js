async page => {
  // Only an ephemeral file input transfers the already-authorized private file
  // into this owned browser's memory. No token literal, saved state or snapshot.
  await page.evaluate(() => {
    const input = document.createElement('input')
    input.type = 'file'
    input.id = 'review-private-credential-transfer'
    input.hidden = true
    document.body.appendChild(input)
  })
  let credentials
  try {
    const input = page.locator('#review-private-credential-transfer')
    await input.setInputFiles('/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-intake-fixtures/unified-YQmRz8/review-credentials.json')
    credentials = await input.evaluate(async element => JSON.parse(await element.files[0].text()))
    await input.evaluate(element => element.remove())
    if (!credentials.synthetic || credentials.namespace !== 'review') throw new Error('Wrong synthetic namespace')
    await page.getByLabel('Workspace address', { exact: true }).fill('http://127.0.0.1:4354')
    await page.getByLabel('Personal access token', { exact: true }).fill(credentials.sessions.owner.token)
    const pending = page.waitForResponse(response => response.url().endsWith('/api/business/context') && response.request().method() === 'POST')
    await page.getByRole('button', { name: 'Connect', exact: true }).click()
    const response = await pending
    await page.getByLabel('Personal access token', { exact: true }).waitFor({ state: 'detached' })
    const identity = await response.json()
    const pass = response.status() === 200 && identity.organization.id === credentials.organization.id && identity.member.id === credentials.sessions.owner.memberId && identity.operator === false
    if (!pass) throw new Error('Unexpected protected principal')
    return { login: 'passed', status: response.status(), namespace: 'review', operator: identity.operator, tokenFieldDetached: true, coldOrAuthenticatedTraffic: page.__reviewTraffic }
  } catch {
    // Withhold arbitrary framework errors that could include a filled value.
    for (const field of await page.locator('input[type="password"]').all()) await field.fill('').catch(() => {})
    throw new Error('Private credential login did not complete; details withheld')
  } finally {
    credentials = undefined
    await page.locator('#review-private-credential-transfer').evaluateAll(elements => elements.forEach(element => element.remove()))
  }
}
