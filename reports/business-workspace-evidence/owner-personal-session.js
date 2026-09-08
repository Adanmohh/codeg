async page => {
  await page.getByRole('button').filter({hasText: 'Amal · Synthetic owner'}).click();
  const member = page.getByRole('dialog', {name: 'Member access', exact: true});
  await member.getByLabel('Access label', {exact: true}).fill('Synthetic owner browser4340');
  await member.getByRole('button', {name: 'Create personal access token', exact: true}).click();
  const secret = page.getByRole('dialog', {name: 'Share this access token privately.', exact: true});
  await secret.waitFor();
  if (await secret.locator('input').count()) throw Error('Credential must start masked without plaintext DOM input');
  await page.screenshot({path: 'reports/business-workspace-evidence/credential-masked.png'});
  await secret.getByRole('button', {name: 'Reveal token', exact: true}).click();
  let token = await secret.getByLabel('Personal access token', {exact: true}).inputValue();
  if (!token.startsWith('bdm_')) throw Error('Unexpected synthetic credential format');
  await secret.getByRole('button', {name: 'Hide token', exact: true}).click();
  await secret.getByRole('button', {name: 'I have saved it; close', exact: true}).click();
  await member.getByRole('button', {name: 'Close', exact: true}).click();
  await page.getByRole('button', {name: 'Disconnect', exact: true}).click();
  await page.getByLabel('Personal access token', {exact: true}).fill(token);
  await page.getByRole('button', {name: 'Connect', exact: true}).click();
  token = '';
  await page.getByRole('heading', {name: 'My work', exact: true}).waitFor();
  await page.getByRole('button', {name: 'Create task', exact: true}).first().waitFor();
  const legacyVisible = await page.getByRole('link', {name: 'Open engineering workspace', exact: true}).count();
  const privateStored = await page.evaluate(() => Object.values(localStorage).some(value => value.startsWith('bdm_')));
  const ambientPreserved = await page.evaluate(() => localStorage.getItem('codeg_token') === 'synthetic-ambient-old-operator');
  if (legacyVisible || privateStored || !ambientPreserved || await page.locator('input[type=password]').count()) throw Error('Personal owner boundary leaked legacy entry or stored credentials');
  await page.screenshot({path: 'reports/business-workspace-evidence/personal-owner-no-legacy.png'});
  return {synthetic: true, personalCredentialCreatedThroughUI: true, maskedByDefault: true, deliberateRevealThenHide: true, secretExcludedFromCaptures: true, personalOwnerConnected: true, legacyVisible, privateStored, ambientPreserved};
}
