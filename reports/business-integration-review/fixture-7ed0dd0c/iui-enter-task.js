async page => {
  page.__reviewIui = { source: '1974d97f44b69db96051ace1c68717cc8c27b6e9', backend: 'e55f3bfd1f069d6d6111370223993596b19ecb9b', assertions: [] }
  page.__reviewIui.trafficStart = page.__reviewTraffic.length
  const brief = page.getByRole('textbox', { name: 'Brief', exact: true })
  if (await brief.inputValue() !== 'Reviewer saved candidate A baseline') throw new Error('Unexpected reviewer baseline')
  await brief.fill('Synthetic unsaved source A sentinel')
  page.__reviewIui.assertions.push({ name: 'Unsaved sentinel entered in actual candidate A', pass: await brief.inputValue() === 'Synthetic unsaved source A sentinel' })
  await page.getByRole('tab', { name: 'My work', exact: true }).click()
  await page.getByRole('button', { name: /To do Feedback · Normal review: existing reviewed customer follow-up/ }).click()
  return { taskOpened: true, assertions: page.__reviewIui.assertions }
}
