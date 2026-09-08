async page => {
  await page.setViewportSize({width:390,height:844})
  const results=[]
  for(const reducedMotion of ['no-preference','reduce']) {
    await page.emulateMedia({reducedMotion})
    await page.evaluate(()=>{
      window.__reviewFrames=[]
      window.__reviewFramesDone=new Promise(resolve=>{
        const start=performance.now()
        function sample(time){
          const e=document.querySelector('[data-slot="drawer-popup"]')
          if(e){const s=getComputedStyle(e),r=e.getBoundingClientRect();window.__reviewFrames.push({time:time-start,x:r.x,width:r.width,transform:s.transform,transition:s.transitionProperty,duration:s.transitionDuration,visible:e.checkVisibility({opacityProperty:true,visibilityProperty:true}),animations:e.getAnimations().map(a=>({type:a.constructor.name,keyframes:a.effect?.getKeyframes()}))})}
          if(time-start<900)requestAnimationFrame(sample);else resolve()
        }
        requestAnimationFrame(sample)
      })
    })
    await page.getByRole('button',{name:'Open workspace navigation',exact:true}).click()
    const popup=page.locator('[data-slot="drawer-popup"]')
    await popup.waitFor()
    const frames=await page.evaluate(async()=>{await window.__reviewFramesDone;return window.__reviewFrames})
    await page.keyboard.press('Tab')
    const focus=await page.evaluate(()=>{const e=document.activeElement,s=getComputedStyle(e);return{text:e.textContent,insideDrawer:!!e.closest('[data-slot="drawer-popup"]'),outline:s.outline,shadow:s.boxShadow,focusVisible:e.matches(':focus-visible')}})
    await page.screenshot({path:'reports/review-business-workspace-evidence/drawer-'+reducedMotion+'-390-light.png'})
    await page.keyboard.press('Escape')
    await popup.waitFor({state:'detached'})
    const restored=await page.getByRole('button',{name:'Open workspace navigation',exact:true}).evaluate(e=>e===document.activeElement)
    results.push({reducedMotion,frames,focus,restored,detached:true})
  }
  await page.emulateMedia({reducedMotion:'no-preference'})
  await page.getByRole('button',{name:'Open workspace navigation',exact:true}).click()
  const popup=page.locator('[data-slot="drawer-popup"]')
  await popup.getByRole('combobox',{name:'Language',exact:true}).selectOption('ar')
  await popup.getByRole('combobox',{name:'المظهر',exact:true}).selectOption('dark')
  await page.evaluate(()=>new Promise(resolve=>setTimeout(resolve,600)))
  const rtl=await popup.evaluate(e=>({dir:getComputedStyle(e).direction,x:e.getBoundingClientRect().x,width:e.getBoundingClientRect().width,viewport:innerWidth,side:e.getAttribute('data-swipe-direction')}))
  await page.screenshot({path:'reports/review-business-workspace-evidence/drawer-390-dark-ar.png'})
  await popup.getByRole('button',{name:'العمل المشترك',exact:true}).click()
  await popup.waitFor({state:'detached'})
  await page.getByRole('heading',{name:'العمل المشترك',exact:true}).waitFor()
  await page.screenshot({path:'reports/review-business-workspace-evidence/shared-390-dark-ar.png'})
  await page.setViewportSize({width:1280,height:900})
  await page.getByRole('combobox',{name:'اللغة',exact:true}).selectOption('en')
  await page.getByRole('combobox',{name:'Appearance',exact:true}).selectOption('light')
  return {results,rtl,navigationSettled:true}
}
