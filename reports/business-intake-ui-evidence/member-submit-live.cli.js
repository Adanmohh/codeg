// Second personal session; actual human submission, no engine/provider route.
async page => {
  const meta = await page.evaluate(async () => (await fetch('/__business_intake_fixture')).json());
  if (!meta.synthetic || meta.workspaceBackendPort !== 4353) throw Error('Wrong fixture');
  await page.getByRole('button', {name: /Shape the website launch brief/}).click();
  await page.locator('summary').filter({hasText: /^Submit for review$/}).click();
  await page.getByRole('textbox', {name: 'Deliverable', exact: true}).fill('Synthetic human result: audience and evidence are ready for review. No agent execution.');
  const submitted = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/submit') && r.request().method() === 'POST');
  await page.getByRole('button', {name: 'Submit for review', exact: true}).last().click();
  const response = await submitted;
  if (response.status() !== 200) throw Error('Submit status ' + response.status());
  const detail = await response.json();
  if (detail.task.status !== 'review' || detail.execution !== null) throw Error('Unexpected review/execution state');
  return {status: response.status(), taskId: detail.task.id, revision: detail.task.revision, taskStatus: detail.task.status, execution: detail.execution};
}
