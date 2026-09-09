async page => {
  const draft=page.getByRole('region',{name:'Private task draft',exact:true});
  await draft.getByRole('button',{name:'Link an existing task',exact:true}).click();
  const finder=draft.getByRole('region',{name:'Find an existing task',exact:true});
  await finder.getByLabel('Find an existing task',{exact:true}).fill('existing reviewed customer follow-up');
  const searched=page.waitForResponse(r=>r.url().endsWith('/tasks/list'));
  await finder.getByRole('button',{name:'Find a task',exact:true}).click();
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
  return {candidateId:result.decision.candidateId,taskId:target.task.id,disabledBeforeConfirmation,status:r.status(),decision:result.decision.kind,beforeRevision:target.task.revision,afterRevision:after.task.revision,textUnchanged:true,execution:after.execution};
}
