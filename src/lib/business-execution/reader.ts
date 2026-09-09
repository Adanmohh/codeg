import {
  ExecutionError,
  parseErrorCode,
  protocolError,
  safeExecutionError,
} from "./protocol"
import { parseSessionFrame, SessionFrameDecoder } from "./frames"
import type { DetachReason, EventsInput, SessionFrame } from "./types"

export function assertActive(signal: AbortSignal): void {
  if (signal.aborted) throw new ExecutionError("cancelled")
}
export async function readChunks(
  response: Response,
  signal: AbortSignal,
  onChunk: (chunk: Uint8Array) => boolean | void
): Promise<void> {
  assertActive(signal)
  if (!response.body) return protocolError()
  const reader = response.body.getReader()
  const cancel = () => {
    void reader.cancel().catch(() => {})
  }
  signal.addEventListener("abort", cancel, { once: true })
  try {
    while (true) {
      const result = await reader.read()
      assertActive(signal)
      if (result.done) return
      if (onChunk(result.value) === false) return
    }
  } catch (error) {
    assertActive(signal)
    throw safeExecutionError(error)
  } finally {
    signal.removeEventListener("abort", cancel)
    cancel()
    reader.releaseLock()
  }
}
export async function readBoundedJson(
  response: Response,
  signal: AbortSignal,
  maximum = 4 * 1024 * 1024
): Promise<unknown> {
  let bytes = 0
  const parts: string[] = []
  const decoder = new TextDecoder("utf-8", { fatal: true })
  await readChunks(response, signal, (chunk) => {
    bytes += chunk.byteLength
    if (bytes > maximum) return protocolError()
    parts.push(decoder.decode(chunk, { stream: true }))
  })
  assertActive(signal)
  try {
    parts.push(decoder.decode())
    return JSON.parse(parts.join("")) as unknown
  } catch {
    return protocolError()
  }
}
export async function requireSuccess(
  response: Response,
  signal: AbortSignal
): Promise<void> {
  if (response.status === 200) return
  const code = {
    400: "invalid",
    401: "unauthorized",
    403: "forbidden",
    404: "missing",
    409: "conflict",
    416: "invalid",
    429: "rate_limited",
  } as const
  const known = code[response.status as keyof typeof code]
  if ([401, 403, 404].includes(response.status) && known) {
    void response.body?.cancel().catch(() => {})
    throw new ExecutionError(known)
  }
  const body = await readBoundedJson(response, signal, 4096).catch(() => null)
  assertActive(signal)
  throw new ExecutionError(
    parseErrorCode(body) ?? known ?? "transport_unavailable"
  )
}
export function requirePrivateHeaders(response: Response): void {
  if (
    !response.headers
      .get("cache-control")
      ?.toLowerCase()
      .split(",")
      .some((part) => part.trim() === "no-store") ||
    response.headers.get("x-content-type-options")?.toLowerCase() !== "nosniff"
  )
    return protocolError()
}
export type StreamEnd =
  | { kind: "detached"; reason: DetachReason }
  | { kind: "interrupted" }

export async function readSessionStream(
  response: Response,
  input: EventsInput,
  signal: AbortSignal,
  onFrame: (frame: SessionFrame) => void
): Promise<StreamEnd> {
  await requireSuccess(response, signal)
  requirePrivateHeaders(response)
  if (
    !/^application\/x-ndjson\s*;\s*charset=utf-8$/i.test(
      response.headers.get("content-type") ?? ""
    )
  )
    return protocolError()
  const decoder = new SessionFrameDecoder()
  let initial = true
  let end: StreamEnd = { kind: "interrupted" }
  await readChunks(response, signal, (chunk) => {
    decoder.push(chunk, (value) => {
      assertActive(signal)
      const frame = parseSessionFrame(value, input)
      const handshake = frame.type === "snapshot" || frame.type === "replay"
      if (initial ? !handshake : handshake) return protocolError()
      initial = false
      if (frame.type === "detached")
        end = { kind: "detached", reason: frame.reason }
      onFrame(frame)
      if (frame.type === "detached") return false
    })
    if (end.kind === "detached") return false
  })
  assertActive(signal)
  if (initial) return protocolError()
  decoder.finish()
  // No reconnect, launch, write or prompt is issued here, even on an EOF.
  return end
}
