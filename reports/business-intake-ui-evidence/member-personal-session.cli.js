async page => {
  await page.reload();
  await page.getByRole('combobox',{name:/^(Language|اللغة)$/}).selectOption('en');
  const issued=await page.evaluate(async()=>{
    const meta=await(await fetch('/__business_intake_fixture')).json();if(!meta.synthetic||meta.workspaceBackendPort!==4353)throw Error('Wrong synthetic fixture');
    const call=async(path,input)=>{const r=await fetch('/api/business/'+path,{method:'POST',headers:{'Content-Type':'application/json',Authorization:'Bearer business-tasks-synthetic-operator'},body:JSON.stringify({input}),credentials:'omit',cache:'no-store'});if(!r.ok)throw Error(path+' status '+r.status);return await r.json()};
    const c=await call('context',{});const members=await call('members/list',{organizationId:c.organization.id});const member=members.find(m=>m.displayName==='Yusuf (Synthetic)');if(!member)throw Error('Missing synthetic member');
    const credential=await call('credentials/issue',{organizationId:c.organization.id,memberId:member.id,label:'Synthetic UI member4350'});return {token:credential.token};
  });
  await page.getByLabel('Personal access token',{exact:true}).fill(issued.token);
  await page.getByRole('button',{name:'Connect',exact:true}).click();
  await page.getByRole('heading',{name:'My work',exact:true}).waitFor();
  return {personalMember:true,legacyLinkCount:await page.getByRole('link',{name:'Open engineering workspace',exact:true}).count(),tokenInputs:await page.locator('input[type=password]').count(),overflow:await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth)};
}
