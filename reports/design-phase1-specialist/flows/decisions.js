async (page) => {
  if (!page.url().startsWith("http://127.0.0.1:4327/")) throw Error("Owned fixture only")
  const capture = // Report-only adapter. Attribution and measurement limits: ../NOTICE.md.
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

  const scope = page.getByRole("region", { name: "Ops desk", exact: true })
  const cases = [], checks = []
  const stats = () => page.evaluate(async () => (await fetch("/api/ops_design_fixture_stats", { headers: { Authorization: "Bearer ops-design-synthetic-operator" } })).json())
  const before = await stats()
  const shot = async (name) => { await page.screenshot({ path: `reports/design-phase1-specialist/screenshots/${name}.png` }); cases.push({ name, raw: await capture(page) }) }
  for (const [id, phrase] of [[2,"it does not send again"],[3,"Provider acceptance does not confirm recipient delivery"],[4,"Reconcile the provider outcome first"],[5,"The provider did not accept this reply"],[6,"Its private reply content has been redacted"]]) {
    await page.setViewportSize({ width:1280,height:900 })
    await scope.getByRole("button",{name:new RegExp(`Reply review #${id} Task`)}).click()
    await scope.locator("article h1").waitFor()
    const copy = await scope.locator("article").innerText()
    if (!copy.includes(phrase) || copy.includes("No delivery receipt is implied")) throw Error(`Terminal copy mismatch ${id}`)
    if (await scope.getByRole("button",{name:/Approve and send|Deny proposal/}).count()) throw Error("Terminal dispatch available")
    await page.setViewportSize({ width:390,height:844 })
    await page.keyboard.press("Escape")
    await scope.locator("article h1").scrollIntoViewIfNeeded()
    await shot(`terminal-${id}-390-dark`)
    checks.push({ id, copy, offersAnotherSend:false })
  }
  await page.setViewportSize({ width:1280,height:900 })
  await scope.getByRole("button",{name:/Reply review #2 Task/}).click()
  await scope.getByRole("button",{name:"Finish recording receipt",exact:true}).click()
  await scope.getByRole("heading",{name:"Sent · provider accepted",exact:true}).waitFor()
  await shot("receipt-finished-1280-dark")
  if (await scope.getByRole("button",{name:"Finish recording receipt",exact:true}).count()) throw Error("Recording action remains")
  const after = await stats()
  if (after.providerRequests !== before.providerRequests) throw Error("Recording resent")
  checks.push({ BC14:true, providerBefore:before.providerRequests, providerAfter:after.providerRequests })
  return { syntheticOnly:true, checks, cases }
}
