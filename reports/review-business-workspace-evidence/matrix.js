// Sequential actual CLI captures; accepted probe adapter attribution is in NOTICE.md.
async page => {
  const capture = (// Report-only adapter. Attribution and measurement limits: ../NOTICE.md.
async (page) => {
  const raw = await page.evaluate(() => {
    const canvas = document.createElement("canvas")
    canvas.width = canvas.height = 1
    const ctx = canvas.getContext("2d", { willReadFrequently: true })
    const rgba = (css) => {
      if (!CSS.supports("color", css)) throw new Error(`Unsupported color: ${css}`)
      ctx.clearRect(0, 0, 1, 1)
      ctx.fillStyle = css
      ctx.fillRect(0, 0, 1, 1)
      const p = Array.from(ctx.getImageData(0, 0, 1, 1).data)
      return [...p.slice(0, 3), p[3] / 255]
    }
    const blend = (front, back) => front.slice(0, 3).map((v, i) => v * front[3] + back[i] * (1 - front[3]))
    const rgb = (v) => `rgb(${v.slice(0, 3).map(Math.round).join(", ")})`
    const box = (el) => {
      const r = el.getBoundingClientRect()
      return { x: r.x, y: r.y, width: r.width, height: r.height }
    }
    const painted = (el) => el.checkVisibility({ opacityProperty: true, visibilityProperty: true, contentVisibilityAuto: true })
    const inViewport = (el) => {
      const r = el.getBoundingClientRect()
      return painted(el) && r.width > 0 && r.height > 0 && r.right > 0 && r.bottom > 0 && r.left < innerWidth && r.top < innerHeight
    }
    const colors = new Set(), fontFamilies = new Set(), fontSizesPx = new Set(), spacingPx = new Set(), radiiPx = new Set(), easings = new Set()
    const samples = [], exclusions = [], transitions = [], colorEvidence = []
    const els = [...document.querySelectorAll("body *")]
    const describe = (el) => ({ tag: el.tagName, id: el.id, class: typeof el.className === "string" ? el.className : "", label: el.getAttribute("aria-label") })
    const fields = []
    for (const el of els) {
      const s = getComputedStyle(el)
      const duration = s.transitionDuration.split(",").some((n) => parseFloat(n) > 0)
      if (duration) transitions.push({ ...describe(el), visible: painted(el), inViewport: inViewport(el), properties: s.transitionProperty, duration: s.transitionDuration, timing: s.transitionTimingFunction, transform: s.transform, translate: s.translate, scale: s.scale, rotate: s.rotate, animation: s.animationName, scrollBehavior: s.scrollBehavior })
      if (["INPUT", "TEXTAREA", "SELECT"].includes(el.tagName)) fields.push({ ...describe(el), valuePresent: !!el.value, selectedText: el.tagName === "SELECT" ? [...el.selectedOptions].map((o) => o.textContent.trim()).join("; ") : null, painted: painted(el), inViewport: inViewport(el), closedDetails: !!el.closest("details:not([open])"), disabled: el.matches(":disabled"), type: el.type, box: box(el), labels: [...(el.labels || [])].map((l) => l.textContent.trim()) })
      if (!inViewport(el) || ["SCRIPT", "STYLE", "OPTION", "SVG", "PATH"].includes(el.tagName)) continue
      const chain = []
      for (let node = el; node; node = node.parentElement) {
        const cs = getComputedStyle(node)
        chain.unshift({ ...describe(node), bg: cs.backgroundColor, opacity: cs.opacity, image: cs.backgroundImage, filter: cs.filter, blend: cs.mixBlendMode })
      }
      let bg = [255, 255, 255]
      for (const layer of chain) bg = blend(rgba(layer.bg), bg)
      let text = [...el.childNodes].filter((n) => n.nodeType === Node.TEXT_NODE).map((n) => n.textContent).join(" ").trim()
      let kind = "text", ink = s.color
      if (el.tagName === "SELECT") {
        // An empty value still paints its selected option, e.g. All statuses.
        text = [...el.selectedOptions].map((o) => o.textContent.trim()).join("; ")
        kind = "selected-option"
      } else if (["INPUT", "TEXTAREA"].includes(el.tagName) && !["checkbox", "radio", "hidden", "range", "color"].includes(el.type)) {
        kind = el.value ? "value" : "placeholder"
        text = el.type === "password" ? (el.value ? "[masked credential]" : el.placeholder) : el.value || el.placeholder || ""
        const style = kind === "placeholder" ? getComputedStyle(el, "::placeholder") : s
        ink = style.webkitTextFillColor && style.webkitTextFillColor !== "currentcolor" ? style.webkitTextFillColor : style.color
      }
      if (text) {
        fontFamilies.add(s.fontFamily)
        const fg = blend(rgba(ink), bg)
        const measurable = chain.every((c) => c.opacity === "1" && c.image === "none" && c.filter === "none" && c.blend === "normal")
        const disabled = !!el.closest(":disabled,[aria-disabled=true]")
        const sample = { ...describe(el), kind, text: text.slice(0, 450), cssInk: ink, fg: rgb(fg), bg: rgb(bg), sizePx: parseFloat(s.fontSize), bold: parseInt(s.fontWeight, 10) >= 700, box: box(el), measurable, disabled, direction: s.direction, chain }
        samples.push(sample)
        if (!measurable || disabled) exclusions.push({ text: sample.text, kind, reason: disabled ? "disabled control; recorded but not contrast-ranked" : "group opacity/image/filter; needs pixel inspection" })
        colors.add(rgb(fg))
        colorEvidence.push({ css: ink, composed: rgb(fg), role: "foreground", text: sample.text, class: sample.class, background: rgb(bg), alpha: rgba(ink)[3] })
      }
      if (rgba(s.backgroundColor)[3] > 0) {
        colors.add(rgb(bg))
        colorEvidence.push({ css: s.backgroundColor, composed: rgb(bg), role: "background", class: describe(el).class, alpha: rgba(s.backgroundColor)[3] })
      }
      if (el.hasAttribute("class") || el.hasAttribute("style") || ["BUTTON", "INPUT", "SELECT", "TEXTAREA", "A"].includes(el.tagName)) {
        fontSizesPx.add(parseFloat(s.fontSize))
        for (const p of ["marginTop", "marginBottom", "paddingTop", "paddingBottom", "gap", "columnGap", "rowGap"]) if (parseFloat(s[p]) > 0) spacingPx.add(Math.round(parseFloat(s[p])))
        if (parseFloat(s.borderTopLeftRadius) > 0) radiiPx.add(Math.round(parseFloat(s.borderTopLeftRadius)))
      }
      if (duration) easings.add(s.transitionTimingFunction)
    }
    const rules = (list) => [...list].some((r) => (r.media && /prefers-reduced-motion/.test(r.media.mediaText)) || (r.cssRules && rules(r.cssRules)))
    const reducedMotionHandled = [...document.styleSheets].some((ss) => { try { return rules(ss.cssRules) } catch { return false } })
    const activeAnimations = document.getAnimations().map((a) => ({ state: a.playState, type: a.constructor.name, target: a.effect?.target ? describe(a.effect.target) : null, keyframes: a.effect?.getKeyframes?.(), timing: a.effect?.getComputedTiming?.() }))
    const root = getComputedStyle(document.documentElement)
    const tokenNames = ["background", "foreground", "card", "card-foreground", "primary", "primary-foreground", "secondary", "secondary-foreground", "muted", "muted-foreground", "accent", "accent-foreground", "destructive", "border", "input", "ring", "sidebar", "sidebar-foreground", "sidebar-accent", "sidebar-accent-foreground"]
    const tokens = tokenNames.map((name) => { const css = root.getPropertyValue(`--${name}`).trim(); return { name: `--${name}`, css, rgba: rgba(css) } })
    const headings = [...document.querySelectorAll("h1,h2,h3,h4,h5,h6")].filter(painted).map((e) => ({ level: +e.tagName[1], text: e.textContent.trim() }))
    const focus = document.activeElement
    const fs = getComputedStyle(focus)
    return {
      capturedAt: new Date().toISOString(), url: location.href, viewport: { width: innerWidth, height: innerHeight }, documentWidth: document.documentElement.scrollWidth,
      theme: { class: document.documentElement.className, preset: document.documentElement.dataset.theme, direction: root.direction, scheme: root.colorScheme }, reducedMotion: matchMedia("(prefers-reduced-motion: reduce)").matches,
      colors: [...colors], fontFamilies: [...fontFamilies], fontSizesPx: [...fontSizesPx], spacingPx: [...spacingPx], radiiPx: [...radiiPx],
      textSamples: samples.filter((s) => s.measurable && !s.disabled), samples, exclusions, fields, colorEvidence, tokens,
      transitionsCount: transitions.length, visibleTransitionCount: transitions.filter((t) => t.visible).length, transitions, easings: [...easings], reducedMotionHandled, activeAnimations,
      focus: { ...describe(focus), text: focus.textContent.slice(0, 120), box: box(focus), outline: fs.outline, shadow: fs.boxShadow, focusVisible: focus.matches(":focus-visible") },
      aria: { headings, headingOrderBreaks: headings.filter((h, i) => i > 0 && h.level > headings[i - 1].level + 1).length, landmarks: { main: !!document.querySelector("main,[role=main]"), nav: !!document.querySelector("nav,[role=navigation]") } }
    }
  })
  raw.aria.snapshot = await page.locator("body").ariaSnapshot()
  return raw
}
)
  const originalBrief = {"version": 1, "mode": "scan", "stack": {"framework": "Next.js 16 static export / React 19", "styling": "Tailwind v4 / inherited shadcn UI"}, "tokens": {"colors": {"--background": "oklch(1 0 0)", "--foreground": "oklch(0.145 0 0)", "--card": "oklch(1 0 0)", "--card-foreground": "oklch(0.145 0 0)", "--popover": "oklch(1 0 0)", "--popover-foreground": "oklch(0.145 0 0)", "--primary": "#245e58", "--primary-foreground": "oklch(0.985 0 0)", "--secondary": "oklch(0.97 0 0)", "--secondary-foreground": "oklch(0.205 0 0)", "--muted": "oklch(0.97 0 0)", "--muted-foreground": "oklch(0.556 0 0)", "--accent": "oklch(0.97 0 0)", "--accent-foreground": "oklch(0.205 0 0)", "--destructive": "oklch(0.58 0.22 27)", "--border": "oklch(0.922 0 0)", "--input": "oklch(0.922 0 0)", "--ring": "#245e58", "--chart-1": "oklch(0.809 0.105 251.813)", "--chart-2": "oklch(0.623 0.214 259.815)", "--chart-3": "oklch(0.546 0.245 262.881)", "--chart-4": "oklch(0.488 0.243 264.376)", "--chart-5": "oklch(0.424 0.199 265.638)", "--sidebar": "oklch(0.985 0 0)", "--sidebar-foreground": "oklch(0.145 0 0)", "--sidebar-primary": "#245e58", "--sidebar-primary-foreground": "oklch(0.985 0 0)", "--sidebar-accent": "oklch(0.94 0 0)", "--sidebar-accent-foreground": "oklch(0.205 0 0)", "--sidebar-border": "oklch(0.922 0 0)", "--sidebar-ring": "#245e58"}, "type": {"families": ["\"Inter Variable\", system-ui, sans-serif"], "sizesPx": [12, 14, 16, 20, 24]}, "spacingPx": [4, 8, 12, 16, 24, 32], "radiiPx": [6, 8, 10, 14]}, "surfaces": [{"name": "login-page", "components": ["src/app/login/page.tsx"], "applies": true}, {"name": "navigation", "components": ["src/components/layout/sidebar.tsx"], "applies": true}, {"name": "task-board", "components": ["src/components/tasks/task-card.tsx"], "applies": true}, {"name": "input-field", "components": ["src/components/ui/input.tsx"], "applies": true}, {"name": "tabs", "components": ["src/components/ui/tabs.tsx"], "applies": true}, {"name": "ops-workspace", "components": ["src/components/ops/ops-page.tsx", "src/components/ops/morning-view.tsx", "src/components/ops/inbox-view.tsx", "src/components/ops/proposals-view.tsx"], "applies": true}, {"name": "evidence-review", "components": ["src/components/ops-intake/bug-workflow-page.tsx", "src/components/ops-intake/issue-review.tsx", "src/components/ops-telegram/issue-review.tsx"], "applies": true}], "conventions": ["Hafidh Ops Desk serves the team handling tester bugs and support email. Source of scope: FOUNDING.md P1/P2; this brief does not authorize later social/release phases.", "Inherit codeg components, static query-parameter navigation, Inter UI font and Hafidh primary accent. Keep existing user font/theme choices. Tokens shown are the default neutral light block in src/app/globals.css; dark uses its paired neutral.dark block with primary #9bd4c5. Do not flatten those modes.", "The source scanner was narrowed to src to exclude exported and vendored CSS. Its theme-preset literals and filename-based surface guesses are not confirmed defects; this curated brief scopes the Ops work and records actual component paths.", "Compact operational layout: inbox list, thread, and reply/approval detail; reflow to one readable pane on narrow screens. Distinguish selected rows and unread state without color alone.", "Private note, draft awaiting approval, approved authorization, sent receipt and delivery failure must be visibly distinct. Approval must present exact recipients and complete content; edits require revalidation.", "No decorative metrics or invented customer data in production. Loading, empty, missing configuration, validation error, denied, stale approval and success states must provide a concrete next action.", "Use semantic labels, keyboard navigation, visible focus, dialogs with focus restoration, accessible errors and sufficient contrast. Validate at desktop and narrow mobile widths in light/dark; respect RTL locale direction.", "Use Playwright CLI for real-server browser checks. Mock external providers only in explicitly identified test fixtures. Run Design Studio measured audit, worker fixes and targeted rechecks at completion.", "Owner-authorized visual refresh: follow docs/design/VISUAL-DIRECTION.md. Dispatch-desk composition, clear work hierarchy and correspondence-to-decision sheet; preserve existing themes/fonts and all approved workflow semantics."], "scenarios": [{"id": "login", "task": "Connect to the isolated local test server", "assertions": ["Invalid token shows an accessible error", "Valid test token opens workspace", "No horizontal overflow at 390px"]}, {"id": "support-reply", "task": "Open a ticket, draft and review a reply", "assertions": ["Private note cannot appear as public reply", "Recipients and text are reviewed together", "Nothing sends before explicit approval", "A stale approval cannot send a changed draft"]}, {"id": "bug-evidence", "task": "Prepare a Hafidh bug for GitHub filing", "assertions": ["Missing build/screen/reciter/log blocks filing", "Suggested severity remains a human-reviewed guess", "External issue creation requires the approval seam"]}], "gaps": [{"priority": 1, "current": "Owner finds the accepted functional UI visually basic: empty Morning is a long text column; workspace hierarchy and list/detail finish need development.", "target": "Coherent dispatch-desk visual refresh with before/after evidence across populated and empty states."}, {"priority": 2, "current": "New visual refresh has not been implemented or audited.", "target": "Final Design Studio aesthetic/a11y/flow review, root CLI checks and refreshed app."}], "inspiration": []}
  const normalization = await page.evaluate(original => {
    const derived=structuredClone(original), conversions=[]
    const canvas=document.createElement('canvas'); canvas.width=canvas.height=1
    const ctx=canvas.getContext('2d',{willReadFrequently:true})
    for(const [name,css] of Object.entries(original.tokens.colors)) {
      if(name==='rogue') continue
      if(!CSS.supports('color',css)) throw Error('Unsupported brief color '+name)
      ctx.clearRect(0,0,1,1);ctx.fillStyle=css;ctx.fillRect(0,0,1,1)
      const rgba=[...ctx.getImageData(0,0,1,1).data]
      if(rgba[3]!==255) throw Error('Alpha brief requires backdrop evidence')
      const hex='#'+rgba.slice(0,3).map(v=>v.toString(16).padStart(2,'0')).join('')
      conversions.push({name,css,rgba,hex}); derived.tokens.colors[name]=hex
    }
    return {label:'Derived measurement copy only; original BRIEF unchanged. Dark/alpha/semantic colors classified separately.',derived,conversions}
  },originalBrief)
  const results=[]
  for(const locale of ['en','ar']) for(const theme of ['light','dark']) {
    await page.setViewportSize({width:1280,height:900})
    await page.getByRole('combobox',{name:/^(Language|اللغة)$/}).selectOption(locale)
    await page.getByRole('combobox',{name:/^(Appearance|المظهر)$/}).selectOption(theme)
    await page.getByRole('heading',{name:locale==='en'?'My work':'عملي',exact:true}).waitFor()
    for(const width of [1280,768,390]) {
      await page.setViewportSize({width,height:900})
      await page.evaluate(async()=>{await document.fonts.ready; await new Promise(requestAnimationFrame);await new Promise(requestAnimationFrame)})
      const name='workspace-'+width+'-'+theme+'-'+locale
      await page.screenshot({path:'reports/review-business-workspace-evidence/'+name+'.png'})
      const raw=await capture(page)
      results.push({name,raw})
    }
  }
  await page.setViewportSize({width:1280,height:900})
  await page.getByRole('combobox',{name:'اللغة',exact:true}).selectOption('en')
  await page.getByRole('combobox',{name:'Appearance',exact:true}).selectOption('light')
  return {normalization,results,ledger:page.__businessReview}
}
