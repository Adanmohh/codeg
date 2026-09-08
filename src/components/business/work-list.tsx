"use client"

import { ArrowUpRight, CalendarDays } from "lucide-react"
import {
  BUSINESS_STATUSES,
  type TaskPreview,
} from "@/lib/business/presentation"
import { useBusinessCopy } from "@/lib/business/copy"
import { Person, StatusBadge } from "./ui"

export function WorkList({
  tasks,
  mode,
  onOpen,
}: {
  tasks: TaskPreview[]
  mode: "list" | "board" | "table"
  onOpen: (key: string) => void
}) {
  const copy = useBusinessCopy()
  if (mode === "table")
    return (
      <div
        className="border-border focus-visible:ring-ring overflow-x-auto rounded-xl border bg-card outline-none focus-visible:ring-2"
        tabIndex={0}
        role="region"
        aria-label={copy.table}
      >
        <table className="w-full min-w-[820px] border-collapse text-start text-sm">
          <caption className="sr-only">{copy.workTab}</caption>
          <thead className="bg-muted/40 border-border border-b text-xs">
            <tr>
              {[
                copy.taskTitle,
                copy.status,
                copy.domain,
                copy.priority,
                copy.assignee,
                copy.dueDate,
              ].map((label) => (
                <th
                  key={label}
                  scope="col"
                  className="px-4 py-3 text-start font-medium whitespace-nowrap"
                >
                  {label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="divide-y">
            {tasks.map((task) => (
              <tr key={task.key} className="hover:bg-muted/30">
                <th
                  scope="row"
                  className="min-w-64 max-w-96 px-4 py-2 text-start font-medium"
                >
                  <button
                    type="button"
                    onClick={() => onOpen(task.key)}
                    className="focus-visible:ring-ring min-h-11 w-full rounded-md py-2 text-start leading-relaxed break-words outline-none focus-visible:ring-2"
                  >
                    <bdi>{task.title}</bdi>
                  </button>
                </th>
                <td className="px-4 py-3">
                  <StatusBadge status={task.status} />
                </td>
                <td className="px-4 py-3 whitespace-nowrap">
                  {copy[task.domain]}
                </td>
                <td className="px-4 py-3 whitespace-nowrap">
                  {copy[task.priority]}
                </td>
                <td className="min-w-44 px-4 py-3">
                  <Person
                    compact
                    person={task.assignee}
                    fallback={copy.unassigned}
                  />
                </td>
                <td className="px-4 py-3 text-xs whitespace-nowrap">
                  <DueDay value={task.dueDate} fallback={copy.noDate} />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    )
  if (mode === "board")
    return (
      <div
        className="flex min-w-0 gap-4 overflow-x-auto pb-4"
        aria-label={copy.board}
        tabIndex={0}
      >
        {BUSINESS_STATUSES.map((status) => {
          const column = tasks.filter((task) => task.status === status)
          return (
            <section
              key={status}
              className="bg-muted/35 w-[260px] shrink-0 rounded-2xl p-3"
            >
              <h2 className="mb-4 flex items-center justify-between gap-2 px-1">
                <StatusBadge status={status} />
                <span className="text-muted-foreground text-xs tabular-nums">
                  {column.length}
                </span>
              </h2>
              <div className="space-y-3">
                {column.map((task) => (
                  <button
                    type="button"
                    key={task.key}
                    onClick={() => onOpen(task.key)}
                    className="group border-border bg-card hover:border-primary/50 focus-visible:ring-ring w-full rounded-xl border p-4 text-start shadow-xs outline-none focus-visible:ring-2"
                  >
                    <p className="text-muted-foreground mb-2 text-xs">
                      {copy[task.domain]} · {copy[task.priority]}
                    </p>
                    <h3 className="mb-4 text-sm leading-relaxed font-semibold break-words">
                      <bdi>{task.title}</bdi>
                    </h3>
                    <Responsibility task={task} />
                    <div className="text-muted-foreground mt-4 flex items-center gap-2 border-t pt-3 text-xs">
                      <CalendarDays className="size-3.5" aria-hidden="true" />
                      <DueDay value={task.dueDate} fallback={copy.noDate} />
                    </div>
                  </button>
                ))}
                {column.length === 0 && (
                  <p className="text-muted-foreground rounded-xl border border-dashed px-4 py-6 text-center text-xs">
                    {copy.emptyColumn}
                  </p>
                )}
              </div>
            </section>
          )
        })}
      </div>
    )
  return (
    <div className="border-border overflow-hidden rounded-2xl border bg-card">
      <div
        aria-hidden="true"
        className="text-muted-foreground bg-muted/30 hidden grid-cols-[minmax(0,1fr)_200px_140px] gap-5 border-b px-6 py-3 text-xs lg:grid"
      >
        <span>{copy.taskTitle}</span>
        <span>{copy.assignment}</span>
        <span>{copy.dueDate}</span>
      </div>
      <ul className="divide-y">
        {tasks.map((task) => (
          <li key={task.key}>
            <button
              type="button"
              onClick={() => onOpen(task.key)}
              className="group hover:bg-muted/40 focus-visible:ring-ring grid w-full min-w-0 gap-4 px-4 py-5 text-start outline-none focus-visible:ring-2 focus-visible:ring-inset sm:px-6 lg:grid-cols-[minmax(0,1fr)_200px_140px] lg:items-center lg:gap-5"
            >
              <div className="min-w-0">
                <div className="mb-2 flex flex-wrap items-center gap-x-3 gap-y-2">
                  <StatusBadge status={task.status} />
                  <span className="text-muted-foreground text-xs">
                    {copy[task.domain]} · {copy[task.priority]}
                  </span>
                </div>
                <h2 className="text-base leading-relaxed font-semibold tracking-tight break-words">
                  <bdi>{task.title}</bdi>
                </h2>
              </div>
              <Responsibility task={task} />
              <div className="text-muted-foreground flex items-center gap-2 text-xs">
                <CalendarDays className="size-4 shrink-0" aria-hidden="true" />
                <DueDay value={task.dueDate} fallback={copy.noDate} />
                <ArrowUpRight
                  className="text-foreground ms-auto size-4 shrink-0 rtl:-rotate-90"
                  aria-hidden="true"
                />
              </div>
            </button>
          </li>
        ))}
      </ul>
    </div>
  )
}
function Responsibility({ task }: { task: TaskPreview }) {
  const copy = useBusinessCopy()
  return (
    <div className="grid min-w-0 gap-2">
      <div className="flex min-w-0 items-center gap-2">
        <span className="text-muted-foreground text-xs">{copy.owner}</span>
        <Person compact person={task.owner} fallback={copy.noOwner} />
      </div>
      <div className="flex min-w-0 items-center gap-2">
        <span className="text-muted-foreground text-xs">{copy.assignee}</span>
        <Person compact person={task.assignee} fallback={copy.unassigned} />
      </div>
    </div>
  )
}
export function DueDay({
  value,
  fallback,
}: {
  value: string | null
  fallback: string
}) {
  // dueDate is a calendar day, not a timestamp. Never convert via Date/UTC.
  return value ? (
    <time dateTime={value} dir="ltr" className="tabular-nums">
      {value}
    </time>
  ) : (
    <span>{fallback}</span>
  )
}
