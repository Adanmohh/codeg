async page => {
  const fs = page.constructor.constructor('return process')().getBuiltinModule('node:fs');
  const credentials = JSON.parse(fs.readFileSync('/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-intake-fixtures/unified-YQmRz8/ui-credentials.json','utf8'));
  if(!await page.getByRole('dialog',{name:'Set up a source',exact:true}).count())
    await page.getByRole('button',{name:'Set up a source',exact:true}).click();
  const dialog = page.getByRole('dialog',{name:'Set up a source',exact:true});
  const kinds = await dialog.getByLabel('Source type',{exact:true}).locator('option').allTextContents();
  if(kinds.length!==1 || kinds[0]!=='Fireflies meetings')throw Error('Unexpected tenant setup kind');
  await dialog.getByLabel('Connection name',{exact:true}).fill('Synthetic UI meeting notes');
  await dialog.getByLabel('Area of work',{exact:true}).selectOption('feedback');
  await dialog.getByLabel('Source account owner',{exact:true}).selectOption(credentials.sessions.owner.memberId);
  await dialog.getByRole('checkbox',{name:'Feedback',exact:true}).check();
  await dialog.getByRole('checkbox',{name:'I allow explicitly accepted task text to remain',exact:false}).check();
  await page.screenshot({path:'reports/business-intake-ui-evidence/unified-setup-before-secret-1280-light.png'});
  let created;
  try {
    await dialog.getByLabel('Fireflies API key',{exact:true}).fill(credentials.firefliesApiKey);
    const response = page.waitForResponse(r=>r.url().endsWith('/intake/bindings/create') && r.request().method()==='POST');
    await dialog.getByRole('button',{name:'Create disabled connection',exact:true}).click();
    const saved = await response;
    if(saved.status()!==200)throw Error('Create failed');
    created = await saved.json();
    await page.getByRole('dialog',{name:'Review access and setup',exact:true}).waitFor();
  } catch {
    throw Error('Synthetic setup did not complete; inspect only safe status metadata');
  }
  const review = page.getByRole('dialog',{name:'Review access and setup',exact:true});
  await review.getByText('Nobody has an explicit grant yet.',{exact:true}).waitFor();
  const replacementEmpty = (await review.getByLabel('Replace stored Fireflies key (optional)',{exact:true}).inputValue())==='';
  if(created.enabled!==false || !replacementEmpty)throw Error('Disabled/secret clearing contract failed');
  await page.screenshot({path:'reports/business-intake-ui-evidence/unified-disabled-zero-grants-1280-light.png'});
  const grants = review.getByRole('region',{name:'People with source access',exact:true});
  const statuses=[];
  for(const role of ['owner','manager']) {
    await grants.getByLabel('Person receiving access',{exact:true}).selectOption(credentials.sessions[role].memberId);
    await grants.getByRole('checkbox',{name:'Read retained and newly imported source records',exact:true}).check();
    await grants.getByRole('checkbox',{name:'Import and refresh source records',exact:true}).check();
    await grants.getByRole('checkbox',{name:'Prepare and decide on source drafts',exact:true}).check();
    await grants.getByRole('checkbox',{name:'Feedback',exact:true}).check();
    await grants.getByRole('checkbox',{name:'I am granting this person access through this exact connection',exact:false}).check();
    const response = page.waitForResponse(r=>r.url().endsWith('/intake/grants/upsert') && r.request().method()==='POST');
    await grants.getByRole('button',{name:'Save explicit access',exact:true}).click();
    const saved = await response;
    statuses.push(saved.status());
    if(saved.status()!==200)throw Error('Explicit grant failed');
    await review.getByRole('button',{name:'Enable connection',exact:true}).waitFor();
    await page.waitForFunction(()=>[...document.querySelectorAll('button')].some(b=>b.textContent==='Enable connection'&&!b.disabled));
  }
  const enabledResponse = page.waitForResponse(r=>r.url().endsWith('/intake/bindings/update')&&r.request().method()==='POST');
  await review.getByRole('button',{name:'Enable connection',exact:true}).click();
  const enabled = await enabledResponse;
  if(enabled.status()!==200)throw Error('Enable failed');
  await review.getByRole('button',{name:'Disable connection',exact:true}).waitFor();
  await page.screenshot({path:'reports/business-intake-ui-evidence/unified-enabled-grants-1280-light.png'});
  return {bindingId:created.id,createdDisabled:!created.enabled,zeroInitialGrants:true,replacementEmpty,kinds,grantStatuses:statuses,enableStatus:enabled.status()};
}
