"use client"

import { useEffect, useRef, useState } from "react"
import { Send } from "lucide-react"
import {
  RichComposer,
  type RichComposerHandle,
} from "@/components/chat/composer/rich-composer"
import { Action } from "@/components/business/ui"
import type { ExecutionClient } from "@/lib/business-execution/client"
import {
  useExecutionCopy,
  executionErrorMessage,
} from "@/lib/business-execution/copy"
import { parseOperationSummary } from "@/lib/business-execution/frames"
import { ExecutionError, record, text } from "@/lib/business-execution/protocol"
import type {
  ExecutionInputs,
  OperationSummary,
  PromptInputRef,
  SessionSummary,
} from "@/lib/business-execution/types"

export interface PromptReference {
  label: string
  input: PromptInputRef
}
export interface PromptGuard {
  dirty: boolean
  busy: boolean
  unresolved: boolean
}
type PromptInput = ExecutionInputs["sessions/prompt"]
interface Attempt {
  input: PromptInput
  operation: OperationSummary | null
  references: PromptReference[]
}
function referenceKey(input: PromptInputRef): string {
  return input.kind === "task"
    ? `task:${input.taskId}:${input.expectedRevision}`
    : `asset:${input.assetId}:${input.versionId}`
}

/** Mounted only within one authorized session/pane scope. The owner rekeys on
 * identity changes, but keeps this component mounted through locale/pane changes.
 * RichComposer owns editing; E1 owns only its explicit send intent and receipt.
 */
export function SessionPrompt({
  client,
  session,
  references = [],
  initialDraft = "",
  onAccepted,
  onGuard,
}: {
  client: ExecutionClient
  session: SessionSummary
  references?: PromptReference[]
  initialDraft?: string
  onAccepted: () => void
  onGuard?: (guard: PromptGuard) => void
}) {
  const copy = useExecutionCopy()
  const composer = useRef<RichComposerHandle>(null)
  const lifetime = useRef<AbortController | null>(null)
  const held = useRef<PromptInput | null>(null)
  const inFlight = useRef(false)
  const guardCallback = useRef(onGuard)
  const [ready, setReady] = useState(false)
  const [draft, setDraft] = useState(initialDraft)
  const [selected, setSelected] = useState<string[]>([])
  const [attempt, setAttempt] = useState<Attempt | null>(null)
  const [busy, setBusy] = useState(false)
  const [accepted, setAccepted] = useState(false)
  const [error, setError] = useState<unknown>(null)
  useEffect(() => {
    const scope = new AbortController()
    lifetime.current = scope
    return () => {
      scope.abort()
      lifetime.current = null
    }
  }, [client, session.id])
  useEffect(() => {
    guardCallback.current = onGuard
  }, [onGuard])
  useEffect(() => {
    guardCallback.current?.({
      dirty: draft.length > 0 || selected.length > 0,
      busy,
      unresolved: attempt !== null,
    })
  }, [draft, selected, busy, attempt])
  const choices = (attempt?.references ?? references).map((reference) => ({
    ...reference,
    key: referenceKey(reference.input),
  }))
  const staleReference = selected.some(
    (key) => !choices.some((choice) => choice.key === key)
  )
  const tooLong = Array.from(draft).length > 32000 || selected.length > 16
  const locked = busy || attempt !== null
  const canSend =
    ready &&
    session.capabilities.prompt &&
    !locked &&
    !staleReference &&
    !tooLong &&
    draft.trim().length > 0

  function receive(operation: OperationSummary, input: PromptInput) {
    if (held.current !== input) return
    if (operation.id !== input.operationId)
      throw new ExecutionError("transport_unavailable")
    if (operation.status === "confirmed") {
      held.current = null
      setAttempt(null)
      setAccepted(true)
      // A receipt confirms this exact request, not any later programmatic edit.
      if (composer.current?.getText() === input.text) {
        composer.current.clear()
        setDraft("")
        setSelected([])
      }
      onAccepted()
    } else {
      setAttempt((current) =>
        current?.input === input ? { ...current, operation } : current
      )
    }
  }
  async function send() {
    const scope = lifetime.current
    if (!canSend || held.current || !scope || scope.signal.aborted) return
    const exactText = composer.current?.getText() ?? ""
    if (!exactText.trim() || Array.from(exactText).length > 32000) return
    const input: PromptInput = {
      operationId: crypto.randomUUID(),
      sessionId: session.id,
      expectedSessionRevision: session.revision,
      text: exactText,
      inputs: choices
        .filter((choice) => selected.includes(choice.key))
        .map((choice) => ({ ...choice.input })),
    }
    held.current = input
    inFlight.current = true
    setAttempt({
      input,
      operation: null,
      references: choices
        .filter((choice) => selected.includes(choice.key))
        .map((choice) => ({ label: choice.label, input: { ...choice.input } })),
    })
    setBusy(true)
    setAccepted(false)
    setError(null)
    try {
      const value = await client.json("sessions/prompt", input, scope.signal)
      if (scope.signal.aborted) return
      const result = record(value, ["operation", "messageId", "inputHash"])
      text(result.messageId)
      if (!/^[a-f0-9]{64}$/.test(text(result.inputHash)))
        throw new ExecutionError("transport_unavailable")
      receive(parseOperationSummary(result.operation), input)
    } catch (caught) {
      if (!scope.signal.aborted) setError(caught)
    } finally {
      if (!scope.signal.aborted) {
        inFlight.current = false
        setBusy(false)
      }
    }
  }
  async function reconcile() {
    const scope = lifetime.current
    const input = held.current
    if (!scope || scope.signal.aborted || !input || inFlight.current) return
    inFlight.current = true
    setBusy(true)
    setError(null)
    try {
      const value = await client.json(
        "operations/get",
        { operationId: input.operationId, kind: "prompt" },
        scope.signal
      )
      if (scope.signal.aborted) return
      const result = record(value, ["operation", "resourceId"])
      receive(parseOperationSummary(result.operation), input)
    } catch (caught) {
      if (!scope.signal.aborted) setError(caught)
    } finally {
      if (!scope.signal.aborted) {
        inFlight.current = false
        setBusy(false)
      }
    }
  }
  return (
    <section
      aria-label={copy.prompt}
      className="border-border bg-background border-t p-4 sm:p-5"
    >
      <div className="focus-within:ring-ring rounded-xl border bg-background focus-within:ring-2">
        <RichComposer
          ref={composer}
          defaultText={initialDraft}
          placeholder={copy.promptHint}
          ariaLabel={copy.prompt}
          disabled={locked || !session.capabilities.prompt}
          onReady={() => setReady(true)}
          onChange={setDraft}
          onSubmit={() => void send()}
          className="max-h-56 min-h-28 overflow-y-auto p-3 text-base md:text-sm"
        />
        <div className="flex flex-wrap items-center justify-between gap-3 px-3 pb-3">
          <p className="text-muted-foreground max-w-md text-xs leading-relaxed">
            {copy.noContext}
          </p>
          <Action disabled={!canSend} onClick={() => void send()}>
            <Send aria-hidden="true" className="size-4 rtl:-scale-x-100" />
            {copy.send}
          </Action>
        </div>
      </div>
      {choices.length > 0 && (
        <fieldset disabled={locked} className="mt-3 space-y-1">
          <legend className="text-muted-foreground text-xs">
            {copy.context}
          </legend>
          {choices.map((choice) => (
            <label
              key={choice.key}
              className="flex min-h-11 items-center gap-2 text-sm"
            >
              <input
                type="checkbox"
                className="accent-primary size-4 shrink-0"
                checked={selected.includes(choice.key)}
                onChange={(event) =>
                  setSelected((current) =>
                    event.target.checked
                      ? [...current, choice.key]
                      : current.filter((key) => key !== choice.key)
                  )
                }
              />
              <bdi className="min-w-0 [overflow-wrap:anywhere]">
                {choice.label}
              </bdi>
            </label>
          ))}
        </fieldset>
      )}
      {staleReference && (
        <div role="alert" className="mt-3 space-y-2 text-sm">
          <p>{copy.contextChanged}</p>
          <Action
            variant="outline"
            disabled={locked}
            onClick={() => setSelected([])}
          >
            {copy.clearContext}
          </Action>
        </div>
      )}
      {tooLong && (
        <p role="alert" className="mt-3 text-sm">
          {copy.tooLong}
        </p>
      )}
      {!session.capabilities.prompt && (
        <p className="text-muted-foreground mt-3 text-sm">{copy.unavailable}</p>
      )}
      {busy && (
        <p role="status" className="text-muted-foreground mt-3 text-sm">
          {attempt?.operation ? copy.checking : copy.sending}
        </p>
      )}
      {!busy && attempt && (
        <div
          role="status"
          className="bg-muted/40 mt-3 space-y-3 rounded-xl border p-3 text-sm"
        >
          <p>
            {attempt.operation?.status === "failed"
              ? copy.refused
              : copy.uncertain}
          </p>
          {attempt.operation?.status === "failed" ? (
            <Action
              variant="outline"
              onClick={() => {
                held.current = null
                setAttempt(null)
                setError(null)
                composer.current?.focus()
              }}
            >
              {copy.reviewPrompt}
            </Action>
          ) : (
            <Action variant="outline" onClick={() => void reconcile()}>
              {copy.checkReceipt}
            </Action>
          )}
        </div>
      )}
      {accepted && (
        <p role="status" className="mt-3 text-sm">
          {copy.accepted}
        </p>
      )}
      {error != null && (
        <p role="alert" className="mt-3 text-sm">
          {executionErrorMessage(error, copy)}
        </p>
      )}
    </section>
  )
}
