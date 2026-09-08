async page => {
  const calls = [];
  page.on('response', response => {
    if (response.url().includes('/api/business/')) calls.push({operation: response.url().split('/api/business/')[1], status: response.status()});
  });
  await page.getByRole('heading', {name: 'Start with your organization.'}).waitFor();
  await page.screenshot({path: 'reports/business-workspace-evidence/bootstrap-before.png'});
  await page.getByLabel('Organization name', {exact: true}).fill('Hafidh Studio · Synthetic');
  await page.getByLabel('Your name', {exact: true}).fill('Amal · Synthetic owner');
  await page.getByRole('button', {name: 'Create workspace', exact: true}).click();
  await page.getByRole('heading', {name: 'My work', exact: true}).waitFor();
  await page.getByRole('button', {name: 'Create task', exact: true}).first().waitFor();
  await page.screenshot({path: 'reports/business-workspace-evidence/work-empty-real-1280-light.png'});
  await page.getByRole('navigation', {name: 'Workspace', exact: true}).getByRole('button', {name: 'People & agents', exact: true}).click();
  const records = [
    {name: 'Samira · Synthetic manager', kind: 'human', role: 'manager'},
    {name: 'Nabil · Synthetic member', kind: 'human', role: 'member'},
    {name: 'Atlas · Synthetic agent', kind: 'agent', role: 'member'},
    {name: 'Lina · Synthetic viewer', kind: 'human', role: 'viewer'},
  ];
  for (const record of records) {
    await page.getByRole('button', {name: 'Add a person or agent', exact: true}).click();
    let dialog = page.getByRole('dialog');
    await dialog.getByLabel('Display name', {exact: true}).fill(record.name);
    await dialog.getByLabel('Member type', {exact: true}).selectOption(record.kind);
    if (record.kind === 'human') await dialog.getByLabel('Role', {exact: true}).selectOption(record.role);
    for (const domain of ['Marketing', 'Website', 'Feedback']) await dialog.getByLabel(domain, {exact: true}).check();
    await dialog.getByRole('button', {name: 'Save changes', exact: true}).click();
    dialog = page.getByRole('dialog', {name: 'Member access', exact: true});
    await dialog.waitFor();
    await dialog.getByRole('button', {name: 'Close', exact: true}).click();
    await page.getByRole('button').filter({hasText: record.name}).waitFor();
  }
  await page.screenshot({path: 'reports/business-workspace-evidence/people-real-1280-light.png'});
  if (calls.some(v => v.status !== 200) || calls.filter(v => v.operation === 'members/create').length !== records.length || calls.filter(v => v.operation === 'bootstrap').length !== 1) throw Error('Real bootstrap/member creation did not complete: ' + JSON.stringify(calls));
  return {synthetic: true, uiSource: 'c1a35955', backendSource: '1ba73e3c', createdOrganizationThroughUI: true, createdMembersThroughUI: records, calls, noEngineRequired: true};
}
