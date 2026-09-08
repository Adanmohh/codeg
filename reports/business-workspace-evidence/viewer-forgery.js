async page => {
  return await page.evaluate(async () => {
    const operatorPost = async (path, input) => {
      const response = await fetch('/api/business/' + path, {method: 'POST', headers: {'Content-Type': 'application/json', Authorization: 'Bearer business-tasks-synthetic-operator'}, body: JSON.stringify({input}), credentials: 'omit', redirect: 'error'});
      if (!response.ok) throw Error('Synthetic identity setup failed');
      return response.json();
    };
    const context = await operatorPost('context', {});
    const people = await operatorPost('members/list', {organizationId: context.organization.id});
    const member = people.find(value => value.displayName === 'Nabil · Synthetic member');
    if (member.role !== 'viewer') throw Error('Viewer downgrade is not live');
    const issued = await operatorPost('credentials/issue', {organizationId: context.organization.id, memberId: member.id, label: 'Synthetic denied-request proof4340'});
    const checked = [];
    for (const [path,input] of [
      ['tasks/create',{title:'FORBIDDEN SYNTHETIC TASK',domain:'feedback'}],
      ['members/create',{organizationId:context.organization.id,displayName:'FORBIDDEN SYNTHETIC IDENTITY',kind:'human',role:'owner',domains:['engineering']}],
      ['bootstrap',{organizationName:'FORBIDDEN SYNTHETIC ORG',ownerName:'Impersonated'}],
    ]) {
      const response = await fetch('/api/business/'+path, {method:'POST',headers:{'Content-Type':'application/json',Authorization:'Bearer '+issued.token},body:JSON.stringify({input}),credentials:'omit',redirect:'error'});
      checked.push({path,status:response.status});
      if (response.status !== 403) throw Error('Real viewer forgery was not denied: '+path+' '+response.status);
    }
    issued.token = '';
    return {synthetic:true,actualViewerCredential:true,checked,noResponseBodiesCaptured:true};
  });
}
