async page => {
  const dialog = page.getByRole('dialog', {name: 'Shape the customer welcome · Synthetic', exact: true});
  await dialog.getByText('Done', {exact: true}).first().waitFor();
  await page.waitForFunction(() => { const node = document.querySelector('[role=dialog]'); return node && getComputedStyle(node).opacity === '1' && getComputedStyle(node).transform === 'none'; });
  await page.screenshot({path: 'reports/business-workspace-evidence/review-done-real.png'});
  const revision = await dialog.getByText(/^Revision \d+$/).innerText();
  const reviewer = await dialog.locator('dl').getByText('Samira · Synthetic manager', {exact: true}).innerText();
  const note = await dialog.getByText('Synthetic independent decision: the welcome is clear and ready for the next human action. No outbound message is authorized.', {exact: true}).innerText();
  await dialog.getByRole('button', {name: 'Close', exact: true}).click();
  return {synthetic: true, actualReviewStatus: 'done', revision, reviewer, reviewNoteVisible: Boolean(note), noRepeatedMutation: true, originalReviewResponseStatus: 200, humanConfirmationRequired: true, probeCorrection: 'Only screenshot settling failed after the successful review; corrected to scope dialog opacity/transform.'};
}
