// Pi 0.85.1 TypeBox tool pattern; UI Ops DTO contracts. See NOTICE.
import { Type, type Static } from "typebox"
import { Check } from "typebox/value"

const closed = { additionalProperties: false } as const
const id = Type.Integer({ minimum: 1, maximum: 2147483647 })
const revision = Type.Integer({ minimum: 0, maximum: 2147483647 })
const header = Type.String({ maxLength: 998, pattern: "^[^\\r\\n\\u0000]*$" })
const nullableHeader = Type.Union([header, Type.Null()])
const addresses = Type.Array(header, { maxItems: 50 })

export const replySchema = Type.Object({
  inboxId: id, conversationId: id, from: header,
  to: addresses, cc: addresses, bcc: addresses,
  subject: header, text: Type.String({ minLength: 1, maxLength: 100000 }),
  inReplyTo: nullableHeader, references: Type.Array(header, { maxItems: 100 }),
}, closed)

export const inputSchemas = {
  desk_context: Type.Object({}, closed),
  desk_tickets: Type.Object({
    inboxId: id, status: Type.Optional(Type.Union([Type.Integer({ minimum: 0, maximum: 3 }), Type.Null()])),
    page: Type.Optional(Type.Integer({ minimum: 0, maximum: 10000 })),
  }, closed),
  desk_thread: Type.Object({ inboxId: id, conversationId: id }, closed),
  desk_save_reply: Type.Object({
    inboxId: id, conversationId: id, expectedRevision: revision, reply: replySchema,
  }, closed),
  desk_propose_reply: Type.Object({ draftId: id, expectedRevision: id }, closed),
} as const

export type DeskTool = keyof typeof inputSchemas
export type DeskInputs = { [K in DeskTool]: Static<(typeof inputSchemas)[K]> }
export type DeskCall = { [K in DeskTool]: { tool: K; input: DeskInputs[K] } }[DeskTool]

export const DESK_TOOLS = Object.keys(inputSchemas) as DeskTool[]
export function isDeskTool(value: string): value is DeskTool {
  return Object.prototype.hasOwnProperty.call(inputSchemas, value)
}

/** Executed again after every mutable pi tool_call hook, immediately before IO. */
export function snapshotCall<K extends DeskTool>(tool: K, input: unknown): DeskCall {
  if (!Check(inputSchemas[tool], input)) throw new Error("Desk input is invalid")
  // JSON detaches every nested value; callers cannot substitute arguments while
  // an async guard/transport is pending. Backend performs its own validation.
  const copy = JSON.parse(JSON.stringify(input)) as DeskInputs[K]
  return { tool, input: copy } as DeskCall
}

export interface DeskContext {
  taskId: number
  runSeq: number
  accountId: number
  inboxes: Array<{ id: number; name: string; email: string }>
}

export function isDeskContext(value: unknown): value is DeskContext {
  return Check(Type.Object({
    taskId: id, runSeq: id, accountId: id,
    inboxes: Type.Array(Type.Object({ id, name: Type.String(), email: header }, closed), { maxItems: 1000 }),
  }, closed), value)
}

export type DeskErrorCode = "unavailable" | "denied" | "stale" | "invalid_input" | "storage"
export type DeskResponse =
  | { ok: true; value: unknown }
  | { ok: false; code: DeskErrorCode }

export function parseResponse(frame: unknown): DeskResponse {
  const schema = Type.Object({ outcome: Type.Union([
    Type.Object({ ok: Type.Literal(true), value: Type.Unknown() }, closed),
    Type.Object({ ok: Type.Literal(false), code: Type.Union([
      Type.Literal("unavailable"), Type.Literal("denied"), Type.Literal("stale"),
      Type.Literal("invalid_input"), Type.Literal("storage"),
    ]) }, closed),
  ]) }, closed)
  if (!Check(schema, frame)) throw new Error("Invalid Desk response")
  return frame.outcome
}
