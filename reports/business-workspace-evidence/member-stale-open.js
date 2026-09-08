async page => {
  await page.getByRole('button', {name: 'Refresh', exact: true}).click();
  await page.getByRole('button').filter({has: page.getByRole('heading', {name: 'Shape the customer welcome · Synthetic', exact: true})}).click();
  const dialog = page.getByRole('dialog', {name: 'Shape the customer welcome · Synthetic', exact: true});
  await dialog.getByRole('button', {name: 'Edit task', exact: true}).click();
  await dialog.getByLabel('Brief', {exact: true}).fill('Synthetic member draft kept through a concurrent edit. Welcome customers with one clear next step.');
  const calendar = await dialog.getByLabel('Due date', {exact: true}).inputValue();
  if (calendar !== '2026-10-01') throw Error('Calendar date shifted');
  return {synthetic: true, openedRevision: 1, independentPersonalMember: true, calendar, privateDraftReady: true};
}
