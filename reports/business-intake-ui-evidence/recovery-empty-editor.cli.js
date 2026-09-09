// Actual protected UI, synthetic ui namespace only; no mocked responses.
// Reuses unified-refresh/accept scripts at08417b4ff (Codeg Apache-2.0).
async page => {
  await page.getByRole('button',{name:'Sources',exact:true}).click();
  await page.getByRole('region',{name:'Source connections',exact:true}).getByRole('combobox').selectOption({label:'Synthetic UI meeting notes'});
  await page.getByRole('button',{name:'ui: meeting-follow-up',exact:false}).click();
  await page.getByRole('heading',{name:'Source content is currently withheld.',exact:true}).waitFor();
  const opened=page.waitForResponse(r=>r.url().endsWith('/intake/candidates/get'));
  await page.getByRole('region',{name:'Private drafts & decisions',exact:true}).getByRole('button',{name:/^Pending 1 /}).click();
  const original=await (await opened).json();
  if(original.candidate.hasPreparedDraft || original.candidate.disclosure!=='metadata_only')throw Error('Expected expired unprepared candidate');
  if(await page.getByLabel('Task title',{exact:true}).count())throw Error('Expired fields disclosed');
  await page.screenshot({path:'reports/business-intake-ui-evidence/recovery-expired-before-1280-light.png'});
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
    item=await response.json();states.push(item.state);
  }
  if(item.state!=='complete')throw Error('Refresh incomplete');
  const editor=page.getByRole('region',{name:'Private task draft',exact:true});
  const title=editor.getByLabel('Task title',{exact:true});
  await title.waitFor();
  if(await title.inputValue()!=='' || await editor.getByLabel('Brief',{exact:true}).inputValue()!=='' || await page.getByRole('region',{name:'Current saved candidate',exact:true}).count())throw Error('Same-version editor not restored empty');
  const save=editor.getByRole('button',{name:'Save private draft',exact:true});
  if(!await save.isDisabled())throw Error('Empty task can be saved');
  await page.screenshot({path:'reports/business-intake-ui-evidence/recovery-empty-after-1280-light.png'});
  await title.fill('Synthetic recovery: customer response draft');
  await editor.getByLabel('Brief',{exact:true}).fill('Synthetic saved baseline for explicit recovery. No source text is published.');
  await editor.getByLabel('Due date',{exact:true}).fill('2026-11-03');
  const saved=page.waitForResponse(r=>r.url().endsWith('/intake/candidates/edit'));
  await save.click();
  const result=await saved;
  if(result.status()!==200)throw Error('Restored selection could not save');
  const candidate=await result.json();
  await editor.getByRole('region',{name:'Review before sharing',exact:true}).waitFor();
  const confirm=editor.getByRole('checkbox',{name:'I have reviewed these exact passages',exact:false});
  if(await confirm.isChecked() || !await editor.getByRole('button',{name:'Accept into shared work',exact:true}).isDisabled())throw Error('Restoration confirmed publication');
  return {candidateId:original.candidate.id,beforeRevision:original.candidate.revision,beforeDisclosure:original.candidate.disclosure,beforePrepared:original.candidate.hasPreparedDraft,states,emptyFieldsRestored:true,emptySaveDisabled:true,savedRevision:candidate.candidate.revision,saveStatus:result.status(),confirmationReset:true,taskPublicationAttempted:false};
}
