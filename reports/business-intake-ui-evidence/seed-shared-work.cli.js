async page => {
  const seeded = await page.evaluate(async () => {
    const meta = await (await fetch('/__business_intake_fixture')).json();
    if (!meta.synthetic || meta.workspaceBackendPort !== 4353) throw Error('Wrong synthetic fixture');
    const call = async (path, input) => {
      const response = await fetch('/api/business/' + path, { method:'POST', headers:{'Content-Type':'application/json', Authorization:'Bearer business-tasks-synthetic-operator'}, body:JSON.stringify({input}), credentials:'omit', cache:'no-store' });
      if (!response.ok) throw Error(path + ' status ' + response.status);
      return await response.json();
    };
    const context = await call('context', {});
    if (context.organization.name !== 'Synthetic Studio') throw Error('Unexpected workspace');
    const organizationId = context.organization.id;
    const manager = await call('members/create', { organizationId, displayName:'Rania (Synthetic)', kind:'human', role:'manager', domains:['marketing','website','feedback','channels','ads','engineering'] });
    const contributor = await call('members/create', { organizationId, displayName:'Yusuf (Synthetic)', kind:'human', role:'member', domains:['marketing','website','feedback','channels','ads','engineering'] });
    const agent = await call('members/create', { organizationId, displayName:'Draft assistant (Synthetic)', kind:'agent', role:'member', domains:['marketing','website','feedback','channels','ads','engineering'] });
    const samples = [
      ['Synthetic · Shape the website launch brief', 'website', 'high', contributor.id, '2026-09-12'],
      ['Synthetic · Review customer interview themes', 'feedback', 'normal', agent.id, '2026-09-14'],
      ['Synthetic · Prepare next week’s channel plan', 'channels', 'normal', contributor.id, '2026-09-16'],
      ['Synthetic · Compare two campaign directions', 'marketing', 'high', manager.id, null],
      ['Synthetic · Check the signup error report', 'engineering', 'urgent', contributor.id, '2026-09-10'],
      ['Synthetic · Clarify the ad creative brief', 'ads', 'low', agent.id, '2026-09-18'],
    ];
    const tasks=[];
    for (const [title,domain,priority,assigneeId,dueDate] of samples) {
      let detail=await call('tasks/create',{title,notes:'Synthetic work only. Define the audience, prepare the evidence and return the result for a human decision. No live messages or model calls.',domain,priority,assigneeId,dueDate,ownerId:context.member.id,reviewerId:manager.id});
      if (tasks.length===0 || tasks.length===4) detail=await call('tasks/progress',{taskId:detail.task.id,expectedRevision:detail.task.revision,status:'in_progress'});
      tasks.push({id:detail.task.id,title:detail.task.title,status:detail.task.status});
    }
    return {organizationId,managerId:manager.id,contributorId:contributor.id,agentId:agent.id,tasks};
  });
  await page.getByRole('button',{name:'Refresh',exact:true}).click();
  await page.getByRole('button',{name:/Shape the website launch brief/}).waitFor();
  await page.screenshot({path:'reports/business-intake-ui-evidence/workbench-list-1280-light-before.png'});
  return seeded;
}
