async page => {
  const dialog = page.getByRole('dialog', {name: 'Shape the customer welcome · Synthetic', exact: true});
  if (!(await dialog.getByRole('button', {name: 'Accept work', exact: true}).isDisabled())) throw Error('Human confirmation missing');
  await dialog.getByLabel('I have reviewed this task and its current deliverable.', {exact: true}).check();
  await page.screenshot({path: 'reports/business-workspace-evidence/independent-review-real.png'});
  const pending = page.waitForResponse(response => response.url().endsWith('/api/business/tasks/review'));
  await dialog.getByRole('button', {name: 'Accept work', exact: true}).click();
  const response = await pending;
  if (response.status() !== 200) throw Error('Human review failed: ' + response.status());
  const detail = await response.json();
  if (detail.task.status !== 'done' || detail.task.reviewerId !== '4c92ce61-9954-4a2a-8b03-c440ef7dd3aa' || !detail.task.currentDeliverableId) throw Error('Separate reviewer result differs');
  await dialog.getByText('Done', {exact: true}).first().waitFor();
  await page.waitForFunction(() => { const node = document.querySelector('[role=dialog]'); return node && getComputedStyle(node).opacity === '1' && getComputedStyle(node).transform === 'none'; });
  await page.screenshot({path: 'reports/business-workspace-evidence/review-done-real.png'});
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  return {synthetic: true, selectorCorrectionOnly: 'I have reviewed matches actual copy', reviewerId: detail.task.reviewerId, status: detail.task.status, revision: detail.task.revision, taskId: detail.task.id, humanConfirmationRequired: true, separateReviewer: true, noOutboundOrEngine: true};
}
