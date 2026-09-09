"use client"

import { useEffect, useRef, useState } from "react"
import {
  Action,
  controlClass,
  Field,
  Person,
  StatusBadge,
} from "@/components/business/ui"
import type { BusinessClient } from "@/lib/business/client"
import { BusinessError } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import type { Member } from "@/lib/business/identity"
import { personFor, type Task, type TaskDetail } from "@/lib/business/tasks"
import { parseAssetResult } from "@/lib/business-execution/assets"
import type { ExecutionClient } from "@/lib/business-execution/client"
import {
  executionErrorMessage,
  useExecutionCopy,
} from "@/lib/business-execution/copy"
import { parseOperationSummary } from "@/lib/business-execution/frames"
import { ExecutionError, record, text } from "@/lib/business-execution/protocol"
import type {
  AssetSummary,
  AssetVersion,
  ExecutionInputs,
  OperationSummary,
} from "@/lib/business-execution/types"
import type { PromptGuard } from "./session-prompt"

export interface SelectedVersion {
  asset: AssetSummary
  version: AssetVersion
}
type Input = ExecutionInputs["assets/submit"]
interface Reviewed {
  task: Task
  selection: SelectedVersion[]
  body: string
  fingerprint: string
}
interface Attempt {
  input: Input
  reviewed: Reviewed
  operation: OperationSummary | null
}
export function selectionKey(item: SelectedVersion) {
  return `${item.asset.id}:${item.version.id}`
}
function fingerprint(selection: SelectedVersion[], body: string) {
  return JSON.stringify([
    selection.map(({ asset, version }) => [
      asset.id,
      version.id,
      version.sha256,
      version.byteSize,
      version.mediaType,
    ]),
    body,
  ])
}

export function AssetSubmission({
  business,
  client,
  taskId,
  members,
  selection,
  onRemove,
  onClear,
  onGuard,
  onPublished,
  onTask,
  onDenied,
}: {
  business: BusinessClient
  client: ExecutionClient
  taskId: string
  members: Member[]
  selection: SelectedVersion[]
  onRemove: (key: string) => void
  onClear: () => void
  onGuard: (guard: PromptGuard) => void
  onPublished: (detail: TaskDetail) => void
  onTask: () => void
  onDenied: (error: unknown) => void
}) {
  const copy = useExecutionCopy()
  const common = useBusinessCopy()
  const [body, setBody] = useState("")
  const [reviewed, setReviewed] = useState<Reviewed | null>(null)
  const [confirmed, setConfirmed] = useState<Reviewed | null>(null)
  const [attempt, setAttempt] = useState<Attempt | null>(null)
  const [published, setPublished] = useState(false)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const held = useRef<Attempt | null>(null)
  const flight = useRef(false)
  const lifetime = useRef<AbortController | null>(null)
  const guard = useRef(onGuard)
  const signature = fingerprint(selection, body)
  const dirty = selection.length > 0 || body.length > 0
  const valid =
    selection.length >= 1 &&
    selection.length <= 16 &&
    Array.from(body).length <= 20000
  const currentReview = reviewed?.fingerprint === signature ? reviewed : null
  const locked = busy || attempt !== null
  useEffect(() => {
    guard.current = onGuard
  }, [onGuard])
  useEffect(() => {
    guard.current({ dirty, busy, unresolved: attempt !== null })
  }, [dirty, busy, attempt])
  useEffect(() => {
    const owner = new AbortController()
    lifetime.current = owner
    return () => {
      owner.abort()
      lifetime.current = null
    }
  }, [client, business])
  function failed(caught: unknown) {
    if (
      caught instanceof BusinessError &&
      ["unauthorized", "forbidden", "missing", "closed"].includes(caught.kind)
    ) {
      onDenied(new ExecutionError("forbidden"))
      return
    }
    if (
      caught instanceof ExecutionError &&
      ["unauthorized", "forbidden", "missing", "authority_changed"].includes(
        caught.reason
      )
    ) {
      onDenied(caught)
      return
    }
    setError(caught)
  }
  async function review() {
    const owner = lifetime.current
    if (
      !owner ||
      owner.signal.aborted ||
      flight.current ||
      held.current ||
      !valid
    )
      return
    const exact = selection.map(({ asset, version }) => ({
      asset: { ...asset },
      version: { ...version },
    }))
    const exactBody = body
    flight.current = true
    setBusy(true)
    setError(null)
    setConfirmed(null)
    guard.current({ dirty: true, busy: true, unresolved: false })
    try {
      const [detail, checked] = await Promise.all([
        business.tasks("get", { taskId }),
        Promise.all(
          exact.map(async (item) => {
            const expected = {
              taskId,
              assetId: item.asset.id,
              versionId: item.version.id,
            }
            const value = parseAssetResult(
              await client.json(
                "assets/get",
                { assetId: expected.assetId, versionId: expected.versionId },
                owner.signal
              ),
              expected
            )
            if (!value.capabilities.submit)
              throw new ExecutionError("forbidden")
            if (
              value.version.sha256 !== item.version.sha256 ||
              value.version.byteSize !== item.version.byteSize ||
              value.version.mediaType !== item.version.mediaType
            )
              throw new ExecutionError("content_changed")
            return { asset: value.asset, version: value.version }
          })
        ),
      ])
      if (owner.signal.aborted) return
      if (detail.task.id !== taskId || !detail.task.capabilities.submit)
        throw new ExecutionError("forbidden")
      setReviewed({
        task: detail.task,
        selection: checked,
        body: exactBody,
        fingerprint: fingerprint(exact, exactBody),
      })
    } catch (caught) {
      if (!owner.signal.aborted) failed(caught)
    } finally {
      if (!owner.signal.aborted) {
        flight.current = false
        setBusy(false)
      }
    }
  }
  function acceptDetail(
    detail: TaskDetail,
    value: Attempt,
    deliverableId?: string
  ) {
    if (
      !detail ||
      detail.task?.id !== taskId ||
      !Array.isArray(detail.deliverables)
    )
      throw new ExecutionError("transport_unavailable")
    const selected = value.reviewed.selection
    const delivered = detail.deliverables.find(
      (item) =>
        (deliverableId === undefined
          ? item.revision === value.input.expectedTaskRevision + 1
          : item.id === deliverableId) &&
        item.body === value.input.body &&
        Array.isArray(item.assets) &&
        item.assets.length === selected.length &&
        item.assets.every((file, index) => {
          const expected = selected[index]
          return (
            file.assetId === expected.asset.id &&
            file.versionId === expected.version.id &&
            file.sha256 === expected.version.sha256 &&
            file.byteSize === expected.version.byteSize &&
            file.mediaType === expected.version.mediaType
          )
        })
    )
    if (!delivered) throw new ExecutionError("content_changed")
    held.current = null
    setAttempt(null)
    setReviewed(null)
    setConfirmed(null)
    setBody("")
    setPublished(true)
    onClear()
    guard.current({ dirty: false, busy: false, unresolved: false })
    onPublished(detail)
  }
  async function submit() {
    const owner = lifetime.current
    if (
      !owner ||
      owner.signal.aborted ||
      flight.current ||
      held.current ||
      !currentReview ||
      confirmed !== currentReview
    )
      return
    const input: Input = {
      operationId: crypto.randomUUID(),
      taskId,
      expectedTaskRevision: currentReview.task.revision,
      versions: currentReview.selection.map(({ asset, version }) => ({
        assetId: asset.id,
        versionId: version.id,
      })),
      body: currentReview.body,
    }
    const value: Attempt = { input, reviewed: currentReview, operation: null }
    held.current = value
    flight.current = true
    guard.current({ dirty: true, busy: true, unresolved: true })
    setAttempt(value)
    setBusy(true)
    setError(null)
    setPublished(false)
    try {
      const result = await client.json("assets/submit", input, owner.signal)
      if (owner.signal.aborted) return
      const row = record(result, ["detail", "operation"])
      const operation = parseOperationSummary(row.operation)
      if (operation.id !== input.operationId)
        throw new ExecutionError("transport_unavailable")
      if (operation.status === "confirmed") acceptDetail(result.detail, value)
      else setAttempt({ ...value, operation })
    } catch (caught) {
      if (!owner.signal.aborted) failed(caught)
    } finally {
      if (!owner.signal.aborted) {
        flight.current = false
        setBusy(false)
      }
    }
  }
  async function reconcile() {
    const owner = lifetime.current
    const value = held.current
    if (!owner || owner.signal.aborted || !value || flight.current) return
    flight.current = true
    setBusy(true)
    setError(null)
    try {
      const result = await client.json(
        "operations/get",
        { operationId: value.input.operationId, kind: "submit" },
        owner.signal
      )
      if (owner.signal.aborted) return
      const row = record(result, ["operation", "resourceId"])
      const operation = parseOperationSummary(row.operation)
      if (operation.id !== value.input.operationId)
        throw new ExecutionError("transport_unavailable")
      if (operation.status === "confirmed") {
        // d0d56a36 publication.rs retains deliverable_id as the receipt target.
        const id = text(row.resourceId)
        const detail = await business.tasks("get", { taskId })
        if (!owner.signal.aborted) acceptDetail(detail, value, id)
      } else setAttempt({ ...value, operation })
    } catch (caught) {
      // Missing receipt is an unresolved outcome, not a new submit opportunity.
      if (!owner.signal.aborted) {
        if (caught instanceof ExecutionError && caught.reason === "missing")
          setError(caught)
        else failed(caught)
      }
    } finally {
      if (!owner.signal.aborted) {
        flight.current = false
        setBusy(false)
      }
    }
  }
  const shown =
    attempt?.reviewed.selection ?? currentReview?.selection ?? selection
  const audience = attempt?.reviewed ?? currentReview
  return (
    <section
      aria-label={copy.submission}
      className="space-y-4 rounded-xl border p-4 sm:p-5"
    >
      <h3 className="font-semibold">{copy.submission}</h3>
      <p className="text-muted-foreground text-sm leading-relaxed">
        {copy.audienceHint}
      </p>
      {published && (
        <div role="status" className="space-y-3 text-sm">
          <p>{copy.submitted}</p>
          <Action variant="outline" onClick={onTask}>
            {copy.openTask}
          </Action>
        </div>
      )}
      <ul aria-label={copy.selectedVersions} className="space-y-2">
        {shown.map((item) => (
          <li
            key={selectionKey(item)}
            className="flex min-w-0 flex-wrap items-center justify-between gap-3 rounded-xl border p-3 text-sm"
          >
            <span className="min-w-0 [overflow-wrap:anywhere]">
              <bdi>{item.asset.title}</bdi> · {copy.version}{" "}
              {item.version.version} · {item.version.byteSize} {copy.bytes}
            </span>
            <Action
              variant="ghost"
              disabled={locked}
              onClick={() => onRemove(selectionKey(item))}
            >
              {copy.removeVersion}
            </Action>
          </li>
        ))}
      </ul>
      <Field label={copy.submissionBody}>
        {(id) => (
          <textarea
            id={id}
            dir="auto"
            className={`${controlClass} min-h-28 resize-y`}
            value={body}
            disabled={locked}
            onChange={(event) => setBody(event.target.value)}
          />
        )}
      </Field>
      {!valid && (
        <p className="text-muted-foreground text-sm">{copy.selectionLimit}</p>
      )}
      {reviewed && !currentReview && !attempt && (
        <p role="status" className="text-sm">
          {copy.selectionChanged}
        </p>
      )}
      <Action
        variant="outline"
        disabled={locked || !valid}
        onClick={() => void review()}
      >
        {copy.reviewSubmission}
      </Action>
      {audience && (
        <div className="space-y-4 rounded-xl border bg-card/40 p-4">
          <h4 className="text-sm font-semibold">{copy.submissionAudience}</h4>
          <p className="text-sm font-medium">
            <bdi>{audience.task.title}</bdi>
          </p>
          <div className="flex flex-wrap items-center gap-3">
            <StatusBadge status={audience.task.status} />
            <span className="text-muted-foreground text-xs">
              {copy.savedTaskRevision}: {audience.task.revision}
            </span>
          </div>
          <dl className="grid gap-4 text-sm sm:grid-cols-2">
            <div>
              <dt className="text-muted-foreground">{common.domain}</dt>
              <dd>{common[audience.task.domain]}</dd>
            </div>
            <div>
              <dt className="text-muted-foreground">{common.owner}</dt>
              <dd>
                <Person
                  person={personFor(
                    audience.task.ownerId,
                    members,
                    common.memberUnavailable
                  )}
                  fallback={common.memberUnavailable}
                />
              </dd>
            </div>
            <div>
              <dt className="text-muted-foreground">{common.assignee}</dt>
              <dd>
                <Person
                  person={personFor(
                    audience.task.assigneeId,
                    members,
                    common.memberUnavailable
                  )}
                  fallback={common.unassigned}
                />
              </dd>
            </div>
            <div>
              <dt className="text-muted-foreground">{common.reviewer}</dt>
              <dd>
                <Person
                  person={personFor(
                    audience.task.reviewerId,
                    members,
                    common.memberUnavailable
                  )}
                  fallback={common.anyReviewer}
                />
              </dd>
            </div>
          </dl>
          <details>
            <summary className="focus-visible:ring-ring cursor-pointer rounded py-2 text-sm focus-visible:ring-2">
              {copy.fileInformation}
            </summary>
            <ul className="space-y-3 text-xs">
              {audience.selection.map((item) => (
                <li key={selectionKey(item)} className="space-y-1">
                  <bdi>{item.asset.title}</bdi>
                  <p>
                    {copy.version} {item.version.version} ·{" "}
                    <bdi>{item.version.mediaType}</bdi>
                  </p>
                  <p dir="ltr" className="break-all font-mono">
                    SHA-256 {item.version.sha256}
                  </p>
                </li>
              ))}
            </ul>
          </details>
          <label className="flex min-h-11 items-start gap-3 text-sm leading-relaxed">
            <input
              type="checkbox"
              className="accent-primary mt-1 size-4 shrink-0"
              checked={confirmed === audience}
              disabled={locked}
              onChange={(event) =>
                setConfirmed(event.target.checked ? audience : null)
              }
            />
            <span>{copy.confirmSubmission}</span>
          </label>
          <Action
            disabled={locked || !currentReview || confirmed !== currentReview}
            onClick={() => void submit()}
          >
            {copy.submitFiles}
          </Action>
        </div>
      )}
      {busy && (
        <p role="status" className="text-sm">
          {attempt ? copy.sending : copy.checkingFile}
        </p>
      )}
      {attempt && !busy && (
        <div role="status" className="space-y-3 text-sm">
          <p>
            {attempt.operation?.status === "failed"
              ? copy.requestFailed
              : copy.waitingSubmission}
          </p>
          {attempt.operation?.status === "failed" ? (
            <Action
              variant="outline"
              onClick={() => {
                held.current = null
                setAttempt(null)
                setReviewed(null)
                setConfirmed(null)
                setError(null)
              }}
            >
              {copy.reviewRequest}
            </Action>
          ) : (
            <Action variant="outline" onClick={() => void reconcile()}>
              {copy.checkReceipt}
            </Action>
          )}
        </div>
      )}
      {error != null && (
        <p role="alert" className="text-sm">
          {executionErrorMessage(error, copy)}
        </p>
      )}
    </section>
  )
}
