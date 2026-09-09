// Resume the existing queued refresh; do not start another operation.
async page => {
  const panel=page.getByRole('region',{name:'Import progress',exact:true});
  if(!await panel.isVisible())await page.locator('summary').filter({hasText:'Refresh source access'}).click();
  const advanced=page.waitForResponse(r=>r.url().endsWith('/intake/imports/advance'));
  await panel.getByRole('button',{name:'Read next step',exact:true}).click();
  const r=await advanced;
  const item=await r.json();
  if(r.status()!==200 || item.state!=='complete')throw Error('Existing refresh incomplete');
  const editor=page.getByRole('region',{name:'Private task draft',exact:true});
  await editor.getByLabel('Task title',{exact:true}).waitFor();
  if(await editor.getByLabel('Task title',{exact:true}).inputValue()!=='Synthetic local title retained through source refresh' || await editor.getByLabel('Brief',{exact:true}).inputValue()!=='Synthetic private draft survives successful fresh access, locale and panes. Not submitted.')throw Error('Fresh read replaced local draft');
  if(await page.getByRole('region',{name:'Current saved candidate',exact:true}).count())throw Error('Same version incorrectly requires adoption');
  await page.screenshot({path:'reports/business-intake-ui-evidence/recovery-retained-draft-1280-light.png'});
  return {sourceRevalidated:true,state:item.state,status:r.status(),localTitleRetained:true,localBriefRetained:true,currentComparison:false,privateSaveAttempted:false,resumedExistingAttempt:true};
}
