import { describe, expect, it } from "vitest"
import {
  intakeReason,
  sourceIsFresh,
  validPassages,
  type SourceSummary,
  type Passage,
} from "./intake"

const source: SourceSummary = {
  id: "source",
  bindingId: "binding",
  kind: "fireflies",
  title: "Synthetic meeting",
  revision: 2,
  observedAt: "2026-09-08T09:00:00Z",
  accessValidUntil: "2026-09-08T09:05:00Z",
  access: "fresh",
  content: "available",
  summary: "empty",
  providerSummaryStatus: null,
  requiresRefresh: false,
}
const passage: Passage = {
  id: "p1",
  sourceRevision: 2,
  kind: "sentence",
  text: "Exact words",
  index: 0,
  start: null,
  end: null,
}
describe("source disclosure and exact selection", () => {
  it("only removes server freshness, including the exact expiry boundary", () => {
    const now = Date.parse("2026-09-08T09:04:59Z")
    expect(sourceIsFresh(source, now)).toBe(true)
    expect(sourceIsFresh(source, now + 1000)).toBe(false)
    for (const access of ["unverified", "expired", "denied"] as const)
      expect(sourceIsFresh({ ...source, access }, now)).toBe(false)
    for (const change of [
      { revision: null },
      { requiresRefresh: true },
      { accessValidUntil: null },
      { accessValidUntil: "invalid" },
    ])
      expect(sourceIsFresh({ ...source, ...change }, now)).toBe(false)
  })
  it("rejects mixed revisions, missing/duplicate/too many passages and oversize text without clipping", () => {
    expect(validPassages([passage], ["p1"], 2)).toBe(true)
    expect(validPassages([passage], ["p1"], 3)).toBe(false)
    expect(validPassages([passage], ["missing"], 2)).toBe(false)
    expect(validPassages([passage], ["p1", "p1"], 2)).toBe(false)
    expect(validPassages([passage], [], 2)).toBe(false)
    expect(validPassages([passage], ["p1"], null)).toBe(false)
    const many = Array.from({ length: 21 }, (_, i) => ({
      ...passage,
      id: `p${i}`,
    }))
    expect(
      validPassages(
        many,
        many.map((p) => p.id),
        2
      )
    ).toBe(false)
    expect(
      validPassages([{ ...passage, text: "🙂".repeat(20000) }], ["p1"], 2)
    ).toBe(true)
    expect(
      validPassages([{ ...passage, text: "ع".repeat(20001) }], ["p1"], 2)
    ).toBe(false)
  })
  it("recognizes only finite exact reason keys", () => {
    expect(intakeReason("business.intake.binding_unavailable")).toBe(
      "binding_unavailable"
    )
    expect(
      intakeReason("business.intake.provider_unavailable.private-token")
    ).toBeUndefined()
    expect(intakeReason("other.binding_unavailable")).toBeUndefined()
  })
})
