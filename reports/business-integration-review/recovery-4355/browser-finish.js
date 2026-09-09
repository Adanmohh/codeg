async page => {
  const panel = page.getByRole('region', { name: 'Review before sharing', exact: true })
  const confirmation = panel.getByRole('checkbox')
  const unconfirmed = !(await confirmation.isChecked())
  const acceptDisabled = await panel.getByRole('button', { name: 'Accept into shared work', exact: true }).isDisabled()
  if (!unconfirmed || !acceptDisabled) throw new Error('Publication confirmation changed')
  page.__reviewChecks.push({ case: 'prepared_review_unconfirmed', passed: true, unconfirmed, acceptDisabled })
  await panel.scrollIntoViewIfNeeded()
  await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))))
  await page.screenshot({ path: 'reports/business-integration-review/recovery-4355/prepared-unconfirmed-1280.png', fullPage: true })
  await Promise.all(page.__reviewDtoPending)
  const traffic = page.__reviewTraffic
  const denied = traffic.filter(item => item.blocked)
  const errors = traffic.filter(item => item.status && item.status !== 200)
  const candidateWrites = traffic.filter(item => /\/candidates\/(create|select|edit|accept|link|discard)$/.test(item.path))
  const taskWrites = traffic.filter(item => /\/tasks\/(create|update|assign|progress|note|submit|review|cancel|archive|link-execution)$/.test(item.path))
  const imports = traffic.filter(item => /\/imports\/(start|advance)$/.test(item.path))
  if (denied.length || errors.length || candidateWrites.length || taskWrites.length || imports.length !== 4 || page.__reviewDtos.some(item => item.evidenceReadError)) throw new Error('Unexpected browser transport activity')
  return {
    session: 'review-intake-recovery-4355', namespace: 'review',
    source: '60d600db0f224d44ff191490ab79bc25530ba959',
    ui: 'http://127.0.0.1:4355/business', backend: 'http://127.0.0.1:4351',
    interceptedResponses: 0, syntheticProvider: 'http://127.0.0.1:4352',
    checks: page.__reviewChecks, traffic, dtos: page.__reviewDtos,
    counts: { requests: traffic.length, blocked: denied.length, non200: errors.length, browserCandidateWrites: candidateWrites.length, browserTaskWrites: taskWrites.length, browserImportActions: imports.length },
    now: new Date().toISOString(), status: 'passed',
  }
}
