async page => {
  const meta = await page.evaluate(async () => (await fetch('/__business_intake_fixture')).json());
  if (!meta.synthetic || meta.workspaceBackendPort !== 4353 || !meta.export.endsWith('scoped-settings')) throw Error('Wrong synthetic fixture');
  await page.getByRole('combobox', { name: 'Language', exact: true }).selectOption('en');
  await page.getByText('Administrator access', { exact: true }).click();
  await page.getByRole('checkbox', { name: 'Original administrator token', exact: true }).check();
  await page.locator('input[type="password"]').fill('business-tasks-synthetic-operator');
  await page.getByRole('button', { name: 'Connect', exact: true }).click();
  await page.getByLabel('Organization name', { exact: true }).fill('Synthetic Studio');
  await page.getByLabel('Your name', { exact: true }).fill('Synthetic Platform Operator');
  await page.getByRole('button', { name: 'Create workspace', exact: true }).click();
  await page.getByRole('heading', { name: 'My work', exact: true }).waitFor();
  await page.getByRole('button', { name: 'Create task', exact: true }).waitFor();
  await page.screenshot({ path: 'reports/business-intake-ui-evidence/workbench-empty-1280-light.png' });
  return await page.evaluate(() => ({
    title: document.querySelector('h1')?.textContent,
    emptyVisible: document.body.innerText.includes('Nothing assigned to you'),
    nativeChrome: !!document.querySelector('[data-business-native-chrome]'),
    theme: document.querySelector('[data-business-appearance]')?.getAttribute('data-theme'),
    overflow: document.documentElement.scrollWidth > innerWidth,
    tokenPersisted: JSON.stringify(localStorage).includes('business-tasks-synthetic-operator'),
  }));
}
