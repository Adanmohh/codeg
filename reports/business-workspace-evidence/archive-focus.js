async page => {
  await page.setViewportSize({width:1280,height:900});
  await page.getByRole('navigation',{name:'Workspace',exact:true}).getByRole('button',{name:'Shared work',exact:true}).click();
  const title=await page.getByRole('heading',{name:/^Synthetic long title /}).textContent();
  const row=()=>page.getByRole('button').filter({has:page.getByRole('heading',{name:title,exact:true})});
  const open=async()=>{await row().click();const d=page.getByRole('dialog',{name:title,exact:true});await d.getByRole('button',{name:'Close',exact:true}).waitFor();return d;};
  const focus=async()=>{await page.waitForFunction(()=>document.activeElement.tagName==='BUTTON'||document.activeElement.id==='business-main',undefined,{timeout:2000});return page.evaluate(()=>({tag:document.activeElement.tagName,id:document.activeElement.id,focusVisible:document.activeElement.matches(':focus-visible')}));};
  let dialog=await open();
  await dialog.getByRole('button',{name:'Close',exact:true}).click();
  const ordinaryClose=await focus();
  dialog=await open();
  const outcomes=[];
  async function confirmAction(label,operation,hint) {
    await dialog.getByRole('button',{name:label,exact:true}).click();
    const confirm=page.getByRole('dialog',{name:label,exact:true});
    await confirm.getByText(hint,{exact:true}).waitFor();
    const pending=page.waitForResponse(r=>r.url().endsWith('/api/business/tasks/'+operation));
    await confirm.getByRole('button',{name:label,exact:true}).click();
    const response=await pending;if(response.status()!==200)throw Error(label+' failed: '+response.status());
    const result=await response.json();
    await confirm.waitFor({state:'hidden'});
    outcomes.push({operation,revision:result.task.revision,status:result.task.status,archived:result.task.archivedAt!==null});
    return result.task;
  }
  const cancelled=await confirmAction('Cancel task','cancel','The task is marked Cancelled. Its history is retained.');
  if(cancelled.status!=='cancelled')throw Error('Cancellation did not apply');
  await dialog.getByText('Cancelled',{exact:true}).first().waitFor();
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/task-cancelled.png'});
  const archived=await confirmAction('Archive task','archive','The task leaves active lists. Its history is retained.');
  if(!archived.archivedAt)throw Error('Archive did not apply');
  await dialog.getByRole('button',{name:'Close',exact:true}).click();
  const afterArchive=await focus();
  await row().waitFor({state:'hidden'});
  await page.getByLabel('Archived',{exact:true}).check();
  await row().waitFor();dialog=await open();
  const restored=await confirmAction('Restore from archive','archive','The task returns to current lists. Its status and history are retained.');
  if(restored.archivedAt||restored.status!=='cancelled')throw Error('Restoration changed history/status');
  await dialog.getByRole('button',{name:'Close',exact:true}).click();
  const afterRestore=await focus();
  await page.getByLabel('Archived',{exact:true}).uncheck();
  await row().getByText('Cancelled',{exact:true}).waitFor();
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/workspace-final-focus.png'});
  return {synthetic:true,taskId:restored.id,ordinaryClose,afterArchive,afterRestore,outcomes,restoredToCurrentList:true};
}
