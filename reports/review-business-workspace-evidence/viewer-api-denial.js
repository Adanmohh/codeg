async page => {
  return page.evaluate(async()=>{
    const post=async(path,input,token)=>fetch('/api/business/'+path,{method:'POST',headers:{'Content-Type':'application/json',Authorization:'Bearer '+token},body:JSON.stringify({input}),credentials:'omit',redirect:'error'})
    const operator='business-tasks-synthetic-operator'
    const context=await (await post('context',{},operator)).json()
    const organizationId=context.organization.id
    const members=await (await post('members/list',{organizationId},operator)).json()
    const member=members.find(m=>m.displayName==='Synthetic review · Idris · Viewer')
    const issued=await (await post('credentials/issue',{organizationId,memberId:member.id,label:'Independent negative API assertions only'},operator)).json()
    try {
      const hidden=await post('tasks/get',{taskId:'e2705d60-ed03-4770-b545-8ae25e88607e'},issued.token)
      const readable=await post('tasks/get',{taskId:'e64a0ac7-cc34-4442-a90b-ceaa25a86323'},issued.token)
      const readableBody=await readable.json()
      const deniedCreate=await post('tasks/create',{title:'Synthetic forbidden viewer write',notes:'This must not persist',domain:'marketing',priority:'normal',dueDate:null},issued.token)
      const deniedNote=await post('tasks/note',{taskId:'e64a0ac7-cc34-4442-a90b-ceaa25a86323',expectedRevision:6,body:'Synthetic forbidden viewer note'},issued.token)
      const directory=await (await post('members/list',{organizationId},issued.token)).json()
      return {kind:'Real protected API, synthetic individual viewer bearer held only in function memory',hiddenTaskStatus:hidden.status,readableStatus:readable.status,canWrite:readableBody.task.capabilities,createStatus:deniedCreate.status,noteStatus:deniedNote.status,directoryDomains:directory.map(m=>({name:m.displayName,domains:m.domains}))}
    } finally {
      await post('credentials/revoke',{organizationId,credentialId:issued.credential.id},operator)
      issued.token=''
    }
  })
}
