// Actual protected synthetic API + UI. No response/header/credential recording.
async page => {
  const meta = await page.evaluate(async () => (await fetch('/__business_intake_fixture')).json());
  if (!meta.synthetic || meta.workspaceBackendPort !== 4353) throw Error('Wrong fixture');
  const originalRoot = await page.evaluate(() => document.documentElement.getAttribute('data-theme'));
  await page.getByRole('button', {name: 'Workspace appearance', exact: true}).click();
  await page.getByRole('combobox', {name: 'Workspace colors', exact: true}).selectOption('blue');
  const saved = page.waitForResponse(r => r.url().endsWith('/api/business/settings/update') && r.request().method() === 'POST');
  await page.getByRole('button', {name: 'Save workspace defaults', exact: true}).click();
  const response = await saved;
  if (response.status() !== 200) throw Error('Settings save status ' + response.status());
  await page.getByText('Workspace defaults saved.', {exact: true}).waitFor();
  await page.getByRole('tab', {name: 'Synthetic · Shape the website launch brief', exact: true}).click();
  const draft = await page.getByRole('textbox', {name: 'Brief', exact: true}).inputValue();
  const result = await page.evaluate(() => ({
    root: document.documentElement.getAttribute('data-theme'),
    scopedPalette: document.querySelector('[data-business-appearance]')?.getAttribute('data-theme'),
    legacyEntry: !!document.querySelector('a[href="/workspace"]'),
    privateStored: JSON.stringify(localStorage).includes('Synthetic private draft retained'),
  }));
  if (draft !== 'Synthetic private draft retained across tabs, panes, locale and appearance.') throw Error('Private draft changed');
  if (result.root !== originalRoot || result.scopedPalette !== 'blue' || result.privateStored || result.legacyEntry) throw Error('Appearance/scope isolation failed');
  return {settingsStatus: response.status(), privateDraftRetained: true, ...result};
}
