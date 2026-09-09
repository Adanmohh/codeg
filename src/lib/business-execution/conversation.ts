import { ExecutionError } from "./protocol"
import type {
  MessagePart,
  SessionEvent,
  SessionFrame,
  SessionStatus,
  ToolState,
} from "./types"

export interface ConversationState {
  messages: MessagePart[]
  tools: ToolState[]
  status: SessionStatus
  cursor: string | null
  olderCursor: string | null
  trimmed: boolean
  seen: string[]
}
export function emptyConversation(status: SessionStatus): ConversationState {
  return {
    messages: [],
    tools: [],
    status,
    cursor: null,
    olderCursor: null,
    trimmed: false,
    seen: [],
  }
}
export function messageKey(part: MessagePart): string {
  return `${part.messageId}:${part.part}`
}
export function uniqueMessageParts(parts: MessagePart[]): MessagePart[] {
  const items = new Map<string, MessagePart>()
  const roles = new Map<string, MessagePart["role"]>()
  for (const part of parts) {
    const role = roles.get(part.messageId)
    if (role && role !== part.role)
      throw new ExecutionError("transport_unavailable")
    roles.set(part.messageId, part.role)
    items.set(messageKey(part), part)
  }
  return [...items.values()]
}
function applyEvent(
  state: ConversationState,
  event: SessionEvent
): ConversationState {
  if (event.kind === "status") return { ...state, status: event.status }
  if (event.kind === "terminal") return state // never render terminal bytes in chat/task review
  if (event.kind === "tool") {
    const tools = [...state.tools]
    const index = tools.findIndex((tool) => tool.id === event.tool.id)
    if (index < 0) tools.push(event.tool)
    else tools[index] = event.tool
    return { ...state, tools: tools.slice(-40) }
  }
  const messages = uniqueMessageParts([...state.messages, event.message])
  return {
    ...state,
    messages: messages.slice(-160),
    trimmed: state.trimmed || messages.length > 160,
  }
}

// Presentation cache only; server-issued cursors remain opaque. No prompt,
// authority, provider lifecycle or task status is derived from this cache.
export function applyConversationFrame(
  state: ConversationState,
  frame: SessionFrame
): ConversationState {
  if (frame.type === "snapshot")
    return {
      ...emptyConversation(frame.state.status),
      messages: uniqueMessageParts(frame.state.messages),
      tools: frame.state.tools,
      cursor: frame.cursor,
      olderCursor: frame.olderCursor,
      seen: [frame.cursor],
    }
  if (frame.type === "detached") return state
  if (state.seen.includes(frame.cursor)) return state
  let next = state
  if (frame.type === "event") next = applyEvent(next, frame.event)
  if (frame.type === "replay")
    for (const event of frame.events) next = applyEvent(next, event)
  return {
    ...next,
    cursor: frame.cursor,
    seen: [...next.seen, frame.cursor].slice(-1024),
  }
}
