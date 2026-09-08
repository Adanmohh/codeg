async page => {
  await page.bringToFront();
  const refresh = page.getByRole('button', {name: 'Refresh', exact: true});
  if (await refresh.count()) await refresh.click();
  await page.getByLabel('Personal access token', {exact: true}).waitFor();
  const error = await page.getByText('Your access has expired or was revoked.', {exact: true}).isVisible();
  const retained = await page.evaluate(() => document.body.textContent.includes('Prepare the feedback summary') || Object.values(localStorage).some(value => value.startsWith('bdm_') || value.includes('PRIVATE SYNTHETIC DRAFT')));
  if (!error || retained || await page.getByLabel('Personal access token', {exact: true}).inputValue() !== '') throw Error('Revocation retained private state or failed recovery');
  await page.screenshot({path: 'reports/business-workspace-evidence/generation2/member-revoked-real.png'});
  return {synthetic: true, expiredRecoveryVisible: error, retainedPrivateState: retained, credentialCleared: true};
}
