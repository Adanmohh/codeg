"use client"

import { useState } from "react"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { BusinessError, type BusinessClient } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import type { Member } from "@/lib/business/identity"
import {
  BUSINESS_PRIORITIES,
  type BusinessDomain,
  type BusinessPriority,
} from "@/lib/business/presentation"
import type { Assignment, TaskDetail, TaskFields } from "@/lib/business/tasks"
import { Action, controlClass, ErrorNotice, Field, Modal } from "./ui"

export function MetadataFields({
  value,
  setValue,
  domains,
  disabled,
}: {
  value: TaskFields
  setValue: (value: TaskFields) => void
  domains: BusinessDomain[]
  disabled: boolean
}) {
  const copy = useBusinessCopy()
  return (
    <fieldset disabled={disabled} className="@container min-w-0 space-y-5">
      <Field label={copy.taskTitle}>
        {(id) => (
          <Input
            id={id}
            className="min-h-11 rounded-xl"
            value={value.title}
            onChange={(event) =>
              setValue({ ...value, title: event.target.value })
            }
            maxLength={240}
            required
            dir="auto"
          />
        )}
      </Field>
      <Field label={copy.outcome}>
        {(id) => (
          <Textarea
            id={id}
            value={value.notes}
            onChange={(event) =>
              setValue({ ...value, notes: event.target.value })
            }
            maxLength={20000}
            className="min-h-32"
            dir="auto"
          />
        )}
      </Field>
      <div className="grid gap-5 @[30rem]:grid-cols-3">
        <Field label={copy.domain}>
          {(id) => (
            <select
              id={id}
              className={controlClass}
              value={value.domain}
              onChange={(event) =>
                setValue({
                  ...value,
                  domain: event.target.value as BusinessDomain,
                })
              }
              required
            >
              {domains.map((domain) => (
                <option key={domain} value={domain}>
                  {copy[domain]}
                </option>
              ))}
            </select>
          )}
        </Field>
        <Field label={copy.priority}>
          {(id) => (
            <select
              id={id}
              className={controlClass}
              value={value.priority}
              onChange={(event) =>
                setValue({
                  ...value,
                  priority: event.target.value as BusinessPriority,
                })
              }
            >
              {BUSINESS_PRIORITIES.map((priority) => (
                <option key={priority} value={priority}>
                  {copy[priority]}
                </option>
              ))}
            </select>
          )}
        </Field>
        <Field label={copy.dueDate} hint={copy.allDay}>
          {(id) => (
            <Input
              id={id}
              className="min-h-11 rounded-xl"
              type="date"
              dir="ltr"
              min="0001-01-01"
              max="9999-12-31"
              value={value.dueDate ?? ""}
              onChange={(event) =>
                setValue({ ...value, dueDate: event.target.value || null })
              }
            />
          )}
        </Field>
      </div>
    </fieldset>
  )
}

export function AssignmentFields({
  value,
  setValue,
  members,
  domain,
  disabled,
  selfOnly,
}: {
  value: Assignment
  setValue: (value: Assignment) => void
  members: Member[]
  domain: BusinessDomain
  disabled: boolean
  selfOnly?: string
}) {
  const copy = useBusinessCopy()
  const active = members.filter(
    (member) =>
      member.status === "active" &&
      member.domains.includes(domain) &&
      member.role !== "viewer"
  )
  const owners = active.filter(
    (member) => member.kind === "human" && (!selfOnly || member.id === selfOnly)
  )
  const executors = active.filter(
    (member) => !selfOnly || member.id === selfOnly
  )
  const reviewers = active.filter(
    (member) =>
      member.kind === "human" &&
      ["owner", "admin", "manager"].includes(member.role)
  )
  function options(entries: Member[]) {
    return entries.map((member) => (
      <option key={member.id} value={member.id}>
        {member.displayName}
        {member.kind === "agent" ? ` · ${copy.agent}` : ""}
      </option>
    ))
  }
  function missing(id: string | null, entries: Member[]) {
    return id && !entries.some((member) => member.id === id) ? (
      <option value={id} disabled>
        {copy.memberUnavailable}
      </option>
    ) : null
  }
  return (
    <fieldset disabled={disabled} className="@container min-w-0 space-y-4">
      <legend className="mb-2 text-sm font-medium">{copy.assignment}</legend>
      <p className="text-muted-foreground text-xs leading-relaxed">
        {copy.assignmentHint}
      </p>
      <div className="grid gap-4 @[36rem]:grid-cols-3">
        <Field label={copy.owner}>
          {(id) => (
            <select
              id={id}
              className={controlClass}
              required
              value={value.ownerId}
              disabled={disabled || !!selfOnly}
              onChange={(event) =>
                setValue({ ...value, ownerId: event.target.value })
              }
            >
              {missing(value.ownerId, owners)}
              {options(owners)}
            </select>
          )}
        </Field>
        <Field label={copy.assignee}>
          {(id) => (
            <select
              id={id}
              className={controlClass}
              value={value.assigneeId ?? ""}
              onChange={(event) =>
                setValue({ ...value, assigneeId: event.target.value || null })
              }
            >
              <option value="">{copy.unassigned}</option>
              {missing(value.assigneeId, executors)}
              {options(executors)}
            </select>
          )}
        </Field>
        <Field label={copy.reviewer} hint={copy.reviewerHint}>
          {(id) => (
            <select
              id={id}
              className={controlClass}
              value={value.reviewerId ?? ""}
              disabled={disabled || !!selfOnly}
              onChange={(event) =>
                setValue({ ...value, reviewerId: event.target.value || null })
              }
            >
              <option value="">{copy.anyReviewer}</option>
              {missing(value.reviewerId, reviewers)}
              {options(reviewers)}
            </select>
          )}
        </Field>
      </div>
    </fieldset>
  )
}

export function CreateTask({
  client,
  actor,
  members,
  onClose,
  onCreated,
}: {
  client: BusinessClient
  actor: Member
  members: Member[]
  onClose: () => void
  onCreated: (detail: TaskDetail) => void
}) {
  const copy = useBusinessCopy()
  const [fields, setFields] = useState<TaskFields>({
    title: "",
    notes: "",
    domain: actor.domains[0],
    priority: "normal",
    dueDate: null,
  })
  const [assignment, setAssignment] = useState<Assignment>({
    ownerId: actor.id,
    assigneeId: null,
    reviewerId: null,
  })
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState<unknown>(null)
  const [discard, setDiscard] = useState(false)
  const dirty =
    fields.title !== "" ||
    fields.notes !== "" ||
    fields.dueDate !== null ||
    fields.priority !== "normal" ||
    assignment.assigneeId !== null ||
    assignment.reviewerId !== null ||
    assignment.ownerId !== actor.id
  const uncertain = error instanceof BusinessError && error.kind === "offline"
  function close() {
    if (busy) return
    if (dirty) setDiscard(true)
    else onClose()
  }
  return (
    <Modal
      title={copy.createTask}
      description={copy.emptyHint}
      onClose={close}
      wide
    >
      <form
        className="space-y-6"
        onSubmit={async (event) => {
          event.preventDefault()
          setBusy(true)
          setError(null)
          try {
            onCreated(
              await client.tasks("create", {
                ...fields,
                title: fields.title.trim(),
                ...assignment,
              })
            )
          } catch (caught) {
            setError(caught)
          } finally {
            setBusy(false)
          }
        }}
      >
        <MetadataFields
          value={fields}
          setValue={setFields}
          domains={actor.domains}
          disabled={busy}
        />
        <AssignmentFields
          value={assignment}
          setValue={setAssignment}
          domain={fields.domain}
          members={members}
          disabled={busy}
          selfOnly={actor.role === "member" ? actor.id : undefined}
        />
        {error != null && (
          <ErrorNotice error={error}>
            {uncertain && <p>{copy.refreshBeforeRetry}</p>}
          </ErrorNotice>
        )}
        <div className="flex flex-wrap justify-end gap-3">
          <Action
            type="button"
            variant="outline"
            disabled={busy}
            onClick={close}
          >
            {copy.cancel}
          </Action>
          <Action
            type="submit"
            disabled={
              busy ||
              uncertain ||
              !fields.title.trim() ||
              !assignment.ownerId ||
              !fields.domain
            }
          >
            {busy ? copy.saving : copy.createTask}
          </Action>
        </div>
      </form>
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
