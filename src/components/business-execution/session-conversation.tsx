"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import { Message, MessageContent } from "@/components/ai-elements/message"
import { Action } from "@/components/business/ui"
import type { ExecutionClient } from "@/lib/business-execution/client"
import {
  applyConversationFrame,
  emptyConversation,
} from "@/lib/business-execution/conversation"
import {
  executionErrorMessage,
  useExecutionCopy,
} from "@/lib/business-execution/copy"
import { ExecutionError, record } from "@/lib/business-execution/protocol"
import { parseHistory, parseSession } from "@/lib/business-execution/session"
import type {
  ExecutionResults,
  MessagePart,
  SessionSummary,
} from "@/lib/business-execution/types"
import {
  SessionPrompt,
  type PromptGuard,
  type PromptReference,
} from "./session-prompt"

function denied(error: unknown) {
  return (
    error instanceof ExecutionError &&
    ["unauthorized", "forbidden", "missing", "authority_changed"].includes(
      error.reason
    )
  )
}

export function SessionConversation({
  client,
  session,
  references,
  onGuard,
  onFiles,
  initialDraft = "",
}: {
  client: ExecutionClient
  session: SessionSummary
  references: PromptReference[]
  onGuard?: (guard: PromptGuard) => void
  onFiles?: () => void
  initialDraft?: string
}) {
  const copy = useExecutionCopy()
  const [current, setCurrent] = useState(session)
  const [feed, setFeed] = useState(() => emptyConversation(session.status))
  const feedRef = useRef(feed)
  const feedGeneration = useRef(session.generation)
  const lifetime = useRef<AbortController | null>(null)
  const checking = useRef(false)
  const guardCallback = useRef(onGuard)
  const [attempt, setAttempt] = useState(0)
  const [attaching, setAttaching] = useState(true)
  const [attached, setAttached] = useState(false)
  const [blocked, setBlocked] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const [history, setHistory] = useState(false)
  const [historyEpoch, setHistoryEpoch] = useState(0)
  const [checkingSession, setCheckingSession] = useState(false)
  useEffect(() => {
    guardCallback.current = onGuard
  }, [onGuard])

  useEffect(() => {
    const owner = new AbortController()
    lifetime.current = owner
    return () => {
      owner.abort()
      lifetime.current = null
    }
  }, [client, session.id])

  const withhold = useCallback(() => {
    lifetime.current?.abort()
    client.close()
    feedRef.current = emptyConversation("revoked")
    setFeed(feedRef.current)
    setBlocked(true)
    setHistory(false)
    setAttached(false)
    guardCallback.current?.({ dirty: false, busy: false, unresolved: false })
  }, [client])
  useEffect(() => {
    if (
      !current.capabilities.read ||
      current.mode !== "chat" ||
      current.status === "revoked" ||
      blocked
    )
      return
    const reader = new AbortController()
    const owner = lifetime.current
    if (!owner || owner.signal.aborted) return
    const abort = () => reader.abort()
    owner.signal.addEventListener("abort", abort, { once: true })
    if (feedGeneration.current !== current.generation) {
      feedGeneration.current = current.generation
      feedRef.current = emptyConversation(current.status)
      setFeed(feedRef.current)
      setHistoryEpoch((value) => value + 1)
    }
    setAttaching(true)
    setAttached(false)
    setError(null)
    void client
      .events(
        {
          operationId: crypto.randomUUID(),
          sessionId: current.id,
          expectedGeneration: current.generation,
          cursor: feedRef.current.cursor,
        },
        (frame) => {
          if (reader.signal.aborted) return
          if (
            frame.type === "detached" &&
            frame.reason === "authority_changed"
          ) {
            withhold()
            return
          }
          if (
            (frame.type === "snapshot" || frame.type === "replay") &&
            frame.operation.status !== "confirmed"
          )
            throw new ExecutionError("transport_unavailable")
          const next = applyConversationFrame(feedRef.current, frame)
          if (next.status === "revoked") {
            withhold()
            return
          }
          feedRef.current = next
          setFeed(next)
          if (frame.type === "snapshot") setHistoryEpoch((value) => value + 1)
          setAttaching(false)
          setAttached(frame.type !== "detached")
        },
        reader.signal
      )
      .then(() => {
        if (!reader.signal.aborted) {
          setAttached(false)
          setAttaching(false)
        }
      })
      .catch((caught: unknown) => {
        if (reader.signal.aborted) return
        setAttaching(false)
        setAttached(false)
        if (denied(caught)) {
          withhold()
        } else setError(caught)
      })
    return () => {
      reader.abort()
      owner.signal.removeEventListener("abort", abort)
    }
  }, [
    client,
    current.id,
    current.generation,
    current.mode,
    current.status,
    current.capabilities.read,
    attempt,
    blocked,
    withhold,
  ])

  async function refresh(reconnect: boolean) {
    const owner = lifetime.current
    if (!owner || owner.signal.aborted || checking.current) return
    checking.current = true
    setCheckingSession(true)
    setError(null)
    try {
      const value = await client.json(
        "sessions/get",
        { sessionId: current.id },
        owner.signal
      )
      if (owner.signal.aborted) return
      const row = record(value, ["session"])
      const next = parseSession(row.session, {
        taskId: current.taskId,
        sessionId: current.id,
      })
      if (!next.capabilities.read || next.status === "revoked") {
        withhold()
        return
      }
      setCurrent(next)
      if (reconnect) setAttempt((value) => value + 1)
    } catch (caught) {
      if (owner.signal.aborted) return
      if (denied(caught)) withhold()
      else setError(caught)
    } finally {
      checking.current = false
      if (!owner.signal.aborted) setCheckingSession(false)
    }
  }
  if (
    blocked ||
    !current.capabilities.read ||
    current.mode !== "chat" ||
    current.status === "revoked"
  )
    return (
      <p role="alert" className="p-5 text-sm">
        {copy.currentAccessRequired}
      </p>
    )
  return (
    <section
      aria-label={copy.conversation}
      className="flex min-h-full min-w-0 flex-col"
    >
      <header className="space-y-3 border-b p-4 sm:p-5">
        {onFiles && (
          <Action variant="outline" onClick={onFiles}>
            {copy.documents}
          </Action>
        )}
        <div className="flex flex-wrap items-center justify-between gap-3">
          <h2 className="min-w-0 text-lg font-semibold">
            <bdi>{current.title}</bdi>
          </h2>
          <span className="bg-muted/40 rounded-lg border px-2 py-1 text-xs">
            {copy[feed.status]}
          </span>
        </div>
        <p role="status" className="text-muted-foreground text-sm">
          {attaching
            ? copy.attaching
            : attached
              ? copy.attached
              : copy.disconnected}
        </p>
        <div className="flex flex-wrap gap-2">
          {!attached && !attaching && (
            <Action
              variant="outline"
              disabled={checkingSession}
              onClick={() => void refresh(true)}
            >
              {copy.reconnect}
            </Action>
          )}
          <Action variant="ghost" onClick={() => setHistory((value) => !value)}>
            {history ? copy.returnLive : copy.savedConversation}
          </Action>
        </div>
        {error != null && (
          <p role="alert" className="text-sm">
            {executionErrorMessage(error, copy)}
          </p>
        )}
      </header>
      <div className="flex-1 space-y-5 p-4 sm:p-5">
        {history ? (
          <SavedMessages
            key={historyEpoch}
            client={client}
            sessionId={current.id}
            onDenied={withhold}
          />
        ) : (
          <>
            {feed.trimmed && (
              <p className="text-muted-foreground text-sm">{copy.recentOnly}</p>
            )}
            {feed.messages.length === 0 && attached && (
              <p className="text-muted-foreground py-8 text-sm">
                {copy.emptyConversation}
              </p>
            )}
            <MessageParts parts={feed.messages} />
            {feed.tools.length > 0 && (
              <details className="rounded-xl border p-3">
                <summary className="focus-visible:ring-ring cursor-pointer rounded py-2 text-sm focus-visible:ring-2 focus-visible:outline-none">
                  {copy.activity}
                </summary>
                <ul className="mt-3 space-y-2 text-sm">
                  {feed.tools.map((tool) => (
                    <li
                      key={tool.id}
                      className="flex flex-wrap justify-between gap-2"
                    >
                      <bdi className="min-w-0 [overflow-wrap:anywhere]">
                        {tool.name}
                      </bdi>
                      <span className="text-muted-foreground">
                        {copy[tool.status]}
                      </span>
                    </li>
                  ))}
                </ul>
              </details>
            )}
          </>
        )}
      </div>
      <SessionPrompt
        client={client}
        initialDraft={initialDraft}
        session={{
          ...current,
          capabilities: {
            ...current.capabilities,
            prompt: current.capabilities.prompt && attached,
          },
        }}
        references={references}
        onAccepted={() => void refresh(false)}
        onGuard={onGuard}
      />
    </section>
  )
}

function MessageParts({ parts }: { parts: MessagePart[] }) {
  const copy = useExecutionCopy()
  const groups = new Map<string, MessagePart[]>()
  for (const part of parts)
    groups.set(part.messageId, [...(groups.get(part.messageId) ?? []), part])
  return (
    <div className="space-y-6">
      {Array.from(groups, ([id, messages]) => {
        const role = messages[0].role
        return (
          <Message
            key={id}
            from={role}
            className={role === "user" ? "ms-auto rtl:ml-0" : undefined}
          >
            <span className="text-muted-foreground text-xs">
              {role === "user" ? copy.you : copy.assistant}
            </span>
            <MessageContent className="group-[.is-user]:ms-auto rtl:group-[.is-user]:ml-0">
              <p
                dir="auto"
                className="text-sm leading-relaxed whitespace-pre-wrap [overflow-wrap:anywhere]"
              >
                {[...messages]
                  .sort((a, b) => a.part - b.part)
                  .map((part) => part.text)
                  .join("")}
              </p>
            </MessageContent>
          </Message>
        )
      })}
    </div>
  )
}

function SavedMessages({
  client,
  sessionId,
  onDenied,
}: {
  client: ExecutionClient
  sessionId: string
  onDenied: () => void
}) {
  const copy = useExecutionCopy()
  const deniedCallback = useRef(onDenied)
  const [cursors, setCursors] = useState<(string | null)[]>([null])
  const cursor = cursors[cursors.length - 1]
  const [result, setResult] = useState<{
    cursor: string | null
    page: ExecutionResults["sessions/history"] | null
    error: unknown
  } | null>(null)
  const page = result?.cursor === cursor ? result.page : null
  const error = result?.cursor === cursor ? result.error : null
  useEffect(() => {
    deniedCallback.current = onDenied
  }, [onDenied])
  useEffect(() => {
    const owner = new AbortController()
    void client
      .json(
        "sessions/history",
        { sessionId, beforeCursor: cursor, limit: 40 },
        owner.signal
      )
      .then((value) => {
        if (!owner.signal.aborted)
          setResult({ cursor, page: parseHistory(value), error: null })
      })
      .catch((caught: unknown) => {
        if (owner.signal.aborted) return
        if (denied(caught)) deniedCallback.current()
        else setResult({ cursor, page: null, error: caught })
      })
    return () => owner.abort()
  }, [client, sessionId, cursor])
  return (
    <section aria-label={copy.savedConversation} className="space-y-4">
      {!page && error == null && (
        <p role="status" className="text-sm">
          {copy.loadingHistory}
        </p>
      )}
      {error != null && (
        <p role="alert" className="text-sm">
          {executionErrorMessage(error, copy)}
        </p>
      )}
      {page && (
        <>
          {page.messages.length === 0 && (
            <p className="text-muted-foreground text-sm">{copy.historyEmpty}</p>
          )}
          <MessageParts parts={page.messages} />
          <div className="flex flex-wrap gap-2">
            {cursors.length > 1 && (
              <Action
                variant="outline"
                onClick={() => setCursors((value) => value.slice(0, -1))}
              >
                {copy.newerMessages}
              </Action>
            )}
            {page.nextBeforeCursor !== null &&
              !cursors.includes(page.nextBeforeCursor) && (
                <Action
                  variant="outline"
                  onClick={() =>
                    setCursors((value) => [...value, page.nextBeforeCursor])
                  }
                >
                  {copy.olderMessages}
                </Action>
              )}
          </div>
        </>
      )}
    </section>
  )
}
