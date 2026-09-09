async page => {
  await page.getByRole('tab',{name:'My work',exact:true}).click();
  await page.getByRole('tab',{name:'Sources',exact:true}).click();
  await page.getByRole('region',{name:'Source connections',exact:true}).getByRole('combobox').selectOption({label:'Synthetic UI meeting notes'});
  await page.getByRole('button',{name:'ui: meeting-long',exact:false}).click();
  await page.getByRole('heading',{name:'ui: meeting-long',exact:true}).waitFor();
  if(!await page.getByRole('region',{name:'Exact source passage',exact:true}).count()) {
    const panel=page.getByRole('region',{name:'Import progress',exact:true});
    const start=page.waitForResponse(r=>r.url().endsWith('/intake/imports/start'));
    await panel.getByRole('button',{name:'Refresh source access',exact:true}).click();
    let r=await start;
    if(r.status()!==200)throw Error('Refresh start failed');
    let item=await r.json();
    for(let n=0;n<4 && item.capabilities.advance;n++){
      await panel.getByText(`Revision ${item.revision}`,{exact:true}).waitFor();
      const response=page.waitForResponse(r=>r.url().endsWith('/intake/imports/advance'));
      await panel.getByRole('button',{name:'Read next step',exact:true}).click();
      r=await response;
      if(r.status()!==200)throw Error('Refresh failed');
      item=await r.json();
    }
  }
  const passages=page.getByRole('region',{name:'Exact source passage',exact:true});
  await passages.getByRole('checkbox',{name:'Exact source passage 2',exact:true}).check();
  const created=page.waitForResponse(r=>r.url().endsWith('/intake/candidates/create'));
  await page.getByRole('button',{name:'Prepare private draft',exact:true}).click();
  const c=await created;
  if(c.status()!==200)throw Error('Candidate creation failed');
  const candidate=await c.json();
  const draft=page.getByRole('region',{name:'Private task draft',exact:true});
  await draft.getByRole('button',{name:'Link an existing task',exact:true}).click();
  const finder=draft.getByRole('region',{name:'Find an existing task',exact:true});
  await finder.getByLabel('Find an existing task',{exact:true}).fill('existing reviewed customer follow-up');
  const searched=page.waitForResponse(r=>r.url().endsWith('/tasks/list'));
  await finder.getByRole('button',{name:'Search',exact:true}).click();
  if((await searched).status()!==200)throw Error('Authorized task search failed');
  const targetRead=page.waitForResponse(r=>r.url().endsWith('/tasks/get'));
  await finder.getByRole('button',{name:'Review this task',exact:true}).click();
  const target=await (await targetRead).json();
  const exact=draft.getByRole('region',{name:'Exact target task',exact:true});
  const confirm=exact.getByRole('button',{name:'Confirm text-free link',exact:true});
  const disabledBeforeConfirmation=await confirm.isDisabled();
  if(!disabledBeforeConfirmation)throw Error('Missing explicit target confirmation');
  await page.screenshot({path:'reports/business-intake-ui-evidence/unified-text-free-target-1280-light.png'});
  await exact.getByRole('checkbox',{name:'I reviewed this exact task and destination',exact:false}).check();
  const linked=page.waitForResponse(r=>r.url().endsWith('/intake/candidates/link'));
  await confirm.click();
  const r=await linked;
  if(r.status()!==200)throw Error('Text-free link failed');
  const result=await r.json();
  await page.getByRole('region',{name:'Decision recorded',exact:true}).getByRole('heading',{name:'Linked to existing task',exact:true}).waitFor();
  const opened=page.waitForResponse(r=>r.url().endsWith('/tasks/get'));
  await page.getByRole('button',{name:'Open shared task',exact:true}).click();
  const after=await (await opened).json();
  if(after.task.notes!==target.task.notes || after.task.title!==target.task.title || after.task.revision!==target.task.revision+1 || after.execution!==null)throw Error('Link changed text or launched execution');
  return {candidateId:candidate.id,taskId:target.task.id,hadPreparedDraft:candidate.hasPreparedDraft,disabledBeforeConfirmation,status:r.status(),decision:result.decision.kind,beforeRevision:target.task.revision,afterRevision:after.task.revision,textUnchanged:true,execution:after.execution};
}
