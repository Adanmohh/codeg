// @vitest-environment node
import { describe, expect, it, vi } from "vitest"
import { MAX_FRAME_BYTES, SessionFrameDecoder } from "./frames"
import { readSessionStream } from "./reader"
import type { EventsInput, MessagePart, SessionFrame } from "./types"

const input: EventsInput = {
  operationId: "synthetic-attach",
  sessionId: "synthetic-session",
  expectedGeneration: 2,
  cursor: null,
}
const message: MessagePart = {
  messageId: "synthetic-message",
  role: "assistant",
  part: 0,
  text: "Synthetic plan — خطة 🧭",
  complete: true,
}
const snapshot: SessionFrame = {
  type: "snapshot",
  sessionId: input.sessionId,
  generation: 2,
  operation: { id: input.operationId, status: "confirmed", reason: null },
  cursor: "opaque-cursor-1",
  reset: true,
  reason: "initial",
  state: { status: "idle", messages: [message], tools: [] },
  olderCursor: "opaque-older-history",
}
const detached: SessionFrame = {
  type: "detached",
  sessionId: input.sessionId,
  generation: 2,
  reason: "authority_changed",
}
const headers = {
  "content-type": "application/x-ndjson; charset=utf-8",
  "cache-control": "no-store",
  "x-content-type-options": "nosniff",
}
const encode = (value: unknown) =>
  new TextEncoder().encode(`${JSON.stringify(value)}\n`)
function stream(chunks: Uint8Array[]) {
  return new Response(
    new ReadableStream({
      start(controller) {
        for (const chunk of chunks) controller.enqueue(chunk)
        controller.close()
      },
    }),
    { headers }
  )
}
const read = (response: Response, callback = vi.fn()) =>
  readSessionStream(response, input, new AbortController().signal, callback)

describe("E1 closed authenticated stream reader", () => {
  it("keeps split UTF-8/lines and a final detach in exact order", async () => {
    const data = new TextEncoder().encode(
      [snapshot, detached].map((row) => JSON.stringify(row)).join("\n") + "\n"
    )
    const callback = vi.fn()
    const chunks = Array.from(data, (byte) => Uint8Array.of(byte))
    await expect(read(stream(chunks), callback)).resolves.toEqual({
      kind: "detached",
      reason: "authority_changed",
    })
    expect(callback.mock.calls.map(([frame]) => frame)).toEqual([
      snapshot,
      detached,
    ])
  })
  it("handles several complete frames in one network chunk", async () => {
    const heartbeat = {
      type: "heartbeat",
      sessionId: input.sessionId,
      generation: 2,
      cursor: "opaque-cursor-1",
    }
    const event = {
      type: "event",
      sessionId: input.sessionId,
      generation: 2,
      cursor: "opaque-cursor-2",
      event: {
        kind: "message",
        message: { ...message, part: 1, text: "Next" },
      },
    }
    const callback = vi.fn()
    const body =
      [snapshot, heartbeat, event]
        .map((frame) => JSON.stringify(frame))
        .join("\n") + "\n"
    await expect(
      read(new Response(body, { headers }), callback)
    ).resolves.toEqual({
      kind: "interrupted",
    })
    expect(callback.mock.calls.map(([frame]) => frame)).toEqual([
      snapshot,
      heartbeat,
      event,
    ])
  })
  it.each([
    { ...snapshot, sessionId: "foreign-session" },
    { ...snapshot, generation: 3 },
    { ...snapshot, operation: { ...snapshot.operation, id: "foreign-attach" } },
  ])(
    "withholds foreign session/generation/receipt before delivery",
    async (value) => {
      const callback = vi.fn()
      await expect(read(stream([encode(value)]), callback)).rejects.toThrow()
      expect(callback).not.toHaveBeenCalled()
    }
  )
  it.each([
    { ...snapshot, reset: false },
    { ...snapshot, privatePath: "synthetic-private-path" },
    {
      ...snapshot,
      state: {
        ...snapshot.state,
        messages: Array.from({ length: 41 }, () => message),
      },
    },
    {
      ...snapshot,
      state: {
        ...snapshot.state,
        messages: [{ ...message, text: "أ".repeat(8193) }],
      },
    },
    {
      ...snapshot,
      state: {
        ...snapshot.state,
        tools: [
          {
            id: "tool",
            name: "Write",
            status: "running",
            arguments: "synthetic-secret",
          },
        ],
      },
    },
  ])(
    "rejects malformed/over-limit closed fields without releasing them",
    async (value) => {
      const callback = vi.fn()
      await expect(
        read(stream([encode(value)]), callback)
      ).rejects.toMatchObject({
        reason: "transport_unavailable",
        message: "transport_unavailable",
      })
      expect(callback).not.toHaveBeenCalled()
    }
  )
  it("caps frame bytes before building or parsing an oversized line", () => {
    const decoder = new SessionFrameDecoder()
    const callback = vi.fn()
    decoder.push(new Uint8Array(MAX_FRAME_BYTES).fill(32), callback)
    expect(() => decoder.push(Uint8Array.of(32), callback)).toThrow(
      "transport_unavailable"
    )
    expect(callback).not.toHaveBeenCalled()
  })
  it.each([
    new TextEncoder().encode(JSON.stringify(snapshot)),
    Uint8Array.of(0xff, 0x0a),
    new TextEncoder().encode("{not-json}\n"),
  ])("refuses truncated lines or invalid UTF-8/JSON", async (chunk) => {
    const callback = vi.fn()
    await expect(read(stream([chunk]), callback)).rejects.toMatchObject({
      reason: "transport_unavailable",
    })
    expect(callback).not.toHaveBeenCalled()
  })
  it("requires exactly one initial snapshot or replay", async () => {
    await expect(read(stream([encode(detached)]))).rejects.toThrow()
    const callback = vi.fn()
    await expect(
      read(stream([encode(snapshot), encode(snapshot)]), callback)
    ).rejects.toThrow()
    expect(callback).toHaveBeenCalledTimes(1)
  })
  it("discards queued bytes after an authority detach and cancels the reader", async () => {
    const cancel = vi.fn()
    const body =
      [snapshot, detached].map((frame) => JSON.stringify(frame)).join("\n") +
      '\n{"private":"must never be parsed or released"'
    const response = new Response(
      new ReadableStream({
        start(controller) {
          controller.enqueue(new TextEncoder().encode(body))
        },
        cancel,
      }),
      { headers }
    )
    const callback = vi.fn()
    await expect(read(response, callback)).resolves.toEqual({
      kind: "detached",
      reason: "authority_changed",
    })
    expect(callback).toHaveBeenCalledTimes(2)
    expect(cancel).toHaveBeenCalledOnce()
  })
  it("scope abort closes an idle stream without delivering later data", async () => {
    const scope = new AbortController()
    const cancel = vi.fn()
    const callback = vi.fn(() => scope.abort())
    const response = new Response(
      new ReadableStream({
        start(controller) {
          controller.enqueue(encode(snapshot))
        },
        cancel,
      }),
      { headers }
    )
    await expect(
      readSessionStream(response, input, scope.signal, callback)
    ).rejects.toMatchObject({
      reason: "transport_unavailable",
      kind: "aborted",
    })
    expect(callback).toHaveBeenCalledOnce()
    expect(cancel).toHaveBeenCalledOnce()
  })
  it("requires no-store/nosniff and the exact NDJSON media type", async () => {
    for (const field of Object.keys(headers)) {
      const invalid = new Headers(headers)
      invalid.delete(field)
      await expect(
        read(new Response(encode(snapshot), { headers: invalid }))
      ).rejects.toMatchObject({ reason: "transport_unavailable" })
    }
  })
})
