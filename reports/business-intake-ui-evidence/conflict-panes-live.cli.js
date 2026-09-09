// Exact stale revision from the other personal session; no simulated response.
async page => {
  const meta = await page.evaluate(async () => (await fetch('/__business_intake_fixture')).json());
  if (!meta.synthetic || meta.workspaceBackendPort !== 4353) throw Error('Wrong fixture');
  const expectedDraft = 'Synthetic private draft retained across tabs, panes, locale and appearance.';
  const save = page.getByRole('button', {name: 'Save changes', exact: true});
  const conflict = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/update') && r.request().method() === 'POST');
  await save.click();
  const stale = await conflict;
  if (stale.status() !== 409) throw Error('Expected real409, got ' + stale.status());
  await page.getByRole('button', {name: 'Load current task', exact: true}).click();
  const current = page.getByRole('region', {name: 'Current saved version', exact: true});
  await current.waitFor();
  const baseText = await page.getByRole('group', {name: "Your draft's base version", exact: true}).innerText();
  const savedText = await current.innerText();
  if (!baseText.includes('Revision 2') || !savedText.includes('Revision 3') || !savedText.includes('Review')) throw Error('Unclear comparison');
  if (await page.getByRole('textbox', {name: 'Brief', exact: true}).inputValue() !== expectedDraft || !await save.isDisabled()) throw Error('Stale draft not preserved/locked');
  await page.screenshot({path: 'reports/business-intake-ui-evidence/workbench-conflict-1280-light.png'});
  await page.getByRole('button', {name: 'Use my draft with this version', exact: true}).click();
  const changed = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/update') && r.request().method() === 'POST');
  await save.click();
  const response = await changed;
  if (response.status() !== 200) throw Error('Explicit adopted save status ' + response.status());
  const result = await response.json();
  if (result.task.revision !== 4 || result.task.notes !== expectedDraft || result.execution !== null) throw Error('Wrong saved draft or unexpected execution');
  return {conflictStatus: stale.status(), baseRevision: 2, currentRevision: 3, currentStatus: 'review', retainedDraft: true, lockedUntilAdopt: true, explicitSave: response.status(), savedRevision: result.task.revision, execution: result.execution};
}
