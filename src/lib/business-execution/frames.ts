import {
  DETACH_REASONS,
  OPERATION_REASONS,
  OPERATION_STATUSES,
  SESSION_STATUSES,
  type EventsInput,
  type MessagePart,
  type OperationSummary,
  type SessionEvent,
  type SessionFrame,
  type ToolState,
} from "./types"
import {
  boolean,
  boundedText,
  choice,
  ExecutionError,
  integer,
  nullableCursor,
  protocolError,
  record,
  text,
  values,
} from "./protocol"

export const MAX_FRAME_BYTES = 1024 * 1024
export const MAX_PART_BYTES = 16 * 1024

export function parseMessagePart(value: unknown): MessagePart {
  const row = record(value, ["messageId", "role", "part", "text", "complete"])
  return {
    messageId: text(row.messageId),
    role: choice(row.role, ["user", "assistant"]),
    part: integer(row.part, 0),
    text: boundedText(row.text, MAX_PART_BYTES),
    complete: boolean(row.complete),
  }
}
function parseTool(value: unknown): ToolState {
  const row = record(value, ["id", "name", "status"])
  return {
    id: text(row.id),
    name: text(row.name),
    status: choice(row.status, ["running", "completed", "failed"]),
  }
}
function parseOperation(value: unknown): OperationSummary {
  const row = record(value, ["id", "status", "reason"])
  return {
    id: text(row.id),
    status: choice(row.status, OPERATION_STATUSES),
    reason: row.reason === null ? null : choice(row.reason, OPERATION_REASONS),
  }
}
function parseEvent(value: unknown): SessionEvent {
  if (!value || typeof value !== "object" || !("kind" in value))
    return protocolError()
  switch (value.kind) {
    case "message": {
      const row = record(value, ["kind", "message"])
      return { kind: "message", message: parseMessagePart(row.message) }
    }
    case "tool": {
      const row = record(value, ["kind", "tool"])
      return { kind: "tool", tool: parseTool(row.tool) }
    }
    case "status": {
      const row = record(value, ["kind", "status"])
      return { kind: "status", status: choice(row.status, SESSION_STATUSES) }
    }
    case "terminal": {
      const row = record(value, ["kind", "data"])
      return { kind: "terminal", data: boundedText(row.data, MAX_PART_BYTES) }
    }
    default:
      return protocolError()
  }
}
export function parseSessionFrame(
  value: unknown,
  expected: EventsInput
): SessionFrame {
  if (!value || typeof value !== "object" || !("type" in value))
    return protocolError()
  const common = ["type", "sessionId", "generation"]
  function binding(row: Record<string, unknown>) {
    const sessionId = text(row.sessionId)
    const generation = integer(row.generation)
    if (
      sessionId !== expected.sessionId ||
      generation !== expected.expectedGeneration
    )
      throw new ExecutionError("conflict")
    return { sessionId, generation }
  }
  function receipt(row: Record<string, unknown>) {
    const operation = parseOperation(row.operation)
    if (operation.id !== expected.operationId) return protocolError()
    return operation
  }
  switch (value.type) {
    case "snapshot": {
      const row = record(value, [
        ...common,
        "operation",
        "cursor",
        "reset",
        "reason",
        "state",
        "olderCursor",
      ])
      if (row.reset !== true) return protocolError()
      const state = record(row.state, ["status", "messages", "tools"])
      return {
        ...binding(row),
        type: "snapshot",
        operation: receipt(row),
        cursor: text(row.cursor),
        reset: true,
        reason: choice(row.reason, ["initial", "cursor_expired", "cursor_invalid"]),
        state: {
          status: choice(state.status, SESSION_STATUSES),
          messages: values(state.messages, 40).map(parseMessagePart),
          tools: values(state.tools, 40).map(parseTool),
        },
        olderCursor: nullableCursor(row.olderCursor),
      }
    }
    case "replay": {
      const row = record(value, [
        ...common,
        "operation",
        "cursor",
        "reset",
        "events",
      ])
      if (row.reset !== false) return protocolError()
      return {
        ...binding(row),
        type: "replay",
        operation: receipt(row),
        cursor: text(row.cursor),
        reset: false,
        events: values(row.events).map(parseEvent),
      }
    }
    case "event": {
      const row = record(value, [...common, "cursor", "event"])
      return {
        ...binding(row),
        type: "event",
        cursor: text(row.cursor),
        event: parseEvent(row.event),
      }
    }
    case "heartbeat": {
      const row = record(value, [...common, "cursor"])
      return { ...binding(row), type: "heartbeat", cursor: text(row.cursor) }
    }
    case "detached": {
      const row = record(value, [...common, "reason"])
      return {
        ...binding(row),
        type: "detached",
        reason: choice(row.reason, DETACH_REASONS),
      }
    }
    default:
      return protocolError()
  }
}

// A frame may span chunks, including the bytes within one UTF-8 character.
// Count bytes before decoding/concatenation, not UTF-16 string length.
export class SessionFrameDecoder {
  private decoder = new TextDecoder("utf-8", { fatal: true })
  private parts: string[] = []
  private bytes = 0

  push(chunk: Uint8Array, onFrame: (value: unknown) => boolean | void): void {
    let start = 0
    for (let end = 0; end < chunk.length; end++) {
      if (chunk[end] !== 10) continue
      this.append(chunk.subarray(start, end + 1))
      let value: unknown
      try {
        this.parts.push(this.decoder.decode())
        value = JSON.parse(this.parts.join(""))
      } catch {
        return protocolError()
      }
      this.decoder = new TextDecoder("utf-8", { fatal: true })
      this.parts = []
      this.bytes = 0
      if (onFrame(value) === false) return
      start = end + 1
    }
    if (start < chunk.length) this.append(chunk.subarray(start))
  }
  finish(): void {
    // Every object must be LF terminated. Truncation is not a complete frame.
    if (this.bytes !== 0) return protocolError()
  }
  private append(chunk: Uint8Array) {
    this.bytes += chunk.byteLength
    if (this.bytes > MAX_FRAME_BYTES) return protocolError()
    try {
      this.parts.push(this.decoder.decode(chunk, { stream: true }))
    } catch {
      return protocolError()
    }
  }
}
