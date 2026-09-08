async page => {
  await page.setViewportSize({width: 1280, height: 900});
  await page.emulateMedia({colorScheme: 'light', reducedMotion: 'reduce'});
  await page.getByText('Administrator access', {exact: true}).click();
  await page.getByLabel('Original administrator token', {exact: true}).check();
  await page.locator('input[type=password]').fill('business-tasks-synthetic-operator');
  await page.getByRole('button', {name: 'Connect', exact: true}).click();
  await page.getByRole('heading', {name: 'My work', exact: true}).waitFor();
  await page.getByRole('button', {name: 'Create task', exact: true}).first().click();
  const form = page.getByRole('dialog', {name: 'Create task', exact: true});
  const title = 'Worker conflict clarity · Synthetic';
  await form.getByLabel('Task title', {exact: true}).fill(title);
  await form.getByLabel('Brief', {exact: true}).fill('Synthetic worker-only recovery check. No provider or engine action.');
  const creating = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/create'));
  await form.getByRole('button', {name: 'Create task', exact: true}).click();
  const createdResponse = await creating;
  if (createdResponse.status() !== 200) throw Error('Synthetic creation failed: ' + createdResponse.status());
  const created = (await createdResponse.json()).task;
  const dialog = page.getByRole('dialog', {name: title, exact: true});
  await dialog.getByRole('button', {name: 'Edit task', exact: true}).click();
  const draft = 'Unsubmitted worker draft retained during conflict.';
  await dialog.getByLabel('Brief', {exact: true}).fill(draft);
  const current = await page.evaluate(async task => {
    let saved = task;
    for (const status of ['in_progress', 'review']) {
      const response = await fetch('/api/business/tasks/progress', {
        method: 'POST', credentials: 'omit', redirect: 'error',
        headers: {'Content-Type': 'application/json', Authorization: 'Bearer business-tasks-synthetic-operator'},
        body: JSON.stringify({input: {taskId: task.id, expectedRevision: saved.revision, status}}),
      });
      if (response.status !== 200) throw Error('Synthetic concurrent progress failed: ' + response.status);
      saved = (await response.json()).task;
    }
    return {id: saved.id, status: saved.status, revision: saved.revision};
  }, created);
  const saving = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/update'));
  await dialog.getByRole('button', {name: 'Save changes', exact: true}).click();
  const conflict = await saving;
  if (conflict.status() !== 409) throw Error('Expected real conflict');
  await dialog.getByRole('button', {name: 'Load current task', exact: true}).click();
  const comparison = dialog.getByRole('heading', {name: 'Current saved version', exact: true}).locator('..');
  await comparison.waitFor();
  const missingCurrentStatus = await comparison.getByText('Review', {exact: true}).count() === 0;
  const missingCurrentRevision = await comparison.getByText('Revision 3', {exact: true}).count() === 0;
  const baseUnlabelled = await dialog.getByText("Your draft's base version", {exact: true}).count() === 0;
  const locked = await dialog.getByRole('button', {name: 'Save changes', exact: true}).isDisabled();
  if (!missingCurrentStatus || !missingCurrentRevision || !baseUnlabelled || !locked || await dialog.getByLabel('Brief', {exact: true}).inputValue() !== draft) throw Error('Before-state differs from reviewed finding');
  await page.screenshot({path: 'reports/business-workspace-evidence/conflict-clarity-before.png'});
  return {synthetic: true, fixturePort: 4346, frontend: '33b9cbcb', taskId: created.id, base: {status: created.status, revision: created.revision}, current, conflictStatus: 409, missingCurrentStatus, missingCurrentRevision, baseUnlabelled, draftPreserved: true, writesLocked: locked};
}
