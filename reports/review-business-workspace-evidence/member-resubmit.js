async page => {
  await page.getByRole('dialog').getByRole('button', {name: 'Close', exact: true}).click()
  await page.getByRole('button', {name: 'Refresh', exact: true}).click()
  await page.getByRole('button', {name: /Synthetic review: customer welcome brief/}).click()
  const dialog = page.getByRole('dialog', {name: 'Synthetic review: customer welcome brief', exact: true})
  await dialog.getByText('Revision 4', {exact: true}).waitFor()
  await dialog.locator('summary').filter({hasText: /^Submit for review$/}).click()
  await dialog.getByLabel('Deliverable', {exact: true}).fill('Synthetic review revised deliverable: the customer receives a concise welcome brief with a clear next step.')
  const pending = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/submit'))
  await dialog.locator('details[open]').getByRole('button', {name: 'Submit for review', exact: true}).click()
  const response = await pending
  const result = await response.json()
  await dialog.getByText('Revision 5', {exact: true}).waitFor()
  return {status: response.status(), revision: result.task.revision, taskStatus: result.task.status, execution: result.execution, author: result.deliverables.find(d => d.id === result.task.currentDeliverableId).author}
}
