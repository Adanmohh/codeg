// Pure report conversion only. Browser measurements come from Playwright CLI.
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const studio = process.argv[2];
if (!studio) throw new Error("Pass the existing Design Studio repository path");
const { buildReport, countUnnamedInteractive } = await import(pathToFileURL(path.join(studio, "lab/tools/probe.mjs")));
const { parseBrief } = await import(pathToFileURL(path.join(studio, "scripts/brief.mjs")));
const root = "reports/business-workspace-evidence";
const original = parseBrief(fs.readFileSync("docs/design/BRIEF.html", "utf8"));
const conversion = JSON.parse(fs.readFileSync(`${root}/brief-canvas-normalization.json`, "utf8"));
for (const token of conversion.conversions) {
  if (original.tokens.colors[token.name] !== token.original)
    throw new Error(`Brief changed: ${token.name}`);
}
const names = ["list-1280-light", "review-1280-light", "review-390-arabic-dark", "review-1280-light-final", "review-1280-dark-final"];
const summaries = names.map(name => {
  const raw = JSON.parse(fs.readFileSync(`${root}/generation2/${name}.raw.json`, "utf8"));
  if (typeof raw.aria?.snapshot === "string")
    raw.aria.unnamedInteractive = countUnnamedInteractive(raw.aria.snapshot);
  const report = buildReport(raw, conversion.derived);
  fs.writeFileSync(`${root}/generation2/${name}.original-brief.json`, JSON.stringify(buildReport(raw, original), null, 2) + "\n");
  fs.writeFileSync(`${root}/generation2/${name}.measured.json`, JSON.stringify(report, null, 2) + "\n");
  return { name, contrast: report.contrast, unnamedInteractive: report.aria?.unnamedInteractive, headingOrderBreaks: report.aria?.headingOrderBreaks, briefViolations: report.briefViolations, motion: report.motion };
});
fs.writeFileSync(`${root}/measured-summary.json`, JSON.stringify({ normalization: "Canvas-normalized copy; original brief is unchanged. Paired dark and alpha/status colors still require source classification.", summaries }, null, 2) + "\n");
process.stdout.write(JSON.stringify(summaries.map(({ name, contrast, unnamedInteractive, headingOrderBreaks, briefViolations }) => ({ name, contrast, unnamedInteractive, headingOrderBreaks, briefViolations: briefViolations.length })), null, 2) + "\n");
