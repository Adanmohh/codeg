async page => {
  const fs=page.constructor.constructor('return process')().getBuiltinModule('node:fs');
  const c=JSON.parse(fs.readFileSync('/Users/mohamedadan/projects/_worktrees/ops-desk/tickets/.docs/business-intake-fixtures/unified-YQmRz8/ui-credentials.json','utf8'));
  return await page.evaluate(async token=>{
    const call=async(path,input)=>{
      const r=await fetch('/api/business/'+path,{method:'POST',headers:{'Content-Type':'application/json',Authorization:'Bearer '+token},body:JSON.stringify({input}),cache:'no-store',credentials:'omit'});
      if(!r.ok)throw Error('Safe read failed '+r.status);
      return r.json();
    };
    const b=await call('intake/bindings/list',{});
    const binding=b.items.find(v=>v.binding.label==='Synthetic UI meeting notes');
    if(!binding)throw Error('Missing own synthetic binding');
    const sources=await call('intake/sources/list',{bindingId:binding.binding.id});
    const rows=[];
    for(const source of sources.items){
      const pending=await call('intake/candidates/list',{sourceId:source.id,state:'pending'});
      rows.push({sourceId:source.id,title:source.title,revision:source.revision,access:source.access,pending:pending.items.map(v=>({id:v.id,sourceId:v.sourceId,revision:v.revision,prepared:v.hasPreparedDraft}))});
    }
    const tasks=await call('tasks/list',{view:'shared',domain:'feedback'});
    return {sources:rows,tasks:tasks.tasks.map(v=>({id:v.id,title:v.title,revision:v.revision,status:v.status}))};
  },c.sessions.owner.token);
}
