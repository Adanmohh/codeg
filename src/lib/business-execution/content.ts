import { ExecutionError, protocolError } from "./protocol"
import {
  assertActive,
  readChunks,
  requirePrivateHeaders,
  requireSuccess,
} from "./reader"
import type { Disposition } from "./types"

export const MAX_ASSET_BYTES = 50 * 1024 * 1024
export interface ContentMetadata {
  mediaType: string
  byteSize: number
  sha256: string
}
// The first reader is escaped text. Other managed formats remain downloadable;
// the inherited path/watch OfficePreview is not a version-authorized renderer.
export function supportsTextPreview(mediaType: string): boolean {
  return ["text/plain", "text/markdown"].includes(mediaType.toLowerCase())
}
export async function readAssetContent(
  response: Response,
  selection: ContentMetadata,
  disposition: Disposition,
  signal: AbortSignal
): Promise<Blob> {
  const expected = { ...selection }
  await requireSuccess(response, signal)
  requirePrivateHeaders(response)
  if (
    !Number.isSafeInteger(expected.byteSize) ||
    expected.byteSize < 0 ||
    expected.byteSize > MAX_ASSET_BYTES ||
    !/^[a-f0-9]{64}$/.test(expected.sha256)
  )
    return protocolError()
  if (disposition === "preview" && !supportsTextPreview(expected.mediaType))
    throw new ExecutionError("unavailable")
  const type = response.headers.get("content-type")?.split(";")[0].trim()
  const headerDisposition = response.headers
    .get("content-disposition")
    ?.split(";")[0]
    .trim()
  if (
    response.headers.get("content-length") !== String(expected.byteSize) ||
    response.headers.get("etag") !== `"sha256-${expected.sha256}"` ||
    type?.toLowerCase() !== expected.mediaType.toLowerCase() ||
    headerDisposition !== (disposition === "preview" ? "inline" : "attachment")
  )
    throw new ExecutionError("content_changed")
  if (!globalThis.crypto?.subtle) throw new ExecutionError("unavailable")
  const bytes = new Uint8Array(expected.byteSize)
  let offset = 0
  await readChunks(response, signal, (chunk) => {
    if (offset + chunk.byteLength > bytes.byteLength)
      throw new ExecutionError("content_changed")
    bytes.set(chunk, offset)
    offset += chunk.byteLength
  })
  if (offset !== expected.byteSize) throw new ExecutionError("content_changed")
  const digest = await crypto.subtle.digest("SHA-256", bytes)
  assertActive(signal)
  const hash = Array.from(new Uint8Array(digest), (byte) =>
    byte.toString(16).padStart(2, "0")
  ).join("")
  if (hash !== expected.sha256) throw new ExecutionError("content_changed")
  // Never mint an application-origin executable HTML/SVG Blob URL. Download
  // bytes are unchanged, but the browser gets an inert binary MIME type.
  return new Blob([bytes], {
    type: disposition === "preview" ? "text/plain" : "application/octet-stream",
  })
}
