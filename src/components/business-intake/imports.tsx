"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import { Input } from "@/components/ui/input"
import { Action, Field } from "@/components/business/ui"
import type { BusinessClient } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import { useIntakeCopy } from "@/lib/business/intake-copy"
import type { BindingSummary, IntakeImport, Page } from "@/lib/business/intake"
import { accessWasLost, IntakeError, outcomeIsUncertain } from "./ui"

export function ImportPanel({
  client,
  binding,
  active,
  onRead,
  refreshSourceId,
}: {
  client: BusinessClient
  binding: BindingSummary
  active: boolean
  onRead: () => void
  refreshSourceId?: string
}) {
  const copy = useIntakeCopy()
  const common = useBusinessCopy()
  const [page, setPage] = useState<Page<IntakeImport> | null>(null)
  const [view, setView] = useState<"unfinished" | "all">("unfinished")
  const [error, setError] = useState<unknown>(null)
  const [busy, setBusy] = useState(false)
  const [unknown, setUnknown] = useState(false)
  const [from, setFrom] = useState("")
  const [to, setTo] = useState("")
  const [conversation, setConversation] = useState("")
  const [message, setMessage] = useState("")
  const [feedback, setFeedback] = useState("")
  const current = useRef(true)
  const working = useRef(false)
  const replay = useRef<(() => Promise<IntakeImport>) | null>(null)
  const readCallback = useRef(onRead)
  useEffect(() => {
    readCallback.current = onRead
  }, [onRead])
  useEffect(() => {
    current.current = true
    return () => {
      current.current = false
    }
  }, [])
  const load = useCallback(
    async (next = 0) => {
      try {
        const result = await client.intake("imports/list", {
          bindingId: binding.id,
          view,
          page: next,
        })
        if (!current.current) return
        setPage((prior) =>
          next && prior
            ? {
                ...result,
                items: [
                  ...prior.items,
                  ...result.items.filter(
                    (item) => !prior.items.some((old) => old.id === item.id)
                  ),
                ],
              }
            : result
        )
        setError(null)
      } catch (caught) {
        if (!current.current) return
        setError(caught)
        if (accessWasLost(caught)) {
          setPage(null)
          replay.current = null
          setUnknown(false)
        }
      }
    },
    [client, binding.id, view]
  )
  useEffect(() => {
    if (active && binding.capabilities.import) void load()
  }, [load, active, binding.capabilities.import])
  function received(result: IntakeImport) {
    setPage((prior) => ({
      page: prior?.page ?? 0,
      hasMore: prior?.hasMore ?? false,
      items: [
        result,
        ...(prior?.items ?? []).filter((entry) => entry.id !== result.id),
      ],
    }))
    readCallback.current()
  }
  async function run(operation: () => Promise<IntakeImport>) {
    if (working.current || !active || !binding.capabilities.import) return
    working.current = true
    replay.current = operation
    setBusy(true)
    setError(null)
    try {
      const result = await operation()
      if (!current.current) return
      received(result)
      setUnknown(false)
      replay.current = null
    } catch (caught) {
      if (!current.current) return
      setError(caught)
      setUnknown(outcomeIsUncertain(caught))
      if (!outcomeIsUncertain(caught)) replay.current = null
      if (accessWasLost(caught)) setPage(null)
    } finally {
      working.current = false
      if (current.current) setBusy(false)
    }
  }
  async function check(item: IntakeImport) {
    if (working.current) return
    working.current = true
    setBusy(true)
    try {
      const result = await client.intake("imports/get", { importId: item.id })
      if (!current.current) return
      received(result)
      setError(null)
      // A read status is authority for its next capabilities, but cannot resolve
      // an uncertain start whose response did not identify an import.
      if (!unknown) replay.current = null
    } catch (caught) {
      if (current.current) {
        setError(caught)
        if (accessWasLost(caught)) setPage(null)
      }
    } finally {
      working.current = false
      if (current.current) setBusy(false)
    }
  }
  const windowFrom = Date.parse(`${from}:00Z`)
  const windowTo = Date.parse(`${to}:00Z`)
  const validWindow =
    Number.isFinite(windowFrom) &&
    Number.isFinite(windowTo) &&
    windowFrom < windowTo &&
    windowTo - windowFrom <= 31 * 86400000
  const positive = (value: string) =>
    /^\d+$/.test(value) && Number(value) > 0 && Number(value) <= 2147483647
  const validCapture =
    binding.kind === "email"
      ? positive(conversation) && positive(message)
      : /^[0-7][0-9A-HJKMNP-TV-Z]{25}$/.test(feedback)
  if (!binding.capabilities.import) return null
  return (
    <section
      aria-label={copy.imports}
      className="border-border space-y-5 rounded-2xl border p-4 sm:p-5"
    >
      <div className="flex flex-wrap items-center justify-between gap-3">
        <h2 className="font-semibold">{copy.imports}</h2>
        <Action variant="ghost" disabled={busy} onClick={() => void load()}>
          {common.refresh}
        </Action>
      </div>
      <form
        className="space-y-4"
        onSubmit={(event) => {
          event.preventDefault()
          if (busy || unknown) return
          const operationId = crypto.randomUUID()
          if (refreshSourceId) {
            const input = {
              operationId,
              bindingId: binding.id,
              selection: { kind: "record" as const, sourceId: refreshSourceId },
            }
            void run(() => client.intake("imports/start", input))
          } else if (binding.kind === "fireflies" && validWindow) {
            const input = {
              operationId,
              bindingId: binding.id,
              selection: {
                kind: "window" as const,
                fromDate: new Date(windowFrom).toISOString(),
                toDate: new Date(windowTo).toISOString(),
              },
            }
            void run(() => client.intake("imports/start", input))
          } else if (validCapture && binding.kind === "email") {
            const input = {
              operationId,
              bindingId: binding.id,
              ref: {
                kind: "email" as const,
                conversationId: Number(conversation),
                messageId: Number(message),
              },
            }
            void run(() => client.intake("imports/capture", input))
          } else if (validCapture && binding.kind === "hafidh_testflight") {
            const input = {
              operationId,
              bindingId: binding.id,
              ref: { kind: "hafidh_testflight" as const, ulid: feedback },
            }
            void run(() => client.intake("imports/capture", input))
          }
        }}
      >
        {refreshSourceId ? (
          <p className="text-muted-foreground text-sm leading-relaxed">
            {copy.sourceRefreshHint}
          </p>
        ) : binding.kind === "fireflies" ? (
          <>
            <p className="text-muted-foreground text-sm leading-relaxed">
              {copy.importHint}
            </p>
            <div className="grid gap-4 sm:grid-cols-2">
              <Field label={copy.from} hint={copy.utcHint}>
                {(id) => (
                  <Input
                    id={id}
                    type="datetime-local"
                    dir="ltr"
                    value={from}
                    onChange={(event) => setFrom(event.target.value)}
                    required
                    className="min-h-11 rounded-xl"
                    disabled={busy || unknown}
                  />
                )}
              </Field>
              <Field label={copy.to}>
                {(id) => (
                  <Input
                    id={id}
                    type="datetime-local"
                    dir="ltr"
                    value={to}
                    onChange={(event) => setTo(event.target.value)}
                    required
                    className="min-h-11 rounded-xl"
                    disabled={busy || unknown}
                  />
                )}
              </Field>
            </div>
          </>
        ) : (
          <>
            <p className="text-muted-foreground text-sm leading-relaxed">
              {copy.captureHint}
            </p>
            {binding.kind === "email" ? (
              <div className="grid gap-4 sm:grid-cols-2">
                <Field label={copy.conversationId}>
                  {(id) => (
                    <Input
                      id={id}
                      inputMode="numeric"
                      pattern="[0-9]+"
                      value={conversation}
                      onChange={(event) => setConversation(event.target.value)}
                      className="min-h-11 rounded-xl"
                      disabled={busy || unknown}
                      required
                    />
                  )}
                </Field>
                <Field label={copy.messageId}>
                  {(id) => (
                    <Input
                      id={id}
                      inputMode="numeric"
                      pattern="[0-9]+"
                      value={message}
                      onChange={(event) => setMessage(event.target.value)}
                      className="min-h-11 rounded-xl"
                      disabled={busy || unknown}
                      required
                    />
                  )}
                </Field>
              </div>
            ) : (
              <Field label={copy.feedbackId}>
                {(id) => (
                  <Input
                    id={id}
                    dir="ltr"
                    value={feedback}
                    onChange={(event) => setFeedback(event.target.value)}
                    maxLength={26}
                    className="min-h-11 rounded-xl"
                    disabled={busy || unknown}
                    required
                  />
                )}
              </Field>
            )}
          </>
        )}
        <Action
          type="submit"
          variant="outline"
          disabled={
            busy ||
            unknown ||
            (!refreshSourceId &&
              !(binding.kind === "fireflies" ? validWindow : validCapture))
          }
        >
          {refreshSourceId
            ? copy.sourceRefresh
            : binding.kind === "fireflies"
              ? copy.startImport
              : copy.captureRecord}
        </Action>
      </form>
      {error != null && <IntakeError error={error} />}
      {unknown && (
        <div role="status" className="space-y-3 text-sm">
          <p>{copy.importUnknown}</p>
          <Action
            variant="outline"
            disabled={busy || !replay.current}
            onClick={() => {
              if (replay.current) void run(replay.current)
            }}
          >
            {copy.retryExact}
          </Action>
        </div>
      )}
      <label className="flex min-h-11 items-center gap-3 text-sm">
        <input
          type="checkbox"
          checked={view === "all"}
          onChange={(event) =>
            setView(event.target.checked ? "all" : "unfinished")
          }
          className="accent-primary size-4"
        />
        {copy.importHistory}
      </label>
      {page?.items.length === 0 && (
        <p className="text-muted-foreground text-sm">{copy.noImports}</p>
      )}
      <ul className="divide-border divide-y">
        {page?.items.map((item) => (
          <li key={item.id} className="space-y-3 py-4">
            <div className="flex flex-wrap items-center justify-between gap-2">
              <strong className="text-sm">
                {copy.importState[item.state]}
              </strong>
              <span className="text-muted-foreground text-xs">
                {common.sourceVersion} {item.revision}
              </span>
            </div>
            <p className="text-muted-foreground text-sm tabular-nums">
              {copy.discovered}: {item.discovered} · {copy.completed}:{" "}
              {item.completed} · {copy.failedCount}: {item.failed}
            </p>
            <p className="text-sm">
              {copy.coverage}: {copy.coverageState[item.coverage]}
            </p>
            {item.coverage === "partial" && (
              <p className="text-muted-foreground text-sm leading-relaxed">
                {copy.partialHint}
              </p>
            )}
            {item.nextAttemptAt && (
              <p className="text-sm">
                {copy.nextAttempt} <bdi>{item.nextAttemptAt}</bdi>
              </p>
            )}
            {item.errorCode && (
              <p className="text-sm">{copy.reason[item.errorCode]}</p>
            )}
            <div className="flex flex-wrap gap-2">
              <Action
                variant="outline"
                disabled={busy}
                onClick={() => void check(item)}
              >
                {copy.checkImport}
              </Action>
              {item.capabilities.advance && (
                <Action
                  disabled={
                    busy ||
                    unknown ||
                    (!!item.nextAttemptAt &&
                      Date.parse(item.nextAttemptAt) > Date.now())
                  }
                  onClick={() => {
                    const input = {
                      operationId: crypto.randomUUID(),
                      importId: item.id,
                      expectedRevision: item.revision,
                    }
                    void run(() => client.intake("imports/advance", input))
                  }}
                >
                  {copy.advance}
                </Action>
              )}
              {item.capabilities.cancel && (
                <Action
                  variant="ghost"
                  disabled={busy || unknown}
                  onClick={() => {
                    const input = {
                      operationId: crypto.randomUUID(),
                      importId: item.id,
                      expectedRevision: item.revision,
                    }
                    void run(() => client.intake("imports/cancel", input))
                  }}
                >
                  {copy.cancelImport}
                </Action>
              )}
            </div>
          </li>
        ))}
      </ul>
      {page?.hasMore && (
        <Action
          variant="outline"
          disabled={busy}
          onClick={() => void load(page.page + 1)}
        >
          {common.more}
        </Action>
      )}
    </section>
  )
}
