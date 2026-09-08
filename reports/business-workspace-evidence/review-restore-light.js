async page => {
  const preferences=await page.context().newPage();
  await preferences.route('**/*',route=>route.request().url().startsWith('http://127.0.0.1:4340/')?route.continue():route.abort());
  await preferences.goto('http://127.0.0.1:4340/business.html');
  const language=preferences.locator('select').filter({has:preferences.locator('option[value=ar]')});
  await language.selectOption('en');
  await preferences.getByLabel('Appearance',{exact:true}).selectOption('light');
  await preferences.close();
  await page.bringToFront();
  await page.setViewportSize({width:1280,height:900});
  const dialog=page.getByRole('dialog',{name:'Prepare the feedback summary · Synthetic',exact:true});
  const cancel=dialog.getByRole('button',{name:'Cancel task',exact:true});
  await cancel.scrollIntoViewIfNeeded();
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/cancel-contrast-before.png'});
  return {synthetic:true,button:await cancel.evaluate(el=>({foreground:getComputedStyle(el).color,background:getComputedStyle(el).backgroundColor,disabled:el.disabled,fontSize:getComputedStyle(el).fontSize,opacity:getComputedStyle(el).opacity,visible:{top:el.getBoundingClientRect().top,bottom:el.getBoundingClientRect().bottom}})),reviewStillAwaitingDecision:true};
}
