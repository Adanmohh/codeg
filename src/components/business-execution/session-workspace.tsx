"use client"

import { useEffect, useRef, useState } from "react"
import { MessageSquare } from "lucide-react"
import { Action, controlClass, Field } from "@/components/business/ui"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import type { Task } from "@/lib/business/tasks"
import type { ExecutionClient } from "@/lib/business-execution/client"
import {
  executionErrorMessage,
  useExecutionCopy,
} from "@/lib/business-execution/copy"
import { parseOperationSummary } from "@/lib/business-execution/frames"
import { ExecutionError, record, text } from "@/lib/business-execution/protocol"
import {
  parseProfiles,
  parseSession,
  parseSessions,
} from "@/lib/business-execution/session"
import type {
  ExecutionInputs,
  ExecutionResults,
  OperationSummary,
  SessionSummary,
} from "@/lib/business-execution/types"
import type { PromptGuard } from "./session-prompt"
import { SessionConversation } from "./session-conversation"

export interface SessionIntent {
  taskId: string
  sessionId: string
  title: string
}
interface CommonProps {
  client: BusinessClient
  taskId: string
  originalOperator: boolean
  onGuard: (guard: PromptGuard) => void
  onFiles?: (taskId: string, sessionId: string | null) => void
}
function safeTaskError(error: unknown) {
  if (
    error instanceof BusinessError &&
    ["unauthorized", "forbidden", "missing", "closed"].includes(error.kind)
  )
    return new ExecutionError("forbidden")
  return error
}

export function TaskAiWorkspace(
  props: CommonProps & { onOpen: (session: SessionIntent) => void }
) {
  const copy = useExecutionCopy()
  if (!props.originalOperator || props.client.native)
    return (
      <p role="alert" className="p-5 text-sm">
        {copy.operatorOnly}
      </p>
    )
  return <TaskAiLoader key={props.taskId} {...props} />
}
interface HubData {
  client: ExecutionClient
  task: Task
  profiles: ExecutionResults["profiles/list"]
  sessions: ExecutionResults["sessions/list"]
}
function TaskAiLoader({
  client,
  taskId,
  onOpen,
  onGuard,
  onFiles,
}: CommonProps & { onOpen: (session: SessionIntent) => void }) {
  const copy = useExecutionCopy()
  const [reload, setReload] = useState(0)
  const [result, setResult] = useState<{
    attempt: number
    owner: BusinessClient
    data: HubData | null
    error: unknown
  } | null>(null)
  const current =
    result?.attempt === reload && result.owner === client ? result : null
  useEffect(() => {
    const lifetime = new AbortController()
    let scope: ExecutionClient | null = null
    void client
      .tasks("get", { taskId })
      .then(async (detail) => {
        if (lifetime.signal.aborted) return
        if (detail.task.id !== taskId) throw new ExecutionError("forbidden")
        const child = client.execution()
        scope = child
        const [profiles, sessions] = await Promise.all([
          child.json("profiles/list", { taskId }, lifetime.signal),
          child.json(
            "sessions/list",
            { taskId, cursor: null, limit: 50 },
            lifetime.signal
          ),
        ])
        if (!lifetime.signal.aborted)
          setResult({
            attempt: reload,
            owner: client,
            error: null,
            data: {
              client: child,
              task: detail.task,
              profiles: parseProfiles(profiles),
              sessions: parseSessions(sessions, taskId),
            },
          })
      })
      .catch((error: unknown) => {
        if (!lifetime.signal.aborted)
          setResult({
            attempt: reload,
            owner: client,
            data: null,
            error: safeTaskError(error),
          })
      })
    return () => {
      lifetime.abort()
      scope?.close()
    }
  }, [client, taskId, reload])
  return (
    <section className="mx-auto max-w-3xl space-y-6 p-5 sm:p-7">
      <header className="space-y-2">
        <h2 className="text-xl font-semibold tracking-tight">
          {copy.aiWorkspace}
        </h2>
        <p className="text-muted-foreground max-w-xl text-sm leading-relaxed">
          {copy.aiHint}
        </p>
        {onFiles && (
          <Action variant="outline" onClick={() => onFiles(taskId, null)}>
            {copy.documents}
          </Action>
        )}
      </header>
      {!current && (
        <p role="status" className="text-sm">
          {copy.loadingSessions}
        </p>
      )}
      {current?.error != null && (
        <div role="alert" className="space-y-3 text-sm">
          <p>{executionErrorMessage(current.error, copy)}</p>
          <Action
            variant="outline"
            onClick={() => setReload((value) => value + 1)}
          >
            {copy.retry}
          </Action>
        </div>
      )}
      {current?.data && (
        <SessionChoices
          key={reload}
          data={current.data}
          onOpen={onOpen}
          onGuard={onGuard}
          onReload={() => setReload((value) => value + 1)}
        />
      )}
    </section>
  )
}

type SessionRequest =
  | { kind: "start"; input: ExecutionInputs["sessions/start"]; label: string }
  | {
      kind: "continue"
      input: ExecutionInputs["sessions/continue"]
      label: string
    }

function SessionChoices({
  data,
  onOpen,
  onGuard,
  onReload,
}: {
  data: HubData
  onOpen: (intent: SessionIntent) => void
  onGuard: (guard: PromptGuard) => void
  onReload: () => void
}) {
  const copy = useExecutionCopy()
  const [profileId, setProfileId] = useState("")
  const [sessions, setSessions] = useState(data.sessions)
  const [attempt, setAttempt] = useState<{
    request: SessionRequest
    operation: OperationSummary | null
  } | null>(null)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const held = useRef<SessionRequest | null>(null)
  const reading = useRef(false)
  const scope = useRef<AbortController | null>(null)
  const guard = useRef(onGuard)
  const selected = data.profiles.profiles.find(
    (profile) => profile.id === profileId
  )
  const ready =
    selected?.custody === "original_operator" &&
    selected.readiness === "ready" &&
    selected.capabilities.start &&
    selected.modes.includes("chat") &&
    data.task.capabilities.submit
  useEffect(() => {
    guard.current = onGuard
  }, [onGuard])
  useEffect(() => {
    guard.current({ dirty: false, busy, unresolved: attempt !== null })
  }, [busy, attempt])
  useEffect(() => {
    const lifetime = new AbortController()
    scope.current = lifetime
    return () => {
      lifetime.abort()
      scope.current = null
    }
  }, [data.client])
  function validateSession(
    value: unknown,
    request: SessionRequest,
    sessionId?: string
  ) {
    const session = parseSession(value, { taskId: data.task.id, sessionId })
    if (
      session.mode !== "chat" ||
      (request.kind === "start"
        ? session.profileId !== request.input.profileId ||
          session.profileRevision !== request.input.expectedProfileRevision
        : session.id !== request.input.sessionId)
    )
      throw new ExecutionError("forbidden")
    return session
  }
  function receive(
    operation: OperationSummary,
    request: SessionRequest,
    session?: SessionSummary
  ) {
    if (held.current !== request || operation.id !== request.input.operationId)
      throw new ExecutionError("transport_unavailable")
    if (operation.status === "confirmed") {
      if (!session) throw new ExecutionError("transport_unavailable")
      held.current = null
      setAttempt(null)
      setSessions((value) => ({
        ...value,
        items: [
          session,
          ...value.items.filter((item) => item.id !== session.id),
        ],
      }))
      onOpen({
        taskId: data.task.id,
        sessionId: session.id,
        title: session.title,
      })
    } else setAttempt({ request, operation })
  }
  async function issue(request: SessionRequest) {
    const owner = scope.current
    if (!owner || owner.signal.aborted || held.current || reading.current)
      return
    held.current = request
    reading.current = true
    guard.current({ dirty: false, busy: true, unresolved: true })
    setAttempt({ request, operation: null })
    setBusy(true)
    setError(null)
    try {
      const value =
        request.kind === "start"
          ? await data.client.json(
              "sessions/start",
              request.input,
              owner.signal
            )
          : await data.client.json(
              "sessions/continue",
              request.input,
              owner.signal
            )
      if (owner.signal.aborted) return
      const row = record(value, ["session", "operation"])
      const operation = parseOperationSummary(row.operation)
      receive(
        operation,
        request,
        operation.status === "confirmed"
          ? validateSession(row.session, request)
          : undefined
      )
    } catch (caught) {
      if (!owner.signal.aborted) setError(caught)
    } finally {
      if (!owner.signal.aborted) {
        reading.current = false
        setBusy(false)
      }
    }
  }
  async function reconcile() {
    const owner = scope.current
    const request = held.current
    if (!owner || owner.signal.aborted || !request || reading.current) return
    reading.current = true
    setBusy(true)
    setError(null)
    try {
      const value = await data.client.json(
        "operations/get",
        { operationId: request.input.operationId, kind: request.kind },
        owner.signal
      )
      if (owner.signal.aborted) return
      const row = record(value, ["operation", "resourceId"])
      const operation = parseOperationSummary(row.operation)
      if (operation.id !== request.input.operationId)
        throw new ExecutionError("transport_unavailable")
      let session: SessionSummary | undefined
      if (operation.status === "confirmed") {
        const id = text(row.resourceId)
        const detail = await data.client.json(
          "sessions/get",
          { sessionId: id },
          owner.signal
        )
        if (owner.signal.aborted) return
        session = validateSession(
          record(detail, ["session"]).session,
          request,
          id
        )
      }
      receive(operation, request, session)
    } catch (caught) {
      if (!owner.signal.aborted) setError(caught)
    } finally {
      if (!owner.signal.aborted) {
        reading.current = false
        setBusy(false)
      }
    }
  }
  async function more() {
    const owner = scope.current
    if (
      !owner ||
      owner.signal.aborted ||
      !sessions.nextCursor ||
      held.current ||
      reading.current
    )
      return
    reading.current = true
    setBusy(true)
    setError(null)
    try {
      const next = parseSessions(
        await data.client.json(
          "sessions/list",
          { taskId: data.task.id, cursor: sessions.nextCursor, limit: 50 },
          owner.signal
        ),
        data.task.id
      )
      if (!owner.signal.aborted)
        setSessions((current) => ({
          ...next,
          items: [
            ...current.items,
            ...next.items.filter(
              (item) => !current.items.some((old) => old.id === item.id)
            ),
          ],
        }))
    } catch (caught) {
      if (!owner.signal.aborted) setError(caught)
    } finally {
      if (!owner.signal.aborted) {
        reading.current = false
        setBusy(false)
      }
    }
  }
  const locked = busy || attempt !== null
  return (
    <>
      <section className="space-y-4 rounded-xl border p-4 sm:p-5">
        <h3 className="font-semibold">
          <bdi>{data.task.title}</bdi>
        </h3>
        {data.profiles.profiles.length === 0 ? (
          <p className="text-muted-foreground text-sm leading-relaxed">
            {copy.noProfiles}
          </p>
        ) : (
          <>
            <Field label={copy.chooseProfile}>
              {(id) => (
                <select
                  id={id}
                  className={controlClass}
                  value={profileId}
                  disabled={locked}
                  onChange={(event) => setProfileId(event.target.value)}
                >
                  <option value="">{copy.profilePrompt}</option>
                  {data.profiles.profiles.map((profile) => (
                    <option
                      key={profile.id}
                      value={profile.id}
                      disabled={
                        profile.readiness !== "ready" ||
                        profile.custody !== "original_operator" ||
                        !profile.capabilities.start ||
                        !profile.modes.includes("chat")
                      }
                    >
                      {profile.label}
                      {profile.readiness !== "ready"
                        ? ` (${copy.profileBlocked})`
                        : ""}
                    </option>
                  ))}
                </select>
              )}
            </Field>
            {selected && (
              <div className="space-y-2 text-sm">
                <p>
                  <bdi>{selected.clientId}</bdi> ·{" "}
                  <bdi>{selected.model?.id ?? copy.modelUnavailable}</bdi>
                  {selected.model && (
                    <>
                      {" "}
                      · <bdi>{selected.model.reasoning}</bdi>
                    </>
                  )}
                </p>
                {!selected.capabilities.managedOutput && (
                  <p className="text-muted-foreground">
                    {copy.managedOutputUnavailable}
                  </p>
                )}
              </div>
            )}
            {!data.task.capabilities.submit && (
              <p className="text-muted-foreground text-sm">
                {copy.taskSubmitRequired}
              </p>
            )}
            <Action
              disabled={locked || !ready}
              onClick={() => {
                if (!ready || !selected) return
                void issue({
                  kind: "start",
                  label: selected.label,
                  input: {
                    operationId: crypto.randomUUID(),
                    taskId: data.task.id,
                    expectedTaskRevision: data.task.revision,
                    profileId: selected.id,
                    expectedProfileRevision: selected.revision,
                    mode: "chat",
                  },
                })
              }}
            >
              <MessageSquare aria-hidden="true" className="size-4" />
              {copy.startConversation}
            </Action>
          </>
        )}
        <Action variant="ghost" disabled={locked} onClick={onReload}>
          {copy.retry}
        </Action>
      </section>
      {busy && (
        <p role="status" className="text-sm">
          {copy.sending}
        </p>
      )}
      {attempt && !busy && (
        <div role="status" className="space-y-3 rounded-xl border p-4 text-sm">
          <p>
            <bdi>{attempt.request.label}</bdi>
          </p>
          <p>
            {attempt.operation?.status === "failed"
              ? copy.requestFailed
              : copy.requestNotConfirmed}
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
      <section className="space-y-3">
        <h3 className="font-semibold">{copy.savedSessions}</h3>
        {sessions.items.length === 0 && (
          <p className="text-muted-foreground text-sm">{copy.noSessions}</p>
        )}
        <ul className="divide-y rounded-xl border">
          {sessions.items.map((session) => (
            <li key={session.id} className="space-y-3 p-4">
              <div className="flex flex-wrap items-center justify-between gap-3">
                <bdi className="min-w-0 text-sm font-medium [overflow-wrap:anywhere]">
                  {session.title}
                </bdi>
                <span className="text-muted-foreground text-xs">
                  {copy[session.status]}
                </span>
              </div>
              {session.mode === "terminal" ? (
                <p className="text-muted-foreground text-sm">
                  {copy.terminalPending}
                </p>
              ) : (
                <div className="flex flex-wrap gap-2">
                  <Action
                    variant="outline"
                    disabled={locked || !session.capabilities.read}
                    onClick={() =>
                      onOpen({
                        taskId: data.task.id,
                        sessionId: session.id,
                        title: session.title,
                      })
                    }
                  >
                    {copy.openConversation}
                  </Action>
                  {session.capabilities.continue && (
                    <Action
                      variant="outline"
                      disabled={locked || !data.task.capabilities.submit}
                      onClick={() =>
                        void issue({
                          kind: "continue",
                          label: session.title,
                          input: {
                            operationId: crypto.randomUUID(),
                            sessionId: session.id,
                            expectedSessionRevision: session.revision,
                          },
                        })
                      }
                    >
                      {copy.continueSession}
                    </Action>
                  )}
                </div>
              )}
            </li>
          ))}
        </ul>
        {sessions.nextCursor && (
          <Action
            variant="outline"
            disabled={locked}
            onClick={() => void more()}
          >
            {copy.moreSessions}
          </Action>
        )}
      </section>
    </>
  )
}

export function BusinessSessionPane(
  props: CommonProps & { sessionId: string }
) {
  const copy = useExecutionCopy()
  if (!props.originalOperator || props.client.native)
    return (
      <p role="alert" className="p-5 text-sm">
        {copy.operatorOnly}
      </p>
    )
  return <SessionLoader key={`${props.taskId}:${props.sessionId}`} {...props} />
}
function SessionLoader({
  client,
  taskId,
  sessionId,
  onGuard,
  onFiles,
}: CommonProps & { sessionId: string }) {
  const copy = useExecutionCopy()
  const [result, setResult] = useState<{
    owner: BusinessClient
    scope: ExecutionClient
    session: SessionSummary
    task: Task
  } | null>(null)
  const [error, setError] = useState<unknown>(null)
  useEffect(() => {
    const lifetime = new AbortController()
    let scope: ExecutionClient | null = null
    async function load() {
      const child = client.execution()
      scope = child
      const [detail, value] = await Promise.all([
        client.tasks("get", { taskId }),
        child.json("sessions/get", { sessionId }, lifetime.signal),
      ])
      if (lifetime.signal.aborted) return
      if (detail.task.id !== taskId) throw new ExecutionError("forbidden")
      const session = parseSession(record(value, ["session"]).session, {
        taskId,
        sessionId,
      })
      setResult({ owner: client, scope: child, session, task: detail.task })
    }
    void load().catch((caught: unknown) => {
      if (!lifetime.signal.aborted) setError(safeTaskError(caught))
    })
    return () => {
      lifetime.abort()
      scope?.close()
    }
  }, [client, taskId, sessionId])
  if (error != null)
    return (
      <p role="alert" className="p-5 text-sm">
        {executionErrorMessage(error, copy)}
      </p>
    )
  if (!result || result.owner !== client)
    return (
      <p role="status" className="p-5 text-sm">
        {copy.loadingSessions}
      </p>
    )
  return (
    <SessionConversation
      client={result.scope}
      session={result.session}
      onGuard={onGuard}
      onFiles={onFiles ? () => onFiles(taskId, sessionId) : undefined}
      references={[
        {
          label: `${result.task.title} (${copy.savedTaskRevision}: ${result.task.revision})`,
          input: {
            kind: "task",
            taskId,
            expectedRevision: result.task.revision,
          },
        },
      ]}
    />
  )
}
