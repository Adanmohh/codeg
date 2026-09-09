async page => {
  const editor=page.getByRole('region',{name:'Private task draft',exact:true});
  await editor.getByRole('button',{name:'Edit task',exact:true}).click();
  const title='Synthetic local title retained through source refresh';
  const brief='Synthetic private draft survives successful fresh access, locale and panes. Not submitted.';
  await editor.getByLabel('Task title',{exact:true}).fill(title);
  await editor.getByLabel('Brief',{exact:true}).fill(brief);
  await page.locator('summary').filter({hasText:'Refresh source access'}).click();
  const panel=page.getByRole('region',{name:'Import progress',exact:true});
  const start=page.waitForResponse(r=>r.url().endsWith('/intake/imports/start'));
  await panel.getByRole('button',{name:'Refresh source access',exact:true}).click();
  let response=await start;
  if(response.status()!==200)throw Error('Refresh start failed');
  let item=await response.json();
  for(let n=0;n<4&&item.capabilities.advance;n++){
    await panel.getByText(`Revision ${item.revision}`,{exact:true}).waitFor();
    const next=page.waitForResponse(r=>r.url().endsWith('/intake/imports/advance'));
    await panel.getByRole('button',{name:'Read next step',exact:true}).click();
    response=await next;
    if(response.status()!==200)throw Error('Refresh step failed');
    item=await response.json();
  }
  if(item.state!=='complete')throw Error('Refresh incomplete');
  await editor.getByLabel('Task title',{exact:true}).waitFor();
  if(await editor.getByLabel('Task title',{exact:true}).inputValue()!==title || await editor.getByLabel('Brief',{exact:true}).inputValue()!==brief)throw Error('Fresh read replaced local draft');
  if(await page.getByRole('region',{name:'Current saved candidate',exact:true}).count())throw Error('Same version incorrectly requires adoption');
  await page.locator('summary').filter({hasText:'Refresh source access'}).click();
  await page.screenshot({path:'reports/business-intake-ui-evidence/recovery-retained-draft-1280-light.png'});
  return {sourceRevalidated:true,localTitleRetained:true,localBriefRetained:true,currentComparison:false,privateSaveAttempted:false};
}
