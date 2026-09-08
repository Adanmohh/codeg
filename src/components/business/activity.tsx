"use client"

import { useLocale } from "next-intl"
import type { ReactNode } from "react"
import type { Member } from "@/lib/business/identity"
import {
  BUSINESS_DOMAINS,
  BUSINESS_PRIORITIES,
  BUSINESS_STATUSES,
} from "@/lib/business/presentation"
import { personFor, type Activity, type TaskDetail } from "@/lib/business/tasks"
import { useBusinessCopy } from "@/lib/business/copy"
import { DueDay } from "./work-list"

function known<T extends string>(
  values: readonly T[],
  value: unknown
): value is T {
  return typeof value === "string" && values.includes(value as T)
}
function Plain({ children }: { children: ReactNode }) {
  return (
    <p
      className="mt-2 text-sm leading-relaxed break-words whitespace-pre-wrap"
      dir="auto"
    >
      {children}
    </p>
  )
}

// Exact public payloads read from tickets' business_tasks/store.rs at 76bb6909.
// Do not stringify arbitrary future payloads or render HTML/private metadata.
function Change({
  entry,
  detail,
  members,
}: {
  entry: Activity
  detail: TaskDetail
  members: Member[]
}) {
  const copy = useBusinessCopy()
  const payload =
    entry.payload &&
    typeof entry.payload === "object" &&
    !Array.isArray(entry.payload)
      ? (entry.payload as Record<string, unknown>)
      : {}
  const text = (key: string) =>
    typeof payload[key] === "string" ? (payload[key] as string) : ""
  if (entry.kind === "note") return <Plain>{text("body")}</Plain>
  if (entry.kind === "reviewed")
    return (
      <>
        <p className="mt-2 text-xs">
          {payload.decision === "accept"
            ? copy.done
            : payload.decision === "return"
              ? copy.in_progress
              : copy.review}
        </p>
        {text("comment") && <Plain>{text("comment")}</Plain>}
      </>
    )
  if (entry.kind === "progressed")
    return (
      <p className="mt-2 text-xs">
        {known(BUSINESS_STATUSES, payload.from) && copy[payload.from]} →{" "}
        {known(BUSINESS_STATUSES, payload.to) && copy[payload.to]}
      </p>
    )
  if (entry.kind === "assigned")
    return (
      <dl className="text-muted-foreground mt-3 grid gap-2 text-xs">
        {(
          [
            ["ownerId", copy.owner, copy.noOwner],
            ["assigneeId", copy.assignee, copy.unassigned],
            ["reviewerId", copy.reviewer, copy.anyReviewer],
          ] as const
        ).map(([key, label, empty]) => (
          <div className="flex flex-wrap gap-2" key={key}>
            <dt>{label}</dt>
            <dd>
              <bdi>
                {personFor(text(key) || null, members, copy.memberUnavailable)
                  ?.name ?? empty}
              </bdi>
            </dd>
          </div>
        ))}
      </dl>
    )
  if (entry.kind === "submitted") {
    const delivered = detail.deliverables.find(
      (value) => value.id === text("deliverableId")
    )
    return delivered ? (
      <details className="mt-2">
        <summary className="focus-visible:ring-ring flex min-h-11 cursor-pointer items-center rounded-lg text-sm outline-none focus-visible:ring-2">
          {copy.deliverable}
        </summary>
        <Plain>{delivered.body}</Plain>
      </details>
    ) : null
  }
  if (entry.kind === "created" || entry.kind === "updated")
    return (
      <details className="mt-2">
        <summary className="focus-visible:ring-ring flex min-h-11 cursor-pointer items-center rounded-lg text-sm outline-none focus-visible:ring-2">
          {copy.savedVersion} · {copy.sourceVersion} {entry.revision}
        </summary>
        <Plain>{text("title")}</Plain>
        {text("notes") && <Plain>{text("notes")}</Plain>}
        <p className="text-muted-foreground mt-2 text-xs">
          {known(BUSINESS_DOMAINS, payload.domain) && copy[payload.domain]}{" "}
          {known(BUSINESS_PRIORITIES, payload.priority) &&
            ` · ${copy[payload.priority]}`}
        </p>
        <p className="mt-2 text-xs">
          <DueDay value={text("dueDate") || null} fallback={copy.noDate} />
        </p>
      </details>
    )
  if (entry.kind === "archived")
    return (
      <p className="mt-2 text-xs">
        {payload.archived === false ? copy.restore : copy.archived}
      </p>
    )
  return null
}

export function ActivityList({
  detail,
  members,
}: {
  detail: TaskDetail
  members: Member[]
}) {
  const copy = useBusinessCopy()
  const locale = useLocale()
  const labels: Record<string, string> = {
    created: copy.createdTask,
    updated: copy.workChanged,
    progressed: copy.workChanged,
    assigned: copy.assigned,
    note: copy.noteAdded,
    submitted: copy.delivered,
    reviewed: copy.reviewed,
    cancelled: copy.cancelled,
    archived: copy.archived,
    execution_linked: copy.executionLinked,
    execution_entrusted: copy.executionEntrusted,
  }
  return (
    <section className="space-y-4 border-t pt-5">
      <h3 className="text-sm font-semibold">{copy.activity}</h3>
      <ol className="space-y-5">
        {detail.activity.map((entry) => (
          <li key={entry.id} className="border-s border-border ps-4">
            <div className="flex flex-wrap items-center justify-between gap-2">
              <p className="text-sm">
                <bdi className="font-medium">{entry.actor.displayName}</bdi> ·{" "}
                {labels[entry.kind] ?? copy.workChanged}
              </p>
              <Timestamp value={entry.createdAt} locale={locale} />
            </div>
            <Change entry={entry} detail={detail} members={members} />
          </li>
        ))}
      </ol>
      {detail.activity.length === 0 && (
        <p className="text-muted-foreground text-sm">{copy.activityEmpty}</p>
      )}
    </section>
  )
}
function Timestamp({ value, locale }: { value: string; locale: string }) {
  // Activity timestamps are instants. Never use this conversion for dueDate.
  const parsed = new Date(value)
  return (
    <time className="text-muted-foreground text-xs" dateTime={value}>
      {Number.isNaN(parsed.getTime())
        ? "—"
        : new Intl.DateTimeFormat(locale, {
            dateStyle: "medium",
            timeStyle: "short",
          }).format(parsed)}
    </time>
  )
}
