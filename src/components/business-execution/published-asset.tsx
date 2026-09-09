"use client"

import { useEffect, useRef, useState } from "react"
import { Download, FileText } from "lucide-react"
import { Action } from "@/components/business/ui"
import type { BusinessClient } from "@/lib/business/client"
import type { Deliverable, PublishedAssetRef } from "@/lib/business/tasks"
import type {
  AssetContentHandle,
  ExecutionClient,
} from "@/lib/business-execution/client"
import { supportsTextPreview } from "@/lib/business-execution/content"
import {
  executionErrorMessage,
  useExecutionCopy,
} from "@/lib/business-execution/copy"
import { ExecutionError } from "@/lib/business-execution/protocol"
import type {
  Disposition,
  PublishedAsset,
  PublishedSelection,
} from "@/lib/business-execution/types"

export interface PublishedFileIntent {
  selection: PublishedSelection
  expected: PublishedAssetRef
}

// Adapted from ReplyArtifacts' file-card framing, with selected IDs instead of
// file hints, host paths, reveal-in-folder or a global file opener.
export function PublishedAssetCards({
  taskId,
  deliverable,
  onOpen,
}: {
  taskId: string
  deliverable: Deliverable
  onOpen: (intent: PublishedFileIntent) => void
}) {
  const copy = useExecutionCopy()
  // Older text-only Task Detail responses precede the additive asset field.
  const assets = deliverable.assets ?? []
  if (assets.length === 0) return null
  return (
    <section aria-label={copy.submittedFiles} className="space-y-2">
      <h4 className="text-sm font-medium">{copy.submittedFiles}</h4>
      <div className="grid gap-2">
        {assets.map((asset) => (
          <button
            key={`${asset.assetId}:${asset.versionId}`}
            type="button"
            onClick={() =>
              onOpen({
                selection: {
                  taskId,
                  deliverableId: deliverable.id,
                  assetId: asset.assetId,
                  versionId: asset.versionId,
                },
                expected: { ...asset },
              })
            }
            className="bg-card/40 hover:bg-accent/40 focus-visible:ring-ring flex min-h-14 min-w-0 items-center gap-3 rounded-xl border p-3 text-start transition-colors focus-visible:ring-2 focus-visible:outline-none motion-reduce:transition-none"
          >
            <FileText
              aria-hidden="true"
              className="text-muted-foreground size-5 shrink-0"
            />
            <span className="min-w-0 flex-1 space-y-1">
              <bdi className="block text-sm font-medium [overflow-wrap:anywhere]">
                {asset.title}
              </bdi>
              <span className="text-muted-foreground block text-xs">
                {asset.byteSize} {copy.bytes}
              </span>
            </span>
          </button>
        ))}
      </div>
    </section>
  )
}

const MAX_TEXT_PREVIEW = 1024 * 1024
interface Scope {
  client: ExecutionClient
  lifetime: AbortController
}

export function PublishedAssetPane({
  client,
  intent,
}: {
  client: BusinessClient
  intent: PublishedFileIntent
}) {
  // Exact version changes dispose private handles. Locale/pane visibility do not.
  const key = JSON.stringify([intent.selection, intent.expected])
  return <PublishedReader key={key} client={client} intent={intent} />
}

function PublishedReader({
  client,
  intent,
}: {
  client: BusinessClient
  intent: PublishedFileIntent
}) {
  const copy = useExecutionCopy()
  const [{ selection, expected }] = useState(() => ({
    selection: { ...intent.selection },
    expected: { ...intent.expected },
  }))
  const scope = useRef<Scope | null>(null)
  const handles = useRef(new Set<AssetContentHandle>())
  const reading = useRef(false)
  const [metadata, setMetadata] = useState<PublishedAsset | null>(null)
  const [preview, setPreview] = useState<string | null>(null)
  const [download, setDownload] = useState<AssetContentHandle | null>(null)
  const [busy, setBusy] = useState(true)
  const [error, setError] = useState<unknown>(null)
  const [reload, setReload] = useState(0)

  useEffect(() => {
    const lifetime = new AbortController()
    const ownedHandles = handles.current
    let child: ExecutionClient | null = null
    setBusy(true)
    setMetadata(null)
    setPreview(null)
    setDownload(null)
    setError(null)
    reading.current = false
    try {
      child = client.execution()
      scope.current = { client: child, lifetime }
      void child
        .publishedAsset(selection, lifetime.signal)
        .then((value) => {
          if (lifetime.signal.aborted) return
          if (
            value.version.sha256 !== expected.sha256 ||
            value.version.byteSize !== expected.byteSize ||
            value.version.mediaType !== expected.mediaType
          )
            throw new ExecutionError("content_changed")
          setMetadata(value)
        })
        .catch((caught: unknown) => {
          if (!lifetime.signal.aborted) setError(caught)
        })
        .finally(() => {
          if (!lifetime.signal.aborted) setBusy(false)
        })
    } catch (caught) {
      setError(caught)
      setBusy(false)
    }
    return () => {
      lifetime.abort()
      child?.close()
      ownedHandles.forEach((handle) => handle.dispose())
      ownedHandles.clear()
      scope.current = null
    }
  }, [client, selection, expected, reload])

  async function read(disposition: Disposition) {
    const owner = scope.current
    if (!owner || owner.lifetime.signal.aborted || !metadata || reading.current)
      return
    reading.current = true
    setBusy(true)
    setError(null)
    try {
      const handle = await owner.client.publishedContent(
        selection,
        metadata.version,
        disposition,
        owner.lifetime.signal
      )
      if (owner.lifetime.signal.aborted) {
        handle.dispose()
        return
      }
      handles.current.add(handle)
      if (disposition === "preview") {
        const text = await handle.blob.text()
        if (!owner.lifetime.signal.aborted) setPreview(text)
        handle.dispose()
        handles.current.delete(handle)
      } else {
        download?.dispose()
        if (download) handles.current.delete(download)
        setDownload(handle)
      }
    } catch (caught) {
      if (!owner.lifetime.signal.aborted) {
        handles.current.forEach((handle) => handle.dispose())
        handles.current.clear()
        setPreview(null)
        setDownload(null)
        setMetadata(null)
        setError(caught)
      }
    } finally {
      if (!owner.lifetime.signal.aborted) {
        reading.current = false
        setBusy(false)
      }
    }
  }
  const canPreview =
    supportsTextPreview(expected.mediaType) &&
    expected.byteSize <= MAX_TEXT_PREVIEW
  return (
    <article
      aria-label={copy.selectedFile}
      className="mx-auto max-w-4xl space-y-5 p-5 [overflow-wrap:anywhere] sm:p-7"
    >
      <header className="space-y-3 border-b pb-5">
        <p className="text-muted-foreground text-sm">{copy.submittedVersion}</p>
        <h2 className="text-xl font-semibold tracking-tight">
          <bdi>{expected.title}</bdi>
        </h2>
        <p className="text-muted-foreground text-sm">
          <bdi>{expected.mediaType}</bdi> · {expected.byteSize} {copy.bytes}
        </p>
      </header>
      {busy && (
        <p role="status" className="text-sm">
          {metadata ? copy.loadingFile : copy.checkingFile}
        </p>
      )}
      {error != null && (
        <div role="alert" className="space-y-3 rounded-xl border p-4 text-sm">
          <p>
            {error instanceof ExecutionError && error.reason === "unavailable"
              ? copy.fileUnavailable
              : executionErrorMessage(error, copy)}
          </p>
          <Action
            variant="outline"
            disabled={busy}
            onClick={() => setReload((value) => value + 1)}
          >
            {copy.retry}
          </Action>
        </div>
      )}
      {metadata && (
        <>
          <div className="flex flex-wrap items-center gap-3">
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
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => void read("download")}
            >
              <Download aria-hidden="true" className="size-4" />
              {copy.download}
            </Action>
          </div>
          {!canPreview && (
            <p className="text-muted-foreground text-sm leading-relaxed">
              {supportsTextPreview(expected.mediaType)
                ? copy.previewTooLarge
                : copy.previewUnavailable}
            </p>
          )}
          {preview !== null && (
            <pre
              dir="auto"
              className="max-w-full overflow-hidden rounded-xl border bg-card/40 p-5 font-sans text-base leading-relaxed whitespace-pre-wrap [overflow-wrap:anywhere] sm:p-7"
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
                  download={expected.title.replace(
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
          <details className="rounded-xl border p-4">
            <summary className="focus-visible:ring-ring cursor-pointer rounded py-2 text-sm font-medium focus-visible:ring-2 focus-visible:outline-none">
              {copy.fileInformation}
            </summary>
            <dl className="mt-3 grid gap-3 text-sm sm:grid-cols-2">
              <div>
                <dt className="text-muted-foreground">{copy.submittedBy}</dt>
                <dd>
                  <bdi>{metadata.publication.submittedBy.displayName}</bdi>
                </dd>
              </div>
              <div>
                <dt className="text-muted-foreground">{copy.taskRevision}</dt>
                <dd>{metadata.publication.taskRevision}</dd>
              </div>
              <div>
                <dt className="text-muted-foreground">{copy.createdWith}</dt>
                <dd>
                  <bdi>{metadata.producer.clientId}</bdi>
                  <br />
                  <bdi>{metadata.producer.model ?? copy.modelUnavailable}</bdi>
                </dd>
              </div>
              <div>
                <dt className="text-muted-foreground">SHA-256</dt>
                <dd dir="ltr" className="break-all font-mono text-xs">
                  {metadata.version.sha256}
                </dd>
              </div>
            </dl>
          </details>
        </>
      )}
    </article>
  )
}
