// Run via Playwright CLI on the original local BRIEF.html file.
async (page) => page.evaluate(() => {
  const original = JSON.parse(document.querySelector("#design-state").textContent)
  const derived = structuredClone(original)
  const canvas = document.createElement("canvas")
  canvas.width = canvas.height = 1
  const ctx = canvas.getContext("2d", { willReadFrequently: true })
  const conversions = []
  for (const [name, css] of Object.entries(original.tokens.colors)) {
    if (name === "rogue") continue
    if (!CSS.supports("color", css)) throw new Error(`Unsupported token ${name}`)
    ctx.clearRect(0, 0, 1, 1)
    ctx.fillStyle = css
    ctx.fillRect(0, 0, 1, 1)
    const bytes = [...ctx.getImageData(0, 0, 1, 1).data]
    if (bytes[3] !== 255) throw new Error(`Alpha token needs separate backdrop evidence: ${name}`)
    const hex = "#" + bytes.slice(0, 3).map((v) => v.toString(16).padStart(2, "0")).join("")
    conversions.push({ name, original: css, hex, rgba: bytes })
    derived.tokens.colors[name] = hex
  }
  derived.conventions.push("DERIVED MEASUREMENT COPY ONLY: original BRIEF.html is unchanged; Canvas-normalized opaque light tokens. Paired-dark tokens and alpha composites are classified separately, not silently added to this palette.")
  return { capturedAt: new Date().toISOString(), browser: navigator.userAgent, sourceUrl: location.href, purpose: "Derived measurement copy, not a new product brief", conversions, derived }
})
