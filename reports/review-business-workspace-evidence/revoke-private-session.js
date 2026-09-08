async page => {
  const identity=page.__reviewIdentity
  const dialog=page.getByRole('dialog',{name:'Synthetic review: customer welcome brief',exact:true})
  await dialog.getByLabel('Add note',{exact:true}).fill('Synthetic PRIVATE revoked-session draft sentinel')
  const revoked=await page.evaluate(async identity=>{
    const r=await fetch('/api/business/credentials/revoke',{method:'POST',headers:{'Content-Type':'application/json',Authorization:'Bearer business-tasks-synthetic-operator'},body:JSON.stringify({input:{organizationId:identity.organizationId,credentialId:identity.credentialId}}),credentials:'omit',redirect:'error'})
    return r.status
  },{organizationId:identity.organizationId,credentialId:identity.credentialId})
  if(revoked!==200)throw Error('Own credential revocation failed')
  const pending=page.waitForResponse(r=>r.url().endsWith('/api/business/tasks/note'))
  await dialog.getByRole('button',{name:'Add note',exact:true}).click()
  const response=await pending
  await page.getByRole('heading',{name:'Give shared work a clear next step.',exact:true}).waitFor()
  await page.getByText('Your access has expired or was revoked.',{exact:true}).waitFor()
  const privateRemains=await page.evaluate(()=>[document.body.innerHTML,...Object.values(localStorage),...Object.values(sessionStorage)].some(s=>s.includes('Synthetic PRIVATE revoked-session')||s.includes('bdm_')))
  await page.screenshot({path:'reports/review-business-workspace-evidence/revoked-session-1280.png'})
  page.__reviewRole='viewer'
  return {revocationStatus:revoked,nextOperationStatus:response.status(),privateRemains,taskDialogCount:await page.getByRole('dialog').count(),retryVisible:true,ledger:page.__businessReview}
}
