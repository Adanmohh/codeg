async page => {
  const results = [];
  await page.setViewportSize({width:1280,height:900});
  await page.getByRole('navigation',{name:'Workspace',exact:true}).getByRole('button',{name:'Shared work',exact:true}).click();
  await page.getByRole('heading',{name:'Prepare the feedback summary · Synthetic',exact:true}).waitFor();
  for (const theme of ['light','dark']) {
    await page.setViewportSize({width:1280,height:900});
    await page.getByLabel('Appearance',{exact:true}).selectOption(theme);
    await page.waitForFunction(value => document.documentElement.classList.contains('dark') === (value === 'dark'),theme);
    for (const width of [1280,768,390]) {
      await page.setViewportSize({width,height:900});
      for (const mode of ['List','Board']) {
        await page.getByRole('button',{name:mode,exact:true}).click();
        await page.screenshot({path:'reports/business-workspace-evidence/generation2/work-'+mode.toLowerCase()+'-'+width+'-'+theme+'.png'});
        const measured = await page.evaluate(() => {
          const root = document.querySelector('#business-main');
          const visible = el => el.getClientRects().length > 0 && getComputedStyle(el).visibility !== 'hidden';
          return {
            width:innerWidth,pageWidth:document.documentElement.scrollWidth,
            selects:[...root.querySelectorAll('select')].filter(visible).map(el=>({label:el.getAttribute('aria-label'),value:el.selectedOptions[0]?.textContent,width:el.getBoundingClientRect().width,height:el.getBoundingClientRect().height,font:parseFloat(getComputedStyle(el).fontSize)})),
            board:[...root.querySelectorAll('[aria-label="Board"]')].map(el=>({client:el.clientWidth,scroll:el.scrollWidth})),
            smallTargets:[...root.querySelectorAll('button,input,select,summary,a')].filter(visible).filter(el=>!el.classList.contains('sr-only') && !el.disabled).map(el=>{let target=el; if(el.matches('input[type=checkbox]')) target=el.closest('label')||el;const r=target.getBoundingClientRect();return {label:el.getAttribute('aria-label')||el.textContent?.trim().slice(0,45)||el.type,width:r.width,height:r.height};}).filter(rect=>rect.width<43.5||rect.height<43.5),
          };
        });
        if(measured.pageWidth>width) throw Error('Page overflow: '+JSON.stringify({theme,mode,...measured}));
        results.push({theme,mode,...measured});
      }
    }
  }
  await page.setViewportSize({width:1280,height:900});
  await page.getByLabel('Appearance',{exact:true}).selectOption('light');
  await page.getByRole('button',{name:'List',exact:true}).click();
  return {synthetic:true,uiSource:'45255b86',backendSource:'1ba73e3c + fixture-temporary-disk.patch',results};
}
