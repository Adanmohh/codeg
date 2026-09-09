async page => {
  const results=[];
  await page.setViewportSize({width:1280,height:900});
  await page.getByRole('combobox',{name:/^(Language|اللغة)$/}).selectOption('en');
  for(const palette of ['neutral','violet','blue']) {
    await page.getByRole('button',{name:'Workspace appearance',exact:true}).click();
    const colors=page.getByRole('combobox',{name:'Workspace colors',exact:true});
    if(await colors.inputValue()!==palette) {
      await colors.selectOption(palette);
      const saved=page.waitForResponse(r=>r.url().endsWith('/api/business/settings/update')&&r.request().method()==='POST');
      await page.getByRole('button',{name:'Save workspace defaults',exact:true}).click();
      if((await saved).status()!==200)throw Error('Synthetic settings update failed');
    }
    await page.getByRole('tab',{name:'Synthetic · Shape the website launch brief',exact:true}).click();
    for(const theme of ['light','dark']) {
      await page.getByRole('combobox',{name:'Appearance',exact:true}).selectOption(theme);
      const button=page.getByRole('button',{name:'Save changes',exact:true});
      for(const state of ['normal','hover','focus']) {
        if(state==='normal')await page.getByRole('tabpanel',{name:'Synthetic · Shape the website launch brief',exact:true}).getByRole('heading',{name:'Synthetic · Shape the website launch brief',exact:true}).hover();
        if(state==='hover')await button.hover();
        if(state==='focus') { await page.getByRole('tabpanel',{name:'Synthetic · Shape the website launch brief',exact:true}).getByRole('heading',{name:'Synthetic · Shape the website launch brief',exact:true}).hover(); await button.focus();await page.keyboard.press('Tab');await page.keyboard.press('Shift+Tab'); }
        await page.waitForTimeout(500);
        const sample=await button.evaluate(el=>{
          const c=document.createElement('canvas');c.width=1;c.height=1;const ctx=c.getContext('2d',{willReadFrequently:true});
          const rgba=v=>{ctx.clearRect(0,0,1,1);ctx.fillStyle=v;ctx.fillRect(0,0,1,1);return [...ctx.getImageData(0,0,1,1).data];};
          const layers=[];for(let n=el;n;n=n.parentElement)layers.push(rgba(getComputedStyle(n).backgroundColor));
          let bg=[255,255,255];for(const a of layers.reverse())bg=bg.map((v,i)=>Math.round(a[i]*a[3]/255+v*(1-a[3]/255)));
          const style=getComputedStyle(el),scope=getComputedStyle(el.closest('[data-business-appearance]'));
          return {fg:rgba(style.color),bg,themeBackground:rgba(scope.getPropertyValue('--background')),themeForeground:rgba(scope.getPropertyValue('--foreground')),shadow:style.boxShadow,outline:style.outline,disabled:el.disabled,focusVisible:el.matches(':focus-visible')};
        });
        results.push({palette,theme,state,...sample});
      }
      await page.screenshot({path:'reports/business-intake-ui-evidence/primary-'+palette+'-'+theme+'-after.png'});
    }
  }
  if(await page.getByRole('textbox',{name:'Brief',exact:true}).inputValue()!=='Synthetic pane draft retained after the contrast correction.')throw Error('Lost private draft');
  return {results,draftRetained:true,rootPalette:await page.evaluate(()=>document.documentElement.getAttribute('data-theme'))};
}
