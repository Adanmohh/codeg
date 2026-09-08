// Explicitly save this worker's synthetic task draft before reloading its export.
async page => {
  const meta=await page.evaluate(async()=>(await fetch('/__business_intake_fixture')).json());
  if(!meta.synthetic||meta.workspaceBackendPort!==4353)throw Error('Wrong fixture');
  const expected='Synthetic pane draft for locale, focus and responsive preservation.';
  if(await page.getByRole('textbox',{name:'Brief',exact:true}).inputValue()!==expected)throw Error('Wrong owned draft');
  const saved=page.waitForResponse(r=>r.url().endsWith('/api/business/tasks/update')&&r.request().method()==='POST');
  await page.getByRole('button',{name:'Save changes',exact:true}).click();
  const response=await saved;
  if(response.status()!==200)throw Error('Save status '+response.status());
  const result=await response.json();
  if(result.task.notes!==expected||result.execution!==null)throw Error('Unexpected task result');
  return {status:response.status(),taskId:result.task.id,revision:result.task.revision,exactDraftSaved:true,execution:result.execution};
}
