"use client"

import { useEffect, useRef, useState } from "react"
import { Download, FileText } from "lucide-react"
import { Action } from "@/components/business/ui"
import type {
  AssetContentHandle,
  ExecutionClient,
} from "@/lib/business-execution/client"
import { supportsTextPreview } from "@/lib/business-execution/content"
import { useExecutionCopy } from "@/lib/business-execution/copy"
import type {
  ExecutionResults,
  Disposition,
} from "@/lib/business-execution/types"

// Same escaped-reader/verified-Blob seam as the owned published-asset.tsx.
// This component receives already checked private metadata; it never resolves
// file hints, host paths, external resources or a moving latest version.
export function PrivateAssetContent({
  client,
  asset,
  onError,
}: {
  client: ExecutionClient
  asset: ExecutionResults["assets/get"]
  onError: (error: unknown) => void
}) {
  const copy = useExecutionCopy()
  const lifetime = useRef<AbortController | null>(null)
  const handles = useRef(new Set<AssetContentHandle>())
  const reading = useRef(false)
  const [busy, setBusy] = useState(false)
  const [preview, setPreview] = useState<string | null>(null)
  const [download, setDownload] = useState<AssetContentHandle | null>(null)
  useEffect(() => {
    const owner = new AbortController()
    const owned = handles.current
    lifetime.current = owner
    return () => {
      owner.abort()
      lifetime.current = null
      owned.forEach((handle) => handle.dispose())
      owned.clear()
    }
  }, [client, asset.version.id])
  async function read(disposition: Disposition) {
    const owner = lifetime.current
    if (!owner || owner.signal.aborted || reading.current) return
    reading.current = true
    setBusy(true)
    try {
      const handle = await client.assetContent(
        { assetId: asset.asset.id, versionId: asset.version.id },
        asset.version,
        disposition,
        owner.signal
      )
      if (owner.signal.aborted) {
        handle.dispose()
        return
      }
      handles.current.add(handle)
      if (disposition === "preview") {
        const content = await handle.blob.text()
        if (!owner.signal.aborted) setPreview(content)
        handle.dispose()
        handles.current.delete(handle)
      } else {
        download?.dispose()
        if (download) handles.current.delete(download)
        setDownload(handle)
      }
    } catch (error) {
      if (!owner.signal.aborted) {
        handles.current.forEach((handle) => handle.dispose())
        handles.current.clear()
        setPreview(null)
        setDownload(null)
        onError(error)
      }
    } finally {
      if (!owner.signal.aborted) {
        reading.current = false
        setBusy(false)
      }
    }
  }
  const textPreview = supportsTextPreview(asset.version.mediaType)
  const canPreview =
    asset.capabilities.readContent &&
    asset.capabilities.preview &&
    textPreview &&
    asset.version.byteSize <= 1024 * 1024
  const canDownload =
    asset.capabilities.readContent && asset.capabilities.download
  return (
    <div className="space-y-4">
      <div className="flex flex-wrap gap-3">
        {canPreview && (
          <Action
            variant="outline"
            disabled={busy}
            onClick={() => void read("preview")}
          >
            <FileText aria-hidden="true" className="size-4" />
            {copy.readFile}
          </Action>
        )}
        {canDownload && (
          <Action
            variant="outline"
            disabled={busy}
            onClick={() => void read("download")}
          >
            <Download aria-hidden="true" className="size-4" />
            {copy.download}
          </Action>
        )}
      </div>
      {!canPreview && (
        <p className="text-muted-foreground text-sm leading-relaxed">
          {textPreview && asset.version.byteSize > 1024 * 1024
            ? copy.previewTooLarge
            : copy.privatePreviewUnavailable}
        </p>
      )}
      {busy && (
        <p role="status" className="text-sm">
          {copy.loadingFile}
        </p>
      )}
      {preview !== null && (
        <pre
          dir="auto"
          className="overflow-hidden rounded-xl border bg-card/40 p-5 font-sans text-base leading-relaxed whitespace-pre-wrap [overflow-wrap:anywhere]"
        >
          {preview}
        </pre>
      )}
      {download && (
        <div
          role="status"
          className="flex flex-wrap items-center gap-3 text-sm"
        >
          <p>{copy.verifiedDownload}</p>
          <Action asChild variant="outline">
            <a
              href={download.url}
              download={asset.asset.title.replace(
                /[\\/:*?"<>|\u0000-\u001f]/g,
                "_"
              )}
              referrerPolicy="no-referrer"
            >
              {copy.saveFile}
            </a>
          </Action>
        </div>
      )}
    </div>
  )
}
