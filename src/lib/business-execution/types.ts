import type { Deliverable, TaskDetail } from "@/lib/business/tasks"

// Accepted business-ai-execution.md, 3f164c2a989cd08a523e51f1e47559c15a48c0ef.
// Additive private-version/prompt wrappers: compiling business_execution/types.rs
// and contract at 6195d9daf. No runtime operation is implied by a DTO.
// IDs/cursors remain opaque strings; no host IDs, credentials or paths on wire.
export const SESSION_STATUSES = [
  "starting",
  "idle",
  "running",
  "awaiting_input",
  "stopped",
  "failed",
  "interrupted",
  "revoked",
  "closed",
] as const
export type SessionStatus = (typeof SESSION_STATUSES)[number]
export type SessionMode = "chat" | "terminal"
export const SETUP_REASONS = [
  "missing_client",
  "missing_configuration",
  "model_unavailable",
  "profile_unavailable",
  "tenant_execution_unavailable",
  "native_boundary_unavailable",
  "client_resume_unsupported",
] as const
export type SetupReason = (typeof SETUP_REASONS)[number]
export const OPERATION_REASONS = [
  "invalid",
  "unauthorized",
  "forbidden",
  "missing",
  "conflict",
  "busy",
  "unavailable",
  "setup_required",
  "authority_changed",
  "cancelled",
  "launch_uncertain",
  "prompt_uncertain",
  "content_changed",
  "content_unavailable",
  "transport_unavailable",
  "rate_limited",
] as const
export type OperationReason = (typeof OPERATION_REASONS)[number]
export const OPERATION_STATUSES = [
  "pending",
  "confirmed",
  "failed",
  "uncertain",
] as const
export interface OperationSummary {
  id: string
  status: (typeof OPERATION_STATUSES)[number]
  reason: OperationReason | null
}
export type OperationKind =
  | "start"
  | "continue"
  | "prompt"
  | "attach"
  | "stop"
  | "terminal_write"
  | "import_output"
  | "submit"
export interface ProfileSummary {
  id: string
  revision: number
  label: string
  clientId: string
  modes: SessionMode[]
  custody: "original_operator" | "isolated_member"
  model: { id: string; reasoning: string } | null
  readiness: "ready" | "blocked"
  reason: SetupReason | null
  capabilities: {
    start: boolean
    continue: boolean
    managedOutput: boolean
    officePreview: boolean
  }
}
export interface SessionSummary {
  id: string
  taskId: string
  profileId: string
  profileRevision: number
  revision: number
  generation: number
  status: SessionStatus
  mode: SessionMode
  title: string
  createdAt: string
  updatedAt: string
  lastActivityAt: string
  capabilities: {
    read: boolean
    prompt: boolean
    continue: boolean
    stop: boolean
    terminalWrite: boolean
    importOutput: boolean
  }
  reason:
    | SetupReason
    | "authority_changed"
    | "launch_uncertain"
    | "process_gone"
    | null
}
export type InputRef =
  | { kind: "asset"; assetId: string; versionId: string }
  | { kind: "task"; taskId: string; expectedRevision: number }
  | { kind: "account_snapshot"; snapshotId: string; expectedRevision: number }
// E1 refuses the future account resolver before any mutation.
export type PromptInputRef = Exclude<InputRef, { kind: "account_snapshot" }>
export interface OutputCandidate {
  id: string
  revision: number
  name: string
  mediaType: string
  byteSize: number
  modifiedAt: string
  status: "available" | "changed" | "unsupported"
}
export interface AssetSummary {
  id: string
  taskId: string
  revision: number
  title: string
  mediaType: string
  latestVersionId: string
  createdAt: string
  updatedAt: string
}
export interface AssetVersion {
  id: string
  assetId: string
  version: number
  sha256: string
  byteSize: number
  mediaType: string
  createdAt: string
  createdBy: { memberId: string; displayName: string }
  producer: {
    sessionId: string
    turnId: string | null
    clientId: string
    model: string | null
  }
  visibility: "private" | "task"
  reviewReferences: {
    taskId: string
    deliverableId: string
    taskRevision: number
  }[]
}
export interface PublishedAssetRef {
  assetId: string
  versionId: string
  title: string
  mediaType: string
  byteSize: number
  sha256: string
}
export interface PublishedAsset {
  version: PublishedAssetRef
  createdAt: string
  producer: { clientId: string; model: string | null }
  publication: {
    deliverableId: string
    taskRevision: number
    submittedBy: {
      memberId: string
      displayName: string
      authorityKind: "operator" | "credential"
    }
  }
}
// Isolated until the backend's additive Deliverable.assets and PR29 integrate.
export type AssetTaskDetail = Omit<TaskDetail, "deliverables"> & {
  deliverables: (Deliverable & { assets: PublishedAssetRef[] })[]
}
export interface MessagePart {
  messageId: string
  role: "user" | "assistant"
  part: number
  text: string
  complete: boolean
}
export interface ToolState {
  id: string
  name: string
  status: "running" | "completed" | "failed"
}
export type SessionEvent =
  | { kind: "message"; message: MessagePart }
  | { kind: "tool"; tool: ToolState }
  | { kind: "status"; status: SessionStatus }
  | { kind: "terminal"; data: string }
export const DETACH_REASONS = [
  "authority_changed",
  "generation_changed",
  "process_gone",
  "lagged",
  "server_shutdown",
] as const
export type DetachReason = (typeof DETACH_REASONS)[number]
interface FrameBinding {
  sessionId: string
  generation: number
}
export type SessionFrame = FrameBinding &
  (
    | {
        type: "snapshot"
        operation: OperationSummary
        cursor: string
        reset: true
        reason: "initial" | "cursor_expired" | "cursor_invalid"
        state: {
          status: SessionStatus
          messages: MessagePart[]
          tools: ToolState[]
        }
        olderCursor: string | null
      }
    | {
        type: "replay"
        operation: OperationSummary
        cursor: string
        reset: false
        events: SessionEvent[]
      }
    | { type: "event"; cursor: string; event: SessionEvent }
    | { type: "heartbeat"; cursor: string }
    | { type: "detached"; reason: DetachReason }
  )
export interface EventsInput {
  operationId: string
  sessionId: string
  expectedGeneration: number
  cursor: string | null
}
export type Disposition = "preview" | "download"
export interface AssetSelection {
  assetId: string
  versionId: string
}
export interface PublishedSelection extends AssetSelection {
  taskId: string
  deliverableId: string
}
export interface ExecutionInputs {
  "operations/get": { operationId: string; kind: OperationKind }
  "profiles/list": { taskId: string }
  "sessions/list": { taskId: string; cursor: string | null; limit: number }
  "sessions/start": {
    operationId: string
    taskId: string
    expectedTaskRevision: number
    profileId: string
    expectedProfileRevision: number
    mode: SessionMode
  }
  "sessions/get": { sessionId: string }
  "sessions/continue": {
    operationId: string
    sessionId: string
    expectedSessionRevision: number
  }
  "sessions/prompt": {
    operationId: string
    sessionId: string
    expectedSessionRevision: number
    text: string
    inputs: InputRef[]
  }
  "sessions/stop": ExecutionInputs["sessions/continue"]
  "sessions/events": EventsInput
  "sessions/history": {
    sessionId: string
    beforeCursor: string | null
    limit: number
  }
  "sessions/terminal-write": {
    operationId: string
    sessionId: string
    expectedGeneration: number
    data: string
  }
  "sessions/terminal-resize": {
    sessionId: string
    expectedGeneration: number
    cols: number
    rows: number
  }
  "outputs/list": { sessionId: string; cursor: string | null; limit: number }
  "assets/import-output": {
    operationId: string
    sessionId: string
    outputId: string
    expectedOutputRevision: number
    title: string
    assetId: string | null
    expectedAssetRevision: number | null
  }
  "assets/list": {
    taskId: string
    query: string | null
    cursor: string | null
    limit: number
  }
  "assets/get": AssetSelection
  "assets/versions": { assetId: string; cursor: string | null; limit: number }
  "assets/content": AssetSelection & { disposition: Disposition }
  "assets/submit": {
    operationId: string
    taskId: string
    expectedTaskRevision: number
    versions: AssetSelection[]
    body: string
  }
}
interface SessionResult {
  session: SessionSummary
  operation: OperationSummary
}
// Only specified JSON envelopes: other wrappers await compiling backend DTOs.
export interface ExecutionResults {
  "operations/get": {
    operation: OperationSummary
    resourceId: string | null
  }
  "profiles/list": {
    profiles: ProfileSummary[]
    unavailableReason: SetupReason | null
  }
  "sessions/list": { items: SessionSummary[]; nextCursor: string | null }
  "sessions/start": SessionResult
  "sessions/get": { session: SessionSummary }
  "sessions/continue": SessionResult
  "sessions/prompt": {
    operation: OperationSummary
    messageId: string
    inputHash: string
  }
  "sessions/history": {
    messages: MessagePart[]
    nextBeforeCursor: string | null
  }
  "outputs/list": { items: OutputCandidate[]; nextCursor: string | null }
  "assets/import-output": {
    asset: AssetSummary
    version: AssetVersion
    operation: OperationSummary
  }
  "assets/list": { items: AssetSummary[]; nextCursor: string | null }
  "assets/versions": { items: AssetVersion[]; nextCursor: string | null }
  "assets/get": {
    asset: AssetSummary
    version: AssetVersion
    capabilities: {
      readContent: boolean
      preview: boolean
      download: boolean
      submit: boolean
      addVersion: boolean
    }
    previewReason: "unavailable" | null
  }
  "assets/submit": { detail: AssetTaskDetail; operation: OperationSummary }
}
export type JsonOperation = keyof ExecutionResults
export interface PublishedOperations {
  "deliverables/assets/get": {
    input: PublishedSelection
    result: PublishedAsset
  }
}
