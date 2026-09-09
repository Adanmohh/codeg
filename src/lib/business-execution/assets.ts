import { MAX_ASSET_BYTES } from "./content"
import {
  boolean,
  choice,
  ExecutionError,
  integer,
  nullableCursor,
  protocolError,
  record,
  text,
  values,
} from "./protocol"
import type {
  AssetSelection,
  AssetSummary,
  AssetVersion,
  ExecutionResults,
} from "./types"

// Closed private projection from the accepted E1 contract / 6195d9daf types.
// Public task readers deliberately use published.ts instead.
export function parseAssetSummary(
  value: unknown,
  taskId: string
): AssetSummary {
  const row = record(value, [
    "id",
    "taskId",
    "revision",
    "title",
    "mediaType",
    "latestVersionId",
    "createdAt",
    "updatedAt",
  ])
  if (row.taskId !== taskId) throw new ExecutionError("forbidden")
  return {
    id: text(row.id),
    taskId,
    revision: integer(row.revision),
    title: text(row.title),
    mediaType: text(row.mediaType),
    latestVersionId: text(row.latestVersionId),
    createdAt: text(row.createdAt),
    updatedAt: text(row.updatedAt),
  }
}
export function parseAssetVersion(
  value: unknown,
  expected: { taskId: string; assetId: string; versionId?: string }
): AssetVersion {
  const row = record(value, [
    "id",
    "assetId",
    "version",
    "sha256",
    "byteSize",
    "mediaType",
    "createdAt",
    "createdBy",
    "producer",
    "visibility",
    "reviewReferences",
  ])
  if (
    row.assetId !== expected.assetId ||
    (expected.versionId !== undefined && row.id !== expected.versionId)
  )
    throw new ExecutionError("content_changed")
  const author = record(row.createdBy, ["memberId", "displayName"])
  const producer = record(row.producer, [
    "sessionId",
    "turnId",
    "clientId",
    "model",
  ])
  const byteSize = integer(row.byteSize, 0)
  const sha256 = text(row.sha256)
  if (byteSize > MAX_ASSET_BYTES || !/^[a-f0-9]{64}$/.test(sha256))
    return protocolError()
  return {
    id: text(row.id),
    assetId: expected.assetId,
    version: integer(row.version),
    byteSize,
    sha256,
    mediaType: text(row.mediaType),
    createdAt: text(row.createdAt),
    createdBy: {
      memberId: text(author.memberId),
      displayName: text(author.displayName),
    },
    producer: {
      sessionId: text(producer.sessionId),
      turnId: nullableCursor(producer.turnId),
      clientId: text(producer.clientId),
      model: nullableCursor(producer.model),
    },
    visibility: choice(row.visibility, ["private", "task"]),
    reviewReferences: values(row.reviewReferences).map((value) => {
      const reference = record(value, [
        "taskId",
        "deliverableId",
        "taskRevision",
      ])
      if (reference.taskId !== expected.taskId)
        throw new ExecutionError("forbidden")
      return {
        taskId: expected.taskId,
        deliverableId: text(reference.deliverableId),
        taskRevision: integer(reference.taskRevision),
      }
    }),
  }
}
export function parseAssetPage(
  value: unknown,
  taskId: string
): ExecutionResults["assets/list"] {
  const row = record(value, ["items", "nextCursor"])
  return {
    items: values(row.items, 50).map((item) => parseAssetSummary(item, taskId)),
    nextCursor: nullableCursor(row.nextCursor),
  }
}
export function parseVersionPage(
  value: unknown,
  expected: { taskId: string; assetId: string }
): ExecutionResults["assets/versions"] {
  const row = record(value, ["items", "nextCursor"])
  return {
    items: values(row.items, 50).map((item) =>
      parseAssetVersion(item, expected)
    ),
    nextCursor: nullableCursor(row.nextCursor),
  }
}
export function parseAssetResult(
  value: unknown,
  expected: AssetSelection & { taskId: string }
): ExecutionResults["assets/get"] {
  const row = record(value, [
    "asset",
    "version",
    "capabilities",
    "previewReason",
  ])
  const asset = parseAssetSummary(row.asset, expected.taskId)
  if (asset.id !== expected.assetId) throw new ExecutionError("content_changed")
  const cap = record(row.capabilities, [
    "readContent",
    "preview",
    "download",
    "submit",
    "addVersion",
  ])
  return {
    asset,
    version: parseAssetVersion(row.version, expected),
    capabilities: {
      readContent: boolean(cap.readContent),
      preview: boolean(cap.preview),
      download: boolean(cap.download),
      submit: boolean(cap.submit),
      addVersion: boolean(cap.addVersion),
    },
    previewReason:
      row.previewReason === null
        ? null
        : choice(row.previewReason, ["unavailable"] as const),
  }
}
export function parseOutputPage(
  value: unknown
): ExecutionResults["outputs/list"] {
  const row = record(value, ["items", "nextCursor"])
  return {
    items: values(row.items, 50).map((value) => {
      const output = record(value, [
        "id",
        "revision",
        "name",
        "mediaType",
        "byteSize",
        "modifiedAt",
        "status",
      ])
      return {
        id: text(output.id),
        revision: integer(output.revision),
        name: text(output.name),
        mediaType: text(output.mediaType),
        byteSize: integer(output.byteSize, 0),
        modifiedAt: text(output.modifiedAt),
        status: choice(output.status, ["available", "changed", "unsupported"]),
      }
    }),
    nextCursor: nullableCursor(row.nextCursor),
  }
}
