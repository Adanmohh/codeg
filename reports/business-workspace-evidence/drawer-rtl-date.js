async page => {
  await page.keyboard.press('Escape');
  await page.setViewportSize({width:1280,height:900});
  if(await page.getByLabel('اللغة',{exact:true}).count())await page.getByLabel('اللغة',{exact:true}).selectOption('en');
  await page.getByRole('navigation',{name:'Workspace',exact:true}).getByRole('button',{name:'Shared work',exact:true}).click();
  await page.getByLabel('Language',{exact:true}).selectOption('ar');
  await page.waitForFunction(()=>document.documentElement.dir==='rtl');
  await page.setViewportSize({width:390,height:900});
  const menu=page.getByRole('button',{name:'فتح قائمة مساحة العمل',exact:true});
  await menu.focus();await page.keyboard.press('Enter');
  const drawer=page.getByRole('dialog',{name:'مساحة العمل',exact:true});
  await drawer.waitFor();
  await drawer.getByRole('button',{name:'إغلاق',exact:true}).waitFor();
  await page.waitForFunction(()=>{const el=document.querySelector('[data-slot=drawer-popup]');if(!el||el.hasAttribute('data-starting-style'))return false;const transform=getComputedStyle(el).transform;return transform==='none'||transform==='matrix(1, 0, 0, 1, 0, 0)';},undefined,{timeout:2000});
  const drawerBox=await drawer.evaluate(el=>{const r=el.getBoundingClientRect();return {left:r.left,right:r.right,width:r.width,viewport:innerWidth,scroll:document.documentElement.scrollWidth,direction:document.documentElement.dir};});
  if(drawerBox.direction!=='rtl'||drawerBox.left<0||drawerBox.right>390||drawerBox.left<drawerBox.viewport-drawerBox.right)throw Error('RTL drawer did not use right edge');
  const focus=[];for(let i=0;i<10;i++){await page.keyboard.press('Tab');await page.waitForFunction(()=>!!document.activeElement.closest('[role=dialog]'),undefined,{timeout:1000});focus.push(await page.evaluate(()=>!!document.activeElement.closest('[role=dialog]')));}
  if(focus.some(value=>!value))throw Error('Drawer focus escaped');
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/drawer-390-rtl.png'});
  await page.keyboard.press('Escape');
  await drawer.waitFor({state:'hidden'});
  const restored=await menu.evaluate(el=>el===document.activeElement);
  if(!restored)throw Error('Drawer did not restore menu focus');
  await page.getByRole('button').filter({has:page.getByRole('heading',{name:'Check the homepage promise · Synthetic',exact:true})}).click();
  const dialog=page.getByRole('dialog',{name:'Check the homepage promise · Synthetic',exact:true});
  const time=dialog.locator('time[datetime="2026-12-31"]').first();
  await time.waitFor();
  const date=await time.evaluate(el=>({text:el.textContent,datetime:el.getAttribute('datetime'),direction:getComputedStyle(el).direction}));
  if(date.text!=='2026-12-31'||date.datetime!==date.text||date.direction!=='ltr')throw Error('RTL calendar day changed');
  await time.scrollIntoViewIfNeeded();
  await page.screenshot({path:'reports/business-workspace-evidence/generation2/date-390-rtl.png'});
  await dialog.getByRole('button',{name:'إغلاق',exact:true}).click();
  await page.waitForFunction(()=>document.activeElement.tagName==='BUTTON'||document.activeElement.id==='business-main');
  const taskFocus=await page.evaluate(()=>({tag:document.activeElement.tagName,id:document.activeElement.id}));
  await page.setViewportSize({width:1280,height:900});
  await page.getByLabel('اللغة',{exact:true}).selectOption('en');
  return {synthetic:true,drawerBox,focusContained:focus.length,menuFocusRestored:restored,date,taskFocus};
}
