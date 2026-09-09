import { afterEach, beforeEach, describe, expect, it, vi } from "vitest"
import {
  createTerminalInput,
  type TerminalInputAdapter,
} from "./terminal-input"
import { ExecutionError } from "./protocol"
import type { OperationSummary, SessionSummary } from "./types"

function deferred<T>() {
  let resolve!: (value: T) => void
  let reject!: (error: unknown) => void
  const promise = new Promise<T>((done, fail) => {
    resolve = done
    reject = fail
  })
  return { promise, resolve, reject }
}
const pump = () => vi.advanceTimersByTimeAsync(0)
let current: SessionSummary
const write = vi.fn<TerminalInputAdapter["write"]>()
const receipt = vi.fn<TerminalInputAdapter["receipt"]>()
const revalidate = vi.fn<TerminalInputAdapter["current"]>()
const changed = vi.fn()
const queues: ReturnType<typeof createTerminalInput>[] = []
function queue() {
  const value = createTerminalInput(
    {
      taskId: current.taskId,
      sessionId: current.id,
      generation: current.generation,
    },
    { write, receipt, current: revalidate },
    changed
  )
  queues.push(value)
  return value
}
beforeEach(() => {
  vi.useFakeTimers()
  vi.clearAllMocks()
  current = {
    id: "synthetic-terminal",
    taskId: "synthetic-task",
    profileId: "synthetic-profile",
    profileRevision: 1,
    revision: 2,
    generation: 1,
    mode: "terminal",
    status: "running",
    title: "Synthetic terminal",
    createdAt: "2026-09-09T00:00:00Z",
    updatedAt: "2026-09-09T00:00:00Z",
    lastActivityAt: "2026-09-09T00:00:00Z",
    capabilities: {
      read: true,
      prompt: false,
      continue: false,
      stop: true,
      terminalWrite: true,
      importOutput: true,
    },
    reason: null,
  }
  write.mockImplementation(async (input) => ({
    id: input.operationId,
    status: "confirmed",
    reason: null,
  }))
  receipt.mockImplementation(async (id) => ({
    id,
    status: "confirmed",
    reason: null,
  }))
  revalidate.mockImplementation(async () => current)
})
afterEach(() => {
  queues.splice(0).forEach((value) => value.dispose())
  vi.useRealTimers()
})

describe("E1 terminal input around the inherited write queue", () => {
  it("does nothing on creation and preserves ordered coalesced input with distinct operation receipts", async () => {
    const first = deferred<OperationSummary>()
    write.mockReturnValueOnce(first.promise)
    const q = queue()
    expect(write).not.toHaveBeenCalled()
    q.enqueue("a")
    q.enqueue("b")
    q.enqueue("c")
    expect(write).toHaveBeenCalledOnce()
    first.resolve({
      id: write.mock.calls[0][0].operationId,
      status: "confirmed",
      reason: null,
    })
    await pump()
    expect(write.mock.calls.map(([input]) => input.data)).toEqual(["a", "bc"])
    expect(
      new Set(write.mock.calls.map(([input]) => input.operationId)).size
    ).toBe(2)
    expect(
      write.mock.calls.every(
        ([input]) =>
          input.sessionId === current.id && input.expectedGeneration === 1
      )
    ).toBe(true)
    expect(q.state().busy).toBe(false)
    expect(q.state().unresolved).toBe(false)
  })
  it("drops buffered later bytes after response loss and never resends the uncertain input", async () => {
    const first = deferred<OperationSummary>()
    write.mockReturnValueOnce(first.promise)
    const q = queue()
    q.enqueue("synthetic-first-input")
    q.enqueue("must-not-follow\r")
    first.reject(new Error("Synthetic response lost"))
    await pump()
    expect(q.state()).toMatchObject({
      frozen: true,
      unresolved: true,
      canResume: false,
    })
    expect(q.enqueue("later")).toBe(false)
    expect(await q.resume()).toBe(false)
    receipt.mockRejectedValueOnce(new ExecutionError("missing"))
    await q.reconcile()
    expect(q.state().unresolved).toBe(true)
    expect(write).toHaveBeenCalledOnce()
    await q.reconcile()
    expect(q.state()).toMatchObject({
      frozen: true,
      unresolved: false,
      canResume: true,
    })
    expect(write).toHaveBeenCalledOnce()
    expect(await q.resume()).toBe(true)
    expect(revalidate).toHaveBeenCalledWith(current.id, expect.any(AbortSignal))
    q.enqueue("new explicit input")
    await pump()
    expect(write.mock.calls.map(([input]) => input.data)).toEqual([
      "synthetic-first-input",
      "new explicit input",
    ])
    expect(
      receipt.mock.calls.every(
        ([id]) => id === write.mock.calls[0][0].operationId
      )
    ).toBe(true)
    expect(JSON.stringify(changed.mock.calls)).not.toContain(
      "synthetic-first-input"
    )
    expect(JSON.stringify(changed.mock.calls)).not.toContain("must-not-follow")
  })
  it.each(["pending", "uncertain"] as const)(
    "freezes after a %s receipt instead of treating promise resolution as write acceptance",
    async (status) => {
      write.mockImplementationOnce(async (input) => ({
        id: input.operationId,
        status,
        reason: null,
      }))
      const q = queue()
      q.enqueue("one")
      q.enqueue("two")
      await pump()
      expect(write).toHaveBeenCalledOnce()
      expect(q.state()).toMatchObject({
        frozen: true,
        unresolved: true,
        canResume: false,
      })
    }
  )
  it("rejects a receipt for a different operation and retains the original lookup identity", async () => {
    write.mockResolvedValueOnce({
      id: "foreign-operation",
      status: "confirmed",
      reason: null,
    })
    const q = queue()
    q.enqueue("one")
    await pump()
    expect(q.state().unresolved).toBe(true)
    await q.reconcile()
    expect(receipt.mock.calls[0][0]).toBe(write.mock.calls[0][0].operationId)
    expect(write).toHaveBeenCalledOnce()
  })
  it("splits a large Unicode paste into at most16KiB writes without splitting characters or losing byte order", async () => {
    const q = queue()
    const data = "😀".repeat(5000) + "العربيةé"
    expect(q.enqueue(data)).toBe(true)
    await pump()
    expect(write).toHaveBeenCalledTimes(2)
    const sent = write.mock.calls.map(([input]) => input.data)
    expect(sent.join("")).toBe(data)
    expect(
      sent.every((part) => new TextEncoder().encode(part).byteLength <= 16384)
    ).toBe(true)
    expect(sent.every((part) => !part.includes("\ufffd"))).toBe(true)
  })
  it("rejects an oversized whole paste and invalid Unicode before sending, without exposing input in state", () => {
    const q = queue()
    expect(q.enqueue("x".repeat(65537))).toBe(false)
    expect(q.enqueue("\ud800")).toBe(false)
    expect(write).not.toHaveBeenCalled()
    expect(q.state().reason).toBe("invalid")
  })
  it.each(["generation", "task", "read", "write", "mode"])(
    "requires fresh matching %s authority before explicitly resuming input",
    async (kind) => {
      write.mockImplementationOnce(async (input) => ({
        id: input.operationId,
        status: "failed",
        reason: "busy",
      }))
      const q = queue()
      q.enqueue("initial")
      await pump()
      if (kind === "generation") current.generation = 2
      if (kind === "task") current.taskId = "foreign-task"
      if (kind === "read") current.capabilities.read = false
      if (kind === "write") current.capabilities.terminalWrite = false
      if (kind === "mode") current.mode = "chat"
      expect(await q.resume()).toBe(false)
      expect(q.enqueue("new input")).toBe(false)
      expect(write).toHaveBeenCalledOnce()
    }
  )
  it("aborts and ignores late acknowledgements after disposal without delivering buffered input", async () => {
    const first = deferred<OperationSummary>()
    write.mockReturnValueOnce(first.promise)
    const q = queue()
    q.enqueue("one")
    q.enqueue("two")
    const count = changed.mock.calls.length
    q.dispose()
    expect(write.mock.calls[0][1].aborted).toBe(true)
    first.resolve({
      id: write.mock.calls[0][0].operationId,
      status: "confirmed",
      reason: null,
    })
    await pump()
    expect(write).toHaveBeenCalledOnce()
    expect(changed).toHaveBeenCalledTimes(count)
    expect(q.enqueue("three")).toBe(false)
    expect(q.state().canResume).toBe(false)
  })
})
