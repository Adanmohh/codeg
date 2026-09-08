async page => {
  await page.getByRole('navigation', {name: 'Workspace', exact: true}).getByRole('button', {name: 'People & agents', exact: true}).click();
  await page.getByRole('button').filter({hasText: 'Nabil · Synthetic member'}).click();
  const dialog = page.getByRole('dialog', {name: 'Member access', exact: true});
  await dialog.getByLabel('Role', {exact: true}).selectOption('viewer');
  const pending = page.waitForResponse(response => response.url().endsWith('/api/business/members/update'));
  await dialog.getByRole('button', {name: 'Save changes', exact: true}).click();
  const response = await pending;
  if (response.status() !== 200) throw Error('Actual member downgrade failed');
  const member = await response.json();
  if (member.role !== 'viewer' || member.revision < 2) throw Error('Downgrade contract differs');
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  return {synthetic: true, role: member.role, memberId: member.id, revision: member.revision, updatedThroughUI: true};
}
