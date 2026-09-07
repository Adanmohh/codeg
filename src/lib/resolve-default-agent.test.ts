import { describe, expect, it } from "vitest"
import { resolveDefaultAgent } from "./resolve-default-agent"

describe("Desk default agent", () => {
  const input = {
    folderDefault: null,
    inherit: null,
    sortedTypes: [],
    fresh: false,
  } as const
  it("uses Pi only when no saved or inherited choice exists", () => {
    expect(resolveDefaultAgent({ ...input, sortedTypes: [] })).toEqual({
      agentType: "pi",
      provisional: true,
    })
  })
  it("preserves the explicit folder default", () => {
    expect(
      resolveDefaultAgent({
        ...input,
        sortedTypes: ["pi"],
        folderDefault: "codex",
      }).agentType
    ).toBe("codex")
  })
  it("preserves the active conversation's agent", () => {
    expect(
      resolveDefaultAgent({
        ...input,
        sortedTypes: ["pi"],
        inherit: "claude_code",
      }).agentType
    ).toBe("claude_code")
  })
  it("preserves the user's saved agent ordering", () => {
    expect(
      resolveDefaultAgent({
        ...input,
        sortedTypes: ["open_code", "pi"],
        fresh: true,
      })
    ).toEqual({ agentType: "open_code", provisional: false })
  })
})
