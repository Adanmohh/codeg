import type {
  AssetSummary,
  AssetVersion,
  ExecutionResults,
  OutputCandidate,
} from "@/lib/business-execution/types"

export const fixtureTaskId = "synthetic-task"
export function retained(
  assetId = "synthetic-asset",
  versionId = "synthetic-version",
  taskId = fixtureTaskId
): ExecutionResults["assets/get"] {
  const asset: AssetSummary = {
    id: assetId,
    taskId,
    revision: 2,
    title: "Synthetic launch brief.md",
    mediaType: "text/markdown",
    latestVersionId: versionId,
    createdAt: "2026-09-09T00:00:00Z",
    updatedAt: "2026-09-09T00:00:00Z",
  }
  const version: AssetVersion = {
    id: versionId,
    assetId,
    version: 1,
    sha256: "a".repeat(64),
    byteSize: 17,
    mediaType: "text/markdown",
    createdAt: "2026-09-09T00:00:00Z",
    createdBy: {
      memberId: "synthetic-person",
      displayName: "Synthetic operator",
    },
    producer: {
      sessionId: "synthetic-session",
      turnId: null,
      clientId: "synthetic-client",
      model: null,
    },
    visibility: "private",
    reviewReferences: [],
  }
  return {
    asset,
    version,
    capabilities: {
      readContent: true,
      preview: true,
      download: true,
      submit: true,
      addVersion: true,
    },
    previewReason: null,
  }
}
export function outputCandidate(): OutputCandidate {
  return {
    id: "synthetic-output",
    revision: 3,
    name: "Synthetic launch brief.md",
    mediaType: "text/markdown",
    byteSize: 17,
    modifiedAt: "2026-09-09T00:00:00Z",
    status: "available",
  }
}
