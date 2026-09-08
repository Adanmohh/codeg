async page => {
  await page.getByRole('button').filter({has: page.getByRole('heading', {name: 'Shape the customer welcome · Synthetic', exact: true})}).click();
  const dialog = page.getByRole('dialog', {name: 'Shape the customer welcome · Synthetic', exact: true});
  await dialog.getByRole('button', {name: 'Edit task', exact: true}).click();
  await dialog.getByLabel('Brief', {exact: true}).fill('Synthetic owner revision: keep the welcome focused on the first useful action.');
  const pending = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/update'));
  await dialog.getByRole('button', {name: 'Save changes', exact: true}).click();
  const response = await pending;
  if (response.status() !== 200) throw Error('Owner save failed: ' + response.status());
  const {task} = await response.json();
  await dialog.getByRole('button', {name: 'Edit task', exact: true}).waitFor();
  return {synthetic: true, taskId: task.id, revision: task.revision, dueDate: task.dueDate, updatedThroughUI: true};
}
