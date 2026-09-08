async page => {
  await page.getByRole('button').filter({has:page.getByRole('heading',{name:'Prepare the feedback summary · Synthetic',exact:true})}).click();
  const dialog=page.getByRole('dialog',{name:'Prepare the feedback summary · Synthetic',exact:true});
  let pending=page.waitForResponse(r=>r.url().endsWith('/api/business/tasks/progress'));
  await dialog.getByRole('button',{name:'Start work',exact:true}).click();
  let response=await pending;
  if(response.status()!==200)throw Error('Human progress failed');
  await dialog.locator('summary').filter({hasText:'Submit for review'}).click();
  await dialog.getByLabel('Deliverable',{exact:true}).fill('Synthetic feedback summary: customers need a clear first step and a named person for questions. Recommend reviewing the welcome copy. No customer message, provider call or agent launch occurred.');
  pending=page.waitForResponse(r=>r.url().endsWith('/api/business/tasks/submit'));
  await dialog.locator('details').filter({has:page.getByLabel('Deliverable',{exact:true})}).getByRole('button',{name:'Submit for review',exact:true}).click();
  response=await pending;
  if(response.status()!==200)throw Error('Human submit failed');
  const detail=await response.json();
  if(detail.task.status!=='review'||detail.task.reviewerId===detail.task.assigneeId||await dialog.getByRole('button',{name:'Accept work',exact:true}).count())throw Error('Designated separate review is not enforced');
  await dialog.getByRole('button',{name:'Close',exact:true}).click();
  return {synthetic:true,taskId:detail.task.id,revision:detail.task.revision,status:detail.task.status,assigneeId:detail.task.assigneeId,reviewerId:detail.task.reviewerId,currentDeliverableId:detail.task.currentDeliverableId,noSelfAcceptControl:true};
}
