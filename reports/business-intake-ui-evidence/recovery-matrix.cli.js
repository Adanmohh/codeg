// Root-provided read-only CLI Canvas probe composed with owned UI checks.
// Installed CLI0.1.18 / Playwright1.63.0-alpha-2026-08-05.
async page => {
const probe = (async page => {
    const raw = await page.evaluate(() => {
      const colors = new Set(), fontFamilies = new Set(), fontSizesPx = new Set(),
        spacingPx = new Set(), radiiPx = new Set(), easings = new Set();
      let transitionsCount = 0;
      const textSamples = [];
      const els = [...document.querySelectorAll("*")].slice(0, 3000);
      const px = (v) => { const n = parseFloat(v); return Number.isFinite(n) && n > 0 ? Math.round(n) : null; };
      const canvas=document.createElement('canvas');canvas.width=1;canvas.height=1;
      const cx=canvas.getContext('2d',{willReadFrequently:true});
      const rgba=v=>{cx.clearRect(0,0,1,1);cx.fillStyle=v;cx.fillRect(0,0,1,1);return [...cx.getImageData(0,0,1,1).data];};
      const background=el=>{const layers=[];for(let n=el;n;n=n.parentElement) layers.push(rgba(getComputedStyle(n).backgroundColor));let rgb=[255,255,255];for(const a of layers.reverse())rgb=rgb.map((v,i)=>Math.round(a[i]*(a[3]/255)+v*(1-a[3]/255)));return `rgb(${rgb.join(', ')})`;};
      for (const el of els) {
        if(!el.getClientRects().length || getComputedStyle(el).visibility==='hidden')continue;
        if (el === document.documentElement) continue; // UA defaults (#000 color, Times) are never authored signal
        if (["STYLE", "SCRIPT", "TEMPLATE", "HEAD", "TITLE", "META", "LINK", "NOSCRIPT"].includes(el.tagName)) continue;
        const cs = getComputedStyle(el);
        // direct text node → this element's color/font are actually painted as text
        const hasText = [...el.childNodes].some((n) => n.nodeType === 3 && n.textContent.trim().length > 2);
        // Authored-signal heuristic: UA default styles (heading margins, form-control fonts, root
        // color) must not enter the inventory as if the author chose them. Text color/font-family
        // are only sampled where text is actually painted (hasText); box metrics (spacing/radii/
        // font sizes) only from elements bearing a plausible authoring signal — a class or style
        // attribute, or an intrinsically styled control tag. Painted backgrounds are real
        // regardless (UA default is transparent), so those are sampled from any rendered element.
        const authored = el.hasAttribute("class") || el.hasAttribute("style") ||
          ["BUTTON", "INPUT", "SELECT", "TEXTAREA", "A"].includes(el.tagName);
        if (hasText) {
          if (cs.color) colors.add(cs.color);
          fontFamilies.add(cs.fontFamily);
        }
        if (cs.backgroundColor && cs.backgroundColor !== "rgba(0, 0, 0, 0)") colors.add(cs.backgroundColor);
        if (authored) {
          const fspx = px(cs.fontSize); if (fspx) fontSizesPx.add(fspx);
          for (const p of ["marginTop", "marginBottom", "paddingTop", "paddingBottom", "gap", "columnGap", "rowGap"]) {
            const v = px(cs[p]); if (v) spacingPx.add(v);
          }
          const r = px(cs.borderTopLeftRadius); if (r) radiiPx.add(r);
        }
        if (parseFloat(cs.transitionDuration) > 0) { transitionsCount++; easings.add(cs.transitionTimingFunction); }
        // direct text node → contrast sample (walk up for effective bg)
        if (hasText && textSamples.length < 200) {
          const bg = background(el);
          textSamples.push({ text: (el.innerText || "").trim().slice(0, 40), fg: cs.color, bg,
            sizePx: parseFloat(cs.fontSize), bold: parseInt(cs.fontWeight, 10) >= 700 });
        }
      }
      const reducedMotionHandled = [...document.styleSheets].some((ss) => {
        try { return [...ss.cssRules].some((r) => r.media && /prefers-reduced-motion/.test(r.media.mediaText)); }
        catch { return false; }
      });
      return { colors: [...colors], fontFamilies: [...fontFamilies], fontSizesPx: [...fontSizesPx].sort((a, b) => a - b),
        spacingPx: [...spacingPx].sort((a, b) => a - b), radiiPx: [...radiiPx].sort((a, b) => a - b),
        textSamples, transitionsCount, easings: [...easings], reducedMotionHandled };
    });

const normalized = await page.evaluate(raw => {
 const c = document.createElement('canvas'); c.width=1;c.height=1;
 const ctx=c.getContext('2d',{willReadFrequently:true});
 const color = value => {ctx.clearRect(0,0,1,1);ctx.fillStyle=value;ctx.fillRect(0,0,1,1); const a=ctx.getImageData(0,0,1,1).data; return `rgba(${a[0]}, ${a[1]}, ${a[2]}, ${a[3]/255})`;};
 return {colors:raw.colors.map(color),textSamples:raw.textSamples.map(v=>({...v,fg:color(v.fg),bg:color(v.bg)}))};
},raw);
Object.assign(raw,normalized);
raw.normalization='Browser canvas sRGB conversion and ancestor background compositing; hidden nodes omitted; gradients, opacity and disabled controls require manual review';
const snapshot = await page.locator('body').ariaSnapshot();
raw.aria = {snapshot, unnamedInteractive:null, headingOrderBreaks:null, landmarks:null};
raw.reducedMotionEffective = null;
raw.jank = {cls:null,longFrames:null};
raw.context = await page.evaluate(() => ({url:location.href,viewport:innerWidth,scrollWidth:document.documentElement.scrollWidth,dark:document.documentElement.classList.contains('dark')}));
return raw;
});


  const cases=[];
  const title='Synthetic local title retained through source refresh';
  const brief='Synthetic private draft survives successful fresh access, locale and panes. Not submitted.';
  for(const locale of ['en','ar']) for(const theme of ['light','dark']) for(const width of [1280,768,390]){
    await page.setViewportSize({width:1280,height:900});
    await page.getByRole('combobox',{name:/^(Language|اللغة)$/}).selectOption(locale);
    await page.getByRole('combobox',{name:/^(Appearance|المظهر)$/}).selectOption(theme);
    await page.setViewportSize({width,height:900});
    const titleInput=page.getByLabel(locale==='ar'?'عنوان المهمة':'Task title',{exact:true});
    const text=page.getByLabel(locale==='ar'?'وصف العمل':'Brief',{exact:true});
    if(await titleInput.inputValue()!==title || await text.inputValue()!==brief)throw Error('Draft lost in measured matrix');
    const tab=page.getByRole('tab',{name:locale==='ar'?'المصادر':'Sources',exact:true});
    await tab.focus();
    await page.keyboard.press(locale==='ar'?'ArrowLeft':'ArrowRight');
    if(await tab.getAttribute('aria-selected')!=='true')throw Error('Arrow focus unexpectedly activated another surface');
    await tab.focus();
    const sourcePanel=page.getByRole('tabpanel',{name:locale==='ar'?'المصادر':'Sources',exact:true});
    const back=sourcePanel.getByRole('button',{name:locale==='ar'?'المصادر':'Sources',exact:true});
    await back.focus();
    await page.waitForTimeout(500);
    const raw=await probe(page);
    const measurement=await page.evaluate(()=>{
      const focused=document.activeElement,box=focused.getBoundingClientRect(),cs=getComputedStyle(focused);
      const root=document.documentElement;
      return {width:innerWidth,scrollWidth:root.scrollWidth,dir:root.dir,focus:{name:focused.textContent,width:box.width,height:box.height,shadow:cs.boxShadow,outline:cs.outline,visibility:cs.visibility},sourceFont:[...document.querySelectorAll('section[aria-label] li')].filter(el=>el.getClientRects().length).map(el=>({font:getComputedStyle(el).fontSize,width:el.getBoundingClientRect().width})),privateStored:JSON.stringify(localStorage).includes('Synthetic private draft'),date:document.querySelector('input[type=date]')?.value,backArrow:focused.querySelector('svg')?getComputedStyle(focused.querySelector('svg')).rotate:null};
    });
    if(measurement.scrollWidth>width||measurement.privateStored||measurement.date!=='2026-11-03'||measurement.focus.width<40||measurement.focus.height<40||measurement.focus.shadow==='none')throw Error('Geometry/focus/privacy measurement failed');
    const name='recovery-'+width+'-'+theme+'-'+locale;
    await page.screenshot({path:'reports/business-intake-ui-evidence/'+name+'.png'});
    cases.push({name,locale,theme,width,draftRetained:true,manualTabFocus:true,measurement,raw});
  }
  await page.emulateMedia({reducedMotion:'reduce'});
  const reduced=await page.evaluate(()=>({enabled:matchMedia('(prefers-reduced-motion: reduce)').matches,overflow:document.documentElement.scrollWidth>innerWidth,taskInputs:[...document.querySelectorAll('input')].filter(el=>el.getClientRects().length&&el.value.startsWith('Synthetic local title')).length}));
  await page.screenshot({path:'reports/business-intake-ui-evidence/recovery-390-dark-ar-reduced.png'});
  await page.emulateMedia({reducedMotion:null});
  await page.setViewportSize({width:1280,height:900});
  await page.getByRole('combobox',{name:'اللغة',exact:true}).selectOption('en');
  await page.getByRole('combobox',{name:'Appearance',exact:true}).selectOption('light');
  return {source:'60d600db0f224d44ff191490ab79bc25530ba959',cases,reduced,scope:'synthetic ui source/private draft; no saved writes, source refresh or provider requests inside this matrix'};
}
