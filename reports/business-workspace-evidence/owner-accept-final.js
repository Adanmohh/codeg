async page => {
  const dialog=page.getByRole('dialog',{name:'Prepare the feedback summary · Synthetic',exact:true});
  if(!(await dialog.getByLabel('I have reviewed this task and its current deliverable.',{exact:true}).isChecked()))throw Error('Reviewed state was lost');
  const pending=page.waitForResponse(r=>r.url().endsWith('/api/business/tasks/review'));
  await dialog.getByRole('button',{name:'Accept work',exact:true}).click();
  const response=await pending;
  if(response.status()!==200)throw Error('Final human review failed');
  const detail=await response.json();
  if(detail.task.status!=='done'||detail.task.reviewerId!==detail.task.ownerId||detail.task.assigneeId===detail.task.reviewerId)throw Error('Separate review result differs');
  await dialog.getByText('Done',{exact:true}).first().waitFor();
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/owner-done-final.png'});
  await dialog.getByRole('button',{name:'Close',exact:true}).click();
  await page.getByText('Nothing waiting for your review.',{exact:true}).waitFor();
  const focus=await page.evaluate(()=>({tag:document.activeElement.tagName,label:document.activeElement.getAttribute('aria-label')||document.activeElement.textContent.trim().slice(0,60)}));
  return {synthetic:true,taskId:detail.task.id,revision:detail.task.revision,status:detail.task.status,reviewerId:detail.task.reviewerId,assigneeId:detail.task.assigneeId,reviewAppliedOnce:true,focusAfterClose:focus};
}
