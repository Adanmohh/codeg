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
  type ProfileSummary,
  type SessionSummary,
} from "./types"

export function parseProfiles(
  value: unknown
): ExecutionResults["profiles/list"] {
  const row = record(value, ["profiles", "unavailableReason"])
  return {
    profiles: values(row.profiles).map((value): ProfileSummary => {
      const profile = record(value, [
        "id",
        "revision",
        "label",
        "clientId",
        "modes",
        "custody",
        "model",
        "readiness",
        "reason",
        "capabilities",
      ])
      const cap = record(profile.capabilities, [
        "start",
        "continue",
        "managedOutput",
        "officePreview",
      ])
      const model =
        profile.model === null
          ? null
          : record(profile.model, ["id", "reasoning"])
      return {
        id: text(profile.id),
        revision: integer(profile.revision),
        label: text(profile.label),
        clientId: text(profile.clientId),
        modes: values(profile.modes, 2).map((mode) =>
          choice(mode, ["chat", "terminal"])
        ),
        custody: choice(profile.custody, [
          "original_operator",
          "isolated_member",
        ]),
        model: model
          ? { id: text(model.id), reasoning: text(model.reasoning) }
          : null,
        readiness: choice(profile.readiness, ["ready", "blocked"]),
        reason:
          profile.reason === null
            ? null
            : choice(profile.reason, SETUP_REASONS),
        capabilities: {
          start: boolean(cap.start),
          continue: boolean(cap.continue),
          managedOutput: boolean(cap.managedOutput),
          officePreview: boolean(cap.officePreview),
        },
      }
    }),
    unavailableReason:
      row.unavailableReason === null
        ? null
        : choice(row.unavailableReason, SETUP_REASONS),
  }
}

export function parseSessions(
  value: unknown,
  taskId: string
): ExecutionResults["sessions/list"] {
  const row = record(value, ["items", "nextCursor"])
  return {
    items: values(row.items, 50).map((item) => parseSession(item, { taskId })),
    nextCursor: nullableCursor(row.nextCursor),
  }
}

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
