async page => {
  const current = page.getByRole('dialog');
  if (await current.count()) await current.getByRole('button', {name: 'Close', exact: true}).click();
  await page.setViewportSize({width: 390, height: 900});
  await page.getByRole('button').filter({has: page.getByRole('heading', {name: /^Synthetic long title/})}).click();
  const dialog = page.getByRole('dialog');
  await dialog.getByRole('button', {name: 'Edit task', exact: true}).waitFor();
  await page.waitForFunction(() => { const node = document.querySelector('[role=dialog]'); return node && getComputedStyle(node).opacity === '1' && getComputedStyle(node).transform === 'none'; });
  await page.screenshot({path: 'reports/business-workspace-evidence/long-detail-before-390.png'});
  const measured = await dialog.evaluate(el => {
    const box = el.getBoundingClientRect();
    return {viewport: innerWidth, pageWidth: document.documentElement.scrollWidth, dialog: {left: box.left, right: box.right, width: box.width, client: el.clientWidth, scroll: el.scrollWidth}, overflowing: [...el.querySelectorAll('*')].filter(node => {const rect = node.getBoundingClientRect(); return rect.width > 0 && (rect.right > box.right + 1 || rect.left < box.left - 1);}).map(node => ({tag: node.tagName, text: (node.textContent || '').slice(0, 60), width: node.getBoundingClientRect().width, class: node.className})).slice(0, 12)};
  });
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  await page.setViewportSize({width: 1280, height: 900});
  return {synthetic: true, measured};
}
