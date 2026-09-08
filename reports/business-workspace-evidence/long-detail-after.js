async page => {
  await page.setViewportSize({width: 390, height: 900});
  await page.getByRole('button').filter({has: page.getByRole('heading', {name: /^Synthetic long title/})}).click();
  const dialog = page.getByRole('dialog');
  await dialog.getByRole('button', {name: 'Edit task', exact: true}).waitFor();
  await page.waitForFunction(() => { const node = document.querySelector('[role=dialog]'); return node && getComputedStyle(node).opacity === '1' && getComputedStyle(node).transform === 'none'; });
  const measured = await dialog.evaluate(el => ({viewport: innerWidth, pageWidth: document.documentElement.scrollWidth, dialogWidth: el.clientWidth, contentWidth: el.scrollWidth, opacity: getComputedStyle(el).opacity, transform: getComputedStyle(el).transform, grid: getComputedStyle(el).gridTemplateColumns, wrap: getComputedStyle(el).overflowWrap}));
  if (measured.pageWidth > 390 || measured.contentWidth > measured.dialogWidth) throw Error('Long content still overflows: ' + JSON.stringify(measured));
  await page.screenshot({path: 'reports/business-workspace-evidence/long-detail-after-390.png'});
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  await page.setViewportSize({width: 1280, height: 900});
  return {synthetic: true, measured, actualRebuiltExport: true};
}
