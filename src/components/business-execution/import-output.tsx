"use client"

import { useEffect, useRef, useState } from "react"
import { Action, controlClass, Field, Modal } from "@/components/business/ui"
import { useBusinessCopy } from "@/lib/business/copy"
import type { ExecutionClient } from "@/lib/business-execution/client"
import {
  parseAssetResult,
  parseAssetSummary,
  parseAssetVersion,
} from "@/lib/business-execution/assets"
import {
  executionErrorMessage,
  useExecutionCopy,
} from "@/lib/business-execution/copy"
import { parseOperationSummary } from "@/lib/business-execution/frames"
import { ExecutionError, record } from "@/lib/business-execution/protocol"
import type {
  AssetSummary,
  ExecutionInputs,
  ExecutionResults,
  OperationSummary,
  OutputCandidate,
} from "@/lib/business-execution/types"
import type { PromptGuard } from "./session-prompt"

type Input = ExecutionInputs["assets/import-output"]
type Result = Pick<
  ExecutionResults["assets/import-output"],
  "asset" | "version"
>
export function ImportOutputDialog({
  client,
  taskId,
  sessionId,
  output,
  assets,
  onClose,
  onImported,
  onGuard,
  onDenied,
}: {
  client: ExecutionClient
  taskId: string
  sessionId: string
  output: OutputCandidate
  assets: AssetSummary[]
  onClose: () => void
  onImported: (result: Result) => void
  onGuard: (guard: PromptGuard) => void
  onDenied: (error: unknown) => void
}) {
  const copy = useExecutionCopy()
  const common = useBusinessCopy()
  // The dialog is keyed by output/revision; neither reload nor locale changes
  // replace the captured source or the proposed destination revision.
  const [initial] = useState(() => ({
    output: { ...output },
    assets: assets.map((asset) => ({ ...asset })),
  }))
  const [title, setTitle] = useState(output.name)
  const [assetId, setAssetId] = useState("")
  const [attempt, setAttempt] = useState<{
    input: Input
    operation: OperationSummary | null
  } | null>(null)
  const held = useRef<Input | null>(null)
  const flight = useRef(false)
  const lifetime = useRef<AbortController | null>(null)
  const guard = useRef(onGuard)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const [discard, setDiscard] = useState(false)
  const dirty = title !== initial.output.name || assetId !== ""
  const validTitle =
    title.trim().length > 0 &&
    Array.from(title.trim()).length <= 240 &&
    !/[\u0000-\u0008\u000b\u000c\u000e-\u001f\u007f-\u009f]/.test(title)
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
  }, [client])
  const locked = busy || attempt !== null
  function receive(value: unknown, input: Input) {
    const row = record(value, ["asset", "version", "operation"])
    const operation = parseOperationSummary(row.operation)
    if (held.current !== input || operation.id !== input.operationId)
      throw new ExecutionError("transport_unavailable")
    if (operation.status !== "confirmed") {
      setAttempt({ input, operation })
      return
    }
    const asset = parseAssetSummary(row.asset, taskId)
    const version = parseAssetVersion(row.version, {
      taskId,
      assetId: asset.id,
    })
    if (
      (input.assetId !== null && asset.id !== input.assetId) ||
      version.producer.sessionId !== input.sessionId
    )
      throw new ExecutionError("content_changed")
    held.current = null
    guard.current({ dirty: false, busy: false, unresolved: false })
    onImported({ asset, version })
  }
  async function send() {
    const owner = lifetime.current
    if (
      !owner ||
      owner.signal.aborted ||
      held.current ||
      flight.current ||
      !validTitle ||
      initial.output.status !== "available"
    )
      return
    const destination = initial.assets.find((asset) => asset.id === assetId)
    if (assetId && !destination) return
    const input: Input = {
      operationId: crypto.randomUUID(),
      sessionId,
      outputId: initial.output.id,
      expectedOutputRevision: initial.output.revision,
      title: title.trim(),
      assetId: destination?.id ?? null,
      expectedAssetRevision: destination?.revision ?? null,
    }
    flight.current = true
    guard.current({ dirty, busy: true, unresolved: false })
    setBusy(true)
    setError(null)
    try {
      if (destination) {
        const selection = {
          assetId: destination.id,
          versionId: destination.latestVersionId,
        }
        const current = parseAssetResult(
          await client.json("assets/get", selection, owner.signal),
          { ...selection, taskId }
        )
        if (owner.signal.aborted) return
        if (!current.capabilities.addVersion)
          throw new ExecutionError("forbidden")
        if (current.asset.revision !== destination.revision)
          throw new ExecutionError("conflict")
      }
      held.current = input
      setAttempt({ input, operation: null })
      guard.current({ dirty, busy: true, unresolved: true })
      const value = await client.json(
        "assets/import-output",
        input,
        owner.signal
      )
      if (!owner.signal.aborted) receive(value, input)
    } catch (caught) {
      if (!owner.signal.aborted) {
        if (
          caught instanceof ExecutionError &&
          [
            "unauthorized",
            "forbidden",
            "missing",
            "authority_changed",
          ].includes(caught.reason)
        )
          onDenied(caught)
        else setError(caught)
      }
    } finally {
      if (!owner.signal.aborted) {
        flight.current = false
        setBusy(false)
      }
    }
  }
  async function reconcile() {
    const owner = lifetime.current
    const input = held.current
    if (!owner || owner.signal.aborted || !input || flight.current) return
    flight.current = true
    setBusy(true)
    setError(null)
    try {
      const value = await client.json(
        "operations/get",
        { operationId: input.operationId, kind: "import_output" },
        owner.signal
      )
      if (owner.signal.aborted) return
      const row = record(value, ["operation", "resourceId"])
      const operation = parseOperationSummary(row.operation)
      if (operation.id !== input.operationId)
        throw new ExecutionError("transport_unavailable")
      if (operation.status === "confirmed") {
        // The accepted receipt contract intentionally exposes only an opaque
        // resourceId. Recover the immutable result by exact confirmed replay,
        // not an invented version lookup or a new import operation.
        const result = await client.json(
          "assets/import-output",
          input,
          owner.signal
        )
        if (!owner.signal.aborted) receive(result, input)
      } else setAttempt({ input, operation })
    } catch (caught) {
      if (!owner.signal.aborted) {
        if (
          caught instanceof ExecutionError &&
          ["unauthorized", "forbidden", "authority_changed"].includes(
            caught.reason
          )
        )
          onDenied(caught)
        else setError(caught)
      }
    } finally {
      if (!owner.signal.aborted) {
        flight.current = false
        setBusy(false)
      }
    }
  }
  return (
    <Modal
      title={copy.importTitle}
      description={copy.importHint}
      onClose={() => {
        if (locked) return
        if (dirty) setDiscard(true)
        else onClose()
      }}
    >
      <div className="space-y-4">
        <p className="text-sm">
          <bdi>{initial.output.name}</bdi> · {initial.output.byteSize}{" "}
          {copy.bytes}
        </p>
        <Field label={copy.retainedTitle}>
          {(id) => (
            <input
              id={id}
              className={controlClass}
              value={title}
              disabled={locked}
              onChange={(event) => setTitle(event.target.value)}
            />
          )}
        </Field>
        <Field label={copy.importDestination}>
          {(id) => (
            <select
              id={id}
              className={controlClass}
              value={assetId}
              disabled={locked}
              onChange={(event) => setAssetId(event.target.value)}
            >
              <option value="">{copy.newAsset}</option>
              {initial.assets.map((asset) => (
                <option key={asset.id} value={asset.id}>
                  {copy.addVersion} {asset.title}
                </option>
              ))}
            </select>
          )}
        </Field>
        <Action
          disabled={
            locked || !validTitle || initial.output.status !== "available"
          }
          onClick={() => void send()}
        >
          {copy.retainOutput}
        </Action>
        {busy && (
          <p role="status" className="text-sm">
            {copy.sending}
          </p>
        )}
        {attempt && !busy && (
          <div role="status" className="space-y-3 text-sm">
            <p>
              {attempt.operation?.status === "failed"
                ? copy.requestFailed
                : copy.waitingImport}
            </p>
            {attempt.operation?.status === "failed" ? (
              <Action
                variant="outline"
                onClick={() => {
                  held.current = null
                  setAttempt(null)
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
        {discard && !locked && (
          <div role="alert" className="space-y-3 rounded-xl border p-4 text-sm">
            <p>{common.discardHint}</p>
            <div className="flex flex-wrap gap-2">
              <Action variant="outline" onClick={() => setDiscard(false)}>
                {common.stay}
              </Action>
              <Action variant="destructive" onClick={onClose}>
                {common.discardDraft}
              </Action>
            </div>
          </div>
        )}
      </div>
    </Modal>
  )
}
