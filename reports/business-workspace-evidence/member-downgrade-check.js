async page => {
  await page.bringToFront();
  await page.getByText('You can read this work. Your current access does not allow changes.', {exact: true}).waitFor({timeout: 35000});
  const privatePresent = await page.evaluate(() => document.body.textContent.includes('PRIVATE SYNTHETIC DRAFT') || [...document.querySelectorAll('textarea')].some(el => el.value.includes('PRIVATE SYNTHETIC DRAFT')));
  if (privatePresent || await page.getByRole('dialog').count() || await page.getByRole('button', {name: 'Create task', exact: true}).count()) throw Error('Downgrade retained private draft or write controls');
  await page.getByRole('button').filter({has: page.getByRole('heading', {name: 'Prepare the feedback summary · Synthetic', exact: true})}).click();
  const dialog = page.getByRole('dialog', {name: 'Prepare the feedback summary · Synthetic', exact: true});
  await dialog.getByText('You can read this work. Your current access does not allow changes.', {exact: true}).waitFor();
  for (const action of ['Edit task','Responsibility','Start work','Accept work','Add note','Cancel task','Archive task']) {
    if (await dialog.getByRole('button', {name: action, exact: true}).count()) throw Error('Viewer sees write action: ' + action);
  }
  await page.screenshot({path: 'reports/business-workspace-evidence/viewer-task-real.png'});
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  return {synthetic: true, actualMembershipRevalidation: true, role: 'viewer', privateDraftCleared: !privatePresent, dialogUnmountedOnRevision: true, noWriteControls: true};
}
