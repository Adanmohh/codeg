async page => {
  await page.getByRole('button', {name: 'Disconnect', exact: true}).click();
  await page.getByLabel('Personal access token', {exact: true}).waitFor();
  const issued = await page.evaluate(async () => {
    const post = async (path, input) => {
      const response = await fetch('/api/business/' + path, {method: 'POST', headers: {'Content-Type': 'application/json', Authorization: 'Bearer business-tasks-synthetic-operator'}, body: JSON.stringify({input}), credentials: 'omit', redirect: 'error'});
      if (!response.ok) throw Error('Synthetic identity setup failed: ' + response.status);
      return response.json();
    };
    const context = await post('context', {});
    const people = await post('members/list', {organizationId: context.organization.id});
    const member = people.find(value => value.displayName === 'Samira · Synthetic manager');
    const issue = await post('credentials/issue', {organizationId: context.organization.id, memberId: member.id, label: 'Synthetic independent reviewer browser4340'});
    return {token: issue.token, memberId: member.id};
  });
  await page.getByLabel('Personal access token', {exact: true}).fill(issued.token);
  await page.getByRole('button', {name: 'Connect', exact: true}).click();
  issued.token = '';
  await page.getByRole('heading', {name: 'My work', exact: true}).waitFor();
  await page.getByRole('navigation', {name: 'Workspace', exact: true}).getByRole('button', {name: 'Review', exact: true}).click();
  await page.getByRole('button').filter({has: page.getByRole('heading', {name: 'Shape the customer welcome · Synthetic', exact: true})}).click();
  const dialog = page.getByRole('dialog', {name: 'Shape the customer welcome · Synthetic', exact: true});
  await dialog.getByText('Synthetic deliverable: Welcome to Hafidh. Start with the one task your team needs next. No message was sent.', {exact: true}).first().waitFor();
  if (!(await dialog.getByRole('button', {name: 'Accept work', exact: true}).isDisabled())) throw Error('Human confirmation missing');
  await dialog.getByLabel('Review note', {exact: true}).fill('Synthetic independent decision: the welcome is clear and ready for the next human action. No outbound message is authorized.');
  await dialog.getByLabel('I have reviewed this task and its current deliverable.', {exact: true}).check();
  await page.screenshot({path: 'reports/business-workspace-evidence/independent-review-real.png'});
  const pending = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/review'));
  await dialog.getByRole('button', {name: 'Accept work', exact: true}).click();
  const response = await pending;
  if (response.status() !== 200) throw Error('Human review failed: ' + response.status());
  const detail = await response.json();
  if (detail.task.status !== 'done' || detail.task.reviewerId !== issued.memberId || !detail.task.currentDeliverableId) throw Error('Separate reviewer result differs');
  await dialog.getByText('Done', {exact: true}).first().waitFor();
  await page.screenshot({path: 'reports/business-workspace-evidence/review-done-real.png'});
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  return {synthetic: true, reviewerId: issued.memberId, status: detail.task.status, revision: detail.task.revision, taskId: detail.task.id, humanConfirmationRequired: true, separateReviewer: true, noOutboundOrEngine: true};
}
