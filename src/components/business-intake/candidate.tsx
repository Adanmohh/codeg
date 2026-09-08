"use client"

import { useCallback, useEffect, useRef, useState } from "react"
import { Input } from "@/components/ui/input"
import { Action, Field, StatusBadge } from "@/components/business/ui"
import {
  AssignmentFields,
  MetadataFields,
} from "@/components/business/task-form"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import { useIntakeCopy } from "@/lib/business/intake-copy"
import type { Member } from "@/lib/business/identity"
import {
  sourceIsFresh,
  validPassages,
  type CandidateDetail,
  type Decision,
  type PreparedTask,
  type SourceDetail,
} from "@/lib/business/intake"
import type { TaskDetail } from "@/lib/business/tasks"
import {
  Check,
  IntakeError,
  Passages,
  PreparedTaskView,
  accessWasLost,
  outcomeIsUncertain,
  useSourceFreshness,
} from "./ui"
import { TaskTarget } from "./task-target"

type Result = { detail?: CandidateDetail; decision?: Decision }
type Mutation = {
  execute: () => Promise<Result>
  retainDraft?: boolean
  review?: boolean
}
export function CandidateReview({
  initial,
  source,
  client,
  actor,
  members,
  active,
  refresh,
  onDirty,
  onChanged,
  onTask,
}: {
  initial: CandidateDetail
  source: SourceDetail
  client: BusinessClient
  actor: Member
  members: Member[]
  active: boolean
  refresh: number
  onDirty: (dirty: boolean) => void
  onChanged: () => void
  onTask: (id: string) => void
}) {
  const copy = useIntakeCopy()
  const common = useBusinessCopy()
  const [detail, setDetail] = useState(initial)
  const [reading, setReading] = useState(source)
  const [current, setCurrent] = useState<CandidateDetail | null>(null)
  function initialDraft(value: CandidateDetail): PreparedTask | null {
    if (value.candidate.hasPreparedDraft) return value.candidate.draft
    const domain = value.candidate.capabilities.publicationDomains[0]
    return domain && value.candidate.disclosure === "fresh"
      ? {
          title: "",
          notes: "",
          domain,
          priority: "normal",
          dueDate: null,
          ownerId: actor.id,
          assigneeId: null,
          reviewerId: null,
        }
      : null
  }
  const [fields, setFields] = useState<PreparedTask | null>(() =>
    initialDraft(initial)
  )
  const [selection, setSelection] = useState(
    initial.passages.map((passage) => passage.id)
  )
  const [ownerSuggestion, setOwnerSuggestion] = useState(
    initial.candidate.ownerSuggestion ?? ""
  )
  const [dueSuggestion, setDueSuggestion] = useState(
    initial.candidate.dueSuggestion ?? ""
  )
  const [mode, setMode] = useState<"edit" | "review" | "link" | "discard">(
    "edit"
  )
  const [selecting, setSelecting] = useState(initial.candidate.requiresRebase)
  const [confirmed, setConfirmed] = useState(false)
  const [busy, setBusy] = useState(false)
  const [verified, setVerified] = useState(false)
  const [conflicted, setConflicted] = useState(false)
  const [unknown, setUnknown] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const [decision, setDecision] = useState(initial.decision)
  const [target, setTarget] = useState<TaskDetail | null>(null)
  const alive = useRef(true)
  const working = useRef(false)
  const serial = useRef(0)
  const invalidateReads = useCallback(() => {
    serial.current++
  }, [])
  const base = useRef(detail)
  const pending = useRef<Mutation | null>(null)
  const authoritative = current ?? detail
  const cap = authoritative.candidate.capabilities
  const fresh = useSourceFreshness(reading.source)
  const visible =
    active &&
    verified &&
    fresh &&
    reading.disclosure === "fresh" &&
    authoritative.candidate.disclosure === "fresh"
  const taskVisible =
    visible &&
    (!authoritative.candidate.hasPreparedDraft ||
      authoritative.candidate.draft !== null)
  const dirty =
    !!fields &&
    (JSON.stringify(fields) !== JSON.stringify(detail.candidate.draft) ||
      ownerSuggestion !== (detail.candidate.ownerSuggestion ?? "") ||
      dueSuggestion !== (detail.candidate.dueSuggestion ?? ""))
  const selectionDirty =
    JSON.stringify(selection) !==
    JSON.stringify(detail.passages.map((passage) => passage.id))
  const locked =
    busy || unknown || conflicted || current !== null || decision !== null
  const readySelection =
    visible &&
    validPassages(reading.passages, selection, reading.source.revision)
  const ready =
    visible &&
    !locked &&
    !detail.candidate.requiresRebase &&
    reading.source.revision === detail.candidate.sourceRevision
  useEffect(() => {
    base.current = detail
  }, [detail])
  useEffect(() => {
    onDirty(dirty || selectionDirty || busy || unknown)
    return () => onDirty(false)
  }, [dirty, selectionDirty, busy, unknown, onDirty])
  useEffect(() => {
    alive.current = true
    return () => {
      alive.current = false
      invalidateReads()
      pending.current = null
    }
  }, [invalidateReads])
  useEffect(() => {
    if (!visible || !taskVisible) setConfirmed(false)
  }, [visible, taskVisible])
  function removePrivate() {
    setVerified(false)
    setFields(null)
    setOwnerSuggestion("")
    setDueSuggestion("")
    setTarget(null)
    setSelection([])
    setConfirmed(false)
  }
  const loadCurrent = useCallback(
    async (compare = true) => {
      const attempt = ++serial.current
      setVerified(false)
      setConfirmed(false)
      setTarget(null)
      try {
        const [latest, full] = await Promise.all([
          client.intake("candidates/get", {
            candidateId: initial.candidate.id,
          }),
          client.intake("sources/get", { sourceId: initial.source.id }),
        ])
        if (!alive.current || attempt !== serial.current) return
        setReading(
          sourceIsFresh(full.source)
            ? full
            : { ...full, disclosure: "metadata_only", passages: [] }
        )
        setVerified(true)
        setError(null)
        if (latest.decision) {
          setDecision(latest.decision)
          setDetail(latest)
          setCurrent(null)
          setUnknown(false)
          pending.current = null
        } else if (
          compare ||
          latest.candidate.revision !== base.current.candidate.revision ||
          latest.source.revision !== base.current.source.revision ||
          latest.candidate.requiresRebase !==
            base.current.candidate.requiresRebase
        ) {
          setCurrent(latest)
          setConflicted(true)
        } else setDetail(latest)
        if (
          latest.candidate.disclosure !== "fresh" ||
          (latest.candidate.hasPreparedDraft && !latest.candidate.draft)
        ) {
          setFields(null)
          setOwnerSuggestion("")
          setDueSuggestion("")
        }
      } catch (caught) {
        if (!alive.current || attempt !== serial.current) return
        setError(caught)
        if (accessWasLost(caught)) {
          setFields(null)
          setOwnerSuggestion("")
          setDueSuggestion("")
          setSelection([])
          setCurrent(null)
        }
      }
    },
    [client, initial.candidate.id, initial.source.id]
  )
  useEffect(() => {
    if (!active) {
      setVerified(false)
      serial.current++
      return
    }
    void loadCurrent(false)
    const check = () => {
      if (document.visibilityState === "visible" && !working.current)
        void loadCurrent(false)
    }
    window.addEventListener("focus", check)
    document.addEventListener("visibilitychange", check)
    return () => {
      window.removeEventListener("focus", check)
      document.removeEventListener("visibilitychange", check)
      invalidateReads()
    }
  }, [active, loadCurrent, refresh, invalidateReads])

  async function run(mutation: Mutation) {
    if (working.current || !active) return
    working.current = true
    serial.current++
    pending.current = mutation
    setBusy(true)
    setConfirmed(false)
    setError(null)
    try {
      const result = await mutation.execute()
      if (!alive.current) return
      if (result.detail) {
        setDetail(result.detail)
        base.current = result.detail
        setCurrent(null)
        setConflicted(false)
        setSelection(result.detail.passages.map((passage) => passage.id))
        setSelecting(false)
        if (!mutation.retainDraft) {
          setFields(initialDraft(result.detail))
          setOwnerSuggestion(result.detail.candidate.ownerSuggestion ?? "")
          setDueSuggestion(result.detail.candidate.dueSuggestion ?? "")
        }
        if (result.detail.decision) setDecision(result.detail.decision)
      }
      if (result.decision) {
        setDecision(result.decision)
        setFields(null)
        setTarget(null)
      }
      if (mutation.review) setMode("review")
      setUnknown(false)
      pending.current = null
      onChanged()
    } catch (caught) {
      if (!alive.current) return
      setError(caught)
      const uncertain = outcomeIsUncertain(caught)
      setUnknown(uncertain)
      if (!uncertain) pending.current = null
      if (caught instanceof BusinessError && caught.kind === "conflict") {
        setConflicted(true)
        setCurrent(null)
      }
      if (accessWasLost(caught)) removePrivate()
    } finally {
      working.current = false
      if (alive.current) setBusy(false)
    }
  }
  function adopt(keep: boolean) {
    if (!current || unknown) return
    const value = current
    setDetail(value)
    base.current = value
    setCurrent(null)
    setConflicted(false)
    setConfirmed(false)
    setTarget(null)
    if (!keep) {
      setFields(initialDraft(value))
      setOwnerSuggestion(value.candidate.ownerSuggestion ?? "")
      setDueSuggestion(value.candidate.dueSuggestion ?? "")
    }
    setSelection(
      value.candidate.requiresRebase
        ? []
        : value.passages.map((passage) => passage.id)
    )
    setSelecting(value.candidate.requiresRebase)
    setMode("edit")
    setError(null)
  }
  function changeMode(value: typeof mode) {
    setMode(value)
    setConfirmed(false)
    setTarget(null)
  }
  function selectCurrent() {
    if (
      !readySelection ||
      locked ||
      !cap.select ||
      reading.source.revision === null
    )
      return
    const input = {
      operationId: crypto.randomUUID(),
      candidateId: detail.candidate.id,
      expectedRevision: detail.candidate.revision,
      expectedSourceRevision: reading.source.revision,
      passageIds: selection,
    }
    void run({
      execute: async () => ({
        detail: await client.intake("candidates/select", input),
      }),
      retainDraft: true,
    })
  }
  function saveDraft() {
    if (
      !fields ||
      !taskVisible ||
      locked ||
      !cap.edit ||
      !readySelection ||
      reading.source.revision === null ||
      !fields.title.trim()
    )
      return
    const input = {
      operationId: crypto.randomUUID(),
      candidateId: detail.candidate.id,
      expectedRevision: detail.candidate.revision,
      expectedSourceRevision: reading.source.revision,
      passageIds: selection,
      task: fields,
      ownerSuggestion: ownerSuggestion || null,
      dueSuggestion: dueSuggestion || null,
    }
    void run({
      execute: async () => ({
        detail: await client.intake("candidates/edit", input),
      }),
      review: true,
    })
  }
  function accept() {
    const draft = detail.candidate.draft
    if (
      !ready ||
      !taskVisible ||
      !draft ||
      dirty ||
      selectionDirty ||
      !confirmed ||
      !cap.accept ||
      !sourceIsFresh(reading.source) ||
      !cap.publicationDomains.includes(draft.domain)
    )
      return
    const input = {
      operationId: crypto.randomUUID(),
      candidateId: detail.candidate.id,
      expectedRevision: detail.candidate.revision,
      expectedSourceRevision: detail.candidate.sourceRevision,
      publishToDomain: draft.domain,
    }
    void run({
      execute: async () => ({
        decision: (await client.intake("candidates/accept", input)).decision,
      }),
    })
  }
  function link() {
    if (
      !ready ||
      !confirmed ||
      !cap.link ||
      !target ||
      !target.task.capabilities.edit ||
      !cap.publicationDomains.includes(target.task.domain) ||
      !sourceIsFresh(reading.source)
    )
      return
    const input = {
      operationId: crypto.randomUUID(),
      candidateId: detail.candidate.id,
      expectedRevision: detail.candidate.revision,
      expectedSourceRevision: detail.candidate.sourceRevision,
      taskId: target.task.id,
      expectedTaskRevision: target.task.revision,
      publishToDomain: target.task.domain,
    }
    void run({
      execute: async () => ({
        decision: (await client.intake("candidates/link", input)).decision,
      }),
    })
  }
  const version = (
    <span className="text-muted-foreground text-xs tabular-nums">
      {common.sourceVersion} {detail.candidate.revision} · {copy.sourceRevision}{" "}
      {detail.candidate.sourceRevision}
    </span>
  )
  if (decision)
    return (
      <section
        className="border-border space-y-4 rounded-2xl border p-5"
        aria-label={copy.outcome}
      >
        <p className="text-primary text-sm font-medium">{copy.outcome}</p>
        <h2 className="text-xl font-semibold">{copy[decision.kind]}</h2>
        {decision.task.state === "restricted" && (
          <p className="text-muted-foreground text-sm">{copy.restrictedTask}</p>
        )}
        {decision.task.state === "accessible" && (
          <Action
            onClick={() => {
              if (decision.task.state === "accessible")
                onTask(decision.task.taskId)
            }}
          >
            {copy.openTask}
          </Action>
        )}
        {decision.kind === "discarded" && (
          <p className="text-muted-foreground text-sm">{copy.discardHint}</p>
        )}
      </section>
    )
  return (
    <section className="space-y-5" aria-label={copy.draft}>
      <header className="space-y-2">
        <h2 className="text-xl font-semibold">{copy.draft}</h2>
        {version}
        <p className="text-muted-foreground text-sm leading-relaxed">
          {copy.draftHint}
        </p>
      </header>
      {error != null && <IntakeError error={error} />}
      {unknown && (
        <div
          role="status"
          className="border-border space-y-3 rounded-xl border p-4 text-sm"
        >
          <p>{copy.unknown}</p>
          <div className="flex flex-wrap gap-2">
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => void loadCurrent()}
            >
              {copy.recover}
            </Action>
            <Action
              variant="outline"
              disabled={busy || !pending.current}
              onClick={() => {
                if (pending.current) void run(pending.current)
              }}
            >
              {copy.retryExact}
            </Action>
          </div>
        </div>
      )}
      {(conflicted || current) && (
        <div className="border-border space-y-4 rounded-xl border p-4 text-sm">
          <p role="status">{copy.conflict}</p>
          <p>
            {copy.draftBase}: {version}
          </p>
          {!current ? (
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => void loadCurrent()}
            >
              {copy.loadCurrent}
            </Action>
          ) : (
            <section aria-label={copy.currentSaved} className="space-y-3">
              <h3 className="font-medium">{copy.currentSaved}</h3>
              <p>
                {copy[current.candidate.state]} · {common.sourceVersion}{" "}
                {current.candidate.revision} · {copy.sourceRevision}{" "}
                {current.candidate.sourceRevision}
              </p>
              {taskVisible && current.candidate.draft && (
                <PreparedTaskView
                  task={current.candidate.draft}
                  members={members}
                />
              )}
              {current.candidate.hasPreparedDraft &&
                !current.candidate.draft && <p>{copy.withheldDraft}</p>}
              <div className="flex flex-wrap gap-2">
                <Action
                  variant="outline"
                  disabled={busy || unknown}
                  onClick={() => adopt(false)}
                >
                  {copy.adoptSaved}
                </Action>
                {fields && taskVisible && (
                  <Action
                    variant="outline"
                    disabled={busy || unknown}
                    onClick={() => adopt(true)}
                  >
                    {copy.adopt}
                  </Action>
                )}
              </div>
            </section>
          )}
        </div>
      )}
      {!visible && (
        <div
          role="status"
          className="bg-muted/50 space-y-2 rounded-xl p-4 text-sm"
        >
          <h3 className="font-medium">{copy.unavailable}</h3>
          <p>{copy.unavailableHint}</p>
          <Action
            variant="outline"
            disabled={busy}
            onClick={() => void loadCurrent()}
          >
            {copy.loadCurrent}
          </Action>
        </div>
      )}
      {visible && (
        <>
          {detail.candidate.requiresRebase && (
            <div className="bg-muted/50 space-y-2 rounded-xl p-4 text-sm">
              <p className="font-medium">{copy.rebase}</p>
              <p>{copy.rebaseHint}</p>
            </div>
          )}
          <section aria-label={copy.selected} className="space-y-3">
            <h3 className="text-sm font-medium">{copy.selected}</h3>
            {selecting ? (
              <>
                <p className="text-muted-foreground text-sm">
                  {copy.selectHint}
                </p>
                <Passages
                  passages={reading.passages}
                  selected={selection}
                  onSelect={(ids) => {
                    setSelection(ids)
                    setConfirmed(false)
                    setTarget(null)
                  }}
                  disabled={locked || !cap.select}
                />
                <Action
                  variant="outline"
                  disabled={locked || !cap.select || !readySelection}
                  onClick={selectCurrent}
                >
                  {copy.confirmSelection}
                </Action>
                {!readySelection && (
                  <p className="text-sm">{copy.selectedInvalid}</p>
                )}
              </>
            ) : (
              <>
                <Passages passages={detail.passages} />
                {cap.select && (
                  <Action
                    variant="outline"
                    disabled={locked}
                    onClick={() => {
                      setSelecting(true)
                      setConfirmed(false)
                      setTarget(null)
                    }}
                  >
                    {copy.rebase}
                  </Action>
                )}
              </>
            )}
          </section>
          <div className="flex flex-wrap gap-2">
            {taskVisible && cap.edit && (
              <Action
                variant={mode === "edit" ? "secondary" : "ghost"}
                disabled={busy || unknown}
                onClick={() => changeMode("edit")}
              >
                {common.edit}
              </Action>
            )}
            {taskVisible && detail.candidate.draft && (
              <Action
                variant={mode === "review" ? "secondary" : "ghost"}
                disabled={busy || unknown}
                onClick={() => changeMode("review")}
              >
                {copy.reviewDraft}
              </Action>
            )}
            {cap.link && (
              <Action
                variant={mode === "link" ? "secondary" : "ghost"}
                disabled={!ready || selecting}
                onClick={() => changeMode("link")}
              >
                {copy.link}
              </Action>
            )}
          </div>
          {!taskVisible && authoritative.candidate.hasPreparedDraft && (
            <p className="text-sm">{copy.withheldDraft}</p>
          )}
          {mode === "edit" && taskVisible && fields && cap.edit && (
            <form
              className="space-y-5"
              onSubmit={(event) => {
                event.preventDefault()
                saveDraft()
              }}
            >
              <MetadataFields
                value={fields}
                setValue={(value) => {
                  setFields({ ...fields, ...value })
                  setConfirmed(false)
                }}
                domains={cap.publicationDomains}
                disabled={locked}
              />
              <AssignmentFields
                value={fields}
                setValue={(value) => {
                  setFields({ ...fields, ...value })
                  setConfirmed(false)
                }}
                members={members}
                domain={fields.domain}
                disabled={locked}
                selfOnly={actor.role === "member" ? actor.id : undefined}
              />
              <div className="grid gap-4 sm:grid-cols-2">
                <Field label={copy.ownerSuggestion} hint={copy.suggestionHint}>
                  {(id) => (
                    <Input
                      id={id}
                      value={ownerSuggestion}
                      onChange={(event) => {
                        setOwnerSuggestion(event.target.value)
                        setConfirmed(false)
                      }}
                      maxLength={240}
                      dir="auto"
                      disabled={locked}
                      className="min-h-11 rounded-xl"
                    />
                  )}
                </Field>
                <Field label={copy.dueSuggestion} hint={copy.suggestionHint}>
                  {(id) => (
                    <Input
                      id={id}
                      value={dueSuggestion}
                      onChange={(event) => {
                        setDueSuggestion(event.target.value)
                        setConfirmed(false)
                      }}
                      maxLength={240}
                      dir="auto"
                      disabled={locked}
                      className="min-h-11 rounded-xl"
                    />
                  )}
                </Field>
              </div>
              <Action
                type="submit"
                disabled={
                  locked ||
                  !readySelection ||
                  !fields.title.trim() ||
                  !cap.publicationDomains.includes(fields.domain)
                }
              >
                {copy.saveDraft}
              </Action>
            </form>
          )}
          {mode === "review" && taskVisible && detail.candidate.draft && (
            <section
              aria-label={copy.reviewDraft}
              className="border-primary/40 space-y-5 rounded-xl border p-4 sm:p-5"
            >
              <h3 className="text-lg font-semibold">{copy.reviewDraft}</h3>
              <PreparedTaskView
                task={detail.candidate.draft}
                members={members}
              />
              <div className="border-border space-y-2 border-t pt-4 text-sm leading-relaxed">
                <h4 className="font-semibold">
                  {copy.audience}: {common[detail.candidate.draft.domain]}
                </h4>
                <p>{copy.audienceHint}</p>
                <p>{copy.retainHint}</p>
              </div>
              {(dirty || selectionDirty) && (
                <p role="status" className="text-sm">
                  {copy.editsKept}
                </p>
              )}
              <Check
                checked={confirmed}
                onChange={setConfirmed}
                disabled={
                  !ready || dirty || selectionDirty || !cap.accept || selecting
                }
              >
                {copy.acceptConfirm}
              </Check>
              <Action
                disabled={
                  !ready ||
                  !confirmed ||
                  dirty ||
                  selectionDirty ||
                  !cap.accept ||
                  selecting
                }
                onClick={accept}
              >
                {copy.acceptTask}
              </Action>
            </section>
          )}
          {mode === "link" && (
            <section className="space-y-5" aria-label={copy.link}>
              <p className="text-muted-foreground text-sm leading-relaxed">
                {copy.linkHint}
              </p>
              <TaskTarget
                client={client}
                domains={cap.publicationDomains}
                disabled={!ready || !cap.link}
                onSelected={(value) => {
                  setTarget(value)
                  setConfirmed(false)
                }}
              />
              {target && (
                <section
                  aria-label={copy.targetVersion}
                  className="border-border space-y-4 rounded-xl border p-4"
                >
                  <h3 className="font-semibold">{copy.targetVersion}</h3>
                  <div className="flex flex-wrap items-center gap-3">
                    <StatusBadge status={target.task.status} />
                    <span className="text-sm">
                      {common.sourceVersion} {target.task.revision}
                    </span>
                  </div>
                  <PreparedTaskView task={target.task} members={members} />
                  <p className="text-sm leading-relaxed">
                    {copy.audience}: {common[target.task.domain]}.{" "}
                    {copy.audienceHint}
                  </p>
                  <Check
                    checked={confirmed}
                    onChange={setConfirmed}
                    disabled={!ready || !cap.link}
                  >
                    {copy.linkConfirm}
                  </Check>
                  <Action
                    disabled={!ready || !confirmed || !cap.link}
                    onClick={link}
                  >
                    {copy.confirmLink}
                  </Action>
                </section>
              )}
            </section>
          )}
        </>
      )}
      {cap.discard && (
        <div className="border-border space-y-3 border-t pt-5">
          {mode !== "discard" ? (
            <Action
              variant="ghost"
              disabled={busy || unknown || !!current || conflicted}
              onClick={() => changeMode("discard")}
            >
              {copy.discard}
            </Action>
          ) : (
            <>
              <p className="text-muted-foreground text-sm">
                {copy.discardHint}
              </p>
              <Check
                checked={confirmed}
                onChange={setConfirmed}
                disabled={locked}
              >
                {copy.discardConfirm}
              </Check>
              <Action
                variant="destructive"
                disabled={locked || !confirmed}
                onClick={() => {
                  if (locked || !confirmed || !cap.discard) return
                  const input = {
                    operationId: crypto.randomUUID(),
                    candidateId: detail.candidate.id,
                    expectedRevision: detail.candidate.revision,
                  }
                  void run({
                    execute: async () => ({
                      decision: await client.intake(
                        "candidates/discard",
                        input
                      ),
                    }),
                  })
                }}
              >
                {copy.discard}
              </Action>
            </>
          )}
        </div>
      )}
    </section>
  )
}
