import { MAX_ASSET_BYTES } from "./content"
import {
  choice,
  ExecutionError,
  integer,
  protocolError,
  record,
  text,
} from "./protocol"
import type { PublishedAsset, PublishedSelection } from "./types"

/** Public task projection only. Never accept the richer private AssetVersion. */
export function parsePublishedAsset(
  value: unknown,
  expected: PublishedSelection
): PublishedAsset {
  const row = record(value, ["version", "createdAt", "producer", "publication"])
  const version = record(row.version, [
    "assetId",
    "versionId",
    "title",
    "mediaType",
    "byteSize",
    "sha256",
  ])
  const producer = record(row.producer, ["clientId", "model"])
  const publication = record(row.publication, [
    "deliverableId",
    "taskRevision",
    "submittedBy",
  ])
  const author = record(publication.submittedBy, [
    "memberId",
    "displayName",
    "authorityKind",
  ])
  if (
    version.assetId !== expected.assetId ||
    version.versionId !== expected.versionId ||
    publication.deliverableId !== expected.deliverableId
  )
    throw new ExecutionError("content_changed")
  const byteSize = integer(version.byteSize, 0)
  const sha256 = text(version.sha256)
  if (byteSize > MAX_ASSET_BYTES || !/^[a-f0-9]{64}$/.test(sha256))
    return protocolError()
  return {
    version: {
      assetId: text(version.assetId),
      versionId: text(version.versionId),
      title: text(version.title),
      mediaType: text(version.mediaType),
      byteSize,
      sha256,
    },
    createdAt: text(row.createdAt),
    producer: {
      clientId: text(producer.clientId),
      model: producer.model === null ? null : text(producer.model),
    },
    publication: {
      deliverableId: text(publication.deliverableId),
      taskRevision: integer(publication.taskRevision),
      submittedBy: {
        memberId: text(author.memberId),
        displayName: text(author.displayName),
        authorityKind: choice(author.authorityKind, ["operator", "credential"]),
      },
    },
  }
}
