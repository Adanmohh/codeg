async page => {
  await page.getByRole('navigation',{name:'Workspace',exact:true}).getByRole('button',{name:'Review',exact:true}).click();
  await page.getByRole('button').filter({has:page.getByRole('heading',{name:'Prepare the feedback summary · Synthetic',exact:true})}).click();
  const dialog=page.getByRole('dialog',{name:'Prepare the feedback summary · Synthetic',exact:true});
  await dialog.getByText(/^Synthetic feedback summary:/).first().waitFor();
  if(!(await dialog.getByRole('button',{name:'Accept work',exact:true}).isDisabled()))throw Error('Human confirmation missing');
  await dialog.getByLabel('Review note',{exact:true}).fill('Synthetic owner decision: the summary is clear; keep the recommendation as work for a later human decision.');
  await dialog.getByLabel('I have reviewed this task and its current deliverable.',{exact:true}).check();
  await page.waitForFunction(()=>{const el=document.querySelector('[role=dialog]');return el&&getComputedStyle(el).opacity==='1'&&getComputedStyle(el).transform==='none';});
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/review-ready-1280-light.png'});
  return {synthetic:true,exactCurrentDeliverableVisible:true,humanConfirmationRequired:true,acceptEnabled:await dialog.getByRole('button',{name:'Accept work',exact:true}).isEnabled(),reviewNotYetApplied:true};
}
