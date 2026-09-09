async page => {
  if(await page.getByRole('dialog',{name:'Review access and setup',exact:true}).count())
    await page.getByRole('dialog',{name:'Review access and setup',exact:true}).getByRole('button',{name:'Close',exact:true}).click();
  if(await page.getByRole('dialog',{name:'Discard these changes?',exact:true}).count())
    await page.getByRole('dialog',{name:'Discard these changes?',exact:true}).getByRole('button').filter({hasText:/^Close$/}).click();
  await page.getByRole('region',{name:'Source connections',exact:true}).getByRole('combobox').selectOption({label:'Synthetic UI meeting notes'});
  const panel=page.getByRole('region',{name:'Import progress',exact:true});
  await panel.getByLabel('From (UTC)',{exact:true}).fill('2026-09-08T00:00');
  await panel.getByLabel('Until (UTC)',{exact:true}).fill('2026-09-09T00:00');
  const started=page.waitForResponse(r=>r.url().endsWith('/intake/imports/start')&&r.request().method()==='POST');
  await panel.getByRole('button',{name:'Start bounded import',exact:true}).click();
  const first=await started;
  if(first.status()!==200)throw Error('Bounded import start failed');
  let item=await first.json();
  const states=[{state:item.state,revision:item.revision,discovered:item.discovered,completed:item.completed}];
  for(let step=0;step<8 && item.capabilities.advance;step++){
    await panel.getByText(`Revision ${item.revision}`,{exact:true}).waitFor();
    const response=page.waitForResponse(r=>r.url().endsWith('/intake/imports/advance')&&r.request().method()==='POST');
    await panel.getByRole('button',{name:'Read next step',exact:true}).click();
    const saved=await response;
    if(saved.status()!==200)throw Error('Bounded import advance failed; no replacement read');
    item=await saved.json();
    states.push({state:item.state,revision:item.revision,discovered:item.discovered,completed:item.completed});
  }
  if(item.state!=='complete'||item.completed!==3)throw Error('Expected three bounded synthetic records');
  await page.getByRole('button',{name:'ui: meeting-follow-up',exact:false}).waitFor();
  await page.screenshot({path:'reports/business-intake-ui-evidence/unified-import-finished-1280-light.png'});
  return {importId:item.id,states,coverage:item.coverage,failed:item.failed,sourceRows:await page.getByRole('region',{name:'Sources',exact:true}).getByRole('button').count()};
}
