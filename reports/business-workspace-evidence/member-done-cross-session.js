async page => {
  await page.getByRole('button',{name:'Refresh',exact:true}).click();
  const task=page.getByRole('button').filter({has:page.getByRole('heading',{name:'Prepare the feedback summary · Synthetic',exact:true})});
  await task.getByText('Done',{exact:true}).waitFor();
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/member-done-cross-session.png'});
  return {synthetic:true,independentPersonalManagerSeesDone:true,noChatOrEngine:true};
}
