"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import { FileText, Search } from "lucide-react"
import { Action, controlClass, Field } from "@/components/business/ui"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import type { Member } from "@/lib/business/identity"
import type { Task, TaskDetail } from "@/lib/business/tasks"
import {
  parseAssetPage,
  parseAssetResult,
  parseOutputPage,
  parseVersionPage,
} from "@/lib/business-execution/assets"
import type { ExecutionClient } from "@/lib/business-execution/client"
import { MAX_ASSET_BYTES } from "@/lib/business-execution/content"
import {
  executionErrorMessage,
  useExecutionCopy,
} from "@/lib/business-execution/copy"
import { ExecutionError, record } from "@/lib/business-execution/protocol"
import { parseSession } from "@/lib/business-execution/session"
import type {
  AssetSelection,
  ExecutionResults,
  OutputCandidate,
  SessionSummary,
} from "@/lib/business-execution/types"
import { PrivateAssetContent } from "./asset-content"
import {
  AssetSubmission,
  selectionKey,
  type SelectedVersion,
} from "./asset-submission"
import { ImportOutputDialog } from "./import-output"
import type { PromptGuard } from "./session-prompt"

interface Props {
  business: BusinessClient
  originalOperator: boolean
  taskId: string
  sessionId: string | null
  members: Member[]
  onGuard: (guard: PromptGuard) => void
  onPublished: (detail: TaskDetail) => void
  onTask: () => void
}
interface Data {
  owner: BusinessClient
  client: ExecutionClient
  task: Task
  session: SessionSummary | null
  assets: ExecutionResults["assets/list"]
}
export function TaskFilesWorkspace(props: Props) {
  const copy = useExecutionCopy()
  if (!props.originalOperator || props.business.native)
    return (
      <p role="alert" className="p-5 text-sm">
        {copy.operatorOnly}
      </p>
    )
  return <FilesLoader key={`${props.taskId}:${props.sessionId}`} {...props} />
}
function FilesLoader(props: Props) {
  const { business, taskId, sessionId } = props
  const copy = useExecutionCopy()
  const [data, setData] = useState<Data | null>(null)
  const [error, setError] = useState<unknown>(null)
  useEffect(() => {
    const owner = new AbortController()
    let child: ExecutionClient | null = null
    async function load() {
      const detail = await business.tasks("get", { taskId })
      if (owner.signal.aborted) return
      if (detail.task.id !== taskId) throw new ExecutionError("forbidden")
      const scope = business.execution()
      child = scope
      const [assets, saved] = await Promise.all([
        scope.json(
          "assets/list",
          { taskId, query: null, cursor: null, limit: 50 },
          owner.signal
        ),
        sessionId
          ? scope.json("sessions/get", { sessionId }, owner.signal)
          : Promise.resolve(null),
      ])
      if (owner.signal.aborted) return
      const session =
        saved && sessionId
          ? parseSession(record(saved, ["session"]).session, {
              taskId,
              sessionId,
            })
          : null
      if (
        session &&
        (!session.capabilities.read || session.status === "revoked")
      )
        throw new ExecutionError("forbidden")
      setData({
        owner: business,
        client: scope,
        task: detail.task,
        session,
        assets: parseAssetPage(assets, taskId),
      })
    }
    void load().catch((caught: unknown) => {
      if (!owner.signal.aborted)
        setError(
          caught instanceof BusinessError &&
            ["unauthorized", "forbidden", "missing", "closed"].includes(
              caught.kind
            )
            ? new ExecutionError("forbidden")
            : caught
        )
    })
    return () => {
      owner.abort()
      child?.close()
    }
  }, [business, taskId, sessionId])
  if (error != null)
    return (
      <p role="alert" className="p-5 text-sm">
        {executionErrorMessage(error, copy)}
      </p>
    )
  if (!data || data.owner !== business)
    return (
      <p role="status" className="p-5 text-sm">
        {copy.loadingAssets}
      </p>
    )
  return <FileLibrary {...props} data={data} />
}
const cleanGuard: PromptGuard = { dirty: false, busy: false, unresolved: false }
function FileLibrary({
  data,
  business,
  members,
  onGuard,
  onPublished,
  onTask,
}: Props & { data: Data }) {
  const copy = useExecutionCopy()
  const [assets, setAssets] = useState(data.assets)
  const [search, setSearch] = useState("")
  const [query, setQuery] = useState("")
  const [listing, setListing] = useState(false)
  const [selected, setSelected] = useState<
    ExecutionResults["assets/get"] | null
  >(null)
  const [versions, setVersions] = useState<
    ExecutionResults["assets/versions"] | null
  >(null)
  const [reading, setReading] = useState(false)
  const [outputs, setOutputs] = useState<
    ExecutionResults["outputs/list"] | null
  >(null)
  const [outputBusy, setOutputBusy] = useState(false)
  const [importing, setImporting] = useState<OutputCandidate | null>(null)
  const [selection, setSelection] = useState<SelectedVersion[]>([])
  const [locked, setLocked] = useState(false)
  const [blocked, setBlocked] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const lifetime = useRef<AbortController | null>(null)
  const listRequest = useRef<AbortController | null>(null)
  const fileRequest = useRef<AbortController | null>(null)
  const outputRequest = useRef<AbortController | null>(null)
  const guardCallback = useRef(onGuard)
  const guards = useRef({ submission: cleanGuard, import: cleanGuard })
  useEffect(() => {
    guardCallback.current = onGuard
  }, [onGuard])
  useEffect(() => {
    const owner = new AbortController()
    lifetime.current = owner
    return () => {
      owner.abort()
      lifetime.current = null
      listRequest.current?.abort()
      fileRequest.current?.abort()
      outputRequest.current?.abort()
    }
  }, [data.client])
  const updateGuard = useCallback(
    (name: "submission" | "import", value: PromptGuard) => {
      guards.current[name] = value
      const combined = {
        dirty: guards.current.submission.dirty || guards.current.import.dirty,
        busy: guards.current.submission.busy || guards.current.import.busy,
        unresolved:
          guards.current.submission.unresolved ||
          guards.current.import.unresolved,
      }
      setLocked(combined.busy || combined.unresolved)
      guardCallback.current(combined)
    },
    []
  )
  function failure(caught: unknown) {
    if (
      caught instanceof ExecutionError &&
      ["unauthorized", "forbidden", "missing", "authority_changed"].includes(
        caught.reason
      )
    ) {
      lifetime.current?.abort()
      data.client.close()
      setBlocked(true)
      guardCallback.current(cleanGuard)
    }
    setError(caught)
  }
  async function list(nextQuery: string, cursor: string | null = null) {
    const owner = lifetime.current
    if (!owner || owner.signal.aborted) return
    listRequest.current?.abort()
    const request = new AbortController()
    listRequest.current = request
    setListing(true)
    setError(null)
    try {
      const page = parseAssetPage(
        await data.client.json(
          "assets/list",
          { taskId: data.task.id, query: nextQuery || null, cursor, limit: 50 },
          request.signal
        ),
        data.task.id
      )
      if (owner.signal.aborted || request.signal.aborted) return
      setQuery(nextQuery)
      setAssets((current) => ({
        ...page,
        items:
          cursor === null
            ? page.items
            : [
                ...current.items,
                ...page.items.filter(
                  (item) => !current.items.some((old) => old.id === item.id)
                ),
              ],
      }))
    } catch (caught) {
      if (!owner.signal.aborted && !request.signal.aborted) failure(caught)
    } finally {
      if (!owner.signal.aborted && !request.signal.aborted) setListing(false)
    }
  }
  async function openVersion(selection: AssetSelection) {
    const owner = lifetime.current
    if (!owner || owner.signal.aborted) return
    fileRequest.current?.abort()
    const request = new AbortController()
    fileRequest.current = request
    setSelected(null)
    setVersions(null)
    setReading(true)
    setError(null)
    try {
      const [value, page] = await Promise.all([
        data.client.json("assets/get", selection, request.signal),
        data.client.json(
          "assets/versions",
          { assetId: selection.assetId, cursor: null, limit: 50 },
          request.signal
        ),
      ])
      if (owner.signal.aborted || request.signal.aborted) return
      setSelected(
        parseAssetResult(value, { ...selection, taskId: data.task.id })
      )
      setVersions(
        parseVersionPage(page, {
          assetId: selection.assetId,
          taskId: data.task.id,
        })
      )
    } catch (caught) {
      if (!owner.signal.aborted && !request.signal.aborted) failure(caught)
    } finally {
      if (!owner.signal.aborted && !request.signal.aborted) setReading(false)
    }
  }
  async function moreVersions() {
    const owner = lifetime.current
    const request = fileRequest.current
    if (
      !owner ||
      owner.signal.aborted ||
      !request ||
      request.signal.aborted ||
      !selected ||
      !versions?.nextCursor ||
      reading
    )
      return
    setReading(true)
    try {
      const value = parseVersionPage(
        await data.client.json(
          "assets/versions",
          {
            assetId: selected.asset.id,
            cursor: versions.nextCursor,
            limit: 50,
          },
          request.signal
        ),
        { assetId: selected.asset.id, taskId: data.task.id }
      )
      if (!owner.signal.aborted && !request.signal.aborted)
        setVersions((current) => ({
          ...value,
          items: [
            ...(current?.items ?? []),
            ...value.items.filter(
              (item) => !current?.items.some((old) => old.id === item.id)
            ),
          ],
        }))
    } catch (caught) {
      if (!owner.signal.aborted && !request.signal.aborted) failure(caught)
    } finally {
      if (!owner.signal.aborted && !request.signal.aborted) setReading(false)
    }
  }
  async function checkOutput(cursor: string | null = null) {
    const owner = lifetime.current
    if (!owner || owner.signal.aborted || !data.session || outputBusy) return
    outputRequest.current?.abort()
    const request = new AbortController()
    outputRequest.current = request
    setOutputBusy(true)
    setError(null)
    try {
      const result = parseOutputPage(
        await data.client.json(
          "outputs/list",
          { sessionId: data.session.id, cursor, limit: 50 },
          request.signal
        )
      )
      if (!owner.signal.aborted && !request.signal.aborted)
        setOutputs((current) => ({
          ...result,
          items:
            cursor === null
              ? result.items
              : [
                  ...(current?.items ?? []),
                  ...result.items.filter(
                    (item) => !current?.items.some((old) => old.id === item.id)
                  ),
                ],
        }))
    } catch (caught) {
      if (!owner.signal.aborted && !request.signal.aborted) failure(caught)
    } finally {
      if (!owner.signal.aborted && !request.signal.aborted) setOutputBusy(false)
    }
  }
  if (blocked)
    return (
      <p role="alert" className="p-5 text-sm">
        {copy.denied}
      </p>
    )
  return (
    <section className="mx-auto max-w-6xl space-y-6 p-5 sm:p-7 [overflow-wrap:anywhere]">
      <header className="space-y-2">
        <h2 className="text-xl font-semibold tracking-tight">
          {copy.documents}
        </h2>
        <p className="text-sm font-medium">
          <bdi>{data.task.title}</bdi>
        </p>
        <p className="text-muted-foreground max-w-2xl text-sm leading-relaxed">
          {copy.documentsHint}
        </p>
      </header>
      <div className="grid min-w-0 gap-6 xl:grid-cols-[minmax(16rem,1fr)_minmax(0,2fr)]">
        <section className="min-w-0 space-y-4">
          <form
            className="space-y-3"
            onSubmit={(event) => {
              event.preventDefault()
              void list(search)
            }}
          >
            <Field label={copy.findFiles}>
              {(id) => (
                <input
                  id={id}
                  type="search"
                  className={controlClass}
                  value={search}
                  onChange={(event) => setSearch(event.target.value)}
                />
              )}
            </Field>
            <Action type="submit" variant="outline" disabled={listing}>
              <Search aria-hidden="true" className="size-4" />
              {copy.searchFiles}
            </Action>
          </form>
          {listing && (
            <p role="status" className="text-sm">
              {copy.loadingAssets}
            </p>
          )}
          {assets.items.length === 0 && (
            <p className="text-muted-foreground text-sm">
              {query ? copy.noMatchingFiles : copy.noFiles}
            </p>
          )}
          <ul className="space-y-2">
            {assets.items.map((asset) => (
              <li key={asset.id}>
                <button
                  type="button"
                  className="hover:bg-accent/40 focus-visible:ring-ring flex min-h-14 w-full min-w-0 items-center gap-3 rounded-xl border p-3 text-start focus-visible:ring-2 focus-visible:outline-none"
                  onClick={() =>
                    void openVersion({
                      assetId: asset.id,
                      versionId: asset.latestVersionId,
                    })
                  }
                >
                  <FileText
                    aria-hidden="true"
                    className="text-muted-foreground size-5 shrink-0"
                  />
                  <span className="min-w-0 text-sm">
                    <bdi className="block font-medium [overflow-wrap:anywhere]">
                      {asset.title}
                    </bdi>
                    <bdi className="text-muted-foreground text-xs">
                      {asset.mediaType}
                    </bdi>
                  </span>
                </button>
              </li>
            ))}
          </ul>
          {assets.nextCursor && (
            <Action
              variant="outline"
              disabled={listing}
              onClick={() => void list(query, assets.nextCursor)}
            >
              {copy.moreFiles}
            </Action>
          )}
          {data.session && (
            <section className="space-y-3 rounded-xl border p-4">
              <h3 className="font-semibold">{copy.sessionOutput}</h3>
              <p className="text-sm">
                <bdi>{data.session.title}</bdi>
              </p>
              <p className="text-muted-foreground text-sm leading-relaxed">
                {copy.outputHint}
              </p>
              <Action
                variant="outline"
                disabled={outputBusy || locked}
                onClick={() => void checkOutput()}
              >
                {copy.checkOutput}
              </Action>
              {outputBusy && (
                <p role="status" className="text-sm">
                  {copy.loadingAssets}
                </p>
              )}
              {outputs?.items.length === 0 && (
                <p className="text-muted-foreground text-sm">
                  {copy.noOutputs}
                </p>
              )}
              <ul className="space-y-3">
                {outputs?.items.map((output) => (
                  <li
                    key={output.id}
                    className="space-y-2 border-t pt-3 text-sm"
                  >
                    <bdi className="block font-medium">{output.name}</bdi>
                    <p className="text-muted-foreground">
                      <bdi>{output.mediaType}</bdi> · {output.byteSize}{" "}
                      {copy.bytes}
                    </p>
                    {output.status === "available" &&
                    output.byteSize <= MAX_ASSET_BYTES &&
                    data.session?.capabilities.importOutput ? (
                      <Action
                        variant="outline"
                        disabled={locked}
                        onClick={() => setImporting(output)}
                      >
                        {copy.retainOutput}
                      </Action>
                    ) : (
                      <p>
                        {output.status === "changed"
                          ? copy.outputChanged
                          : copy.outputUnsupported}
                      </p>
                    )}
                  </li>
                ))}
              </ul>
              {outputs?.nextCursor && (
                <Action
                  variant="outline"
                  disabled={outputBusy || locked}
                  onClick={() => void checkOutput(outputs.nextCursor)}
                >
                  {copy.moreOutputs}
                </Action>
              )}
            </section>
          )}
        </section>
        <div className="min-w-0 space-y-6">
          {reading && (
            <p role="status" className="text-sm">
              {copy.checkingFile}
            </p>
          )}
          {selected && (
            <article
              aria-label={copy.fileInformation}
              className="space-y-4 rounded-xl border p-4 sm:p-5"
            >
              <header className="space-y-2">
                <p className="text-muted-foreground text-xs">
                  {selected.version.visibility === "private"
                    ? copy.privateVersion
                    : copy.sharedVersion}
                </p>
                <h3 className="text-lg font-semibold">
                  <bdi>{selected.asset.title}</bdi>
                </h3>
                <p className="text-sm">
                  {copy.version} {selected.version.version} ·{" "}
                  {selected.version.byteSize} {copy.bytes} ·{" "}
                  <bdi>{selected.version.mediaType}</bdi>
                </p>
              </header>
              {versions && (
                <Field label={copy.fileVersions}>
                  {(id) => (
                    <select
                      id={id}
                      className={controlClass}
                      disabled={reading}
                      value={selected.version.id}
                      onChange={(event) =>
                        void openVersion({
                          assetId: selected.asset.id,
                          versionId: event.target.value,
                        })
                      }
                    >
                      {[
                        selected.version,
                        ...versions.items.filter(
                          (version) => version.id !== selected.version.id
                        ),
                      ].map((version) => (
                        <option key={version.id} value={version.id}>
                          {copy.version} {version.version} · {version.byteSize}{" "}
                          {copy.bytes}
                        </option>
                      ))}
                    </select>
                  )}
                </Field>
              )}
              {versions?.nextCursor && (
                <Action
                  variant="outline"
                  disabled={reading}
                  onClick={() => void moreVersions()}
                >
                  {copy.moreVersions}
                </Action>
              )}
              <PrivateAssetContent
                key={selected.version.id}
                client={data.client}
                asset={selected}
                onError={(caught) => {
                  setSelected(null)
                  setVersions(null)
                  failure(caught)
                }}
              />
              <details>
                <summary className="focus-visible:ring-ring cursor-pointer rounded py-2 text-sm focus-visible:ring-2">
                  {copy.fileInformation}
                </summary>
                <dl className="space-y-3 text-sm">
                  <div>
                    <dt className="text-muted-foreground">
                      {copy.createdWith}
                    </dt>
                    <dd>
                      <bdi>{selected.version.producer.clientId}</bdi> ·{" "}
                      <bdi>
                        {selected.version.producer.model ??
                          copy.modelUnavailable}
                      </bdi>
                    </dd>
                  </div>
                  <div>
                    <dt className="text-muted-foreground">SHA-256</dt>
                    <dd dir="ltr" className="break-all font-mono text-xs">
                      {selected.version.sha256}
                    </dd>
                  </div>
                </dl>
              </details>
              {selected.capabilities.submit && (
                <Action
                  variant="outline"
                  disabled={
                    locked ||
                    selection.length >= 16 ||
                    selection.some(
                      (item) => selectionKey(item) === selectionKey(selected)
                    )
                  }
                  onClick={() =>
                    setSelection((current) =>
                      current.some(
                        (item) => selectionKey(item) === selectionKey(selected)
                      )
                        ? current
                        : [
                            ...current,
                            {
                              asset: { ...selected.asset },
                              version: { ...selected.version },
                            },
                          ]
                    )
                  }
                >
                  {copy.selectVersion}
                </Action>
              )}
            </article>
          )}
          <AssetSubmission
            business={business}
            client={data.client}
            taskId={data.task.id}
            members={members}
            selection={selection}
            onRemove={(key) =>
              setSelection((current) =>
                current.filter((item) => selectionKey(item) !== key)
              )
            }
            onClear={() => setSelection([])}
            onGuard={(value) => updateGuard("submission", value)}
            onPublished={onPublished}
            onTask={onTask}
            onDenied={failure}
          />
        </div>
      </div>
      {error != null && (
        <p role="alert" className="text-sm">
          {executionErrorMessage(error, copy)}
        </p>
      )}
      {importing && data.session && (
        <ImportOutputDialog
          key={`${importing.id}:${importing.revision}`}
          client={data.client}
          taskId={data.task.id}
          sessionId={data.session.id}
          output={importing}
          assets={assets.items}
          onDenied={failure}
          onGuard={(value) => updateGuard("import", value)}
          onClose={() => {
            setImporting(null)
            updateGuard("import", cleanGuard)
          }}
          onImported={({ asset, version }) => {
            setImporting(null)
            updateGuard("import", cleanGuard)
            void list(query)
            void openVersion({ assetId: asset.id, versionId: version.id })
          }}
        />
      )}
    </section>
  )
}
