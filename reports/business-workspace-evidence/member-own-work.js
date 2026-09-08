async page => {
  await page.getByRole('button', {name: 'Create task', exact: true}).first().click();
  const form = page.getByRole('dialog', {name: 'Create task', exact: true});
  if (!(await form.getByLabel('Accountable owner', {exact: true}).isDisabled()) || !(await form.getByLabel('Reviewer', {exact: true}).isDisabled())) throw Error('Member can change restricted assignment');
  const executors = await form.getByLabel('Assigned to', {exact: true}).locator('option').allTextContents();
  if (executors.some(value => value.includes('Atlas') || value.includes('Amal'))) throw Error('Member sees unauthorized creation assignments');
  await form.getByLabel('Task title', {exact: true}).fill('Prepare the feedback summary · Synthetic');
  await form.getByLabel('Brief', {exact: true}).fill('Synthetic member-owned work, created without a conversation, folder, agent or engine.');
  await form.getByLabel('Area of work', {exact: true}).selectOption('feedback');
  await form.getByLabel('Assigned to', {exact: true}).selectOption({label: 'Nabil · Synthetic member'});
  const pending = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/create'));
  await form.getByRole('button', {name: 'Create task', exact: true}).click();
  const response = await pending;
  if (response.status() !== 200) throw Error('Member create failed');
  const {task} = await response.json();
  if (task.ownerId !== task.assigneeId || task.ownerId !== task.creatorId || task.reviewerId !== null || task.dueDate !== null) throw Error('Self/null creation contract differs');
  const dialog = page.getByRole('dialog', {name: task.title, exact: true});
  await dialog.getByRole('button', {name: 'Edit task', exact: true}).click();
  await dialog.getByLabel('Brief', {exact: true}).fill('PRIVATE SYNTHETIC DRAFT: retained across language and viewport changes; cleared on membership downgrade.');
  return {synthetic: true, taskId: task.id, ownerId: task.ownerId, assigneeId: task.assigneeId, reviewerId: task.reviewerId, dueDate: task.dueDate, restrictedAssignment: true, privateDraftReady: true};
}
