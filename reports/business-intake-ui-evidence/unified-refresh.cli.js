async page => {
  const withheldBefore=await page.getByText('Source content is currently withheld.',{exact:true}).count();
  const panel=page.getByRole('region',{name:'Import progress',exact:true});
  const start=page.waitForResponse(r=>r.url().endsWith('/intake/imports/start'));
  await panel.getByRole('button',{name:'Refresh source access',exact:true}).click();
  let response=await start;
  if(response.status()!==200)throw Error('Refresh start failed');
  let item=await response.json();
  const states=[item.state];
  for(let n=0;n<4&&item.capabilities.advance;n++){
    await panel.getByText(`Revision ${item.revision}`,{exact:true}).waitFor();
    const next=page.waitForResponse(r=>r.url().endsWith('/intake/imports/advance'));
    await panel.getByRole('button',{name:'Read next step',exact:true}).click();
    response=await next;
    if(response.status()!==200)throw Error('Refresh step failed');
    item=await response.json();
    states.push(item.state);
  }
  if(item.state!=='complete')throw Error('Refresh incomplete');
  await page.getByRole('region',{name:'Exact source passage',exact:true}).waitFor();
  await page.waitForTimeout(500);
  return {withheldBefore,states,taskTitleInputs:await page.getByLabel('Task title',{exact:true}).count(),editButtons:await page.getByRole('button',{name:'Edit task',exact:true}).count(),linkButtons:await page.getByRole('button',{name:'Link an existing task',exact:true}).count(),currentComparison:await page.getByRole('region',{name:'Current saved candidate',exact:true}).count(),withheldAfter:await page.getByText('Source content is currently withheld.',{exact:true}).count()};
}
