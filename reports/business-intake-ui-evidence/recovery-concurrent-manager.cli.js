async page => {
  await page.getByRole('button',{name:'Sources',exact:true}).click();
  await page.getByRole('region',{name:'Source connections',exact:true}).getByRole('combobox').selectOption({label:'Synthetic UI meeting notes'});
  await page.getByRole('button',{name:'ui: meeting-follow-up',exact:false}).click();
  const opened=page.waitForResponse(r=>r.url().endsWith('/intake/candidates/get'));
  await page.getByRole('region',{name:'Private drafts & decisions',exact:true}).getByRole('button',{name:'Synthetic recovery: customer response draft Pending · Revision 2',exact:true}).click();
  const original=await (await opened).json();
  const editor=page.getByRole('region',{name:'Private task draft',exact:true});
  await editor.getByLabel('Task title',{exact:true}).fill('Synthetic current saved version from manager');
  await editor.getByLabel('Brief',{exact:true}).fill('Synthetic manager correction for the explicit adoption comparison. No task publication.');
  const saved=page.waitForResponse(r=>r.url().endsWith('/intake/candidates/edit'));
  await editor.getByRole('button',{name:'Save private draft',exact:true}).click();
  const response=await saved;
  if(response.status()!==200)throw Error('Manager private save failed');
  const result=await response.json();
  if(result.candidate.revision!==original.candidate.revision+1)throw Error('Unexpected candidate CAS revision');
  return {candidateId:original.candidate.id,beforeRevision:original.candidate.revision,afterRevision:result.candidate.revision,status:response.status(),published:false};
}
