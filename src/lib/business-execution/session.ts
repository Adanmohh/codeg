import { parseMessagePart } from "./frames"
import { uniqueMessageParts } from "./conversation"
import {
  boolean,
  choice,
  ExecutionError,
  integer,
  nullableCursor,
  record,
  text,
  values,
} from "./protocol"
import {
  SESSION_STATUSES,
  SETUP_REASONS,
  type ExecutionResults,
  type SessionSummary,
} from "./types"

export function parseSession(
  value: unknown,
  expected: { taskId: string; sessionId?: string }
): SessionSummary {
  const row = record(value, [
    "id",
    "taskId",
    "profileId",
    "profileRevision",
    "revision",
    "generation",
    "status",
    "mode",
    "title",
    "createdAt",
    "updatedAt",
    "lastActivityAt",
    "capabilities",
    "reason",
  ])
  if (
    row.taskId !== expected.taskId ||
    (expected.sessionId && row.id !== expected.sessionId)
  )
    throw new ExecutionError("forbidden")
  const cap = record(row.capabilities, [
    "read",
    "prompt",
    "continue",
    "stop",
    "terminalWrite",
    "importOutput",
  ])
  return {
    id: text(row.id),
    taskId: text(row.taskId),
    profileId: text(row.profileId),
    profileRevision: integer(row.profileRevision),
    revision: integer(row.revision),
    generation: integer(row.generation),
    status: choice(row.status, SESSION_STATUSES),
    mode: choice(row.mode, ["chat", "terminal"]),
    title: text(row.title),
    createdAt: text(row.createdAt),
    updatedAt: text(row.updatedAt),
    lastActivityAt: text(row.lastActivityAt),
    capabilities: {
      read: boolean(cap.read),
      prompt: boolean(cap.prompt),
      continue: boolean(cap.continue),
      stop: boolean(cap.stop),
      terminalWrite: boolean(cap.terminalWrite),
      importOutput: boolean(cap.importOutput),
    },
    reason:
      row.reason === null
        ? null
        : choice(row.reason, [
            ...SETUP_REASONS,
            "authority_changed",
            "launch_uncertain",
            "process_gone",
          ] as const),
  }
}
export function parseHistory(
  value: unknown
): ExecutionResults["sessions/history"] {
  const row = record(value, ["messages", "nextBeforeCursor"])
  return {
    messages: uniqueMessageParts(
      values(row.messages, 40).map(parseMessagePart)
    ),
    nextBeforeCursor: nullableCursor(row.nextBeforeCursor),
  }
}
