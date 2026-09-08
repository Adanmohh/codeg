async page => {
  const dialog = page.getByRole('dialog', {name: 'Synthetic review: customer welcome brief', exact: true})
  await dialog.waitFor()
  const started = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/progress'))
  await dialog.getByRole('button', {name: 'Start work', exact: true}).click()
  const startResponse = await started
  const start = await startResponse.json()
  await dialog.locator('summary').filter({hasText: /^Submit for review$/}).click()
  await dialog.getByLabel('Deliverable', {exact: true}).fill('Synthetic review deliverable: Welcome new customers with a clear first step. Prepared by Noura for Maya to review.')
  const submitted = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/submit'))
  await dialog.locator('details[open]').getByRole('button', {name: 'Submit for review', exact: true}).click()
  const submitResponse = await submitted
  const result = await submitResponse.json()
  await dialog.getByText('Revision 3', {exact: true}).waitFor()
  return {startedStatus: startResponse.status(), startedRevision: start.task.revision, submitStatus: submitResponse.status(), task: result.task, deliverable: result.deliverables, memberReviewControlCount: await dialog.getByRole('button', {name: 'Accept work', exact: true}).count(), execution: result.execution}
}
