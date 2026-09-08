"use client"

import { useEffect, useState } from "react"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import type { Member } from "@/lib/business/identity"
import {
  personFor,
  type Assignment,
  type Task,
  type TaskDetail,
  type TaskFields,
} from "@/lib/business/tasks"
import { Action, ErrorNotice, Field, Modal, Person, StatusBadge } from "./ui"
import { AssignmentFields, MetadataFields } from "./task-form"
import { DueDay } from "./work-list"
import { ActivityList } from "./activity"

function fieldsOf(task: Task): TaskFields {
  return {
    title: task.title,
    notes: task.notes,
    domain: task.domain,
    priority: task.priority,
    dueDate: task.dueDate,
  }
}
function assignmentOf(task: Task): Assignment {
  return {
    ownerId: task.ownerId,
    assigneeId: task.assigneeId,
    reviewerId: task.reviewerId,
  }
}

export function TaskDetailDialog({
  taskId,
  initial,
  client,
  actor,
  members,
  legacyOperator = false,
  onClose,
  onChanged,
}: {
  taskId: string
  initial?: TaskDetail
  client: BusinessClient
  actor: Member
  members: Member[]
  legacyOperator?: boolean
  onClose: () => void
  onChanged: () => void
}) {
  const copy = useBusinessCopy()
  const [detail, setDetail] = useState<TaskDetail | null>(initial ?? null)
  const [error, setError] = useState<unknown>(null)
  const [reload, setReload] = useState(0)
  useEffect(() => {
    if (initial && reload === 0) return
    let cancelled = false
    client
      .tasks("get", { taskId })
      .then((value) => {
        if (!cancelled) {
          setDetail(value)
          setError(null)
        }
      })
      .catch((caught) => {
        if (!cancelled) setError(caught)
      })
    return () => {
      cancelled = true
    }
  }, [client, taskId, initial, reload])
  if (!detail)
    return (
      <Modal title={copy.loading} onClose={onClose}>
        {error != null && (
          <ErrorNotice error={error}>
            <Action variant="outline" onClick={() => setReload(reload + 1)}>
              {copy.retry}
            </Action>
          </ErrorNotice>
        )}
      </Modal>
    )
  return (
    <TaskEditor
      initial={detail}
      client={client}
      actor={actor}
      members={members}
      legacyOperator={legacyOperator}
      onClose={onClose}
      onChanged={onChanged}
    />
  )
}

function TaskEditor({
  initial,
  client,
  actor,
  members,
  legacyOperator,
  onClose,
  onChanged,
}: {
  initial: TaskDetail
  client: BusinessClient
  actor: Member
  members: Member[]
  legacyOperator: boolean
  onClose: () => void
  onChanged: () => void
}) {
  const copy = useBusinessCopy()
  const [detail, setDetail] = useState(initial)
  const [fields, setFields] = useState(fieldsOf(initial.task))
  const [assignment, setAssignment] = useState(assignmentOf(initial.task))
  const [mode, setMode] = useState<"view" | "edit" | "assign">("view")
  const [note, setNote] = useState("")
  const [deliverable, setDeliverable] = useState("")
  const [reviewNote, setReviewNote] = useState("")
  const [reviewed, setReviewed] = useState(false)
  const [workTask, setWorkTask] = useState("")
  const [entrustConfirmed, setEntrustConfirmed] = useState(false)
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const [conflicted, setConflicted] = useState(false)
  const [current, setCurrent] = useState<TaskDetail | null>(null)
  const [discard, setDiscard] = useState(false)
  const [confirm, setConfirm] = useState<"cancel" | "archive" | null>(null)
  const task = detail.task
  const cap = task.capabilities
  const assignedAgent = members.find(
    (person) => person.id === task.assigneeId && person.kind === "agent"
  )
  const validWorkTask =
    Number.isInteger(Number(workTask)) &&
    Number(workTask) >= 1 &&
    Number(workTask) <= 2147483647
  const revision = { taskId: task.id, expectedRevision: task.revision }
  const dirty =
    JSON.stringify(fields) !== JSON.stringify(fieldsOf(task)) ||
    JSON.stringify(assignment) !== JSON.stringify(assignmentOf(task)) ||
    !!note ||
    !!deliverable ||
    !!reviewNote ||
    !!workTask
  const locked = busy || conflicted || !!current
  function close() {
    if (busy) return
    if (dirty) setDiscard(true)
    else onClose()
  }
  function failed(caught: unknown) {
    setError(caught)
    setConfirm(null)
    if (caught instanceof BusinessError && caught.kind === "conflict")
      setConflicted(true)
  }
  async function mutate(
    operation: () => Promise<TaskDetail>,
    after: () => void = () => {}
  ) {
    if (locked) return
    setBusy(true)
    setError(null)
    try {
      const saved = await operation()
      setDetail(saved)
      setReviewed(false)
      setEntrustConfirmed(false)
      setFields(fieldsOf(saved.task))
      setAssignment(assignmentOf(saved.task))
      after()
      onChanged()
    } catch (caught) {
      failed(caught)
    } finally {
      setBusy(false)
    }
  }
  async function loadCurrent() {
    setBusy(true)
    try {
      setCurrent(await client.tasks("get", { taskId: task.id }))
    } catch (caught) {
      failed(caught)
    } finally {
      setBusy(false)
    }
  }
  function adoptCurrent(keepDraft: boolean) {
    if (!current) return
    if (mode !== "edit") setFields(fieldsOf(current.task))
    if (mode !== "assign") setAssignment(assignmentOf(current.task))
    setDetail(current)
    setReviewed(false)
    setEntrustConfirmed(false)
    if (!keepDraft) {
      setFields(fieldsOf(current.task))
      setAssignment(assignmentOf(current.task))
      setNote("")
      setDeliverable("")
      setReviewNote("")
      setWorkTask("")
      setMode("view")
    }
    setCurrent(null)
    setConflicted(false)
    setError(null)
  }
  return (
    <Modal title={task.title} onClose={close} wide>
      <div className="flex flex-wrap items-center gap-3">
        <StatusBadge status={task.status} />
        <span className="text-muted-foreground text-xs">
          {copy[task.domain]} · {copy[task.priority]}
        </span>
        {task.archivedAt && (
          <span className="text-muted-foreground text-xs">{copy.archived}</span>
        )}
        <span className="text-muted-foreground ms-auto text-xs tabular-nums">
          {copy.sourceVersion} {task.revision}
        </span>
      </div>
      {error != null && (
        <ErrorNotice error={error}>
          {conflicted && <p>{copy.conflictHint}</p>}
        </ErrorNotice>
      )}
      {conflicted && !current && (
        <Action
          variant="outline"
          disabled={busy}
          onClick={() => void loadCurrent()}
        >
          {copy.loadCurrent}
        </Action>
      )}
      {current && (
        <section className="bg-muted/40 space-y-4 rounded-xl border p-4">
          <h3 className="font-semibold">{copy.currentVersion}</h3>
          <p className="text-sm font-medium">
            <bdi>{current.task.title}</bdi>
          </p>
          <SavedTask detail={current} members={members} />
          <div className="flex flex-wrap gap-3">
            <Action disabled={busy} onClick={() => adoptCurrent(true)}>
              {copy.keepDraft}
            </Action>
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => adoptCurrent(false)}
            >
              {copy.discardDraft}
            </Action>
          </div>
        </section>
      )}
      {mode === "view" ? (
        <>
          <SavedTask detail={detail} members={members} />
          <div className="flex flex-wrap gap-2">
            {cap.edit && (
              <Action
                variant="outline"
                disabled={locked}
                onClick={() => setMode("edit")}
              >
                {copy.edit}
              </Action>
            )}
            {cap.assign && (
              <Action
                variant="outline"
                disabled={locked}
                onClick={() => setMode("assign")}
              >
                {copy.assignment}
              </Action>
            )}
            {cap.progress && task.status !== "in_progress" && (
              <Action
                variant="outline"
                disabled={locked}
                onClick={() =>
                  void mutate(() =>
                    client.tasks("progress", {
                      ...revision,
                      status: "in_progress",
                    })
                  )
                }
              >
                {copy.startWork}
              </Action>
            )}
            {cap.progress && task.status !== "todo" && (
              <Action
                variant="outline"
                disabled={locked}
                onClick={() =>
                  void mutate(() =>
                    client.tasks("progress", { ...revision, status: "todo" })
                  )
                }
              >
                {copy.returnTodo}
              </Action>
            )}
            {cap.progress && ["todo", "in_progress"].includes(task.status) && (
              <Action
                variant="outline"
                disabled={locked}
                onClick={() =>
                  void mutate(() =>
                    client.tasks("progress", { ...revision, status: "review" })
                  )
                }
              >
                {copy.submitReview}
              </Action>
            )}
          </div>
          {!Object.values(cap).some(Boolean) && (
            <p className="bg-muted/40 rounded-xl p-4 text-sm leading-relaxed">
              {copy.readOnlyHint}
            </p>
          )}
        </>
      ) : (
        <form
          className="space-y-5"
          onSubmit={(event) => {
            event.preventDefault()
            void mutate(
              () =>
                mode === "edit"
                  ? client.tasks("update", {
                      ...revision,
                      ...fields,
                      title: fields.title.trim(),
                    })
                  : client.tasks("assign", { ...revision, ...assignment }),
              () => setMode("view")
            )
          }}
        >
          {mode === "edit" ? (
            <MetadataFields
              value={fields}
              setValue={setFields}
              domains={actor.domains}
              disabled={locked || !cap.edit}
            />
          ) : (
            <AssignmentFields
              value={assignment}
              setValue={setAssignment}
              members={members}
              domain={task.domain}
              disabled={locked || !cap.assign}
            />
          )}
          <div className="flex flex-wrap justify-end gap-3">
            <Action
              type="button"
              variant="outline"
              disabled={busy}
              onClick={() => {
                setFields(fieldsOf(task))
                setAssignment(assignmentOf(task))
                setMode("view")
              }}
            >
              {copy.cancel}
            </Action>
            <Action
              type="submit"
              disabled={
                locked ||
                !fields.title.trim() ||
                (mode === "edit" ? !cap.edit : !cap.assign)
              }
            >
              {busy ? copy.saving : copy.save}
            </Action>
          </div>
        </form>
      )}
      {mode === "view" && (
        <>
          {cap.review && task.status === "review" && (
            <section className="border-primary/25 bg-primary/5 space-y-4 rounded-2xl border p-5">
              <h3 className="font-semibold">{copy.review}</h3>
              <Field label={copy.reviewComment}>
                {(id) => (
                  <Textarea
                    id={id}
                    value={reviewNote}
                    onChange={(event) => setReviewNote(event.target.value)}
                    maxLength={20000}
                    disabled={locked}
                    dir="auto"
                  />
                )}
              </Field>
              <label className="flex min-h-11 items-start gap-3 text-sm leading-relaxed">
                <input
                  type="checkbox"
                  className="accent-primary mt-1 size-4 shrink-0"
                  checked={reviewed}
                  onChange={(event) => setReviewed(event.target.checked)}
                  disabled={locked}
                />
                {copy.reviewedTask}
              </label>
              <div className="flex flex-wrap gap-3">
                <Action
                  disabled={locked || !reviewed}
                  onClick={() =>
                    void mutate(
                      () =>
                        client.tasks("review", {
                          ...revision,
                          decision: "accept",
                          comment: reviewNote,
                        }),
                      () => setReviewNote("")
                    )
                  }
                >
                  {copy.accept}
                </Action>
                <Action
                  variant="outline"
                  disabled={locked || !reviewed}
                  onClick={() =>
                    void mutate(
                      () =>
                        client.tasks("review", {
                          ...revision,
                          decision: "return",
                          comment: reviewNote,
                        }),
                      () => setReviewNote("")
                    )
                  }
                >
                  {copy.returnWork}
                </Action>
              </div>
            </section>
          )}
          {cap.submit && (
            <details className="rounded-xl border p-4">
              <summary className="focus-visible:ring-ring flex min-h-11 cursor-pointer items-center rounded-lg text-sm font-medium outline-none focus-visible:ring-2">
                {copy.submitReview}
              </summary>
              <form
                className="mt-3 space-y-4"
                onSubmit={(event) => {
                  event.preventDefault()
                  void mutate(
                    () =>
                      client.tasks("submit", {
                        ...revision,
                        body: deliverable,
                      }),
                    () => setDeliverable("")
                  )
                }}
              >
                <Field label={copy.deliverable} hint={copy.submitHint}>
                  {(id) => (
                    <Textarea
                      id={id}
                      value={deliverable}
                      onChange={(event) => setDeliverable(event.target.value)}
                      required
                      maxLength={20000}
                      disabled={locked}
                      className="min-h-28"
                      dir="auto"
                    />
                  )}
                </Field>
                <Action type="submit" disabled={locked || !deliverable.trim()}>
                  {copy.submitReview}
                </Action>
              </form>
            </details>
          )}
          {cap.comment && (
            <form
              className="space-y-3 border-t pt-5"
              onSubmit={(event) => {
                event.preventDefault()
                void mutate(
                  () => client.tasks("note", { ...revision, body: note }),
                  () => setNote("")
                )
              }}
            >
              <Field label={copy.addNote} hint={copy.noteHint}>
                {(id) => (
                  <Textarea
                    id={id}
                    value={note}
                    onChange={(event) => setNote(event.target.value)}
                    maxLength={20000}
                    required
                    disabled={locked}
                    dir="auto"
                  />
                )}
              </Field>
              <div className="flex justify-end">
                <Action
                  type="submit"
                  variant="outline"
                  disabled={locked || !note.trim()}
                >
                  {copy.addNote}
                </Action>
              </div>
            </form>
          )}
        </>
      )}
      <ActivityList detail={detail} members={members} />
      <details className="border-t pt-3">
        <summary className="focus-visible:ring-ring flex min-h-11 cursor-pointer items-center rounded-lg text-sm font-medium outline-none focus-visible:ring-2">
          {copy.execution}
        </summary>
        <div className="space-y-4 py-3 text-sm">
          {detail.execution ? (
            <p>
              {copy.executionLinked}{" "}
              <span className="tabular-nums">
                #{detail.execution.workTaskId}
              </span>{" "}
              · {detail.execution.active ? copy.active : copy.executionInactive}
            </p>
          ) : (
            <p className="text-muted-foreground leading-relaxed">
              {copy.noExecution}
            </p>
          )}
          {cap.linkExecution && assignedAgent && (
            <form
              className="space-y-3"
              onSubmit={(event) => {
                event.preventDefault()
                void mutate(
                  () =>
                    client.tasks("link-execution", {
                      ...revision,
                      workTaskId: Number(workTask),
                    }),
                  () => setWorkTask("")
                )
              }}
            >
              <Field label={copy.workTaskId} hint={copy.linkHint}>
                {(id) => (
                  <Input
                    id={id}
                    type="number"
                    min={1}
                    max={2147483647}
                    step={1}
                    value={workTask}
                    onChange={(event) => {
                      setWorkTask(event.target.value)
                      setEntrustConfirmed(false)
                    }}
                    className="min-h-11 rounded-xl"
                    disabled={locked}
                    required
                    dir="ltr"
                  />
                )}
              </Field>
              {legacyOperator && (
                <div className="space-y-3 rounded-xl border p-4">
                  <p className="text-muted-foreground text-sm leading-relaxed">
                    {copy.entrustHint}
                  </p>
                  <label className="flex min-h-11 items-start gap-3 text-sm leading-relaxed">
                    <input
                      type="checkbox"
                      className="accent-primary mt-1 size-4 shrink-0"
                      checked={entrustConfirmed}
                      onChange={(event) =>
                        setEntrustConfirmed(event.target.checked)
                      }
                      disabled={locked || !validWorkTask}
                    />
                    <span>
                      {copy.entrustConfirm}{" "}
                      <bdi>{assignedAgent.displayName}</bdi>.
                    </span>
                  </label>
                  <Action
                    variant="outline"
                    disabled={locked || !validWorkTask || !entrustConfirmed}
                    onClick={() =>
                      void mutate(() =>
                        client.tasks("entrust-execution", {
                          ...revision,
                          workTaskId: Number(workTask),
                        })
                      )
                    }
                  >
                    {copy.entrustExecution}
                  </Action>
                </div>
              )}
              <Action
                type="submit"
                variant="outline"
                disabled={locked || !validWorkTask}
              >
                {copy.linkExecution}
              </Action>
            </form>
          )}
        </div>
      </details>
      <div className="flex flex-wrap justify-end gap-3 border-t pt-4">
        {cap.cancel && (
          <Action
            variant="destructive"
            disabled={locked}
            onClick={() => setConfirm("cancel")}
          >
            {copy.cancelTask}
          </Action>
        )}
        {cap.archive && (
          <Action
            variant="outline"
            disabled={locked}
            onClick={() => setConfirm("archive")}
          >
            {task.archivedAt ? copy.restore : copy.archive}
          </Action>
        )}
      </div>
      {confirm && (
        <Modal
          title={
            confirm === "cancel"
              ? copy.cancelTask
              : task.archivedAt
                ? copy.restore
                : copy.archive
          }
          description={
            confirm === "cancel"
              ? copy.cancelTaskHint
              : task.archivedAt
                ? copy.restoreHint
                : copy.archiveHint
          }
          onClose={() => {
            if (!busy) setConfirm(null)
          }}
        >
          <div className="flex flex-wrap justify-end gap-3">
            <Action
              variant="outline"
              disabled={busy}
              onClick={() => setConfirm(null)}
            >
              {copy.cancel}
            </Action>
            <Action
              variant="destructive"
              disabled={locked}
              onClick={() =>
                void mutate(
                  () =>
                    confirm === "cancel"
                      ? client.tasks("cancel", revision)
                      : client.tasks("archive", {
                          ...revision,
                          archived: !task.archivedAt,
                        }),
                  () => setConfirm(null)
                )
              }
            >
              {confirm === "cancel"
                ? copy.cancelTask
                : task.archivedAt
                  ? copy.restore
                  : copy.archive}
            </Action>
          </div>
        </Modal>
      )}
      {discard && (
        <Modal
          title={copy.discardTitle}
          description={copy.discardHint}
          onClose={() => setDiscard(false)}
        >
          <div className="flex flex-wrap justify-end gap-3">
            <Action variant="outline" onClick={() => setDiscard(false)}>
              {copy.stay}
            </Action>
            <Action variant="destructive" onClick={onClose}>
              {copy.discardDraft}
            </Action>
          </div>
        </Modal>
      )}
    </Modal>
  )
}

function SavedTask({
  detail,
  members,
}: {
  detail: TaskDetail
  members: Member[]
}) {
  const copy = useBusinessCopy()
  const task = detail.task
  const deliverable = detail.deliverables.find(
    (entry) => entry.id === task.currentDeliverableId
  )
  return (
    <div className="space-y-5">
      <p
        className="text-sm leading-relaxed break-words whitespace-pre-wrap"
        dir="auto"
      >
        {task.notes || copy.briefEmpty}
      </p>
      <dl className="grid gap-4 rounded-xl bg-muted/30 p-4 sm:grid-cols-2">
        <div>
          <dt className="text-muted-foreground mb-2 text-xs">{copy.owner}</dt>
          <dd>
            <Person
              person={personFor(task.ownerId, members, copy.memberUnavailable)}
              fallback={copy.noOwner}
            />
          </dd>
        </div>
        <div>
          <dt className="text-muted-foreground mb-2 text-xs">
            {copy.assignee}
          </dt>
          <dd>
            <Person
              person={personFor(
                task.assigneeId,
                members,
                copy.memberUnavailable
              )}
              fallback={copy.unassigned}
            />
          </dd>
        </div>
        <div>
          <dt className="text-muted-foreground mb-2 text-xs">
            {copy.reviewer}
          </dt>
          <dd>
            <Person
              person={personFor(
                task.reviewerId,
                members,
                copy.memberUnavailable
              )}
              fallback={copy.anyReviewer}
            />
          </dd>
        </div>
        <div>
          <dt className="text-muted-foreground mb-2 text-xs">{copy.dueDate}</dt>
          <dd className="text-sm">
            <DueDay value={task.dueDate} fallback={copy.noDate} />
          </dd>
        </div>
      </dl>
      {(deliverable || task.status === "review") && (
        <section className="space-y-3 border-s-2 border-primary ps-4">
          <h3 className="text-sm font-semibold">{copy.deliverable}</h3>
          {deliverable ? (
            <>
              <p
                className="text-sm leading-relaxed break-words whitespace-pre-wrap"
                dir="auto"
              >
                {deliverable.body}
              </p>
              <Person
                compact
                person={{
                  name: deliverable.author.displayName,
                  kind: deliverable.author.kind,
                }}
                fallback=""
              />
            </>
          ) : (
            <p className="text-muted-foreground text-sm leading-relaxed">
              {copy.reviewWithoutDeliverable}
            </p>
          )}
        </section>
      )}
    </div>
  )
}
