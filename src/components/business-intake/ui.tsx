"use client"

import { useEffect, useState, type ReactNode } from "react"
import { LockKeyhole } from "lucide-react"
import { BusinessError } from "@/lib/business/client"
import { useBusinessCopy } from "@/lib/business/copy"
import { useIntakeCopy } from "@/lib/business/intake-copy"
import type { Member } from "@/lib/business/identity"
import type {
  Passage,
  PreparedTask,
  SourceSummary,
} from "@/lib/business/intake"
import { sourceIsFresh } from "@/lib/business/intake"
import { personFor } from "@/lib/business/tasks"
import { ErrorNotice, Person } from "@/components/business/ui"
import { cn } from "@/lib/utils"

export function IntakeError({
  error,
  children,
}: {
  error: unknown
  children?: ReactNode
}) {
  const copy = useIntakeCopy()
  if (error instanceof BusinessError && error.intakeReason)
    return (
      <div
        role="alert"
        className="border-border bg-muted/50 space-y-3 rounded-xl border p-4 text-sm leading-relaxed"
      >
        <p>{copy.reason[error.intakeReason]}</p>
        {children}
      </div>
    )
  return <ErrorNotice error={error}>{children}</ErrorNotice>
}

export function accessWasLost(error: unknown): boolean {
  if (!(error instanceof BusinessError)) return false
  return (
    ["unauthorized", "closed", "missing"].includes(error.kind) ||
    (error.kind === "forbidden" &&
      error.intakeReason !== "publication_not_allowed") ||
    [
      "binding_missing",
      "binding_disabled",
      "credential_unavailable",
      "binding_unavailable",
    ].some((reason) => error.intakeReason === reason)
  )
}
export function outcomeIsUncertain(error: unknown): boolean {
  return error instanceof BusinessError && error.kind === "offline"
}

// Local expiry only hides already-disclosed text. Only a returned upstream
// validation can supply a new deadline; visibility/focus never extends it.
export function useSourceFreshness(source: SourceSummary | null): boolean {
  const [now, setNow] = useState(Date.now)
  useEffect(() => {
    if (!source?.accessValidUntil) return
    const remaining = Date.parse(source.accessValidUntil) - Date.now()
    const timer = window.setTimeout(
      () => setNow(Date.now()),
      Math.max(0, Math.min(remaining + 1, 2147483647))
    )
    const check = () => setNow(Date.now())
    window.addEventListener("focus", check)
    document.addEventListener("visibilitychange", check)
    return () => {
      window.clearTimeout(timer)
      window.removeEventListener("focus", check)
      document.removeEventListener("visibilitychange", check)
    }
  }, [source?.accessValidUntil])
  return source !== null && sourceIsFresh(source, now)
}

export function Check({
  checked,
  onChange,
  disabled,
  children,
}: {
  checked: boolean
  onChange: (checked: boolean) => void
  disabled?: boolean
  children: ReactNode
}) {
  return (
    <label
      className={cn(
        "border-border flex min-h-11 cursor-pointer items-start gap-3 rounded-xl border p-3 text-sm leading-relaxed",
        disabled && "cursor-default"
      )}
    >
      <input
        type="checkbox"
        checked={checked}
        onChange={(event) => onChange(event.target.checked)}
        disabled={disabled}
        className="accent-primary focus-visible:outline-ring mt-1 size-4 shrink-0 focus-visible:outline-2 focus-visible:outline-offset-4"
      />
      <span className="min-w-0 break-words">{children}</span>
    </label>
  )
}

export function PrivateNotice() {
  const copy = useIntakeCopy()
  return (
    <div className="border-border text-muted-foreground flex gap-3 border-b pb-5 text-sm leading-relaxed">
      <LockKeyhole className="mt-0.5 size-4 shrink-0" aria-hidden />
      <div>
        <p className="text-foreground mb-1 font-medium">{copy.private}</p>
        <p>{copy.privateHint}</p>
      </div>
    </div>
  )
}

export function Passages({
  passages,
  selected,
  onSelect,
  disabled = false,
}: {
  passages: Passage[]
  selected?: string[]
  onSelect?: (ids: string[]) => void
  disabled?: boolean
}) {
  const copy = useIntakeCopy()
  return (
    <ol className="space-y-3">
      {passages.map((passage, index) => {
        const checked = selected?.includes(passage.id) ?? false
        const text = (
          <span className="block min-w-0 flex-1">
            <span className="text-muted-foreground mb-2 block text-xs font-medium tabular-nums">
              {copy.exactPassage} {index + 1}
            </span>
            <span
              dir="auto"
              className="block whitespace-pre-wrap break-words text-base leading-7 [overflow-wrap:anywhere]"
            >
              {passage.text}
            </span>
          </span>
        )
        return (
          <li
            key={passage.id}
            className={cn(
              "border-border rounded-xl border",
              checked ? "border-primary bg-primary/5" : "bg-background"
            )}
          >
            {onSelect ? (
              <label className="flex cursor-pointer items-start gap-3 p-4 sm:p-5">
                <input
                  type="checkbox"
                  checked={checked}
                  disabled={disabled}
                  aria-label={`${copy.exactPassage} ${index + 1}`}
                  className="accent-primary focus-visible:outline-ring mt-1 size-4 shrink-0 focus-visible:outline-2 focus-visible:outline-offset-4"
                  onChange={() =>
                    onSelect(
                      checked
                        ? (selected ?? []).filter((id) => id !== passage.id)
                        : [...(selected ?? []), passage.id]
                    )
                  }
                />
                {text}
              </label>
            ) : (
              <div className="p-4 sm:p-5">{text}</div>
            )}
          </li>
        )
      })}
    </ol>
  )
}

export function PreparedTaskView({
  task,
  members,
}: {
  task: PreparedTask
  members: Member[]
}) {
  const copy = useBusinessCopy()
  const people = [
    [copy.owner, task.ownerId, copy.memberUnavailable],
    [copy.assignee, task.assigneeId, copy.unassigned],
    [copy.reviewer, task.reviewerId, copy.anyReviewer],
  ] as const
  return (
    <div className="space-y-5">
      <div>
        <h3
          dir="auto"
          className="break-words text-lg font-semibold [overflow-wrap:anywhere]"
        >
          {task.title}
        </h3>
        <p
          dir="auto"
          className="mt-3 whitespace-pre-wrap break-words text-sm leading-6 [overflow-wrap:anywhere]"
        >
          {task.notes || copy.briefEmpty}
        </p>
      </div>
      <dl className="grid gap-4 text-sm sm:grid-cols-2">
        <div>
          <dt className="text-muted-foreground mb-1 text-xs">{copy.domain}</dt>
          <dd>{copy[task.domain]}</dd>
        </div>
        <div>
          <dt className="text-muted-foreground mb-1 text-xs">
            {copy.priority}
          </dt>
          <dd>{copy[task.priority]}</dd>
        </div>
        <div>
          <dt className="text-muted-foreground mb-1 text-xs">{copy.dueDate}</dt>
          <dd>
            {task.dueDate ? <span dir="ltr">{task.dueDate}</span> : copy.noDate}
          </dd>
        </div>
        {people.map(([label, id, empty]) => (
          <div key={label}>
            <dt className="text-muted-foreground mb-1 text-xs">{label}</dt>
            <dd>
              {id ? (
                <Person
                  person={personFor(id, members, copy.memberUnavailable)}
                  fallback={empty}
                />
              ) : (
                empty
              )}
            </dd>
          </div>
        ))}
      </dl>
    </div>
  )
}
