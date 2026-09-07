import { describe, expect, it } from "vitest"
import { afterLoginPath, reviewLoginPath, reviewNotice } from "./locator"

const notice = "731030ee-94c7-4c77-ae33-1d2ad8999be6"
describe("review locator across operator login", () => {
  it("preserves only a canonical internal locator and never a token or redirect destination", () => {
    expect(
      reviewLoginPath("/ops-review", `?notice=${notice}&token=never-copy`)
    ).toBe(`/login?opsNotice=${notice}`)
    expect(
      afterLoginPath(`?opsNotice=${notice}&returnTo=https://evil.example`)
    ).toBe(`/ops-review?notice=${notice}`)
    expect(reviewLoginPath("/workspace", `?notice=${notice}`)).toBe("/login")
  })
  it("rejects malformed, duplicate and executable locator inputs", () => {
    for (const value of [
      "https://evil.example",
      "//evil.example",
      "javascript:alert(1)",
      notice.toUpperCase(),
      `${notice}&notice=${notice}`,
    ]) {
      expect(reviewNotice(`?notice=${value}`)).toBeNull()
      expect(
        afterLoginPath(
          `?opsNotice=${value.replace(/notice=/g, "opsNotice=")}`
        )
      ).toBe("/workspace")
    }
  })
})
