async page => {
  const preferences=await page.context().newPage();
  await preferences.route('**/*',route=>route.request().url().startsWith('http://127.0.0.1:4340/')?route.continue():route.abort());
  await preferences.goto('http://127.0.0.1:4340/business.html');
  const dialog=page.getByRole('dialog',{name:'Prepare the feedback summary · Synthetic',exact:true});
  const button=dialog.getByRole('button',{name:'Cancel task',exact:true});
  const results=[];
  for(const theme of ['light','dark']) {
    await preferences.getByLabel('Appearance',{exact:true}).selectOption(theme);
    await page.waitForFunction(value=>document.documentElement.classList.contains('dark')===(value==='dark'),theme);
    await page.bringToFront();
    await button.scrollIntoViewIfNeeded();
    for(const state of ['normal','hover','focus']) {
      if(state==='normal') {await dialog.getByRole('button',{name:'Close',exact:true}).focus();await button.scrollIntoViewIfNeeded();await page.mouse.move(1,1);}
      if(state==='hover') await button.hover();
      if(state==='focus') {await button.focus();await page.keyboard.press('Tab');await page.keyboard.press('Shift+Tab');await page.mouse.move(1,1);}
      await button.evaluate(el=>Promise.all(el.getAnimations().filter(a=>a.currentTime!==null&&a.effect.getTiming().iterations!==Infinity).map(a=>a.finished.catch(()=>{}))));
      const sample=await button.evaluate(el=>{
        const canvas=document.createElement('canvas');canvas.width=canvas.height=1;const ctx=canvas.getContext('2d',{willReadFrequently:true});
        const rgba=value=>{ctx.clearRect(0,0,1,1);ctx.fillStyle=value;ctx.fillRect(0,0,1,1);return [...ctx.getImageData(0,0,1,1).data];};
        const layers=[];for(let n=el;n;n=n.parentElement)layers.push(rgba(getComputedStyle(n).backgroundColor));
        let bg=[255,255,255];for(const color of layers.reverse())bg=bg.map((v,i)=>Math.round(color[i]*(color[3]/255)+v*(1-color[3]/255)));
        const cs=getComputedStyle(el),color=rgba(cs.color),border=rgba(cs.borderColor);
        return {text:el.textContent.trim(),fg:'rgb('+color.slice(0,3).join(', ')+')',bg:'rgb('+bg.join(', ')+')',sizePx:parseFloat(cs.fontSize),bold:parseFloat(cs.fontWeight)>=700,disabled:el.disabled,focusVisible:el.matches(':focus-visible'),border:'rgba('+border.slice(0,3).join(', ')+', '+border[3]/255+')',shadow:cs.boxShadow};
      });
      if(sample.disabled||(state==='focus'&&!sample.focusVisible))throw Error('State capture does not match real active control');
      await page.screenshot({path:'reports/business-workspace-evidence/generation2/cancel-'+theme+'-'+state+'-after.png'});
      results.push({theme,state,...sample});
    }
  }
  await preferences.close();await page.bringToFront();
  return {synthetic:true,uiSource:'08286d45 plus scoped motion/destructive fixes',results,noCancellationPerformed:true};
}
