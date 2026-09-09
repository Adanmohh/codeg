import { createWriteQueue } from "@/lib/terminal/write-queue"
import { parseOperationSummary } from "./frames"
import { ExecutionError, safeExecutionError } from "./protocol"
import { parseSession } from "./session"
import type {
  ExecutionInputs,
  OperationReason,
  OperationSummary,
  SessionSummary,
} from "./types"

type WriteInput = ExecutionInputs["sessions/terminal-write"]
// This is an injected lower-component adapter, NOT a proposed HTTP envelope.
// The actual write result wrapper must be published before client/route wiring.
export interface TerminalInputAdapter {
  write(input: WriteInput, signal: AbortSignal): Promise<OperationSummary>
  receipt(operationId: string, signal: AbortSignal): Promise<OperationSummary>
  current(sessionId: string, signal: AbortSignal): Promise<SessionSummary>
}
export interface TerminalInputState {
  frozen: boolean
  busy: boolean
  unresolved: boolean
  canResume: boolean
  operation: OperationSummary | null
  reason: OperationReason | null
}
const MAX_WRITE_BYTES = 16 * 1024
const MAX_BUFFER_BYTES = 64 * 1024
const encoder = new TextEncoder()
function chunks(data: string): string[] {
  const result: string[] = []
  let chunk = ""
  let bytes = 0
  for (const character of data) {
    const length = encoder.encode(character).byteLength
    if (bytes + length > MAX_WRITE_BYTES) {
      result.push(chunk)
      chunk = ""
      bytes = 0
    }
    chunk += character
    bytes += length
  }
  if (chunk) result.push(chunk)
  return result
}
function hasLoneSurrogate(data: string) {
  // for-of keeps complete surrogate pairs together. A remaining single code
  // unit is not a UTF-8 character and must not be silently replaced on write.
  for (const character of data) {
    if (character.length === 1 && /[\ud800-\udfff]/.test(character)) return true
  }
  return false
}

export function createTerminalInput(
  binding: { taskId: string; sessionId: string; generation: number },
  adapter: TerminalInputAdapter,
  onState: (state: TerminalInputState) => void
) {
  const expected = { ...binding }
  const lifetime = new AbortController()
  let disposed = false
  let frozen = false
  let sending = false
  let checking = false
  let buffered = 0
  let held: WriteInput | null = null
  let operation: OperationSummary | null = null
  let reason: OperationReason | null = null
  function state(): TerminalInputState {
    return {
      frozen,
      busy: !disposed && (sending || checking || (!frozen && buffered > 0)),
      unresolved: held !== null,
      canResume: !disposed && frozen && !sending && !checking && held === null,
      operation: operation ? { ...operation } : null,
      reason,
    }
  }
  function emit() {
    if (!disposed) onState(state())
  }
  function freeze(error?: unknown) {
    frozen = true
    queue.dispose()
    buffered = 0
    if (error !== undefined) reason = safeExecutionError(error).reason
    emit()
  }
  function accept(value: unknown, input: WriteInput) {
    const receipt = parseOperationSummary(value)
    if (receipt.id !== input.operationId)
      throw new ExecutionError("transport_unavailable")
    operation = receipt
    reason = receipt.reason
    if (receipt.status === "confirmed" || receipt.status === "failed")
      held = null
    if (receipt.status !== "confirmed") freeze()
    return receipt.status === "confirmed"
  }
  function makeQueue() {
    return createWriteQueue(async (batch) => {
      // The inherited queue serializes/coalesces input. Its failure policy would
      // otherwise continue the next batch; dispose it before returning any
      // uncertain result so later shell input cannot pass the unknown write.
      for (const data of chunks(batch)) {
        if (disposed || frozen) return
        const input: WriteInput = {
          operationId: crypto.randomUUID(),
          sessionId: expected.sessionId,
          expectedGeneration: expected.generation,
          data,
        }
        held = input
        operation = { id: input.operationId, status: "pending", reason: null }
        sending = true
        reason = null
        emit()
        try {
          const receipt = await adapter.write({ ...input }, lifetime.signal)
          if (disposed) return
          if (!accept(receipt, input)) return
          buffered -= encoder.encode(data).byteLength
        } catch (error) {
          if (!disposed) freeze(error)
          return
        } finally {
          if (!disposed) {
            sending = false
            emit()
          }
        }
      }
    })
  }
  let queue = makeQueue()
  return {
    state,
    enqueue(data: string): boolean {
      if (disposed || frozen) return false
      if (!data) return true
      const bytes = encoder.encode(data).byteLength
      // Reject the whole incoming paste before enqueue; retain prior accepted
      // input. Report the rejection so the view never silently truncates it.
      if (hasLoneSurrogate(data) || buffered + bytes > MAX_BUFFER_BYTES) {
        reason = "invalid"
        emit()
        return false
      }
      buffered += bytes
      queue.enqueue(data)
      return true
    },
    async reconcile() {
      if (disposed || !frozen || sending || checking || !held) return
      const input = held
      checking = true
      emit()
      try {
        const value = await adapter.receipt(input.operationId, lifetime.signal)
        if (!disposed) accept(value, input)
      } catch (error) {
        if (!disposed) reason = safeExecutionError(error).reason
      } finally {
        if (!disposed) {
          checking = false
          emit()
        }
      }
    },
    // Explicit resumption rechecks the actual current session. It never replays
    // discarded bytes, changes binding, starts a process or resolves an unknown
    // receipt. The eventual adapter uses the existing sessions/get contract.
    async resume(): Promise<boolean> {
      if (disposed || !state().canResume) return false
      checking = true
      emit()
      try {
        const value = await adapter.current(expected.sessionId, lifetime.signal)
        if (disposed) return false
        const current = parseSession(value, {
          taskId: expected.taskId,
          sessionId: expected.sessionId,
        })
        if (current.generation !== expected.generation)
          throw new ExecutionError("conflict")
        if (
          current.mode !== "terminal" ||
          !current.capabilities.terminalWrite ||
          !current.capabilities.read ||
          current.status === "revoked"
        )
          throw new ExecutionError("forbidden")
        frozen = false
        operation = null
        reason = null
        queue = makeQueue()
        return true
      } catch (error) {
        if (!disposed) freeze(error)
        return false
      } finally {
        if (!disposed) {
          checking = false
          emit()
        }
      }
    },
    dispose() {
      disposed = true
      frozen = true
      operation = null
      lifetime.abort()
      queue.dispose()
      buffered = 0
      held = null
    },
  }
}
