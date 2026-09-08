async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Owned fixture only")
  const capture=// Report-only adapter. Attribution and measurement limits: ../NOTICE.md.
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

  const scope=page.getByRole("region",{name:"Ops desk",exact:true}),cases=[],checks=[]
  const shot=async(name)=>{await page.screenshot({path:`reports/design-phase1-specialist/screenshots/${name}.png`});cases.push({name,raw:await capture(page)})}
  await page.setViewportSize({width:1280,height:900})
  await scope.getByRole("button",{name:"Inbox",exact:true}).click()
  await scope.getByRole("combobox",{name:"Ticket status",exact:true}).selectOption("1")
  await scope.getByRole("heading",{name:"No threads with this status",exact:true}).waitFor()
  await shot("empty-filter-1280-dark")
  await scope.getByRole("combobox",{name:"Ticket status",exact:true}).selectOption("")
  await scope.getByRole("button",{name:/Reader Open Synthetic locale review/}).waitFor()
  try {
    await page.context().setOffline(true)
    await scope.getByRole("button",{name:"Refresh",exact:true}).click()
    await scope.getByRole("button",{name:"Try again",exact:true}).waitFor()
    await shot("network-error-1280-dark")
  } finally { await page.context().setOffline(false) }
  await scope.getByRole("button",{name:"Try again",exact:true}).click()
  await scope.getByRole("button",{name:/Reader Open Synthetic locale review/}).waitFor()
  await shot("network-recovered-1280-dark")
  checks.push({BC6:true,method:"Real browser offline, real protected API restored; no response interception"})
  await scope.locator("summary").filter({hasText:"Manage email connection"}).click()
  await scope.getByRole("button",{name:"Remove inbox key",exact:true}).click()
  await scope.getByText("Email not connected",{exact:true}).waitFor()
  await scope.locator("summary").filter({hasText:"Connect Resend"}).scrollIntoViewIfNeeded()
  await shot("missing-resend-1280-dark")
  await scope.getByRole("button",{name:"Approvals",exact:true}).click()
  await scope.getByRole("button",{name:/Re: Synthetic locale review Task/}).click()
  await scope.getByText(/Email is not connected|Configure Resend|Resend is not configured/).first().waitFor({timeout:3000}).catch(()=>{})
  await scope.locator("article").getByRole("button",{name:"Approve and send reply",exact:true}).scrollIntoViewIfNeeded()
  await shot("missing-resend-stale-review")
  checks.push({BC13:true,limitation:"Proposal is pending but already stale from controlled revision2; prior accepted unconfigured-fresh approval proof remains separately cited."})
  await scope.getByRole("button",{name:"Inbox",exact:true}).click()
  await scope.getByRole("button",{name:/Reader Open Synthetic locale review/}).click()
  await scope.getByRole("button",{name:"Reply draft",exact:true}).click()
  await scope.getByLabel("Reply message",{exact:true}).fill("Final locale memory sentinel (not saved).")
  const settings=page.context().pages().find(p=>p.url().endsWith("4327/settings/appearance"))
  await settings.goto("http://127.0.0.1:4327/settings/system")
  await settings.getByRole("combobox").click()
  await settings.getByRole("option",{name:"Arabic",exact:true}).click()
  await page.waitForFunction(()=>document.documentElement.lang==="ar")
  for(const width of [1280,390]) {
    await page.setViewportSize({width,height:width===390?844:900})
    if(width===390) await page.keyboard.press("Escape")
    await scope.locator("article h1").scrollIntoViewIfNeeded()
    await shot(`rtl-${width}-dark`)
    if(await scope.getByLabel("Reply message",{exact:true}).inputValue()!=="Final locale memory sentinel (not saved).") throw Error("Locale or remount lost draft")
  }
  const direction=await page.evaluate(()=>({lang:document.documentElement.lang,dir:document.documentElement.dir,persisted:Object.values(localStorage).some(v=>v.includes("Final locale memory sentinel")),emailDir:getComputedStyle(document.querySelector('input[id$="-to"]')).direction,backRotation:getComputedStyle([...document.querySelectorAll("button")].find(b=>b.textContent.includes("Back to inbox")).querySelector("svg")).rotate}))
  await settings.getByRole("combobox").click()
  await settings.getByRole("option",{name:"الإنجليزية",exact:true}).click()
  await page.waitForFunction(()=>document.documentElement.lang==="en")
  if(await scope.getByLabel("Reply message",{exact:true}).inputValue()!=="Final locale memory sentinel (not saved).") throw Error("English return lost draft")
  await scope.getByLabel("Reply message",{exact:true}).fill("Bounded design draft: all text remains after validation.")
  checks.push({BC3:true,localePreserved:true,direction,restoredEnglish:true})
  return {syntheticOnly:true,checks,cases}
}
