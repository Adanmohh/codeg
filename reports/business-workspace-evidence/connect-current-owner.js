async page => {
  await page.reload();
  await page.getByLabel('Personal access token', {exact: true}).waitFor();
  if (await page.getByLabel('Personal access token', {exact: true}).inputValue() !== '') throw Error('Reload retained credential');
  const issued = await page.evaluate(async () => {
    const post = async (path, input) => {
      const response = await fetch('/api/business/' + path, {method: 'POST', headers: {'Content-Type': 'application/json', Authorization: 'Bearer business-tasks-synthetic-operator'}, body: JSON.stringify({input}), credentials: 'omit', redirect: 'error'});
      if (!response.ok) throw Error('Synthetic identity setup failed: ' + response.status);
      return response.json();
    };
    const context = await post('context', {});
    const people = await post('members/list', {organizationId: context.organization.id});
    const owner = people.find(value => value.displayName === 'Amal · Synthetic owner');
    const issue = await post('credentials/issue', {organizationId: context.organization.id, memberId: owner.id, label: 'Synthetic owner current export4340'});
    return {token: issue.token};
  });
  await page.getByLabel('Personal access token', {exact: true}).fill(issued.token);
  await page.getByRole('button', {name: 'Connect', exact: true}).click();
  issued.token = '';
  await page.getByRole('heading', {name: 'My work', exact: true}).waitFor();
  await page.getByRole('button').filter({has: page.getByRole('heading', {name: 'Shape the customer welcome · Synthetic', exact: true})}).getByText('Done', {exact: true}).waitFor();
  return {synthetic: true, reloadClearedCredential: true, realDoneVisibleInIndependentOwnerSession: true, currentExportLoaded: true};
}
