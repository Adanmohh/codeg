// Synthetic fixture setup only. Credentials stay in this CLI invocation and the UI client.
// Adapted from the Apache-2.0 PR21 member-connect.js; see this directory's NOTICE.
async page => {
  const which = page.__reviewRole || 'manager'
  const issued = await page.evaluate(async role => {
    const post = async (path, input) => {
      const response = await fetch('/api/business/' + path, {method: 'POST', headers: {'Content-Type': 'application/json', Authorization: 'Bearer business-tasks-synthetic-operator'}, body: JSON.stringify({input}), credentials: 'omit', redirect: 'error'})
      if (!response.ok) throw Error('Synthetic setup status ' + response.status)
      return response.json()
    }
    const context = await post('context', {})
    const organizationId = context.organization.id
    const people = await post('members/list', {organizationId})
    const definitions = [
      {role: 'manager', displayName: 'Synthetic review · Maya · Manager', domains: ['marketing', 'feedback']},
      {role: 'member', displayName: 'Synthetic review · Noura · Member', domains: ['marketing']},
      {role: 'viewer', displayName: 'Synthetic review · Idris · Viewer', domains: ['marketing']},
    ]
    const members = []
    for (const definition of definitions) {
      let member = people.find(item => item.displayName === definition.displayName)
      if (!member) member = await post('members/create', {...definition, organizationId, kind: 'human'})
      if (member.status !== 'active') throw Error('Review identity unexpectedly inactive')
      members.push(member)
    }
    const member = members.find(item => item.role === role)
    const result = await post('credentials/issue', {organizationId, memberId: member.id, label: 'Independent synthetic UI review · ' + role})
    return {token: result.token, credentialId: result.credential.id, organizationId, members: members.map(({id, displayName, role}) => ({id, displayName, role})), memberId: member.id, role}
  }, which)
  await page.getByLabel('Personal access token', {exact: true}).fill(issued.token)
  await page.getByRole('button', {name: 'Connect', exact: true}).click()
  issued.token = ''
  await page.getByRole('heading', {name: 'My work', exact: true}).waitFor()
  page.__reviewIdentity = issued
  const stored = await page.evaluate(() => [...Object.values(localStorage), ...Object.values(sessionStorage)].some(value => value.includes('bdm_')))
  if (stored) throw Error('Personal bearer persisted')
  return {memberId: issued.memberId, role: issued.role, members: issued.members, organizationId: issued.organizationId, privateCredentialStored: stored, legacyLinkVisible: await page.getByRole('link', {name: 'Open engineering workspace', exact: true}).count()}
}
