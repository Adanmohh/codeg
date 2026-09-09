async page => {
  const cases=[];
  for(const locale of ['en','ar']) for(const theme of ['light','dark']) for(const width of [1280,768,390]) {
    await page.setViewportSize({width:1280,height:900});
    await page.getByRole('combobox',{name:/^(Language|اللغة)$/}).selectOption(locale);
    await page.getByRole('combobox',{name:/^(Appearance|المظهر)$/}).selectOption(theme);
    await page.setViewportSize({width,height:width===1280?900:844});
    if(width<1024)await page.getByRole('button',{name:locale==='en'?'Open workspace navigation':'فتح قائمة مساحة العمل',exact:true}).click();
    const selected=page.getByRole('navigation',{name:locale==='en'?'Workspace':'مساحة العمل',exact:true}).getByRole('button',{name:locale==='en'?'Sources':'المصادر',exact:true});
    if(await selected.getAttribute('aria-current')!=='page')throw Error('Wrong selected navigation item');
    await selected.focus();await page.keyboard.press('Tab');await page.keyboard.press('Shift+Tab');
    await page.waitForTimeout(500);
    const sample=await selected.evaluate(el=>{
      const c=document.createElement('canvas');c.width=1;c.height=1;const ctx=c.getContext('2d',{willReadFrequently:true});
      const rgba=v=>{ctx.clearRect(0,0,1,1);ctx.fillStyle=v;ctx.fillRect(0,0,1,1);return [...ctx.getImageData(0,0,1,1).data];};
      const layers=[];for(let n=el;n;n=n.parentElement)layers.push(rgba(getComputedStyle(n).backgroundColor));
      let bg=[255,255,255];for(const a of layers.reverse())bg=bg.map((v,i)=>Math.round(a[i]*a[3]/255+v*(1-a[3]/255)));
      const cs=getComputedStyle(el),box=el.getBoundingClientRect();
      return {text:el.textContent,fg:rgba(cs.color),bg,size:parseFloat(cs.fontSize),weight:cs.fontWeight,width:box.width,height:box.height,focus:el.matches(':focus-visible'),shadow:cs.boxShadow,overflow:document.documentElement.scrollWidth>innerWidth,dir:document.documentElement.dir};
    });
    if(!sample.focus||sample.shadow==='none'||sample.overflow||sample.width<44||sample.height<44)throw Error('Navigation focus or layout failed');
    await page.screenshot({path:'reports/business-intake-ui-evidence/navigation-'+width+'-'+theme+'-'+locale+'.png'});
    if(width<1024){await page.keyboard.press('Escape');await page.waitForTimeout(500);}
    cases.push({locale,theme,width,...sample});
  }
  return {cases,legacyEntry:await page.getByRole('link',{name:/Open engineering workspace/}).count()};
}
