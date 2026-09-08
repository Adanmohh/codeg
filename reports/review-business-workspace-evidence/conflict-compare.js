async page => {
  const saving = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/update'))
  await page.getByRole('button', {name: 'Save changes', exact: true}).click()
  const response = await saving
  await page.getByText('This task changed while you were editing.', {exact: true}).waitFor()
  const draft = await page.getByLabel('Brief', {exact: true}).inputValue()
  const fetching = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/get'))
  await page.getByRole('button', {name: 'Load current task', exact: true}).click()
  const currentResponse = await fetching
  const current = await currentResponse.json()
  const heading = page.getByRole('heading', {name: 'Current saved version', exact: true})
  await heading.waitFor()
  await heading.scrollIntoViewIfNeeded()
  await page.evaluate(async () => { await document.fonts.ready; await new Promise(requestAnimationFrame); await new Promise(requestAnimationFrame) })
  const section = page.locator('section').filter({has: heading}).first()
  const comparison = await section.innerText()
  const snapshot = await page.getByRole('dialog').ariaSnapshot()
  const disabled = await page.getByRole('button', {name: 'Save changes', exact: true}).isDisabled()
  await page.screenshot({path: 'reports/review-business-workspace-evidence/conflict-current-390-light.png'})
  return {saveStatus: response.status(), draft, saveDisabled: disabled, actualCurrent: {status: current.task.status, revision: current.task.revision}, comparison, snapshot, comparisonHasRevision: /Revision\s+3/.test(comparison), comparisonHasStatus: /\bReview\b/.test(comparison)}
}
