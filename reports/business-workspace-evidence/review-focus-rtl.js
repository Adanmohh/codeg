async page => {
  const dialog=page.getByRole('dialog',{name:'Prepare the feedback summary · Synthetic',exact:true});
  await dialog.getByRole('button',{name:'Close',exact:true}).focus();
  const focus=[];
  for(let i=0;i<12;i++) {
    await page.keyboard.press('Tab');
    const value=await page.evaluate(()=>{const el=document.activeElement;const r=el.getBoundingClientRect();const s=getComputedStyle(el);return {tag:el.tagName,label:el.getAttribute('aria-label')||el.labels?.[0]?.textContent?.trim()||el.textContent?.trim().slice(0,55),inside:!!el.closest('[role=dialog]'),visible:r.top>=0&&r.bottom<=innerHeight,focusVisible:el.matches(':focus-visible'),ring:s.boxShadow,outline:s.outlineStyle};});
    if(!value.inside||!value.visible||!value.focusVisible)throw Error('Keyboard focus escaped or was clipped: '+JSON.stringify(value));
    focus.push(value);
  }
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/review-keyboard-focus.png'});
  await page.emulateMedia({reducedMotion:'reduce'});
  const motion=await dialog.evaluate(el=>({dialogAnimation:getComputedStyle(el).animationName,dialogTransform:getComputedStyle(el).transform,buttons:[...el.querySelectorAll('button')].map(node=>({text:node.textContent.trim().slice(0,30),property:getComputedStyle(node).transitionProperty,duration:getComputedStyle(node).transitionDuration})),overlayAnimation:getComputedStyle(document.querySelector('[data-slot=dialog-overlay]')).animationName}));
  if(motion.dialogAnimation!=='none'||motion.buttons.some(button=>button.property!=='none'))throw Error('Scoped reduced motion ineffective');
  await page.emulateMedia({reducedMotion:'no-preference'});
  const preferences=await page.context().newPage();
  await preferences.route('**/*',route=>route.request().url().startsWith('http://127.0.0.1:4340/')?route.continue():route.abort());
  await preferences.goto('http://127.0.0.1:4340/business.html');
  await preferences.getByLabel('Appearance',{exact:true}).selectOption('dark');
  await preferences.getByLabel('Language',{exact:true}).selectOption('ar');
  await dialog.getByLabel('ملاحظة المراجعة',{exact:true}).waitFor();
  await page.setViewportSize({width:390,height:900});
  await page.bringToFront();
  const rtl=await dialog.evaluate(el=>({direction:document.documentElement.dir,width:innerWidth,pageWidth:document.documentElement.scrollWidth,client:el.clientWidth,scroll:el.scrollWidth,dark:document.documentElement.classList.contains('dark')}));
  if(rtl.direction!=='rtl'||!rtl.dark||rtl.pageWidth>390||rtl.scroll>rtl.client)throw Error('RTL review overflow or theme failure');
  await dialog.getByLabel('ملاحظة المراجعة',{exact:true}).scrollIntoViewIfNeeded();
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/review-390-arabic-dark.png'});
  await preferences.close();
  await page.bringToFront();
  return {synthetic:true,focus,motion,rtl,reviewStillAwaitingDecision:true};
}
