async page => {
  await page.getByRole('button', {name: 'Shared work', exact: true}).click();
  await page.getByRole('button', {name: /Shape the website launch brief/}).click();
  await page.getByRole('button', {name: 'Edit task', exact: true}).click();
  await page.getByRole('textbox', {name: 'Brief', exact: true}).fill('Synthetic pane draft for locale, focus and responsive preservation.');
  await page.getByRole('button', {name: 'Show side by side', exact: true}).click();
  await page.getByRole('separator', {name: 'Resize work panes', exact: true}).waitFor();
  await page.getByRole('tabpanel', {name: 'Shared work', exact: true}).evaluate(el => { for (const child of el.querySelectorAll('*')) if (child.scrollHeight > child.clientHeight) child.scrollTop = 0; });
  await page.waitForTimeout(500);
  const measured = await page.evaluate(() => {
    const panel = [...document.querySelectorAll('[role=tabpanel]')].find(el => el.getAttribute('data-reference') === 'true') ?? [...document.querySelectorAll('[role=tabpanel]')].find(el => document.getElementById(el.getAttribute('aria-labelledby'))?.textContent === 'Shared work');
    if (!panel) throw Error('Missing reference pane');
    return {width: innerWidth, overflow: document.documentElement.scrollWidth > innerWidth, paneWidth: panel.getBoundingClientRect().width, titles: [...panel.querySelectorAll('h2')].map(el => ({text: el.textContent, width: el.getBoundingClientRect().width, height: el.getBoundingClientRect().height})), privateStored: JSON.stringify(localStorage).includes('Synthetic pane draft')};
  });
  if (measured.overflow || measured.privateStored || measured.titles.length !== 6 || measured.titles.some(t => t.width < 200 || t.height > 100)) throw Error('Pane list is not readable: ' + JSON.stringify(measured));
  await page.screenshot({path: 'reports/business-intake-ui-evidence/workbench-split-1280-light-after.png'});
  return measured;
}
