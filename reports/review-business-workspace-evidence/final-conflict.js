// Report-only real protected API/CLI fixture check. No provider or engine activity.
// Synthetic credential-in-memory setup adapts member-connect.js; see NOTICE.md.
async page => {
  const title = 'Synthetic review: final comparison'
  const draft = 'Synthetic review FINAL retained draft — explicit adoption required.'
  const submitted = 'Synthetic review FINAL deliverable: a clear first step for the customer, ready for Maya.'
  const identity = page.__reviewIdentity
  const assignee = identity.members.find(m => m.role === 'member')
  const manager = identity.members.find(m => m.role === 'manager')
  await page.getByRole('button', {name:'Create task', exact:true}).click()
  let dialog = page.getByRole('dialog', {name:'Create task', exact:true})
  await dialog.getByLabel('Task title', {exact:true}).fill(title)
  await dialog.getByLabel('Brief', {exact:true}).fill('Synthetic current saved brief. Maya will compare it with an unsaved draft.')
  await dialog.getByLabel('Assigned to', {exact:true}).selectOption(assignee.id)
  await dialog.getByLabel('Reviewer', {exact:true}).selectOption(manager.id)
  await dialog.getByLabel('Due date', {exact:true}).fill('2026-10-02')
  const creating = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/create'))
  await dialog.getByRole('button', {name:'Create task', exact:true}).click()
  const creationResponse = await creating
  if (creationResponse.status() !== 200) throw Error('Synthetic create failed')
  const created = await creationResponse.json()
  page.__finalReviewTaskId = created.task.id
  dialog = page.getByRole('dialog', {name:title, exact:true})
  await dialog.getByRole('button', {name:'Edit task', exact:true}).click()
  await dialog.getByLabel('Brief', {exact:true}).fill(draft)

  // An independent human credential is the intervening writer. Real backend
  // responses are preserved; this fixture setup is distinct from the UI recheck.
  const intervention = await page.evaluate(async ({organizationId, memberId, taskId, submitted}) => {
    const operator = 'business-tasks-synthetic-operator'
    const post = async (path, input, token) => fetch('/api/business/' + path, {
      method:'POST', headers:{'Content-Type':'application/json', Authorization:'Bearer ' + token},
      body:JSON.stringify({input}), credentials:'omit', redirect:'error',
    })
    const issueResponse = await post('credentials/issue', {organizationId, memberId, label:'Independent final conflict intervening writer'}, operator)
    if (!issueResponse.ok) throw Error('Synthetic individual credential issuance failed')
    const issued = await issueResponse.json()
    try {
      const startResponse = await post('tasks/progress', {taskId, expectedRevision:1, status:'in_progress'}, issued.token)
      if (!startResponse.ok) throw Error('Intervening start status ' + startResponse.status)
      const start = await startResponse.json()
      const submitResponse = await post('tasks/submit', {taskId, expectedRevision:start.task.revision, body:submitted}, issued.token)
      if (!submitResponse.ok) throw Error('Intervening submit status ' + submitResponse.status)
      const current = await submitResponse.json()
      return {kind:'Real API synthetic individual human writer', startStatus:startResponse.status, submitStatus:submitResponse.status, task:current.task, deliverables:current.deliverables, execution:current.execution}
    } finally {
      const revoked = await post('credentials/revoke', {organizationId, credentialId:issued.credential.id}, operator)
      issued.token = ''
      if (!revoked.ok) throw Error('Temporary writer credential revocation failed')
    }
  }, {organizationId:identity.organizationId, memberId:assignee.id, taskId:created.task.id, submitted})

  const saving = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/update'))
  await dialog.getByRole('button', {name:'Save changes', exact:true}).click()
  const rejected = await saving
  if (rejected.status() !== 409) throw Error('Expected actual stale write409')
  await dialog.getByText('This task changed while you were editing.', {exact:true}).waitFor()
  const loading = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/get'))
  await dialog.getByRole('button', {name:'Load current task', exact:true}).click()
  const loadedResponse = await loading
  const loaded = await loadedResponse.json()
  const base = dialog.getByRole('group', {name:"Your draft's base version", exact:true})
  const current = dialog.getByRole('region', {name:'Current saved version', exact:true})
  await current.getByText('Revision 3', {exact:true}).waitFor()
  if (!(await base.innerText()).includes('Revision 1') || !(await base.innerText()).includes('To do')) throw Error('Draft base version unclear')
  if (!(await current.innerText()).includes('Review') || !(await current.innerText()).includes(submitted)) throw Error('Current saved state/evidence missing')
  if (await dialog.getByLabel('Brief', {exact:true}).inputValue() !== draft) throw Error('Draft lost')
  if (!await dialog.getByRole('button', {name:'Save changes', exact:true}).isDisabled()) throw Error('Stale Save not locked')
  await page.setViewportSize({width:390,height:900})
  await current.getByRole('heading', {name:'Current saved version', exact:true}).scrollIntoViewIfNeeded()
  await page.waitForTimeout(600)
  const geometry = await dialog.evaluate(el => ({opacity:getComputedStyle(el).opacity, activeAnimations:el.getAnimations({subtree:true}).filter(a=>a.playState==='running').length, documentWidth:document.documentElement.scrollWidth}))
  await page.screenshot({path:'reports/review-business-workspace-evidence/final-conflict-current-390-light.png'})
  const comparison = {base:await base.innerText(), current:await current.innerText(), draft:await dialog.getByLabel('Brief', {exact:true}).inputValue(), snapshot:await dialog.ariaSnapshot(), geometry}
  await current.getByRole('button', {name:'Use my draft with this version', exact:true}).click()
  if (await dialog.getByLabel('Brief', {exact:true}).inputValue() !== draft) throw Error('Adoption changed draft')
  const adoptedSaving = page.waitForResponse(r => r.url().endsWith('/api/business/tasks/update'))
  await dialog.getByRole('button', {name:'Save changes', exact:true}).click()
  const adoptedResponse = await adoptedSaving
  const adopted = await adoptedResponse.json()
  if (adoptedResponse.status() !== 200 || adopted.task.revision !== 4 || adopted.task.status !== 'in_progress' || adopted.task.notes !== draft || adopted.task.currentDeliverableId !== null) throw Error('Explicit revision adoption invariant failed')
  await dialog.getByText('Revision 4', {exact:true}).waitFor()
  await page.waitForTimeout(500)
  await page.screenshot({path:'reports/review-business-workspace-evidence/final-conflict-adopted-390-light.png'})
  const noAccept = await dialog.getByRole('button', {name:'Accept work', exact:true}).count() === 0
  if (!noAccept) throw Error('Stale deliverable acceptance remains available')
  const privateStored = await page.evaluate(() => [...Object.values(localStorage), ...Object.values(sessionStorage)].some(s=>s.includes('Synthetic review FINAL retained draft') || s.includes('bdm_')))
  if (privateStored) throw Error('Private draft/credential persisted')
  return {source:'e72cc44b612068e67a3e6dc3bc593f10988ae7ed', taskId:created.task.id, creationStatus:creationResponse.status(), intervention, staleWrite:{status:rejected.status(), input:rejected.request().postDataJSON().input}, loaded:{status:loadedResponse.status(), revision:loaded.task.revision, statusName:loaded.task.status}, comparison, adopted:{status:adoptedResponse.status(), input:adoptedResponse.request().postDataJSON().input, task:adopted.task, execution:adopted.execution}, noAccept, privateStored, ledger:page.__businessReview}
}
