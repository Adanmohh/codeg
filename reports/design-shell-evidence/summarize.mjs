// Report-only glue. Calls the already-installed Design Studio pure helper;
// does not launch its browser, flow executor or a model.
import { readFileSync, writeFileSync } from "node:fs"
import { contrastRatio } from "/Users/mohamedadan/projects/design-studio/lab/tools/probe.mjs"

const root = new URL("./", import.meta.url)
const files = [
  "01-before-light.json",
  "02-before-dark.json",
  "05-after-light.json",
  "12-after-dark.json",
]
const rgb = ([r, g, b]) => ({ r, g, b })
const exact = new Set([
  "Folders",
  "Chat",
  "Recent",
  "fresh-desk-workspace",
  "No chats",
  "No conversations",
  "No recent conversations",
  "[status inherited text]",
])
const results = files.map((file) => {
  const capture = JSON.parse(readFileSync(new URL(file, root), "utf8"))
  return {
    file,
    theme: capture.theme,
    viewport: capture.viewport,
    documentWidth: capture.documentWidth,
    samples: capture.samples
      .filter(
        (sample) =>
          sample.measurable &&
          (exact.has(sample.text) ||
            sample.text.startsWith("Pi Desk is not installed"))
      )
      .map((sample) => ({
        text: sample.text,
        ratio: Number(contrastRatio(rgb(sample.fg), rgb(sample.bg)).toFixed(2)),
        fg: sample.fg,
        bg: sample.bg,
        sizePx: sample.sizePx,
        weight: sample.weight,
      })),
  }
})
writeFileSync(
  new URL("contrast-summary.json", root),
  JSON.stringify(results, null, 2) + "\n"
)
for (const result of results) {
  console.log(
    result.file,
    result.samples.map(({ text, ratio }) => [text.slice(0, 45), ratio])
  )
}
