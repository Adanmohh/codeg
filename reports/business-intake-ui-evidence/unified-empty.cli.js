async page => {
  await page.getByRole('button',{name:'Sources',exact:true}).click();
  await page.getByText('No sources available yet.',{exact:true}).waitFor();
  const setup = await page.getByRole('button',{name:'Set up a source',exact:true}).count();
  await page.screenshot({path:`reports/business-intake-ui-evidence/unified-empty-${setup?'owner':'manager'}-1280-light.png`});
  return {empty:true,setupControls:setup,importControls:await page.getByRole('button',{name:'Import records',exact:true}).count(),legacyEntry:await page.getByRole('link',{name:'Open engineering workspace',exact:true}).count(),overflow:await page.evaluate(()=>document.documentElement.scrollWidth>innerWidth)};
}
