async page => {
  await page.getByRole('button').filter({hasText: 'Nabil · Synthetic member'}).click();
  const dialog = page.getByRole('dialog', {name: 'Member access', exact: true});
  await dialog.getByRole('button', {name: 'Revoke member access', exact: true}).click();
  const confirmation = page.getByRole('dialog', {name: 'Revoke this access?', exact: true});
  await confirmation.waitFor();
  const pending = page.waitForResponse(response => response.url().endsWith('/api/business/members/revoke'));
  await confirmation.getByRole('button', {name: 'Revoke access', exact: true}).click();
  const response = await pending;
  if (response.status() !== 200) throw Error('Actual member revoke failed: ' + response.status());
  const member = await response.json();
  if (member.status !== 'revoked') throw Error('Member remains active');
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  return {synthetic: true, status: member.status, memberId: member.id, revision: member.revision, confirmedThroughUI: true};
}
