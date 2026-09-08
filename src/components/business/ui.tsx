"use client"

import { useId, useRef, type ComponentProps, type ReactNode } from "react"
import {
  AlertCircle,
  Bot,
  Circle,
  CircleCheck,
  CircleDashed,
  CircleDot,
  CircleX,
  X,
} from "lucide-react"
import { Button } from "@/components/ui/button"
import { useOverlayHostHidden } from "@/components/ui/overlay-host-hidden"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "@/components/ui/dialog"
import { cn } from "@/lib/utils"
import { BusinessError } from "@/lib/business/client"
import { useBusinessCopy, type BusinessCopy } from "@/lib/business/copy"
import type { BusinessStatus, PersonLabel } from "@/lib/business/presentation"
import type { MemberRole } from "@/lib/business/identity"

export function Action({
  className,
  type = "button",
  ...props
}: ComponentProps<typeof Button>) {
  return (
    <Button
      type={type}
      className={cn(
        "min-h-11 rounded-xl px-4 motion-reduce:transition-none",
        props.variant === "destructive" &&
          "text-red-800 focus-visible:border-red-800 focus-visible:ring-red-800 dark:text-red-300 dark:focus-visible:border-red-300 dark:focus-visible:ring-red-300",
        className
      )}
      {...props}
    />
  )
}
export const controlClass =
  "min-h-11 w-full min-w-0 rounded-xl border border-input bg-background px-3 py-2 text-base text-foreground outline-none focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50 disabled:opacity-60 md:text-sm"

export function Field({
  label,
  hint,
  children,
}: {
  label: string
  hint?: string
  children: (id: string) => ReactNode
}) {
  const id = useId()
  return (
    <div className="grid min-w-0 gap-2">
      <label htmlFor={id} className="text-sm font-medium">
        {label}
      </label>
      {children(id)}
      {hint && (
        <p className="text-muted-foreground text-xs leading-relaxed">{hint}</p>
      )}
    </div>
  )
}

export function Brand({ compact = false }: { compact?: boolean }) {
  return (
    <div className="flex min-w-0 items-center gap-3">
      <svg
        viewBox="0 0 512 512"
        className="size-10 shrink-0"
        aria-hidden="true"
      >
        <rect width="512" height="512" rx="90" fill="#123c3a" />
        <path
          d="M152 136V376 M360 136V376 M152 256H360"
          stroke="#f5f1e7"
          strokeWidth="48"
          strokeLinecap="round"
        />
        <circle cx="256" cy="256" r="32" fill="#dbb66b" />
      </svg>
      {!compact && (
        <div className="min-w-0 leading-tight">
          <p className="font-semibold tracking-tight">Hafidh</p>
          <p className="text-muted-foreground text-xs">Ops Desk</p>
        </div>
      )}
    </div>
  )
}

const statusIcons = {
  todo: Circle,
  in_progress: CircleDot,
  review: CircleDashed,
  done: CircleCheck,
  cancelled: CircleX,
}
const statusClasses: Record<BusinessStatus, string> = {
  todo: "border-border text-muted-foreground bg-muted/40",
  in_progress:
    "border-blue-700/20 bg-blue-500/10 text-blue-800 dark:text-blue-300",
  review:
    "border-amber-700/20 bg-amber-500/10 text-amber-800 dark:text-amber-300",
  done: "border-emerald-700/20 bg-emerald-500/10 text-emerald-800 dark:text-emerald-300",
  cancelled: "border-border bg-muted/40 text-muted-foreground",
}
export function StatusBadge({ status }: { status: BusinessStatus }) {
  const copy = useBusinessCopy()
  const Icon = statusIcons[status]
  return (
    <span
      className={cn(
        "inline-flex w-fit shrink-0 items-center gap-1.5 rounded-lg border px-2 py-1 text-xs font-medium",
        statusClasses[status]
      )}
    >
      <Icon className="size-3.5" aria-hidden="true" />
      {copy[status]}
    </span>
  )
}

export function Person({
  person,
  fallback,
  compact = false,
}: {
  person: PersonLabel | null
  fallback: string
  compact?: boolean
}) {
  const copy = useBusinessCopy()
  if (!person)
    return <span className="text-muted-foreground text-xs">{fallback}</span>
  return (
    <span className="inline-flex min-w-0 max-w-full items-center gap-2">
      <span
        aria-hidden="true"
        className={cn(
          "bg-muted text-foreground flex shrink-0 items-center justify-center rounded-full text-xs font-semibold",
          compact ? "size-6" : "size-8"
        )}
      >
        {person.kind === "agent" ? (
          <Bot className="size-4" />
        ) : person.kind === "unknown" ? (
          "?"
        ) : (
          Array.from(person.name.trim())[0]?.toUpperCase()
        )}
      </span>
      <span
        className={cn(
          "min-w-0 text-sm",
          compact ? "truncate" : "[overflow-wrap:anywhere]"
        )}
      >
        <bdi>{person.name}</bdi>
        {person.kind === "agent" && (
          <span className="text-muted-foreground ms-1.5 text-xs">
            {copy.agent}
          </span>
        )}
      </span>
    </span>
  )
}

export function roleLabel(role: MemberRole, copy: BusinessCopy): string {
  return {
    owner: copy.roleOwner,
    admin: copy.roleAdmin,
    manager: copy.roleManager,
    member: copy.roleMember,
    viewer: copy.roleViewer,
  }[role]
}

export function ErrorNotice({
  error,
  children,
}: {
  error: unknown
  children?: ReactNode
}) {
  const copy = useBusinessCopy()
  const kind = error instanceof BusinessError ? error.kind : "offline"
  const message = {
    unauthorized: copy.sessionExpired,
    forbidden: copy.forbidden,
    missing: copy.notFound,
    conflict: copy.conflict,
    invalid: copy.invalid,
    bootstrap: copy.bootstrapTitle,
    offline: copy.offline,
    closed: copy.sessionExpired,
  }[kind]
  return (
    <div
      role="alert"
      className="border-destructive/25 bg-destructive/5 flex gap-3 rounded-xl border p-4 text-sm"
    >
      <AlertCircle
        className="text-destructive mt-0.5 size-4 shrink-0"
        aria-hidden="true"
      />
      <div className="min-w-0 space-y-2">
        <p>{message}</p>
        {children}
      </div>
    </div>
  )
}

export function Modal({
  title,
  description,
  children,
  onClose,
  wide = false,
}: {
  title: string
  description?: string
  children: ReactNode
  onClose: () => void
  wide?: boolean
}) {
  const copy = useBusinessCopy()
  const returnFocus = useRef<HTMLElement | null>(null)
  const hidden = useOverlayHostHidden()
  return (
    <Dialog
      open={!hidden}
      onOpenChange={(open) => {
        if (!open && !hidden) onClose()
      }}
    >
      <DialogContent
        showCloseButton={false}
        onOpenAutoFocus={() => {
          returnFocus.current =
            document.activeElement instanceof HTMLElement
              ? document.activeElement
              : null
        }}
        onCloseAutoFocus={(event) => {
          // These controlled dialogs have no Radix Trigger. Restore the actual
          // opener, or the work area when an updated/archived row has disappeared.
          event.preventDefault()
          if (document.activeElement?.closest('[role="dialog"]')) return
          // A closed work tab may already have restored focus to its surviving
          // sibling. Do not override that with this dialog's removed opener.
          if (document.activeElement?.getAttribute("role") === "tab") return
          if (
            returnFocus.current?.isConnected &&
            returnFocus.current !== document.body &&
            returnFocus.current !== document.documentElement
          ) {
            returnFocus.current.focus()
            if (document.activeElement === returnFocus.current) return
          }
          document.getElementById("business-main")?.focus()
        }}
        className={cn(
          "min-w-0 grid-cols-1 gap-5 rounded-2xl p-5 [overflow-wrap:anywhere] motion-reduce:animate-none! sm:p-7",
          wide ? "max-w-3xl" : "max-w-lg"
        )}
      >
        <div className="flex items-start justify-between gap-4">
          <div className="min-w-0 space-y-2">
            <DialogTitle className="text-xl font-semibold tracking-tight">
              {title}
            </DialogTitle>
            <DialogDescription
              className={description ? "text-sm leading-relaxed" : "sr-only"}
            >
              {description ?? title}
            </DialogDescription>
          </div>
          <Action
            variant="ghost"
            size="icon"
            className="size-11 shrink-0 p-0"
            onClick={onClose}
            aria-label={copy.close}
          >
            <X aria-hidden="true" />
          </Action>
        </div>
        {children}
      </DialogContent>
    </Dialog>
  )
}
