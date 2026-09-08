async page => {
  await page.goto('http://127.0.0.1:4346/business.html');
  await page.setViewportSize({width: 1280, height: 900});
  await page.getByText('Administrator access', {exact: true}).click();
  await page.getByLabel('Original administrator token', {exact: true}).check();
  await page.locator('input[type=password]').fill('business-tasks-synthetic-operator');
  await page.getByRole('button', {name: 'Connect', exact: true}).click();
  const title = 'Worker conflict clarity · Synthetic';
  await page.getByRole('button', {name: new RegExp(title)}).click();
  const dialog = page.getByRole('dialog', {name: title, exact: true});
  await dialog.getByRole('button', {name: 'Edit task', exact: true}).click();
  const draft = 'Corrected comparison worker draft, still private before saving.';
  await dialog.getByLabel('Brief', {exact: true}).fill(draft);
  const current = await page.evaluate(async () => {
    const response = await fetch('/api/business/tasks/progress', {
      method: 'POST', credentials: 'omit', redirect: 'error',
      headers: {'Content-Type': 'application/json', Authorization: 'Bearer business-tasks-synthetic-operator'},
      body: JSON.stringify({input: {taskId: 'ec9d106d-fa40-4e26-b2dc-612cedd587ad', expectedRevision: 3, status: 'todo'}}),
    });
    if (response.status !== 200) throw Error('Worker-only concurrent change failed: ' + response.status);
    const {task} = await response.json();
    return {status: task.status, revision: task.revision};
  });
  const saving = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/update'));
  await dialog.getByRole('button', {name: 'Save changes', exact: true}).click();
  if ((await saving).status() !== 409) throw Error('Expected real conflict');
  await dialog.getByRole('button', {name: 'Load current task', exact: true}).click();
  const base = dialog.getByRole('group', {name: "Your draft's base version", exact: true});
  const comparison = dialog.getByRole('region', {name: 'Current saved version', exact: true});
  await comparison.waitFor();
  if (await base.getByText('Review', {exact: true}).count() !== 1 || await base.getByText('Revision 3', {exact: true}).count() !== 1 || await comparison.getByText('To do', {exact: true}).count() !== 1 || await comparison.getByText('Revision 4', {exact: true}).count() !== 1) throw Error('Revision/status comparison is unclear');
  if (!(await dialog.getByRole('button', {name: 'Save changes', exact: true}).isDisabled())) throw Error('Stale save was not locked');
  await base.scrollIntoViewIfNeeded();
  await page.screenshot({path: 'reports/business-workspace-evidence/conflict-clarity-after-1280-light.png'});
  const preferences = await page.context().newPage();
  await preferences.goto('http://127.0.0.1:4346/business.html');
  await preferences.getByLabel('Language', {exact: true}).selectOption('ar');
  const arabicBase = dialog.getByRole('group', {name: 'النسخة التي تستند إليها مسودتك', exact: true});
  const arabicCurrent = dialog.getByRole('region', {name: 'النسخة المحفوظة الحالية', exact: true});
  await arabicCurrent.waitFor();
  await page.setViewportSize({width: 390, height: 900});
  await page.bringToFront();
  await arabicBase.scrollIntoViewIfNeeded();
  const layout = await dialog.evaluate(el => ({direction: document.documentElement.dir, viewport: innerWidth, page: document.documentElement.scrollWidth, dialog: el.clientWidth, content: el.scrollWidth}));
  if (layout.direction !== 'rtl' || layout.page > 390 || layout.content > layout.dialog) throw Error('Comparison overflow/RTL failure');
  if (await arabicBase.getByText('النسخة 3', {exact: true}).count() !== 1 || await arabicCurrent.getByText('النسخة 4', {exact: true}).count() !== 1 || await dialog.getByLabel('وصف العمل', {exact: true}).inputValue() !== draft) throw Error('Arabic comparison or draft mismatch');
  await page.screenshot({path: 'reports/business-workspace-evidence/conflict-clarity-after-390-ar-light.png'});
  await preferences.getByLabel('المظهر', {exact: true}).selectOption('dark');
  await page.waitForFunction(() => document.documentElement.classList.contains('dark'));
  await page.screenshot({path: 'reports/business-workspace-evidence/conflict-clarity-after-390-ar-dark.png'});
  await preferences.getByLabel('اللغة', {exact: true}).selectOption('en');
  await preferences.getByLabel('Appearance', {exact: true}).selectOption('light');
  await comparison.waitFor();
  await preferences.close();
  await page.bringToFront();
  await page.setViewportSize({width: 1280, height: 900});
  await dialog.getByRole('button', {name: 'Use my draft with this version', exact: true}).click();
  if (await dialog.getByLabel('Brief', {exact: true}).inputValue() !== draft) throw Error('Adoption lost draft');
  const adoptedSave = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/update'));
  await dialog.getByRole('button', {name: 'Save changes', exact: true}).click();
  const response = await adoptedSave;
  if (response.status() !== 200) throw Error('Explicit recovery save failed');
  const saved = (await response.json()).task;
  if (saved.revision !== 5 || saved.notes !== draft) throw Error('Saved wrong draft/version');
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  return {synthetic: true, frontend: '095c6164', fixturePort: 4346, taskId: saved.id, conflictStatus: 409, labelledBase: {status: 'review', revision: 3}, current, writesLockedBeforeAdopt: true, localeDraftPreserved: true, layout, savedRevision: saved.revision, savedStatus: saved.status};
}
