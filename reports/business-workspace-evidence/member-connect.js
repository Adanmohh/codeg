async page => {
  const calls = [], blocked = [];
  await page.route('**/*', route => {
    const url = route.request().url();
    if (url.startsWith('http://127.0.0.1:4340/') || url === 'about:blank') return route.continue();
    blocked.push(url.split('?')[0]);
    return route.abort();
  });
  page.on('response', response => {
    if (response.url().includes('/api/')) calls.push({path: response.url().split('/api/')[1].split('?')[0], status: response.status()});
  });
  await page.addInitScript(() => {
    localStorage.setItem('codeg_token', 'synthetic-ambient-old-operator');
    localStorage.setItem('codeg.system_language_settings', JSON.stringify({mode: 'manual', language: 'en'}));
    localStorage.setItem('theme', 'light');
  });
  await page.goto('http://127.0.0.1:4340/business.html');
  await page.setViewportSize({width: 1280, height: 900});
  await page.getByRole('heading', {name: 'Give shared work a clear next step.'}).waitFor();
  // Test-only credential setup against the real guard. No response replacement,
  // raw token output, browser storage or legacy token installation.
  const issued = await page.evaluate(async () => {
    const post = async (path, input) => {
      const response = await fetch('/api/business/' + path, {method: 'POST', headers: {'Content-Type': 'application/json', Authorization: 'Bearer business-tasks-synthetic-operator'}, body: JSON.stringify({input}), credentials: 'omit', redirect: 'error'});
      if (!response.ok) throw Error('Synthetic identity setup failed: ' + response.status);
      return response.json();
    };
    const context = await post('context', {});
    const people = await post('members/list', {organizationId: context.organization.id});
    const member = people.find(value => value.displayName === 'Nabil · Synthetic member');
    if (!member) throw Error('Named synthetic member was not created through owner UI');
    const issue = await post('credentials/issue', {organizationId: context.organization.id, memberId: member.id, label: 'Synthetic member browser4340'});
    return {token: issue.token, memberId: member.id, organizationId: member.organizationId};
  });
  await page.getByLabel('Personal access token', {exact: true}).fill(issued.token);
  await page.getByRole('button', {name: 'Connect', exact: true}).click();
  issued.token = '';
  await page.getByRole('heading', {name: 'My work', exact: true}).waitFor();
  await page.getByRole('button', {name: 'Create task', exact: true}).first().waitFor();
  const legacyVisible = await page.getByRole('link', {name: 'Open engineering workspace', exact: true}).count();
  const privateStored = await page.evaluate(() => Object.values(localStorage).some(value => value.startsWith('bdm_')));
  if (legacyVisible || privateStored || blocked.length || calls.some(v => !v.path.startsWith('business/') || v.status !== 200)) throw Error('Member connection boundary failed');
  await page.screenshot({path: 'reports/business-workspace-evidence/member-connected-real.png'});
  return {synthetic: true, credentialSetup: 'real identity API; separate from UI-issued owner credential check', memberId: issued.memberId, organizationId: issued.organizationId, legacyVisible, privateStored, calls, blocked};
}
