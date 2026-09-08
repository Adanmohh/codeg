async page => {
  await page.setViewportSize({width:1280,height:900});
  const styles=[];
  for(const theme of ['light','dark']) {
    await page.getByLabel('Appearance',{exact:true}).selectOption(theme);
    await page.waitForFunction(dark=>document.documentElement.classList.contains('dark')===dark,theme==='dark');
    styles.push(await page.evaluate(theme=>{
      const sample=(el)=>{const s=getComputedStyle(el);return {className:el.className,color:s.color,background:s.backgroundColor,font:s.fontFamily,radius:s.borderRadius,marginTop:s.marginTop,gap:s.gap};};
      return {theme,sidebarAutoSpace:sample(document.querySelector('aside .mt-auto')),selectedNavigation:sample(document.querySelector('aside nav [aria-current=page]')),roundContainers:[...document.querySelectorAll('#business-main .rounded-2xl')].map(sample),button:sample(document.querySelector('button[data-variant=default]'))};
    },theme));
  }
  await page.getByLabel('Appearance',{exact:true}).selectOption('light');
  const row=page.getByRole('button').filter({has:page.getByRole('heading',{name:'Check the homepage promise · Synthetic',exact:true})});
  await row.focus();await page.keyboard.press('Enter');
  const dialog=page.getByRole('dialog',{name:'Check the homepage promise · Synthetic',exact:true});
  await dialog.waitFor();await page.keyboard.press('Escape');await dialog.waitFor({state:'hidden'});
  await page.waitForFunction(()=>document.activeElement.id==='business-main'||document.activeElement.tagName==='BUTTON',undefined,{timeout:2000});
  const focus=await page.evaluate(()=>{const el=document.activeElement,s=getComputedStyle(el);return {tag:el.tagName,id:el.id,visible:el.matches(':focus-visible'),ring:s.boxShadow};});
  if(!focus.visible||focus.ring==='none')throw Error('Keyboard focus return is invisible');
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/final-keyboard-return.png'});
  return {synthetic:true,styles,keyboardReturn:focus};
}
