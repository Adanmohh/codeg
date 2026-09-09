import { OPERATION_REASONS, type OperationReason } from "./types"

// Only allowlisted codes leave the transport; never carry a raw body/cause.
export class ExecutionError extends Error {
  constructor(readonly reason: OperationReason) {
    super(reason)
    this.name = "ExecutionError"
  }
}
// Local teardown does not prove that an admitted operation was cancelled.
// Keep its original receipt for explicit reconciliation; never replay a write.
export class ExecutionInterrupted extends ExecutionError {
  constructor(readonly kind: "aborted" | "closed" | "timeout") {
    super("transport_unavailable")
    this.name = "ExecutionInterrupted"
  }
}
export function protocolError(): never {
  throw new ExecutionError("transport_unavailable")
}
export function record(
  value: unknown,
  fields: readonly string[]
): Record<string, unknown> {
  if (!value || typeof value !== "object" || Array.isArray(value))
    return protocolError()
  const keys = Object.keys(value)
  if (
    keys.length !== fields.length ||
    !keys.every((key) => fields.includes(key))
  )
    return protocolError()
  return value as Record<string, unknown>
}
export function choice<T extends string>(
  value: unknown,
  choices: readonly T[]
): T {
  const match = choices.find((candidate) => candidate === value)
  return match ?? protocolError()
}
export function text(value: unknown, allowEmpty = false): string {
  return typeof value === "string" && (allowEmpty || value.length > 0)
    ? value
    : protocolError()
}
export function integer(value: unknown, minimum = 1): number {
  return typeof value === "number" &&
    Number.isSafeInteger(value) &&
    value >= minimum
    ? value
    : protocolError()
}
export function boolean(value: unknown): boolean {
  return typeof value === "boolean" ? value : protocolError()
}
export function nullableCursor(value: unknown): string | null {
  return value === null ? null : text(value)
}
export function values(value: unknown, maximum?: number): unknown[] {
  if (
    !Array.isArray(value) ||
    (maximum !== undefined && value.length > maximum)
  )
    return protocolError()
  return value
}
export function boundedText(value: unknown, maxBytes: number): string {
  const result = text(value, true)
  if (new TextEncoder().encode(result).byteLength > maxBytes)
    return protocolError()
  return result
}
export function safeExecutionError(error: unknown): ExecutionError {
  return error instanceof ExecutionError
    ? error
    : new ExecutionError("transport_unavailable")
}
export function parseErrorCode(value: unknown): OperationReason | null {
  if (!value || typeof value !== "object" || !("error" in value)) return null
  const nested = value.error
  if (!nested || typeof nested !== "object" || !("code" in nested)) return null
  return OPERATION_REASONS.find((code) => code === nested.code) ?? null
}
