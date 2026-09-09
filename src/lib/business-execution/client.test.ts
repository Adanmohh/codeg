// @vitest-environment node
import { afterEach, describe, expect, it, vi } from "vitest"
import {
  createExecutionHttpClient,
  type ExecutionClient,
  type ExecutionHttpTransport,
} from "./client"
import type { EventsInput, SessionFrame } from "./types"

const clients: ExecutionClient[] = []
function setup(post: ExecutionHttpTransport["post"]) {
  const host = { post: vi.fn(post), unauthorized: vi.fn() }
  const client = createExecutionHttpClient(host)
  clients.push(client)
  return { client, host }
}
function deferred<T>() {
  let resolve!: (value: T) => void
  const promise = new Promise<T>((done) => {
    resolve = done
  })
  return { promise, resolve }
}
const attach = (): EventsInput => ({
  operationId: "synthetic-attach",
  sessionId: "synthetic-session",
  expectedGeneration: 1,
  cursor: null,
})
function snapshot(input: EventsInput): SessionFrame {
  return {
    type: "snapshot",
    sessionId: input.sessionId,
    generation: input.expectedGeneration,
    operation: { id: input.operationId, status: "confirmed", reason: null },
    cursor: "opaque-position",
    reset: true,
    reason: "initial",
    state: { status: "idle", messages: [], tools: [] },
    olderCursor: null,
  }
}
const streamHeaders = {
  "content-type": "application/x-ndjson; charset=utf-8",
  "cache-control": "no-store",
  "x-content-type-options": "nosniff",
}
const encoded = (frame: SessionFrame) =>
  new TextEncoder().encode(`${JSON.stringify(frame)}\n`)

afterEach(() => {
  for (const client of clients.splice(0)) client.close()
  vi.useRealTimers()
  vi.restoreAllMocks()
})

describe("E1 scoped client lifecycle", () => {
  it("does not issue a request after its scope is closed", async () => {
    const { client, host } = setup(async () => Response.json({}))
    client.close()
    await expect(
      client.json("profiles/list", { taskId: "synthetic-task" })
    ).rejects.toMatchObject({
      reason: "transport_unavailable",
      kind: "closed",
    })
    expect(host.post).not.toHaveBeenCalled()
  })

  it("cancels a response that arrives after close without returning data", async () => {
    const pending = deferred<Response>()
    const cancel = vi.fn()
    const { client, host } = setup(() => pending.promise)
    const result = client.json("profiles/list", { taskId: "synthetic-task" })
    const rejected = expect(result).rejects.toMatchObject({ kind: "closed" })
    client.close()
    pending.resolve(new Response(new ReadableStream({ cancel })))
    await rejected
    expect(cancel).toHaveBeenCalledOnce()
    expect(host.post.mock.calls[0][2].aborted).toBe(true)
    expect(host.post).toHaveBeenCalledOnce()
  })

  it("classifies a local deadline as uncertain transport, never a durable cancellation", async () => {
    vi.useFakeTimers()
    const { client, host } = setup(
      (_path, _input, signal) =>
        new Promise((_resolve, reject) => {
          signal.addEventListener("abort", () =>
            reject(new Error("synthetic private transport diagnostic"))
          )
        })
    )
    const result = client
      .json("sessions/prompt", {
        operationId: "synthetic-prompt-receipt",
        sessionId: "synthetic-session",
        expectedSessionRevision: 3,
        text: "Synthetic draft",
        inputs: [],
      })
      .catch((error: unknown) => error)
    await vi.advanceTimersByTimeAsync(20000)
    expect(await result).toMatchObject({
      kind: "timeout",
      reason: "transport_unavailable",
      message: "transport_unavailable",
    })
    expect(host.post).toHaveBeenCalledOnce()
    expect(vi.getTimerCount()).toBe(0)
  })

  it("keeps the original stream binding while a caller changes its selection", async () => {
    const pending = deferred<Response>()
    const { client, host } = setup(() => pending.promise)
    const input = attach()
    const original = { ...input }
    const onFrame = vi.fn()
    const result = client.events(input, onFrame)
    input.sessionId = "other-session"
    input.expectedGeneration = 9
    input.operationId = "other-attach"
    pending.resolve(
      new Response(encoded(snapshot(original)), { headers: streamHeaders })
    )
    await expect(result).resolves.toEqual({ kind: "interrupted" })
    expect(onFrame).toHaveBeenCalledOnce()
    expect(onFrame).toHaveBeenCalledWith(snapshot(original))
    expect(host.post.mock.calls[0][1]).toEqual(original)
  })

  it("keeps an attached stream beyond the request deadline and detaches on close", async () => {
    vi.useFakeTimers()
    const ready = deferred<void>()
    const cancel = vi.fn()
    const input = attach()
    const { client, host } = setup(
      async () =>
        new Response(
          new ReadableStream({
            start(controller) {
              controller.enqueue(encoded(snapshot(input)))
            },
            cancel,
          }),
          { headers: streamHeaders }
        )
    )
    const onFrame = vi.fn(() => ready.resolve())
    const result = client.events(input, onFrame)
    const rejected = expect(result).rejects.toMatchObject({ kind: "closed" })
    await ready.promise
    await vi.advanceTimersByTimeAsync(20001)
    expect(host.post.mock.calls[0][2].aborted).toBe(false)
    expect(onFrame).toHaveBeenCalledOnce()
    client.close()
    await rejected
    expect(cancel).toHaveBeenCalledOnce()
    expect(host.post).toHaveBeenCalledOnce()
  })

  it("invalidates the parent on 401 without reading or exposing the error body", async () => {
    const cancel = vi.fn()
    const { client, host } = setup(
      async () => new Response(new ReadableStream({ cancel }), { status: 401 })
    )
    await expect(
      client.json("profiles/list", { taskId: "synthetic-task" })
    ).rejects.toMatchObject({ reason: "unauthorized", message: "unauthorized" })
    expect(cancel).toHaveBeenCalledOnce()
    expect(host.unauthorized).toHaveBeenCalledOnce()
    await expect(
      client.json("sessions/list", {
        taskId: "synthetic-task",
        cursor: null,
        limit: 20,
      })
    ).rejects.toThrow()
    expect(host.post).toHaveBeenCalledOnce()
  })

  it.each([
    [403, "forbidden"],
    [404, "missing"],
  ] as const)(
    "withholds a %i body without invalidating other authorized work",
    async (status, reason) => {
      const cancel = vi.fn()
      const { client, host } = setup(
        async () => new Response(new ReadableStream({ cancel }), { status })
      )
      await expect(
        client.json("sessions/get", { sessionId: "synthetic-session" })
      ).rejects.toMatchObject({ reason, message: reason })
      expect(cancel).toHaveBeenCalledOnce()
      expect(host.unauthorized).not.toHaveBeenCalled()
      host.post.mockResolvedValueOnce(
        Response.json({ profiles: [], unavailableReason: null })
      )
      await expect(
        client.json("profiles/list", { taskId: "synthetic-task" })
      ).resolves.toEqual({ profiles: [], unavailableReason: null })
    }
  )

  it("never replays a lost prompt response or returns its private diagnostic", async () => {
    const { client, host } = setup(async () => {
      throw new Error("synthetic private transport diagnostic")
    })
    const input = {
      operationId: "synthetic-original-operation",
      sessionId: "synthetic-session",
      expectedSessionRevision: 3,
      text: "Retain this exact synthetic draft",
      inputs: [],
    }
    await expect(client.json("sessions/prompt", input)).rejects.toMatchObject({
      reason: "transport_unavailable",
      message: "transport_unavailable",
    })
    expect(host.post).toHaveBeenCalledOnce()
    expect(host.post.mock.calls[0][1]).toEqual(input)
  })

  it("keeps a server-reported cancellation distinct from local interruption", async () => {
    const { client } = setup(async () =>
      Response.json({ error: { code: "cancelled" } }, { status: 409 })
    )
    await expect(
      client.json("sessions/continue", {
        operationId: "synthetic-continue",
        sessionId: "synthetic-session",
        expectedSessionRevision: 3,
      })
    ).rejects.toMatchObject({ name: "ExecutionError", reason: "cancelled" })
  })

  it("refuses unavailable account references before transport", async () => {
    const { client, host } = setup(async () => Response.json({}))
    await expect(
      client.json("sessions/prompt", {
        operationId: "synthetic-prompt",
        sessionId: "synthetic-session",
        expectedSessionRevision: 3,
        text: "Synthetic prompt",
        inputs: [
          {
            kind: "account_snapshot",
            snapshotId: "future",
            expectedRevision: 1,
          },
        ],
      })
    ).rejects.toMatchObject({ reason: "unavailable" })
    expect(host.post).not.toHaveBeenCalled()
  })

  it("does not post when its caller already detached", async () => {
    const scope = new AbortController()
    scope.abort()
    const { client, host } = setup(async () => Response.json({}))
    await expect(
      client.json("profiles/list", { taskId: "synthetic-task" }, scope.signal)
    ).rejects.toMatchObject({
      reason: "transport_unavailable",
      kind: "aborted",
    })
    expect(host.post).not.toHaveBeenCalled()
  })
})
