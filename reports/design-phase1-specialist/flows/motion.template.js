async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Owned fixture only")
  const capture = __CAPTURE__
  await page.setViewportSize({width:390,height:844})
  await page.keyboard.press("Escape")
  const cases=[],checks=[]
  for (const preference of ["no-preference","reduce"]) {
    await page.emulateMedia({reducedMotion:preference})
    const closed=await capture(page)
    cases.push({name:`motion-${preference}-closed`,raw:closed})
    const sample=page.evaluate(()=>new Promise(resolve=>{
      const frames=[],start=performance.now()
      const frame=()=>{
        const popup=document.querySelector('[data-slot="drawer-popup"]')
        const s=popup&&getComputedStyle(popup),r=popup?.getBoundingClientRect()
        frames.push({t:performance.now()-start,reduce:matchMedia("(prefers-reduced-motion: reduce)").matches,visible:popup?.checkVisibility(),x:r?.x,y:r?.y,width:r?.width,height:r?.height,transform:s?.transform,transition:s?.transition,scrollY,animationProperties:popup?.getAnimations().map(a=>({name:a.constructor.name,property:a.transitionProperty,keyframes:a.effect?.getKeyframes?.()}))})
        if(performance.now()-start<800) requestAnimationFrame(frame);else resolve(frames)
      };requestAnimationFrame(frame)
    }))
    await page.getByRole("button",{name:"Show Sidebar",exact:true}).click()
    const frames=await sample
    await page.screenshot({path:`reports/design-phase1-specialist/screenshots/drawer-${preference}-390.png`})
    cases.push({name:`motion-${preference}-open`,raw:await capture(page)})
    checks.push({preference,frames})
    await page.keyboard.press("Escape")
    await page.waitForTimeout(500)
    if(await page.getByRole("button",{name:"Show Sidebar",exact:true}).count()!==1) throw Error("Drawer did not close")
  }
  await page.emulateMedia({reducedMotion:"no-preference"})
  return {syntheticOnly:true,method:"Real Show Sidebar click with rAF bounds/transform and current motion media; Escape closes; no provider action",checks,cases}
}
