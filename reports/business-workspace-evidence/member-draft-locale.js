async page => {
  const dialog = page.getByRole('dialog', {name: 'Prepare the feedback summary · Synthetic', exact: true});
  const draft = await dialog.getByLabel('Brief', {exact: true}).inputValue();
  const preferences = await page.context().newPage();
  const blocked = [];
  await preferences.route('**/*', route => {
    if (route.request().url().startsWith('http://127.0.0.1:4340/')) return route.continue();
    blocked.push(route.request().url().split('?')[0]); return route.abort();
  });
  await preferences.goto('http://127.0.0.1:4340/business.html');
  await preferences.getByLabel('Language', {exact: true}).selectOption('ar');
  await dialog.getByLabel('وصف العمل', {exact: true}).waitFor();
  await page.setViewportSize({width: 390, height: 900});
  await page.bringToFront();
  const arabic = await dialog.getByLabel('وصف العمل', {exact: true}).inputValue();
  const layout = await dialog.evaluate(el => ({direction: document.documentElement.dir, width: innerWidth, pageWidth: document.documentElement.scrollWidth, dialogWidth: el.clientWidth, contentWidth: el.scrollWidth}));
  if (arabic !== draft || layout.pageWidth > 390 || layout.contentWidth > layout.dialogWidth || layout.direction !== 'rtl') throw Error('RTL/resize lost private edit or overflowed');
  await page.screenshot({path: 'reports/business-workspace-evidence/private-draft-390-arabic.png'});
  await preferences.getByLabel('المظهر', {exact: true}).selectOption('dark');
  await page.waitForFunction(() => document.documentElement.classList.contains('dark'));
  if (await dialog.getByLabel('وصف العمل', {exact: true}).inputValue() !== draft) throw Error('Theme switch lost draft');
  await page.screenshot({path: 'reports/business-workspace-evidence/private-draft-390-arabic-dark.png'});
  await preferences.getByLabel('اللغة', {exact: true}).selectOption('en');
  await preferences.getByLabel('Appearance', {exact: true}).selectOption('light');
  await dialog.getByLabel('Brief', {exact: true}).waitFor();
  if (await dialog.getByLabel('Brief', {exact: true}).inputValue() !== draft) throw Error('English return lost draft');
  await preferences.close();
  await page.bringToFront();
  await page.setViewportSize({width: 1280, height: 900});
  const stored = await page.evaluate(() => Object.values(localStorage).some(value => value.includes('PRIVATE SYNTHETIC DRAFT') || value.startsWith('bdm_')));
  if (stored || blocked.length) throw Error('Private draft/credential stored or external request attempted');
  return {synthetic: true, crossTabPreferenceUI: true, arabicAndEnglishDraftPreserved: true, darkDraftPreserved: true, layout, privateStored: stored, blocked, remainingTabs: page.context().pages().length};
}
