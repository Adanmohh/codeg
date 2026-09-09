async page => {
  const editor=page.getByRole('region',{name:'Private task draft',exact:true});
  const title=editor.getByLabel('Task title',{exact:true});
  const brief=editor.getByLabel('Brief',{exact:true});
  const expectedTitle='Synthetic local title retained through source refresh';
  const expectedBrief='Synthetic private draft survives successful fresh access, locale and panes. Not submitted.';
  if(await title.inputValue()!==expectedTitle || await brief.inputValue()!==expectedBrief)throw Error('Concurrent write replaced local draft before adoption');
  let conflictStatus=null;
  const current=editor.getByRole('region',{name:'Current saved candidate',exact:true});
  if(!await current.count()){
    const saved=page.waitForResponse(r=>r.url().endsWith('/intake/candidates/edit'));
    await editor.getByRole('button',{name:'Save private draft',exact:true}).click();
    conflictStatus=(await saved).status();
    if(conflictStatus!==409)throw Error('Expected stale candidate conflict');
    await editor.getByRole('button',{name:'Load current source and candidate',exact:true}).click();
  }
  await current.getByRole('heading',{name:'Synthetic current saved version from manager',exact:true}).waitFor();
  if(!(await current.innerText()).includes('Revision 3') || await title.inputValue()!==expectedTitle || await brief.inputValue()!==expectedBrief || !await editor.getByRole('button',{name:'Save private draft',exact:true}).isDisabled())throw Error('Current comparison did not preserve and lock draft');
  await current.getByRole('heading',{name:'Current saved candidate',exact:true}).scrollIntoViewIfNeeded();
  await page.screenshot({path:'reports/business-intake-ui-evidence/recovery-current-before-adopt-1280-light.png'});
  await page.setViewportSize({width:390,height:900});
  await current.getByRole('heading',{name:'Current saved candidate',exact:true}).scrollIntoViewIfNeeded();
  await page.screenshot({path:'reports/business-intake-ui-evidence/recovery-current-before-adopt-390-light.png'});
  await current.getByRole('button',{name:'Use the current saved draft',exact:true}).click();
  if(await title.inputValue()!=='Synthetic current saved version from manager' || await brief.inputValue()!=='Synthetic manager correction for the explicit adoption comparison. No task publication.' || await current.count())throw Error('Explicit saved adoption mismatch');
  await editor.getByRole('button',{name:'Review before sharing',exact:true}).click();
  const confirm=editor.getByRole('checkbox',{name:'I have reviewed these exact passages',exact:false});
  if(await confirm.isChecked() || !await editor.getByRole('button',{name:'Accept into shared work',exact:true}).isDisabled())throw Error('Adoption confirmed publication');
  await page.screenshot({path:'reports/business-intake-ui-evidence/recovery-adopted-unconfirmed-390-light.png'});
  await page.setViewportSize({width:1280,height:900});
  return {candidateId:'675fe092-1265-4201-9117-863e2dde2dde',draftBaseRevision:2,currentSavedRevision:3,conflictStatus,exactLocalDraftRetainedUntilAdopt:true,comparisonLockedSave:true,explicitSavedAdoption:true,confirmationReset:true,publicationAttempted:false};
}
