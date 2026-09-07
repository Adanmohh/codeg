async (page) => page.evaluate(() => {
  // Targeted computed-style evidence, not a whole-page WCAG certification.
  // Convert browser-supported OKLCH/color-mix to sRGB before Design Studio's
  // contrastRatio helper. Only rank samples without group opacity or images.
  const canvas = document.createElement("canvas")
  canvas.width = canvas.height = 1
  const context = canvas.getContext("2d", { willReadFrequently: true })
  const rgba = (css) => {
    context.clearRect(0, 0, 1, 1)
    context.fillStyle = css
    context.fillRect(0, 0, 1, 1)
    const p = Array.from(context.getImageData(0, 0, 1, 1).data)
    return [p[0], p[1], p[2], p[3] / 255]
  }
  const blend = (front, back) => front.slice(0, 3).map((v, i) => v * front[3] + back[i] * (1 - front[3]))
  const visible = (el) => {
    const r = el.getBoundingClientRect()
    const s = getComputedStyle(el)
    return r.width > 0 && r.height > 0 && r.right > 0 && r.bottom > 0 && r.left < innerWidth && r.top < innerHeight && s.visibility === "visible" && s.display !== "none"
  }
  const box = (el) => {
    const r = el.getBoundingClientRect()
    return { x: r.x, y: r.y, width: r.width, height: r.height }
  }
  const samples = [...document.querySelectorAll("body *")].filter((el) => visible(el) && [...el.childNodes].some((n) => n.nodeType === Node.TEXT_NODE && n.textContent.trim().length > 2)).map((el) => {
    const s = getComputedStyle(el)
    const chain = []
    for (let node = el; node; node = node.parentElement) {
      const style = getComputedStyle(node)
      chain.unshift({ bg: style.backgroundColor, opacity: style.opacity, image: style.backgroundImage })
    }
    let bg = [255, 255, 255]
    for (const layer of chain) bg = blend(rgba(layer.bg), bg)
    const fg = blend(rgba(s.color), bg)
    return {
      text: el.textContent.trim().slice(0, 450), tag: el.tagName, class: el.className,
      color: s.color, fg, bg, sizePx: parseFloat(s.fontSize), weight: s.fontWeight,
      measurable: chain.every((c) => c.opacity === "1" && c.image === "none"),
      box: box(el), scrollWidth: el.scrollWidth, clientWidth: el.clientWidth,
      whiteSpace: s.whiteSpace, textOverflow: s.textOverflow,
      title: el.getAttribute("title"), role: el.getAttribute("role"), live: el.getAttribute("aria-live")
    }
  })
  return {
    url: location.pathname + location.search, viewport: { width: innerWidth, height: innerHeight },
    documentWidth: document.documentElement.scrollWidth, theme: document.documentElement.className,
    samples,
    unnamedButtons: [...document.querySelectorAll("button")].filter((el) => visible(el) && !el.textContent.trim() && !el.getAttribute("aria-label") && !el.getAttribute("aria-labelledby") && !el.title).map((el) => ({ html: el.outerHTML.slice(0, 1200), box: box(el) })),
    fields: [...document.querySelectorAll("input,[role=combobox]")].filter(visible).map((el) => ({
      tag: el.tagName, type: el.getAttribute("type"), placeholder: el.getAttribute("placeholder"),
      labels: [...(el.labels || [])].map((l) => l.textContent.trim()),
      ariaLabel: el.getAttribute("aria-label"), labelledBy: el.getAttribute("aria-labelledby"),
      describedBy: el.getAttribute("aria-describedby"), box: box(el)
    })),
    focus: { tag: document.activeElement.tagName, text: document.activeElement.textContent.slice(0, 450), outline: getComputedStyle(document.activeElement).outline, shadow: getComputedStyle(document.activeElement).boxShadow }
  }
})
