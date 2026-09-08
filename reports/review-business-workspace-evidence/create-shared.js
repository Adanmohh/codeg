async page => {
  const dialog = page.getByRole('dialog', {name: 'Create task', exact: true})
  const members = page.__reviewIdentity.members
  await dialog.getByLabel('Task title', {exact: true}).fill('Synthetic review: customer welcome brief')
  await dialog.getByLabel('Brief', {exact: true}).fill('Explain the first useful customer outcome. A person prepares the brief; Maya makes the final decision.')
  await dialog.getByLabel('Assigned to', {exact: true}).selectOption(members.find(m => m.role === 'member').id)
  await dialog.getByLabel('Reviewer', {exact: true}).selectOption(members.find(m => m.role === 'manager').id)
  await dialog.getByLabel('Due date', {exact: true}).fill('2026-10-01')
  const pending = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/create'))
  await dialog.getByRole('button', {name: 'Create task', exact: true}).click()
  const response = await pending
  if (response.status() !== 200) throw Error('Task creation status ' + response.status())
  const detail = await response.json()
  page.__reviewTaskId = detail.task.id
  await page.getByRole('dialog', {name: detail.task.title, exact: true}).waitFor()
  await page.screenshot({path: 'reports/review-business-workspace-evidence/created-human-1280-light.png'})
  return {task: detail.task, execution: detail.execution, activity: detail.activity.map(a => ({kind: a.kind, actor: a.actor, revision: a.revision})), dialog: await page.getByRole('dialog').ariaSnapshot()}
}
