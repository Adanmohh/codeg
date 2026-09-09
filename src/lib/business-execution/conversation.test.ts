// @vitest-environment node
import { describe, expect, it } from "vitest"
import { applyConversationFrame, emptyConversation } from "./conversation"
import { parseHistory } from "./session"
import type { MessagePart, SessionFrame } from "./types"

const part: MessagePart = {
  messageId: "synthetic-message",
  part: 0,
  role: "assistant",
  text: "Synthetic saved text",
  complete: true,
}
const snapshot: SessionFrame = {
  type: "snapshot",
  sessionId: "synthetic-session",
  generation: 1,
  operation: { id: "synthetic-attach", status: "confirmed", reason: null },
  cursor: "opaque-start",
  reset: true,
  reason: "initial",
  state: { status: "idle", messages: [part], tools: [] },
  olderCursor: "opaque-earlier",
}

describe("bounded conversation presentation", () => {
  it("deduplicates snapshot and history message-part identities as well as live delivery", () => {
    const view = applyConversationFrame(emptyConversation("starting"), {
      ...snapshot,
      state: { ...snapshot.state, messages: [part, part] },
    })
    expect(view.messages).toEqual([part])
    expect(
      parseHistory({ messages: [part, part], nextBeforeCursor: null }).messages
    ).toEqual([part])
  })
  it("retains a bounded live window with an explicit history indicator and a real server cursor", () => {
    let view = applyConversationFrame(emptyConversation("starting"), snapshot)
    for (let index = 1; index <= 170; index++)
      view = applyConversationFrame(view, {
        type: "event",
        sessionId: snapshot.sessionId,
        generation: 1,
        cursor: `opaque-${index}`,
        event: {
          kind: "message",
          message: { ...part, messageId: `synthetic-${index}` },
        },
      })
    expect(view.messages).toHaveLength(160)
    expect(view.trimmed).toBe(true)
    expect(view.olderCursor).toBe("opaque-earlier")
    expect(view.cursor).toBe("opaque-170")
    const reset = applyConversationFrame(view, {
      ...snapshot,
      reason: "cursor_expired",
    })
    expect(reset.messages).toEqual([part])
    expect(reset.trimmed).toBe(false)
  })
  it("does not turn private terminal output into conversation text", () => {
    const view = applyConversationFrame(emptyConversation("idle"), {
      type: "event",
      sessionId: snapshot.sessionId,
      generation: 1,
      cursor: "opaque-terminal",
      event: { kind: "terminal", data: "Synthetic private shell bytes" },
    })
    expect(view.messages).toEqual([])
    expect(JSON.stringify(view)).not.toContain("Synthetic private shell bytes")
  })
  it("rejects oversize history instead of silently trimming an authoritative response", () => {
    expect(() =>
      parseHistory({
        messages: Array.from({ length: 41 }, () => part),
        nextBeforeCursor: null,
      })
    ).toThrow()
  })
})
